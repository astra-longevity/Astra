# Rust code protection

This document explains, without needless jargon, what keeps poor-quality Rust code out of the Astra repository, and how to use it.

The big picture: Astra's Rust code (the small programs that build the database and the site) must follow very strict rules. These rules are checked **automatically**, at three points:

1. while an agent writes code (immediate local check);
2. on every push to GitHub (continuous integration, "CI");
3. before anything lands on the main branch `main` (branch protection, **active**).

Above all: **the rules themselves are sealed**. Nobody can quietly loosen them, because any change is detected until it has been approved with a password that only the president holds.

---

## 1. What protects the code

### 1.1 Pinned Rust version: 1.97.1

- **File**: `rust-toolchain.toml`
- **What it is**: Rust is the programming language used. This file forces one exact version (1.97.1) on everyone: agents, local machine, CI.
- **Why**: the list of rules (see 1.3) depends on the version. Another version would know different rules, and the check would fail.
- The file also installs two tools: `rustfmt` (formatting) and `clippy` (quality control).

### 1.2 Automatic formatting (rustfmt)

- **File**: `rustfmt.toml`
- **What it is**: a tool that enforces a single layout for the code (120-character line width, 2024 edition of the language, shorthand forms).
- **Effect**: if the code is not laid out exactly as the tool would do it, the check fails. So there is never any style debate.

### 1.3 About 1,000 forbidden rules ("lints")

- **File**: `Cargo.toml` (root), sections `[workspace.lints.rust]` and `[workspace.lints.clippy]`
- **What it is**: a "lint" is a rule that flags a risky or clumsy practice in the code. Here, nearly every known Rust and clippy rule is listed one by one, and almost all are at level `forbid` (forbidden, no local override possible). A few are at level `deny` (forbidden, but a registered exception remains technically possible).
- **Examples of what is forbidden**: leaving an error unhandled, crashing the program on purpose, undocumented code, dead code, unused variables, calculations that can overflow.
- **For each crate** (Rust sub-project in `crates/`): its `Cargo.toml` contains `[lints] workspace = true`, which forces it to inherit these rules. No crate can redefine them.

### 1.4 Complexity thresholds (clippy.toml)

- **File**: `clippy.toml`
- **What it is**: numeric limits, stricter than the defaults. The main ones:
  - a function must not exceed **60 lines**;
  - a function takes at most **4 parameters**;
  - "cognitive complexity" (how hard a function is to read) is capped at **7**;
  - variable names are at least **3 characters** long (except a short list of exceptions such as `i`, `id`);
  - vague names are forbidden (`foo`, `tmp`, `thing`…);
  - some dangerous language functions are banned (for example abruptly stopping the program).
- In **tests** only, making the program fail on purpose is allowed: that is how a test reports a problem.

### 1.5 Exception registry: empty

- **File**: `.github/checks/allowed-lint-exceptions.yaml` (contains `exceptions: []`)
- **Check**: `.github/checks/check-lint-exceptions.sh`
- **What it is**: in theory, a developer can write an annotation in the code that disables a rule at one precise spot. Here, **every** such annotation must be declared in this registry, with a justification and a plan to remove it. Otherwise the check fails.
- The registry is **empty**: today, Astra contains no exception.
- The script checks three things: no undeclared annotation, no orphan entry (declared but missing from the code), no ambiguous entry (one that would match several places).

### 1.6 File and folder size

- **Check**: `.github/checks/check-structure.sh`
- **Rules**:
  - each Rust file (`.rs`) has **at most 500 lines**;
  - each folder contains **at most 8 entries** (some folders are exempt: `docs/`, `crates/`, `.github/checks`, `.github/workflows`, build folders…).
- **Why**: to force splitting into small readable pieces rather than big catch-all files.
- The same script also checks the seal (see 1.7).

### 1.7 The seal: a password-protected chain of fingerprints

- **Files**:
  - `.github/checks/protected-files.yaml`: the list of what is sealed;
  - `.github/checks/lint-config.chain`: the record of successive seals;
  - `.github/checks/check-lint-config.sh`: the script that computes and checks;
  - `chain.sh` (root): shortcut to update the seal.
- **The idea, in a picture**: we compute a "fingerprint" (SHA-256) of all the rule files. A fingerprint is like a unique signature of some content: changing a single comma completely changes the fingerprint. Each new fingerprint is recorded in the registry, linked to the previous one **and to the password**, forming a chain.
- **Consequence**:
  - checking the seal requires **no** password (CI does it on every push);
  - if someone changes a rule without renewing the seal, the check fails;
  - renewing the seal requires the password; without it, a valid entry cannot be forged.
