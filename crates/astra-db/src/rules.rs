//! Cross-file rules (`BUILD_SPEC.md` §4): references E020–E024, evidence
//! rules E030–E034 and warnings W040–W044.

use std::collections::{BTreeMap, BTreeSet};
use std::time::{SystemTime, UNIX_EPOCH};

use serde_json::Value;

use crate::diag::Diags;
use crate::load::{Corpus, Record};
use crate::schema::{OPEN, SOLID, SUPPORTS, TO_VERIFY, VERIFIED};

/// Kind of object a reference points to.
#[derive(Clone, Copy, Debug)]
enum Target {
    /// A mission id.
    Mission,
    /// A concept id.
    Concept,
    /// A claim id.
    Claim,
    /// A source id.
    Source,
}

/// References of a claim.
const CLAIM_REFS: [(&str, Target); 3] =
    [("concepts", Target::Concept), ("organism", Target::Concept), ("mission", Target::Mission)];
/// References of an evidence link.
const EVIDENCE_REFS: [(&str, Target); 2] = [("claim", Target::Claim), ("source", Target::Source)];
/// References of a relation.
const RELATION_REFS: [(&str, Target); 3] =
    [("from", Target::Concept), ("to", Target::Concept), ("claims", Target::Claim)];
/// References of a question.
const QUESTION_REFS: [(&str, Target); 3] =
    [("concepts", Target::Concept), ("mission", Target::Mission), ("resolved_by", Target::Claim)];
/// References of a contradiction.
const CONTRADICTION_REFS: [(&str, Target); 2] = [("claims", Target::Claim), ("mission", Target::Mission)];
/// Fields that count as a use of a concept (W043).
const CONCEPT_USES: [(&str, &str); 5] = [
    ("claims", "concepts"),
    ("claims", "organism"),
    ("relations", "from"),
    ("relations", "to"),
    ("questions", "concepts"),
];

/// One evidence link seen from its claim.
#[derive(Debug)]
struct Link<'corp> {
    /// Source id.
    source: &'corp str,
    /// `supports`, `contradicts` or `nuances`.
    direction: &'corp str,
}

/// Lookup tables over the whole corpus.
#[derive(Debug)]
struct Index<'corp> {
    /// Mission ids.
    missions: BTreeSet<&'corp str>,
    /// Concept id → kind.
    concepts: BTreeMap<&'corp str, &'corp str>,
    /// Claim ids.
    claims: BTreeSet<&'corp str>,
    /// Source id → status.
    sources: BTreeMap<&'corp str, &'corp str>,
    /// Claim id → its evidence links.
    links: BTreeMap<&'corp str, Vec<Link<'corp>>>,
}

/// Ids of a folder.
fn ids(records: &[Record]) -> BTreeSet<&str> {
    records.iter().filter_map(|record| record.text("id")).collect()
}

/// Id → value of `key`, for a folder.
fn keyed<'corp>(records: &'corp [Record], key: &str) -> BTreeMap<&'corp str, &'corp str> {
    records.iter().filter_map(|record| Some((record.text("id")?, record.text(key).unwrap_or_default()))).collect()
}

/// Evidence links grouped by claim.
fn links(evidence: &[Record]) -> BTreeMap<&str, Vec<Link<'_>>> {
    let mut map: BTreeMap<&str, Vec<Link<'_>>> = BTreeMap::new();
    for record in evidence {
        if let (Some(claim), Some(source)) = (record.text("claim"), record.text("source")) {
            let direction = record.text("direction").unwrap_or_default();
            map.entry(claim).or_default().push(Link { source, direction });
        }
    }
    map
}

/// Values of a field that holds either one id or a list of ids.
fn values<'rec>(record: &'rec Record, field: &str) -> Vec<&'rec str> {
    record.text(field).map_or_else(|| record.list(field), |one| vec![one])
}

impl<'corp> Index<'corp> {
    /// Builds the tables.
    fn new(corpus: &'corp Corpus) -> Self {
        Self {
            missions: ids(corpus.records("missions")),
            concepts: keyed(corpus.records("concepts"), "kind"),
            claims: ids(corpus.records("claims")),
            sources: keyed(corpus.records("sources"), "status"),
            links: links(corpus.records("evidence")),
        }
    }

    /// Whether `id` names an existing object of kind `target`.
    fn exists(&self, target: Target, id: &str) -> bool {
        match target {
            Target::Mission => self.missions.contains(id),
            Target::Concept => self.concepts.contains_key(id),
            Target::Claim => self.claims.contains(id),
            Target::Source => self.sources.contains_key(id),
        }
    }

    /// Whether the source `id` has status `status`.
    fn source_is(&self, id: &str, status: &str) -> bool {
        self.sources.get(id).is_some_and(|found| *found == status)
    }
}

