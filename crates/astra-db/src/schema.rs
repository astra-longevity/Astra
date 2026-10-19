//! JSON Schema validation (E011, E033, E035) and the evidence scale (R01/E001).

use std::fs;
use std::path::Path;

use jsonschema::Validator;
use jsonschema::error::{ValidationErrorKind, ValidationErrorParts};
use serde_json::{Map, Value};

use crate::yaml;

/// `status` of a checked source (`verified`).
pub const VERIFIED: &str = "verified";
/// `status` of a source still to check (`to_verify`).
pub const TO_VERIFY: &str = "to_verify";
/// `status` of an answered question (`resolved`).
pub const RESOLVED: &str = "resolved";
/// `status` of an open question.
pub const OPEN: &str = "open";
/// Evidence direction that supports a claim.
pub const SUPPORTS: &str = "supports";
/// Strongest claim confidence.
pub const SOLID: &str = "established";

/// One level of `evidence-levels.yaml`.
#[derive(Debug)]
pub struct Level {
    /// Level number, 1 (weakest) to 6.
    pub rank: i64,
    /// Unique short code.
    pub code: String,
    /// Plain-English label.
    pub label: String,
    /// Plain-English definition.
    pub definition: String,
}

/// One schema violation, already translated.
#[derive(Debug)]
pub struct Violation {
    /// Rule code (`E011`, `E033` or `E035`).
    pub code: &'static str,
    /// Dotted path of the offending field, or `(racine)`.
    pub field: String,
    /// Reason, in English.
    pub reason: String,
}

/// Reads a text field of one level entry.
fn level_text(entry: &Value, key: &str) -> Result<String, String> {
    entry.get(key).and_then(Value::as_str).map(str::to_owned).ok_or_else(|| format!("missing field {key}"))
}

/// Parses and checks one entry of the scale.
fn level_entry(entry: &Value, seen: &[Level]) -> Result<Level, String> {
    let rank = entry.get("level").and_then(Value::as_i64).ok_or("missing level")?;
    let code = level_text(entry, "code")?;
    if seen.iter().any(|other| other.rank == rank) {
        return Err(format!("duplicate level {rank}"));
    }
    if seen.iter().any(|other| other.code == code) {
        return Err(format!("duplicate code {code}"));
    }
    let label = level_text(entry, "label")?;
    let definition = level_text(entry, "definition")?;
    Ok(Level { rank, code, label, definition })
}

/// Loads `evidence-levels.yaml` and checks it holds exactly levels 1 to 6.
///
/// # Errors
///
/// Returns the detail to put in the fatal E001 message.
pub fn load_levels(path: &Path) -> Result<Vec<Level>, String> {
    let text = fs::read_to_string(path).map_err(|err| format!("cannot read ({err})"))?;
    let root = yaml::parse(&text)?;
    let entries = root.get("levels").and_then(Value::as_array).ok_or("missing levels key")?;
    let mut levels: Vec<Level> = Vec::new();
    for entry in entries {
        let level = level_entry(entry, &levels)?;
        levels.push(level);
    }
    levels.sort_by_key(|level| level.rank);
    let ranks: Vec<i64> = levels.iter().map(|level| level.rank).collect();
    if ranks != [1, 2, 3, 4, 5, 6] {
        return Err("exactly levels 1 to 6 are required".to_owned());
    }
    Ok(levels)
}

/// Definition named by a `common.schema.json#/$defs/X` reference, if any.
fn referenced(map: &Map<String, Value>, defs: &Map<String, Value>) -> Option<Map<String, Value>> {
    map.get("$ref")
        .and_then(Value::as_str)
        .and_then(|reference| {
            reference.strip_prefix("common.schema.json#/$defs/").or_else(|| reference.strip_prefix("#/$defs/"))
        })
        .and_then(|name| defs.get(name))
        .and_then(Value::as_object)
        .cloned()
}

/// Replaces every `{"$ref": "common.schema.json#/$defs/X"}` by the definition `X`.
fn inline_refs(node: &mut Value, defs: &Map<String, Value>) {
    if let Some(map) = node.as_object_mut() {
        if let Some(def) = referenced(map, defs) {
            drop(map.remove("$ref"));
            for (key, value) in def {
                if !map.contains_key(&key) {
                    drop(map.insert(key, value));
                }
            }
        }
        map.values_mut().for_each(|child| inline_refs(child, defs));
    } else if let Some(items) = node.as_array_mut() {
        for child in items.iter_mut() {
            inline_refs(child, defs);
        }
    } else {
        // Scalars hold no reference.
    }
}

