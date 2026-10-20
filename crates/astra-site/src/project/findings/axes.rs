//! Coverage map of the biological-age axes: tables A (organ systems) and B
//! (hallmarks) of `research/metric/v0/axes.md`, with their status.

use std::fs;
use std::path::Path;

use minijinja::{Value, context};

use crate::project::status::cells;
use crate::{SiteError, io_at};

/// Axis list, relative to the repository root.
pub(crate) const FILE: &str = "research/metric/v0/axes.md";

/// Coverage level of one axis.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum Level {
    /// A public dataset carries a functional or own readout.
    Readout,
    /// Only a molecular readout (e.g. transcriptome) is named.
    Molecular,
    /// No public readout yet: a named blind spot.
    Gap,
}

impl Level {
    /// Level from the status cell of the table.
    fn from_status(status: &str) -> Self {
        if status.starts_with("unmeasured") {
            Self::Gap
        } else if status.contains("molecular only") {
            Self::Molecular
        } else {
            Self::Readout
        }
    }

    /// CSS class.
    const fn class(self) -> &'static str {
        match self {
            Self::Readout => "cov-readout",
            Self::Molecular => "cov-molecular",
            Self::Gap => "cov-gap",
        }
    }

    /// Plain label (never the raw status code).
    const fn label(self) -> &'static str {
        match self {
            Self::Readout => "public readout named",
            Self::Molecular => "molecular data only",
            Self::Gap => "blind spot: no public readout yet",
        }
    }
}

/// One axis row.
#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct Axis {
    /// Code, `A1`..`A18` or `H1`..`H12`.
    code: String,
    /// Organ system or hallmark name.
    name: String,
    /// What is measured to anchor the axis.
    anchor: String,
    /// Coverage level.
    level: Level,
}

impl Axis {
    /// True for organ-system axes (table A).
    fn is_organ(&self) -> bool {
        self.code.starts_with('A')
    }

    /// Template value.
    fn to_value(&self) -> Value {
        context! {
            code => self.code,
            name => self.name,
            anchor => self.anchor,
            class => self.level.class(),
            label => self.level.label(),
        }
    }
}

/// True for an axis code: `A` or `H` followed by digits.
fn is_code(text: &str) -> bool {
    let mut chars = text.chars();
    matches!(chars.next(), Some('A' | 'H'))
        && !text.get(1..).unwrap_or_default().is_empty()
        && chars.all(|chr| chr.is_ascii_digit())
}

/// Axis rows of the file, in file order.
pub(crate) fn parse(text: &str) -> Vec<Axis> {
    text.lines()
        .filter_map(cells)
        .filter(|row| row.first().is_some_and(|code| is_code(code)))
        .map(|row| Axis {
            code: row.first().cloned().unwrap_or_default(),
            name: row.get(1).cloned().unwrap_or_default(),
            anchor: row.get(2).cloned().unwrap_or_default(),
            level: Level::from_status(row.last().map_or("", String::as_str)),
        })
        .collect()
}

/// Axes of [`FILE`], empty if the file is missing.
pub(crate) fn load(repo: &Path) -> Result<Vec<Axis>, SiteError> {
    let path = repo.join(FILE);
    if !path.is_file() {
        return Ok(Vec::new());
    }
    let text = fs::read_to_string(&path).map_err(io_at(&path))?;
    Ok(parse(&text))
}

/// `axis_groups` variable: organ systems then hallmarks, `{ label, axes[] }`.
pub(crate) fn groups(axes: &[Axis]) -> Vec<Value> {
    [("Organ systems", true), ("Hallmarks of ageing", false)]
        .into_iter()
        .map(|(label, organ)| {
            let rows: Vec<Value> = axes.iter().filter(|axis| axis.is_organ() == organ).map(Axis::to_value).collect();
            context! { label => label, axes => rows }
        })
        .collect()
}

/// `axis_counts` variable: number of axes per level, `{ total, readout, molecular, gap }`.
pub(crate) fn counts(axes: &[Axis]) -> Value {
    let count = |level: Level| axes.iter().filter(|axis| axis.level == level).count();
    context! {
        total => axes.len(),
        readout => count(Level::Readout),
        molecular => count(Level::Molecular),
        gap => count(Level::Gap),
    }
}

#[cfg(test)]
mod tests {
    use super::{Level, is_code, parse};

    #[test]
    fn rows_and_levels() {
        let text = "| # | Axis | Anchor | Readout | R | W | Status |\n|---|---|---|---|---|---|---|\n| A1 | Muscle | grip | TMS | x | y | candidate |\n| A8 | Lung | breath | TMS | x | y | candidate (molecular only) |\n| H2 | Telomeres | length | none | x | y | unmeasured |\n| ITP | not an axis | a | b |\n";
        let axes = parse(text);
        assert_eq!(axes.len(), 3, "three axis rows");
        assert_eq!(
            axes.iter().map(|axis| axis.level).collect::<Vec<_>>(),
            [Level::Readout, Level::Molecular, Level::Gap],
            "levels"
        );
        assert!(is_code("H12") && !is_code("A") && !is_code("ACA"), "codes");
    }
}
