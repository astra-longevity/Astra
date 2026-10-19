# 0002 — Le dépôt est l'unique source de vérité

Date : 2026-10-19. Statut : acceptée. Décidée par le président, transmise par A1.

## Contexte

Le projet va utiliser des outils annexes : une base SQLite, peut-être un graphe de connaissances, un index de recherche, un site d'avancement. Si l'un d'eux gardait des informations qui n'existent nulle part ailleurs, le dépôt ne suffirait plus à comprendre ni à reconstruire le projet, et ces informations pourraient se perdre ou diverger.

## Décision

- Tout ce qui compte vit dans le dépôt Git, sous forme de texte versionné.
- Un outil annexe est permis seulement si ses données sont stockées dans le dépôt et s'il peut tout reconstruire à partir d'elles.
- Aucun outil ne garde d'état propre. Les fichiers qu'il produit sont ignorés par Git.
- Pas de serveur ni de service externe.
- Exemple : `data/astra.sqlite` est reconstruite à partir des fichiers YAML de `data/` par `astra db build`.

Cette décision précise la décision 0001 sans la remplacer.

## Conséquences

- Supprimer un fichier généré ne fait rien perdre : il suffit de relancer l'outil.
- Toute information doit d'abord être écrite dans le dépôt avant d'être utilisée par un outil.
- Un outil qui ne respecte pas ces conditions n'est pas accepté dans le projet.
