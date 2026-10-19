//! Markdown reads: the hallmark map (from the overview of `literature/aging-map`)
//! and the "to verify" marks left in `literature/`.

use std::ffi::OsStr;
use std::fs;
use std::path::{Path, PathBuf};

use minijinja::{Value, context};
use rusqlite::Connection;

use crate::{SiteError, io_at, markdown, query};

/// Folder of the hallmark notes.
const OVERVIEW_DIR: &str = "literature/aging-map";

/// Overview listing the hallmarks by family.
const OVERVIEW: &str = "literature/aging-map/00-overview.md";

/// Marks searched in the literature (lowercase): English, and the older French form.
const TO_VERIFY: [&str; 2] = ["to verify", "\u{e0} v\u{e9}rifier"];

/// Longest excerpt kept for one mark, in characters.
const EXCERPT_CHARS: usize = 280;

/// Em dash used in overview headings.
const DASH: &str = " \u{2014} ";

/// One family of hallmarks.
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub(crate) struct Family {
    /// Family name.
    name: String,
    /// First paragraph under the heading.
    description: String,
    /// Hallmarks, in overview order.
    hallmarks: Vec<Hallmark>,
}

/// One hallmark as listed in the overview.
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub(crate) struct Hallmark {
    /// Number, as written (`01`).
    number: String,
    /// Display name.
    name: String,
    /// Concept id (file name without number and extension).
    concept: String,
    /// Markdown file name, relative to the overview folder.
    file: String,
    /// One-paragraph plain summary.
    summary: String,
}

/// A mission and its folder, to attribute marks to missions.
#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct MissionDir {
    /// Mission id.
    id: String,
    /// Folder relative to the repository root.
    folder: String,
}

impl MissionDir {
    /// Mission `id` stored under `folder`.
    pub(crate) const fn new(id: String, folder: String) -> Self {
        Self { id, folder }
    }
}

/// `families` variable of the map page; empty if the overview is absent.
pub(crate) fn families(conn: &Connection, repo: &Path) -> Result<Vec<Value>, SiteError> {
    let path = repo.join(OVERVIEW);
    if !path.is_file() {
        return Ok(Vec::new());
    }
    let text = fs::read_to_string(&path).map_err(io_at(&path))?;
    parse_overview(&text).iter().map(|family| family_value(conn, repo, family)).collect()
}

/// Template value of one family, with database counters for each hallmark.
fn family_value(conn: &Connection, repo: &Path, family: &Family) -> Result<Value, SiteError> {
    let hallmarks = family
        .hallmarks
        .iter()
        .map(|item| hallmark_value(conn, repo, item))
        .collect::<Result<Vec<Value>, SiteError>>()?;
    Ok(context! { name => family.name, description => family.description, hallmarks => hallmarks })
}

/// Template value of one hallmark.
fn hallmark_value(conn: &Connection, repo: &Path, item: &Hallmark) -> Result<Value, SiteError> {
    let stats = query::one(conn, query::HALLMARK, &[item.concept.as_str()])?;
    Ok(context! {
        number => item.number,
        name => item.name,
        summary => item.summary,
        doc_href => doc_href(repo, &item.file),
        ..stats
    })
}

/// Link to the rendered page of an overview note, empty if the note is absent.
fn doc_href(repo: &Path, file: &str) -> String {
    let note = format!("{OVERVIEW_DIR}/{file}");
    if repo.join(&note).is_file() { markdown::html_name(&note) } else { String::new() }
}

/// Reads families (`### Group N — name`) and their hallmark bullets.
pub(crate) fn parse_overview(text: &str) -> Vec<Family> {
    let mut found: Vec<Family> = Vec::new();
    let mut inside = false;
    for line in text.lines() {
        if line.starts_with('#') {
            inside = start_family(line, &mut found);
        } else if inside {
            add_line(line, &mut found);
        } else {
            // Text outside the family sections is not shown on the map.
        }
    }
    found
}

/// Opens a family if `line` is a family heading; returns whether it did.
fn start_family(line: &str, found: &mut Vec<Family>) -> bool {
    let name = line
        .strip_prefix("### Group ")
        .or_else(|| line.strip_prefix("### Groupe "))
        .and_then(|rest| rest.split_once(DASH))
        .map(|pair| pair.1.trim());
    name.is_some_and(|title| {
        found.push(Family { name: title.to_owned(), ..Family::default() });
        true
    })
}

/// Adds a hallmark bullet, or the first paragraph as description.
fn add_line(line: &str, found: &mut [Family]) {
    let Some(family) = found.last_mut() else {
        return;
    };
    let trimmed = line.trim();
    match parse_item(trimmed) {
        Some(item) => family.hallmarks.push(item),
        None => {
            if family.description.is_empty() && family.hallmarks.is_empty() && !trimmed.is_empty() {
                trimmed.clone_into(&mut family.description);
            }
        }
    }
}