- **What is sealed**:
  - the `protected-files.yaml` list itself;
  - the two rule sections of the root `Cargo.toml` (not the rest of the file, so dependencies can still be added);
  - `rust-toolchain.toml`, `clippy.toml`, `rustfmt.toml`;
  - all scripts in `.github/checks/` (`check-*.sh`, `lib-failures.sh`) and the exception registry;
  - the CI file `.github/workflows/ci.yml`;
  - the local checks `.context-pilot/shared/callbacks.yaml`;
  - `chain.sh`.
- The registry currently holds **one entry** (the original one, called "genesis"), set at commit `c78eeb6`.
- **The password is never written in the repository.**

### 1.8 Continuous integration (CI) on GitHub

- **File**: `.github/workflows/ci.yml`
- **When**: on every `push` to `main` and on every pull request to `main`.
- **Two jobs**:
  - **`Rust (lints · tests)`**: installs Rust 1.97.1, then runs
    - `.github/checks/check-rust-lints.sh --ci`: formatting, clippy, build with zero warnings, exception registry, secrets check;
    - `.github/checks/check-rust-tests.sh --ci`: optimized build, then runs all tests.
  - **`Structure (lengths · folders · chain)`**: runs `.github/checks/check-structure.sh --ci` (file and folder sizes, seal).
- Each script runs **all** its sub-checks, then prints at the end the list of those that failed (shared mechanism in `.github/checks/lib-failures.sh`).
- Note: `check-vault-bypass.sh` comes from context-pilot; it forbids reading secret keys (`*_API_KEY`, `*_TOKEN`… variables) directly in the code. It is kept as is.

### 1.9 Blocking local checks

- **File**: `.context-pilot/shared/callbacks.yaml`
- **What it is**: in the tool the agents work in (context-pilot), every change to a `.rs` file immediately triggers, in a **blocking** way, the same scripts as CI:
  - `rust-lints` → `check-rust-lints.sh --callback` (120 s limit);
  - `structure-rust` → `check-structure.sh --callback` (30 s limit).
- **Why**: an agent sees its errors right away, before pushing anything. CI and the local check use **the same script**: they cannot drift apart.
- There are no "git hooks" (scripts git runs before a commit): as in context-pilot, these local checks play that role.

### 1.10 Code owners (CODEOWNERS)

- **File**: `.github/CODEOWNERS` (contains `* @bigmoostache`)
- **What it is**: says who is responsible for each part of the repository. Here, the whole repository belongs to `@bigmoostache`. This file automatically suggests that reviewer on every pull request, but review is **not required** (see section 4).

---

## 2. Running the checks yourself

All commands are run from the repository root.

| Goal | Command |
|---|---|
| Code rules (formatting, clippy, build, exceptions, secrets) | `.github/checks/check-rust-lints.sh --ci` |
| Build and tests | `.github/checks/check-rust-tests.sh --ci` |
| File/folder size + seal | `.github/checks/check-structure.sh --ci` |
| Seal only (check) | `.github/checks/check-lint-config.sh` |
| Exception registry only | `.github/checks/check-lint-exceptions.sh` |

Faster local variant: replace `--ci` with `--callback`. The only difference concerns tests: `--callback` builds in "debug" mode (faster) instead of optimized mode; test coverage is identical.

Basic commands if you want to isolate one step:

```bash
cargo fmt -- --check
cargo clippy --workspace --all-targets -- -D warnings
cargo test --workspace
```

Expected result: each script ends with a `… OK` line and exit code 0. On failure, the last line lists the failing sub-checks; details are further up, under the `=== … ===` headings.

---

## 3. The seal day to day

### 3.1 Inspecting

```bash
.github/checks/check-lint-config.sh --show          # list what is sealed + current fingerprint
.github/checks/check-lint-config.sh --history       # seal history
.github/checks/check-lint-config.sh --deep-verify   # thorough check of the whole history
```

### 3.2 Changing a rule later

Procedure, in order:

1. Edit the relevant file(s) (for example `clippy.toml`, or a rule section of `Cargo.toml`).
2. Run the checks (section 2): the seal must **fail** at this stage, that is expected; everything else must pass.
3. The password holder renews the seal:
   ```bash
   .github/checks/check-lint-config.sh --update
   ```
   The password is asked for with masked input. Equivalent alternative: `./chain.sh`.
   The script rejects a wrong password. It appends a new line to the `lint-config.chain` registry, followed by the details of the changes (to keep a readable trace).
