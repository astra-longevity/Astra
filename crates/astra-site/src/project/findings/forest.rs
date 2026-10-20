//! Static SVG forest plot of the ITP arms: one row per arm, grouped by
//! compound, a dot at the median lifespan change, a zero line. Integer
//! arithmetic only, so the same inputs give the same bytes.

use std::cmp::Ordering;

use super::itp::Arm;
use crate::project::blob;
use crate::project::gantt::escape;

/// Width of the compound column, in pixels.
const COMPOUND_W: i32 = 150;
/// Width of the arm detail column, in pixels.
const DETAIL_W: i32 = 220;
/// Pixels per tenth of a percent.
const PX_PER_TENTH: i32 = 1;
/// Axis step and rounding, in tenths of a percent (5 %).
const STEP: i32 = 50;
/// Width of the value column, in pixels.
const VALUE_W: i32 = 110;
/// Height of one row, in pixels.
const ROW_H: i32 = 16;
/// Height of the axis band, in pixels.
const AXIS_H: i32 = 24;
/// Dot radius, in pixels.
const DOT_R: i32 = 4;

/// Horizontal scale: lowest and highest value shown, in tenths.
struct Scale {
    /// Left end, a multiple of [`STEP`], at most 0.
    low: i32,
    /// Right end, a multiple of [`STEP`], at least 0.
    high: i32,
}

impl Scale {
    /// Scale covering zero and every median, rounded out to [`STEP`].
    fn new(arms: &[&Arm]) -> Self {
        let values: Vec<i32> = arms.iter().filter_map(|arm| arm.median()).collect();
        let min = values.iter().copied().min().unwrap_or(0i32).min(0i32);
        let max = values.iter().copied().max().unwrap_or(0i32).max(0i32);
        let low = min.checked_div_euclid(STEP).unwrap_or(0i32).saturating_mul(STEP);
        let high = max.saturating_neg().checked_div_euclid(STEP).unwrap_or(0i32).saturating_mul(STEP).saturating_neg();
        Self { low, high: high.max(low.saturating_add(STEP)) }
    }

    /// Left edge of the plot area.
    const fn left() -> i32 {
        COMPOUND_W.saturating_add(DETAIL_W)
    }

    /// X pixel of a value in tenths.
    const fn x(&self, value: i32) -> i32 {
        Self::left().saturating_add(value.saturating_sub(self.low).saturating_mul(PX_PER_TENTH))
    }

    /// Total width in pixels.
    const fn width(&self) -> i32 {
        self.x(self.high).saturating_add(VALUE_W)
    }

    /// Tick lines and labels every [`STEP`], plus the zero line.
    fn ticks(&self, height: i32) -> Vec<String> {
        let mut parts = Vec::new();
        let mut value = self.low;
        while value <= self.high {
            let x = self.x(value);
            let class = if value == 0i32 { "zero" } else { "grid" };
            let label = value.checked_div(10i32).unwrap_or(0i32);
            let sign = if value > 0i32 { "+" } else { "" };
            parts.push(format!(
                "<line class=\"{class}\" x1=\"{x}\" y1=\"{AXIS_H}\" x2=\"{x}\" y2=\"{height}\"/><text class=\"tick\" x=\"{x}\" y=\"14\">{sign}{label}%</text>"
            ));
            value = value.saturating_add(STEP);
        }
        parts
    }
}

/// `+12.3%` style label of a value in tenths.
fn percent(value: i32) -> String {
    let sign = match value.cmp(&0i32) {
        Ordering::Greater => "+",
        Ordering::Less => "-",
        Ordering::Equal => "",
    };
    let abs = value.saturating_abs();
    format!("{sign}{}.{}%", abs.checked_div(10i32).unwrap_or(0i32), abs.checked_rem(10i32).unwrap_or(0i32))
}

