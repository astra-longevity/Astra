# Mouse glucose tolerance test (GTT)

Axis: A10 liver and metabolism (axes.md @116f1f8). Related: A15 fat (body composition drives the result).

## What it measures

After a fast, the mouse gets a glucose dose (injected into the belly, "intraperitoneal" (IP), or by mouth, "oral"), and blood glucose is read from the tail at set times (usually 0, 15, 30, 60, 90 and 120 min). The reading is the area under the glucose curve (AUC): higher AUC = worse glucose handling. It mixes insulin release, insulin sensitivity of muscle, liver and fat, and insulin clearance. It does not separate them; a clamp study or an insulin tolerance test (ITT) is needed for that.

## Training or reference population

No standard reference. Protocols vary across labs (Moro 2024, review table):
- **Fast:** 5–6 h (most common) or overnight 14–18 h.
- **Dose:** 1–3 g/kg, by body weight or by lean mass.
- **Route:** IP, oral or intravenous.

Age data are almost all from C57BL/6 (B6), mostly males. No UM-HET3 reference curve found `[to verify]`.

## Noise

- No published same-mouse test-retest coefficient (ICC or CV) found. `[to verify]`
- Females gave a smaller but **more reproducible** GTT response than males once the protocol was optimised (Kennard 2022).
- Sources of noise the protocol can control (Kennard 2022): handling when the fast starts (females were less sensitive to it); keeping the old bedding when changing the cage for a 6 h fast, which lets blood glucose settle faster.
- Smallest real change for one mouse: not published.

## Known failure modes

- **Fast length.** A 16 h fast exaggerated the GTT response but caused low blood sugar at baseline (Kennard 2022). Overnight (12 h) and 24 h fasts caused >10% body-weight loss and muscle wasting, which itself makes mice insulin resistant (Moro 2024). Bellantuono 2020 instead recommends a 16 h overnight fast for ageing studies, since short fasts were more variable in their hands. **The two camps disagree; Astra must pick one protocol and never pool across them.**
- **Dose by body weight.** Fat mice get a bigger dose although fat takes up little glucose; dosing by lean mass is recommended (Moro 2024, citing McGuinness 2009). In old mice, whose fat and lean mass change in opposite directions (below), this matters a lot.
- **Route.** In one comparison, high-fat vs chow differences were larger after oral than IP glucose (142% vs 127%); a 6 h fast with 2 g/kg oral was judged best (Andrikopoulos 2008).
- **Late-life weight loss improves the GTT.** In very old mice, fat loss lowers AUC, so a "better" GTT can be a sign of decline, not health (below).

## Functional anchor and age trajectory

The age curve in B6 mice is **not monotonic**: glucose tolerance worsens into middle age, then improves again in late life.

| Source | Mice | Finding |
| --- | --- | --- |
| Reynolds 2019 | Male and female C57BL/6J | Males: AUC 95,103 ± 6,818 at 18 months vs 64,005 ± 2,031 at 6 months (worse, p = 0.002). Females: no age decline. From 20 to 28 months (n = 7) GTT improved markedly while fat fell from 12.5 to 4.8 g (32.5% to 14.0% body fat) and lean mass rose from 20.1 to 23.4 g |
| Reynolds 2024 | Male C57BL/6J, young, 18 mo, 26–27 mo | 26–27-month mice had lower glucose during the IP GTT than young and 18-month mice, with less fat; no age difference in insulin tolerance |
| Kondo 2023 | Male C57BL/6N, four ages | Insulin resistance appears at mature adulthood, then improves markedly; glucose uptake 24.3, 17.1, 25.5 and 31.8 mg/kg/min in young, mature adult, presenile and aged mice, tracking visceral fat |
| Marmentini 2021 | Male C57BL/6, 3 vs 18 months | 18-month mice: lower glucose at 15 and 30 min, lower AUC, higher insulin, unchanged insulin sensitivity, lower insulin clearance. The same group found glucose intolerance at 10 months |
| Ehrhardt 2019 | Aged vs young mice matched for body composition, chow and high-fat diet | GTT not worse with age (slightly better); clamp showed lower insulin sensitivity and lower insulin clearance in aged mice |
| Leiter 1988 | Male C57BL/6J | Glucose tolerance did not deteriorate with age; no loss of beta-cell response |

Take-home: in B6 mice, GTT AUC is a poor ageing readout on its own. It tracks fat mass, rises then falls with age, and hides real age changes (lower insulin sensitivity and clearance) seen in clamp studies (Ehrhardt 2019; Marmentini 2021). Use it only with body composition measured on the same day, dose by lean mass, and read alongside fasting insulin. A clamp or insulin-clearance measure is the better anchor for A10 `[to verify]` feasibility at scale.

## Confounders

Fast length and cage handling, dose basis (body weight vs lean mass), route, sex, strain and substrain, fat and lean mass, time of day, stress from handling and tail bleeding, late-life weight loss.

## Sources

- Moro C et al. Revisited guidelines for metabolic tolerance tests in mice. Lab Anim 2024. doi:10.1038/s41684-024-01473-5 (Crossref OK)
- Kennard MR et al. The glucose tolerance test in mice: sex, drugs and protocol. Diabetes Obes Metab 2022. doi:10.1111/dom.14811 (Crossref OK)
- Andrikopoulos S et al. Evaluating the glucose tolerance test in mice. Am J Physiol Endocrinol Metab 2008. doi:10.1152/ajpendo.90617.2008 (Crossref OK)
- Bellantuono I et al. A toolbox for the longitudinal assessment of healthspan in aging mice. Nat Protoc 2020. doi:10.1038/s41596-019-0256-1 (Crossref OK)
- Reynolds TH et al. The impact of age and sex on body composition and glucose sensitivity in C57BL/6J mice. Physiol Rep 2019. doi:10.14814/phy2.13995 (Crossref OK)
- Reynolds TH et al. The oldest of old male C57B/6J mice are protected from sarcopenic obesity. Int J Mol Sci 2024. doi:10.3390/ijms251910278 (Crossref OK)
- Kondo et al. Insulin sensitivity initially worsens but later improves with aging in male C57BL/6N mice. J Gerontol A 2023. doi:10.1093/gerona/glad126 (Crossref OK)
- Marmentini C et al. Aging reduces insulin clearance in mice. Front Endocrinol 2021. doi:10.3389/fendo.2021.679492 (Crossref OK)
- Ehrhardt N et al. Adiposity-independent effects of aging on insulin sensitivity and clearance in mice and humans. Obesity 2019. doi:10.1002/oby.22418 (Crossref OK)
- Leiter EH et al. Aging and glucose homeostasis in C57BL/6J male mice. FASEB J 1988. doi:10.1096/fasebj.2.12.3044905 (Crossref OK)
