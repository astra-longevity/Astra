# Spécification de `astra db build`

Destinataire : Rust Manager (A10). Propriétaire du format : Database Manager (A9).
Version du format : **0.1**. Toute divergence entre ce document et les schémas
`data/schema/*.schema.json` / `schema.sql` est un bug à signaler à A9.

## 1. Rôle

Lire tous les fichiers YAML de `data/`, les valider, et produire
`data/astra.sqlite` (gitignoré). La base est **jetable** : on la supprime et on
la reconstruit entièrement à chaque build. Aucune migration, aucun état hors du dépôt.

```
astra db build            # valide + écrit data/astra.sqlite
astra db build --check    # valide seulement, n'écrit rien (CI, hook pre-commit)
astra db build --root P   # racine du dépôt (défaut : dossier courant, doit contenir data/)
```

## 2. Entrées

| Dossier (relatif à la racine) | Type | Schéma | Nom de fichier attendu |
|---|---|---|---|
| `data/schema/evidence-levels.yaml` | échelle | (voir §4, R01) | — |
| `data/missions/*.yaml` | Mission | `mission.schema.json` | `<id>.yaml` |
| `data/sources/*.yaml` | Source | `source.schema.json` | `filename(id).yaml` |
| `data/concepts/*.yaml` | Concept | `concept.schema.json` | `<id>.yaml` |
| `data/claims/*.yaml` | Claim | `claim.schema.json` | `<id>.yaml` |
| `data/evidence/*.yaml` | Evidence | `evidence.schema.json` | `<claim>--filename(source).yaml` |
| `data/relations/*.yaml` | Relation | `relation.schema.json` | `<id>.yaml` |
| `data/questions/*.yaml` | Question | `question.schema.json` | `<id>.yaml` |
| `data/contradictions/*.yaml` | Contradiction | `contradiction.schema.json` | `<id>.yaml` |

- Un fichier = un objet (mapping YAML à la racine). Extension `.yaml` uniquement ;
  tout autre fichier dans ces dossiers (sauf `.gitkeep` et `README.md`) est une erreur E002.
- Dossier absent = zéro objet (pas une erreur).
- `data/examples/` est **ignoré** par le build normal (voir §8).
- Encodage UTF-8 obligatoire, NFC. Pas de sous-dossiers.
- YAML : sous-ensemble sûr (pas d'ancres/alias, pas de tags `!`, pas de clés dupliquées).

**`filename(source_id)`** : DOI → remplacer `/` par `_` ; tout caractère hors
`[a-z0-9._()+;-]` est encodé `%xx` (hexa minuscule, octets UTF-8).
ISBN → `isbn:9780000000000` devient `isbn-9780000000000`.
Ex. `10.1038/nature02517` → `10.1038_nature02517.yaml`.

## 3. Ordre de chargement

1. Exécuter `schema.sql` sur une base **neuve en mémoire** (`PRAGMA foreign_keys=ON`).
2. Charger `evidence-levels.yaml`.
3. Lire et valider (schéma JSON) **tous** les fichiers, toutes catégories.
4. Insérer dans cet ordre : missions → sources → concepts (+ concept_synonyms)
   → claims (+ claim_concepts) → evidence → relations (+ relation_claims)
   → questions (+ question_concepts, question_resolved_by)
   → contradictions (+ contradiction_claims).
5. Vérifier les règles transverses (§4, R10 et suivantes).
6. Remplir `build_info` (§6).
7. `VACUUM INTO` un fichier temporaire `data/astra.sqlite.tmp`, puis renommage
   atomique vers `data/astra.sqlite`. En cas d'erreur, l'ancien fichier n'est pas touché.

Le build **collecte toutes les erreurs** avant de s'arrêter (ne pas s'arrêter à
la première), sauf erreur fatale E001/E003.

## 4. Règles de validation

Format de chaque message (une ligne, sur stderr) :
`<niveau> <code> <fichier>[:<champ>] : <message en français>`
avec niveau `erreur` ou `avertissement`. Les messages entre « » sont à reproduire
tels quels ; `{x}` = valeur interpolée.

### Lecture et forme

| Code | Niveau | Règle | Message |
|---|---|---|---|
| R01/E001 | erreur fatale | `evidence-levels.yaml` existe et contient exactement les niveaux 1 à 6, codes uniques | « échelle de preuve invalide : {détail} » |
| E002 | erreur | fichier inattendu dans un dossier de données | « fichier inattendu (attendu : *.yaml) » |
| E003 | erreur fatale | `schema.sql` absent ou ne s'exécute pas | « schema.sql invalide : {erreur sqlite} » |
| E010 | erreur | YAML illisible / non UTF-8 / ancre, tag ou clé dupliquée | « YAML invalide : {détail} » |
| E011 | erreur | non conforme au JSON Schema (champ manquant, champ inconnu, type, enum, motif) | « {champ} : {raison} » (une ligne par violation) |
| E012 | erreur | nom de fichier ≠ nom attendu (§2) | « le fichier devrait s'appeler {attendu} » |
| E013 | erreur | id en double dans une même catégorie (y compris DOI ne différant que par la casse) | « id {id} déjà défini dans {autre fichier} » |
| E014 | erreur | DOI non canonique : majuscules, préfixe `doi:` / `https://doi.org/`, espaces | « DOI non canonique, écrire {forme canonique} » |
| E015 | erreur | doublons dans une liste (concepts, claims, synonyms) | « {champ} : valeur en double {v} » |

