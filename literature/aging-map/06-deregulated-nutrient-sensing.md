# 06 — Dérégulation de la détection des nutriments (*deregulated nutrient sensing*)

> Hallmark « antagoniste » (López-Otín et al., *Cell* 2023, doi:10.1016/j.cell.2022.11.001). En clair : les capteurs qui disent à la cellule « il y a à manger, grandis » ou « c'est la disette, économise et répare » restent bloqués trop longtemps en mode « croissance ».

## 1. Définition

Les cellules mesurent en permanence l'énergie et les nutriments disponibles grâce à quatre grands réseaux : **IIS** (insuline / IGF-1), **mTOR** (acides aminés, croissance), **AMPK** (manque d'énergie) et **sirtuines** (niveau de NAD+). Avec l'âge, l'équilibre penche vers les voies anaboliques (IIS, mTOR élevés) et s'éloigne des voies de maintenance (AMPK, sirtuines). Freiner la croissance et activer la maintenance allonge la vie dans toutes les espèces modèles testées.

## 2. Mécanismes et sources

**Axe GH / insuline / IGF-1 (IIS).** C'est la première voie de longévité découverte : un seul gène muté (*daf-2*, récepteur insuline/IGF) double la durée de vie du ver *C. elegans* (Kenyon 1993, doi:10.1038/366461a0). L'effet passe par l'activation du facteur de transcription FOXO (DAF-16 chez le ver), qui met en marche les gènes de résistance au stress. Chez l'humain :
- des variants du récepteur IGF-1 qui réduisent son activité sont plus fréquents chez les centenaires ashkénazes (Suh 2008, doi:10.1073/pnas.0705467105) ;
- le génotype **FOXO3A** est l'une des associations génétiques à la longévité les plus répliquées (Willcox 2008, doi:10.1073/pnas.0801030105) ;
- les personnes atteintes du syndrome de Laron (récepteur de l'hormone de croissance déficient, IGF-1 très bas) développent très peu de cancers et de diabète (Guevara-Aguirre 2011, doi:10.1126/scitranslmed.3001845).

Paradoxe : au cours du vieillissement normal, GH et IGF-1 *baissent*. Cela ressemble à une réponse de défense de l'organisme, mais qui devient délétère quand elle est poussée trop loin (fragilité, sarcopénie).

**mTOR.** mTOR est une kinase qui fonctionne en deux complexes. **mTORC1** capte les acides aminés (surtout la leucine), l'insuline et l'énergie ; il stimule la synthèse des protéines et des lipides et freine l'autophagie (Saxton & Sabatini 2017, doi:10.1016/j.cell.2017.02.004). **mTORC2** sert surtout la signalisation insulinique et la survie. Si mTORC1 reste activé en permanence chez l'âgé, l'autophagie est bloquée, les protéines endommagées s'accumulent et la sécrétion inflammatoire des cellules sénescentes est amplifiée. Les données qui le montrent :
- supprimer S6K1, l'effecteur de mTORC1, allonge la vie des souris femelles (Selman 2009, doi:10.1126/science.1177221) ;
- les effets indésirables de la rapamycine sur le glucose viennent de l'inhibition « collatérale » de mTORC2, qu'on peut dissocier du gain de longévité (Lamming 2012, doi:10.1126/science.1215135).

**AMPK.** C'est le capteur du « carburant bas » (rapport AMP/ATP élevé). Il inhibe mTORC1, active l'autophagie et la formation de nouvelles mitochondries (via PGC-1α). Sa capacité à être activé diminue avec l'âge. AMPK est la cible indirecte de la metformine et de l'exercice.

**Sirtuines.** Ce sont des enzymes qui dépendent du NAD+ et retirent des groupes acétyle des protéines. Elles relient l'état métabolique à la réparation de l'ADN, à l'épigénome et à la fonction mitochondriale. Chez la souris mâle, surexprimer **SIRT6** allonge la vie (Kanfi 2012, doi:10.1038/nature10815). Comme le NAD+ baisse avec l'âge, les sirtuines tournent au ralenti (voir hallmark 07). Les promesses du resvératrol comme « activateur de sirtuines » ne se sont pas confirmées chez l'humain.

**Causes environnementales.** Suralimentation, excès de protéines et de sucres, sédentarité et obésité maintiennent l'IIS et mTOR en mode « croissance ». Dans la cohorte NHANES, un apport protéique élevé entre 50 et 65 ans est associé à des IGF-1 plus élevés et à plus de cancers et de décès, avec une inversion après 65 ans (Levine 2014, doi:10.1016/j.cmet.2014.02.006 ; étude observationnelle, donc à interpréter avec prudence). Synthèse régime-longévité : Fontana & Partridge 2015 (doi:10.1016/j.cell.2015.02.020).

## 3. Symptômes et manifestations cliniques

- **Résistance à l'insuline, diabète de type 2, syndrome métabolique** : c'est l'expression clinique la plus directe. Le diabète de type 2 accélère presque tous les aspects du vieillissement (maladies cardiovasculaires, rénales, cognitives).
- **Cancer** : l'excès d'IIS/mTOR favorise la prolifération. L'IGF-1 bas des patients Laron les protège (Guevara-Aguirre 2011).
- **Sarcopénie et fragilité** : à l'inverse, une réponse anabolique émoussée (« résistance anabolique » du muscle âgé) fait perdre de la masse musculaire. Le problème n'est pas « trop » ou « pas assez » de signal, c'est la perte de flexibilité.
- **Immunosénescence** : une activité mTOR excessive dans les cellules immunitaires âgées affaiblit la réponse aux vaccins. Des inhibiteurs de mTOR à faible dose améliorent cette réponse (Mannick 2014, doi:10.1126/scitranslmed.3009892).
- **Preuves humaines génétiques** : variants IGF1R chez les centenaires, FOXO3A, syndrome de Laron (voir §2).

## 4. Interactions avec les autres hallmarks

- **Macroautophagie altérée (05)** : mTORC1 actif bloque l'autophagie, AMPK la stimule. C'est le lien le plus direct, et la rapamycine agit en grande partie par cette voie.
- **Perte de protéostasie (04)** : mTORC1 pousse à fabriquer des protéines plus vite que la cellule ne peut contrôler leur qualité.
- **Dysfonction mitochondriale (07)** : AMPK/PGC-1α et sirtuines (SIRT1, SIRT3) commandent la formation et la qualité des mitochondries. La baisse du NAD+ relie les deux hallmarks.
- **Sénescence cellulaire (08)** : mTOR régule la traduction de l'IL-1α et donc le SASP (sécrétion inflammatoire des cellules sénescentes) ; la rapamycine le réduit (Laberge 2015, doi:10.1038/ncb3195).
- **Altérations épigénétiques (03)** : les sirtuines (SIRT1, SIRT6) sont des désacétylases d'histones, et le métabolisme fournit les substrats des marques épigénétiques (acétyl-CoA, NAD+, SAM).
- **Instabilité génomique (01)** : SIRT6 participe à la réparation des cassures de l'ADN.
- **Épuisement des cellules souches (09)** : un mTOR hyperactif épuise les cellules souches hématopoïétiques.
- **Inflammation chronique (11)** et **altération de la communication intercellulaire (10)** : hyperinsulinémie, adipokines, signalisation IGF-1 systémique.
- **Dysbiose (12)** : le microbiote module la sensibilité à l'insuline et la production d'acides gras à chaîne courte, qui signalent via AMPK.

## 5. Remèdes existants ou à l'étude

| Intervention | Cible | Niveau de preuve | Résultat clé | Source |
|---|---|---|---|---|
| Rapamycine (sirolimus), dès 20 mois chez la souris | mTORC1 | Préclinique robuste (ITP, 3 sites) | Durée de vie +9 % (mâles) et +14 % (femelles), même commencée tard | Harrison 2009, doi:10.1038/nature08221 |
| Rapamycine transitoire (3 mois, âge moyen) | mTORC1 | Préclinique | Jusqu'à +60 % d'espérance de vie restante, effet durable | Bitto 2016, doi:10.7554/eLife.16351 |
| Everolimus (RAD001) faible dose, 6 sem. | mTORC1 | Essai humain phase 2 (n≈218, ≥65 ans) | Réponse au vaccin grippal augmentée d'environ 20 % | Mannick 2014, doi:10.1126/scitranslmed.3009892 |
| RAD001 + BEZ235 faible dose | mTORC1 (+ PI3K) | Essai humain phase 2a | Moins d'infections respiratoires sur un an | Mannick 2018, doi:10.1126/scitranslmed.aaq1564 |
| RTB101 (dactolisib) | TORC1 | Essai humain phase 2b puis phase 3 | Phase 2b positive ; la phase 3 n'atteint pas son critère principal (infections respiratoires « cliniquement symptomatiques ») | Mannick 2021, doi:10.1016/S2666-7568(21)00062-3 |
| Rapamycine 5-10 mg/semaine, 48 sem. (PEARL) | mTORC1 | Essai humain phase 2 (n=114) | Bien tolérée ; signal modeste chez les femmes à 10 mg (masse maigre, douleur) ; forme magistrale peu absorbée | Moel 2025, doi:10.18632/aging.206235 |
| Metformine | AMPK / complexe I mitochondrial | Approuvée (diabète) ; essai TAME conçu mais non lancé : aucun enregistrement sur ClinicalTrials.gov (recherche du 19/10/2026), page AFAR hors ligne ; plusieurs sources secondaires de 2026 le disent toujours non financé [à vérifier : pas de source primaire] | Diabétiques sous metformine : mortalité observée comparable ou inférieure à celle des non-diabétiques (données observationnelles, biais possibles) | Bannister 2014, doi:10.1111/dom.12354 ; Campbell 2017, doi:10.1016/j.arr.2017.08.003 ; justification de TAME : Barzilai 2016, doi:10.1016/j.cmet.2016.05.011 |
| Metformine + exercice | — | Essai humain (RCT) | La metformine **atténue** les gains mitochondriaux de l'entraînement chez l'âgé | Konopka 2018, doi:10.1111/acel.12880 |
| Restriction calorique (≈12 % effective, 2 ans, CALERIE) | IIS, mTOR, AMPK, sirtuines | Essai humain phase 2 (non-obèses) | Facteurs de risque cardiométaboliques améliorés | Kraus 2019, doi:10.1016/S2213-8587(19)30151-2 |
| CALERIE, horloges biologiques | — | Analyse secondaire d'un RCT | Ralentissement du rythme de vieillissement (DunedinPACE) ; peu d'effet sur les horloges de type GrimAge | Belsky 2017, doi:10.1093/gerona/glx096 ; Waziry 2023, doi:10.1038/s43587-022-00357-y |
| Restriction calorique chez le primate | — | Préclinique (singes rhésus) | Moins de maladies liées à l'âge, survie améliorée ; résultats NIA/Wisconsin réconciliés | Colman 2009, doi:10.1126/science.1173635 ; Mattison 2017, doi:10.1038/ncomms14063 |
| Acarbose | Pic glycémique post-prandial | Préclinique (ITP) ; approuvé pour le diabète | Durée de vie augmentée, surtout chez les mâles | Harrison 2014, doi:10.1111/acel.12170 |
| Restriction protéique / en méthionine | mTOR, IGF-1 | Observationnel humain ; préclinique | IGF-1 plus bas, moins de mortalité entre 50 et 65 ans | Levine 2014, doi:10.1016/j.cmet.2014.02.006 |
| Inhibition génétique S6K1 | mTORC1 → S6K1 | Préclinique | Allonge la vie (femelles) | Selman 2009, doi:10.1126/science.1177221 |
| Surexpression SIRT6 | Sirtuine | Préclinique | Allonge la vie (mâles) | Kanfi 2012, doi:10.1038/nature10815 |
| Exercice physique | AMPK, sensibilité à l'insuline | Recommandé (preuves épidémiologiques et RCT) | Active AMPK et PGC-1α, restaure la sensibilité à l'insuline | Revue : Fontana 2015, doi:10.1016/j.cell.2015.02.020 |

Pour le contexte pharmacologique général (rapalogues, inhibiteurs sélectifs de mTORC1) : Bonkowski & Sinclair 2016, doi:10.1038/nrm.2016.93.

## 6. Questions ouvertes

1. **Dose et rythme pour la rapamycine chez l'humain** : faut-il une prise intermittente pour épargner mTORC2 ? Il n'existe pas encore de biomarqueur fiable de l'inhibition de mTORC1 dans les tissus. Les inhibiteurs sélectifs de mTORC1 restent à valider cliniquement.
2. **Metformine** : effet anti-âge réel, ou artefact des études observationnelles (biais de « survie » et de comparateur) ? TAME n'est toujours pas lancé, et la metformine gêne l'adaptation à l'exercice.
3. **Le paradoxe IGF-1** : un IGF-1 bas protège-t-il ou fragilise-t-il les personnes âgées ? L'effet semble dépendre de l'âge, du sexe et du tissu.
4. **Différences selon le sexe** : beaucoup d'interventions (acarbose, 17α-estradiol, S6K1, SIRT6) agissent de façon très différente chez les mâles et les femelles.
5. **Restriction calorique contre jeûne intermittent contre composition protéique** : qu'est-ce qui compte vraiment ? L'adhérence à long terme reste un obstacle majeur.
6. **Sirtuines et NAD+** : les activateurs (resvératrol, STAC) n'ont pas confirmé leurs promesses ; les précurseurs de NAD+ sont traités au hallmark 07.

## 7. Références

- López-Otín C et al. Hallmarks of aging: An expanding universe. *Cell* 2023. doi:10.1016/j.cell.2022.11.001
- Kenyon C et al. A C. elegans mutant that lives twice as long as wild type. *Nature* 1993. doi:10.1038/366461a0
- Suh Y et al. Functionally significant IGF-I receptor mutations in centenarians. *PNAS* 2008. doi:10.1073/pnas.0705467105
- Willcox BJ et al. FOXO3A genotype is strongly associated with human longevity. *PNAS* 2008. doi:10.1073/pnas.0801030105
- Guevara-Aguirre J et al. Growth hormone receptor deficiency… *Sci Transl Med* 2011. doi:10.1126/scitranslmed.3001845
- Saxton RA, Sabatini DM. mTOR signaling in growth, metabolism, and disease. *Cell* 2017. doi:10.1016/j.cell.2017.02.004
- Selman C et al. Ribosomal protein S6 kinase 1 signaling regulates mammalian life span. *Science* 2009. doi:10.1126/science.1177221
- Lamming DW et al. Rapamycin-induced insulin resistance is mediated by mTORC2 loss… *Science* 2012. doi:10.1126/science.1215135
- Kanfi Y et al. The sirtuin SIRT6 regulates lifespan in male mice. *Nature* 2012. doi:10.1038/nature10815
- Levine ME et al. Low protein intake is associated with a major reduction in IGF-1, cancer, and overall mortality… *Cell Metab* 2014. doi:10.1016/j.cmet.2014.02.006
- Fontana L, Partridge L. Promoting health and longevity through diet. *Cell* 2015. doi:10.1016/j.cell.2015.02.020
- Laberge RM et al. MTOR regulates the pro-tumorigenic SASP… *Nat Cell Biol* 2015. doi:10.1038/ncb3195
- Harrison DE et al. Rapamycin fed late in life extends lifespan… *Nature* 2009. doi:10.1038/nature08221
- Bitto A et al. Transient rapamycin treatment can increase lifespan and healthspan in middle-aged mice. *eLife* 2016. doi:10.7554/eLife.16351
- Mannick JB et al. mTOR inhibition improves immune function in the elderly. *Sci Transl Med* 2014. doi:10.1126/scitranslmed.3009892
- Mannick JB et al. TORC1 inhibition enhances immune function and reduces infections in the elderly. *Sci Transl Med* 2018. doi:10.1126/scitranslmed.aaq1564
- Mannick JB et al. Targeting the biology of ageing with mTOR inhibitors… phase 2b and phase 3 trials. *Lancet Healthy Longev* 2021. doi:10.1016/S2666-7568(21)00062-3
- Moel M et al. Influence of rapamycin on safety and healthspan metrics after one year: PEARL trial results. *Aging* 2025. doi:10.18632/aging.206235
- Barzilai N et al. Metformin as a tool to target aging. *Cell Metab* 2016. doi:10.1016/j.cmet.2016.05.011
- Bannister CA et al. Can people with type 2 diabetes live longer than those without? *Diabetes Obes Metab* 2014. doi:10.1111/dom.12354
- Campbell JM et al. Metformin reduces all-cause mortality and diseases of ageing… *Ageing Res Rev* 2017. doi:10.1016/j.arr.2017.08.003
- Konopka AR et al. Metformin inhibits mitochondrial adaptations to aerobic exercise training in older adults. *Aging Cell* 2019. doi:10.1111/acel.12880
- Kraus WE et al. 2 years of calorie restriction and cardiometabolic risk (CALERIE). *Lancet Diabetes Endocrinol* 2019. doi:10.1016/S2213-8587(19)30151-2
- Belsky DW et al. Change in the rate of biological aging in response to caloric restriction. *J Gerontol A* 2017. doi:10.1093/gerona/glx096
- Waziry R et al. Effect of long-term caloric restriction on DNA methylation measures of biological aging. *Nat Aging* 2023. doi:10.1038/s43587-022-00357-y
- Colman RJ et al. Caloric restriction delays disease onset and mortality in rhesus monkeys. *Science* 2009. doi:10.1126/science.1173635
- Mattison JA et al. Caloric restriction improves health and survival of rhesus monkeys. *Nat Commun* 2017. doi:10.1038/ncomms14063
- Harrison DE et al. Acarbose, 17-α-estradiol, and NDGA extend mouse lifespan preferentially in males. *Aging Cell* 2014. doi:10.1111/acel.12170
- Bonkowski MS, Sinclair DA. Slowing ageing by design: the rise of NAD+ and sirtuin-activating compounds. *Nat Rev Mol Cell Biol* 2016. doi:10.1038/nrm.2016.93
