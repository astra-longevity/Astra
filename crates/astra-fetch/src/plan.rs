//! Fetch plan: which files to download, which are unpinned, which are skipped.

use crate::Manifest;

/// One file the workflow downloads.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PlannedFile {
    /// Dataset id.
    pub dataset: String,
    /// Index in `files`.
    pub index: usize,
    /// Download URL.
    pub url: String,
    /// Expected sha256; also the cache file name.
    pub sha256: String,
    /// Expected size in bytes.
    pub size_bytes: u64,
    /// Minimum seconds between requests to the same host.
    pub interval: u64,
}

/// An approved file with no pinned hash yet.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Unpinned {
    /// Dataset id.
    pub dataset: String,
    /// Index in `files`.
    pub index: usize,
    /// Download URL.
    pub url: String,
    /// Minimum seconds between requests to the same host.
    pub interval: u64,
}

/// A dataset whose licence is not approved.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Skipped {
    /// Dataset id.
    pub dataset: String,
    /// Its `licence_status`.
    pub status: String,
}

/// Plan built from the manifests.
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct Plan {
    /// Approved and pinned: download and verify.
    pub fetch: Vec<PlannedFile>,
    /// Approved but no sha256 yet. Fails a scheduled run.
    pub unpinned: Vec<Unpinned>,
    /// Licence not approved. Never downloaded.
    pub skipped: Vec<Skipped>,
}

impl Plan {
    /// Splits the files of `manifests`.
    #[must_use]
    pub fn new(manifests: &[Manifest]) -> Self {
        let mut plan = Self::default();
        for manifest in manifests {
            if !manifest.approved() {
                plan.skipped.push(Skipped { dataset: manifest.id.clone(), status: manifest.licence_status.clone() });
                continue;
            }
            let interval = manifest.min_interval_seconds;
            for (index, file) in manifest.files.iter().enumerate() {
                match (file.sha256.as_ref(), file.size_bytes) {
                    (Some(sha256), Some(size_bytes)) => plan.fetch.push(PlannedFile {
                        dataset: manifest.id.clone(),
                        index,
                        url: file.url.clone(),
                        sha256: sha256.clone(),
                        size_bytes,
                        interval,
                    }),
                    _ => plan.unpinned.push(Unpinned {
                        dataset: manifest.id.clone(),
                        index,
                        url: file.url.clone(),
                        interval,
                    }),
                }
            }
        }
        plan
    }

    /// Tab-separated lines read by `scripts/fetch-datasets.sh`:
    /// `fetch\t<id>\t<n>\t<interval>\t<sha256>\t<url>`, `unpinned\t<id>\t<n>\t<interval>\t-\t<url>`,
    /// `skipped\t<id>\t<status>`.
    #[must_use]
    pub fn render(&self) -> String {
        let fetch = self
            .fetch
            .iter()
            .map(|f| format!("fetch\t{}\t{}\t{}\t{}\t{}", f.dataset, f.index, f.interval, f.sha256, f.url));
        let unpinned =
            self.unpinned.iter().map(|u| format!("unpinned\t{}\t{}\t{}\t-\t{}", u.dataset, u.index, u.interval, u.url));
        let skipped = self.skipped.iter().map(|s| format!("skipped\t{}\t{}", s.dataset, s.status));
        fetch.chain(unpinned).chain(skipped).collect::<Vec<_>>().join("\n")
    }
}
