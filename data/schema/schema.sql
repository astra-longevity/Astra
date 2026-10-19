-- Astra: SQLite schema of data/astra.sqlite.
-- This file is the truth for the GENERATED database. The truth of the DATA is
-- the YAML files in data/. `astra db build` runs this file on an empty
-- database, then inserts the YAML (see BUILD_SPEC.md).
-- No migrations: the database is thrown away and rebuilt on every build.

PRAGMA foreign_keys = ON;

-- Evidence scale, loaded from data/schema/evidence-levels.yaml.
CREATE TABLE evidence_levels (
  level      INTEGER PRIMARY KEY CHECK (level BETWEEN 1 AND 6),
  code       TEXT NOT NULL UNIQUE,
  label      TEXT NOT NULL,
  definition TEXT NOT NULL
) STRICT;

CREATE TABLE missions (
  id      TEXT PRIMARY KEY,
  title   TEXT NOT NULL,
  folder  TEXT NOT NULL,
  summary TEXT,
  status  TEXT CHECK (status IN ('active','completed')),   -- optional
  owner   TEXT,                                            -- optional agent id, e.g. A2
  file    TEXT NOT NULL            -- path of the YAML, relative to the repo root
) STRICT;

CREATE TABLE sources (
  id           TEXT PRIMARY KEY,      -- lowercase DOI (10.xxxx/...) or isbn:97...
  doi          TEXT UNIQUE,           -- = id for a DOI, NULL for an ISBN
  title        TEXT NOT NULL,
  year         INTEGER NOT NULL,
  authors      TEXT NOT NULL,         -- JSON: array of "Family, Given"
  first_author TEXT NOT NULL,         -- family name of the 1st author (display)
  journal      TEXT,
  type         TEXT NOT NULL CHECK (type IN ('study','review','trial','meta-analysis','book','book-chapter','preprint','guideline','other')),
  status       TEXT NOT NULL CHECK (status IN ('verified','to_verify')),
  verified_via TEXT CHECK (verified_via IN ('crossref','pubmed','isbn','manual')),
  verified_at  TEXT,
  note         TEXT,
  file         TEXT NOT NULL
) STRICT;

CREATE TABLE concepts (
  id          TEXT PRIMARY KEY,
  kind        TEXT NOT NULL CHECK (kind IN ('hallmark','gene','protein','pathway','process','intervention','disease','biomarker','organism','cell-type','tissue','other')),
  name        TEXT NOT NULL,
  description TEXT,
  file        TEXT NOT NULL
) STRICT;

CREATE TABLE concept_synonyms (
  concept_id TEXT NOT NULL REFERENCES concepts(id),
  synonym    TEXT NOT NULL,
  PRIMARY KEY (concept_id, synonym)
) STRICT, WITHOUT ROWID;

CREATE TABLE claims (
  id             TEXT PRIMARY KEY,
  statement      TEXT NOT NULL,
  organism       TEXT NOT NULL REFERENCES concepts(id),
  evidence_level INTEGER NOT NULL REFERENCES evidence_levels(level),
  confidence     TEXT NOT NULL CHECK (confidence IN ('established','debated','speculative')),
  mission        TEXT NOT NULL REFERENCES missions(id),
  note           TEXT,
  file           TEXT NOT NULL
) STRICT;

CREATE TABLE claim_concepts (
  claim_id   TEXT NOT NULL REFERENCES claims(id),
  concept_id TEXT NOT NULL REFERENCES concepts(id),
  position   INTEGER NOT NULL,        -- order in the YAML
  PRIMARY KEY (claim_id, concept_id)
) STRICT, WITHOUT ROWID;

CREATE TABLE evidence (
  claim_id  TEXT NOT NULL REFERENCES claims(id),
  source_id TEXT NOT NULL REFERENCES sources(id),
  direction TEXT NOT NULL CHECK (direction IN ('supports','contradicts','nuances')),
  locator   TEXT,
  note      TEXT,
  file      TEXT NOT NULL,
  PRIMARY KEY (claim_id, source_id)
) STRICT, WITHOUT ROWID;

CREATE TABLE relations (
  id      TEXT PRIMARY KEY,
  from_id TEXT NOT NULL REFERENCES concepts(id),
  to_id   TEXT NOT NULL REFERENCES concepts(id),
  type    TEXT NOT NULL CHECK (type IN ('causes','aggravates','inhibits','activates','marker_of','treats','prevents','part_of','associated_with')),
  note    TEXT,
  file    TEXT NOT NULL,
  CHECK (from_id <> to_id),
  UNIQUE (from_id, to_id, type)
) STRICT;

CREATE TABLE relation_claims (
  relation_id TEXT NOT NULL REFERENCES relations(id),
  claim_id    TEXT NOT NULL REFERENCES claims(id),
  PRIMARY KEY (relation_id, claim_id)
) STRICT, WITHOUT ROWID;