/// Reads a JSON file of the schema directory.
fn read_json(path: &Path) -> Result<Value, String> {
    let text = fs::read_to_string(path).map_err(|err| format!("{}: {err}", path.display()))?;
    serde_json::from_str(&text).map_err(|err| format!("{}: {err}", path.display()))
}

/// Compiles `<dir>/<name>`, with the shared definitions of `common.schema.json` inlined.
///
/// # Errors
///
/// Returns a message when a schema file is missing, unreadable or invalid.
pub fn compile(dir: &Path, name: &str) -> Result<Validator, String> {
    let common = read_json(&dir.join("common.schema.json"))?;
    let defs = common.get("$defs").and_then(Value::as_object).cloned().unwrap_or_default();
    let mut schema = read_json(&dir.join(name))?;
    inline_refs(&mut schema, &defs);
    if let Some(map) = schema.as_object_mut() {
        drop(map.remove("$id"));
    }
    jsonschema::validator_for(&schema).map_err(|err| format!("{name}: {err}"))
}

/// Renders a JSON value for a message (strings without quotes).
#[must_use]
pub fn shown(value: &Value) -> String {
    value.as_str().map_or_else(|| value.to_string(), str::to_owned)
}

/// Rewords the reason of a violation in plain English; `fallback` is the library message.
fn reason(keyword: &str, instance: &Value, fallback: String) -> String {
    let value = shown(instance);
    match keyword {
        "required" => "missing required field".to_owned(),
        "additionalProperties" => "unknown field".to_owned(),
        "type" => format!("type incorrect ({value})"),
        "enum" | "const" => format!("value not allowed: {value}"),
        "pattern" | "oneOf" | "anyOf" => format!("invalid format: {value}"),
        "minItems" => "liste trop courte".to_owned(),
        "minLength" => "empty text".to_owned(),
        "maxLength" => "text too long".to_owned(),
        "minimum" | "maximum" => format!("value out of range: {value}"),
        _ => fallback,
    }
}

/// Name of the missing or unknown key carried by the error kind, if any.
fn kind_key(kind: ValidationErrorKind) -> Option<String> {
    if let ValidationErrorKind::Required { property } = kind {
        Some(shown(&property))
    } else if let ValidationErrorKind::AdditionalProperties { unexpected } = kind {
        Some(unexpected.join(", "))
    } else {
        None
    }
}

/// Field path of a violation (`authors.0`), naming the missing or unknown key when relevant.
fn field(pointer: &str, kind: ValidationErrorKind) -> String {
    let path = pointer.trim_start_matches('/').replace('/', ".");
    match (path.is_empty(), kind_key(kind)) {
        (true, Some(name)) => name,
        (false, Some(name)) => format!("{path}.{name}"),
        (true, None) => "(root)".to_owned(),
        (false, None) => path,
    }
}

/// Rule code of a violation: E033 and E035 are schema rules with their own code.
fn code_of(keyword: &str, pointer: &str, instance: &Value) -> &'static str {
    let is_contradiction = instance.get("explanation").is_some();
    let is_verified = instance.get("status").and_then(Value::as_str) == Some(VERIFIED);
    match (keyword, pointer) {
        ("minItems", "/claims") if is_contradiction => "E033",
        ("required", "") if is_verified => "E035",
        _ => "E011",
    }
}

/// Validates `instance`; `uniqueItems` is left to E015, which names the duplicate.
#[must_use]
pub fn violations(validator: &Validator, instance: &Value) -> Vec<Violation> {
    let mut found: Vec<Violation> = Vec::new();
    for error in validator.iter_errors(instance) {
        let fallback = error.to_string();
        let ValidationErrorParts { instance: bad, kind, instance_path, .. } = error.into_parts();
        let keyword = kind.keyword().to_owned();
        if keyword == "uniqueItems" {
            continue;
        }
        let pointer = instance_path.as_str().to_owned();
        let code = code_of(&keyword, &pointer, instance);
        let text = reason(&keyword, &bad, fallback);
        found.push(Violation { code, field: field(&pointer, kind), reason: text });
    }
    found
}
