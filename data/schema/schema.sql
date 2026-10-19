-- Astra : schéma SQLite de data/astra.sqlite.
-- Ce fichier est la vérité pour la base GÉNÉRÉE. La vérité des DONNÉES est
-- dans les fichiers YAML de data/. `astra db build` exécute ce fichier sur
-- une base vide puis insère les YAML (voir BUILD_SPEC.md).
-- Aucune migration : la base est jetée et reconstruite à chaque build.

PRAGMA foreign_keys = ON;

-- Échelle de preuve, chargée depuis data/schema/evidence-levels.yaml.
CREATE TABLE evidence_levels (
  level         INTEGER PRIMARY KEY CHECK (level BETWEEN 1 AND 6),
  code          TEXT NOT NULL UNIQUE,
  label_fr      TEXT NOT NULL,
  definition_fr TEXT NOT NULL
) STRICT;

CREATE TABLE missions (
  id         TEXT PRIMARY KEY,
  title      TEXT NOT NULL,
  folder     TEXT NOT NULL,
  summary_fr TEXT,
  file       TEXT NOT NULL            -- chemin du YAML, relatif à la racine
) STRICT;

CREATE TABLE sources (
  id           TEXT PRIMARY KEY,      -- DOI minuscule (10.xxxx/...) ou isbn:97...
  doi          TEXT UNIQUE,           -- = id si DOI, NULL pour un ISBN
  title        TEXT NOT NULL,
  year         INTEGER NOT NULL,
  authors      TEXT NOT NULL,         -- JSON : tableau de « Nom, Prénom »
  first_author TEXT NOT NULL,         -- nom de famille du 1er auteur (affichage)
  journal      TEXT,
  type         TEXT NOT NULL CHECK (type IN ('study','review','trial','meta-analysis','book','book-chapter','preprint','guideline','other')),
  status       TEXT NOT NULL CHECK (status IN ('vérifié','à vérifier')),
  verified_via TEXT CHECK (verified_via IN ('crossref','pubmed','isbn','manual')),
  verified_at  TEXT,
  note_fr      TEXT,
  file         TEXT NOT NULL
) STRICT;

CREATE TABLE concepts (
  id             TEXT PRIMARY KEY,
  kind           TEXT NOT NULL CHECK (kind IN ('hallmark','gene','protein','pathway','process','intervention','disease','biomarker','organism','cell-type','tissue','other')),
  name_fr        TEXT NOT NULL,
  name_en        TEXT NOT NULL,
  description_fr TEXT,
  file           TEXT NOT NULL
) STRICT;

CREATE TABLE concept_synonyms (
  concept_id TEXT NOT NULL REFERENCES concepts(id),
  synonym    TEXT NOT NULL,
  PRIMARY KEY (concept_id, synonym)
) STRICT, WITHOUT ROWID;

CREATE TABLE claims (
  id             TEXT PRIMARY KEY,
  statement_fr   TEXT NOT NULL,
  organism       TEXT NOT NULL REFERENCES concepts(id),
  evidence_level INTEGER NOT NULL REFERENCES evidence_levels(level),
  confidence     TEXT NOT NULL CHECK (confidence IN ('solide','débattu','spéculatif')),
  mission        TEXT NOT NULL REFERENCES missions(id),
  note_fr        TEXT,
  file           TEXT NOT NULL
) STRICT;

CREATE TABLE claim_concepts (
  claim_id   TEXT NOT NULL REFERENCES claims(id),
  concept_id TEXT NOT NULL REFERENCES concepts(id),
  position   INTEGER NOT NULL,        -- ordre dans le YAML
  PRIMARY KEY (claim_id, concept_id)
) STRICT, WITHOUT ROWID;

CREATE TABLE evidence (
  claim_id  TEXT NOT NULL REFERENCES claims(id),
  source_id TEXT NOT NULL REFERENCES sources(id),
  direction TEXT NOT NULL CHECK (direction IN ('soutient','contredit','nuance')),
  locator   TEXT,
  note_fr   TEXT,
  file      TEXT NOT NULL,
  PRIMARY KEY (claim_id, source_id)
) STRICT, WITHOUT ROWID;

CREATE TABLE relations (
  id      TEXT PRIMARY KEY,
  from_id TEXT NOT NULL REFERENCES concepts(id),
  to_id   TEXT NOT NULL REFERENCES concepts(id),
  type    TEXT NOT NULL CHECK (type IN ('cause','aggrave','inhibe','active','marqueur_de','traite','previent','partie_de','associe_a')),
  note_fr TEXT,
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
  id           TEXT PRIMARY KEY,
  statement_fr TEXT NOT NULL,
  mission      TEXT NOT NULL REFERENCES missions(id),
  status       TEXT NOT NULL CHECK (status IN ('ouverte','résolue')),
  note_fr      TEXT,
  file         TEXT NOT NULL
) STRICT;

