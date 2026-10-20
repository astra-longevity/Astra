//! `astra fetch plan|verify|pin` (P1 dataset fetch job) and `astra check sizes`
//! (no committed file over 10 MB outside the allow-list).

use std::path::{Path, PathBuf};
use std::process::Command;

use astra_fetch::plan::Plan;
use astra_fetch::sizes::{LIMIT_BYTES, oversize, read_allow_list};
use astra_fetch::verify::{pin, verify};

use crate::Reply;

/// Allow-list of files allowed above 10 MB, relative to the repository root.
const ALLOW_LIST: &str = ".github/large-files-allow.txt";
/// Exit code of a usage error.
const USAGE_ERROR: u8 = 2;

/// Options shared by the fetch and check commands.
#[derive(Debug, Clone, PartialEq, Eq)]
struct Opts {
    /// Repository root (default: current folder).
    root: PathBuf,
    /// Download cache (`fetch verify` only).
    cache: Option<PathBuf>,
}

/// Parses `--root DIR` and `--cache DIR`.
fn parse(args: &[String]) -> Result<Opts, String> {
    let mut opts = Opts { root: PathBuf::from("."), cache: None };
    let mut iter = args.iter();
    while let Some(flag) = iter.next() {
        let value = iter.next().ok_or_else(|| format!("missing value after {flag}"))?;
        match flag.as_str() {
            "--root" => opts.root = PathBuf::from(value),
            "--cache" => opts.cache = Some(PathBuf::from(value)),
            other => return Err(format!("unknown option: {other}")),
        }
    }
    Ok(opts)
}

/// Usage-error reply.
const fn usage(message: String) -> Reply {
    Reply { code: USAGE_ERROR, out: String::new(), err: message }
}

/// Failure reply listing every problem.
fn failed(problems: &[String]) -> Reply {
    Reply { code: 1, out: String::new(), err: problems.join("\n") }
}

/// Loads the manifests under `root` and builds the plan.
fn load_plan(root: &Path) -> Result<Plan, Vec<String>> {
    astra_fetch::load_all(root).map(|manifests| Plan::new(&manifests))
}

/// Runs `astra fetch pin FILE`.
fn pin_file(args: &[String]) -> Reply {
    let single = if args.len() == 1 { args.first() } else { None };
    let Some(file) = single else {
        return usage("usage: astra fetch pin FILE".to_owned());
    };
    match pin(Path::new(file)) {
        Ok(got) => Reply {
            code: 0,
            out: format!("sha256: {}\nsize_bytes: {}\nfetched_at: {}", got.sha256, got.size_bytes, got.fetched_at),
            err: String::new(),
        },
        Err(err) => failed(&[err]),
    }
}

/// Runs `astra fetch verify` once the plan is loaded.
fn verify_cache(plan: &Plan, cache: &Path) -> Reply {
    match verify(plan, cache) {
        Ok(count) => Reply {
            code: 0,
            out: format!("{count} files verified, {} datasets skipped (licence not approved)", plan.skipped.len()),
            err: String::new(),
        },
        Err(problems) => failed(&problems),
    }
}

/// Runs `astra fetch <action>`.
pub(crate) fn fetch(action: &str, args: &[String]) -> Reply {
    if action == "pin" {
        return pin_file(args);
    }
    let opts = match parse(args) {
        Ok(parsed) => parsed,
        Err(message) => return usage(message),
    };
    let plan = match load_plan(&opts.root) {
        Ok(found) => found,
        Err(problems) => return failed(&problems),
    };
    match (action, opts.cache) {
        ("plan", _) => Reply { code: 0, out: plan.render(), err: String::new() },
        ("verify", Some(cache)) => verify_cache(&plan, &cache),
        ("verify", None) => usage("astra fetch verify needs --cache DIR".to_owned()),
        _ => usage(format!("unknown fetch action: {action}")),
    }
}

/// Files tracked by git under `root`.
fn tracked(root: &Path) -> Result<Vec<String>, String> {
    let output = Command::new("git")
        .arg("-C")
        .arg(root)
        .args(["ls-files", "-z"])
        .output()
        .map_err(|err| format!("git ls-files: {err}"))?;
    if !output.status.success() {
        return Err(format!("git ls-files: {}", String::from_utf8_lossy(&output.stderr).trim()));
    }
    Ok(String::from_utf8_lossy(&output.stdout).split('\0').filter(|s| !s.is_empty()).map(str::to_owned).collect())
}

/// Runs `astra check sizes`.
pub(crate) fn check_sizes(args: &[String]) -> Reply {
    let opts = match parse(args) {
        Ok(parsed) => parsed,
        Err(message) => return usage(message),
    };
    let found =
        tracked(&opts.root).and_then(|files| read_allow_list(&opts.root.join(ALLOW_LIST)).map(|allow| (files, allow)));
    let (files, allow) = match found {
        Ok(pair) => pair,
        Err(err) => return failed(&[err]),
    };
    let big = oversize(&opts.root, &files, &allow, LIMIT_BYTES);
    if big.is_empty() {
        return Reply { code: 0, out: format!("{} files, none over 10 MB", files.len()), err: String::new() };
    }
    let mut lines: Vec<String> =
        big.iter().map(|item| format!("{}: {} bytes (limit {LIMIT_BYTES})", item.0, item.1)).collect();
    lines.push(format!(
        "Committed files over 10 MB. Fetch them with a data/datasets manifest instead, or add the path to {ALLOW_LIST} \
         with a reason."
    ));
    failed(&lines)
}

#[cfg(test)]
mod tests {
    use super::{check_sizes, fetch, parse};

    /// Owned argument list.
    fn args(words: &[&str]) -> Vec<String> {
        words.iter().map(|word| (*word).to_owned()).collect()
    }

    /// Repository root of this worktree.
    const ROOT: &str = concat!(env!("CARGO_MANIFEST_DIR"), "/../..");

    #[test]
    fn options_are_parsed() {
        let opts = parse(&args(&["--root", "r", "--cache", "c"])).unwrap();
        assert_eq!(opts.root.to_str(), Some("r"), "root");
        assert_eq!(opts.cache.as_deref().and_then(|path| path.to_str()), Some("c"), "cache");
        assert!(parse(&args(&["--root"])).is_err(), "missing value");
        assert!(parse(&args(&["--bad", "x"])).is_err(), "unknown flag");
    }

    #[test]
    fn usage_errors() {
        assert_eq!(fetch("verify", &args(&["--root", ROOT])).code, 2, "verify without cache");
        assert_eq!(fetch("pin", &[]).code, 2, "pin without file");
        assert_eq!(fetch("nope", &args(&["--root", ROOT])).code, 2, "unknown action");
    }

    #[test]
    fn repository_plan_and_sizes_pass() {
        // Real data/datasets manifests, validated against data/schema/dataset.schema.json.
        let plan = fetch("plan", &args(&["--root", ROOT]));
        assert_eq!(plan.code, 0, "plan: {}", plan.err);
        assert!(plan.out.contains("unpinned\tmpd-itp1\t0\t2\t"), "mpd-itp1 with its 2 s interval: {}", plan.out);
        let sizes = check_sizes(&args(&["--root", ROOT]));
        assert_eq!(sizes.code, 0, "sizes: {}", sizes.err);
    }
}
