//! Page assembly: gathers template variables and writes every HTML file.

use std::fs;
use std::path::Path;

use minijinja::value::merge_maps;
use minijinja::{Environment, Value, context, path_loader};
use rusqlite::Connection;

use crate::docs::{self, MissionDir};
use crate::git::{self, History};
use crate::query::{self, rows, text};
use crate::{BuildReport, SiteError, SitePaths, io_at};
use crate::{glossary, markdown};

/// Number of commits shown on the dashboard.
const RECENT_COMMITS: usize = 8;

/// D109: attribute of the banner element every page must carry.
pub(crate) const BANNER_MARKER: &str = "data-astra-banner";
/// D109: text of the banner.
pub(crate) const BANNER_TEXT: &str = "Research prototype, not medical advice.";
/// D109: attribute that marks a score on a page (any element showing one).
pub(crate) const SCORE_MARKER: &str = "data-astra-score";
/// D88 review record, relative to the repository root. Scores are allowed only
/// once this file exists and holds the line [`D88_DONE`].
pub(crate) const D88_RECORD: &str = "docs/decisions/d88-legal-medical-review.md";
/// Line that marks the D88 review as done in [`D88_RECORD`].
pub(crate) const D88_DONE: &str = "Status: done";

/// True when the D88 legal/medical-advice review is recorded as done.
fn d88_review_done(repo: &Path) -> bool {
    fs::read_to_string(repo.join(D88_RECORD)).is_ok_and(|text| text.lines().any(|line| line.trim() == D88_DONE))
}

/// D109 launch gate on one rendered page.
fn check_gate(file: &str, html: &str, scores_allowed: bool) -> Result<(), SiteError> {
    if !html.contains(BANNER_MARKER) || !html.contains(BANNER_TEXT) {
        return Err(SiteError::gate(file, "the \"Research prototype, not medical advice\" banner is missing"));
    }
    if !scores_allowed && html.contains(SCORE_MARKER) {
        return Err(SiteError::gate(file, &format!("shows a score but {D88_RECORD} does not record the D88 review")));
    }
    Ok(())
}

/// One page to render.
struct Page {
    /// Output path relative to the site root.
    file: String,
    /// Template file name.
    template: &'static str,
    /// Page-specific variables.
    vars: Value,
}

impl Page {
    /// Page at the site root.
    fn top(name: &'static str, vars: Value) -> Self {
        Self { file: format!("{name}.html"), template: leak_name(name), vars }
    }
}

