#!/usr/bin/env python3
"""runs.py — run manifests: schema check, re-run, record.

A run manifest (research/runs/<id>/manifest.yaml) pins everything needed to
reproduce a run: the code commit and code file hashes, the params file and
its hash, the seed, input data hashes, toolchain versions, and the hashes of
the outputs. Re-running a manifest must give byte-identical outputs.

  python3 scripts/runs.py check  research/runs/*/manifest.yaml
  python3 scripts/runs.py rerun  research/runs/*/manifest.yaml
  python3 scripts/runs.py record research/runs/<id>   # writes manifest.yaml

Exit 0 = all good, 1 = at least one failure, 2 = usage.
Stdlib only (no PyYAML): manifests use a small YAML subset, see SCHEMA.
"""

import hashlib
import os
import re
import shutil
import subprocess
import sys
import tempfile

SCHEMA_ID = "astra-run-manifest/1"

# field -> kind. "map" = one nested level of `key: value`.
SCHEMA = {
    "schema": "str",
    "id": "str",
    "description": "str",
    "code_commit": "sha",
    "command": "str",
    "seed": "int",
    "params_file": "str",
    "params_sha256": "sha256",
    "code": "map",
    "data": "map",
    "toolchain": "map",
    "outputs": "map",
}
NON_EMPTY_MAPS = ("code", "toolchain", "outputs")

HEX40 = re.compile(r"^[0-9a-f]{40}$")
HEX64 = re.compile(r"^[0-9a-f]{64}$")
ID_RE = re.compile(r"^[a-z0-9][a-z0-9-]*$")


# --- YAML subset --------------------------------------------------------------

def _scalar(v):
    v = v.strip()
    if len(v) >= 2 and v[0] == v[-1] and v[0] in "\"'":
        return v[1:-1]
    return v


def parse(text):
    """Top-level `key: value` and `key:` + indented `sub: value`. `{}` = empty map."""
    out, cur = {}, None
    for n, raw in enumerate(text.splitlines(), 1):
        line = raw.split(" #", 1)[0].rstrip() if not raw.lstrip().startswith("#") else ""
        if not line.strip():
            continue
        indented = line[0] in " \t"
        if ":" not in line:
            raise ValueError(f"line {n}: expected 'key: value'")
        k, v = line.strip().split(":", 1)
        k, v = k.strip(), v.strip()
        if indented:
            if cur is None:
                raise ValueError(f"line {n}: indented key outside a map")
            out[cur][_scalar(k)] = _scalar(v)
            continue
        if k in out:
            raise ValueError(f"line {n}: duplicate key {k}")
        if v == "":
            out[k], cur = {}, k
        elif v == "{}":
            out[k], cur = {}, None
        else:
            out[k], cur = _scalar(v), None
    return out


def dump(m):
    lines = []
    for k in SCHEMA:
        v = m[k]
        if isinstance(v, dict):
            if not v:
                lines.append(f"{k}: {{}}")
            else:
                lines.append(f"{k}:")
                lines += [f"  {sk}: {v[sk]}" for sk in sorted(v)]
        else:
            lines.append(f"{k}: {v}")
    return "\n".join(lines) + "\n"


# --- helpers -----------------------------------------------------------------

def sha256(path):
    h = hashlib.sha256()
    with open(path, "rb") as f:
        for chunk in iter(lambda: f.read(1 << 16), b""):
            h.update(chunk)
    return h.hexdigest()


def repo_root():
    return subprocess.run(["git", "rev-parse", "--show-toplevel"], capture_output=True,
                          text=True, check=True).stdout.strip()


def toolchain_now(names):
    out = {}
    for name in names:
        try:
            r = subprocess.run([name, "--version"], capture_output=True, text=True)
            txt = (r.stdout or r.stderr).strip().splitlines()[0]
            m = re.search(r"\d+\.\d+(\.\d+)?", txt)
            out[name] = m.group(0) if m else txt
        except (OSError, IndexError):
            out[name] = "missing"
    return out


def _same_minor(a, b):
    return a.split(".")[:2] == b.split(".")[:2]


# --- check -------------------------------------------------------------------