/// Runs the cross-file rules.
struct Checker<'corp, 'run> {
    /// Lookup tables.
    index: Index<'corp>,
    /// Collected diagnostics.
    diags: &'run mut Diags,
}

impl Checker<'_, '_> {
    /// E020 for every listed reference field.
    fn check_refs(&mut self, record: &Record, fields: &[(&str, Target)]) {
        for (field, target) in fields.iter().copied() {
            for id in values(record, field) {
                if !self.index.exists(target, id) {
                    self.diags.error("E020", &record.file, format!("{field}: {id} does not exist"));
                }
            }
        }
    }

    /// E021: the organism must be a concept of kind `organism`.
    fn check_organism(&mut self, record: &Record) {
        let wrong = record
            .text("organism")
            .and_then(|org| Some((org, self.index.concepts.get(org).copied()?)))
            .filter(|&(_, kind)| kind != "organism");
        if let Some((org, kind)) = wrong {
            let msg = format!("organism: {org} is not an organism (kind={kind})");
            self.diags.error("E021", &record.file, msg);
        }
    }

    /// E030, E032, W040, W041 for one claim.
    fn check_claim_sources(&mut self, record: &Record) {
        let id = record.text("id").unwrap_or_default();
        let links = self.index.links.get(id).map(Vec::as_slice).unwrap_or_default();
        if links.is_empty() {
            self.diags.error("E030", &record.file, "claim without a source".to_owned());
            return;
        }
        if !links.iter().any(|link| link.direction == SUPPORTS) {
            let msg = "no source supports this claim".to_owned();
            self.diags.warn("W041", &record.file, msg);
        }
        let verified_support =
            links.iter().any(|link| link.direction == SUPPORTS && self.index.source_is(link.source, VERIFIED));
        if record.text("confidence") == Some(SOLID) && !verified_support {
            let msg = "confidence \"established\" without a verified source supporting it".to_owned();
            self.diags.error("E032", &record.file, msg);
        }
        for link in links.iter().filter(|link| self.index.source_is(link.source, TO_VERIFY)) {
            let msg = format!("relies on a source still to verify: {}", link.source);
            self.diags.warn("W040", &record.file, msg);
        }
    }

    /// E024: one evidence file per (claim, source) pair.
    fn check_evidence(&mut self, records: &[Record]) {
        let mut seen: BTreeMap<(&str, &str), &str> = BTreeMap::new();
        for record in records {
            self.check_refs(record, &EVIDENCE_REFS);
            let pair = (record.text("claim").unwrap_or_default(), record.text("source").unwrap_or_default());
            if let Some(other) = seen.get(&pair).copied() {
                let msg = format!("pair already described in {other}");
                self.diags.error("E024", &record.file, msg);
            } else {
                let _previous: Option<&str> = seen.insert(pair, &record.file);
            }
        }
    }

    /// E022 and E031 for one relation.
    fn check_relation(&mut self, record: &Record) {
        let from = record.text("from").unwrap_or_default();
        if !from.is_empty() && Some(from) == record.text("to") {
            let msg = "a relation cannot link a concept to itself".to_owned();
            self.diags.error("E022", &record.file, msg);
        }
        for claim in record.list("claims") {
            if self.index.claims.contains(claim) && !self.index.links.contains_key(claim) {
                let msg = format!("relation backed by {claim}, which has no source");
                self.diags.error("E031", &record.file, msg);
            }
        }
    }

    /// E020–E023, E031 over all relations.
    fn check_relations(&mut self, records: &[Record]) {
        let mut seen: BTreeMap<(&str, &str, &str), &str> = BTreeMap::new();
        for record in records {
            self.check_refs(record, &RELATION_REFS);
            self.check_relation(record);
            let key = (
                record.text("from").unwrap_or_default(),
                record.text("to").unwrap_or_default(),
                record.text("type").unwrap_or_default(),
            );
            let id = record.text("id").unwrap_or_default();
            if let Some(other) = seen.get(&key).copied() {
                self.diags.error("E023", &record.file, format!("relation identical to {other}"));
            } else {
                let _previous: Option<&str> = seen.insert(key, id);
            }
        }
    }

    /// E034: an open question cannot be resolved.
    fn check_question(&mut self, record: &Record) {
        if record.text("status") == Some(OPEN) && record.value.get("resolved_by").is_some() {
            let msg = "resolved_by not allowed on an open question".to_owned();
            self.diags.error("E034", &record.file, msg);
        }
    }

