//! Static SVG Gantt chart of the obstacles: one row per obstacle, a month
//! axis, a bar from today to the exit date, a diamond for the milestone.
//! Integer arithmetic only, so the same inputs give the same bytes.

use super::obstacles::Obstacle;

/// Short month names, January first.
const MONTHS: [&str; 12] = ["Jan", "Feb", "Mar", "Apr", "May", "Jun", "Jul", "Aug", "Sep", "Oct", "Nov", "Dec"];
/// Width of the label column, in pixels.
const LABEL_W: u32 = 70;
/// Width of one month, in pixels.
const MONTH_W: u32 = 48;
/// Height of one row, in pixels.
const ROW_H: u32 = 24;
/// Height of the month axis, in pixels.
const AXIS_H: u32 = 34;
/// Days counted per month for positions inside a month.
const MONTH_DAYS: u32 = 31;

/// A `YYYY-MM-DD` date as (absolute month number, day of month).
fn parse_date(date: &str) -> Option<(u32, u32)> {
    let mut parts = date.split('-');
    let year: u32 = parts.next()?.parse().ok()?;
    let month: u32 = parts.next()?.parse().ok()?;
    let day: u32 = parts.next()?.parse().ok()?;
    let valid = (1..=12).contains(&month) && (1..=MONTH_DAYS).contains(&day);
    valid.then(|| (year.saturating_mul(12).saturating_add(month).saturating_sub(1), day))
}

/// Escapes text for SVG content and attributes.
pub(crate) fn escape(text: &str) -> String {
    text.replace('&', "&amp;").replace('<', "&lt;").replace('>', "&gt;").replace('"', "&quot;")
}

/// Horizontal geometry: the first month shown and how many.
struct Axis {
    /// Absolute month number of the first column.
    first: u32,
    /// Number of month columns.
    months: u32,
}

impl Axis {
    /// X pixel of `date`, clamped to the chart; `None` if unparsable.
    fn x(&self, date: &str) -> Option<u32> {
        let (month, day) = parse_date(date)?;
        let column = month.saturating_sub(self.first).min(self.months);
        let inside = day.saturating_sub(1).saturating_mul(MONTH_W).checked_div(MONTH_DAYS).unwrap_or(0);
        Some(LABEL_W.saturating_add(column.saturating_mul(MONTH_W)).saturating_add(inside))
    }

    /// Total width in pixels.
    const fn width(&self) -> u32 {
        LABEL_W.saturating_add(self.months.saturating_mul(MONTH_W)).saturating_add(8)
    }

    /// Month labels and grid lines.
    fn ticks(&self, height: u32) -> Vec<String> {
        (0..self.months)
            .map(|step| {
                let month = self.first.saturating_add(step);
                let x = LABEL_W.saturating_add(step.saturating_mul(MONTH_W));
                let (year, index) = (month.checked_div(12).unwrap_or(0), month.checked_rem(12).unwrap_or(0));
                let name = MONTHS.get(usize::try_from(index).unwrap_or(0)).copied().unwrap_or("");
                let year_text = if index == 0 || step == 0 { format!("{year}") } else { String::new() };
                format!(
                    "<line class=\"grid\" x1=\"{x}\" y1=\"{AXIS_H}\" x2=\"{x}\" y2=\"{height}\"/><text class=\"month\" x=\"{}\" y=\"14\">{name}</text><text class=\"year\" x=\"{}\" y=\"28\">{year_text}</text>",
                    x.saturating_add(4),
                    x.saturating_add(4)
                )
            })
            .collect()
    }
}

/// Axis from the month of `today` to the month after the latest date.
fn axis(obstacles: &[Obstacle], today: &str) -> Axis {
    let months: Vec<u32> = obstacles
        .iter()
        .flat_map(|item| [item.date(), item.milestone_date()])
        .filter_map(parse_date)
        .map(|pair| pair.0)
        .collect();
    let earliest = months.iter().copied().min().unwrap_or(0);
    let first = parse_date(today).map_or(earliest, |pair| pair.0);
    let last = months.iter().copied().max().unwrap_or(first).max(first);
    Axis { first, months: last.saturating_sub(first).saturating_add(2) }
}

