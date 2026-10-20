//! Reproducible runs: one folder per run under `research/runs/`, described
//! by its `manifest.yaml`. Listed only; charts come later.

use std::fs;
use std::path::Path;

use minijinja::{Value, context};

use super::scalar;
use crate::project::blob;
use crate::{SiteError, io_at};

/// Folder of the runs, relative to the repository root.
pub(crate) const DIR: &str = "research/runs";

/// `{ id, description, seed, outputs[], href }` from one manifest.
fn run_value(text: &str, folder: &str) -> Value {
    let mut id = String::new();
    let mut description = String::new();
    let mut seed = String::new();
    let mut outputs = Vec::new();
    let mut section = String::new();
    for line in text.lines() {
        let Some((key, value)) = line.split_once(':') else {
            continue;
        };
        if !line.starts_with(' ') {
            key.trim().clone_into(&mut section);
        }
        match (section.as_str(), key.trim(), line.starts_with(' ')) {
            ("id", _, false) => id = scalar(value),
            ("description", _, false) => description = scalar(value),
            ("seed", _, false) => seed = scalar(value),
            ("outputs", name, true) => {
                outputs.push(context! { name => name, href => blob(&format!("{folder}/{name}")) });
            }
            _ => {}
        }
    }
    context! { id => id, description => description, seed => seed, outputs => outputs, href => blob(&format!("{folder}/manifest.yaml")) }
}

/// Every run folder holding a manifest, sorted by folder name.
pub(crate) fn load(repo: &Path) -> Result<Vec<Value>, SiteError> {
    let dir = repo.join(DIR);
    if !dir.is_dir() {
        return Ok(Vec::new());
    }
    let mut names = Vec::new();
    for item in fs::read_dir(&dir).map_err(io_at(&dir))? {
        let path = item.map_err(io_at(&dir))?.path();
        if path.join("manifest.yaml").is_file() {
            names.push(path.file_name().map(|name| name.to_string_lossy().into_owned()).unwrap_or_default());
        }
    }
    names.sort();
    names
        .iter()
        .map(|name| {
            let path = dir.join(name).join("manifest.yaml");
            let text = fs::read_to_string(&path).map_err(io_at(&path))?;
            Ok(run_value(&text, &format!("{DIR}/{name}")))
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::run_value;

    #[test]
    fn manifest_fields() {
        let text = "schema: astra-run-manifest/1\nid: toy\ndescription: Toy run\nseed: 42\ncode:\n  a.py: abc\noutputs:\n  output.csv: 7f\n";
        let value = run_value(text, "research/runs/toy");
        let shown = format!("{value:?}");
        assert!(shown.contains("Toy run") && shown.contains("42"), "fields read");
        assert!(shown.contains("research/runs/toy/output.csv"), "output linked");
        assert!(!shown.contains("a.py"), "code pins not listed as outputs");
    }
}
