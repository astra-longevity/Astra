# Règles communes

Ces règles valent pour tous les agents qui travaillent sur Astra.

## Noms et langue

- Noms de fichiers et de dossiers en anglais, en `kebab-case` (minuscules, mots séparés par des tirets).
- Si l'ordre compte, préfixe numérique : `01-...`, `02-...` (ou `NNN-` pour les missions, `NNNN-` pour les décisions).
- Le contenu des textes est en français simple, sans jargon inutile. Un terme technique indispensable est expliqué ou ajouté à `docs/glossary.md`.

## Organisation

- Chaque dossier de premier niveau a un `README.md` qui explique son rôle et son responsable.
- Un agent n'écrit que dans le périmètre de son manager, défini dans `OWNERS.md`.

## Rigueur scientifique

- Toute affirmation scientifique cite sa source avec un DOI vérifié (le lien doit mener à la bonne publication).
- La mention `[à vérifier]` est permise en brouillon, mais interdite dans un fichier livré.

## Git

- Messages de commit : `<dossier>: <action>`, par exemple `literature: add aging map overview`.
- Des commits petits, un sujet à la fois.
- Une branche par mission ; fusion dans `main` après relecture par le manager responsable.

## Aucun état hors du dépôt

- Tout ce qui compte est dans le dépôt : pas de serveur, pas de service externe, pas de base de données hébergée.
- La base SQLite (`data/astra.sqlite`) est régénérée à partir des fichiers YAML par l'outil Rust ; elle n'est pas versionnée.
