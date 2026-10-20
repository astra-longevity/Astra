//! Obstacles of charter part 9, one flat `key: value` file each under
//! `research/obstacles/**/*.yaml`.

use std::ffi::OsStr;
use std::fs;
use std::path::{Path, PathBuf};

use minijinja::{Value, context};

use super::blob;
use super::git::day_label;
use crate::docs::relative_name;
use crate::{SiteError, io_at};

/// Folder of the obstacle files, relative to the repository root.
const DIR: &str = "research/obstacles";

/// One obstacle file.
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub(crate) struct Obstacle {
    /// Id, e.g. `o1-ageing-measure`.
    id: String,
    /// Plain title.
    title: String,
    /// Owners, as written (`A28 + A50`).
    owner: String,
    /// What counts as the obstacle being taken down.
    exit: String,
    /// Exit date, `YYYY-MM-DD`.
    date: String,
    /// Status code (`open`, `met`).
    status: String,
    /// Intermediate milestone, empty if none.
    milestone: String,
    /// Milestone date, empty if none.
    milestone_date: String,
    /// Free note.
    note: String,
    /// File, relative to the repository root.
    file: String,
}

impl Obstacle {
    /// Id, e.g. `o1-ageing-measure`.
    pub(crate) fn id(&self) -> &str {
        &self.id
    }

    /// Plain title.
    pub(crate) fn title(&self) -> &str {
        &self.title
    }

    /// Owners, as written.
    pub(crate) fn owner(&self) -> &str {
        &self.owner
    }

    /// Exit date, `YYYY-MM-DD`.
    pub(crate) fn date(&self) -> &str {
        &self.date
    }

    /// Milestone date, empty if none.
    pub(crate) fn milestone_date(&self) -> &str {
        &self.milestone_date
    }

    /// File, relative to the repository root.
    pub(crate) fn file(&self) -> &str {
        &self.file
    }

    /// True once the exit is met or the obstacle is closed.
    pub(crate) fn is_closed(&self) -> bool {
        matches!(self.status.as_str(), "met" | "closed")
    }

    /// True when the exit date has passed (before `today`) and it is not closed.
    pub(crate) fn is_late(&self, today: &str) -> bool {
        !self.is_closed() && !self.date.is_empty() && self.date.as_str() < today
    }

    /// True when the milestone date has passed and the obstacle is not closed.
    pub(crate) fn milestone_late(&self, today: &str) -> bool {
        !self.is_closed() && !self.milestone_date.is_empty() && self.milestone_date.as_str() < today
    }

    /// Short code: `o1-ageing-measure` gives `O1`.
    pub(crate) fn code(&self) -> String {
        self.id.split('-').next().unwrap_or_default().to_uppercase()
    }

    /// Template value; `today` decides lateness.
    pub(crate) fn to_value(&self, today: &str) -> Value {
        context! {
            id => self.id,
            code => self.code(),
            title => self.title,
            owner => self.owner,
            exit => self.exit,
            date => self.date,
            date_label => day_label(&self.date),
            status => status_label(&self.status),
            late => self.is_late(today),
            milestone => self.milestone,
            milestone_label => day_label(&self.milestone_date),
            milestone_late => self.milestone_late(today),
            note => self.note,
            file => self.file,
            href => blob(&self.file),
        }
    }
}

/// Plain label of a status code (never the raw code).
pub(crate) fn status_label(code: &str) -> &'static str {
    match code {
        "open" => "Open",
        "met" => "Exit met",
        "closed" => "Closed",
        _ => "Status not recognised",
    }
}

/// Parses one obstacle file; unknown keys and comments are ignored.
pub(crate) fn parse(text: &str, file: &str) -> Obstacle {
    let mut found = Obstacle { file: file.to_owned(), ..Obstacle::default() };
    for line in text.lines().filter(|line| !line.starts_with([' ', '\t', '#'])) {
        let Some((key, value)) = line.split_once(':') else {
            continue;
        };
        let slot = match key.trim() {
            "id" => &mut found.id,
            "title" => &mut found.title,
            "owner" => &mut found.owner,
            "exit" => &mut found.exit,
            "date" => &mut found.date,
            "status" => &mut found.status,
            "milestone" => &mut found.milestone,
            "milestone_date" => &mut found.milestone_date,
            "note" => &mut found.note,
            _ => continue,
        };
        value.trim().clone_into(slot);
    }
    found
}

/// Every obstacle of `repo`, sorted by exit date then id (empty if none).
pub(crate) fn load(repo: &Path) -> Result<Vec<Obstacle>, SiteError> {
    let root = repo.join(DIR);
    let mut files = Vec::new();
    if root.is_dir() {
        collect_yaml(&root, &mut files)?;
    }
    let mut found = files
        .iter()
        .map(|path| {
            let text = fs::read_to_string(path).map_err(io_at(path))?;
            Ok(parse(&text, &relative_name(repo, path)))
        })
        .collect::<Result<Vec<Obstacle>, SiteError>>()?;
    found.sort_by(|left, right| (&left.date, &left.id).cmp(&(&right.date, &right.id)));
    Ok(found)
}

/// Collects `*.yaml` files under `dir`, recursively.
fn collect_yaml(dir: &Path, files: &mut Vec<PathBuf>) -> Result<(), SiteError> {
    for item in fs::read_dir(dir).map_err(io_at(dir))? {
        let path = item.map_err(io_at(dir))?.path();
        if path.is_dir() {
            collect_yaml(&path, files)?;
        } else if path.extension() == Some(OsStr::new("yaml")) {
            files.push(path);
        } else {
            // Other files are not obstacles.
        }
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::{parse, status_label};

    /// Obstacle file in the repository format.
    const FILE: &str = "# comment: ignored\nid: v1-blind-test\ntitle: No blind test: yet\nowner: A50 + A28\ndate: 2027-01-20\nstatus: open\nmilestone_date: 2026-10-01\n";

    #[test]
    fn parses_flat_yaml() {
        let found = parse(FILE, "research/obstacles/v/v1.yaml");
        assert_eq!(found.code(), "V1", "code");
        assert_eq!(found.title, "No blind test: yet", "colon kept in value");
        assert!(!found.is_late("2026-10-20"), "exit in the future");
        assert!(found.is_late("2027-02-01"), "exit passed");
        assert!(found.milestone_late("2026-10-20"), "milestone passed");
    }

    #[test]
    fn closed_obstacles_are_never_late() {
        let found = parse("id: v4\ndate: 2026-01-01\nstatus: met\n", "f.yaml");
        assert!(!found.is_late("2026-10-20"), "met is not late");
        assert_eq!(status_label(&found.status), "Exit met", "plain label");
        assert_eq!(status_label("weird"), "Status not recognised", "no raw code");
    }
}
