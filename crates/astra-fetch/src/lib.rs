//! Dataset manifests (`data/datasets/<id>.yaml`, owned by A9) and the P1 fetch job.
//!
//! The repository never holds external datasets: a manifest names each file
//! (URL, expected sha256, size, pin date) and its licence status. The fetch
//! workflow downloads only files whose licence Verification approved, and this
//! crate checks every download against its pinned hash and size. It also holds
//! the "no committed file over 10 MB" check.

pub mod plan;
pub mod sizes;
#[cfg(test)]
mod tests;
pub mod verify;

use std::fs;
use std::path::Path;

use astra_db::{schema, yaml};
use serde_json::Value;

/// Folder of the manifests, relative to the repository root.
pub const DATASETS_DIR: &str = "data/datasets";
/// Schema A9 owns; applied when present.
pub const SCHEMA_FILE: &str = "dataset.schema.json";
/// Only status that allows a download.
pub const APPROVED: &str = "approved";
/// Accepted values of `licence_status`.
const STATUSES: [&str; 3] = ["pending", APPROVED, "refused"];

/// One file of a dataset.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FileSpec {
    /// HTTPS download URL.
    pub url: String,
    /// Expected sha256 (64 lowercase hex), `None` until pinned.
    pub sha256: Option<String>,
    /// Expected size in bytes, `None` until pinned.
    pub size_bytes: Option<u64>,
    /// Date of the pin (`YYYY-MM-DD`), `None` until pinned.
    pub fetched_at: Option<String>,
}

/// One dataset manifest.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Manifest {
    /// Slug, equal to the file stem.
    pub id: String,
    /// Licence name (SPDX id or free text).
    pub licence: String,
    /// `pending`, `approved` or `refused`.
    pub licence_status: String,
    /// Minimum wait between two requests to the source host (licence condition); 0 if none.
    pub min_interval_seconds: u64,
    /// Route-out ref (e.g. `O9`) when the source blocks automated fetch; the job skips the dataset.
    pub access_blocked: Option<String>,
    /// Files to fetch.
    pub files: Vec<FileSpec>,
}

impl Manifest {
    /// True when Verification approved the licence.
    #[must_use]
    pub fn approved(&self) -> bool {
        self.licence_status == APPROVED
    }
}

/// Text field `key` of `value`, if a non-blank string.
fn text(value: &Value, key: &str) -> Option<String> {
    value.get(key).and_then(Value::as_str).filter(|s| !s.trim().is_empty()).map(str::to_owned)
}

/// True for 64 lowercase hex characters.
fn is_sha256(hex: &str) -> bool {
    hex.len() == 64 && hex.bytes().all(|b| b.is_ascii_digit() || (b'a'..=b'f').contains(&b))
}

/// True for `YYYY-MM-DD` (shape only).
fn is_date(date: &str) -> bool {
    let bytes = date.as_bytes();
    bytes.len() == 10
        && bytes.iter().enumerate().all(|(i, b)| if i == 4 || i == 7 { *b == b'-' } else { b.is_ascii_digit() })
}

/// True for a route-out ref such as `O9`, `LA-021` or `PR12` (schema pattern `^[A-Z][A-Z-]*[0-9]+$`).
fn is_ref(reference: &str) -> bool {
    let head = reference.trim_end_matches(|c: char| c.is_ascii_digit());
    head.len() < reference.len()
        && head.starts_with(|c: char| c.is_ascii_uppercase())
        && head.bytes().all(|b| b.is_ascii_uppercase() || b == b'-')
}

/// `access_blocked`: absent or null = not blocked; otherwise a route-out ref.
fn access_blocked(value: &Value, errors: &mut Vec<String>) -> Option<String> {
    let found = value.get("access_blocked").filter(|found| !found.is_null())?;
    let reference = found.as_str().filter(|reference| is_ref(reference));
    if reference.is_none() {
        errors.push(format!("access_blocked: {found} is not a route-out ref (e.g. \"O9\")"));
    }
    reference.map(str::to_owned)
}

/// Licence name: a plain string, or the `spdx_or_name` of a licence object.
fn licence_name(value: &Value) -> String {
    let licence = value.get("licence");
    let name = licence.and_then(Value::as_str).or_else(|| licence?.get("spdx_or_name")?.as_str());
    name.map(str::trim).unwrap_or_default().to_owned()
}

