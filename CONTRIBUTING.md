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

## Le dépôt est l'unique source de vérité

- Tout ce qui compte vit dans le dépôt, sous forme de texte versionné.
- Les outils annexes (base SQLite, graphe, index, site…) sont permis seulement si leurs données sont stockées dans le dépôt et si l'outil peut tout reconstruire à partir d'elles.
- Aucun outil ne garde d'état propre. Les fichiers qu'ils produisent sont ignorés par Git.
- Pas de serveur, pas de service externe.
- Exemple : `data/astra.sqlite` est reconstruite à partir des fichiers YAML par `astra db build` ; elle n'est pas versionnée.

Voir la décision `docs/decisions/0002-repo-single-source-of-truth.md`.
