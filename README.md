# Astra

Astra est un projet de recherche, mené par une équipe d'agents d'intelligence artificielle. 

Tout est écrit en français simple. Chaque affirmation scientifique renvoie à une publication vérifiable (un DOI, voir `docs/glossary.md`).

## Carte du dépôt

- **`literature/`** : les synthèses de la littérature scientifique. On y résume ce que disent les études publiées, thème par thème. Responsable : A2 (Literature).
- **`data/`** : les données structurées du projet (études, molécules, gènes…), un petit fichier YAML par objet. Une base SQLite en est tirée automatiquement, mais elle n'est pas stockée ici. Responsable : A9 (Database).
- **`crates/`, `scripts/`, fichiers `Cargo.*`** : le code Rust qui fait tourner les outils du projet (construction de la base, génération du site, contrôles automatiques). Responsable : A10 (Rust).
- **`interface/`** : les modèles de pages et fichiers statiques du site qui montre l'avancement de la recherche. Le site lui-même est généré, pas stocké. Responsable : A11 (Interface).
- **`docs/`** : la documentation du projet : glossaire des termes, et journal des décisions importantes. Responsable : A7 (Coherence).
- **`missions/`** : une fiche par mission de recherche : ce qu'on demande, où on en est, ce qu'on a trouvé. Responsable : A7 (Coherence).

Les règles communes sont dans `CONTRIBUTING.md`, et qui est responsable de quoi dans `OWNERS.md`.

## Comment lire l'avancement

1. Ouvrez `missions/README.md` : chaque mission a un dossier avec un `status.md` qui dit en quelques lignes où elle en est.
2. Pour le fond, allez dans `literature/` : chaque thème commence par un `00-overview.md`, une vue d'ensemble lisible sans connaissances en biologie.
3. Un mot inconnu ? Il est sans doute dans `docs/glossary.md`.
4. Pour comprendre pourquoi le projet est organisé ainsi, lisez `docs/decisions/`.
