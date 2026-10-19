# scripts/ — où sont les vérifications ?

Les scripts de vérification ne sont pas ici : ils sont dans `.github/checks/`,
au même endroit que dans le projet context-pilot dont Astra reprend les règles.
Ce dossier ne fait que les documenter.

## Les scripts

| Fichier | Ce qu'il vérifie |
|---------|------------------|
| `check-rust-lints.sh` | Mise en forme (`cargo fmt`), analyse `clippy` sans aucun avertissement, compilation sans avertissement, registre des exceptions, accès aux secrets. |
| `check-rust-tests.sh` | Compilation complète puis exécution de tous les tests. |
| `check-structure.sh` | Fichiers `.rs` ≤ 500 lignes, dossiers ≤ 8 éléments, puis intégrité de la chaîne de protection. |
| `check-lint-exceptions.sh` | Chaque `#[allow]` / `#[expect]` du code correspond à exactement une entrée de `allowed-lint-exceptions.yaml`. |
| `check-vault-bypass.sh` | Aucune clé secrète (`*_API_KEY`, `*_TOKEN`…) n'est lue directement dans l'environnement. |
| `check-lint-config.sh` | Chaîne de hachage : détecte toute modification des fichiers de règles listés dans `protected-files.yaml`. |
| `lib-failures.sh` | Outil commun : liste à la fin toutes les vérifications qui ont échoué. |

## Les lancer

Depuis la racine du dépôt :

```sh
.github/checks/check-rust-lints.sh --ci
.github/checks/check-rust-tests.sh --ci
.github/checks/check-structure.sh --ci
```

Les mêmes scripts tournent automatiquement :
- sur GitHub à chaque push ou pull request vers `main` (`.github/workflows/ci.yml`) ;
- localement à chaque modification d'un fichier `.rs`, via les callbacks
  bloquants de context-pilot (`.context-pilot/shared/callbacks.yaml`).

## Modifier les règles

Les fichiers de règles sont scellés par une chaîne de hachage protégée par mot de passe.
Après une modification légitime, le détenteur du mot de passe lance `./chain.sh`.
Sans ça, la vérification `check-structure.sh` échoue.
