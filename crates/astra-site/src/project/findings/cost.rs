//! Static SVG cost curve of the R-Sim v0 challenge design: for each mouse
//! budget (mice per sex), the most organs any design of that size or smaller
//! splits, one line per repair-limit value; solid for all organs, dashed for
//! organs whose readout can be repeated on the same mice. Integers only, so
//! the same file gives the same bytes.

use std::fs;
use std::path::Path;

use serde_json::Value as Json;

use super::challenge::kappa_text;
use crate::project::blob;
use crate::project::gantt::escape;
use crate::{SiteError, io_at};

/// Cost table, relative to the repository root.
pub(crate) const FILE: &str = "research/sim/v0/results/cost-curve.json";

/// Left margin (axis labels), in pixels.
const LEFT: i32 = 48;
/// Top margin (legend), in pixels.
const TOP: i32 = 40;
/// Pixels between two mouse budgets on the horizontal axis.
const STEP_X: i32 = 52;
/// Half of [`STEP_X`]: first budget sits mid-slot.
const HALF_X: i32 = 26;
/// Pixels per organ on the vertical axis.
const STEP_Y: i32 = 18;
/// Organs on the vertical axis.
const ORGANS: i32 = 11;
/// Bottom margin (budget labels and axis title), in pixels.
const BOTTOM: i32 = 40;
/// Right margin, in pixels.
const RIGHT: i32 = 16;

/// One repair-limit value: its label and its rows `(mice per sex, organs split, repeatable organs split)`.
struct Series {
    /// Repair-limit value, as shown.
    kappa: String,
    /// Design rows.
    rows: Vec<(i64, i32, i32)>,
}

/// Parsed cost table.
pub(crate) struct Cost {
    /// One series per repair-limit value.
    series: Vec<Series>,
    /// Distinct mouse budgets, ascending.
    budgets: Vec<i64>,
}

/// Length of a JSON list as an organ count.
fn count(value: &Json) -> Option<i32> {
    i32::try_from(value.as_array()?.len()).ok()
}

impl Series {
    /// Series from one entry of `kappas`.
    fn parse(kappa: &Json) -> Option<Self> {
        let rows = kappa
            .get("rows")?
            .as_array()?
            .iter()
            .map(|row| {
                Some((
                    row.get("mice_per_sex")?.as_i64()?,
                    count(row.get("split")?)?,
                    count(row.get("split_repeatable")?)?,
                ))
            })
            .collect::<Option<Vec<_>>>()?;
        Some(Self { kappa: kappa_text(kappa.get("kappa")?), rows })
    }
}

impl Cost {
    /// Table from the cost file's JSON; `None` if its layout is not the expected one.
    pub(crate) fn parse(json: &Json) -> Option<Self> {
        let series = json.get("kappas")?.as_array()?.iter().map(Series::parse).collect::<Option<Vec<_>>>()?;
        let mut budgets: Vec<i64> = series.iter().flat_map(|one| one.rows.iter().map(|row| row.0)).collect();
        budgets.sort_unstable();
        budgets.dedup();
        (!budgets.is_empty()).then_some(Self { series, budgets })
    }

    /// Best `(all, repeatable)` organ counts of `series` within each budget.
    fn frontier(&self, series: &Series) -> Vec<(i32, i32)> {
        self.budgets
            .iter()
            .map(|&budget| {
                series
                    .rows
                    .iter()
                    .filter(|row| row.0 <= budget)
                    .fold((0i32, 0i32), |best, row| (best.0.max(row.1), best.1.max(row.2)))
            })
            .collect()
    }

    /// X pixel of budget number `index`.
    const fn x(index: i32) -> i32 {
        LEFT.saturating_add(HALF_X).saturating_add(index.saturating_mul(STEP_X))
    }

    /// Y pixel of an organ count.
    const fn y(organs: i32) -> i32 {
        TOP.saturating_add(ORGANS.saturating_sub(organs).saturating_mul(STEP_Y))
    }

    /// Plot width, in pixels.
    fn width(&self) -> i32 {
        let budgets = i32::try_from(self.budgets.len()).unwrap_or(0i32);
        LEFT.saturating_add(budgets.saturating_mul(STEP_X)).saturating_add(RIGHT)
    }

    /// Grid lines, organ counts and budget labels.
    fn axes(&self) -> Vec<String> {
        let right = self.width().saturating_sub(RIGHT);
        let mut parts = Vec::new();
        for organs in 0i32..=ORGANS {
            let y = Self::y(organs);
            parts.push(format!(
                "<line class=\"grid\" x1=\"{LEFT}\" y1=\"{y}\" x2=\"{right}\" y2=\"{y}\"/><text class=\"tick-y\" x=\"{}\" y=\"{}\">{organs}</text>",
                LEFT.saturating_sub(6i32),
                y.saturating_add(4i32)
            ));
        }
        let base = Self::y(0i32).saturating_add(16i32);
        for (budget, index) in self.budgets.iter().zip(0i32..) {
            parts.push(format!("<text class=\"tick\" x=\"{}\" y=\"{base}\">{budget}</text>", Self::x(index)));
        }
        parts.push(format!(
            "<text class=\"axis\" x=\"{}\" y=\"{}\">mice per sex (smallest design of at most this size)</text>",
            LEFT.saturating_add(right).checked_div(2i32).unwrap_or(0i32),
            base.saturating_add(18i32)
        ));
        parts
    }

