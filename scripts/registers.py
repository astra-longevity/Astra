#!/usr/bin/env python3
"""registers.py: issue register (issues/) and obstacle register (research/obstacles/).

  python3 scripts/registers.py issues                 # schema check of issues/*.yaml
  python3 scripts/registers.py gate <target>...       # exit 1 if an open S1/S2 hits a target
  python3 scripts/registers.py obstacles --schema-only
  python3 scripts/registers.py obstacles [--today YYYY-MM-DD]
                                                      # run every due check, print issue-ready
                                                      # report of missed/failed exits

Common option: --root <dir> (default: git top level).
Exit 0 = ok, 1 = schema error / blocked / exit missed or failed, 2 = usage.
Never writes to the repository. Stdlib only; files use a flat `key: value`
YAML subset (one key per line, no nesting).

Charter part 8: no sealed output ships while an S1 or S2 on it is open.
Charter part 9 + T8-A10-1: every obstacle has an owner, an exit, a `check:`
and a date; a missed or failed exit must raise an alarm.
"""

import datetime as dt
import os
import re
import subprocess
import sys

ID_RE = re.compile(r"^[a-z0-9][a-z0-9-]*$")
DATE_RE = re.compile(r"^\d{4}-\d{2}-\d{2}$")

ISSUE_FIELDS = ("id", "title", "severity", "target", "opened", "status", "owner")
ISSUE_OPTIONAL = ("closed", "evidence", "note")
OBSTACLE_FIELDS = ("id", "title", "owner", "exit", "check", "date", "status")
OBSTACLE_OPTIONAL = ("judge", "evidence", "note", "milestone", "milestone_date",
                     "milestone_status")
CHECK_TIMEOUT_S = 600


# --- parsing -------------------------------------------------------------------

def _scalar(v):
    v = v.strip()
    if len(v) >= 2 and v[0] == v[-1] and v[0] in "\"'":
        return v[1:-1]
    return v


def parse_flat(text):
    out = {}
    for n, raw in enumerate(text.splitlines(), 1):
        if not raw.strip() or raw.lstrip().startswith("#"):
            continue
        if raw[0] in " \t" or ":" not in raw:
            raise ValueError(f"line {n}: expected top-level 'key: value'")
        k, v = raw.split(":", 1)
        k = k.strip()
        if k in out:
            raise ValueError(f"line {n}: duplicate key {k}")
        out[k] = _scalar(v)
    return out


def _date(v):
    if not DATE_RE.match(v):
        return None
    try:
        return dt.date.fromisoformat(v)
    except ValueError:
        return None


def load_dir(root, rel, required, optional, validate):
    """Returns (records, errors). Every *.yaml under root/rel, recursively
    (sub-folders group files by unit; the structure check allows <= 8 entries
    per folder). Ids are unique across the whole register."""
    d = os.path.join(root, rel)
    recs, errs = [], []
    if not os.path.isdir(d):
        return recs, errs
    files = []
    for dirpath, dirnames, names in os.walk(d):
        dirnames.sort()
        files += [os.path.join(dirpath, n) for n in names if n.endswith(".yaml")]
    seen = {}
    for full in sorted(files):
        name = os.path.basename(full)
        path = os.path.relpath(full, root)
        if name in seen:
            errs.append(f"{path}: duplicate id {name[:-5]} (also {seen[name]})")
            continue
        seen[name] = path
        try:
            with open(full) as f:
                r = parse_flat(f.read())
        except (OSError, ValueError) as e:
            errs.append(f"{path}: {e}")
            continue
        e = [f"missing field: {k}" for k in required if not r.get(k)]
        e += [f"unknown field: {k}" for k in r if k not in required + optional]
        if not e:
            if r["id"] != name[:-5] or not ID_RE.match(r["id"]):
                e.append(f"id must equal the file name, lowercase-dashed ({name[:-5]})")
            e += validate(r)
        errs += [f"{path}: {x}" for x in e]
        if not e:
            r["_path"] = path
            recs.append(r)
    return recs, errs