/// One row: label, bar to the exit, exit mark, milestone diamond, linked to the file.
fn row(axis: &Axis, item: &Obstacle, top: u32, today: &str) -> String {
    let mid = top.saturating_add(ROW_H.checked_div(2).unwrap_or(0));
    let start = axis.x(today).unwrap_or(LABEL_W);
    let state = if item.is_closed() {
        "done"
    } else if item.is_late(today) {
        "late"
    } else {
        "open"
    };
    let mut parts = vec![format!(
        "<a href=\"{}\"><title>{}: {} (exit {})</title><text class=\"code\" x=\"4\" y=\"{}\">{}</text>",
        escape(&super::blob(item.file())),
        escape(&item.code()),
        escape(item.title()),
        escape(item.date()),
        mid.saturating_add(4),
        escape(&item.code())
    )];
    if let Some(end) = axis.x(item.date()) {
        if !item.is_closed() && end > start {
            parts.push(format!(
                "<rect class=\"bar {state}\" x=\"{start}\" y=\"{}\" width=\"{}\" height=\"8\"/>",
                mid.saturating_sub(4),
                end.saturating_sub(start)
            ));
        }
        parts.push(format!(
            "<rect class=\"exit {state}\" x=\"{}\" y=\"{}\" width=\"4\" height=\"16\"/>",
            end.saturating_sub(2),
            mid.saturating_sub(8)
        ));
    }
    if let Some(at) = axis.x(item.milestone_date()) {
        let kind = if item.milestone_late(today) { "late" } else { state };
        parts.push(format!(
            "<path class=\"milestone {kind}\" d=\"M{at} {} l6 6 l-6 6 l-6 -6 z\"/>",
            mid.saturating_sub(6)
        ));
    }
    parts.push(String::from("</a>"));
    parts.concat()
}

/// The whole chart as an SVG element; `today` is a `YYYY-MM-DD` date.
pub(crate) fn svg(obstacles: &[Obstacle], today: &str) -> String {
    let axis = axis(obstacles, today);
    let count = u32::try_from(obstacles.len()).unwrap_or(u32::MAX);
    let height = AXIS_H.saturating_add(count.saturating_mul(ROW_H)).saturating_add(4);
    let width = axis.width();
    let mut parts = vec![format!(
        "<svg class=\"gantt\" xmlns=\"http://www.w3.org/2000/svg\" viewBox=\"0 0 {width} {height}\" width=\"{width}\" height=\"{height}\" role=\"img\" aria-label=\"Obstacle timeline\">"
    )];
    parts.extend(axis.ticks(height));
    for (item, index) in obstacles.iter().zip(0u32..) {
        let top = AXIS_H.saturating_add(index.saturating_mul(ROW_H));
        parts.push(row(&axis, item, top, today));
    }
    if let Some(now) = axis.x(today) {
        parts.push(format!(
            "<line class=\"today\" x1=\"{now}\" y1=\"{AXIS_H}\" x2=\"{now}\" y2=\"{height}\"/><text class=\"today-label\" x=\"{}\" y=\"{height}\">today</text>",
            now.saturating_add(3)
        ));
    }
    parts.push(String::from("</svg>"));
    parts.concat()
}

#[cfg(test)]
mod tests {
    use super::{escape, parse_date, svg};
    use crate::project::obstacles::parse;

    #[test]
    fn dates_and_escaping() {
        assert_eq!(parse_date("2026-10-20"), Some((24_321, 20)), "absolute month");
        assert_eq!(parse_date("2026-13-01"), None, "bad month");
        assert_eq!(escape("a<b & \"c\""), "a&lt;b &amp; &quot;c&quot;", "escaped");
    }

    #[test]
    fn late_rows_milestones_and_determinism() {
        let items = vec![
            parse("id: o1-a\ntitle: A & B\ndate: 2026-10-01\nstatus: open\n", "research/obstacles/a.yaml"),
            parse("id: o2-b\ndate: 2027-04-20\nstatus: open\nmilestone_date: 2027-01-20\n", "b.yaml"),
            parse("id: v4-c\ndate: 2026-10-20\nstatus: met\n", "c.yaml"),
        ];
        let chart = svg(&items, "2026-10-20");
        assert!(chart.starts_with("<svg") && chart.ends_with("</svg>"), "one svg element");
        assert!(chart.contains("exit late"), "past exit drawn as late");
        assert!(chart.contains("milestone open"), "milestone diamond");
        assert!(chart.contains("exit done"), "met obstacle marked");
    }

    #[test]
    fn escapes_axis_and_determinism() {
        let items = vec![
            parse("id: o1-a\ntitle: A & B\ndate: 2026-10-01\nstatus: open\n", "a.yaml"),
            parse("id: o2-b\ndate: 2027-04-20\nstatus: open\n", "b.yaml"),
        ];
        let chart = svg(&items, "2026-10-20");
        assert!(chart.contains("A &amp; B"), "title escaped");
        assert!(chart.contains(">Oct<") && chart.contains(">May<"), "month axis to the month after the last date");
        assert_eq!(chart, svg(&items, "2026-10-20"), "same bytes");
    }
}