4. Run `.github/checks/check-lint-config.sh` again: it must print "verified".
5. Make a **single** commit containing both the change and the updated registry (prefix `rust:`).

Security rules:

- the password must **never** appear in the repository, in a message, or in a log;
- avoid passing it in clear on the command line (`./chain.sh --password …` still works but leaves a trace in the terminal history); prefer masked input;
- the `LINT_GUARD_PASSWORD` environment variable is also accepted, for one-off use only.

### 3.3 Sealing an additional file

Add an entry to `.github/checks/protected-files.yaml` (format described at the top of the file), then renew the seal as in 3.2. Since the list is itself sealed, a file cannot be quietly **removed** from protection either.

---

## 4. Protection of the `main` branch (**active**)

**How to work with it day to day: [`pr-workflow.md`](pr-workflow.md)** (branch, PR, merge queue).

### 4.1 What it enforces

On GitHub, `main` cannot be changed unless these conditions are met:

- **pull request required**: nobody pushes to `main` directly; every change goes through a PR;
- **required checks**: the two CI jobs `Rust (lints · tests)` and `Structure (lengths · folders · chain)` must be green;
- **administrators included** (`enforce_admins: true`): the rules apply to everyone, with no bypass;
- **merge queue** (repository ruleset): GitHub re-tests each PR on top of the latest `main` before merging it (squash). This replaces the "up-to-date branch" setting, which is therefore off (`strict: false`);
- **no required review** (0 approvals): the checks are the gate. GitHub forbids approving your own change, so requiring a review would block everything;
- **no history rewriting** (force-push forbidden);
- **no deletion** of the branch.

Agents still use an administrator token, so they could technically change these settings. Doing so is forbidden: any change is reverted.

### 4.2 History

- A first activation attempt was **rejected by GitHub (error 403)**: on the free plan, branch protection is not available for a **private** repository.
- The repository was then made **public**, and a first protection was activated (administrators exempt, direct pushes allowed).
- The repository was transferred to the organization `astra-longevity`. Administrators are now included, PRs are required and the merge queue is on.
- The seal itself has been in place since commit `c78eeb6`.

### 4.3 Configuration and commands

The settings are versioned in the repository:

- branch protection: `docs/rust/branch-protection.json`;
- merge queue ruleset: `docs/rust/merge-queue-ruleset.json`.

```json
{
  "required_status_checks": {
    "strict": false,
    "contexts": [
      "Rust (lints · tests)",
      "Structure (lengths · folders · chain)"
    ]
  },
  "enforce_admins": true,
  "required_pull_request_reviews": {
    "required_approving_review_count": 0,
    "dismiss_stale_reviews": false,
    "require_code_owner_reviews": false
  },
  "restrictions": null,
  "allow_force_pushes": false,
  "allow_deletions": false
}
```

The ruleset targets the default branch (`~DEFAULT_BRANCH`) with one `merge_queue` rule: squash, all PRs of a group must pass (`ALLGREEN`), groups of 1 to 5 PRs, up to 5 built at once, 1 minute wait for a group, checks time out after 60 minutes. No bypass actors.

To apply them (Rust manager only, with the president's approval):

```bash
gh api -X PUT repos/astra-longevity/Astra/branches/main/protection --input docs/rust/branch-protection.json
gh api -X POST repos/astra-longevity/Astra/rulesets --input docs/rust/merge-queue-ruleset.json
```

To check them:

```bash
gh api repos/astra-longevity/Astra/branches/main/protection
gh api repos/astra-longevity/Astra/rulesets
```

Watch out: the check names must match **exactly** the `name:` fields of `ci.yml` (with the middle dot `·`), and they must have run at least once on GitHub to be recognized. `ci.yml` also runs on `merge_group`, otherwise the checks never report in the queue and every PR times out.

---

## 5. Origin

This whole setup is **copied** from the project `github.com/bigmoostache/context-pilot` (branch `master`): same rules, same thresholds, same scripts, same seal mechanism, same CI layout. Adaptations for Astra:

- only the Rust checks were kept (not the ones for context-pilot's TypeScript website);
- branch `main` instead of `master`;
- the structure CI job is called `Structure (lengths · folders · chain)` (without the "web" part);
- `rust-toolchain.toml` and `chain.sh` were added to the sealed list;
- the seal registry starts from scratch, with Astra's own password.

**Important caveat about version 1.97.1**: the list of ~1,000 rules is written for this exact Rust version. Changing versions may bring up unknown or renamed rules; they then produce a warning, and since every warning is treated as an error, CI fails. Upgrading therefore requires updating the rule list at the same time, then renewing the seal (section 3.2).
