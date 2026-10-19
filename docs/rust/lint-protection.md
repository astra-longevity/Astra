# Protection du code Rust

Ce document explique, sans jargon inutile, ce qui empêche du code Rust de mauvaise qualité d'entrer dans le dépôt Astra, et comment s'en servir.

Idée générale : le code Rust d'Astra (les petits programmes qui construisent la base de données et le site) doit respecter des règles très strictes. Ces règles sont vérifiées **automatiquement**, à trois moments :

1. pendant qu'un agent écrit du code (vérification locale immédiate) ;
2. à chaque envoi sur GitHub (intégration continue, « CI ») ;
3. avant toute fusion dans la branche principale `main` (protection de branche, préparée mais pas encore active).

Et surtout : **les règles elles-mêmes sont scellées**. Personne ne peut les assouplir discrètement, car toute modification est détectée tant qu'elle n'a pas été validée avec un mot de passe que seul le président détient.

---

## 1. Ce qui protège le code

### 1.1 Version de Rust figée : 1.97.1

- **Fichier** : `rust-toolchain.toml`
- **Ce que c'est** : Rust est le langage de programmation utilisé. Ce fichier impose une version précise (1.97.1) à tout le monde : agents, machine locale, CI.
- **Pourquoi** : la liste des règles (voir 1.3) dépend de la version. Une autre version connaîtrait des règles différentes, et la vérification échouerait.
- Le fichier installe aussi deux outils : `rustfmt` (mise en forme) et `clippy` (contrôle qualité).

### 1.2 Mise en forme automatique (rustfmt)

- **Fichier** : `rustfmt.toml`
- **Ce que c'est** : un outil qui impose une présentation unique du code (largeur de ligne 120 caractères, édition 2024 du langage, écritures abrégées).
- **Effet** : si le code n'est pas présenté exactement comme l'outil le ferait, la vérification échoue. Il n'y a donc jamais de débat de style.

### 1.3 Environ 1 000 règles interdites (« lints »)

- **Fichier** : `Cargo.toml` (racine), sections `[workspace.lints.rust]` et `[workspace.lints.clippy]`
- **Ce que c'est** : un « lint » est une règle qui signale une pratique risquée ou maladroite dans le code. Ici, la quasi-totalité des règles connues de Rust et de clippy sont listées une par une, et presque toutes sont au niveau `forbid` (interdit, sans dérogation locale possible). Quelques-unes sont au niveau `deny` (interdit, mais une exception enregistrée reste techniquement possible).
- **Exemples de ce qui est interdit** : laisser une erreur non traitée, faire planter le programme volontairement, code non documenté, code mort, variables inutilisées, calculs pouvant déborder.
- **Pour chaque crate** (sous-projet Rust dans `crates/`) : son `Cargo.toml` contient `[lints] workspace = true`, ce qui l'oblige à hériter de ces règles. Aucune crate ne peut les redéfinir.

### 1.4 Seuils de complexité (clippy.toml)

