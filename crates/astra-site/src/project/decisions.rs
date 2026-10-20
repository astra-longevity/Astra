//! Decisions pages: the charter, the commandments and every file of
//! `docs/decisions/`, rendered as pages, plus an index.

use std::ffi::OsStr;
use std::fs;
use std::path::{Path, PathBuf};

use minijinja::{Value, context};

use crate::docs::relative_name;
use crate::{SiteError, io_at, markdown};

/// Folders whose markdown files become decision pages, in index order.
pub(crate) const FOLDERS: [&str; 2] = ["docs/charter", "docs/decisions"];

/// Markdown files of [`FOLDERS`] (not recursive), each folder sorted.
pub(crate) fn files(repo: &Path) -> Result<Vec<PathBuf>, SiteError> {
    let mut found = Vec::new();
    for folder in FOLDERS {
        let dir = repo.join(folder);
        if !dir.is_dir() {
            continue;
        }
        let mut own = Vec::new();
        for item in fs::read_dir(&dir).map_err(io_at(&dir))? {
            let path = item.map_err(io_at(&dir))?.path();
            if path.is_file() && path.extension() == Some(OsStr::new("md")) {
                own.push(path);
            }
        }
        own.sort();
        found.extend(own);
    }
    Ok(found)
}

/// `groups` variable of the decisions index: `{ folder, label, docs[] }`.
pub(crate) fn index(repo: &Path) -> Result<Vec<Value>, SiteError> {
    let all = files(repo)?;
    FOLDERS
        .iter()
        .map(|folder| {
            let docs = all
                .iter()
                .map(|path| relative_name(repo, path))
                .filter(|file| file.rsplit_once('/').is_some_and(|pair| pair.0 == *folder))
                .map(|file| {
                    let path = repo.join(&file);
                    let text = fs::read_to_string(&path).map_err(io_at(&path))?;
                    let title =
                        text.lines().find_map(|line| line.strip_prefix("# ")).unwrap_or(&file).trim().to_owned();
                    Ok(context! { title => title, file => file, href => markdown::html_name(&file) })
                })
                .collect::<Result<Vec<Value>, SiteError>>()?;
            let label = if *folder == "docs/charter" { "Charter and commandments" } else { "Decision log and rulings" };
            Ok(context! { folder => folder, label => label, docs => docs })
        })
        .collect()
}
