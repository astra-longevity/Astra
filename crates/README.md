# crates/ — le code Rust d'Astra

Chaque dossier ici est une « crate » (un module Rust compilé séparément).
Toutes partagent les mêmes règles de qualité, définies une seule fois dans le
`Cargo.toml` à la racine (`[lints] workspace = true` dans chaque crate).

| Crate | Rôle |
|-------|------|
| `astra-cli` | Le programme `astra`, point d'entrée unique (`astra db build`, `astra site build`). Pour l'instant il affiche seulement sa version. |
| `astra-db` | Bibliothèque qui construira la base `data/astra.sqlite` à partir des fichiers YAML de `data/`. |
| `astra-site` | Bibliothèque qui générera le site web (l'interface) à partir du dépôt. |

## Règles

- Les règles sont très strictes : presque tous les avertissements du compilateur
  et de `clippy` (l'outil d'analyse de Rust) sont des erreurs bloquantes.
- Il est interdit de désactiver une règle localement (`#[allow]` ou `#[expect]`),
  sauf si l'exception est inscrite, avec sa justification, dans
  `.github/checks/allowed-lint-exceptions.yaml`.
- Un fichier `.rs` fait au maximum 500 lignes et un dossier contient au maximum 8 éléments.
- Toute nouvelle dépendance doit être validée par le Rust Manager.

## Lancer les vérifications

Voir `scripts/README.md`.