- **Fichier** : `clippy.toml`
- **Ce que c'est** : des limites chiffrées, plus sévères que les valeurs par défaut. Principales :
  - une fonction ne doit pas dépasser **60 lignes** ;
  - une fonction prend au plus **4 paramètres** ;
  - « complexité cognitive » (difficulté à lire une fonction) plafonnée à **7** ;
  - les noms de variables font au moins **3 caractères** (sauf une courte liste d'exceptions comme `i`, `id`) ;
  - noms vagues interdits (`foo`, `tmp`, `thing`…) ;
  - certaines fonctions dangereuses du langage sont bannies (par exemple arrêter brutalement le programme).
- Dans les **tests** uniquement, il est permis de faire échouer volontairement le programme : c'est ainsi qu'un test signale un problème.

### 1.5 Registre des exceptions : vide

- **Fichier** : `.github/checks/allowed-lint-exceptions.yaml` (contient `exceptions: []`)
- **Vérification** : `.github/checks/check-lint-exceptions.sh`
- **Ce que c'est** : un développeur peut, en théorie, écrire dans le code une annotation qui désactive une règle à un endroit précis. Ici, **toute** annotation de ce type doit être déclarée dans ce registre, avec une justification et un plan pour la supprimer. Sinon la vérification échoue.
- Le registre est **vide** : aujourd'hui, Astra ne contient aucune dérogation.
- Le script vérifie trois choses : pas d'annotation non déclarée, pas d'entrée orpheline (déclarée mais absente du code), pas d'entrée ambiguë (qui correspondrait à plusieurs endroits).

### 1.6 Taille des fichiers et des dossiers

- **Vérification** : `.github/checks/check-structure.sh`
- **Règles** :
  - chaque fichier Rust (`.rs`) fait **500 lignes au plus** ;
  - chaque dossier contient **8 éléments au plus** (certains dossiers sont exemptés : `docs/`, `crates/`, `.github/checks`, `.github/workflows`, dossiers de compilation…).
- **Pourquoi** : forcer un découpage en petits morceaux lisibles, plutôt que de gros fichiers fourre-tout.
- Ce même script vérifie aussi le sceau (voir 1.7).

### 1.7 Le sceau : une chaîne d'empreintes protégée par mot de passe

- **Fichiers** :
  - `.github/checks/protected-files.yaml` : la liste de ce qui est scellé ;
  - `.github/checks/lint-config.chain` : le registre des sceaux successifs ;
  - `.github/checks/check-lint-config.sh` : le script qui calcule et vérifie ;
  - `chain.sh` (racine) : raccourci pour mettre à jour le sceau.
- **Le principe, en image** : on calcule une « empreinte » (SHA-256) de tous les fichiers de règles. Une empreinte est comme une signature unique d'un contenu : changer une seule virgule change complètement l'empreinte. Chaque nouvelle empreinte est enregistrée dans le registre en la liant à la précédente **et au mot de passe**, formant une chaîne.
- **Conséquence** :
  - vérifier le sceau ne demande **aucun** mot de passe (la CI le fait à chaque envoi) ;
  - si quelqu'un modifie une règle sans renouveler le sceau, la vérification échoue ;
  - renouveler le sceau exige le mot de passe ; sans lui, impossible de fabriquer une entrée valide.
- **Ce qui est scellé** :
  - la liste `protected-files.yaml` elle-même ;
  - les deux sections de règles du `Cargo.toml` racine (pas le reste du fichier, pour pouvoir ajouter des dépendances) ;
  - `rust-toolchain.toml`, `clippy.toml`, `rustfmt.toml` ;
  - tous les scripts de `.github/checks/` (`check-*.sh`, `lib-failures.sh`) et le registre des exceptions ;
  - le fichier de CI `.github/workflows/ci.yml` ;
  - les vérifications locales `.context-pilot/shared/callbacks.yaml` ;
  - `chain.sh`.
- Le registre contient aujourd'hui **une entrée** (l'entrée d'origine, dite « genesis »), posée au commit `c78eeb6`.
- **Le mot de passe n'est jamais écrit dans le dépôt.**

### 1.8 Intégration continue (CI) sur GitHub

- **Fichier** : `.github/workflows/ci.yml`
- **Quand** : à chaque envoi (`push`) sur `main` et à chaque demande de fusion (« pull request ») vers `main`.
- **Deux tâches** :
  - **`Rust (lints · tests)`** : installe Rust 1.97.1, puis lance
    - `.github/checks/check-rust-lints.sh --ci` : mise en forme, clippy, compilation sans aucun avertissement, registre des exceptions, contrôle des secrets ;
    - `.github/checks/check-rust-tests.sh --ci` : compilation optimisée puis exécution de tous les tests.
  - **`Structure (lengths · folders · chain)`** : lance `.github/checks/check-structure.sh --ci` (tailles de fichiers et de dossiers, sceau).
- Chaque script exécute **toutes** ses sous-vérifications, puis affiche à la fin la liste de celles qui ont échoué (mécanisme commun dans `.github/checks/lib-failures.sh`).
- Note : `check-vault-bypass.sh` vient de context-pilot ; il interdit de lire directement des clés secrètes (variables `*_API_KEY`, `*_TOKEN`…) dans le code. Il est conservé tel quel.

### 1.9 Vérifications locales bloquantes

- **Fichier** : `.context-pilot/shared/callbacks.yaml`
- **Ce que c'est** : dans l'outil où travaillent les agents (context-pilot), chaque modification d'un fichier `.rs` déclenche immédiatement, et de façon **bloquante**, les mêmes scripts que la CI :
  - `rust-lints` → `check-rust-lints.sh --callback` (limite 120 s) ;
  - `structure-rust` → `check-structure.sh --callback` (limite 30 s).
- **Pourquoi** : un agent voit ses erreurs tout de suite, avant même d'envoyer quoi que ce soit. CI et vérification locale utilisent **le même script** : elles ne peuvent pas diverger.
- Il n'y a pas de « git hooks » (scripts lancés par git avant un commit) : comme dans context-pilot, ce rôle est tenu par ces vérifications locales.

### 1.10 Propriétaires du code (CODEOWNERS)

- **Fichier** : `.github/CODEOWNERS` (contient `* @bigmoostache`)
- **Ce que c'est** : désigne qui est responsable de chaque partie du dépôt. Ici, tout le dépôt appartient à `@bigmoostache`. Ce fichier propose automatiquement ce relecteur sur chaque demande de fusion, mais la relecture n'est **pas obligatoire** (voir section 4).

---

## 2. Lancer les vérifications soi-même

Toutes les commandes se lancent depuis la racine du dépôt.

| But | Commande |
|---|---|
| Règles de code (mise en forme, clippy, compilation, exceptions, secrets) | `.github/checks/check-rust-lints.sh --ci` |
| Compilation et tests | `.github/checks/check-rust-tests.sh --ci` |
| Taille des fichiers/dossiers + sceau | `.github/checks/check-structure.sh --ci` |
| Sceau seul (vérification) | `.github/checks/check-lint-config.sh` |
| Registre des exceptions seul | `.github/checks/check-lint-exceptions.sh` |

Variante plus rapide en local : remplacer `--ci` par `--callback`. La seule différence concerne les tests : `--callback` compile en mode « debug » (plus rapide) au lieu du mode optimisé ; la couverture des tests est identique.

Commandes de base si on veut isoler une étape :

```bash
cargo fmt -- --check
cargo clippy --workspace --all-targets -- -D warnings
cargo test --workspace
```

Résultat attendu : chaque script se termine par une ligne `… OK` et un code de sortie 0. En cas d'échec, la dernière ligne liste les sous-vérifications fautives ; le détail se trouve plus haut, sous les titres `=== … ===`.

---

## 3. Le sceau au quotidien

### 3.1 Consulter

```bash
.github/checks/check-lint-config.sh --show          # liste ce qui est scellé + empreinte actuelle
.github/checks/check-lint-config.sh --history       # historique des sceaux
.github/checks/check-lint-config.sh --deep-verify   # vérification approfondie de tout l'historique
```

### 3.2 Modifier une règle plus tard

Procédure, dans l'ordre :

1. Modifier le ou les fichiers concernés (par exemple `clippy.toml`, ou une section de règles du `Cargo.toml`).
2. Lancer les vérifications (section 2) : le sceau doit **échouer** à ce stade, c'est normal ; tout le reste doit passer.
3. Le détenteur du mot de passe renouvelle le sceau :
   ```bash
   .github/checks/check-lint-config.sh --update
   ```
   Le mot de passe est demandé de façon masquée. Alternative équivalente : `./chain.sh`.
   Le script refuse un mauvais mot de passe. Il ajoute une nouvelle ligne au registre `lint-config.chain`, suivie du détail des modifications (pour garder une trace lisible).
4. Relancer `.github/checks/check-lint-config.sh` : il doit afficher « verified ».
5. Faire un **seul** commit contenant à la fois la modification et le registre mis à jour (préfixe `rust:`).

Règles de sécurité :

- le mot de passe ne doit **jamais** apparaître dans le dépôt, dans un message, ni dans un journal ;
- éviter de le passer en clair sur la ligne de commande (`./chain.sh --password …` reste possible mais laisse une trace dans l'historique du terminal) ; préférer la saisie masquée ;
- la variable d'environnement `LINT_GUARD_PASSWORD` est aussi acceptée, à n'utiliser que de façon ponctuelle.

### 3.3 Ajouter un fichier sous scellé

Ajouter une entrée dans `.github/checks/protected-files.yaml` (format décrit en tête du fichier), puis renouveler le sceau comme en 3.2. Comme la liste est elle-même scellée, on ne peut pas non plus **retirer** discrètement un fichier de la protection.

---

## 4. Protection de la branche `main` (préparée, **pas encore active**)

### 4.1 Ce qu'elle apportera

Une fois activée sur GitHub, il deviendra impossible de modifier `main` sans respecter ces conditions :

- **vérifications obligatoires** : les deux tâches de CI `Rust (lints · tests)` et `Structure (lengths · folders · chain)` doivent être vertes ;
- **branche à jour** (« strict ») : la modification doit avoir été testée sur la toute dernière version de `main` ;
- **pas de réécriture d'historique** (« force-push » interdit) ;
- **pas de suppression** de la branche ;
- **pas de relecture obligatoire** : le dépôt n'a qu'un propriétaire, et les agents envoient directement sur `main`. Or GitHub interdit d'approuver sa propre modification ; exiger une relecture bloquerait donc tout ;
- les administrateurs ne sont pas soumis à ces règles (`enforce_admins: false`), pour garder une porte de sortie en cas de blocage.

### 4.2 État actuel : pas active

- La tentative d'activation a été **refusée par GitHub (erreur 403)** : sur l'offre gratuite, la protection de branche n'est pas disponible pour un dépôt **privé**. Il faut soit passer à GitHub Pro, soit rendre le dépôt public.
- **Décision en attente du président.**
- Le sceau, lui, est déjà en place (commit `c78eeb6`).

### 4.3 Commande d'activation

À lancer **uniquement sur décision explicite** (une copie de la configuration se trouve dans `/tmp/astra-protect.json` ; on peut aussi l'écrire dans un fichier à partir du bloc ci-dessous) :

```bash
gh api -X PUT repos/bigmoostache/Astra/branches/main/protection --input <fichier>
```

Contenu du fichier :

```json
{
  "required_status_checks": {
    "strict": true,
    "contexts": [
      "Rust (lints · tests)",
      "Structure (lengths · folders · chain)"
    ]
  },
  "enforce_admins": false,
  "required_pull_request_reviews": null,
  "restrictions": null,
  "allow_force_pushes": false,
  "allow_deletions": false
}
```

Vérifier ensuite :

```bash
gh api repos/bigmoostache/Astra/branches/main/protection
```

Point d'attention : les noms des vérifications doivent correspondre **exactement** aux champs `name:` de `ci.yml` (avec le point médian `·`), et elles doivent avoir tourné au moins une fois sur GitHub pour être reconnues.

---

## 5. Origine

Tout ce dispositif est **recopié** du projet `github.com/bigmoostache/context-pilot` (branche `master`) : mêmes règles, mêmes seuils, mêmes scripts, même mécanisme de sceau, même organisation de la CI. Les adaptations pour Astra :

- seules les vérifications Rust ont été reprises (pas celles du site web en TypeScript de context-pilot) ;
- branche `main` au lieu de `master` ;
- la tâche CI de structure s'appelle `Structure (lengths · folders · chain)` (sans la partie « web ») ;
- `rust-toolchain.toml` et `chain.sh` ont été ajoutés à la liste scellée ;
- le registre du sceau repart de zéro, avec le mot de passe propre à Astra.

**Réserve importante sur la version 1.97.1** : la liste des ~1 000 règles est écrite pour cette version précise de Rust. Changer de version peut faire apparaître des règles inconnues ou renommées ; elles produisent alors un avertissement, et comme tout avertissement est traité comme une erreur, la CI échoue. Monter de version demande donc de mettre à jour la liste des règles en même temps, puis de renouveler le sceau (section 3.2).
