# 00 — Vue d'ensemble : la carte du vieillissement en langage simple

Ce fichier résume les 13 autres fichiers du dossier pour un lecteur qui n'est pas biologiste. Chaque terme technique est expliqué la première fois qu'il apparaît. Pour les détails et toutes les sources, suivre les liens vers les fichiers spécialisés.

## 1. Qu'est-ce que le vieillissement ?

Le vieillissement est la **perte progressive des capacités du corps avec le temps**. Les tissus réparent moins bien, les organes fonctionnent moins bien, et le risque de tomber malade ou de mourir augmente. Chez l'humain adulte, ce risque double environ tous les 8 ans.

On distingue souvent le vieillissement des maladies : le vieillissement touche tout le monde, alors qu'une maladie ne touche qu'une partie de la population. Mais cette frontière est floue. La plupart des grandes maladies (cancer, maladies du cœur, Alzheimer, diabète de type 2) deviennent beaucoup plus fréquentes avec l'âge, car elles reposent sur les mêmes mécanismes de fond. D'où l'idée centrale de la **géroscience** (l'étude des liens entre vieillissement et maladies) : agir sur le vieillissement lui-même pourrait retarder plusieurs maladies à la fois ([Kennedy 2014](https://doi.org/10.1016/j.cell.2014.10.039)).

Pourquoi vieillit-on ? Les théories évolutives donnent une réponse générale : la sélection naturelle protège mal ce qui se passe après l'âge de la reproduction. Les défauts qui ne se manifestent que tard dans la vie ne sont donc pas éliminés. Détails dans [framework.md](framework.md).

### Comment le mesure-t-on ?

L'âge du calendrier dit mal à quel point un corps a vieilli. Les chercheurs utilisent donc des **biomarqueurs** (des mesures biologiques qui reflètent un état) :

- **Horloges épigénétiques** : elles lisent des marques chimiques posées sur l'ADN (la *méthylation*, de petites étiquettes qui allument ou éteignent les gènes). Ces marques changent de façon régulière avec l'âge. À partir d'une prise de sang, on calcule un « âge biologique ». Exemples : l'horloge de Horvath ([2013](https://doi.org/10.1186/gb-2013-14-10-r115)), GrimAge, qui prédit la mortalité ([Lu 2019](https://doi.org/10.18632/aging.101684)), et DunedinPACE, qui mesure une *vitesse* de vieillissement ([Belsky 2022](https://doi.org/10.7554/eLife.73420)).
- **Indice de fragilité** : on compte les problèmes de santé d'une personne (maladies, difficultés au quotidien). Plus le total est élevé, plus la personne a « vieilli » ([Mitnitski 2001](https://doi.org/10.1100/tsw.2001.58)).
- **Mesures fonctionnelles** : vitesse de marche, force de la main, capacité respiratoire.

Limite importante : aucune de ces mesures n'est encore reconnue comme un critère suffisant pour prouver qu'un traitement ralentit le vieillissement. Un médicament qui fait baisser une horloge n'allonge pas forcément la vie.

## 2. Les 12 « hallmarks » du vieillissement

En 2013, puis en 2023, une équipe menée par Carlos López-Otín a recensé les **12 caractéristiques biologiques** (en anglais *hallmarks*) que l'on retrouve dans tout organisme qui vieillit ([López-Otín 2023](https://doi.org/10.1016/j.cell.2022.11.001)). Pour figurer dans la liste, un mécanisme doit apparaître avec l'âge, et l'aggraver doit accélérer le vieillissement tandis que le corriger doit le ralentir (au moins chez l'animal).

Ces 12 caractéristiques se rangent en trois groupes.

### Groupe 1 — Les dommages de départ (causes primaires)

Ce sont des dégâts qui s'accumulent dans les cellules avec le temps.

- **[01 — Instabilité génomique](01-genomic-instability.md)** : l'ADN, le « mode d'emploi » de chaque cellule, subit chaque jour des milliers de lésions (rayons UV, sous-produits de la respiration, erreurs de copie). Les systèmes de réparation en laissent passer quelques-unes, qui s'accumulent. Résultat : des cellules qui fonctionnent mal ou qui deviennent cancéreuses.
- **[02 — Attrition des télomères](02-telomere-attrition.md)** : les télomères sont les capuchons protecteurs au bout des chromosomes, un peu comme l'embout plastique d'un lacet. Ils raccourcissent à chaque division de la cellule. Trop courts, ils déclenchent l'arrêt définitif de la cellule.
- **[03 — Altérations épigénétiques](03-epigenetic-alterations.md)** : l'épigénétique est l'ensemble des réglages qui décident quels gènes sont actifs, sans changer l'ADN lui-même. Avec l'âge, ces réglages se dérèglent : des gènes s'allument ou s'éteignent au mauvais moment. C'est aussi ce que lisent les horloges épigénétiques.
- **[04 — Perte de protéostasie](04-loss-of-proteostasis.md)** : les protéines sont les petites machines de la cellule. Elles doivent être bien pliées pour fonctionner. Avec l'âge, le contrôle qualité faiblit et des protéines mal pliées s'agglutinent en amas, comme dans Alzheimer ou Parkinson.
- **[05 — Macroautophagie altérée](05-disabled-macroautophagy.md)** : l'autophagie (« se manger soi-même ») est le service de recyclage de la cellule : elle emballe les déchets et les pièces usées pour les détruire. Ce service ralentit avec l'âge et les déchets s'accumulent.

### Groupe 2 — Les réponses qui deviennent nocives (antagonistes)

Ces mécanismes protègent d'abord la cellule face aux dommages, puis deviennent nuisibles quand ils durent trop.

- **[06 — Dérégulation de la détection des nutriments](06-deregulated-nutrient-sensing.md)** : la cellule possède des capteurs qui lui disent s'il y a de la nourriture (insuline, mTOR) ou s'il y a pénurie (AMPK, sirtuines). Abondance = croissance ; pénurie = entretien et réparation. Avec l'âge, ces capteurs restent trop souvent sur « abondance ». C'est le hallmark le mieux étudié : manger moins ou bloquer mTOR allonge la vie de nombreux animaux.
- **[07 — Dysfonction mitochondriale](07-mitochondrial-dysfunction.md)** : les mitochondries sont les centrales énergétiques de la cellule. Avec l'âge, elles produisent moins d'énergie et plus de déchets chimiques nocifs (radicaux libres).
- **[08 — Sénescence cellulaire](08-cellular-senescence.md)** : une cellule sénescente est une cellule abîmée qui a cessé de se diviser mais refuse de mourir. Utile à court terme (elle évite un cancer), elle devient gênante en s'accumulant, car elle émet des signaux inflammatoires qui abîment ses voisines.

### Groupe 3 — Les conséquences à l'échelle du corps (intégratifs)

Quand les dommages des deux premiers groupes dépassent ce que le corps peut compenser, les tissus et les organes en souffrent.

- **[09 — Épuisement des cellules souches](09-stem-cell-exhaustion.md)** : les cellules souches sont les réserves qui renouvellent les tissus (sang, peau, muscle, intestin). Elles deviennent moins nombreuses ou moins efficaces, donc les tissus se réparent moins bien.
- **[10 — Altération de la communication intercellulaire](10-altered-intercellular-communication.md)** : les cellules et les organes se parlent par des hormones et des molécules circulant dans le sang. Avec l'âge, ces messages se brouillent. Des expériences où l'on relie la circulation d'une souris jeune et d'une souris âgée montrent que le sang jeune peut rajeunir certains tissus.
- **[11 — Inflammation chronique](11-chronic-inflammation.md)** : l'inflammation est la réaction de défense du corps (rougeur, gonflement après une blessure). Avec l'âge s'installe une inflammation faible mais permanente, appelée « inflammaging », qui use les tissus à bas bruit.
- **[12 — Dysbiose](12-dysbiosis.md)** : le microbiote intestinal est l'ensemble des milliards de bactéries qui vivent dans notre intestin. Sa composition se déséquilibre avec l'âge (c'est la dysbiose), ce qui favorise l'inflammation.

### Tout est lié

Ces 12 mécanismes ne sont pas indépendants. Chaque fichier contient un tableau de ses liens avec les 11 autres. Exemples : des mitochondries usées (07) doivent être éliminées par l'autophagie (05) ; les cellules sénescentes (08) entretiennent l'inflammation (11) ; le microbiote (12) produit des molécules qui agissent sur l'autophagie. C'est pourquoi une seule intervention peut agir sur plusieurs hallmarks à la fois, et pourquoi il est difficile de savoir lequel est la cause et lequel la conséquence.

## 3. Les interventions les plus solides

« Niveau de preuve » indique jusqu'où la recherche est allée : **préclinique** = chez l'animal ou en laboratoire ; **observationnel** = on a observé des personnes sans leur donner de traitement (risque de confusions) ; **essai randomisé** = on a tiré au sort qui reçoit le traitement et qui reçoit un placebo, la méthode la plus fiable. Les essais de **phase 1** testent la sécurité sur peu de personnes, la **phase 2** cherche un premier effet, la **phase 3** confirme sur un grand nombre.

À ce jour, **aucun traitement n'a démontré chez l'humain qu'il ralentit le vieillissement dans son ensemble.**

| Intervention | Ce qu'elle vise | Niveau de preuve | Ce qu'on sait |
|---|---|---|---|
| Restriction calorique (manger ~10-25 % de moins sans carence) | Capteurs de nutriments (06) | Préclinique solide (y compris singes) ; essai randomisé humain CALERIE (phase 2, 2 ans) | Allonge la vie de nombreux animaux. Chez l'humain : facteurs de risque cardiaques améliorés ([Kraus 2019](https://doi.org/10.1016/S2213-8587(19)30151-2)), vitesse de vieillissement DunedinPACE ralentie de 2-3 % ([Waziry 2023](https://doi.org/10.1038/s43587-022-00357-y)) |
| Rapamycine (médicament qui bloque mTOR) | Capteurs de nutriments (06), autophagie (05) | Préclinique très solide ; essais humains phase 2 | Allonge la vie de la souris, même commencée tard ([Harrison 2009](https://doi.org/10.1038/nature08221)). Chez l'humain, bien tolérée à petite dose, effets modestes ([Moel 2025](https://doi.org/10.18632/aging.206235)) |
| Metformine (médicament du diabète) | Capteurs de nutriments (06) | Données observationnelles ; grand essai TAME conçu mais pas lancé | Les diabétiques sous metformine semblent vivre aussi longtemps que les non-diabétiques, mais ces données peuvent être biaisées ([Barzilai 2016](https://doi.org/10.1016/j.cmet.2016.05.011)). Elle réduit les bénéfices du sport chez l'âgé ([Konopka 2018](https://doi.org/10.1111/acel.12880)) |
| Sénolytiques (dasatinib + quercétine, fisétine) : médicaments qui tuent les cellules sénescentes | Sénescence (08) | Préclinique solide ; essais humains phase 1-2 | Chez la souris âgée, meilleure forme physique et survie allongée ([Zhu 2015](https://doi.org/10.1111/acel.12344)). Chez l'humain, ils éliminent bien des cellules sénescentes ([Hickson 2019](https://doi.org/10.1016/j.ebiom.2019.08.069)) mais sans bénéfice net sur l'os ([Farr 2024](https://doi.org/10.1038/s41591-024-03096-2)). Un candidat (UBX0101) a échoué en phase 2 |
| Précurseurs du NAD+ (NMN, NR) : compléments qui remontent une molécule clé de l'énergie cellulaire | Mitochondries (07), réparation de l'ADN (01) | Préclinique solide ; essais humains phase 1-2 | Chez la souris, atténuent plusieurs déclins liés à l'âge ([Mills 2016](https://doi.org/10.1016/j.cmet.2016.09.013)). Chez l'humain, sûrs et augmentent le NAD+ ([Martens 2018](https://doi.org/10.1038/s41467-018-03421-7)), sans bénéfice clinique démontré |
| Spermidine (molécule présente dans le germe de blé, le fromage affiné) | Autophagie (05) | Préclinique ; observationnel humain ; un essai randomisé | Allonge la vie de la levure, du ver, de la mouche, protège le cœur de la souris ([Eisenberg 2016](https://doi.org/10.1038/nm.4222)) |
| Urolithine A (produite par les bactéries intestinales à partir de la grenade et des noix) | Mitochondries (07), autophagie (05) | Préclinique ; essais humains phase 1-2 | Sûre ; signature mitochondriale améliorée ([Andreux 2019](https://doi.org/10.1038/s42255-019-0073-4)) ; endurance musculaire améliorée, mais objectif principal sur la marche non atteint ([Liu 2022](https://doi.org/10.1001/jamanetworkopen.2021.44279)) |
| Activité physique | Presque tous les hallmarks | Nombreuses études observationnelles et essais randomisés | L'intervention la mieux prouvée sur la santé à l'âge avancé (fichiers 07, 09, 11) |

## 4. Les grandes questions ouvertes

- **Qu'est-ce qui vient en premier ?** On ne sait pas lequel des 12 hallmarks déclenche les autres, ni s'il existe une cause unique.
- **Comment prouver qu'un traitement ralentit le vieillissement chez l'humain ?** Attendre que les participants meurent prendrait des décennies. Il faut des biomarqueurs fiables, et aucun n'est encore validé pour cet usage.
- **Ce qui marche chez la souris marche-t-il chez l'humain ?** La souris vit 2-3 ans ; beaucoup de résultats ne se transposent pas.
- **À quel âge agir, et pendant combien de temps ?** Certains traitements (rapamycine en cure courte, sénolytiques par intermittence) semblent efficaces sans prise continue.
- **Quels effets indésirables ?** Bloquer la croissance ou l'inflammation peut affaiblir les défenses ou la cicatrisation. Le bilan bénéfice/risque chez une personne en bonne santé reste inconnu.
- **Peut-on réparer plutôt que ralentir ?** Des pistes comme la reprogrammation partielle des cellules (les ramener à un état plus jeune sans leur faire perdre leur identité) sont prometteuses chez l'animal mais encore loin de l'humain.

Chaque fichier se termine par sa propre section « Questions ouvertes ».

## 5. Limites de cette cartographie

**Ce qu'elle est.** Une synthèse rédigée par des agents à partir de la littérature scientifique publiée, en octobre 2026. Ce n'est pas un avis médical. Aucune des interventions citées ne doit être prise sans avis médical.

**Contrôle qualité effectué (19/10/2026).**
- Cohérence : les noms des 12 hallmarks ont été harmonisés entre les fichiers (variantes corrigées pour 04, 05 et 10). Les niveaux de preuve des interventions citées dans plusieurs fichiers (rapamycine, metformine, sénolytiques, NAD+, urolithine A, restriction calorique) sont cohérents entre eux.
- Sources : 39 DOI (identifiants uniques d'articles scientifiques) tirés au hasard, 3 par fichier, ont été vérifiés auprès de la base Crossref (auteur, année, titre). Les 39 correspondent à la citation.

**Points non vérifiés, marqués [à vérifier] dans les fichiers :**
- [framework.md](framework.md) : Strehler 1962 (*Time, Cells and Aging*) et Medawar 1952 (*An Unsolved Problem of Biology*) : deux ouvrages classiques sans DOI, non vérifiables en ligne.
- [02](02-telomere-attrition.md) : changement de méthylation des régions proches des télomères chez l'humain ; lien entre télomères et dysbiose.
- [03](03-epigenetic-alterations.md) : lien entre métabolites du microbiote et vieillissement épigénétique ; absence d'effet des précurseurs du NAD+ sur les horloges chez l'humain.
- [06](06-deregulated-nutrient-sensing.md) : statut exact de l'essai TAME (aucune source primaire récente).

**Limites de fond.**
- Le cadre des 12 hallmarks est un outil d'organisation, pas une vérité établie ; d'autres classements existent (voir [framework.md](framework.md)).
- La plupart des résultats viennent d'animaux de laboratoire.
- Le domaine évolue vite : des essais en cours peuvent changer les conclusions du tableau ci-dessus.

## Sources clés

- López-Otín C, et al. Hallmarks of aging: an expanding universe. *Cell* 2023. [doi:10.1016/j.cell.2022.11.001](https://doi.org/10.1016/j.cell.2022.11.001)
- Kennedy BK, et al. Geroscience: linking aging to chronic disease. *Cell* 2014. [doi:10.1016/j.cell.2014.10.039](https://doi.org/10.1016/j.cell.2014.10.039)
- Harrison DE, et al. Rapamycin fed late in life extends lifespan in genetically heterogeneous mice. *Nature* 2009. [doi:10.1038/nature08221](https://doi.org/10.1038/nature08221)
- Barzilai N, et al. Metformin as a tool to target aging. *Cell Metab* 2016. [doi:10.1016/j.cmet.2016.05.011](https://doi.org/10.1016/j.cmet.2016.05.011)
- Waziry R, et al. Effect of long-term caloric restriction on DNA methylation measures of biological aging. *Nat Aging* 2023. [doi:10.1038/s43587-022-00357-y](https://doi.org/10.1038/s43587-022-00357-y)

Les autres références sont dans chaque fichier, section 7.
