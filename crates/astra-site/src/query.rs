//! Database reads. Each row becomes a template map keyed by column name, so
//! the SQL aliases below are the template variable names of `interface/README.md`.

use std::collections::BTreeMap;

use minijinja::{Value, context};
use rusqlite::types::ValueRef;
use rusqlite::{Connection, Row, params_from_iter};

/// Source columns shared by lists and detail pages (slug, authors, labels).
const SOURCE_COLS: &str = "s.id AS id, replace(replace(s.id, '/', '_'), ':', '_') AS slug,
  s.doi AS doi, s.title AS title, s.year AS year, s.journal AS journal,
  s.status AS status, s.type AS type,
  (SELECT group_concat(value, '; ') FROM json_each(s.authors)) AS authors,
  CASE json_array_length(s.authors)
    WHEN 1 THEN s.first_author
    WHEN 2 THEN s.first_author || ' & ' || rtrim(substr(json_extract(s.authors, '$[1]'), 1,
      instr(json_extract(s.authors, '$[1]') || ',', ',') - 1))
    ELSE s.first_author || ' et al.' END AS authors_short,
  CASE s.type
    WHEN 'study' THEN 'Study'
    WHEN 'review' THEN 'Literature review'
    WHEN 'trial' THEN 'Clinical trial'
    WHEN 'meta-analysis' THEN 'Meta-analysis'
    WHEN 'book' THEN 'Book'
    WHEN 'book-chapter' THEN 'Book chapter'
    WHEN 'preprint' THEN 'Preprint (not yet peer-reviewed)'
    WHEN 'guideline' THEN 'Official guideline'
    ELSE 'Other' END AS type_label";

/// Dashboard counters.
pub(crate) const STATS: &str = "SELECT
  (SELECT COUNT(*) FROM missions) AS missions,
  (SELECT COUNT(*) FROM claims) AS claims,
  (SELECT COUNT(*) FROM sources) AS sources,
  (SELECT COUNT(*) FROM sources) - (SELECT COUNT(*) FROM unverified_sources) AS sources_verified,
  (SELECT COUNT(*) FROM questions WHERE status = 'open') AS questions_open";

/// Missions with their folder (the folder dates the mission in git).
pub(crate) const MISSIONS: &str = "SELECT id, title, summary, folder, status, owner FROM missions ORDER BY id";

/// Mission names for filters.
pub(crate) const MISSION_NAMES: &str = "SELECT id, title FROM missions ORDER BY id";

/// All claims, with display fields.
pub(crate) const CLAIMS: &str = "SELECT c.id AS id, c.statement AS statement,
  k.name AS organism, l.code AS evidence_level, c.confidence AS confidence,
  c.mission AS mission, m.title AS mission_title,
  (SELECT COUNT(*) FROM evidence e WHERE e.claim_id = c.id) AS source_count
FROM claims c
JOIN concepts k ON k.id = c.organism
JOIN evidence_levels l ON l.level = c.evidence_level
JOIN missions m ON m.id = c.mission
ORDER BY c.id";

/// Concepts of one claim, in YAML order.
pub(crate) const CLAIM_CONCEPTS: &str = "SELECT k.name AS name FROM claim_concepts cc
JOIN concepts k ON k.id = cc.concept_id WHERE cc.claim_id = ?1 ORDER BY cc.position";

/// Claims linked to one claim: same relation, or same contradiction.
pub(crate) const CLAIM_RELATED: &str = "SELECT r.type AS type, o.id AS id, o.statement AS statement
FROM relation_claims rc
JOIN relations r ON r.id = rc.relation_id
JOIN relation_claims ro ON ro.relation_id = rc.relation_id AND ro.claim_id <> rc.claim_id
JOIN claims o ON o.id = ro.claim_id
WHERE rc.claim_id = ?1
UNION
SELECT 'in tension', o.id, o.statement
FROM contradiction_claims xc
JOIN contradiction_claims xo ON xo.contradiction_id = xc.contradiction_id AND xo.claim_id <> xc.claim_id
JOIN claims o ON o.id = xo.claim_id
WHERE xc.claim_id = ?1
ORDER BY 2, 1";

/// Claims using one source (view made for the interface).
pub(crate) const SOURCE_CLAIMS: &str = "SELECT claim_id AS id, statement, confidence, direction, locator
FROM claims_by_source WHERE source_id = ?1 ORDER BY claim_id";

/// Questions with their mission title.
pub(crate) const QUESTIONS: &str = "SELECT q.statement AS text, q.status AS status,
  q.mission AS mission, m.title AS mission_title, q.note AS note
FROM questions q JOIN missions m ON m.id = q.mission ORDER BY q.status, q.id";

/// Contradictions, flattened to their first and last claim.
const CONTRADICTIONS: &str = "SELECT x.explanation, x.mission,
  a.id, a.statement, b.id, b.statement, x.status
