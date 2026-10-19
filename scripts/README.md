# scripts/ — where are the checks?

The check scripts are not here: they live in `.github/checks/`,
the same place as in the context-pilot project whose rules Astra reuses.
This folder only documents them.

## The scripts

| File | What it checks |
|------|----------------|
| `check-rust-lints.sh` | Formatting (`cargo fmt`), `clippy` analysis with zero warnings, warning-free build, exception registry, secret access. |
| `check-rust-tests.sh` | Full build, then runs all tests. |
| `check-structure.sh` | `.rs` files ≤ 500 lines, folders ≤ 8 entries, then integrity of the protection chain. |
| `check-lint-exceptions.sh` | Every `#[allow]` / `#[expect]` in the code matches exactly one entry in `allowed-lint-exceptions.yaml`. |
| `check-vault-bypass.sh` | No secret key (`*_API_KEY`, `*_TOKEN`…) is read directly from the environment. |
| `check-lint-config.sh` | Hash chain: detects any change to the rule files listed in `protected-files.yaml`. |
| `lib-failures.sh` | Shared helper: lists every failed check at the end. |

## Running them

From the repository root:

```sh
.github/checks/check-rust-lints.sh --ci
.github/checks/check-rust-tests.sh --ci
.github/checks/check-structure.sh --ci
```

The same scripts run automatically:
- on GitHub on every push or pull request to `main` (`.github/workflows/ci.yml`);
- locally on every change to a `.rs` file, through context-pilot's blocking
  callbacks (`.context-pilot/shared/callbacks.yaml`).

## Changing the rules

The rule files are sealed by a password-protected hash chain.
After a legitimate change, the password holder runs `./chain.sh`.
Without it, the `check-structure.sh` check fails.
