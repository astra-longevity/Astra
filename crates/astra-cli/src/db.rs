//! `astra db build`: validates `data/` and builds `data/astra.sqlite` (`data/schema/BUILD_SPEC.md`).

use std::path::PathBuf;

use astra_db::{Failure, Options, Outcome, rules};

use crate::Reply;

/// Exit code of a usage error.
const USAGE_ERROR: u8 = 2;

/// Parses the options following `db build`.
fn parse(args: &[String]) -> Result<Options, String> {
    let mut opts = Options { root: PathBuf::from("."), data_dir: "data".to_owned(), check: false, strict: false };
    let mut iter = args.iter();
    while let Some(flag) = iter.next() {
        match flag.as_str() {
            "--check" => opts.check = true,
            "--strict" => opts.strict = true,
            "--root" | "--data-dir" => {
                let value = iter.next().ok_or_else(|| format!("missing value after {flag}"))?;
                if flag == "--root" {
                    opts.root = PathBuf::from(value);
                } else {
                    opts.data_dir.clone_from(value);
                }
            }
            other => return Err(format!("unknown option: {other}")),
        }
    }
    Ok(opts)
}

/// Stdout of a successful build: one `table: N` line per table, then a summary.
fn success(opts: &Options, outcome: &Outcome) -> String {
    let mut lines: Vec<String> = outcome.counts.iter().map(|&(table, count)| format!("{table}: {count}")).collect();
    let warnings = outcome.diags.warnings();
    let summary = if opts.check {
        format!("data valid ({warnings} warnings)")
    } else {
        format!("{}/{} written ({warnings} warnings)", opts.data_dir, astra_db::DATABASE_FILE)
    };
    lines.push(summary);
    lines.join("\n")
}

/// Runs `astra db build`.
pub(crate) fn build(args: &[String]) -> Reply {
    let opts = match parse(args) {
        Ok(parsed) => parsed,
        Err(message) => return Reply { code: USAGE_ERROR, out: String::new(), err: message },
    };
    match astra_db::build(&opts, &rules::today()) {
        Ok(outcome) => {
            let err = render(&outcome.diags);
            Reply { code: 0, out: success(&opts, &outcome), err }
        }
        Err(failure) => {
            let code = failure.exit_code();
            let err = match failure {
                Failure::Invalid(diags) => render(&diags),
                Failure::Usage(message) | Failure::Internal(message) => message,
            };
            Reply { code, out: String::new(), err }
        }
    }
}

/// One line per diagnostic.
fn render(diags: &astra_db::diag::Diags) -> String {
    diags.items.iter().map(astra_db::diag::Diag::render).collect::<Vec<String>>().join("\n")
}

#[cfg(test)]
mod tests {
    use super::{build, parse};

    /// Owned argument list.
    fn args(words: &[&str]) -> Vec<String> {
        words.iter().map(|word| (*word).to_owned()).collect()
    }

    #[test]
    fn options_are_parsed() {
        let opts = parse(&args(&["--root", "r", "--data-dir", "d", "--check", "--strict"])).unwrap();
        assert_eq!(opts.root.to_str(), Some("r"), "root");
        assert_eq!(opts.data_dir, "d", "data dir");
        assert!(opts.check && opts.strict, "flags");
        assert!(parse(&args(&["--root"])).is_err(), "missing value");
        assert!(parse(&args(&["--bad"])).is_err(), "unknown flag");
    }

    #[test]
    fn examples_check_succeeds() {
        let root = concat!(env!("CARGO_MANIFEST_DIR"), "/../..");
        let reply = build(&args(&["--root", root, "--data-dir", "data/examples", "--check"]));
        assert_eq!(reply.code, 0, "exit code: {}", reply.err);
        assert!(reply.out.contains("claims: 2"), "counts: {}", reply.out);
        assert!(reply.out.ends_with("warnings)"), "summary: {}", reply.out);
        assert_eq!(build(&args(&["--bad"])).code, 2, "usage error");
    }
}