def _validate_issue(r):
    e = []
    if r["severity"] not in ("S1", "S2", "S3"):
        e.append("severity: S1 | S2 | S3")
    if r["status"] not in ("open", "closed"):
        e.append("status: open | closed")
    opened = _date(r["opened"])
    if not opened:
        e.append("opened: YYYY-MM-DD")
    if r["status"] == "closed":
        closed = _date(r.get("closed", ""))
        if not closed:
            e.append("closed: YYYY-MM-DD required when status is closed")
        elif opened and closed < opened:
            e.append("closed is before opened")
    elif "closed" in r:
        e.append("closed: only when status is closed")
    return e


def _validate_obstacle(r):
    e = []
    if r["status"] not in ("open", "met", "failed", "unowned"):
        e.append("status: open | met | failed | unowned")
    if r["date"] == "null":
        if r["status"] != "unowned":
            e.append("date: null only with status: unowned (T8-A1-2)")
    elif not _date(r["date"]):
        e.append("date: YYYY-MM-DD, or null with status: unowned (no relative dates, T8-A1-2)")
    elif r["status"] == "unowned":
        e.append("status: unowned only with date: null")
    has_ms = [k for k in ("milestone", "milestone_date") if r.get(k)]
    if len(has_ms) == 1:
        e.append("milestone and milestone_date go together")
    if r.get("milestone_date") and not _date(r["milestone_date"]):
        e.append("milestone_date: YYYY-MM-DD")
    if r.get("milestone_status", "open") not in ("open", "met"):
        e.append("milestone_status: open | met")
    if "milestone_status" in r and not has_ms:
        e.append("milestone_status without a milestone")
    c = r["check"]
    if c.startswith("job:"):
        if not c[4:].strip():
            e.append("check: job:<workflow job name>")
    elif c.startswith("judge:"):
        if not r.get("evidence"):
            e.append("check judge:<agent> needs an evidence: file (T8-A10-1)")
    return e


def load_issues(root):
    return load_dir(root, "issues", ISSUE_FIELDS, ISSUE_OPTIONAL, _validate_issue)


def load_obstacles(root):
    return load_dir(root, "research/obstacles", OBSTACLE_FIELDS, OBSTACLE_OPTIONAL,
                    _validate_obstacle)


# --- gate ----------------------------------------------------------------------

def _hits(issue_target, target):
    """Same id/path, or one is a folder containing the other."""
    a, b = issue_target.rstrip("/"), target.rstrip("/")
    return a == b or b.startswith(a + "/") or a.startswith(b + "/")


def gate(root, targets):
    issues, errs = load_issues(root)
    if errs:
        for e in errs:
            print(f"FAIL {e}")
        print("gate: issue register invalid; refusing to seal/publish")
        return 1
    blocking = [i for i in issues if i["status"] == "open" and i["severity"] in ("S1", "S2")
                and any(_hits(i["target"], t) for t in targets)]
    for i in blocking:
        print(f"BLOCKED by {i['_path']}: {i['severity']} '{i['title']}' on {i['target']} "
              f"(opened {i['opened']}, owner {i['owner']})")
    if blocking:
        print(f"gate: {len(blocking)} open S1/S2 issue(s) on {' '.join(targets)}; "
              "nothing sealed or published")
        return 1
    print(f"gate: no open S1/S2 issue on {' '.join(targets)}")
    return 0


# --- obstacles -----------------------------------------------------------------

def run_check(root, check):
    """Returns (status, detail): status in pass | fail | manual."""
    if check.startswith("job:"):
        return "manual", f"CI job '{check[4:].strip()}': check its last run on main"
    if check.startswith("judge:"):
        return "manual", f"judged by {check[6:].strip()} from the evidence file"
    try:
        r = subprocess.run(check, shell=True, cwd=root, capture_output=True, text=True,
                           timeout=CHECK_TIMEOUT_S)
    except subprocess.TimeoutExpired:
        return "fail", f"timed out after {CHECK_TIMEOUT_S}s"
    tail = (r.stdout + r.stderr).strip()[-600:]
    return ("pass" if r.returncode == 0 else "fail"), f"exit {r.returncode}\n{tail}"


