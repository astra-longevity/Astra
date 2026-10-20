//! Project state: history, obstacles and their Gantt chart, missions and
//! decisions, all read from repository files (no network).

pub(crate) mod decisions;
pub(crate) mod findings;
pub(crate) mod gantt;
pub(crate) mod git;
pub(crate) mod obstacles;
pub(crate) mod status;

/// Web address of a repository file on GitHub, followed by its path.
pub(crate) const BLOB_URL: &str = "https://github.com/astra-longevity/Astra/blob/main/";

/// Link to `file` (repository-relative) on GitHub.
pub(crate) fn blob(file: &str) -> String {
    format!("{BLOB_URL}{file}")
}
