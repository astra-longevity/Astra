//! Reads every data folder (`BUILD_SPEC.md` §2) and runs the per-file checks
//! E002, E010–E015, E033–E035.

use std::collections::BTreeMap;
use std::fs;
use std::path::Path;

use jsonschema::Validator;
use serde_json::Value;
use unicode_normalization::is_nfc;

use crate::diag::Diags;
use crate::schema::{self, Violation};
use crate::yaml;

/// Accepts `text` only in Unicode normal form C (§2).
fn nfc(text: String) -> Result<String, String> {
    if is_nfc(&text) { Ok(text) } else { Err("text is not in Unicode NFC form".to_owned()) }
}

/// How the expected file name derives from the record (§2).
#[derive(Clone, Copy, Debug)]
pub enum Naming {
    /// `<id>.yaml`.
    Id,
    /// `filename(id).yaml`.
    Source,
    /// `<claim>--filename(source).yaml`.
    Evidence,
}

/// One data folder and its schema.
#[derive(Clone, Copy, Debug)]
pub struct Category {
    /// Folder name under the data directory; also the table name.
    pub dir: &'static str,
    /// JSON Schema file name.
    pub schema: &'static str,
    /// Key holding the record id, when the record has one.
    pub id_key: Option<&'static str>,
    /// Key holding a DOI to check for canonical form (E014).
    pub doi_key: Option<&'static str>,
    /// File-name rule.
    pub naming: Naming,
}

/// Data folders in insertion order (§3).
pub static CATEGORIES: [Category; 8] = [
    Category { dir: "missions", schema: "mission.schema.json", id_key: Some("id"), doi_key: None, naming: Naming::Id },
    Category {
        dir: "sources",
        schema: "source.schema.json",
        id_key: Some("id"),
        doi_key: Some("id"),
        naming: Naming::Source,
    },
    Category { dir: "concepts", schema: "concept.schema.json", id_key: Some("id"), doi_key: None, naming: Naming::Id },
    Category { dir: "claims", schema: "claim.schema.json", id_key: Some("id"), doi_key: None, naming: Naming::Id },
    Category {
        dir: "evidence",
        schema: "evidence.schema.json",
        id_key: None,
        doi_key: Some("source"),
        naming: Naming::Evidence,
    },
    Category {
        dir: "relations",
        schema: "relation.schema.json",
        id_key: Some("id"),
        doi_key: None,
        naming: Naming::Id,
    },
    Category {
        dir: "questions",
        schema: "question.schema.json",
        id_key: Some("id"),
        doi_key: None,
        naming: Naming::Id,
    },
    Category {
        dir: "contradictions",
        schema: "contradiction.schema.json",
        id_key: Some("id"),
        doi_key: None,
        naming: Naming::Id,
    },
];

/// List fields whose duplicates are reported as E015.
const LIST_FIELDS: [&str; 4] = ["concepts", "claims", "synonyms", "resolved_by"];

/// One YAML file whose root is a mapping.
#[derive(Debug)]
pub struct Record {
    /// Path relative to the repository root, `/`-separated.
    pub file: String,
    /// Parsed content.
    pub value: Value,
}

impl Record {
    /// Text value of `key`, if present and a string.
    #[must_use]
    pub fn text(&self, key: &str) -> Option<&str> {
        self.value.get(key).and_then(Value::as_str)
    }

    /// String items of the list `key` (empty when absent).
    #[must_use]
    pub fn list(&self, key: &str) -> Vec<&str> {
        self.value
            .get(key)
            .and_then(Value::as_array)
            .map(|items| items.iter().filter_map(Value::as_str).collect())
            .unwrap_or_default()
    }
}

/// All records, per data folder, each folder in byte order of file names.
#[derive(Debug, Default)]
pub struct Corpus {
    /// Records keyed by folder name.
    sets: BTreeMap<&'static str, Vec<Record>>,
}

impl Corpus {
    /// Records of one folder (empty when the folder is absent).
    #[must_use]
    pub fn records(&self, dir: &str) -> &[Record] {
        self.sets.get(dir).map_or(&[], Vec::as_slice)
    }
}

/// Encodes one byte of a DOI for a file name.
fn encode_byte(byte: u8) -> String {
    let ch = char::from(byte);
    if byte == b'/' {
        "_".to_owned()
    } else if ch.is_ascii_lowercase() || ch.is_ascii_digit() || "._()+;-".contains(ch) {
        ch.to_string()
    } else {
        format!("%{byte:02x}")
    }
}

