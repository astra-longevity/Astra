//! Status page: missions of charter part 6 and the latest change in each
//! top-level folder, with its owner from `OWNERS.md`.

use std::fs;
use std::path::Path;

use minijinja::{Value, context};

use super::blob;
use super::git::{day_label, last_commit};
use super::obstacles::Obstacle;
use crate::{SiteError, io_at};

/// Charter file, relative to the repository root.
pub(crate) const CHARTER: &str = "docs/charter/charter.md";
/// Owners table, relative to the repository root.
pub(crate) const OWNERS: &str = "OWNERS.md";
/// Heading of the charter section listing the missions.
const PART_6: &str = "## Part 6";

/// One mission row: name, owner and output, as written in the charter.
#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct Mission {
    /// Mission name (`R-Metric v0`).
    name: String,
    /// Owner, as written.
    owner: String,
    /// Expected output.
    output: String,
}

/// Cells of a markdown table row, `None` for non-rows and separator rows.
fn cells(line: &str) -> Option<Vec<String>> {
    let inner = line.trim().strip_prefix('|')?.strip_suffix('|')?;
    let found: Vec<String> = inner.split('|').map(|cell| cell.trim().replace('`', "")).collect();
    let separator = found.iter().all(|cell| cell.chars().all(|chr| chr == '-' || chr == ':'));
    (!separator).then_some(found)
}

/// Missions from the `Mission | Owner | Output` table of charter part 6.
pub(crate) fn parse_missions(charter: &str) -> Vec<Mission> {
    let mut inside = false;
    let mut in_table = false;
    let mut found = Vec::new();
    for line in charter.lines() {
        if line.starts_with("## ") {
            inside = line.starts_with(PART_6);
            continue;
        }
        let Some(row) = cells(line).filter(|_| inside) else {
            continue;
        };
        if row.first().is_some_and(|first| first == "Mission") {
            in_table = true;
            continue;
        }
        match <[String; 3]>::try_from(row) {
            Ok([name, owner, output]) if in_table => found.push(Mission { name, owner, output }),
            Ok(_) | Err(_) => in_table = false,
        }
    }
    found
}

/// Agent ids (`A28`) named in an owner field.
pub(crate) fn agent_ids(owner: &str) -> Vec<String> {
    owner
        .split(|chr: char| !chr.is_ascii_alphanumeric())
        .filter(|word| word.len() >= 2 && word.starts_with('A') && word.chars().skip(1).all(|chr| chr.is_ascii_digit()))
        .map(str::to_owned)
        .collect()
}

/// Nearest open obstacle sharing an owner with `mission`.
fn next_obstacle<'obs>(mission: &Mission, obstacles: &'obs [Obstacle]) -> Option<&'obs Obstacle> {
    let ids = agent_ids(&mission.owner);
    obstacles
        .iter()
        .filter(|item| !item.is_closed() && agent_ids(item.owner()).iter().any(|id| ids.contains(id)))
        .min_by(|left, right| (&left.date(), &left.id()).cmp(&(&right.date(), &right.id())))
}

/// `missions` variable: each mission with its next obstacle date, if any.
pub(crate) fn missions(repo: &Path, obstacles: &[Obstacle]) -> Result<Vec<Value>, SiteError> {
    let path = repo.join(CHARTER);
    if !path.is_file() {
        return Ok(Vec::new());
    }
    let text = fs::read_to_string(&path).map_err(io_at(&path))?;
    Ok(parse_missions(&text)
        .iter()
        .map(|mission| {
            let next = next_obstacle(mission, obstacles);
            context! {
                name => mission.name,
                owner => mission.owner,
                output => mission.output,
                next_date => next.map(|item| day_label(item.date())).unwrap_or_default(),
                next_code => next.map(Obstacle::code).unwrap_or_default(),
                next_href => next.map(|item| blob(item.file())).unwrap_or_default(),
            }
        })
        .collect())
}

/// Owner rows of `OWNERS.md`: (paths, manager).
pub(crate) fn parse_owners(text: &str) -> Vec<(Vec<String>, String)> {
    text.lines()
        .filter_map(cells)
        .filter_map(|row| match <[String; 2]>::try_from(row) {
            Ok([paths, manager]) if paths != "Path" => {
                let list = paths.split(',').map(|part| part.trim().to_owned()).collect();
                Some((list, manager))
            }
            Ok(_) | Err(_) => None,
        })
        .collect()
}

/// Manager of top-level folder `dir`: its own row, else the `root` row.
fn owner_of(dir: &str, owners: &[(Vec<String>, String)]) -> String {
    let wanted = format!("{dir}/");
    let own =
        owners.iter().find(|row| row.0.iter().any(|path| path.split_whitespace().next() == Some(wanted.as_str())));
    let root = owners.iter().find(|row| row.0.iter().any(|path| path.starts_with("root")));
    own.or(root).map_or_else(String::new, |row| row.1.clone())
}

/// `activity` variable: last commit per top-level folder, newest first.
pub(crate) fn activity(repo: &Path) -> Result<Vec<Value>, SiteError> {
    let owners_path = repo.join(OWNERS);
    let owners = if owners_path.is_file() {
        parse_owners(&fs::read_to_string(&owners_path).map_err(io_at(&owners_path))?)
    } else {
        Vec::new()
    };
    let mut dirs = Vec::new();
    for item in fs::read_dir(repo).map_err(io_at(repo))? {
        let path = item.map_err(io_at(repo))?.path();
        let name = path.file_name().map(|os| os.to_string_lossy().into_owned()).unwrap_or_default();
        if path.is_dir() && !name.starts_with('.') && name != "target" {
            dirs.push(name);
        }
    }
    let mut rows: Vec<(String, Value)> = dirs
        .iter()
        .filter_map(|dir| {
            let commit = last_commit(repo, dir)?;
            let value = context! {
                folder => dir,
                owner => owner_of(dir, &owners),
                date => day_label(commit.date()),
                message => commit.message(),
                hash => commit.hash(),
            };
            Some((format!("{} {dir}", commit.date()), value))
        })
        .collect();
    rows.sort_by(|left, right| right.0.cmp(&left.0));
    Ok(rows.into_iter().map(|row| row.1).collect())
}

#[cfg(test)]
mod tests {
    use super::{agent_ids, owner_of, parse_missions, parse_owners};

    #[test]
    fn reads_part_6_mission_table_only() {
        let charter = "## Part 5\n\n| Mission | Owner | Output |\n|---|---|---|\n| X | A1 | no |\n\n## Part 6: org\n\n| Phase | R |\n|---|---|\n| 1 | 2 |\n\n| Mission | Owner | Output |\n|---|---|---|\n| M0 Verification | A50 | Seals |\n| P1 Data | A9 (manifests), A10 (fetch job) | `x` |\n\n## Part 7\n";
        let found = parse_missions(charter);
        assert_eq!(found.len(), 2, "two missions from part 6");
        assert_eq!(
            found.get(1).map(|mission| agent_ids(&mission.owner)),
            Some(vec!["A9".to_owned(), "A10".to_owned()]),
            "both owners"
        );
        assert_eq!(found.get(1).map(|mission| mission.output.as_str()), Some("x"), "backticks removed");
    }

    #[test]
    fn folder_owners_fall_back_to_root() {
        let owners = parse_owners(
            "| Path | Manager |\n|---|---|\n| `literature/` | A2 Literature |\n| root (files), `docs/` | A7 Coherence |\n",
        );
        assert_eq!(owner_of("literature", &owners), "A2 Literature", "own row");
        assert_eq!(owner_of("missions", &owners), "A7 Coherence", "root row");
    }
}
