//! Tests: an in-memory database built from `data/schema/schema.sql` plus small
//! fixtures (mirroring `data/examples/`), rendered with the real templates.

use std::fs;
use std::path::{Path, PathBuf};

use rusqlite::Connection;

use crate::{ErrorKind, SitePaths, build, build_from};

/// Fixture rows, mirroring `data/examples/`.
const FIXTURES: &str = "
INSERT INTO evidence_levels VALUES
 (1,'in-vitro','a','a'),(2,'invertebre','b','b'),(3,'mammifere','c','c'),
 (4,'humain-observationnel','d','d'),(5,'humain-essai','e','e'),(6,'approuve','f','f');
INSERT INTO missions VALUES ('aging-map','Map of aging','literature/aging-map','First mission','completed','A2','m.yaml');
INSERT INTO concepts VALUES
 ('mus-musculus','organism','Mouse',NULL,'k1.yaml'),
 ('genomic-instability','hallmark','Genomic instability',NULL,'k2.yaml'),
 ('mitochondrial-dysfunction','hallmark','Mitochondrial dysfunction',NULL,'k3.yaml');
INSERT INTO sources VALUES
 ('10.1038/nature02517','10.1038/nature02517','Premature ageing in mice',2004,
  '[\"Trifunovic, Aleksandra\",\"Wredenberg, Anna\",\"Larsson, Nils\"]','Trifunovic','Nature','study',
  'verified','crossref','2026-10-19',NULL,'s1.yaml'),
 ('isbn:9780000000000',NULL,'A book',2020,'[\"Solo, Han\"]','Solo',NULL,'book',
  'to_verify',NULL,NULL,NULL,'s2.yaml');
INSERT INTO claims VALUES
 ('C-0001','POLG mice age fast.','mus-musculus',3,'established','aging-map',NULL,'c1.yaml'),
 ('C-0002','Uncertain effect in normal mice.','mus-musculus',3,'debated','aging-map',NULL,'c2.yaml');
INSERT INTO claim_concepts VALUES
 ('C-0001','genomic-instability',0),('C-0001','mitochondrial-dysfunction',1),('C-0002','mitochondrial-dysfunction',0);
INSERT INTO evidence VALUES
 ('C-0001','10.1038/nature02517','supports','Fig. 2',NULL,'e1.yaml'),
 ('C-0002','isbn:9780000000000','nuances',NULL,NULL,'e2.yaml');
INSERT INTO relations VALUES ('R-0001','genomic-instability','mitochondrial-dysfunction','aggravates',NULL,'r1.yaml');
INSERT INTO relation_claims VALUES ('R-0001','C-0001');
INSERT INTO questions VALUES ('Q-0001','Do mutations matter?','aging-map','open',NULL,'q1.yaml');
INSERT INTO contradictions VALUES ('X-0001','aging-map','Mutator mice versus normal mice.','open','x1.yaml');
INSERT INTO contradiction_claims VALUES ('X-0001','C-0001'),('X-0001','C-0002');
";

/// Overview fixture with one family and one hallmark.
const OVERVIEW: &str = "# 00 \u{2014} Overview\n\n### Group 1 \u{2014} The initial damage\n\nDamage.\n\n- **[01 \u{2014} Genomic instability](01-genomic-instability.md)** : DNA gets damaged.\n\n### Everything is linked\n";

/// Workspace root (two levels above this crate).
fn workspace() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR")).join("../..")
}

/// Fresh in-memory database with schema and fixtures.
fn fixture_db() -> Connection {
    let schema = fs::read_to_string(workspace().join("data/schema/schema.sql")).unwrap();
    let conn = Connection::open_in_memory().unwrap();
    conn.execute_batch(&schema).unwrap();
    conn.execute_batch(FIXTURES).unwrap();
    conn
}

/// Fake repository in a fresh temp folder: real `interface/`, fixture literature.
fn fake_repo(name: &str) -> PathBuf {
    let repo = std::env::temp_dir().join(format!("astra-site-test-{name}-{}", std::process::id()));
    if repo.exists() {
        fs::remove_dir_all(&repo).unwrap();
    }
    let lit = repo.join("literature/aging-map");
    fs::create_dir_all(&lit).unwrap();
    fs::write(lit.join("00-overview.md"), OVERVIEW).unwrap();
    fs::write(lit.join("01-genomic-instability.md"), "Line\nA book [to verify]\n").unwrap();
    let interface = repo.join("interface");
    fs::create_dir_all(interface.join("static")).unwrap();
    let real = workspace().join("interface");
    for dir in ["templates", "static"] {
        fs::create_dir_all(interface.join(dir)).unwrap();
        for item in fs::read_dir(real.join(dir)).unwrap() {
            let path = item.unwrap().path();
            let _bytes = fs::copy(&path, interface.join(dir).join(path.file_name().unwrap())).unwrap();
        }
    }
    let _bytes = fs::copy(real.join("glossary.yaml"), interface.join("glossary.yaml")).unwrap();
    repo
}