/// `filename(source_id)` of §2, with the `.yaml` extension.
#[must_use]
pub fn filename(source_id: &str) -> String {
    if let Some(isbn) = source_id.strip_prefix("isbn:") {
        return format!("isbn-{isbn}.yaml");
    }
    let parts: Vec<String> = source_id.bytes().map(encode_byte).collect();
    format!("{}.yaml", parts.concat())
}

/// Canonical form of a DOI as an author might have written it.
fn canonical_doi(raw: &str) -> String {
    let lower = raw.trim().to_lowercase();
    let stripped = ["https://doi.org/", "http://doi.org/", "https://dx.doi.org/", "doi:"]
        .iter()
        .find_map(|prefix| lower.strip_prefix(prefix))
        .unwrap_or(&lower);
    stripped.chars().filter(|ch| !ch.is_whitespace()).collect()
}

/// The canonical DOI when `raw` is a DOI written in a non-canonical way.
#[must_use]
pub fn doi_problem(raw: &str) -> Option<String> {
    if raw.starts_with("isbn:") {
        return None;
    }
    let canonical = canonical_doi(raw);
    (canonical != raw && canonical.starts_with("10.")).then_some(canonical)
}

/// Expected file name of a record, when its naming fields are readable.
fn expected_name(category: &Category, record: &Record) -> Option<String> {
    match category.naming {
        Naming::Id => record.text("id").map(|id| format!("{id}.yaml")),
        Naming::Source => record.text("id").map(filename),
        Naming::Evidence => {
            let claim = record.text("claim")?;
            let source = record.text("source")?;
            Some(format!("{claim}--{}", filename(source)))
        }
    }
}

/// Message of a schema violation; rule-specific codes have fixed messages (§4).
fn violation_message(violation: &Violation) -> String {
    match violation.code {
        "E033" => "a contradiction links at least 2 claims".to_owned(),
        "E034" => "resolved question without resolved_by".to_owned(),
        "E035" => "verified source without verified_via/verified_at".to_owned(),
        _ => format!("{}: {}", violation.field, violation.reason),
    }
}

/// Values that appear more than once in `items`, once per extra occurrence.
fn duplicates(items: &[Value]) -> Vec<&Value> {
    let mut seen: Vec<&Value> = Vec::new();
    let mut extra: Vec<&Value> = Vec::new();
    for item in items {
        if seen.contains(&item) {
            extra.push(item);
        } else {
            seen.push(item);
        }
    }
    extra
}

/// What to do with one directory entry.
enum Entry {
    /// Ignored (`.gitkeep`, `README.md`).
    Skip,
    /// Not a `.yaml` file (E002).
    Unexpected,
    /// A YAML record.
    Yaml,
}

/// Classifies a directory entry by name and type.
fn classify(name: &str, path: &Path) -> Entry {
    if name == ".gitkeep" || name == "README.md" {
        Entry::Skip
    } else if path.extension().is_some_and(|ext| ext == "yaml") && path.is_file() {
        Entry::Yaml
    } else {
        Entry::Unexpected
    }
}

/// Entry names of `dir` in byte order; a missing folder holds no entry.
fn list_dir(dir: &Path) -> Result<Vec<String>, String> {
    if !dir.exists() {
        return Ok(Vec::new());
    }
    let entries = fs::read_dir(dir).map_err(|err| format!("{}: {err}", dir.display()))?;
    let mut names: Vec<String> = Vec::new();
    for entry in entries {
        let found = entry.map_err(|err| format!("{}: {err}", dir.display()))?;
        names.push(found.file_name().to_string_lossy().into_owned());
    }
    names.sort();
    Ok(names)
}

/// Loader state shared by all folders.
struct Loader<'run> {
    /// Repository root.
    root: &'run Path,
    /// Data directory, relative to the root.
    data_dir: &'run str,
    /// Collected diagnostics.
    diags: &'run mut Diags,
}

