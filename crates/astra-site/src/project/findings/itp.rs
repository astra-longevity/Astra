//! ITP lifespan arms: one nested YAML file each under `literature/lifespan/itp/`.
//! Only the few fields the site shows are read, with a small line parser.

use std::ffi::OsStr;
use std::fs;
use std::path::Path;

use minijinja::{Value, context};

use super::scalar;
use crate::docs::relative_name;
use crate::project::blob;
use crate::{SiteError, io_at};

/// Folder of the arm files, relative to the repository root.
pub(crate) const DIR: &str = "literature/lifespan/itp";
/// Key path of the list of fields still to check.
const TO_VERIFY: &str = "verification.to_verify";

/// One treated arm (one compound, dose, cohort and sex).
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub(crate) struct Arm {
    /// File id.
    id: String,
    /// Compound slug, e.g. `rapamycin`.
    compound: String,
    /// `F` or `M`.
    sex: String,
    /// Cohort year, e.g. `2004`.
    cohort: String,
    /// Dose value, as written.
    dose_value: String,
    /// Dose unit, e.g. `ppm`.
    dose_unit: String,
    /// `extends` or `null`.
    role: String,
    /// Median lifespan change in percent, as written (`null` if unknown).
    median_pct: String,
    /// Number of fields still to check.
    to_verify: usize,
    /// File, relative to the repository root.
    file: String,
}

impl Arm {
    /// File id.
    pub(crate) fn id(&self) -> &str {
        &self.id
    }

    /// Compound slug.
    pub(crate) fn compound(&self) -> &str {
        &self.compound
    }

    /// File, relative to the repository root.
    pub(crate) fn file(&self) -> &str {
        &self.file
    }

    /// Median change in tenths of a percent, if the file gives one.
    pub(crate) fn median(&self) -> Option<i32> {
        tenths(&self.median_pct)
    }

    /// True while some fields of the file are still to check.
    pub(crate) const fn is_unverified(&self) -> bool {
        self.to_verify > 0
    }

    /// CSS class of the arm's published result.
    pub(crate) fn role_class(&self) -> &'static str {
        if self.role == "extends" { "extends" } else { "no-effect" }
    }

    /// Plain description: sex, cohort and dose.
    pub(crate) fn detail(&self) -> String {
        let sex = match self.sex.as_str() {
            "F" => "females",
            "M" => "males",
            _ => "sex not stated",
        };
        format!("{sex}, {} cohort, {} {}", self.cohort, self.dose_value, self.dose_unit).trim().to_owned()
    }

    /// Template value for the list of arms without a median.
    pub(crate) fn to_value(&self) -> Value {
        context! {
            id => self.id,
            compound => self.compound,
            detail => self.detail(),
            href => blob(&self.file),
            file => self.file,
        }
    }
}

/// Tenths of a percent from `12.3` or `-5`; `None` for `null` or junk.
pub(crate) fn tenths(text: &str) -> Option<i32> {
    let (sign, digits) = text.strip_prefix('-').map_or((1i32, text), |rest| (-1i32, rest));
    let (whole_text, fraction) = digits.split_once('.').unwrap_or((digits, "0"));
    let whole: i32 = whole_text.parse().ok()?;
    let first = fraction.chars().next().unwrap_or('0').to_digit(10)?;
    let tenth = i32::try_from(first).ok()?;
    Some(sign.saturating_mul(whole.saturating_mul(10).saturating_add(tenth)))
}

/// Line reader keeping track of the current section and key path.
struct Reader {
    /// Arm being filled.
    arm: Arm,
    /// Current top-level key.
    section: String,
    /// Key path of the last key line (`section.key`), empty below level 2.
    path: String,
}

impl Reader {
    /// Reads one line of the file.
    fn line(&mut self, line: &str) {
        let body = line.trim();
        if body.is_empty() || body.starts_with('#') {
            // Blank or comment line.
        } else if let Some(item) = body.strip_prefix("- ") {
            self.item(item);
        } else if let Some((key, value)) = body.split_once(':') {
            let indent = line.len().saturating_sub(line.trim_start().len());
            self.key(indent, key, &scalar(value));
        } else {
            // Continuation of a folded text block.
        }
    }