def check(path, root):
    """Schema + hashes. Returns (manifest or None, list of errors, list of warnings)."""
    errs, warns = [], []
    try:
        with open(path) as f:
            m = parse(f.read())
    except (OSError, ValueError) as e:
        return None, [f"cannot read: {e}"], warns

    for k, kind in SCHEMA.items():
        if k not in m:
            errs.append(f"missing field: {k}")
            continue
        v = m[k]
        if kind == "map" and not isinstance(v, dict):
            errs.append(f"{k}: expected a map")
        elif kind != "map" and isinstance(v, dict):
            errs.append(f"{k}: expected a value, got a map")
        elif kind == "int" and not re.fullmatch(r"-?\d+", v):
            errs.append(f"{k}: expected an integer, got {v!r}")
        elif kind == "sha" and not HEX40.match(v):
            errs.append(f"{k}: expected a 40-hex commit id")
        elif kind == "sha256" and not HEX64.match(v):
            errs.append(f"{k}: expected a sha256")
    for k in m:
        if k not in SCHEMA:
            errs.append(f"unknown field: {k}")
    if errs:
        return None, errs, warns

    run_dir = os.path.dirname(os.path.abspath(path))
    if m["schema"] != SCHEMA_ID:
        errs.append(f"schema: expected {SCHEMA_ID}")
    if m["id"] != os.path.basename(run_dir) or not ID_RE.match(m["id"]):
        errs.append(f"id: must be the run folder name, lowercase-dashed ({os.path.basename(run_dir)})")
    if "{seed}" not in m["command"]:
        errs.append("command: must take the seed as {seed}")
    for k in NON_EMPTY_MAPS:
        if not m[k]:
            errs.append(f"{k}: must not be empty")
    for k in ("code", "data", "outputs"):
        for p, h in m[k].items():
            if not HEX64.match(h):
                errs.append(f"{k}.{p}: expected a sha256")

    r = subprocess.run(["git", "-C", root, "cat-file", "-e", m["code_commit"] + "^{commit}"],
                       capture_output=True)
    if r.returncode != 0:
        errs.append(f"code_commit {m['code_commit'][:12]} not in git history")

    pf = os.path.join(run_dir, m["params_file"])
    if not os.path.isfile(pf):
        errs.append(f"params_file missing: {m['params_file']}")
    elif sha256(pf) != m["params_sha256"]:
        errs.append(f"params_sha256 mismatch: {m['params_file']} changed")
    for k in ("code", "data"):
        for p, h in m[k].items():
            fp = os.path.join(root, p)
            if not os.path.isfile(fp):
                errs.append(f"{k} file missing: {p}")
            elif sha256(fp) != h:
                errs.append(f"{k} hash mismatch: {p}")
    for p in m["outputs"]:
        if not os.path.isfile(os.path.join(run_dir, p)):
            errs.append(f"output missing: {p}")

    now = toolchain_now(m["toolchain"])
    for name, ver in m["toolchain"].items():
        if not _same_minor(now[name], ver):
            warns.append(f"toolchain {name}: recorded {ver}, running {now[name]}")
    return m, errs, warns


# --- rerun -------------------------------------------------------------------

def rerun(path, root):
    m, errs, warns = check(path, root)
    if errs:
        return errs, warns
    run_dir = os.path.dirname(os.path.abspath(path))
    with tempfile.TemporaryDirectory() as tmp:
        shutil.copy(os.path.join(run_dir, m["params_file"]), tmp)
        cmd = m["command"].replace("{seed}", m["seed"])
        env = dict(os.environ, ASTRA_ROOT=root, PYTHONHASHSEED="0", LC_ALL="C", TZ="UTC")
        r = subprocess.run(cmd, shell=True, cwd=tmp, env=env, capture_output=True, text=True)
        if r.returncode != 0:
            return [f"command failed ({r.returncode}): {r.stderr.strip()[-400:]}"], warns
        for p, h in sorted(m["outputs"].items()):
            fp = os.path.join(tmp, p)
            if not os.path.isfile(fp):
                errs.append(f"re-run did not produce {p}")
            elif sha256(fp) != h:
                errs.append(f"output mismatch: {p} (recorded {h[:12]}, got {sha256(fp)[:12]})")
            elif sha256(os.path.join(run_dir, p)) != h:
                errs.append(f"committed {p} does not match its recorded hash")
    return errs, warns


# --- record ------------------------------------------------------------------

def record(run_dir, root):
    """Fill hashes of an existing draft manifest after running it once."""
    path = os.path.join(run_dir, "manifest.yaml")
    with open(path) as f:
        m = parse(f.read())
    m["schema"] = SCHEMA_ID
    m["id"] = os.path.basename(os.path.abspath(run_dir))
    m["code_commit"] = subprocess.run(["git", "-C", root, "rev-parse", "HEAD"], capture_output=True,
                                      text=True, check=True).stdout.strip()
    m["params_sha256"] = sha256(os.path.join(run_dir, m["params_file"]))
    for k in ("code", "data"):
        m[k] = {p: sha256(os.path.join(root, p)) for p in m.get(k, {})}
    m["toolchain"] = toolchain_now(m["toolchain"])
    cmd = m["command"].replace("{seed}", m["seed"])
    env = dict(os.environ, ASTRA_ROOT=root, PYTHONHASHSEED="0", LC_ALL="C", TZ="UTC")
    subprocess.run(cmd, shell=True, cwd=run_dir, env=env, check=True)
    m["outputs"] = {p: sha256(os.path.join(run_dir, p)) for p in m["outputs"]}
    with open(path, "w") as f:
        f.write(dump(m))
    print(f"recorded {path}")


def main(argv):
    if len(argv) < 3 or argv[1] not in ("check", "rerun", "record"):
        print(__doc__.strip(), file=sys.stderr)
        return 2
    root = repo_root()
    if argv[1] == "record":
        for d in argv[2:]:
            record(d, root)
        return 0
    bad = 0
    for p in argv[2:]:
        if argv[1] == "check":
            _, errs, warns = check(p, root)
        else:
            errs, warns = rerun(p, root)
        for w in warns:
            print(f"WARN {p}: {w}")
        for e in errs:
            print(f"FAIL {p}: {e}")
        if errs:
            bad += 1
        else:
            print(f"ok   {p}")
    print(f"runs {argv[1]}: {len(argv) - 2 - bad} ok, {bad} failed")
    return 1 if bad else 0


if __name__ == "__main__":
    sys.exit(main(sys.argv))
