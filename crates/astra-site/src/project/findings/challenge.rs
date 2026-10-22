//! Static SVG heatmap of the R-Sim v0 challenge study: for each organ, age at
//! injury and injury size, how well the challenge separates damage made from
//! damage repaired (the "split" share), one panel per repair-limit value.
//! Values are read as hundredths and drawn with integers only, so the same
//! file gives the same bytes.

use std::fs;
use std::path::Path;

use serde_json::Value as Json;

use crate::project::blob;
use crate::project::gantt::escape;
use crate::{SiteError, io_at};

/// Challenge result of Actions run 39594585409, relative to the repository root.
pub(crate) const FILE: &str = "research/runs/sim-v0-challenge-2026-10-21/result-20000.json";
/// Mice per time point whose shares are drawn.
const MICE: &str = "100";
/// Share (hundredths) from which an organ counts as split.
pub(crate) const SPLIT: i32 = 80;

/// Organ codes of the simulation and their plain names, in display order.
pub(crate) const ORGANS: [(&str, &str); 11] = [
    ("A1_muscle", "Muscle"),
    ("A3_brain", "Brain"),
    ("A6_heart", "Heart"),
    ("A8_lung", "Lung"),
    ("A9_kidney", "Kidney"),
    ("A10_liver_metabolism", "Liver and metabolism"),
    ("A11_immune", "Immune system"),
    ("A12_blood", "Blood"),
    ("A13_skin_hair", "Skin and hair"),
    ("A14_gut", "Gut"),
    ("A15_fat", "Fat"),
];

/// Width of the organ column, in pixels.
const LABEL_W: i32 = 140;
/// Width of one cell, in pixels.
const CELL_W: i32 = 38;
/// Height of one cell, in pixels.
const CELL_H: i32 = 18;
/// Gap between panels, in pixels.
const GAP: i32 = 22;
/// Height of the header (panel title, ages, doses), in pixels.
const HEAD_H: i32 = 56;

/// Number as written in the file (`0.15`, `2.0`), if it is a number.
pub(crate) fn number_text(value: &Json) -> Option<String> {
    value.as_number().map(ToString::to_string)
}

/// Non-negative decimal number as hundredths (`0.92` → 92); digits past the
/// second decimal are dropped. `None` for anything else.
pub(crate) fn hundredths(value: &Json) -> Option<i32> {
    let text = number_text(value)?;
    let (whole_text, fraction) = text.split_once('.').unwrap_or((text.as_str(), ""));
    let whole: i32 = whole_text.parse().ok()?;
    let digits: String = fraction.chars().chain(std::iter::repeat('0')).take(2).collect();
    let part: i32 = digits.parse().ok()?;
    whole.checked_mul(100i32)?.checked_add(part)
}

/// `0.97` style label of a share in hundredths.
fn share(value: i32) -> String {
    format!("{}.{:02}", value.checked_div(100i32).unwrap_or(0i32), value.checked_rem(100i32).unwrap_or(0i32))
}

/// Repair-limit value as shown: `"prior"` is the current guess 0.5; `2.0` → `2`.
pub(crate) fn kappa_text(value: &Json) -> String {
    let text = value.as_str().map(ToOwned::to_owned).or_else(|| number_text(value)).unwrap_or_default();
    if text == "prior" {
        return String::from("0.5");
    }
    text.strip_suffix(".0").map(ToOwned::to_owned).unwrap_or(text)
}

/// One column: age at injury in months, injury size as written, and its result key (`t0.5_d0.3`).
struct Column {
    /// Age at injury, in months.
    months: i32,
    /// Injury size, as written in the file.
    dose: String,
    /// Key of the cell in the result file.
    key: String,
}

/// Columns for one start age (in years), one per dose.
fn columns_at(start: &Json, doses: &[Json]) -> Option<Vec<Column>> {
    let months = hundredths(start)?.checked_mul(12i32)?.checked_div(100i32)?;
    let start_text = number_text(start)?;
    doses
        .iter()
        .map(|dose| {
            let text = number_text(dose)?;
            Some(Column { months, key: format!("t{start_text}_d{text}"), dose: text })
        })
        .collect()
}

/// Every column, start age first.
fn columns(json: &Json) -> Option<Vec<Column>> {
    let doses = json.get("doses_xc")?.as_array()?;
    let groups =
        json.get("t0s")?.as_array()?.iter().map(|start| columns_at(start, doses)).collect::<Option<Vec<_>>>()?;
    Some(groups.into_iter().flatten().collect())
}