CREATE TABLE questions (
  id        TEXT PRIMARY KEY,
  statement TEXT NOT NULL,
  mission   TEXT NOT NULL REFERENCES missions(id),
  status    TEXT NOT NULL CHECK (status IN ('open','resolved')),
  note      TEXT,
  file      TEXT NOT NULL
) STRICT;

CREATE TABLE question_concepts (
  question_id TEXT NOT NULL REFERENCES questions(id),
  concept_id  TEXT NOT NULL REFERENCES concepts(id),
  PRIMARY KEY (question_id, concept_id)
) STRICT, WITHOUT ROWID;

-- Claims that answer a resolved question.
CREATE TABLE question_resolved_by (
  question_id TEXT NOT NULL REFERENCES questions(id),
  claim_id    TEXT NOT NULL REFERENCES claims(id),
  PRIMARY KEY (question_id, claim_id)
) STRICT, WITHOUT ROWID;

CREATE TABLE contradictions (
  id          TEXT PRIMARY KEY,
  mission     TEXT NOT NULL REFERENCES missions(id),
  explanation TEXT NOT NULL,
  status      TEXT CHECK (status IN ('open','resolved')),   -- optional
  file        TEXT NOT NULL
) STRICT;

CREATE TABLE contradiction_claims (
  contradiction_id TEXT NOT NULL REFERENCES contradictions(id),
  claim_id         TEXT NOT NULL REFERENCES claims(id),
  PRIMARY KEY (contradiction_id, claim_id)
) STRICT, WITHOUT ROWID;

-- Build metadata (versions, counts). No timestamp: the build is deterministic.
CREATE TABLE build_info (
  key   TEXT PRIMARY KEY,
  value TEXT NOT NULL
) STRICT;

-- Indexes for "reverse direction" lookups.
CREATE INDEX idx_evidence_source          ON evidence(source_id, claim_id);
CREATE INDEX idx_claim_concepts_concept   ON claim_concepts(concept_id, claim_id);
CREATE INDEX idx_claims_mission           ON claims(mission);
CREATE INDEX idx_relations_to             ON relations(to_id);
CREATE INDEX idx_relation_claims_claim    ON relation_claims(claim_id);
CREATE INDEX idx_question_concepts_concept ON question_concepts(concept_id);
CREATE INDEX idx_questions_mission        ON questions(mission, status);
CREATE INDEX idx_contradiction_claims_claim ON contradiction_claims(claim_id);
CREATE INDEX idx_sources_status           ON sources(status);

-- Views read by the interface (A11).

-- For each claim, its sources.
CREATE VIEW sources_by_claim AS
SELECT e.claim_id, e.direction, e.locator,
       s.id AS source_id, s.doi, s.title, s.year, s.first_author, s.journal,
       s.type AS source_type, s.status AS source_status
FROM evidence e JOIN sources s ON s.id = e.source_id;

-- For each source, the claims it supports, contradicts or nuances.
CREATE VIEW claims_by_source AS
SELECT e.source_id, e.direction, e.locator,
       c.id AS claim_id, c.statement, c.evidence_level, c.confidence, c.mission
FROM evidence e JOIN claims c ON c.id = e.claim_id;

-- Summary per claim (counters + "source still to verify" flag).
CREATE VIEW claim_summary AS
SELECT c.id, c.statement, c.mission, c.organism, c.confidence,
       c.evidence_level, l.label AS evidence_label,
       (SELECT COUNT(*) FROM evidence e WHERE e.claim_id = c.id AND e.direction = 'supports')    AS n_supports,
       (SELECT COUNT(*) FROM evidence e WHERE e.claim_id = c.id AND e.direction = 'contradicts') AS n_contradicts,
       (SELECT COUNT(*) FROM evidence e WHERE e.claim_id = c.id AND e.direction = 'nuances')     AS n_nuances,
       EXISTS (SELECT 1 FROM evidence e JOIN sources s ON s.id = e.source_id
               WHERE e.claim_id = c.id AND s.status = 'to_verify')                              AS has_unverified_source
FROM claims c JOIN evidence_levels l ON l.level = c.evidence_level;

-- Concept card: number of claims, relations, open questions.
CREATE VIEW concept_summary AS
SELECT k.id, k.kind, k.name,
       (SELECT COUNT(*) FROM claim_concepts cc WHERE cc.concept_id = k.id) AS n_claims,
       (SELECT COUNT(*) FROM relations r WHERE r.from_id = k.id OR r.to_id = k.id) AS n_relations,
       (SELECT COUNT(*) FROM question_concepts qc JOIN questions q ON q.id = qc.question_id
         WHERE qc.concept_id = k.id AND q.status = 'open') AS n_open_questions
FROM concepts k;

-- Sources still "to verify" (dashboard).
CREATE VIEW unverified_sources AS
SELECT s.id, s.title, s.year, s.first_author,
       (SELECT COUNT(*) FROM evidence e WHERE e.source_id = s.id) AS n_claims
FROM sources s WHERE s.status = 'to_verify';
