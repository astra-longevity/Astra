//! Astra database builder: turns the YAML records under `data/` into `data/astra.sqlite`.
//!
//! Entry point: [`build`], called by `astra db build` (`BUILD_SPEC.md`).

pub mod diag;
pub mod load;
pub mod rules;
pub mod schema;
pub mod sql;
#[cfg(test)]
mod tests;
pub mod yaml;

use std::fs;
use std::path::{Path, PathBuf};

use rusqlite::Connection;

use crate::diag::Diags;
use crate::schema::Level;
use crate::sql::Count;

/// Name of the generated database file.
pub const DATABASE_FILE: &str = "astra.sqlite";

/// Evidence scale, relative to the root.
const LEVELS_FILE: &str = "data/schema/evidence-levels.yaml";

/// SQL schema, relative to the root.
const SCHEMA_FILE: &str = "data/schema/schema.sql";

/// Options of `astra db build`.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Options {
    /// Repository root.
    pub root: PathBuf,
    /// Data folder, relative to the root (`data` or `data/examples`).
    pub data_dir: String,
    /// Validate only, write nothing.
    pub check: bool,
    /// Warnings count as errors.
    pub strict: bool,
}

/// Result of a successful build.
#[derive(Debug)]
pub struct Outcome {
    /// Warnings collected along the way.
    pub diags: Diags,
    /// Row count of every data table.
    pub counts: Vec<Count>,
    /// Database written, `None` with `--check`.
    pub written: Option<PathBuf>,
}

/// Why a build stopped.
#[derive(Debug)]
pub enum Failure {
    /// Bad command line or missing data folder (exit code 2).
    Usage(String),
    /// At least one validation error; nothing written (exit code 1).
    Invalid(Diags),
    /// I/O or internal `SQLite` failure (exit code 3).
    Internal(String),
}

impl Failure {
    /// Process exit code of `BUILD_SPEC.md` §7.
    #[must_use]
    pub const fn exit_code(&self) -> u8 {
        match *self {
            Self::Invalid(_) => 1,
            Self::Usage(_) => 2,
            Self::Internal(_) => 3,
        }
    }
}

/// Single fatal diagnostic (E001, E003).
fn fatal(code: &'static str, file: &str, msg: String) -> Failure {
    let mut diags = Diags::default();
    diags.error(code, file, msg);
    Failure::Invalid(diags)
}

/// Inputs ready before reading the data folder.
#[derive(Debug)]
struct Prepared {
    /// Evidence scale.
    levels: Vec<Level>,
    /// Text of `schema.sql` (hashed into `build_info`).
    schema_sql: String,
    /// Fresh in-memory database.
    conn: Connection,
}

/// Loads the evidence scale and runs `schema.sql` on a fresh database.
fn prepare(root: &Path) -> Result<Prepared, Failure> {
    let levels = schema::load_levels(&root.join(LEVELS_FILE))
        .map_err(|detail| fatal("E001", LEVELS_FILE, format!("invalid evidence scale: {detail}")))?;
    let schema_sql = fs::read_to_string(root.join(SCHEMA_FILE))
        .map_err(|err| fatal("E003", SCHEMA_FILE, format!("invalid schema.sql: {err}")))?;
    let conn =
        sql::open(&schema_sql).map_err(|err| fatal("E003", SCHEMA_FILE, format!("invalid schema.sql: {err}")))?;
    Ok(Prepared { levels, schema_sql, conn })
}

/// Number of diagnostics that stop the build.
fn blocking(diags: &Diags, strict: bool) -> usize {
    if strict { diags.items.len() } else { diags.errors() }
}

/// Validates the data folder of `opts`, then builds (or only checks) the database.
/// `today` (`AAAA-MM-JJ`) feeds the W044 date warning.
///
/// # Errors
///
/// See [`Failure`]: usage, validation errors, or an I/O / `SQLite` failure.
pub fn build(opts: &Options, today: &str) -> Result<Outcome, Failure> {
    let data = opts.root.join(&opts.data_dir);
    if !data.is_dir() {
        return Err(Failure::Usage(format!("data folder not found: {}", data.display())));
    }
    let Prepared { levels, schema_sql, conn } = prepare(&opts.root)?;
    let mut diags = Diags::default();
    let schema_dir = opts.root.join("data").join("schema");
    let corpus = load::load(&opts.root, &opts.data_dir, &schema_dir, &mut diags).map_err(Failure::Internal)?;
    rules::check(&corpus, &mut diags, today);
    if blocking(&diags, opts.strict) > 0 {
        return Err(Failure::Invalid(diags));
    }
    sql::insert_all(&conn, &levels, &corpus).map_err(Failure::Internal)?;
    let counts = sql::counts(&conn).map_err(Failure::Internal)?;
    sql::fill_build_info(&conn, &schema_sql, &counts).map_err(Failure::Internal)?;
    let written = if opts.check {
        None
    } else {
        let out = data.join(DATABASE_FILE);
        sql::write(&conn, &out).map_err(Failure::Internal)?;
        Some(out)
    };
    Ok(Outcome { diags, counts, written })
}
