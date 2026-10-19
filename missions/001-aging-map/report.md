# Mission 001 — Rapport : la carte du vieillissement

## Ce qui a été produit

Une carte de ce que la science sait aujourd'hui du vieillissement, en 14 fichiers rangés dans [`literature/aging-map/`](../../literature/aging-map/) :

- [`00-overview.md`](../../literature/aging-map/00-overview.md) : la vue d'ensemble, en français simple. **C'est le fichier à lire en premier.**
- [`framework.md`](../../literature/aging-map/framework.md) : les définitions, les grandes théories (pourquoi vieillit-on ?) et les façons de mesurer le vieillissement (« horloges » biologiques, indice de fragilité…).
- `01` à `12` : un fichier par grande cause biologique du vieillissement. Les chercheurs en reconnaissent 12, appelées *hallmarks* (« caractéristiques ») depuis un article de référence de 2013, mis à jour en 2023.

## Comment c'est organisé

Les 12 causes sont rangées en trois groupes :

1. **Les dommages de départ** (`01` à `05`) : l'ADN s'abîme, les bouts des chromosomes raccourcissent, les réglages des gènes se dérèglent, les protéines se déforment, le recyclage des déchets de la cellule ralentit.
2. **Les réponses qui deviennent nocives** (`06` à `08`) : des mécanismes de défense utiles au départ (capteurs de nutriments, centrales énergétiques de la cellule, mise à l'arrêt des cellules abîmées) deviennent nuisibles quand ils durent trop.
3. **Les conséquences visibles** (`09` à `12`) : les réserves de cellules de remplacement s'épuisent, les cellules communiquent mal, une inflammation de fond s'installe, les bactéries de l'intestin se déséquilibrent.

Chaque fichier `01` à `12` suit le même plan en 7 parties : définition, mécanismes, effets visibles sur la santé, liens avec les 11 autres causes, traitements existants ou à l'étude (avec leur niveau de preuve), questions ouvertes, références.

## Comment les sources ont été contrôlées

- Chaque affirmation s'appuie sur une publication identifiée par son DOI (un numéro unique attribué à chaque article scientifique).
- Chaque rédacteur a vérifié toutes ses références dans Crossref, la base officielle des DOI (auteur, année, titre). Les références fausses ont été retirées.
- Contrôle final indépendant : 39 DOI tirés au hasard (3 par fichier) ont été revérifiés. Les 39 sont corrects.
- Les sources vérifiées sont aussi enregistrées dans la base de données du projet (`data/`), par un enregistrement séparé (`data: ingest aging-map`).

## Limites

- **7 points restent marqués [à vérifier]** dans les fichiers : deux livres anciens sans DOI (`framework.md`), deux points dans `02`, deux dans `03`, et le statut actuel de l'essai TAME sur la metformine (`06`). Ils sont listés dans la section 5 de `00-overview.md`.
- La plupart des résultats viennent d'animaux de laboratoire ; peu sont démontrés chez l'humain.
- Les 12 causes sont un outil pour ranger les connaissances, pas une vérité établie.
- Le domaine évolue vite. Ce n'est pas un avis médical.