    /// W044: verification date in the future or publication year too far ahead.
    fn check_dates(&mut self, record: &Record, today: &str) {
        if let Some(date) = record.text("verified_at").filter(|date| *date > today) {
            self.diags.warn("W044", &record.file, format!("unlikely date: {date}"));
        }
        let year = record.value.get("year").and_then(Value::as_i64);
        let limit = today.get(..4).and_then(|text| text.parse::<i64>().ok()).and_then(|now| now.checked_add(1));
        if let (Some(found), Some(max)) = (year, limit)
            && found > max
        {
            self.diags.warn("W044", &record.file, format!("unlikely date: {found}"));
        }
    }

    /// W042 and W044 over all sources.
    fn check_sources(&mut self, corpus: &Corpus, today: &str) {
        let cited: BTreeSet<&str> = corpus.records("evidence").iter().filter_map(|ev| ev.text("source")).collect();
        for record in corpus.records("sources") {
            if record.text("id").is_some_and(|id| !cited.contains(id)) {
                let msg = "source not cited by any claim".to_owned();
                self.diags.warn("W042", &record.file, msg);
            }
            self.check_dates(record, today);
        }
    }

    /// W043 over all concepts.
    fn check_concepts(&mut self, corpus: &Corpus) {
        let mut used: BTreeSet<&str> = BTreeSet::new();
        for (dir, field) in CONCEPT_USES {
            for record in corpus.records(dir) {
                used.extend(values(record, field));
            }
        }
        for record in corpus.records("concepts") {
            if record.text("id").is_some_and(|id| !used.contains(id)) {
                self.diags.warn("W043", &record.file, "concept not used anywhere".to_owned());
            }
        }
    }
}

/// Runs every cross-file rule; `today` is the current date `YYYY-MM-DD` (W044).
pub fn check(corpus: &Corpus, diags: &mut Diags, today: &str) {
    let mut checker = Checker { index: Index::new(corpus), diags };
    checker.check_sources(corpus, today);
    checker.check_concepts(corpus);
    for record in corpus.records("claims") {
        checker.check_refs(record, &CLAIM_REFS);
        checker.check_organism(record);
        checker.check_claim_sources(record);
    }
    checker.check_evidence(corpus.records("evidence"));
    checker.check_relations(corpus.records("relations"));
    for record in corpus.records("questions") {
        checker.check_refs(record, &QUESTION_REFS);
        checker.check_question(record);
    }
    for record in corpus.records("contradictions") {
        checker.check_refs(record, &CONTRADICTION_REFS);
    }
}

/// Seconds in a day.
const DAY_SECONDS: u64 = 86_400;

/// Year of era (0..=399) of a day of era (Hinnant).
fn year_of_era(doe: i64) -> Option<i64> {
    doe.checked_sub(doe.checked_div(1460)?)?
        .checked_add(doe.checked_div(36_524)?)?
        .checked_sub(doe.checked_div(146_096)?)?
        .checked_div(365)
}

/// Day of year (March-based) from day of era and year of era.
fn day_of_year(doe: i64, yoe: i64) -> Option<i64> {
    let start = yoe.checked_mul(365)?.checked_add(yoe.checked_div(4)?)?.checked_sub(yoe.checked_div(100)?)?;
    doe.checked_sub(start)
}

/// `(month, day)` of a March-based day of year.
fn month_day(doy: i64) -> Option<(i64, i64)> {
    let mp = doy.checked_mul(5)?.checked_add(2)?.checked_div(153)?;
    let day = doy.checked_sub(mp.checked_mul(153)?.checked_add(2)?.checked_div(5)?)?.checked_add(1)?;
    let month = if mp < 10 { mp.checked_add(3)? } else { mp.checked_sub(9)? };
    Some((month, day))
}

/// Civil date `YYYY-MM-DD` of a day count since 1970-01-01 (H. Hinnant's algorithm).
#[must_use]
pub fn civil(days: i64) -> Option<String> {
    let shifted = days.checked_add(719_468)?;
    let era = shifted.checked_div_euclid(146_097)?;
    let doe = shifted.checked_rem_euclid(146_097)?;
    let yoe = year_of_era(doe)?;
    let (month, day) = month_day(day_of_year(doe, yoe)?)?;
    let base = yoe.checked_add(era.checked_mul(400)?)?;
    let year = if month <= 2 { base.checked_add(1)? } else { base };
    Some(format!("{year:04}-{month:02}-{day:02}"))
}

/// Today's date (UTC), or `9999-12-31` if the clock is unusable (disables W044).
#[must_use]
pub fn today() -> String {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .ok()
        .and_then(|elapsed| elapsed.as_secs().checked_div(DAY_SECONDS))
        .and_then(|days| i64::try_from(days).ok())
        .and_then(civil)
        .unwrap_or_else(|| "9999-12-31".to_owned())
}