### Références (pas d'orphelins)

| Code | Niveau | Règle | Message |
|---|---|---|---|
| E020 | erreur | toute référence pointe vers un id existant : claim.concepts, claim.organism, claim.mission, evidence.claim, evidence.source, relation.from/to/claims, question.concepts/mission/resolved_by, contradiction.claims/mission | « {champ} : {id} n'existe pas » |
| E021 | erreur | `claim.organism` désigne un concept de `kind: organism` | « organism : {id} n'est pas un organisme (kind={kind}) » |
| E022 | erreur | relation `from` ≠ `to` | « une relation ne peut pas relier un concept à lui-même » |
| E023 | erreur | pas deux relations avec le même (from, to, type) | « relation identique à {autre id} » |
| E024 | erreur | une Evidence par paire (claim, source) | « paire déjà décrite dans {autre fichier} » |

### Règles de preuve

| Code | Niveau | Règle | Message |
|---|---|---|---|
| E030 | erreur | chaque claim a ≥ 1 Evidence | « affirmation sans source » |
| E031 | erreur | chaque relation : chacune de ses claims a ≥ 1 Evidence (découle de E030 ; contrôlé pour un message clair) | « relation appuyée sur {claim} qui n'a aucune source » |
| E032 | erreur | `confidence: solide` ⇒ ≥ 1 Evidence `direction: soutient` vers une source `status: vérifié` | « confiance « solide » sans source vérifiée qui la soutient » |
| E033 | erreur | contradiction : ≥ 2 claims distinctes (schéma) | « une contradiction relie au moins 2 affirmations » |
| E034 | erreur | question `status: résolue` ⇒ `resolved_by` non vide (schéma) ; `ouverte` ⇒ `resolved_by` absent | « resolved_by interdit sur une question ouverte » |
| E035 | erreur | `source.status: vérifié` ⇒ `verified_via` et `verified_at` présents (schéma) | « source vérifiée sans verified_via/verified_at » |
| W040 | avertissement | claim citant une source `à vérifier` | « s'appuie sur une source à vérifier : {source} » |
| W041 | avertissement | claim dont aucune Evidence n'est `soutient` | « aucune source ne soutient cette affirmation » |
| W042 | avertissement | source jamais citée par une Evidence | « source citée par aucune affirmation » |
| W043 | avertissement | concept jamais référencé | « concept utilisé nulle part » |
| W044 | avertissement | `verified_at` dans le futur ou année de publication > année courante + 1 | « date improbable : {v} » |

Le niveau de preuve (`evidence_level`) est **déclaré** par l'auteur et n'est pas
recalculé en v0.1 (on ne connaît pas le type d'étude de chaque source). Règle
éditoriale : niveau = le plus haut atteint par les sources qui soutiennent.

## 5. Transformations à l'insertion

- `sources.doi` = id si DOI, `NULL` si ISBN.
- `sources.authors` = JSON compact (`["Trifunovic, Aleksandra",…]`, sans espaces
  hors des chaînes, Unicode non échappé) ; `first_author` = partie avant la
  première virgule du premier auteur.
- `claim_concepts.position` = index 0-based dans la liste YAML.
- Colonne `file` = chemin relatif à la racine, séparateur `/`.
- Chaînes YAML multi-lignes : conserver telles quelles (pas de trim interne),
  supprimer seulement le `\n` final.

## 6. Déterminisme

Deux builds sur le même commit doivent produire des fichiers **octet pour octet identiques**.

- Fichiers parcourus en ordre lexicographique d'octets du chemin ; insertions
  dans cet ordre, listes enfants dans l'ordre du YAML.
- Aucun horodatage, nom de machine, chemin absolu ou aléa dans la base.
- `build_info` contient exactement : `format_version`=`0.1`,
  `schema_sql_sha256`, `n_<table>` pour chaque table de données. Pas de date.
- Fin de build : `PRAGMA user_version = 1`, puis `VACUUM INTO` (réécrit les pages
  proprement). Pas de mode WAL.
- Test CI recommandé : builder deux fois, comparer les SHA-256.

## 7. Sortie et codes de retour

stdout (succès) : une ligne par table `table : N`, puis
`data/astra.sqlite écrit (N avertissements)` ou, avec `--check`,
`données valides (N avertissements)`.

| Code | Sens |
|---|---|
| 0 | succès (avertissements possibles) |
| 1 | au moins une erreur de validation (E0xx) ; rien n'est écrit |
| 2 | erreur d'usage (option inconnue, `--root` sans `data/`) |
| 3 | erreur d'E/S ou SQLite interne (disque, droits) |

`--strict` (optionnel) : les avertissements deviennent des erreurs (code 1).

## 8. Exemples

`astra db build --root . --data-dir data/examples --check` doit valider les
exemples de `data/examples/` (même arborescence que `data/`). C'est le test
minimal du validateur : le CI doit le lancer. Si `--data-dir` n'est pas
retenu par A10, prévoir un test Rust qui charge ce dossier.
