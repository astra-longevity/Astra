//! Astra static site generator: renders the research interface from the repository.
//!
//! Inputs, all read from the repository: the generated database
//! (`data/astra.sqlite`), the markdown under `literature/`, the git history and
//! `interface/` (templates, static files, glossary). Output: a folder of plain
//! HTML files with relative links, opened by double-clicking `index.html`.
//! The same inputs always produce the same bytes (no clock, sorted listings).

mod docs;
mod git;
mod glossary;
mod markdown;
mod pages;
mod query;
#[cfg(test)]
mod tests;

use core::fmt;
use std::error::Error as StdError;
use std::io;
use std::path::{Path, PathBuf};

use rusqlite::{Connection, OpenFlags};

/// Where the generator reads its inputs and writes the site.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SitePaths {
    /// Repository root (holds `interface/`, `literature/` and `.git`).
    pub repo: PathBuf,
    /// Generated database to read.
    pub database: PathBuf,
    /// Output folder, deleted and rebuilt on every run.
    pub output: PathBuf,
}

impl SitePaths {
    /// Default layout under `repo`: reads `data/astra.sqlite`, writes `interface/dist`.
    #[must_use]
    pub fn from_repo(repo: &Path) -> Self {
        Self {
            repo: repo.to_path_buf(),
            database: repo.join("data").join("astra.sqlite"),
            output: repo.join("interface").join("dist"),
        }
    }
}

/// Summary of a finished build.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct BuildReport {
    /// Number of HTML pages written.
    pub pages: usize,
}

/// Category of a build failure.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ErrorKind {
    /// The database file does not exist yet.
    MissingDatabase,
    /// The output folder exists and does not look like a generated site.
    UnsafeOutput,
    /// The database could not be read.
    Database,
    /// A template failed to load or render.
    Template,
    /// A file could not be read or written.
    Io,
}

/// Failure while building the site: a category and a message.
#[derive(Debug)]
pub struct SiteError {
    /// Category, for callers and tests.
    kind: ErrorKind,
    /// Message shown to the user.
    message: String,
}

impl SiteError {
    /// New error of `kind` with `message`.
    const fn new(kind: ErrorKind, message: String) -> Self {
        Self { kind, message }
    }

    /// Category of the failure.
    #[must_use]
    pub const fn kind(&self) -> ErrorKind {
        self.kind
    }

    /// The output folder is neither empty nor a previous build.
    pub(crate) fn unsafe_output(path: &Path) -> Self {
        let message =
            format!("refusing output folder {}: it is neither empty nor a previously generated site", path.display());
        Self::new(ErrorKind::UnsafeOutput, message)
    }
}

impl fmt::Display for SiteError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(&self.message)
    }
}

impl StdError for SiteError {}

impl From<rusqlite::Error> for SiteError {
    fn from(err: rusqlite::Error) -> Self {
        Self::new(ErrorKind::Database, format!("reading the database: {err}"))
    }
}

impl From<minijinja::Error> for SiteError {
    fn from(err: minijinja::Error) -> Self {
        Self::new(ErrorKind::Template, format!("page template: {err:#}"))
    }
}

/// Wraps an I/O error with the path it concerns.
pub(crate) fn io_at(path: &Path) -> impl FnOnce(io::Error) -> SiteError + use<> {
    let shown = path.display().to_string();
    move |err| SiteError::new(ErrorKind::Io, format!("{shown}: {err}"))
}

/// Builds the site from the database at `paths.database` (opened read-only).
///
/// # Errors
///
/// Fails if the database is missing or unreadable, a template fails, or a
/// file cannot be written.
pub fn build(paths: &SitePaths) -> Result<BuildReport, SiteError> {
    if !paths.database.is_file() {
        let message = format!("database not found: {} (run `astra db build` first)", paths.database.display());
        return Err(SiteError::new(ErrorKind::MissingDatabase, message));
    }
    let conn = Connection::open_with_flags(&paths.database, OpenFlags::SQLITE_OPEN_READ_ONLY)?;
    build_from(&conn, paths)
}

/// Builds the site from an already open database (used by tests).
///
/// # Errors
///
/// Same as [`build`], minus the missing-database case.
pub fn build_from(conn: &Connection, paths: &SitePaths) -> Result<BuildReport, SiteError> {
    pages::render_site(conn, paths)
}