    /// Step line through `points` (organ counts per budget), with class `class`.
    fn line(points: &[i32], class: &str, title: &str) -> String {
        let mut path = Vec::new();
        for (organs, index) in points.iter().zip(0i32..) {
            let (x, y) = (Self::x(index), Self::y(*organs));
            path.push(format!("{}{x} {y}", if index == 0i32 { "M" } else { " L" }));
        }
        format!("<path class=\"{class}\" d=\"{}\"><title>{}</title></path>", path.concat(), escape(title))
    }

    /// Legend: one entry per repair-limit value, then solid / dashed meaning.
    fn legend(&self) -> String {
        let mut parts = Vec::new();
        let mut x = LEFT;
        for (series, index) in self.series.iter().zip(0i32..) {
            let guess = if series.kappa == "0.5" { " (current guess)" } else { "" };
            parts.push(format!(
                "<line class=\"all k{index}\" x1=\"{x}\" y1=\"10\" x2=\"{}\" y2=\"10\"/><text class=\"key\" x=\"{}\" y=\"14\">kappa {}{guess}</text>",
                x.saturating_add(18i32),
                x.saturating_add(22i32),
                escape(&series.kappa)
            ));
            x = x.saturating_add(if guess.is_empty() { 90i32 } else { 170i32 });
        }
        parts.push(format!(
            "<line class=\"all\" x1=\"{LEFT}\" y1=\"28\" x2=\"{}\" y2=\"28\"/><text class=\"key\" x=\"{}\" y=\"32\">all organs</text><line class=\"repeat\" x1=\"{}\" y1=\"28\" x2=\"{}\" y2=\"28\"/><text class=\"key\" x=\"{}\" y=\"32\">organs with a repeatable readout</text>",
            LEFT.saturating_add(18i32),
            LEFT.saturating_add(22i32),
            LEFT.saturating_add(100i32),
            LEFT.saturating_add(118i32),
            LEFT.saturating_add(122i32)
        ));
        parts.concat()
    }

    /// The whole curve as an SVG element.
    pub(crate) fn svg(&self) -> String {
        let width = self.width();
        let height = Self::y(0i32).saturating_add(BOTTOM);
        let mut parts = vec![format!(
            "<svg class=\"cost\" xmlns=\"http://www.w3.org/2000/svg\" viewBox=\"0 0 {width} {height}\" width=\"{width}\" height=\"{height}\" role=\"img\" aria-label=\"Organs split against mice per sex\"><a href=\"{}\">",
            escape(&blob(FILE))
        )];
        parts.extend(self.axes());
        parts.push(self.legend());
        for (series, index) in self.series.iter().zip(0i32..) {
            let best = self.frontier(series);
            let all: Vec<i32> = best.iter().map(|pair| pair.0).collect();
            let repeat: Vec<i32> = best.iter().map(|pair| pair.1).collect();
            parts.push(Self::line(&all, &format!("all k{index}"), &format!("kappa {}: all organs", series.kappa)));
            parts.push(Self::line(
                &repeat,
                &format!("repeat k{index}"),
                &format!("kappa {}: repeatable readout", series.kappa),
            ));
        }
        parts.push(String::from("</a></svg>"));
        parts.concat()
    }
}

/// Cost curve of the cost file, or `None` when the file is absent or not in the expected layout.
pub(crate) fn load(repo: &Path) -> Result<Option<String>, SiteError> {
    let path = repo.join(FILE);
    if !path.is_file() {
        return Ok(None);
    }
    let text = fs::read_to_string(&path).map_err(io_at(&path))?;
    let json: Json = serde_json::from_str(&text).unwrap_or(Json::Null);
    Ok(Cost::parse(&json).map(|cost| cost.svg()))
}

#[cfg(test)]
mod tests {
    use serde_json::Value as Json;

    use super::Cost;

    /// JSON value from text (`Null` if invalid).
    fn json(text: &str) -> Json {
        serde_json::from_str(text).unwrap_or(Json::Null)
    }

    /// Two budgets, one repair-limit value: 200 mice split 2 organs (1 repeatable).
    fn sample() -> Json {
        json(
            r#"{"kappas": [{"kappa": "prior", "rows": [
            {"mice_per_sex": 20, "split": [], "split_repeatable": []},
            {"mice_per_sex": 200, "split": ["A6_heart", "A3_brain"], "split_repeatable": ["A6_heart"]}
        ]}]}"#,
        )
    }

    #[test]
    fn frontier_is_cumulative() {
        let cost = Cost::parse(&sample());
        let best = cost.as_ref().and_then(|c| c.series.first().map(|s| c.frontier(s))).unwrap_or_default();
        assert_eq!(best, vec![(0i32, 0i32), (2i32, 1i32)], "best within each budget");
    }

    #[test]
    fn curve_shape_and_determinism() {
        let plot = Cost::parse(&sample()).map(|cost| cost.svg()).unwrap_or_default();
        assert!(plot.starts_with("<svg") && plot.ends_with("</svg>"), "one svg element");
        assert!(plot.contains("kappa 0.5 (current guess)"), "prior shown as the guess");
        assert!(plot.contains(">200<") && plot.contains("class=\"repeat k0\""), "budget label and dashed line");
        assert!(plot.contains("cost-curve.json"), "links to its file");
        assert!(!plot.contains("A6_heart") && !plot.contains("prior"), "no raw codes");
        assert_eq!(plot, Cost::parse(&sample()).map(|cost| cost.svg()).unwrap_or_default(), "same bytes");
    }

    #[test]
    fn wrong_layout() {
        assert!(Cost::parse(&json(r#"{"kappas": []}"#)).is_none(), "no budgets");
    }
}