CREATE TABLE question_concepts (
  question_id TEXT NOT NULL REFERENCES questions(id),
  concept_id  TEXT NOT NULL REFERENCES concepts(id),
  PRIMARY KEY (question_id, concept_id)
) STRICT, WITHOUT ROWID;

-- Ajout par rapport au brief : affirmations qui répondent à une question résolue.
CREATE TABLE question_resolved_by (
  question_id TEXT NOT NULL REFERENCES questions(id),
  claim_id    TEXT NOT NULL REFERENCES claims(id),
  PRIMARY KEY (question_id, claim_id)
) STRICT, WITHOUT ROWID;

CREATE TABLE contradictions (
  id             TEXT PRIMARY KEY,
  mission        TEXT NOT NULL REFERENCES missions(id),
  explanation_fr TEXT NOT NULL,
  file           TEXT NOT NULL
) STRICT;

CREATE TABLE contradiction_claims (
  contradiction_id TEXT NOT NULL REFERENCES contradictions(id),
  claim_id         TEXT NOT NULL REFERENCES claims(id),
  PRIMARY KEY (contradiction_id, claim_id)
) STRICT, WITHOUT ROWID;

-- Métadonnées du build (versions, comptes). Pas d'horodatage : build déterministe.
CREATE TABLE build_info (
  key   TEXT PRIMARY KEY,
  value TEXT NOT NULL
) STRICT;

-- Index pour les lectures « dans l'autre sens ».
CREATE INDEX idx_evidence_source          ON evidence(source_id, claim_id);
CREATE INDEX idx_claim_concepts_concept   ON claim_concepts(concept_id, claim_id);
CREATE INDEX idx_claims_mission           ON claims(mission);
CREATE INDEX idx_relations_to             ON relations(to_id);
CREATE INDEX idx_relation_claims_claim    ON relation_claims(claim_id);
CREATE INDEX idx_question_concepts_concept ON question_concepts(concept_id);
CREATE INDEX idx_questions_mission        ON questions(mission, status);
CREATE INDEX idx_contradiction_claims_claim ON contradiction_claims(claim_id);
CREATE INDEX idx_sources_status           ON sources(status);

-- Vues lues par l'interface (A11).

-- Pour chaque affirmation, ses sources.
CREATE VIEW sources_by_claim AS
SELECT e.claim_id, e.direction, e.locator,
       s.id AS source_id, s.doi, s.title, s.year, s.first_author, s.journal,
       s.type AS source_type, s.status AS source_status
FROM evidence e JOIN sources s ON s.id = e.source_id;

-- Pour chaque source, les affirmations qu'elle appuie, contredit ou nuance.
CREATE VIEW claims_by_source AS
SELECT e.source_id, e.direction, e.locator,
       c.id AS claim_id, c.statement_fr, c.evidence_level, c.confidence, c.mission
FROM evidence e JOIN claims c ON c.id = e.claim_id;

-- Résumé par affirmation (compteurs + drapeau « source à vérifier »).
CREATE VIEW claim_summary AS
SELECT c.id, c.statement_fr, c.mission, c.organism, c.confidence,
       c.evidence_level, l.label_fr AS evidence_label_fr,
       (SELECT COUNT(*) FROM evidence e WHERE e.claim_id = c.id AND e.direction = 'soutient')  AS n_soutient,
       (SELECT COUNT(*) FROM evidence e WHERE e.claim_id = c.id AND e.direction = 'contredit') AS n_contredit,
       (SELECT COUNT(*) FROM evidence e WHERE e.claim_id = c.id AND e.direction = 'nuance')    AS n_nuance,
       EXISTS (SELECT 1 FROM evidence e JOIN sources s ON s.id = e.source_id
               WHERE e.claim_id = c.id AND s.status = 'à vérifier')                           AS has_unverified_source
FROM claims c JOIN evidence_levels l ON l.level = c.evidence_level;

-- Fiche concept : nombre d'affirmations, relations, questions ouvertes.
CREATE VIEW concept_summary AS
SELECT k.id, k.kind, k.name_fr, k.name_en,
       (SELECT COUNT(*) FROM claim_concepts cc WHERE cc.concept_id = k.id) AS n_claims,
       (SELECT COUNT(*) FROM relations r WHERE r.from_id = k.id OR r.to_id = k.id) AS n_relations,
       (SELECT COUNT(*) FROM question_concepts qc JOIN questions q ON q.id = qc.question_id
         WHERE qc.concept_id = k.id AND q.status = 'ouverte') AS n_open_questions
FROM concepts k;

-- Sources encore « à vérifier » (tableau de bord).
CREATE VIEW unverified_sources AS
SELECT s.id, s.title, s.year, s.first_author,
       (SELECT COUNT(*) FROM evidence e WHERE e.source_id = s.id) AS n_claims
FROM sources s WHERE s.status = 'à vérifier';