/// Parses `- **[01 — Name](01-concept-id.md)** : summary`.
pub(crate) fn parse_item(line: &str) -> Option<Hallmark> {
    let rest = line.strip_prefix("- **[")?;
    let (label, after_label) = rest.split_once("](")?;
    let (file, after_link) = after_label.split_once(")**")?;
    let (number, name) = label.split_once(DASH)?;
    let stem = file.strip_suffix(".md")?;
    let concept = stem.split_once('-').map_or(stem, |pair| pair.1);
    let summary = after_link.trim_start_matches([' ', ':']).replace('*', "");
    Some(Hallmark {
        number: number.trim().to_owned(),
        name: name.trim().to_owned(),
        concept: concept.to_owned(),
        file: file.to_owned(),
        summary: summary.trim().to_owned(),
    })
}

/// `to_verify` variable: every line of `literature/**/*.md` marked "to verify".
pub(crate) fn to_verify(repo: &Path, missions: &[MissionDir]) -> Result<Vec<Value>, SiteError> {
    let mut marks = Vec::new();
    for path in literature_files(repo)? {
        let text = fs::read_to_string(&path).map_err(io_at(&path))?;
        let relative = relative_name(repo, &path);
        let mission = mission_of(&relative, missions);
        marks.extend(marks_in(&text, &relative, &mission));
    }
    Ok(marks)
}

/// Every `literature/**/*.md`, sorted.
pub(crate) fn literature_files(repo: &Path) -> Result<Vec<PathBuf>, SiteError> {
    let root = repo.join("literature");
    let mut files = Vec::new();
    if root.is_dir() {
        collect_markdown(&root, &mut files)?;
    }
    files.sort();
    Ok(files)
}

/// Marks found in one file.
pub(crate) fn marks_in(text: &str, file: &str, mission: &str) -> Vec<Value> {
    text.lines()
        .zip(1usize..)
        .filter(|pair| {
            let lower = pair.0.to_lowercase();
            TO_VERIFY.iter().any(|mark| lower.contains(mark))
        })
        .map(|pair| {
            let excerpt: String = pair.0.trim().chars().take(EXCERPT_CHARS).collect();
            context! { text => excerpt, file => file, line => pair.1, mission => mission }
        })
        .collect()
}

/// Mission whose folder contains `file`, empty if none.
fn mission_of(file: &str, missions: &[MissionDir]) -> String {
    missions
        .iter()
        .find(|mission| Path::new(file).starts_with(&mission.folder))
        .map_or_else(String::new, |mission| mission.id.clone())
}

/// Path relative to the repository, with `/` separators.
pub(crate) fn relative_name(repo: &Path, path: &Path) -> String {
    let relative = path.strip_prefix(repo).unwrap_or(path);
    relative.components().map(|part| part.as_os_str().to_string_lossy().into_owned()).collect::<Vec<String>>().join("/")
}

/// Collects `*.md` files under `dir`, recursively.
fn collect_markdown(dir: &Path, files: &mut Vec<PathBuf>) -> Result<(), SiteError> {
    for item in fs::read_dir(dir).map_err(io_at(dir))? {
        let path = item.map_err(io_at(dir))?.path();
        if path.is_dir() {
            collect_markdown(&path, files)?;
        } else if path.extension() == Some(OsStr::new("md")) {
            files.push(path);
        } else {
            // Other files are not literature notes.
        }
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::{marks_in, parse_item, parse_overview};

    #[test]
    fn accepts_english_and_french_headings() {
        let item = "- **[01 \u{2014} A](01-genomic-instability.md)** : x";
        for heading in ["### Group 1 \u{2014} Damage", "### Groupe 1 \u{2014} Damage"] {
            let found = parse_overview(&format!("{heading}\n\n{item}\n"));
            assert_eq!(found.len(), 1, "{heading}");
            assert_eq!(found.first().map(|family| family.hallmarks.len()), Some(1), "{heading}");
        }
    }

    #[test]
    fn finds_both_to_verify_marks() {
        let text = "a [to verify]\nb\nc [\u{e0} v\u{e9}rifier]\n";
        assert_eq!(marks_in(text, "f.md", "").len(), 2, "English and French marks");
    }

    #[test]
    fn parses_overview_item() {
        let line = "- **[08 \u{2014} Senescence](08-cellular-senescence.md)** : a *damaged* cell.";
        let item = parse_item(line).unwrap();
        assert_eq!(item.number, "08", "number");
        assert_eq!(item.concept, "cellular-senescence", "concept id");
        assert_eq!(item.summary, "a damaged cell.", "emphasis removed");
    }
}
