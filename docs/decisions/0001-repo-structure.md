# 0001 — Structure du dépôt

Date : 2026-10-19. Statut : acceptée.

## Contexte

Plusieurs équipes d'agents travaillent en parallèle sur Astra (littérature, données, code, interface). Il faut une organisation claire pour qu'elles ne se marchent pas dessus, et tout doit tenir dans le dépôt Git, sans serveur ni service externe.

## Décision

- Dossiers de premier niveau : `literature/` (A2), `data/` (A9), `interface/` (A11), `docs/` et `missions/` (A7) ; le code Rust (`crates/`, `scripts/`, fichiers `Cargo.*` à la racine) relève de A10. Détail dans `OWNERS.md`.
- Données : un fichier YAML par objet dans `data/`. Ces fichiers sont la seule source de vérité.
- La base SQLite (`data/astra.sqlite`) est générée à partir des YAML par l'outil Rust ; elle est ignorée par Git.
- Le code Rust est un « workspace » à la racine du dépôt, avec un crate par outil dans `crates/`.
- Le site d'interface est généré par un crate Rust à partir du dépôt ; `interface/` ne contient que les modèles et fichiers statiques, le site produit n'est pas versionné.

## Conséquences

- N'importe qui peut tout reconstruire à partir du dépôt seul.
- Les fichiers versionnés restent lisibles par un humain et faciles à relire dans Git.
- Toute modification de cette structure passe par une nouvelle décision.
