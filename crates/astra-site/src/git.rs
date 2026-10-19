//! Project history read from `git log` (the journal page and build stamp).

use std::collections::BTreeSet;
use std::path::Path;
use std::process::Command;

use minijinja::{Value, context};

/// Month names, January first.
const MONTHS: [&str; 12] = [
    "January",
    "February",
    "March",
    "April",
    "May",
    "June",
    "July",
    "August",
    "September",
    "October",
    "November",
    "December",
];

/// Field separator used in the `git log` format.
const SEP: char = '\u{1f}';

/// One commit, as shown in the journal.
#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct Commit {
    /// Abbreviated hash.
    hash: String,
    /// Author name.
    author: String,
    /// Day, `YYYY-MM-DD`.
    date: String,
    /// Time, `HH:MM`.
    time: String,
    /// Subject line.
    message: String,
}

/// Commits of the repository, newest first (empty outside a git repository).
#[derive(Debug, Clone, Default)]
pub(crate) struct History {
    /// The commits.
    commits: Vec<Commit>,
}

impl History {
    /// Reads the whole history of `repo`.
    pub(crate) fn load(repo: &Path) -> Self {
        let args = ["log", "--date=format:%Y-%m-%d %H:%M", "--format=%h%x1f%an%x1f%ad%x1f%s"];
        let commits =
            run_git(repo, &args).map(|out| out.lines().filter_map(parse_commit).collect()).unwrap_or_default();
        Self { commits }
    }

    /// `site` variable: title, generation stamp (last commit, not the clock), short hash.
    pub(crate) fn site(&self) -> Value {
        let last = self.commits.first();
        let generated = last.map_or_else(
            || String::from("(no commit)"),
            |commit| format!("{} at {}", day_label(&commit.date), commit.time),
        );
        let short = last.map_or_else(|| String::from("none"), |commit| commit.hash.clone());
        context! { title => "Astra", generated_at => generated, commit_short => short }
    }

    /// The `count` most recent commits as `{ date, message }`.
    pub(crate) fn recent(&self, count: usize) -> Vec<Value> {
        self.commits
            .iter()
            .take(count)
            .map(|commit| context! { date => commit.date, message => commit.message })
            .collect()
    }

    /// Areas found in commit prefixes, as `{ id, label }`, sorted by id.
    pub(crate) fn areas(&self) -> Vec<Value> {
        let found: BTreeSet<String> = self.commits.iter().filter_map(|commit| area(&commit.message)).collect();
        found.into_iter().map(|code| context! { label => area_label(&code), id => code }).collect()
    }

    /// Commits grouped by day: `{ date, label, commits[] }`.
    pub(crate) fn days(&self) -> Vec<Value> {
        let mut groups: Vec<(String, Vec<Value>)> = Vec::new();
        for commit in &self.commits {
            let entry = commit_value(commit);
            match groups.last_mut() {
                Some(group) if group.0 == commit.date => group.1.push(entry),
                Some(_) | None => groups.push((commit.date.clone(), vec![entry])),
            }
        }
        groups
            .into_iter()
            .map(|group| context! { label => day_label(&group.0), date => group.0, commits => group.1 })
            .collect()
    }
}

/// Date (`YYYY-MM-DD`) of the last commit touching `path`, empty if none.
pub(crate) fn last_change(repo: &Path, path: &str) -> String {
    let args = ["log", "-1", "--format=%ad", "--date=short", "--", path];
    run_git(repo, &args).map_or_else(String::new, |out| out.trim().to_owned())
}

/// Runs git in `repo`; `None` if git is missing or fails.
fn run_git(repo: &Path, args: &[&str]) -> Option<String> {
    let output = Command::new("git").arg("-C").arg(repo).args(args).output().ok()?;
    output.status.success().then(|| String::from_utf8_lossy(&output.stdout).into_owned())
}

/// Parses one `hash SEP author SEP date time SEP subject` line.
pub(crate) fn parse_commit(line: &str) -> Option<Commit> {
    let mut fields = line.splitn(4, SEP);
    let hash = fields.next()?.to_owned();
    let author = fields.next()?.to_owned();
    let (date, time) = fields.next()?.split_once(' ')?;
    let message = fields.next()?.to_owned();
    Some(Commit { hash, author, date: date.to_owned(), time: time.to_owned(), message })
}

/// Journal entry for one commit.
fn commit_value(commit: &Commit) -> Value {
    let code = area(&commit.message);
    let label = code.as_deref().map(area_label);
    context! {
        time => commit.time,
        hash_short => commit.hash,
        author => commit.author,
        message => commit.message,
        area => code,
        area_label => label,
    }
}

/// Area of a commit: its lowercase prefix before `:` (`data: ...` gives `data`).
pub(crate) fn area(message: &str) -> Option<String> {
    let (prefix, _) = message.split_once(':')?;
    let valid =
        !prefix.is_empty() && prefix.len() <= 20 && prefix.chars().all(|chr| chr.is_ascii_lowercase() || chr == '-');
    valid.then(|| prefix.to_owned())
}

/// Plain-language label of an area.
pub(crate) fn area_label(code: &str) -> String {
    let label = match code {
        "data" => "Data",
        "rust" => "Software",
        "literature" => "Literature",
        "interface" => "Website",
        "structure" => "Organization",
        "docs" => "Documentation",
        other => other,
    };
    label.to_owned()
}

/// `2026-10-19` gives `October 19, 2026`; anything else is returned unchanged.
pub(crate) fn day_label(date: &str) -> String {
    let mut parts = date.split('-');
    let parsed = (|| {
        let year = parts.next()?;
        let month: usize = parts.next()?.parse().ok()?;
        let day: u32 = parts.next()?.parse().ok()?;
        let name = MONTHS.get(month.checked_sub(1)?)?;
        Some(format!("{name} {day}, {year}"))
    })();
    parsed.unwrap_or_else(|| date.to_owned())
}

#[cfg(test)]
mod tests {
    use super::{area, day_label, parse_commit};

    #[test]
    fn parses_commits_and_dates() {
        let commit = parse_commit("abc1234\u{1f}Ann\u{1f}2026-10-19 15:04\u{1f}data: add schema").unwrap();
        assert_eq!(commit.time, "15:04", "time");
        assert_eq!(area(&commit.message).as_deref(), Some("data"), "area");
        assert_eq!(area("Initial commit"), None, "no prefix");
        assert_eq!(day_label("2026-08-01"), "August 1, 2026", "label");
    }
}
