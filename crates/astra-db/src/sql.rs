//! `SQLite` side of the build (`BUILD_SPEC.md` §3, §5, §6).
//!
//! Runs `schema.sql` on a fresh in-memory database, inserts the records in the
//! fixed order, fills `build_info` and writes the file with `VACUUM INTO`.

use std::fs;
use std::path::Path;

use rusqlite::types::Value as SqlValue;
use rusqlite::{Connection, params_from_iter};
use serde_json::Value;
use sha2::{Digest as _, Sha256};

use crate::load::{Corpus, Record};
use crate::schema::Level;

/// Version of the data format written to `build_info`.
const FORMAT_VERSION: &str = "1.0";

/// One column value, computed from a record.
#[derive(Clone, Copy, Debug)]
enum Col {
    /// The YAML field of that name (text, integer or absent).
    Field(&'static str),
    /// Path of the YAML file, relative to the root.
    File,
    /// `sources.doi`: the id for a DOI, `NULL` for an ISBN.
    Doi,
    /// `sources.authors`: compact JSON array.
    Authors,
    /// `sources.first_author`: family name of the first author.
    FirstAuthor,
}

/// Rows of one data folder.
#[derive(Debug)]
struct Table {
    /// Data folder.
    dir: &'static str,
    /// `INSERT` statement.
    sql: &'static str,
    /// Bound values, in parameter order.
    cols: &'static [Col],
}

/// Rows built from a list field of a record (`id`, item, optional position).
#[derive(Debug)]
struct Child {
    /// Data folder of the parent record.
    dir: &'static str,
    /// `INSERT` statement.
    sql: &'static str,
    /// List field of the parent.
    list: &'static str,
    /// Whether the 0-based index is bound as a third parameter.
    positioned: bool,
}

/// Parent tables, in insertion order.
static TABLES: [Table; 8] = [
    Table {
        dir: "missions",
        sql: "INSERT INTO missions (id, title, folder, summary, status, owner, file) VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7)",
        cols: &[
            Col::Field("id"),
            Col::Field("title"),
            Col::Field("folder"),
            Col::Field("summary"),
            Col::Field("status"),
            Col::Field("owner"),
            Col::File,
        ],
    },
    Table {
        dir: "sources",
        sql: "INSERT INTO sources (id, doi, title, year, authors, first_author, journal, type, status,
              verified_via, verified_at, note, file) VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10, ?11, ?12, ?13)",
        cols: &[
            Col::Field("id"),
            Col::Doi,
            Col::Field("title"),
            Col::Field("year"),
            Col::Authors,
            Col::FirstAuthor,
            Col::Field("journal"),
            Col::Field("type"),
            Col::Field("status"),
            Col::Field("verified_via"),
            Col::Field("verified_at"),
            Col::Field("note"),
            Col::File,
        ],
    },
    Table {
        dir: "concepts",
        sql: "INSERT INTO concepts (id, kind, name, description, file) VALUES (?1, ?2, ?3, ?4, ?5)",
        cols: &[Col::Field("id"), Col::Field("kind"), Col::Field("name"), Col::Field("description"), Col::File],
    },
    Table {
        dir: "claims",
        sql: "INSERT INTO claims (id, statement, organism, evidence_level, confidence, mission, note, file)
              VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8)",
        cols: &[
            Col::Field("id"),
            Col::Field("statement"),
            Col::Field("organism"),
            Col::Field("evidence_level"),
            Col::Field("confidence"),
            Col::Field("mission"),
            Col::Field("note"),
            Col::File,
        ],
    },
    Table {
        dir: "evidence",
        sql: "INSERT INTO evidence (claim_id, source_id, direction, locator, note, file) VALUES (?1, ?2, ?3, ?4, ?5, ?6)",
        cols: &[
            Col::Field("claim"),
            Col::Field("source"),
            Col::Field("direction"),
            Col::Field("locator"),
            Col::Field("note"),
            Col::File,
        ],
    },
    Table {
        dir: "relations",
        sql: "INSERT INTO relations (id, from_id, to_id, type, note, file) VALUES (?1, ?2, ?3, ?4, ?5, ?6)",
        cols: &[
            Col::Field("id"),
            Col::Field("from"),
            Col::Field("to"),
            Col::Field("type"),
            Col::Field("note"),
            Col::File,
        ],
    },
    Table {
        dir: "questions",
        sql: "INSERT INTO questions (id, statement, mission, status, note, file) VALUES (?1, ?2, ?3, ?4, ?5, ?6)",
        cols: &[
            Col::Field("id"),
            Col::Field("statement"),
            Col::Field("mission"),
            Col::Field("status"),
            Col::Field("note"),
            Col::File,
        ],
    },
    Table {
        dir: "contradictions",
        sql: "INSERT INTO contradictions (id, mission, explanation, status, file) VALUES (?1, ?2, ?3, ?4, ?5)",
        cols: &[Col::Field("id"), Col::Field("mission"), Col::Field("explanation"), Col::Field("status"), Col::File],
    },
];

