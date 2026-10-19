# Données

Responsable : A9 (Database).

Un fichier YAML par objet (étude, molécule, gène…). Ces fichiers sont la source de vérité. Le détail du format est défini par A9.

La base SQLite `astra.sqlite` est générée à partir de ces fichiers par l'outil Rust ; elle n'est pas versionnée.