    /// Counts a list item of `verification.to_verify`.
    fn item(&mut self, item: &str) {
        if self.path == TO_VERIFY && !scalar(item).is_empty() {
            self.arm.to_verify = self.arm.to_verify.saturating_add(1);
        }
    }

    /// Stores a `key: value` line read at `indent` spaces.
    fn key(&mut self, indent: usize, key: &str, value: &str) {
        self.path = match indent {
            0 => {
                key.clone_into(&mut self.section);
                key.to_owned()
            }
            2 => format!("{}.{key}", self.section),
            _ => String::new(),
        };
        let slot = match self.path.as_str() {
            "id" => &mut self.arm.id,
            "compound" => &mut self.arm.compound,
            "sex" => &mut self.arm.sex,
            "cohort_year" => &mut self.arm.cohort,
            "role" => &mut self.arm.role,
            "dose.value" => &mut self.arm.dose_value,
            "dose.unit" => &mut self.arm.dose_unit,
            "effect.median_pct" => &mut self.arm.median_pct,
            _ => return,
        };
        value.clone_into(slot);
    }
}

/// Parses one arm file; `file` is its repository-relative path.
pub(crate) fn parse(text: &str, file: &str) -> Arm {
    let arm = Arm { file: file.to_owned(), ..Arm::default() };
    let mut reader = Reader { arm, section: String::new(), path: String::new() };
    for line in text.lines() {
        reader.line(line);
    }
    reader.arm
}

/// Every arm file, sorted by compound, cohort, sex and id.
pub(crate) fn load(repo: &Path) -> Result<Vec<Arm>, SiteError> {
    let dir = repo.join(DIR);
    let mut arms = Vec::new();
    if !dir.is_dir() {
        return Ok(arms);
    }
    for item in fs::read_dir(&dir).map_err(io_at(&dir))? {
        let path = item.map_err(io_at(&dir))?.path();
        if path.extension() == Some(OsStr::new("yaml")) {
            let text = fs::read_to_string(&path).map_err(io_at(&path))?;
            arms.push(parse(&text, &relative_name(repo, &path)));
        }
    }
    arms.sort_by(|left, right| {
        (&left.compound, &left.cohort, &left.sex, &left.id).cmp(&(
            &right.compound,
            &right.cohort,
            &right.sex,
            &right.id,
        ))
    });
    Ok(arms)
}

#[cfg(test)]
mod tests {
    use super::{parse, tenths};

    const SAMPLE: &str = "# Note: a comment\nid: 2004-a-f\ncompound: aspirin\nrole: \"extends\"\nsex: F\ncohort_year: 2004\ndose:\n  value: 21\n  unit: ppm\nlifespan:\n  control:\n    median_days: 800\neffect:\n  median_pct: 8.5\n  se: null\np_values:\n  - {test: x}\nverification:\n  to_verify:\n    - n.treated\n    - n.control\n";

    #[test]
    fn reads_nested_fields() {
        let arm = parse(SAMPLE, "literature/lifespan/itp/a.yaml");
        assert_eq!(arm.id(), "2004-a-f", "id");
        assert_eq!(arm.median(), Some(85i32), "median in tenths");
        assert!(arm.is_unverified(), "two items to check");
        assert_eq!(arm.detail(), "females, 2004 cohort, 21 ppm", "plain detail");
        assert_eq!(arm.role_class(), "extends", "role");
    }

    #[test]
    fn empty_lists_and_nulls() {
        let arm =
            parse("id: b\nrole: \"null\"\neffect:\n  median_pct: null\nverification:\n  to_verify: []\n", "b.yaml");
        assert_eq!(arm.median(), None, "null median");
        assert!(!arm.is_unverified(), "nothing to check");
        assert_eq!(arm.role_class(), "no-effect", "null role");
        assert_eq!(tenths("-5.3"), Some(-53i32), "negative");
        assert_eq!(tenths("12"), Some(120i32), "whole");
    }
}
