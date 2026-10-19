# How to change `main`: branches, pull requests, merge queue

Read this before your first change to the repository. It applies to every
manager and every worker.

## Why

`main` is protected, and the protection applies to **everyone, administrators
included** (`enforce_admins: true`). Nobody can push to `main` directly. Every
change goes through a pull request (PR), and lands only once the two required
checks are green:

- `Rust (lints · tests)`
- `Structure (lengths · folders · chain)`

PRs are merged by GitHub's **merge queue**: it re-tests each PR on top of the
latest `main` before merging, so `main` never breaks, even when several agents
work at the same time. No human review is required (0 approvals): the checks
are the gate.

## The flow

1. **Start from an up-to-date `main`.**

   ```bash
   git switch main && git pull
   ```

2. **Create a branch**, named `<scope>/<short-description>`, e.g.
   `literature/senolytics-update`, `rust/db-indexes`, `data/new-sources`.

   ```bash
   git switch -c <scope>/<short>
   ```

   Or use the Worktree tool (`enter` with the branch name) if other agents
   are working in the same folder. This is the preferred option when you are
   not alone.

3. **Commit** as usual, with the usual prefix (`literature: ...`,
   `rust: ...`, `data: ...`, `interface: ...`, `docs: ...`, `ci: ...`).
   The local checks (git hooks) still run.

4. **Push the branch.**

   ```bash
   git push -u origin <scope>/<short>
   ```

5. **Open the PR.**

   ```bash
   gh pr create --fill --base main
   ```

6. **Ask for automatic merge.** The PR joins the merge queue as soon as its
   checks are green, then GitHub merges it (squash) by itself. Do not pass
   `--squash`: the merge queue sets the merge method and `gh` ignores it.

   ```bash
   gh pr merge --auto
   ```

7. **Follow the checks.**

   ```bash
   gh pr checks          # status of the checks on your PR
   gh pr view            # state of the PR (open, queued, merged)
   ```

8. **If a check fails**: read the failing job (`gh pr checks` gives the
   link, or `gh run view <id> --log-failed`), fix it **on the same branch**,
   commit and push again. The PR updates itself; auto-merge stays on.

9. **After the merge**: go back to `main`, update it, delete your branch.
   The merge queue squashes, so `git branch -d` refuses (your commits are
   not in `main` as such). Check that the branch adds nothing, then force
   the deletion:

   ```bash
   git switch main && git pull
   git diff --stat main <scope>/<short>   # must print nothing
   git branch -D <scope>/<short>
   ```

   GitHub deletes the remote branch after the merge (`git fetch --prune`
   clears your local copy of it).

Expected time from push to merge: about **4 minutes** observed (checks on
the PR ~1 min, then ~50 s before auto-merge adds the PR to the queue, then
the queue run ~2 min). Allow up to 10 minutes when the queue is busy.

## Hard rules

- **Never push to `main` directly.** It is refused anyway.
- **Never use `--admin`** (`gh pr merge --admin`) or any other bypass.
- **Never disable, edit or delete** the branch protection or the rulesets.
  Agents still use an administrator token, so GitHub cannot stop you
  technically: this is a rule. Any violation is reverted.
- **Never use `--no-verify`** to skip the local checks.
- **One branch per mission.** Do not mix unrelated changes in one PR.
- **Small PRs.** Smaller changes pass the checks faster and are easier to
  revert.
- **Delete the branch after the merge.**
- If something looks stuck (checks pending for more than 60 minutes, PR
  removed from the queue), report it to your manager. Do not work around it.

## Where the settings live

The settings are versioned in the repository; changing them requires the
Rust manager and the president's approval.

- Branch protection: `docs/rust/branch-protection.json`
- Merge queue ruleset: `docs/rust/merge-queue-ruleset.json`
- Details and commands: `docs/rust/lint-protection.md`, section 4