/// Link tables, inserted right after their parent folder.
static CHILDREN: [Child; 6] = [
    Child {
        dir: "concepts",
        sql: "INSERT INTO concept_synonyms (concept_id, synonym) VALUES (?1, ?2)",
        list: "synonyms",
        positioned: false,
    },
    Child {
        dir: "claims",
        sql: "INSERT INTO claim_concepts (claim_id, concept_id, position) VALUES (?1, ?2, ?3)",
        list: "concepts",
        positioned: true,
    },
    Child {
        dir: "relations",
        sql: "INSERT INTO relation_claims (relation_id, claim_id) VALUES (?1, ?2)",
        list: "claims",
        positioned: false,
    },
    Child {
        dir: "questions",
        sql: "INSERT INTO question_concepts (question_id, concept_id) VALUES (?1, ?2)",
        list: "concepts",
        positioned: false,
    },
    Child {
        dir: "questions",
        sql: "INSERT INTO question_resolved_by (question_id, claim_id) VALUES (?1, ?2)",
        list: "resolved_by",
        positioned: false,
    },
    Child {
        dir: "contradictions",
        sql: "INSERT INTO contradiction_claims (contradiction_id, claim_id) VALUES (?1, ?2)",
        list: "claims",
        positioned: false,
    },
];

/// Row count of one table.
pub type Count = (&'static str, i64);

/// Data tables counted in `build_info` and on stdout, in schema order.
pub static DATA_TABLES: [&str; 15] = [
    "evidence_levels",
    "missions",
    "sources",
    "concepts",
    "concept_synonyms",
    "claims",
    "claim_concepts",
    "evidence",
    "relations",
    "relation_claims",
    "questions",
    "question_concepts",
    "question_resolved_by",
    "contradictions",
    "contradiction_claims",
];

/// Converts any `SQLite` error to the message of an internal failure.
fn internal(err: &rusqlite::Error) -> String {
    format!("SQLite: {err}")
}

/// Field of a record as an SQL value (`NULL` when absent).
fn field(record: &Record, key: &str) -> SqlValue {
    let found = record.value.get(key);
    found.and_then(Value::as_str).map_or_else(
        || found.and_then(Value::as_i64).map_or(SqlValue::Null, SqlValue::Integer),
        |text| SqlValue::Text(text.to_owned()),
    )
}

/// Computes one column of a record.
fn column(record: &Record, col: Col) -> SqlValue {
    let authors = record.list("authors");
    match col {
        Col::Field(key) => field(record, key),
        Col::File => SqlValue::Text(record.file.clone()),
        Col::Doi => record
            .text("id")
            .filter(|id| !id.starts_with("isbn:"))
            .map_or(SqlValue::Null, |id| SqlValue::Text(id.to_owned())),
        Col::Authors => serde_json::to_string(&authors).map_or(SqlValue::Null, SqlValue::Text),
        Col::FirstAuthor => authors
            .first()
            .and_then(|first| first.split(',').next())
            .map_or(SqlValue::Null, |name| SqlValue::Text(name.trim().to_owned())),
    }
}

/// Runs `schema.sql` on a fresh in-memory database.
///
/// # Errors
///
/// Returns the `SQLite` message for the fatal E003.
pub fn open(schema_sql: &str) -> Result<Connection, String> {
    let conn = Connection::open_in_memory().map_err(|err| err.to_string())?;
    conn.execute_batch("PRAGMA foreign_keys = ON;").map_err(|err| err.to_string())?;
    conn.execute_batch(schema_sql).map_err(|err| err.to_string())?;
    Ok(conn)
}

/// Inserts the evidence scale.
fn insert_levels(conn: &Connection, levels: &[Level]) -> Result<(), String> {
    let mut stmt = conn
        .prepare("INSERT INTO evidence_levels (level, code, label, definition) VALUES (?1, ?2, ?3, ?4)")
        .map_err(|err| internal(&err))?;
    for level in levels {
        let row = (level.rank, &level.code, &level.label, &level.definition);
        let _inserted: usize = stmt.execute(row).map_err(|err| internal(&err))?;
    }
    Ok(())
}

/// Inserts the rows of one parent table.
fn insert_table(conn: &Connection, table: &Table, records: &[Record]) -> Result<(), String> {
    let mut stmt = conn.prepare(table.sql).map_err(|err| internal(&err))?;
    for record in records {
        let row: Vec<SqlValue> = table.cols.iter().map(|col| column(record, *col)).collect();
        let _inserted: usize = stmt.execute(params_from_iter(row.iter())).map_err(|err| internal(&err))?;
    }
    Ok(())
}

/// Inserts the link rows of one list field.
fn insert_child(conn: &Connection, child: &Child, records: &[Record]) -> Result<(), String> {
    let mut stmt = conn.prepare(child.sql).map_err(|err| internal(&err))?;
    for record in records {
        let parent = field(record, "id");
        for (idx, item) in record.list(child.list).into_iter().enumerate() {
            let mut row = vec![parent.clone(), SqlValue::Text(item.to_owned())];
            if child.positioned {
                let pos = i64::try_from(idx).map_err(|err| err.to_string())?;
                row.push(SqlValue::Integer(pos));
            }
            let _inserted: usize = stmt.execute(params_from_iter(row.iter())).map_err(|err| internal(&err))?;
        }
    }
    Ok(())
}

/// Inserts the scale and every record, in the order of `BUILD_SPEC.md` §3.
///
/// # Errors
///
/// Returns the `SQLite` message when a row is refused (a validation gap).
pub fn insert_all(conn: &Connection, levels: &[Level], corpus: &Corpus) -> Result<(), String> {
    conn.execute_batch("BEGIN").map_err(|err| internal(&err))?;
    insert_levels(conn, levels)?;
    for table in &TABLES {
        insert_table(conn, table, corpus.records(table.dir))?;
        for child in CHILDREN.iter().filter(|child| child.dir == table.dir) {
            insert_child(conn, child, corpus.records(child.dir))?;
        }
    }
    conn.execute_batch("COMMIT").map_err(|err| internal(&err))
}

/// Row count of every data table, in schema order.
///
/// # Errors
///
/// Returns the `SQLite` message on a failed query.
pub fn counts(conn: &Connection) -> Result<Vec<Count>, String> {
    DATA_TABLES
        .iter()
        .map(|table| {
            let sql = format!("SELECT COUNT(*) FROM {table}");
            let count: i64 = conn.query_row(&sql, [], |row| row.get(0)).map_err(|err| internal(&err))?;
            Ok((*table, count))
        })
        .collect()
}

/// Lowercase hexadecimal SHA-256 of `bytes`.
#[must_use]
pub fn sha256_hex(bytes: &[u8]) -> String {
    /// Hexadecimal digits.
    const DIGITS: &[u8; 16] = b"0123456789abcdef";
    let digest = Sha256::digest(bytes);
    let mut hex = String::with_capacity(64);
    for byte in digest {
        for nibble in [byte >> 4u8, byte & 0x0fu8] {
            hex.extend(DIGITS.get(usize::from(nibble)).map(|digit| char::from(*digit)));
        }
    }
    hex
}

/// Fills `build_info`: format version, `schema.sql` hash, one count per table.
///
/// # Errors
///
/// Returns the `SQLite` message on a failed insert.
pub fn fill_build_info(conn: &Connection, schema_sql: &str, counts: &[Count]) -> Result<(), String> {
    let mut rows: Vec<(String, String)> = vec![
        ("format_version".to_owned(), FORMAT_VERSION.to_owned()),
        ("schema_sql_sha256".to_owned(), sha256_hex(schema_sql.as_bytes())),
    ];
    rows.extend(counts.iter().map(|&(table, count)| (format!("n_{table}"), count.to_string())));
    let mut stmt = conn.prepare("INSERT INTO build_info (key, value) VALUES (?1, ?2)").map_err(|err| internal(&err))?;
    for row in &rows {
        let _inserted: usize = stmt.execute((&row.0, &row.1)).map_err(|err| internal(&err))?;
    }
    Ok(())
}

/// Writes the database to `out` through `<out>.tmp` and an atomic rename.
///
/// # Errors
///
/// Returns a message on an I/O or `SQLite` failure; `out` is then untouched.
pub fn write(conn: &Connection, out: &Path) -> Result<(), String> {
    let partial = out.with_extension("sqlite.tmp");
    if partial.exists() {
        fs::remove_file(&partial).map_err(|err| format!("{}: {err}", partial.display()))?;
    }
    let target = partial.to_str().ok_or_else(|| format!("non-UTF-8 path: {}", partial.display()))?;
    conn.execute_batch("PRAGMA user_version = 1;").map_err(|err| internal(&err))?;
    let _written: usize = conn.execute("VACUUM INTO ?1", [target]).map_err(|err| internal(&err))?;
    fs::rename(&partial, out).map_err(|err| format!("{}: {err}", out.display()))
}