/// Template file of a top-level page (`index` gives `index.html`).
fn leak_name(name: &'static str) -> &'static str {
    match name {
        "index" => "index.html",
        "map" => "map.html",
        "claims" => "claims.html",
        "sources" => "sources.html",
        "questions" => "questions.html",
        "journal" => "journal.html",
        _ => "glossary.html",
    }
}

/// Renders the whole site into `paths.output`.
pub(crate) fn render_site(conn: &Connection, paths: &SitePaths) -> Result<BuildReport, SiteError> {
    let history = History::load(&paths.repo);
    let entries = glossary::load(&paths.repo)?;
    let base = context! {
        site => history.site(),
        glossary => entries.iter().map(glossary::Entry::to_value).collect::<Vec<Value>>(),
    };
    let mut pages = top_pages(conn, paths, &history)?;
    pages.push(Page::top("glossary", glossary_vars(&entries)));
    pages.extend(claim_pages(conn)?);
    pages.extend(source_pages(conn)?);
    pages.extend(doc_pages(&paths.repo)?);

    let templates = paths.repo.join("interface").join("templates");
    let mut env = Environment::new();
    if !templates.join(markdown::TEMPLATE_NAME).is_file() {
        env.add_template(markdown::TEMPLATE_NAME, markdown::DEFAULT_TEMPLATE)?;
    }
    env.set_loader(path_loader(templates));
    let scores_allowed = d88_review_done(&paths.repo);
    let rendered = pages
        .iter()
        .map(|page| {
            let html = render_page(&env, page, &base)?;
            check_gate(&page.file, &html, scores_allowed)?;
            Ok((page.file.as_str(), html))
        })
        .collect::<Result<Vec<_>, SiteError>>()?;
    reset_output(&paths.output)?;
    for (file, html) in rendered {
        write_file(&paths.output, file, &html)?;
    }
    copy_static(&paths.repo.join("interface").join("static"), &paths.output.join("static"))?;
    Ok(BuildReport { pages: pages.len() })
}

/// Top-level pages except the glossary.
fn top_pages(conn: &Connection, paths: &SitePaths, history: &History) -> Result<Vec<Page>, SiteError> {
    let mission_rows = rows(conn, query::MISSIONS, &[])?;
    let dirs: Vec<MissionDir> =
        mission_rows.iter().map(|row| MissionDir::new(text(row, "id"), text(row, "folder"))).collect();
    let names = rows(conn, query::MISSION_NAMES, &[])?;
    let index = context! {
        stats => query::one(conn, query::STATS, &[])?,
        missions => mission_rows.iter().map(|row| dashboard_mission(&paths.repo, row)).collect::<Vec<Value>>(),
        recent => history.recent(RECENT_COMMITS),
    };
    let questions = context! {
        missions => names,
        questions => rows(conn, query::QUESTIONS, &[])?,
        contradictions => query::contradictions(conn)?,
        to_verify => docs::to_verify(&paths.repo, &dirs)?,
    };
    Ok(vec![
        Page::top("index", index),
        Page::top("map", context! { families => docs::families(conn, &paths.repo)? }),
        Page::top("claims", context! { claims => rows(conn, query::CLAIMS, &[])?, missions => names }),
        Page::top("sources", context! { sources => query::sources(conn)? }),
        Page::top("questions", questions),
        Page::top("journal", context! { areas => history.areas(), days => history.days() }),
    ])
}

/// Dashboard row of one mission, dated by the last commit in its folder.
fn dashboard_mission(repo: &Path, row: &Value) -> Value {
    let updated = git::last_change(repo, &text(row, "folder"));
    context! { updated_at => updated, ..row.clone() }
}

/// Glossary page variables.
fn glossary_vars(entries: &[glossary::Entry]) -> Value {
    let sorted: Vec<Value> = glossary::sorted(entries).iter().map(glossary::Entry::to_value).collect();
    context! { glossary_sorted => sorted }
}

/// One page per claim.
fn claim_pages(conn: &Connection) -> Result<Vec<Page>, SiteError> {
    rows(conn, query::CLAIMS, &[])?
        .into_iter()
        .map(|claim| {
            let id = text(&claim, "id");
            let vars = context! {
                concepts => rows(conn, query::CLAIM_CONCEPTS, &[id.as_str()])?,
                evidence => query::claim_evidence(conn, &id)?,
                relations => rows(conn, query::CLAIM_RELATED, &[id.as_str()])?,
                claim => claim,
            };
            Ok(Page { file: format!("claims/{id}.html"), template: "claim.html", vars })
        })
        .collect()
}

/// One page per source.
fn source_pages(conn: &Connection) -> Result<Vec<Page>, SiteError> {
    query::sources(conn)?
        .into_iter()
        .map(|source| {
            let slug = text(&source, "slug");
            let id = text(&source, "id");
            let vars = context! { claims => rows(conn, query::SOURCE_CLAIMS, &[id.as_str()])?, source => source };
            Ok(Page { file: format!("sources/{slug}.html"), template: "source.html", vars })
        })
        .collect()
}

/// One page per literature note, at the same path with `.html`.
fn doc_pages(repo: &Path) -> Result<Vec<Page>, SiteError> {
    docs::literature_files(repo)?
        .into_iter()
        .map(|path| {
            let text = fs::read_to_string(&path).map_err(io_at(&path))?;
            let file = docs::relative_name(repo, &path);
            let vars = markdown::doc_vars(&text, &file);
            Ok(Page { file: markdown::html_name(&file), template: markdown::TEMPLATE_NAME, vars })
        })
        .collect()
}

/// Renders one page to HTML.
fn render_page(env: &Environment<'_>, page: &Page, base: &Value) -> Result<String, SiteError> {
    let root = "../".repeat(page.file.matches('/').count());
    let page_id = page.template.trim_end_matches(".html");
    let own = context! { root => root, page => page_id };
    let vars = merge_maps([own, page.vars.clone(), base.clone()]);
    Ok(env.get_template(page.template)?.render(vars)?)
}

/// Writes `html` at `file` under `out`.
fn write_file(out: &Path, file: &str, html: &str) -> Result<(), SiteError> {
    let target = out.join(file);
    if let Some(parent) = target.parent() {
        fs::create_dir_all(parent).map_err(io_at(parent))?;
    }
    fs::write(&target, html).map_err(io_at(&target))
}

/// Deletes a previous build. Refuses a non-empty folder without `index.html`,
/// so a wrong `--out` cannot wipe unrelated files.
fn reset_output(out: &Path) -> Result<(), SiteError> {
    if out.exists() {
        let empty = fs::read_dir(out).map_err(io_at(out))?.next().is_none();
        if !empty && !out.join("index.html").is_file() {
            return Err(SiteError::unsafe_output(out));
        }
        fs::remove_dir_all(out).map_err(io_at(out))?;
    }
    fs::create_dir_all(out).map_err(io_at(out))
}

/// Copies `from` into `to`, recursively.
fn copy_static(from: &Path, to: &Path) -> Result<(), SiteError> {
    if !from.is_dir() {
        return Ok(());
    }
    fs::create_dir_all(to).map_err(io_at(to))?;
    for item in fs::read_dir(from).map_err(io_at(from))? {
        let path = item.map_err(io_at(from))?.path();
        let Some(name) = path.file_name() else {
            continue;
        };
        let target = to.join(name);
        if path.is_dir() {
            copy_static(&path, &target)?;
        } else {
            let _bytes: u64 = fs::copy(&path, &target).map_err(io_at(&path))?;
        }
    }
    Ok(())
}