/// One panel: a repair-limit value and, per organ in [`ORGANS`] order, one
/// share per column (`None` when the file has no value).
struct Panel {
    /// Repair-limit value, as shown.
    kappa: String,
    /// Shares in hundredths, one row per organ.
    rows: Vec<Vec<Option<i32>>>,
}

impl Panel {
    /// Panel from one entry of `runs`, reading the cells of `columns`.
    fn parse(run: &Json, columns: &[Column]) -> Option<Self> {
        let organs = run.get("per_organ")?;
        let rows = ORGANS
            .iter()
            .map(|organ| {
                let cells = organs.get(organ.0).and_then(|found| found.get("challenge"));
                columns
                    .iter()
                    .map(|column| cells.and_then(|c| c.get(&column.key)?.get(MICE)?.get("sum")).and_then(hundredths))
                    .collect()
            })
            .collect();
        Some(Self { kappa: kappa_text(run.get("kappa")?), rows })
    }
}

/// Parsed challenge grid.
pub(crate) struct Challenge {
    /// Columns: age at injury and injury size.
    columns: Vec<Column>,
    /// One panel per repair-limit value.
    panels: Vec<Panel>,
}

impl Challenge {
    /// Grid from the result file's JSON; `None` if its layout is not the expected one.
    pub(crate) fn parse(json: &Json) -> Option<Self> {
        let found = columns(json)?;
        let panels =
            json.get("runs")?.as_array()?.iter().map(|run| Panel::parse(run, &found)).collect::<Option<Vec<_>>>()?;
        (!found.is_empty()).then_some(Self { columns: found, panels })
    }

    /// Width of one panel, in pixels.
    fn panel_w(&self) -> i32 {
        i32::try_from(self.columns.len()).unwrap_or(0i32).saturating_mul(CELL_W)
    }

    /// Header of `panel` starting at `left`: title, ages, doses.
    fn header(&self, panel: &Panel, left: i32) -> String {
        let guess = if panel.kappa == "0.5" { " (current guess)" } else { "" };
        let middle = left.saturating_add(self.panel_w().checked_div(2i32).unwrap_or(0i32));
        let mut parts = vec![format!(
            "<text class=\"panel\" x=\"{middle}\" y=\"14\">Repair limit kappa {}{guess}</text>",
            escape(&panel.kappa)
        )];
        let mut previous = None;
        for (column, index) in self.columns.iter().zip(0i32..) {
            let x = left.saturating_add(index.saturating_mul(CELL_W));
            let centre = x.saturating_add(CELL_W.checked_div(2i32).unwrap_or(0i32));
            if previous != Some(column.months) {
                parts
                    .push(format!("<text class=\"age\" x=\"{x}\" y=\"32\">injured at {} months</text>", column.months));
                previous = Some(column.months);
            }
            parts.push(format!("<text class=\"dose\" x=\"{centre}\" y=\"50\">{}</text>", escape(&column.dose)));
        }
        parts.concat()
    }

    /// Cells of `panel` starting at `left`.
    fn cells(&self, panel: &Panel, left: i32) -> String {
        let mut parts = Vec::new();
        for (organ, (row, line)) in ORGANS.iter().zip(panel.rows.iter().zip(0i32..)) {
            let top = HEAD_H.saturating_add(line.saturating_mul(CELL_H));
            for (column, (cell, index)) in self.columns.iter().zip(row.iter().zip(0i32..)) {
                let x = left.saturating_add(index.saturating_mul(CELL_W));
                let (class, label) =
                    cell.map_or((String::from("none"), String::from("-")), |value| (bucket(value), share(value)));
                let text_x = x.saturating_add(CELL_W.checked_div(2i32).unwrap_or(0i32));
                let text_y = top.saturating_add(13i32);
                parts.push(format!(
                    "<g class=\"cell {class}\"><title>{}, injured at {} months, injury {}: {label}</title><rect x=\"{x}\" y=\"{top}\" width=\"{CELL_W}\" height=\"{CELL_H}\"/><text x=\"{text_x}\" y=\"{text_y}\">{label}</text></g>",
                    organ.1,
                    column.months,
                    escape(&column.dose)
                ));
            }
        }
        parts.concat()
    }