/// One row: compound (first row of its group), detail, dot and value, linked to the file.
fn row(scale: &Scale, arm: &Arm, top: i32, first: bool) -> String {
    let value = arm.median().unwrap_or(0i32);
    let middle = top.saturating_add(ROW_H.checked_div(2i32).unwrap_or(0i32));
    let text_y = middle.saturating_add(4i32);
    let state = if arm.is_unverified() { " unverified" } else { "" };
    let check = if arm.is_unverified() { " (to check)" } else { "" };
    let mut parts = vec![format!(
        "<a href=\"{}\"><title>{}: {}{check}</title>",
        escape(&blob(arm.file())),
        escape(arm.id()),
        percent(value)
    )];
    if first {
        parts.push(format!(
            "<line class=\"group\" x1=\"0\" y1=\"{top}\" x2=\"{}\" y2=\"{top}\"/><text class=\"compound\" x=\"2\" y=\"{text_y}\">{}</text>",
            scale.width(),
            escape(arm.compound())
        ));
    }
    let (x, end) = (scale.x(value), scale.x(scale.high).saturating_add(6i32));
    parts.push(format!(
        "<text class=\"detail\" x=\"{COMPOUND_W}\" y=\"{text_y}\">{}</text><circle class=\"dot {}{state}\" cx=\"{x}\" cy=\"{middle}\" r=\"{DOT_R}\"/><text class=\"value\" x=\"{end}\" y=\"{text_y}\">{}{check}</text></a>",
        escape(&arm.detail()),
        arm.role_class(),
        percent(value)
    ));
    parts.concat()
}

/// The whole plot as an SVG element; `arms` must all have a median and be sorted by compound.
pub(crate) fn svg(arms: &[&Arm]) -> String {
    let scale = Scale::new(arms);
    let count = i32::try_from(arms.len()).unwrap_or(i32::MAX);
    let height = AXIS_H.saturating_add(count.saturating_mul(ROW_H)).saturating_add(4i32);
    let width = scale.width();
    let mut parts = vec![format!(
        "<svg class=\"forest\" xmlns=\"http://www.w3.org/2000/svg\" viewBox=\"0 0 {width} {height}\" width=\"{width}\" height=\"{height}\" role=\"img\" aria-label=\"Median lifespan change per arm\">"
    )];
    parts.extend(scale.ticks(height));
    let mut previous = "";
    for (arm, index) in arms.iter().zip(0i32..) {
        let top = AXIS_H.saturating_add(index.saturating_mul(ROW_H));
        parts.push(row(&scale, arm, top, arm.compound() != previous));
        previous = arm.compound();
    }
    parts.push(String::from("</svg>"));
    parts.concat()
}

#[cfg(test)]
mod tests {
    use super::{percent, svg};
    use crate::project::findings::itp::parse;

    #[test]
    fn labels() {
        assert_eq!(percent(85), "+8.5%", "positive");
        assert_eq!(percent(-53), "-5.3%", "negative");
        assert_eq!(percent(0), "0.0%", "zero");
    }

    /// Two aspirin arms: one gain still to check, one verified null.
    fn chart() -> String {
        let one = parse(
            "id: a-f\ncompound: aspirin\nrole: extends\neffect:\n  median_pct: 8.5\nverification:\n  to_verify:\n    - n.treated\n",
            "literature/lifespan/itp/a.yaml",
        );
        let two = parse(
            "id: a-m\ncompound: aspirin\nrole: \"null\"\neffect:\n  median_pct: -5.3\nverification:\n  to_verify: []\n",
            "b.yaml",
        );
        svg(&[&one, &two])
    }

    #[test]
    fn plot_shape_and_determinism() {
        let plot = chart();
        assert!(plot.starts_with("<svg") && plot.ends_with("</svg>"), "one svg element");
        assert!(plot.contains("class=\"zero\""), "zero line");
        assert!(plot.contains(">-10%<") && plot.contains(">+10%<"), "axis rounded out to 5 %");
        assert_eq!(plot, chart(), "same bytes");
    }

    #[test]
    fn plot_marks_and_links() {
        let plot = chart();
        assert!(plot.contains("dot extends unverified") && plot.contains("(to check)"), "unverified marked");
        assert!(plot.contains("dot no-effect\""), "verified null arm");
        assert_eq!(plot.matches("class=\"compound\"").count(), 1, "one compound heading");
        assert!(plot.contains("literature/lifespan/itp/a.yaml"), "links to its file");
    }
}
