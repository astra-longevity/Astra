//! Literature pages: each `literature/**/*.md` becomes an HTML page at the
//! same relative path (`.md` replaced by `.html`), with links between notes
//! rewritten the same way.

use minijinja::{Value, context};
use pulldown_cmark::{CowStr, Event, Options, Parser, Tag, html};

/// Template used when `interface/templates/doc.html` does not exist.
pub(crate) const DEFAULT_TEMPLATE: &str = r#"{% extends "base.html" %}
{% block title %}{{ doc.title }}{% endblock %}
{% block content %}
<nav class="crumbs"><a href="{{ root }}map.html">Ageing map</a></nav>
<article class="doc">
{{ doc.html | safe }}
</article>
<p class="muted">Source file: <code>{{ doc.file }}</code></p>
{% endblock %}
"#;

/// Name of the literature page template.
pub(crate) const TEMPLATE_NAME: &str = "doc.html";

/// Markdown file name turned into its HTML page name.
pub(crate) fn html_name(file: &str) -> String {
    file.strip_suffix(".md").map_or_else(|| file.to_owned(), |stem| format!("{stem}.html"))
}

/// Page variables `doc { title, file, html }` for one markdown file.
pub(crate) fn doc_vars(text: &str, file: &str) -> Value {
    let title = title_of(text).unwrap_or_else(|| file.to_owned());
    context! { doc => context! { title => title, file => file, html => render(text) } }
}

/// First level-1 heading, without the `# ` marker.
fn title_of(text: &str) -> Option<String> {
    text.lines().find_map(|line| line.strip_prefix("# ")).map(|title| title.trim().to_owned())
}

/// Renders markdown to HTML, rewriting relative `.md` links to `.html`.
pub(crate) fn render(text: &str) -> String {
    let options = Options::ENABLE_TABLES | Options::ENABLE_STRIKETHROUGH | Options::ENABLE_FOOTNOTES;
    let events = Parser::new_ext(text, options).map(fix_link);
    let mut out = String::with_capacity(text.len());
    html::push_html(&mut out, events);
    out
}

/// Rewrites the target of a relative link to a markdown note.
fn fix_link(event: Event<'_>) -> Event<'_> {
    if let Event::Start(Tag::Link { link_type, dest_url, title, id }) = event {
        let dest = md_to_html(&dest_url).map_or(dest_url, CowStr::from);
        Event::Start(Tag::Link { link_type, dest_url: dest, title, id })
    } else {
        event
    }
}

/// `08-x.md#refs` gives `08-x.html#refs`; absolute URLs and non-notes give `None`.
pub(crate) fn md_to_html(url: &str) -> Option<String> {
    if url.contains("://") || url.starts_with('/') || url.starts_with("mailto:") {
        return None;
    }
    let (path, anchor) = url.split_once('#').map_or((url, None), |pair| (pair.0, Some(pair.1)));
    let stem = path.strip_suffix(".md")?;
    Some(anchor.map_or_else(|| format!("{stem}.html"), |frag| format!("{stem}.html#{frag}")))
}

#[cfg(test)]
mod tests {
    use super::{html_name, md_to_html, render};

    #[test]
    fn rewrites_note_links_only() {
        assert_eq!(md_to_html("08-x.md").as_deref(), Some("08-x.html"), "plain");
        assert_eq!(md_to_html("../a/b.md#refs").as_deref(), Some("../a/b.html#refs"), "anchor");
        assert_eq!(md_to_html("https://doi.org/x.md"), None, "absolute");
        assert_eq!(md_to_html("#top"), None, "anchor only");
        assert_eq!(html_name("literature/a.md"), "literature/a.html", "page name");
    }

    #[test]
    fn renders_tables_and_links() {
        let html = render("# T\n\n[see](02-b.md)\n\n| a | b |\n|---|---|\n| 1 | 2 |\n");
        assert!(html.contains("href=\"02-b.html\""), "link rewritten: {html}");
        assert!(html.contains("<table>"), "table rendered");
        assert!(html.contains("<h1>T</h1>"), "heading");
    }
}