impl Loader<'_> {
    /// Reads and parses one file; E010 on bad encoding, bad YAML or non-mapping root.
    fn read(&mut self, path: &Path, file: &str) -> Result<Option<Value>, String> {
        let bytes = fs::read(path).map_err(|err| format!("{file}: {err}"))?;
        let parsed = String::from_utf8(bytes)
            .map_err(|_err| "not UTF-8 encoded".to_owned())
            .and_then(nfc)
            .and_then(|text| yaml::parse(&text));
        match parsed {
            Ok(value) if value.is_object() => Ok(Some(value)),
            Ok(_) => {
                let msg = "invalid YAML: the root must be a mapping".to_owned();
                self.diags.error("E010", file, msg);
                Ok(None)
            }
            Err(detail) => {
                self.diags.error("E010", file, format!("invalid YAML: {detail}"));
                Ok(None)
            }
        }
    }

    /// E014 on the DOI field; returns the field so its schema errors are not repeated.
    fn check_doi(&mut self, category: &Category, record: &Record) -> Option<&'static str> {
        let key = category.doi_key?;
        let canonical = doi_problem(record.text(key)?)?;
        let msg = format!("non-canonical DOI, write {canonical}");
        self.diags.error("E014", &record.file, msg);
        Some(key)
    }

    /// E011/E033/E034/E035 for one record, without repeated messages.
    fn check_schema(&mut self, validator: &Validator, record: &Record, skip: Option<&str>) {
        let mut emitted: Vec<String> = Vec::new();
        for violation in schema::violations(validator, &record.value) {
            let msg = violation_message(&violation);
            if Some(violation.field.as_str()) == skip || emitted.contains(&msg) {
                continue;
            }
            self.diags.error(violation.code, &record.file, msg.clone());
            emitted.push(msg);
        }
    }

    /// E015: duplicate values in list fields.
    fn check_lists(&mut self, record: &Record) {
        for key in LIST_FIELDS {
            let items = record.value.get(key).and_then(Value::as_array);
            for value in duplicates(items.map_or(&[], Vec::as_slice)) {
                let msg = format!("{key}: duplicate value {}", schema::shown(value));
                self.diags.error("E015", &record.file, msg);
            }
        }
    }

    /// Schema, DOI and list checks of one record.
    fn check(&mut self, category: &Category, validator: &Validator, record: &Record) {
        let skip = self.check_doi(category, record);
        self.check_schema(validator, record, skip);
        self.check_lists(record);
    }

    /// E012: the file name must derive from the record (§2).
    fn check_name(&mut self, category: &Category, name: &str, record: &Record) {
        if let Some(expected) = expected_name(category, record).filter(|wanted| wanted != name) {
            let msg = format!("the file should be named {expected}");
            self.diags.error("E012", &record.file, msg);
        }
    }

    /// Loads one folder.
    fn category(&mut self, category: &Category, validator: &Validator) -> Result<Vec<Record>, String> {
        let dir = self.root.join(self.data_dir).join(category.dir);
        let mut records: Vec<Record> = Vec::new();
        for name in list_dir(&dir)? {
            let path = dir.join(&name);
            let file = format!("{}/{}/{name}", self.data_dir, category.dir);
            match classify(&name, &path) {
                Entry::Skip => {}
                Entry::Unexpected => {
                    let msg = "unexpected file (expected: *.yaml)".to_owned();
                    self.diags.error("E002", &file, msg);
                }
                Entry::Yaml => {
                    if let Some(value) = self.read(&path, &file)? {
                        let record = Record { file, value };
                        self.check(category, validator, &record);
                        self.check_name(category, &name, &record);
                        records.push(record);
                    }
                }
            }
        }
        Ok(records)
    }

    /// E013: the same id twice in a folder (DOIs compared case-insensitively).
    fn check_ids(&mut self, category: &Category, records: &[Record]) {
        let Some(key) = category.id_key else {
            return;
        };
        let mut seen: BTreeMap<String, &str> = BTreeMap::new();
        for record in records {
            let Some(id) = record.text(key) else {
                continue;
            };
            let folded = id.to_lowercase();
            match seen.get(&folded).copied() {
                Some(other) => {
                    let msg = format!("id {id} already defined in {other}");
                    self.diags.error("E013", &record.file, msg);
                }
                None => drop(seen.insert(folded, &record.file)),
            }
        }
    }
}

/// Reads and checks every data folder under `<root>/<data_dir>`.
///
/// # Errors
///
/// Returns a message on an I/O failure or an unusable JSON Schema; validation
/// problems go to `diags` instead.
pub fn load(root: &Path, data_dir: &str, schema_dir: &Path, diags: &mut Diags) -> Result<Corpus, String> {
    let mut loader = Loader { root, data_dir, diags };
    let mut corpus = Corpus::default();
    for category in &CATEGORIES {
        let validator = schema::compile(schema_dir, category.schema)?;
        let records = loader.category(category, &validator)?;
        loader.check_ids(category, &records);
        drop(corpus.sets.insert(category.dir, records));
    }
    Ok(corpus)
}
