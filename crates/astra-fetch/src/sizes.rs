//! "No committed file over 10 MB" check, with an allow-list.

use std::fs;
use std::path::Path;

/// Size limit for a committed file: 10 MB.
pub const LIMIT_BYTES: u64 = 10 * 1024 * 1024;

/// Reads the allow-list: one repository-relative path per line; `#` starts a comment.
///
/// # Errors
///
/// Fails if the file exists but cannot be read. A missing file is an empty list.
pub fn read_allow_list(path: &Path) -> Result<Vec<String>, String> {
    if !path.is_file() {
        return Ok(Vec::new());
    }
    let text = fs::read_to_string(path).map_err(|err| format!("{}: {err}", path.display()))?;
    Ok(text
        .lines()
        .map(|line| line.split('#').next().unwrap_or_default().trim())
        .filter(|line| !line.is_empty())
        .map(str::to_owned)
        .collect())
}

/// Files of `files` (relative to `root`) larger than `limit` and not in `allow`,
/// as `(path, size)`. Missing files (deleted in the work tree) are ignored.
#[must_use]
pub fn oversize(root: &Path, files: &[String], allow: &[String], limit: u64) -> Vec<(String, u64)> {
    files
        .iter()
        .filter(|file| !allow.contains(file))
        .filter_map(|file| fs::metadata(root.join(file)).ok().map(|meta| (file.clone(), meta.len())))
        .filter(|&(_, size)| size > limit)
        .collect()
}