def report_block(o, today, status, detail):
    heads = {"fail": "FAILED", "manual": "NEEDS CHECK", "unowned": "UNOWNED",
             "milestone": "MILESTONE DUE"}
    if status == "unowned":
        when = "none (no absolute date: unowned, T8-A1-2)"
    elif status == "milestone":
        late = (today - dt.date.fromisoformat(o["milestone_date"])).days
        when = (f"milestone {o['milestone_date']} ({late} day(s) past): {o['milestone']}; "
                f"exit date {o['date']}")
    else:
        late = (today - dt.date.fromisoformat(o["date"])).days
        when = f"{o['date']} ({late} day(s) past)"
    return "\n".join([
        f"### [{heads[status]}] Obstacle {o['id']}: {o['title']}",
        "",
        f"- Owner: {o['owner']}",
        f"- Exit: {o['exit']}",
        f"- Date: {when}",
        f"- Check: `{o['check']}`",
        f"- File: `{o['_path']}`",
        "",
        "```",
        detail,
        "```",
        "",
    ])


def obstacles(root, today, schema_only):
    recs, errs = load_obstacles(root)
    for e in errs:
        print(f"FAIL {e}")
    if schema_only or errs:
        print(f"obstacles: {len(recs)} valid, {len(errs)} error(s)")
        return 1 if errs else 0
    reports = []
    for o in recs:
        if o["status"] in ("met", "failed"):
            continue
        ms_due = (o.get("milestone_date") and o.get("milestone_status", "open") != "met"
                  and dt.date.fromisoformat(o["milestone_date"]) <= today)
        if ms_due:
            print(f"ms-due {o['id']} (milestone {o['milestone_date']})")
            reports.append(report_block(o, today, "milestone",
                                        "milestone judged by A50; set milestone_status: met"))
        if o["status"] == "unowned":
            print(f"unowned {o['id']}")
            reports.append(report_block(o, today, "unowned", "give the exit an absolute date"))
            continue
        if dt.date.fromisoformat(o["date"]) > today:
            continue
        status, detail = run_check(root, o["check"])
        print(f"{status:6} {o['id']} (due {o['date']})")
        if status != "pass":
            reports.append(report_block(o, today, status, detail))
    if reports:
        print("")
        print(f"## Obstacle register: {len(reports)} exit(s) missed or failing ({today})")
        print("")
        print("\n".join(reports))
        print("Open one issue per block (or update the existing one). "
              "Never move an exit date after the fact (T8-A28-1).")
        return 1
    print(f"obstacles: nothing missed or failing ({today})")
    return 0


# --- cli -----------------------------------------------------------------------

def main(argv):
    args = argv[1:]
    root, today, schema_only = None, dt.date.today(), False
    rest = []
    i = 0
    while i < len(args):
        a = args[i]
        if a == "--root" and i + 1 < len(args):
            root, i = args[i + 1], i + 2
        elif a == "--today" and i + 1 < len(args):
            today = _date(args[i + 1])
            if not today:
                print("--today: YYYY-MM-DD", file=sys.stderr)
                return 2
            i += 2
        elif a == "--schema-only":
            schema_only, i = True, i + 1
        else:
            rest.append(a)
            i += 1
    if not rest or rest[0] not in ("issues", "gate", "obstacles"):
        print(__doc__.strip(), file=sys.stderr)
        return 2
    if root is None:
        root = subprocess.run(["git", "rev-parse", "--show-toplevel"], capture_output=True,
                              text=True, check=True).stdout.strip()
    cmd = rest[0]
    if cmd == "issues":
        recs, errs = load_issues(root)
        for e in errs:
            print(f"FAIL {e}")
        print(f"issues: {len(recs)} valid, {len(errs)} error(s)")
        return 1 if errs else 0
    if cmd == "gate":
        if len(rest) < 2:
            print("gate: give at least one target (path or sealed id)", file=sys.stderr)
            return 2
        return gate(root, rest[1:])
    return obstacles(root, today, schema_only)


if __name__ == "__main__":
    sys.exit(main(sys.argv))