    /// The whole heatmap as an SVG element.
    pub(crate) fn svg(&self) -> String {
        let count = i32::try_from(self.panels.len()).unwrap_or(0i32);
        let step = self.panel_w().saturating_add(GAP);
        let width = LABEL_W.saturating_add(step.saturating_mul(count));
        let organs = i32::try_from(ORGANS.len()).unwrap_or(0i32);
        let height = HEAD_H.saturating_add(organs.saturating_mul(CELL_H)).saturating_add(4i32);
        let mut parts = vec![format!(
            "<svg class=\"heatmap\" xmlns=\"http://www.w3.org/2000/svg\" viewBox=\"0 0 {width} {height}\" width=\"{width}\" height=\"{height}\" role=\"img\" aria-label=\"Share split per organ, age at injury and injury size\">"
        )];
        for (organ, line) in ORGANS.iter().zip(0i32..) {
            let y = HEAD_H.saturating_add(line.saturating_mul(CELL_H)).saturating_add(13i32);
            parts.push(format!("<text class=\"organ\" x=\"2\" y=\"{y}\">{}</text>", organ.1));
        }
        for (panel, index) in self.panels.iter().zip(0i32..) {
            let left = LABEL_W.saturating_add(index.saturating_mul(step));
            parts.push(format!("<a href=\"{}\">", escape(&blob(FILE))));
            parts.push(self.header(panel, left));
            parts.push(self.cells(panel, left));
            parts.push(String::from("</a>"));
        }
        parts.push(String::from("</svg>"));
        parts.concat()
    }
}

/// CSS class of a share: four bands, the top one at [`SPLIT`] and above.
fn bucket(value: i32) -> String {
    let band = match value {
        v if v >= SPLIT => "split",
        v if v >= 50i32 => "s2",
        v if v >= 20i32 => "s1",
        _ => "s0",
    };
    String::from(band)
}

/// Heatmap of the challenge file, or `None` when the file is absent or not in the expected layout.
pub(crate) fn load(repo: &Path) -> Result<Option<String>, SiteError> {
    let path = repo.join(FILE);
    if !path.is_file() {
        return Ok(None);
    }
    let text = fs::read_to_string(&path).map_err(io_at(&path))?;
    let json: Json = serde_json::from_str(&text).unwrap_or(Json::Null);
    Ok(Challenge::parse(&json).map(|grid| grid.svg()))
}

#[cfg(test)]
mod tests {
    use serde_json::Value as Json;

    use super::{Challenge, hundredths, kappa_text};

    /// JSON value from text (`Null` if invalid).
    fn json(text: &str) -> Json {
        serde_json::from_str(text).unwrap_or(Json::Null)
    }

    #[test]
    fn numbers() {
        assert_eq!(hundredths(&json("0.92")), Some(92i32), "two decimals");
        assert_eq!(hundredths(&json("0.5")), Some(50i32), "one decimal");
        assert_eq!(hundredths(&json("1.0")), Some(100i32), "one");
        assert_eq!(hundredths(&json("\"x\"")), None, "not a number");
        assert_eq!(kappa_text(&json("\"prior\"")), "0.5", "prior is the guess");
        assert_eq!(kappa_text(&json("8.0")), "8", "no trailing .0");
    }

    /// One run, one start age, two doses; heart split at the big dose, others missing.
    fn sample() -> String {
        let grid = Challenge::parse(&json(
            r#"{"t0s": [0.5], "doses_xc": [0.3, 0.6], "runs": [{"kappa": "prior", "per_organ": {"A6_heart": {"challenge": {
                "t0.5_d0.3": {"100": {"sum": 0.2}}, "t0.5_d0.6": {"100": {"sum": 0.97}}}}}}]}"#,
        ));
        grid.map(|found| found.svg()).unwrap_or_default()
    }

    #[test]
    fn heatmap_shape_and_determinism() {
        let plot = sample();
        assert!(plot.starts_with("<svg") && plot.ends_with("</svg>"), "one svg element");
        assert!(plot.contains("cell split") && plot.contains(">0.97<"), "split cell marked");
        assert!(plot.contains("cell s1") && plot.contains(">0.20<"), "low cell");
        assert!(plot.contains("cell none"), "missing organ shown as empty");
        assert_eq!(plot, sample(), "same bytes");
    }

    #[test]
    fn heatmap_plain_labels() {
        let plot = sample();
        assert!(plot.contains("injured at 6 months") && plot.contains("(current guess)"), "plain headers");
        assert!(plot.contains("result-20000.json"), "links to its file");
        assert!(!plot.contains("A6_heart") && !plot.contains("t0.5_") && !plot.contains("prior"), "no raw codes");
    }

    #[test]
    fn wrong_layout() {
        assert!(Challenge::parse(&json(r#"{"runs": []}"#)).is_none(), "no columns");
    }
}