FROM contradictions x
JOIN claims a ON a.id = (SELECT MIN(claim_id) FROM contradiction_claims WHERE contradiction_id = x.id)
JOIN claims b ON b.id = (SELECT MAX(claim_id) FROM contradiction_claims WHERE contradiction_id = x.id)
ORDER BY x.id";

/// Counters for one hallmark concept (?1 = concept id).
pub(crate) const HALLMARK: &str = "SELECT
  (SELECT COUNT(DISTINCT cc.claim_id) FROM claim_concepts cc WHERE cc.concept_id = ?1) AS claim_count,
  (SELECT COUNT(DISTINCT e.source_id) FROM claim_concepts cc
     JOIN evidence e ON e.claim_id = cc.claim_id WHERE cc.concept_id = ?1) AS source_count,
  (SELECT c.confidence FROM claim_concepts cc JOIN claims c ON c.id = cc.claim_id
     WHERE cc.concept_id = ?1 GROUP BY c.confidence
     ORDER BY COUNT(*) DESC, c.confidence LIMIT 1) AS confidence";

/// All sources, newest first, with their claim count.
pub(crate) fn sources(conn: &Connection) -> Result<Vec<Value>, rusqlite::Error> {
    let sql = format!(
        "SELECT {SOURCE_COLS},
  (SELECT COUNT(*) FROM evidence e WHERE e.source_id = s.id) AS claim_count
FROM sources s ORDER BY s.year DESC, s.id"
    );
    rows(conn, &sql, &[])
}

/// Sources of one claim: supporting first, then nuancing, then contradicting.
pub(crate) fn claim_evidence(conn: &Connection, claim: &str) -> Result<Vec<Value>, rusqlite::Error> {
    let sql = format!(
        "SELECT e.direction AS direction, e.locator AS locator, {SOURCE_COLS}
FROM evidence e JOIN sources s ON s.id = e.source_id
WHERE e.claim_id = ?1 ORDER BY e.direction DESC, s.year DESC, s.id"
    );
    rows(conn, &sql, &[claim])
}

/// Contradictions shaped as `{ summary, status, mission, claim_a, claim_b }`.
pub(crate) fn contradictions(conn: &Connection) -> Result<Vec<Value>, rusqlite::Error> {
    let mut stmt = conn.prepare(CONTRADICTIONS)?;
    let mapped = stmt.query_map([], |row| {
        Ok(context! {
            summary => row.get::<_, String>(0)?,
            status => row.get::<_, Option<String>>(6)?,
            mission => row.get::<_, String>(1)?,
            claim_a => context! { id => row.get::<_, String>(2)?, statement => row.get::<_, String>(3)? },
            claim_b => context! { id => row.get::<_, String>(4)?, statement => row.get::<_, String>(5)? },
        })
    })?;
    mapped.collect()
}

/// Runs `sql` with text parameters; each row becomes a map keyed by column name.
pub(crate) fn rows(conn: &Connection, sql: &str, args: &[&str]) -> Result<Vec<Value>, rusqlite::Error> {
    let mut stmt = conn.prepare(sql)?;
    let names: Vec<String> = stmt.column_names().into_iter().map(String::from).collect();
    let mut cursor = stmt.query(params_from_iter(args.iter()))?;
    let mut found = Vec::new();
    while let Some(row) = cursor.next()? {
        found.push(row_value(row, &names)?);
    }
    Ok(found)
}

/// First row of `sql`, or an undefined value when there is none.
pub(crate) fn one(conn: &Connection, sql: &str, args: &[&str]) -> Result<Value, rusqlite::Error> {
    Ok(rows(conn, sql, args)?.into_iter().next().unwrap_or_default())
}

/// Text of a map field, empty when absent or null.
pub(crate) fn text(row: &Value, key: &str) -> String {
    row.get_attr(key)
        .ok()
        .filter(|val| !val.is_none() && !val.is_undefined())
        .map_or_else(String::new, |val| val.to_string())
}

/// Converts one row to a map.
fn row_value(row: &Row<'_>, names: &[String]) -> Result<Value, rusqlite::Error> {
    let map = names
        .iter()
        .enumerate()
        .map(|(idx, name)| Ok((name.clone(), cell(row.get_ref(idx)?))))
        .collect::<Result<BTreeMap<String, Value>, rusqlite::Error>>()?;
    Ok(Value::from(map))
}

/// Converts one `SQLite` cell to a template value (blobs are not used).
fn cell(raw: ValueRef<'_>) -> Value {
    match raw {
        ValueRef::Null | ValueRef::Blob(_) => Value::from(()),
        ValueRef::Integer(num) => Value::from(num),
        ValueRef::Real(num) => Value::from(num),
        ValueRef::Text(bytes) => Value::from(String::from_utf8_lossy(bytes).into_owned()),
    }
}