/// Reads one generated file (`/` is HTML-escaped by minijinja; decoded here).
fn page(paths: &SitePaths, file: &str) -> String {
    fs::read_to_string(paths.output.join(file)).unwrap().replace("&#x2f;", "/")
}

#[test]
fn builds_every_page() {
    let paths = SitePaths::from_repo(&fake_repo("pages"));
    let report = build_from(&fixture_db(), &paths).unwrap();
    assert_eq!(report.pages, 13, "7 top pages + 2 claims + 2 sources + 2 notes");
    for file in ["index.html", "map.html", "journal.html", "glossary.html", "static/style.css"] {
        assert!(paths.output.join(file).is_file(), "{file} missing");
    }
}

#[test]
fn claim_and_source_pages_link_both_ways() {
    let paths = SitePaths::from_repo(&fake_repo("links"));
    let _report = build_from(&fixture_db(), &paths).unwrap();
    let claim = page(&paths, "claims/C-0001.html");
    assert!(claim.contains("../sources/10.1038_nature02517.html"), "evidence link");
    assert!(claim.contains("Trifunovic et al."), "authors_short");
    assert!(claim.contains("claims/C-0002.html"), "contradiction partner listed");
    let source = page(&paths, "sources/isbn_9780000000000.html");
    assert!(source.contains("Solo"), "single author");
    assert!(source.contains("claims/C-0002.html"), "claims_by_source direction");
}

#[test]
fn map_and_questions_read_literature() {
    let paths = SitePaths::from_repo(&fake_repo("map"));
    let _report = build_from(&fixture_db(), &paths).unwrap();
    let map = page(&paths, "map.html");
    assert!(map.contains("The initial damage"), "family name");
    assert!(
        map.contains("1 claim \u{b7} 1 source") || map.contains("1 affirmation \u{b7} 1 source"),
        "hallmark counters"
    );
    let questions = page(&paths, "questions.html");
    assert!(questions.contains("literature/aging-map/01-genomic-instability.md"), "to_verify file");
    assert!(questions.contains("Mutator mice versus normal mice."), "contradiction");
}

#[test]
fn rebuild_is_byte_identical() {
    let paths = SitePaths::from_repo(&fake_repo("determinism"));
    let _first = build_from(&fixture_db(), &paths).unwrap();
    let before = format!("{}{}", page(&paths, "claims.html"), page(&paths, "sources.html"));
    let _second = build_from(&fixture_db(), &paths).unwrap();
    let after = format!("{}{}", page(&paths, "claims.html"), page(&paths, "sources.html"));
    assert_eq!(before, after, "same inputs, same bytes");
}

#[test]
fn refuses_foreign_output_dir() {
    let mut paths = SitePaths::from_repo(&fake_repo("unsafe"));
    paths.output = paths.repo.join("literature");
    let kind = build_from(&fixture_db(), &paths).map(|_| ()).map_err(|err| err.kind());
    assert_eq!(kind, Err(ErrorKind::UnsafeOutput), "must not wipe literature/");
}

#[test]
fn missing_database_is_reported() {
    let paths = SitePaths::from_repo(&fake_repo("nodb"));
    let kind = build(&paths).map(|_| ()).map_err(|err| err.kind());
    assert_eq!(kind, Err(ErrorKind::MissingDatabase), "no sqlite file");
}

#[test]
fn literature_notes_become_pages() {
    let paths = SitePaths::from_repo(&fake_repo("docs"));
    let _report = build_from(&fixture_db(), &paths).unwrap();
    let overview = page(&paths, "literature/aging-map/00-overview.html");
    assert!(overview.contains("href=\"01-genomic-instability.html\""), "note link rewritten");
    assert!(overview.contains("href=\"../../static/style.css\""), "root follows depth");
    let note = page(&paths, "literature/aging-map/01-genomic-instability.html");
    assert!(note.contains("A book"), "note body rendered");
    assert!(note.contains("doc prose"), "interface/templates/doc.html used");
    let map = page(&paths, "map.html");
    assert!(map.contains("literature/aging-map/01-genomic-instability.html"), "doc_href filled");
}