/// Parses one `files[]` entry; `manifest_date` is the manifest-level `fetched_at`.
/// Pushes problems to `errors`.
fn file_spec(entry: &Value, at: &str, manifest_date: Option<&str>, errors: &mut Vec<String>) -> Option<FileSpec> {
    let Some(url) = text(entry, "url") else {
        errors.push(format!("{at}.url: missing"));
        return None;
    };
    if !url.starts_with("https://") {
        errors.push(format!("{at}.url: must start with https://"));
    }
    let sha256 = text(entry, "sha256");
    if let Some(hex) = sha256.as_deref().filter(|hex| !is_sha256(hex)) {
        errors.push(format!("{at}.sha256: {hex:?} is not 64 lowercase hex characters"));
    }
    let size_bytes = entry.get("size_bytes").and_then(Value::as_u64);
    let own_date = text(entry, "fetched_at");
    if let Some(date) = own_date.as_deref().filter(|date| !is_date(date)) {
        errors.push(format!("{at}.fetched_at: {date:?} is not YYYY-MM-DD"));
    }
    let fetched_at = own_date.or_else(|| manifest_date.map(str::to_owned));
    if sha256.is_some() && (size_bytes.is_none() || fetched_at.is_none()) {
        errors.push(format!("{at}: a pinned file needs sha256, size_bytes and fetched_at together"));
    }
    Some(FileSpec { url, sha256, size_bytes, fetched_at })
}

/// Checks one parsed manifest; `stem` is its file name without `.yaml`.
///
/// # Errors
///
/// Returns every problem found, each prefixed by the manifest id.
pub fn check(stem: &str, value: &Value) -> Result<Manifest, Vec<String>> {
    let mut errors = Vec::new();
    let id = text(value, "id").unwrap_or_default();
    if id != stem {
        errors.push(format!("id {id:?} must equal the file name {stem:?}"));
    }
    let licence = licence_name(value);
    if licence.is_empty() {
        errors.push("licence: missing (no dataset enters without a licence)".to_owned());
    }
    let licence_status = text(value, "licence_status").unwrap_or_default();
    if !STATUSES.contains(&licence_status.as_str()) {
        errors.push(format!("licence_status: {licence_status:?} is not one of {}", STATUSES.join(", ")));
    }
    let interval = value.pointer("/fetch/min_interval_seconds").or_else(|| value.get("min_interval_seconds"));
    let min_interval_seconds = match interval {
        None | Some(&Value::Null) => 0,
        Some(found) => found.as_u64().unwrap_or_else(|| {
            errors.push(format!("min_interval_seconds: {found} is not a non-negative integer"));
            0
        }),
    };
    let access_blocked = access_blocked(value, &mut errors);
    let Some(entries) = value.get("files").and_then(Value::as_array) else {
        errors.push("files: missing (use [] when nothing can be fetched yet)".to_owned());
        return Err(errors.into_iter().map(|err| format!("{stem}: {err}")).collect());
    };
    let manifest_date = text(value, "fetched_at");
    if let Some(date) = manifest_date.as_deref().filter(|date| !is_date(date)) {
        errors.push(format!("fetched_at: {date:?} is not YYYY-MM-DD"));
    }
    let files: Vec<FileSpec> = entries
        .iter()
        .enumerate()
        .filter_map(|(n, entry)| file_spec(entry, &format!("files[{n}]"), manifest_date.as_deref(), &mut errors))
        .collect();
    if errors.is_empty() {
        Ok(Manifest { id, licence, licence_status, min_interval_seconds, access_blocked, files })
    } else {
        Err(errors.into_iter().map(|err| format!("{stem}: {err}")).collect())
    }
}

/// Loads and checks every manifest under `<root>/data/datasets`, sorted by id.
/// Applies `data/schema/dataset.schema.json` too when it exists.
///
/// # Errors
///
/// Returns every problem found in every manifest.
pub fn load_all(root: &Path) -> Result<Vec<Manifest>, Vec<String>> {
    let dir = root.join(DATASETS_DIR);
    if !dir.is_dir() {
        return Ok(Vec::new());
    }
    let schema_dir = root.join("data/schema");
    let validator = if schema_dir.join(SCHEMA_FILE).is_file() {
        Some(schema::compile(&schema_dir, SCHEMA_FILE).map_err(|err| vec![err])?)
    } else {
        None
    };
    let mut paths: Vec<_> = fs::read_dir(&dir)
        .map_err(|err| vec![format!("{}: {err}", dir.display())])?
        .filter_map(|entry| entry.ok().map(|item| item.path()))
        .filter(|path| path.extension().is_some_and(|ext| ext == "yaml"))
        .collect();
    paths.sort();
    let mut manifests = Vec::new();
    let mut errors = Vec::new();
    for path in paths {
        let stem = path.file_stem().and_then(|s| s.to_str()).unwrap_or_default().to_owned();
        let parsed = fs::read_to_string(&path).map_err(|err| err.to_string()).and_then(|src| yaml::parse(&src));
        let value = match parsed {
            Ok(value) => value,
            Err(err) => {
                errors.push(format!("{stem}: {err}"));
                continue;
            }
        };
        if let Some(found) = validator.as_ref() {
            errors.extend(
                schema::violations(found, &value).into_iter().map(|v| format!("{stem}: {}: {}", v.field, v.reason)),
            );
        }
        match check(&stem, &value) {
            Ok(manifest) => manifests.push(manifest),
            Err(found) => errors.extend(found),
        }
    }
    if errors.is_empty() { Ok(manifests) } else { Err(errors) }
}
