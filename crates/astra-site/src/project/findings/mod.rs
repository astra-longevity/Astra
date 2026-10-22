//! Findings page: forest plot of the ITP lifespan arms, coverage map of the
//! biological-age axes and the list of reproducible research runs.

pub(crate) mod axes;
pub(crate) mod challenge;
pub(crate) mod cost;
pub(crate) mod forest;
pub(crate) mod itp;
pub(crate) mod runs;

use std::path::Path;

use minijinja::{Value, context};

use super::blob;
use crate::SiteError;

/// Value part of a `key: value` YAML line: trailing comment and quotes removed.
pub(crate) fn scalar(raw: &str) -> String {
    let text = raw.split(" #").next().unwrap_or_default().trim();
    text.trim_matches('"').to_owned()
}

/// Variables of the findings page.
pub(crate) fn vars(repo: &Path) -> Result<Value, SiteError> {
    let arms = itp::load(repo)?;
    let plotted: Vec<&itp::Arm> = arms.iter().filter(|arm| arm.median().is_some()).collect();
    let missing: Vec<Value> = arms.iter().filter(|arm| arm.median().is_none()).map(itp::Arm::to_value).collect();
    let unverified = plotted.iter().filter(|arm| arm.is_unverified()).count();
    let found_axes = axes::load(repo)?;
    Ok(context! {
        forest => forest::svg(&plotted),
        arm_count => arms.len(),
        plotted_count => plotted.len(),
        unverified_count => unverified,
        missing => missing,
        itp_href => blob(itp::DIR),
        axis_groups => axes::groups(&found_axes),
        axis_counts => axes::counts(&found_axes),
        axes_href => blob(axes::FILE),
        challenge => challenge::load(repo)?,
        challenge_href => blob(challenge::FILE),
        cost => cost::load(repo)?,
        cost_href => blob(cost::FILE),
        identifiability_href => blob("research/sim/v0/identifiability.md"),
        public_data_href => blob("research/sim/v0/design/public-data.md"),
        runs => runs::load(repo)?,
        runs_href => blob(runs::DIR),
    })
}
