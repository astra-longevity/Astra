//! `astra` command-line entry point.
//!
//! Subcommands:
//! - `astra db build [--root DIR] [--data-dir DIR] [--check] [--strict]`: validates `data/`, builds the database.
//! - `astra site build [--repo DIR] [--out DIR]`: generates the static site.
//! - `astra fetch plan|verify|pin`: P1 dataset fetch job (`data/datasets/`).
//! - `astra check sizes`: no committed file over 10 MB outside the allow-list.
//! - `astra --version`: prints the version.

use std::io::{self, Write};
use std::path::PathBuf;
use std::process::ExitCode;

use astra_site::SitePaths;

mod db;
mod fetch;

/// Version string reported by `astra`.
const VERSION: &str = env!("CARGO_PKG_VERSION");

/// Help text shown on unknown or missing commands.
const USAGE: &str = "usage:
  astra db build [--root DIR] [--data-dir DIR] [--check] [--strict]
  astra site build [--repo DIR] [--out DIR]
  astra fetch plan [--root DIR]
  astra fetch verify --cache DIR [--root DIR]
  astra fetch pin FILE
  astra check sizes [--root DIR]
  astra --version";

/// Builds the version line.
fn version_line() -> String {
    format!("astra {VERSION}")
}

/// Options of `astra site build`.
#[derive(Debug, Clone, PartialEq, Eq)]
struct SiteArgs {
    /// Repository root (default: current folder).
    repo: PathBuf,
    /// Output folder (default: `<repo>/interface/dist`).
    out: Option<PathBuf>,
}

/// Parses the options following `site build`.
fn parse_site_args(args: &[String]) -> Result<SiteArgs, String> {
    let mut parsed = SiteArgs { repo: PathBuf::from("."), out: None };
    let mut iter = args.iter();
    while let Some(flag) = iter.next() {
        let value = iter.next().ok_or_else(|| format!("missing value after {flag}"))?;
        match flag.as_str() {
            "--repo" => parsed.repo = PathBuf::from(value),
            "--out" => parsed.out = Some(PathBuf::from(value)),
            other => return Err(format!("unknown option: {other}")),
        }
    }
    Ok(parsed)
}

/// Runs `astra site build`; returns the message to print.
fn site_build(args: &[String]) -> Result<String, String> {
    let options = parse_site_args(args)?;
    let mut paths = SitePaths::from_repo(&options.repo);
    if let Some(out) = options.out {
        paths.output = out;
    }
    let report = astra_site::build(&paths).map_err(|err| err.to_string())?;
    Ok(format!("site generated: {} pages in {} (open index.html)", report.pages, paths.output.display()))
}

/// What a command prints and its exit code.
#[derive(Debug)]
pub(crate) struct Reply {
    /// Process exit code.
    code: u8,
    /// Text for stdout (may be empty).
    out: String,
    /// Text for stderr (may be empty).
    err: String,
}

impl Reply {
    /// Reply of a command returning a message or an error (exit code 1).
    fn from_result(result: Result<String, String>) -> Self {
        match result {
            Ok(out) => Self { code: 0, out, err: String::new() },
            Err(message) => Self { code: 1, out: String::new(), err: format!("astra: {message}") },
        }
    }
}

/// Dispatches the command line (without the program name).
fn run(args: &[String]) -> Reply {
    let mut words = args.iter().map(String::as_str);
    let rest = args.get(2..).unwrap_or_default();
    match (words.next(), words.next()) {
        (Some("--version" | "-V"), None) => Reply::from_result(Ok(version_line())),
        (Some("db"), Some("build")) => db::build(rest),
        (Some("site"), Some("build")) => Reply::from_result(site_build(rest)),
        (Some("fetch"), Some(action)) => fetch::fetch(action, rest),
        (Some("check"), Some("sizes")) => fetch::check_sizes(rest),
        _ => Reply { code: 2, out: String::new(), err: String::from(USAGE) },
    }
}

/// Writes `text` and a newline to `stream` unless empty; returns `false` on failure.
fn emit(stream: &mut dyn Write, text: &str) -> bool {
    text.is_empty() || writeln!(stream, "{text}").is_ok()
}

/// Entry point: prints the reply and exits with its code.
fn main() -> ExitCode {
    let args: Vec<String> = std::env::args().skip(1).collect();
    let reply = run(&args);
    let printed = emit(&mut io::stdout().lock(), &reply.out);
    let _logged = emit(&mut io::stderr().lock(), &reply.err);
    if printed { ExitCode::from(reply.code) } else { ExitCode::from(3) }
}

#[cfg(test)]
mod tests {
    use super::{parse_site_args, run, version_line};

    /// Owned argument list.
    fn args(words: &[&str]) -> Vec<String> {
        words.iter().map(|word| (*word).to_owned()).collect()
    }

    #[test]
    fn version_line_starts_with_name() {
        assert!(version_line().starts_with("astra "), "name first");
    }

    #[test]
    fn site_options_are_parsed() {
        let parsed = parse_site_args(&args(&["--repo", "r", "--out", "o"])).unwrap();
        assert_eq!(parsed.repo.to_str(), Some("r"), "repo");
        assert_eq!(parsed.out.as_deref().and_then(|path| path.to_str()), Some("o"), "out");
        assert!(parse_site_args(&args(&["--repo"])).is_err(), "missing value");
        assert!(parse_site_args(&args(&["--bad", "x"])).is_err(), "unknown flag");
    }

    #[test]
    fn unknown_command_shows_usage() {
        let usage = run(&args(&["nope"]));
        assert_eq!(usage.code, 2, "usage exit code");
        assert!(usage.err.contains("db build") && usage.err.contains("site build"), "usage");
        assert!(run(&args(&["--version"])).out.starts_with("astra "), "version");
    }
}
