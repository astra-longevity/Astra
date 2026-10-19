# data/ — la base de connaissances d'Astra

Ce dossier contient **tout ce que l'équipe a appris**, rangé sous forme de petites
fiches texte (format YAML, lisible par un humain). Une fiche = un objet.

Une base de données (`data/astra.sqlite`) est fabriquée à partir de ces fiches
par la commande `astra db build`. Elle sert à l'interface pour faire des
recherches rapides. Elle n'est **jamais** enregistrée dans git : on peut la
supprimer et la reconstruire à tout moment. **La vérité, ce sont les fiches.**

> **Règle d'or.** Le dépôt est l'UNIQUE source de vérité : les YAML font foi ;
> astra.sqlite et tout futur outil de données (graphe, index…) doivent stocker
> leurs données dans le dépôt et se reconstruire entièrement à partir d'elles,
> sans état propre.

## Les 8 types de fiches

| Dossier | Ce que c'est | Exemple |
|---|---|---|
| `missions/` | Un chantier de recherche de l'équipe. | « Carte du vieillissement » |
| `sources/` | Une publication scientifique (article, livre), identifiée par son DOI (le « numéro de sécurité sociale » d'un article). | Trifunovic 2004, *Nature* |
| `concepts/` | Une notion : cause du vieillissement, gène, maladie, traitement, organisme… | « Dysfonction mitochondriale » |
| `claims/` | Une **affirmation** scientifique, en une phrase. | « Des souris dont l'ADN des mitochondries se dégrade vieillissent plus vite. » |
| `evidence/` | Le lien entre une affirmation et une source : la source la **soutient**, la **contredit** ou la **nuance**. | C-0001 ← Trifunovic 2004 |
| `relations/` | Un lien entre deux concepts (A cause B, A aggrave B, A traite B…), justifié par des affirmations. | instabilité génomique → aggrave → mitochondries |
| `questions/` | Une question encore sans réponse (ou résolue). | « Est-ce vrai aussi chez l'humain ? » |
| `contradictions/` | Deux affirmations (ou plus) qui semblent se contredire, avec une explication. | |

Le dossier `schema/` contient les règles (ce qu'une fiche doit contenir) et
`examples/` contient une fiche d'exemple de chaque type.

## L'échelle de preuve (6 niveaux)

Chaque affirmation indique **à quel point elle est prouvée**, de 1 (faible) à 6 (fort) :

1. **Cellules ou ordinateur** — observé dans une boîte de culture ou par simulation.
2. **Petits organismes** — levure, ver, mouche : un être vivant complet, mais très loin de nous.
3. **Mammifères de laboratoire** — souris, rat, singe. Plus proche, mais beaucoup de résultats chez la souris échouent chez l'humain.
4. **Observations chez l'humain** — on constate un lien dans une population, sans prouver la cause.
5. **Essai clinique randomisé** — on tire au sort qui reçoit le traitement : seule façon de prouver une cause chez l'humain.
6. **Approuvé** — validé par une autorité de santé (FDA, EMA).

On indique aussi une **confiance** : `solide` (exige au moins une source vérifiée),
`débattu` ou `spéculatif`. Une source est `vérifié` quand on a contrôlé qu'elle
existe vraiment (via Crossref) ; sinon elle est `à vérifier` et l'interface l'affiche.

## Ajouter une fiche

1. Copier la fiche du même type dans `examples/` vers le bon dossier.
2. Remplir les champs. Écrire en français, phrases courtes.
3. Nommer le fichier selon les conventions ci-dessous.
4. Lancer `astra db build --check` : il liste toute erreur, en français.

Une affirmation **doit** avoir au moins un fichier `evidence/`, sinon le build refuse.

## Noms et identifiants

| Type | Identifiant | Nom du fichier |
|---|---|---|
| Source | DOI en minuscules : `10.1038/nature02517` (livre sans DOI : `isbn:978…`) | `/` remplacé par `_` : `10.1038_nature02517.yaml` |
| Concept, mission | minuscules-avec-tirets, en anglais : `mitochondrial-dysfunction` | `<id>.yaml` |
| Affirmation | `C-0001-court-resume` | `<id>.yaml` |
| Relation | `R-0001-…` | `<id>.yaml` |
| Question | `Q-0001-…` | `<id>.yaml` |
| Contradiction | `X-0001-…` | `<id>.yaml` |
| Evidence | (pas d'id) | `<id de l'affirmation>--<nom de fichier de la source>.yaml` |

Les dates s'écrivent entre guillemets : `verified_at: "2026-10-19"`.

## Éviter les conflits quand plusieurs agents travaillent

- Une fiche par fichier : deux agents qui ajoutent des fiches différentes ne se gênent jamais.
- Numéros `C-0001`, `R-0001`… : avant de choisir un numéro, prendre le plus
  grand existant + 1 **sur la branche à jour**. Le texte après le numéro rend
  l'id unique même si deux agents prennent le même numéro ; le build signale
  tout vrai doublon.
- Ne pas modifier la fiche d'un autre chantier sans prévenir son responsable.
- Ne jamais enregistrer `astra.sqlite` dans git (déjà exclu par `.gitignore`).

Détails techniques pour les développeurs : `schema/BUILD_SPEC.md`.
