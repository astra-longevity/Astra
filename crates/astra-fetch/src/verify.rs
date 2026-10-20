//! Hash and size check of downloaded files, and pinning of new ones.

use std::fs;
use std::path::Path;

use astra_db::rules;
use astra_db::sql::sha256_hex;

use crate::plan::Plan;

/// Values to paste into a manifest when pinning a file.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Pin {
    /// sha256 of the file.
    pub sha256: String,
    /// Size in bytes.
    pub size_bytes: u64,
    /// Today (`YYYY-MM-DD`).
    pub fetched_at: String,
}

/// Hashes the file at `path`.
///
/// # Errors
///
/// Fails if the file cannot be read.
pub fn pin(path: &Path) -> Result<Pin, String> {
    let bytes = fs::read(path).map_err(|err| format!("{}: {err}", path.display()))?;
    let size_bytes = u64::try_from(bytes.len()).map_err(|err| err.to_string())?;
    Ok(Pin { sha256: sha256_hex(&bytes), size_bytes, fetched_at: rules::today() })
}

/// Checks every planned file in `cache` (file name = expected sha256).
///
/// # Errors
///
/// Returns one issue-ready message per missing or mismatching file, and one per
/// approved file that has no pinned hash yet.
pub fn verify(plan: &Plan, cache: &Path) -> Result<usize, Vec<String>> {
    let mut problems = Vec::new();
    for file in &plan.fetch {
        let path = cache.join(&file.sha256);
        let where_ = format!("dataset `{}` files[{}] ({})", file.dataset, file.index, file.url);
        match pin(&path) {
            Err(err) => problems.push(format!("Not downloaded: {where_}: {err}")),
            Ok(got) if got.sha256 != file.sha256 || got.size_bytes != file.size_bytes => problems.push(format!(
                "Hash mismatch: {where_}\n  expected sha256 {} ({} bytes)\n  got      sha256 {} ({} bytes)\n  \
                 The upstream file changed or the download is corrupt. Do not use it. Check the source, then \
                 re-pin by PR (`astra fetch pin`) only if the change is legitimate.",
                file.sha256, file.size_bytes, got.sha256, got.size_bytes
            )),
            Ok(_) => {}
        }
    }
    for missing in &plan.unpinned {
        problems.push(format!(
            "Not pinned: dataset `{}` files[{}] ({}) has no sha256. Run the fetch workflow with \
             pin=true and add the printed sha256, size_bytes and fetched_at by PR.",
            missing.dataset, missing.index, missing.url
        ));
    }
    if problems.is_empty() { Ok(plan.fetch.len()) } else { Err(problems) }
}
