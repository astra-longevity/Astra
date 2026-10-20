# Mouse skin: wound closure and coat condition

Axis: A13 skin (axes.md @116f1f8). Related: A18 frailty (coat items are part of the frailty index, file 01), A11 immune, A16 endocrine (sex hormones), A15 fat.

## What it measures

- **Wound closure:** under anaesthesia, round full-thickness wounds are cut in the shaved back skin with a biopsy punch. The wounds are photographed or measured over the following days. Readings: wound area as % of day 0, time to full closure, and, on tissue sections (terminal), re-epithelialisation (how much new surface skin has grown over the wound). In the "splinted" version a ring is stitched around the wound so it cannot simply shrink by skin contraction, which is how loose-skinned rodents mostly close wounds; this is closer to human healing (Mellers 2018).
- **Coat condition:** scored by eye, as in the frailty index: fur loss (alopecia), loss of fur colour (greying), and poor grooming (ruffled or matted coat). Scores are 0, 0.5 or 1 per item (Crum 2025).

## Training or reference population

No standard reference. Wound data come from small groups of young vs old C57BL/6 or BALB/c mice. Coat data come from frailty-index cohorts and from the NCGG Aging Farm colony of C57BL/6J and C57BL/6N mice (Ogiso 2025). No UM-HET3 data found.

## Noise

- Test-retest noise and smallest real change for one mouse: none found in the sources read.
- Wound closure needs fresh wounds, so each test is a new injury; the same wound cannot be measured twice from scratch.
- Coat scores are coarse (three levels per item) and scored by eye.

## Known failure modes

- **Wound area and true healing disagree.** Closure by wound surface area did not correlate with re-epithelialisation measured on sections, and results changed with sex, strain and splinting (Bagood 2021). The metric must state which reading and which model (splinted or not).
- **Most of the decline happens by middle age.** Young (6 months) C57BL/6J males closed wounds faster than 15-month and 26–27-month mice, and the 15- and 26–27-month mice did not differ (Cohen 1987). Like the lung (file 10), this may measure mid-life change more than late ageing.
- **Sex and oestrogen.** In BALB/cByJ, young females healed faster than old females, while males showed no significant age effect (Mellers 2018). Topical oestrogen sped up the delayed healing of 80-week-old female mice (Mukai 2019). Sex and oestrous status change the answer.
- **Coat loss is not mainly ageing in B6.** Alopecia was more common in C57BL/6J females from 3 months on, peaked at about 18 months, and was rarer in B6N; ulcers and bleeding wounds in B6N males came mostly from fighting (Ogiso 2025). Males mostly developed alopecia before 12 months, without progressing to severe loss (Crum 2025). Barbering, fighting and cage effects mimic ageing.
- **Diet and fat.** Old mice on a high-fat diet gained more body fat, and age changed the expression of healing genes at particular stages (Kopcewicz 2020). Fat mass must be recorded.

## Functional anchor and age trajectory

| Source | Mice | Finding |
| --- | --- | --- |
| Cohen 1987 | C57BL/6J males, 6, 15, 26–27 months | Young closed faster; 15 and 26–27 months did not differ |
| Mellers 2018 | BALB/cByJ, splinted, daily photos | Age slowed healing in females; males showed no significant age difference |
| Mukai 2019 | Females, 80 weeks | Delayed healing in old females; topical oestrogen reduced wound area and inflammation |
| Kopcewicz 2020 | C57BL/6, both sexes, 5 vs 21 months, low- or high-fat diet | Age affected healing at particular stages (TGFβ-1, MCP-1, MMP-9, MMP-13 expression) |
| Bagood 2021 | C57BL/6 and BALB/c, both sexes, splinted vs not | Wound area closure did not match re-epithelialisation; the "impaired healing in old mice" result depended on model, sex and strain |
| Shvedova 2025 (preprint) | Young vs aged mice | Aged mice had larger open wounds from day 6; nearly all young closed by day 18, most aged 3–6 days later (sole source for these day counts; preprint, not peer-reviewed) |
| Ogiso 2025 | C57BL/6J and C57BL/6N, both sexes, to 21 months+ | Alopecia highest in B6J females, peak about 18 months; ulcers in B6N males from fighting |
| Crum 2025 | Frailty-index cohort, both sexes | Coat items scored 0/0.5/1; male alopecia mostly before 12 months; female alopecia variable |

Take-home: wound healing slows with age, but the delay is often visible by middle age, depends on sex, strain and model, and wound area alone can mislead. Coat condition is cheap and repeatable but in B6 it is driven by sex, substrain, barbering and fighting more than by age. Use both only with sex as a factor, a fixed wound model, and housing notes.

## Confounders

Sex and oestrous status, strain and substrain, splinted vs unsplinted model, wound size and site, anaesthesia, infection, diet and fat mass, hair-cycle stage at shaving, barbering and fighting (group housing), observer for coat scores.

## Sources

- Cohen et al. Wound repair in mice as influenced by age and antimacrophage serum. J Gerontol 1987. doi:10.1093/geronj/42.3.295 (Crossref OK)
- Mellers et al. Fine-sampled photographic quantitation of dermal wound healing senescence in aged BALB/cByJ mice and therapeutic intervention with fibroblast growth factor-1. Adv Wound Care 2018. doi:10.1089/wound.2018.0801 (Crossref OK)
- Mukai et al. Topical estrogen application to wounds promotes delayed cutaneous wound healing in 80-week-old female mice. PLoS One 2019. doi:10.1371/journal.pone.0225880 (Crossref OK)
- Kopcewicz et al. Cutaneous wound healing in aged, high fat diet-induced obese female or male C57BL/6 mice. Aging 2020. doi:10.18632/aging.103064 (Crossref OK)
- Bagood et al. Re-examining the paradigm of impaired healing in the aged murine excision wound model. J Invest Dermatol 2021. doi:10.1016/j.jid.2020.10.022 (Crossref OK)
- Shvedova et al. Diminished and altered cellular senescence response in delayed wound healing of aging. bioRxiv preprint 2025. doi:10.1101/2025.06.08.658533 (Crossref OK; not peer-reviewed)
- Ogiso et al. Biological characteristics of age-related changes in C57BL/6 mice sub-strains in the National Center for Geriatrics and Gerontology Aging Farm. Exp Anim 2025. doi:10.1538/expanim.24-0095 (Crossref OK)
- Crum et al. Frailty assessment in an aging mouse model. J Vis Exp 2025. doi:10.3791/69076 (Crossref OK)
