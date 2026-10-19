//! Reader for `interface/glossary.yaml` (flat list of `id/term/aka/definition`; the older key `en` is read as `aka`).
//!
//! The file is a simple list of one-line fields, so a tiny line reader is
//! enough and avoids a YAML dependency.

use std::fs;
use std::path::Path;

use minijinja::{Value, context};

use crate::{SiteError, io_at};

/// One glossary entry.
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub(crate) struct Entry {
    /// Stable id used by `<abbr data-term="id">`.
    id: String,
    /// Term.
    term: String,
    /// Other name, or what an abbreviation stands for.
    aka: Option<String>,
    /// Plain-language definition.
    definition: String,
}

impl Entry {
    /// Template value `{ id, term, aka, definition }`.
    pub(crate) fn to_value(&self) -> Value {
        context! { id => self.id, term => self.term, aka => self.aka, definition => self.definition }
    }

    /// Sets one field from a `key: value` pair; unknown keys are ignored.
    fn set(&mut self, key: &str, value: String) {
        match key {
            "id" => self.id = value,
            "term" => self.term = value,
            "aka" | "en" => self.aka = Some(value),
            "definition" => self.definition = value,
            _ => {}
        }
    }
}

/// Reads the glossary of `repo`; an absent file gives an empty glossary.
pub(crate) fn load(repo: &Path) -> Result<Vec<Entry>, SiteError> {
    let path = repo.join("interface").join("glossary.yaml");
    if !path.is_file() {
        return Ok(Vec::new());
    }
    let text = fs::read_to_string(&path).map_err(io_at(&path))?;
    Ok(parse(&text))
}

/// Parses the glossary text, in file order.
pub(crate) fn parse(text: &str) -> Vec<Entry> {
    let mut entries: Vec<Entry> = Vec::new();
    for line in text.lines() {
        if let Some(first) = line.strip_prefix("- ") {
            entries.push(Entry::default());
            set_field(&mut entries, first);
        } else if let Some(field) = line.strip_prefix("  ") {
            set_field(&mut entries, field);
        } else {
            // Comments and blank lines carry no data.
        }
    }
    entries
}

/// Applies `key: value` to the last entry.
fn set_field(entries: &mut [Entry], field: &str) {
    let Some(entry) = entries.last_mut() else {
        return;
    };
    if let Some((key, value)) = field.trim().split_once(':') {
        entry.set(key.trim(), unquote(value.trim()).to_owned());
    }
}

/// Removes matching surrounding quotes.
fn unquote(value: &str) -> &str {
    for quote in ['"', '\''] {
        if let Some(inner) = value.strip_prefix(quote).and_then(|rest| rest.strip_suffix(quote)) {
            return inner;
        }
    }
    value
}

/// Entries sorted by term in alphabetical order (accents ignored).
pub(crate) fn sorted(entries: &[Entry]) -> Vec<Entry> {
    let mut copy = entries.to_vec();
    copy.sort_by(|left, right| {
        sort_key(&left.term).cmp(&sort_key(&right.term)).then_with(|| left.term.cmp(&right.term))
    });
    copy
}

/// Lowercase term without accents, for sorting.
fn sort_key(term: &str) -> String {
    term.to_lowercase().chars().map(fold_accent).collect()
}

/// Maps an accented lowercase letter to its base letter.
const fn fold_accent(chr: char) -> char {
    match chr {
        '\u{e0}' | '\u{e2}' | '\u{e4}' => 'a',
        '\u{e7}' => 'c',
        '\u{e8}'..='\u{eb}' => 'e',
        '\u{ee}' | '\u{ef}' => 'i',
        '\u{f4}' | '\u{f6}' => 'o',
        '\u{f9}' | '\u{fb}' | '\u{fc}' => 'u',
        other => other,
    }
}

#[cfg(test)]
mod tests {
    use super::{parse, sorted};

    #[test]
    fn glossary_sorts_ignoring_accents() {
        let text = "# c\n- id: b\n  term: \u{c9}pig\u{e9}n\u{e9}tique\n  definition: x\n- id: a\n  term: Zinc\n  en: zinc\n  definition: \"y\"\n- id: c\n  term: Cellule\n  definition: z\n";
        let entries = parse(text);
        let terms: Vec<String> = sorted(&entries).into_iter().map(|entry| entry.term).collect();
        assert_eq!(terms, ["Cellule", "\u{c9}pig\u{e9}n\u{e9}tique", "Zinc"], "accent-insensitive order");
        let zinc = entries.get(1).and_then(|entry| entry.aka.clone());
        assert_eq!(zinc.as_deref(), Some("zinc"), "older en key");
        assert_eq!(entries.get(1).map(|entry| entry.definition.as_str()), Some("y"), "unquoted");
    }

    #[test]
    fn reads_aka_key() {
        let entries = parse("- id: ros\n  term: Free radicals\n  aka: ROS, reactive oxygen species\n  definition: d\n");
        let aka = entries.first().and_then(|entry| entry.aka.clone());
        assert_eq!(aka.as_deref(), Some("ROS, reactive oxygen species"), "aka");
    }
}
