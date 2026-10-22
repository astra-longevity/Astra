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
    /// A known intervention that should make the axis look younger is named.
    right_way: bool,
    /// A known life-shortening intervention that should make it look older is named.
    wrong_way: bool,
}

impl Axis {
    /// True for organ-system axes (table A).
    fn is_organ(&self) -> bool {
        self.code.starts_with('A')
    }

    /// True when both test-1 controls are named (otherwise a blind spot by name).
    const fn has_controls(&self) -> bool {
        self.right_way && self.wrong_way
    }

    /// Plain note on missing controls, empty when both are named.
    const fn controls_note(&self) -> &'static str {
        match (self.right_way, self.wrong_way) {
            (true, true) => "",
            (false, true) => "no known treatment that makes it look younger yet",
            (true, false) => "no known harm that makes it look older yet",
            (false, false) => "no test treatments named yet",
        }
    }

    /// Template value.
    fn to_value(&self) -> Value {
        context! {
            code => self.code,
            name => self.name,
            anchor => self.anchor,
            class => self.level.class(),
            label => self.level.label(),
            controls => self.controls_note(),
        }
    }
}

/// True when a control cell names an intervention (not empty, not only
/// "[to verify]", not "No known right-way intervention").
fn names_control(cell: &str) -> bool {
    let text = cell.trim();
    !(text.is_empty() || text == "[to verify]" || text.contains("No known right-way intervention"))
}

/// Column positions of one table, found from its header row by name.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct Columns {
    /// Functional anchor.
    anchor: usize,
    /// Right-way control.
    right: usize,
    /// Wrong-way control.
    wrong: usize,
    /// Status (coverage level).
    status: usize,
}

impl Columns {
    /// Layout of the v0 draft: `# | Axis | Anchor | Readout | Right way | Wrong way | Status`.
    const DEFAULT: Self = Self { anchor: 2, right: 4, wrong: 5, status: 6 };

    /// Positions from a header row, if it names a status column.
    fn from_header(row: &[String]) -> Option<Self> {
        let find = |word: &str| row.iter().position(|cell| cell.to_lowercase().starts_with(word));
        Some(Self {
            anchor: find("functional anchor").unwrap_or(Self::DEFAULT.anchor),
            right: find("right way").unwrap_or(Self::DEFAULT.right),
            wrong: find("wrong way").unwrap_or(Self::DEFAULT.wrong),
            status: find("status")?,
        })
    }

    /// Axis from one table row.
    fn axis(self, row: &[String]) -> Axis {
        let cell = |index: usize| row.get(index).map_or("", String::as_str);
        Axis {
            code: cell(0).to_owned(),
            name: cell(1).to_owned(),
            anchor: cell(self.anchor).to_owned(),
            level: Level::from_status(cell(self.status)),
            right_way: names_control(cell(self.right)),
            wrong_way: names_control(cell(self.wrong)),
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

/// Axis rows of the file, in file order. Only rows under a header naming a
/// status column count (tables A and B); other tables that mention axis codes
/// (obstacle table, trait map, seal-input notes) are skipped. Columns are
/// found by header name, so added or moved columns still parse.
pub(crate) fn parse(text: &str) -> Vec<Axis> {
    let mut columns: Option<Columns> = None;
    let mut found = Vec::new();
    for row in text.lines().filter_map(cells) {
        match (row.first().is_some_and(|code| is_code(code)), columns) {
            (true, Some(layout)) => found.push(layout.axis(&row)),
            (true, None) => {}
            (false, _) => columns = Columns::from_header(&row),
        }
    }
    found
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
        no_controls => axes.iter().filter(|axis| !axis.has_controls()).count(),
    }
}

#[cfg(test)]
mod tests {
    use super::{Axis, Level, is_code, parse};

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

    #[test]
    fn controls_named_or_missing() {
        let text = "| # | Axis | Functional anchor | Readout | Right way | Wrong way | Status |\n| A1 | Muscle | grip | TMS | RAPA | ERCC1 | candidate |\n| A8 | Lung | breath | TMS | **No known right-way intervention** here | `[to verify]` | candidate |\n| H6 | Nutrients | IGF-1 | ITP | RAPA | HFD `[to verify]` | candidate |\n";
        let axes = parse(text);
        assert_eq!(axes.iter().map(Axis::has_controls).collect::<Vec<_>>(), [true, false, true], "controls");
        assert_eq!(axes.get(1).map(Axis::controls_note), Some("no test treatments named yet"), "plain note");
    }

    #[test]
    fn columns_found_by_header_name() {
        let text = "| # | Axis | Stratum | Functional anchor | Status | Right way | Wrong way |\n| A2 | Bone | x | density | unmeasured | CR | `[to verify]` |\n";
        let axes = parse(text);
        let first = axes.first();
        assert_eq!(first.map(|axis| axis.level), Some(Level::Gap), "status by name, not last cell");
        assert_eq!(first.map(|axis| axis.anchor.as_str()), Some("density"), "anchor by name");
        assert_eq!(
            first.map(Axis::controls_note),
            Some("no known harm that makes it look older yet"),
            "wrong way by name"
        );
    }

    #[test]
    fn trailing_sections_ignored() {
        let text = "| # | Axis | Functional anchor | Readout | Right way | Wrong way | Status |\n| A1 | Muscle | grip | TMS | RAPA | ERCC1 | candidate |\n\n  | Axis | Readout | Need | Search |\n  |---|---|---|---|\n  | A8 | lung function | a drug | terms |\n\n## Seal-input status\n\n| Flag | Axis | Note |\n|---|---|---|\n| F1 | A1 | adopted |\n- A1: flux, not level\n";
        let axes = parse(text);
        assert_eq!(axes.len(), 1, "only the axis table row, not obstacle or seal rows");
        assert_eq!(axes.first().map(|axis| axis.level), Some(Level::Readout), "status unchanged");
    }

    #[test]
    fn real_axis_list() {
        let path = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("../..").join(super::FILE);
        let axes = parse(&std::fs::read_to_string(path).unwrap_or_default());
        assert_eq!(axes.len(), 30, "18 organ systems + 12 hallmarks");
        assert!(
            axes.iter().all(|axis| !axis.name.is_empty() && !axis.anchor.is_empty()),
            "every row named and anchored"
        );
    }
}
