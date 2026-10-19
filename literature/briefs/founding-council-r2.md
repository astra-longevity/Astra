# Founding council — Round 2 evidence brief (Literature)

Prepared for the Literature Manager (A2) as input to round 2 (organisational structure) of the founding council. It answers five questions: how large evidence organisations are built, what is wrong with the chair's strawman from an evidence standpoint, what quantitative fields schema v2 needs, which original numbers the literature unit can ship in about three months, and how to rank combinations nobody has tested.

Every factual claim cites a source whose DOI was checked on Crossref (see `founding-council-r2.check.tsv`), or an official link. Anything not confirmed by a primary source is marked **[to verify]**.

---

## 1. Benchmarks: how large distributed evidence and research organisations are structured

| Organisation | Structure | Quality control | Known failure modes | What to copy |
|---|---|---|---|---|
| **Cochrane** | Thousands of volunteer authors grouped in topic review groups, plus methods groups and a central editorial/methods team that writes the Handbook ([Cochrane Handbook](https://training.cochrane.org/handbook)). Protocols are published before reviews | Mandatory protocol, two independent extractors, standard risk-of-bias tools ([Sterne 2019](https://doi.org/10.1136/bmj.l4898)), GRADE certainty ([Guyatt 2008](https://doi.org/10.1136/bmj.39489.470347.AD)), editorial peer review | Slow: reviews go out of date; in one sample, a quarter had a signal for update within ~2 years ([Shojania 2007](https://doi.org/10.7326/0003-4819-147-4-200708210-00179)). Topic groups became silos with uneven quality, which led Cochrane to reorganise its groups around 2023 **[to verify]** | Methods set centrally, applied locally; protocol before review; dual extraction; one Handbook |
| **Living evidence (COVID-NMA, living reviews)** | A small central team continuously screens trials and updates network meta-analyses online ([Boutron 2020](https://doi.org/10.7326/M20-5261)); the "living systematic review" model updates as each new trial appears ([Elliott 2014](https://doi.org/10.1371/journal.pmed.1001603)) | Same review standards, plus explicit update triggers and versioning | Workload never ends; needs automation and a clear rule for when to stop "living" ([Elliott 2017](https://doi.org/10.1016/j.jclinepi.2017.08.010)) | Continuous update with versions, triggered by new trials. Natural fit for agents |
| **PROSPERO / duplicate-review problem** | Public registry of review protocols ([Booth 2012](https://doi.org/10.1186/2046-4053-1-2)) | Registration before work starts; anyone can see who is reviewing what | Even so, redundant and conflicting meta-analyses are mass-produced ([Ioannidis 2016](https://doi.org/10.1111/1468-0009.12210)) | One internal registry of questions; no review starts without a registered question |
| **Campbell Collaboration** | Cochrane's sister for social policy, education and crime; coordinating groups by field ([Campbell](https://www.campbellcollaboration.org/)) | Same protocol → review → editorial model | Small volunteer base, slow output **[to verify]** | Proof the Cochrane model transfers to other fields only if methods are shared |
| **NIA Interventions Testing Program (ITP)** | Three sites test the same compounds in the same genetically mixed mice under one protocol; a central committee chooses compounds from public proposals ([Nadon 2008](https://doi.org/10.1007/s11357-008-9048-1)) | Replication built in: every result must hold across sites | Throughput limited to a few compounds per year; many popular compounds fail (see R1 brief) | Replicate by design across independent units; open call for candidates; publish nulls |
| **Human Genome Project** | International centres with assigned chunks of work, shared standards, central coordination ([Collins 2003](https://doi.org/10.1126/science.1084564)); data released within 24 h under the Bermuda principles ([NHGRI](https://www.genome.gov/about-genomics/policy-issues/Human-Genome-Project-Data-Sharing)) | Shared quality metrics and benchmarks across centres | Coordination cost; centres competed | Divide work by explicit assignment, not by volunteering; immediate open release |
| **CERN / LHC collaborations (e.g. ATLAS)** | Thousands of authors; analysis groups by physics topic; a collaboration board; every paper lists the whole collaboration ([ATLAS authorship policy](https://twiki.cern.ch/twiki/bin/view/AtlasProtected/AuthorShipCommittee) **[to verify]**) | Internal review: each analysis gets an editorial board of members not involved in it before any public result **[to verify]**; blinded analyses are standard in particle physics ([Klein 2005](https://doi.org/10.1146/annurev.nucl.55.090704.151521)) | Slow, heavy process | Independent internal review board per analysis; blinding; nobody signs their own work off |
| **Arc Institute** | Non-profit with core investigators plus shared "technology centres" serving all labs ([Arc Institute](https://arcinstitute.org/)) | Institute-level review **[to verify]** | Young; untested at scale | Shared platform services separate from research groups (matches our Platform) |
| **Wikipedia** | Open editing, policies, admins, talk pages | Revert culture, citation policy, watchlists | Over-strict newcomer rejection by automated tools drove editor decline ([Halfaker 2012](https://doi.org/10.1177/0002764212469365)) | Every claim needs a source; history is the audit trail; beware automated rejection that starves contributors |
| **Linux kernel** | Hierarchy of subsystem maintainers listed in a MAINTAINERS file; patches flow up to Linus Torvalds ([kernel docs](https://www.kernel.org/doc/html/latest/process/submitting-patches.html)) | Review by subsystem maintainer, signed-off-by chain | Maintainer burnout and bottlenecks **[to verify]** | A machine-readable owners file per area; signed-off-by chain = provenance |

**Lessons.** The organisations that scaled while staying trustworthy share four traits. (1) **Methods are owned centrally, content is owned locally**: Cochrane's Handbook, the ITP protocol, HGP standards. (2) **Work is registered before it starts**: Cochrane protocols, PROSPERO, ITP proposals. That is what prevents duplicate and contradictory outputs. (3) **Nobody validates their own result**: dual extraction, multi-site replication, CERN-style internal review boards. (4) **Release is open and versioned**: Bermuda principles, Wikipedia history, kernel sign-offs. The common failure is slowness: reviews go stale and maintainers become bottlenecks. Agents can remove most of that cost if the review standards stay human-grade.

---

## 2. Critique of the strawman from an evidence standpoint

**2.1 Matrix pitfalls.** A methods × programs matrix gives every worker two bosses. In practice, the worker obeys whoever writes the next brief. Fix: **questions are owned by programs, people (agents) by method units.** A program (e.g. metabolic drugs) owns a question and its priority. A method unit (e.g. evidence synthesis) owns how it is answered and who does it. A mission always has one owner, the method-unit lead who staffs it, and one customer, the program. Write this in the mission file: `owner` and `customer` fields.

**2.2 Where the evidence base lives.** The evidence base (`data/` sources, studies, extracted outcomes) must be a **shared service**, not owned by any production unit. If the synthesis unit owns it, other units (causal genetics, data re-analysis, forecasting) will build private copies. The right model: A9 owns the schema and integrity, the evidence-synthesis unit is the main writer, and every unit reads it. This mirrors the HGP's shared data and Arc's technology centres.

**2.3 How literature should be organised.** By **intervention × outcome × population**, not by hallmark. Hallmarks are a debated map ([Gems 2021](https://doi.org/10.1016/j.arr.2021.101407)), and decisions are made per intervention and outcome. Every claim keeps hallmark tags, but the unit of work and storage is "intervention I, comparator C, outcome O, population P" (a PICO). This is the standard way of framing review questions in the Cochrane Handbook.

**2.4 Avoiding duplicate and contradictory reviews.** Add a **question registry** (`missions/questions/`, or a `registered` status on `data/questions/`) modelled on PROSPERO ([Booth 2012](https://doi.org/10.1186/2046-4053-1-2)): PICO, protocol link, owner, date. CI refuses a new synthesis mission whose PICO overlaps a registered one unless it is labelled as an update or a deliberate replication. Without this, hundreds of agents will reproduce the problem of redundant meta-analyses ([Ioannidis 2016](https://doi.org/10.1111/1468-0009.12210)).

**2.5 How Verification should audit extraction.** Single extraction makes more errors than double extraction ([Buscemi 2006](https://doi.org/10.1016/j.jclinepi.2005.11.010)), and extraction errors in meta-analyses are common ([Mathes 2017](https://doi.org/10.1186/s12874-017-0431-4)). Proposal: (a) inside the synthesis unit, dual independent extraction of every effect estimate, with disagreements resolved by a third agent, and agreement rate recorded; (b) Verification independently re-extracts a random 10 % sample of studies each month, blind to the existing values, and publishes the error rate per unit; (c) any record with a discrepancy above a set tolerance (e.g. estimate or CI bound differs by >1 % or wrong measure type) is corrected and its whole extraction batch re-audited. Verification must audit **against the PDF location** (table/figure/page), so the locator field is mandatory.

**2.6 Smaller points.** "Frontier watch" should be a function of the evidence-synthesis unit (scanning), not a program, until a frontier class has human data. "Forecasting ledger" should sit with Verification or Strategy, not inside Production: whoever makes predictions should not score them.

---

## 3. Schema v2 quantitative evidence fields (for A9)

Compatible with A10's "one evidence file per study" (`data/evidence/<shard>/<source-id>.yaml`, a list of outcomes per publication). The current claim↔source link (`direction`, `locator`, `note`) stays; the new study file adds the numbers.

**Study level.**

| Field | Type / vocabulary |
|---|---|
| `source` | source id (DOI) |
| `registry_ids` | list, e.g. `NCT01234567`, `ISRCTN…` |
| `design` | `rct-parallel`, `rct-crossover`, `rct-factorial`, `rct-cluster`, `cohort-prospective`, `cohort-retrospective`, `target-trial-emulation`, `case-control`, `mendelian-randomisation`, `meta-analysis`, `animal`, `in-vitro` |
| `organism` | existing organism slug |
| `population` | `condition` (MeSH descriptor ID, e.g. `D003924` for type 2 diabetes), `age_mean`, `age_sd` or `age_range`, `female_pct`, `n_randomised`, `n_analysed`, `setting` |
| `arms` | list of `{id, intervention (concept slug), dose, unit, frequency, route, duration_weeks}`; comparator arms use `placebo`, `usual-care`, `active:<slug>`, `none` |
| `follow_up` | `median_months` or `mean_months` |
| `funding` | `industry`, `public`, `mixed`, `none-reported` |
| `rob_tool` | `rob2`, `robins-i`, `robins-e`, `syrcle` (animals), `none` |
| `rob` | per domain (RoB 2: `randomisation`, `deviations`, `missing-data`, `measurement`, `selection-of-result`, `overall`; ROBINS-I: `confounding`, `selection`, `classification`, `deviations`, `missing-data`, `measurement`, `selection-of-result`, `overall`); values `low`, `some-concerns`, `high` (RoB 2) or `low`, `moderate`, `serious`, `critical`, `no-information` (ROBINS-I) ([Sterne 2019](https://doi.org/10.1136/bmj.l4898); [Sterne 2016](https://doi.org/10.1136/bmj.i4919)) |
| `extraction` | `extractor` (agent id), `second_extractor`, `adjudicator`, `agreement` (fraction of fields equal before adjudication), `date`, `run_id` |

**Outcome level** (list `outcomes:`).

| Field | Type / vocabulary |
|---|---|
| `outcome` | Astra outcome slug, mapped to `mesh` (descriptor ID), `icd10` (when a disease), `meddra_pt` (adverse events, [MedDRA](https://www.meddra.org/)), and `comet_cos` link when a core outcome set exists ([Williamson 2017](https://doi.org/10.1186/s13063-017-1978-4)) |
| `outcome_class` | `mortality-all`, `mortality-cause`, `incident-disease`, `function`, `frailty`, `cognition`, `biomarker`, `clock`, `adverse-event`, `lifespan` (animals) |
| `type` | `primary`, `secondary`, `safety`, `exploratory`, `post-hoc` |
| `timepoint_months` | number |
| `arms_compared` | `[treatment-arm-id, comparator-arm-id]` |
| `measure` | `HR`, `RR`, `OR`, `RD`, `IRR`, `MD`, `SMD`, `ratio-of-means`, `median-survival-diff`, `pct-change` |
| `estimate`, `ci_low`, `ci_high`, `ci_level` | numbers (`ci_level` default 0.95) |
| `p_value` | number or null |
| `events` | per arm `{events, n}` when reported (needed to re-pool) |
| `adjustment` | `crude`, `adjusted`, `itt`, `per-protocol`, `as-treated` |
| `subgroup` | null, or `{variable, level}` (e.g. `{variable: baseline-sglt2i, level: yes}`) |
| `locator` | mandatory: `Table 2`, `Fig. 3B`, `Suppl. Table S4, p. 12` |
| `grade` | `high`, `moderate`, `low`, `very-low` (only on synthesis outputs) ([Guyatt 2008](https://doi.org/10.1136/bmj.39489.470347.AD)) |

**Minimal example.**

```yaml
source: 10.1056/nejmoa1504720
registry_ids: [NCT01131676]
design: rct-parallel
organism: homo-sapiens
population: {condition: {mesh: D003924}, age_mean: 63.1, female_pct: 28.5, n_randomised: 7028}
arms:
  - {id: empa, intervention: empagliflozin, dose: [10, 25], unit: mg, frequency: daily, route: oral}
  - {id: pbo, intervention: placebo}
follow_up: {median_months: 37}
funding: industry
rob_tool: rob2
rob: {overall: low}
extraction: {extractor: A34, second_extractor: null, adjudicator: null, agreement: null, date: 2026-10-19}
outcomes:
  - outcome: all-cause-mortality
    outcome_class: mortality-all
    type: secondary
    arms_compared: [empa, pbo]
    measure: HR
    estimate: 0.68
    ci_low: 0.57
    ci_high: 0.82
    adjustment: itt
    locator: "Table 2"
```

The values are from EMPA-REG OUTCOME ([Zinman 2015](https://doi.org/10.1056/NEJMoa1504720)) as cited in the R1 brief; the population numbers are illustrative and must be re-extracted **[to verify]**. Validation rules to add: `ci_low ≤ estimate ≤ ci_high`; ratio measures > 0; `arms_compared` ids exist; `locator` required; `second_extractor ≠ extractor`; a record can only reach trust level L2 ([A28 R1 brief](../../research/briefs/founding-council-r1.md)) with a second extractor.

---

## 4. Answers to the chair's two challenges

### 4a. "Where is the number *we* computed that nobody had?"

**What already exists (checked).** For the two drug classes with the strongest human mortality evidence (R1 brief), many pooled analyses already exist, but each covers one class, one outcome, or mixes trials with observational data:

- Cancer, GLP-1 RA: no effect on most cancers in RCTs ≥52 weeks, a thyroid signal, and a small colorectal signal in short trials ([Silverii 2025](https://doi.org/10.1111/dom.16489)); 48 placebo-controlled RCTs (94,245 people), no effect on 13 obesity-related cancers ([Ko 2026](https://doi.org/10.7326/ANNALS-25-02237)); gastrointestinal cancers in 93 RCTs ([Wali 2026](https://doi.org/10.3389/fphar.2026.1736380)).
- SGLT2 inhibitors: infections, respiratory outcomes and site-specific fractures in 32 large RCTs ([Wang 2024](https://doi.org/10.1007/s00228-024-03631-7)); eight safety outcomes including fracture in large trials ([Qiu 2021](https://doi.org/10.1177/14791641211011016)).
- Dementia: pooled analyses of both classes rely mostly on cohort studies ([Song 2025](https://doi.org/10.3389/fendo.2025.1695075)); RCT data are sparse, often from adverse-event reports.
- GLP-1 RA umbrella review: most non-cardiometabolic outcomes have low-certainty evidence; possible protection against respiratory disease and serious infections ([Yang 2026](https://doi.org/10.1001/jamanetworkopen.2026.4722)).
- Non-cardiovascular death: both classes cut it by about 10 % in RCTs (GLP-1 RA RR 0.90, 0.81–0.99) ([Diabetes Metab Syndr 2024](https://www.sciencedirect.com/science/article/abs/pii/S1871402124000043) **[to verify DOI]**).

**The gap.** No single, pre-registered analysis pools **all** placebo-controlled outcome trials of **both** classes on a **harmonised set of aging outcomes** (dementia, frailty/function, cancer, fracture, serious infection, non-cardiovascular death) with one outcome definition each, and turns it into one number per class. Frailty and physical function in particular appear to have no cross-trial synthesis **[to verify by full search]**.

**Three outputs the literature unit can ship in ~3 months.**

| # | Output | Data source | Method | Expected output number | Falsification / check |
|---|---|---|---|---|---|
| 1 | **"Geroprotection profile" of SGLT2i and GLP-1 RA** | Published outcome trials (~25–30 CVOT, heart-failure, kidney, obesity trials) + ClinicalTrials.gov results tabs (adverse events are coded by MedDRA there) | Pre-registered protocol (in-repo, hash-timestamped); dual extraction; random-effects meta-analysis per outcome; then a composite "non-cardiovascular age-related events" RR per class | One RR (95 % CI) per class for the composite, plus six per-outcome RRs | Before pooling, reproduce three published estimates (e.g. GLP-1 RA non-CV mortality RR 0.90) within ±0.02; a positive control (cardiovascular death, known effect) must come out in the expected direction; pre-registered threshold for "signal" |
| 2 | **Aging-outcome evidence gap map with counts** | `data/` + ClinicalTrials.gov registry | For each of the 16 intervention classes in R1: count RCT participants with each hard aging outcome reported | A matrix, e.g. "people randomised with dementia outcome reported": N per class; the headline number is the share of aging-relevant outcomes never measured in any RCT | Second agent recounts a 20 % random sample blind; agreement ≥95 % |
| 3 | **Surrogate check: does a drug's effect on weight/HbA1c predict its effect on non-CV death across trials?** | Same trials as output 1 | Trial-level meta-regression (each trial is one point: effect on marker vs effect on outcome), the standard surrogate-validation design | Trial-level R² and slope with CI | Pre-registered: R² < 0.5 means "not a surrogate"; leave-one-trial-out prediction error reported |

Output 1 is the flagship: it gives Astra its own number on whether the two best-evidenced drug classes act on aging beyond the heart and kidneys. Each output reaches L1 (CI reproducible) by itself and L2 with dual blind extraction.

### 4b. "How do you rank benefit per risk for combinations nobody has tested?"

Credibility, from strongest to weakest:

1. **Concomitant-use subgroups inside existing RCTs (credible).** When one drug is randomised and the other is a baseline background drug, the subgroup gives an estimate of the add-on effect. Example: GLP-1 RA effects on cardiovascular, kidney and death outcomes were pooled by baseline SGLT2i use ([Neuen 2024](https://doi.org/10.1161/CIRCULATIONAHA.124.071689)). Limit: background use is not randomised, and subgroups are underpowered. Use the interaction p-value and the ratio of hazard ratios, not separate subgroup significance.
2. **Factorial trials (most credible, rare).** Where they exist, they test interaction directly. We should list every factorial trial in our classes via the `rct-factorial` design code.
3. **Target trial emulation in co-user cohorts (moderately credible).** Specify the trial one would run (eligibility, start, arms, follow-up), then emulate it in observational data, which removes many design biases ([Hernán 2016](https://doi.org/10.1093/aje/kwv254)). We have no patient-level data, so for now we can only pool published emulations, and design protocols for partners.
4. **Factorial Mendelian randomisation (exploratory).** Genetic proxies for two drug targets combined in a 2×2 genetic design test whether the effects add up ([Rees 2019](https://doi.org/10.1093/ije/dyz161)). Limit: lifelong small genetic effects, not drug doses; pleiotropy; aging outcomes are weak in biobanks (A28 R1 brief).
5. **Mechanistic non-overlap plus additive-risk model (hypothesis only, L0).** Score benefit as independent (multiplicative hazards: HR_combo ≈ HR_A × HR_B) and risk as the union of each drug's adverse-event profile from RCT safety data, penalising shared toxicities (e.g. two drugs that both cause weight and muscle loss). Mouse combination data (rapamycin + acarbose, R1 brief) can adjust the prior.

**Proposed rule.** A combination's rank = expected benefit under the multiplicative assumption, discounted by the credibility of the best available evidence for the interaction (1 → small discount, 5 → large), minus the summed absolute risk of serious adverse events. Every ranking is published as an L0 prediction with the trial that would refute it.

---

## 5. Proposed literature / evidence division

**Units** (span ≤7 under the division lead, A2):

| Unit | Role |
|---|---|
| **Question registry & scanning** | Owns the question registry (PICO, overlap check), scans new trials and preprints weekly, triggers updates of living reviews; includes "frontier watch" |
| **Extraction pool** | Workers who extract studies into schema v2, always in independent pairs |
| **Synthesis (by program)** | One synthesis lead per program (metabolic drugs, mTOR, lifestyle, senescence/inflammation): protocols, meta-analyses, GRADE |
| **Living reviews** | Keeps flagship analyses current (output 1 first), versioned |
| **Hype watch** | Sourced checks of popular claims (R1 brief §5.8) |
| **Methods liaison** | Shares the Handbook (one in-repo methods manual, Cochrane-style) with A28's method units |

**Mission lifecycle.**

```
question registered (PICO, overlap check in CI)
  → protocol written and committed before data (hash = pre-registration)
  → dual independent extraction into data/evidence (schema v2), adjudication
  → synthesis (code + pinned data snapshot, CI re-runs it: L1)
  → internal check: second synthesiser replicates blind (L2)
  → Verification audit (10 % re-extraction sample, error rate published)
  → merge to data/ + report; predictions to the ledger
```

**Hand-offs.** Questions needing causal genetics, re-analysis of patient-level or omics data, or modelling go to A28's method units with the registered PICO and the extracted evidence. Results needing a prediction go to the forecasting ledger (owned outside Production, §2.6). Verification receives every finished synthesis and the monthly extraction sample; it reports to A1, never to A2.

---

## References

DOIs verified via the Crossref API on 2026-10-19 (see `founding-council-r2.check.tsv`); official links checked the same day.

- Arc Institute. https://arcinstitute.org/
- ATLAS Authorship Committee (CERN, access-restricted). https://twiki.cern.ch/twiki/bin/view/AtlasProtected/AuthorShipCommittee
- Booth A et al. 2012. The nuts and bolts of PROSPERO: an international prospective register of systematic reviews. *Syst Rev*. https://doi.org/10.1186/2046-4053-1-2
- Boutron I et al. 2020. The COVID-NMA project: building an evidence ecosystem for the COVID-19 pandemic. *Ann Intern Med*. https://doi.org/10.7326/M20-5261
- Buscemi N et al. 2006. Single data extraction generated more errors than double data extraction in systematic reviews. *J Clin Epidemiol*. https://doi.org/10.1016/j.jclinepi.2005.11.010
- Campbell Collaboration. https://www.campbellcollaboration.org/
- Cochrane Handbook for Systematic Reviews of Interventions. https://training.cochrane.org/handbook
- Collins FS, Morgan M, Patrinos A. 2003. The Human Genome Project: lessons from large-scale biology. *Science*. https://doi.org/10.1126/science.1084564
- Elliott JH et al. 2014. Living systematic reviews: an emerging opportunity to narrow the evidence-practice gap. *PLoS Med*. https://doi.org/10.1371/journal.pmed.1001603
- Elliott JH et al. 2017. Living systematic review: 1. Introduction—the why, what, when, and how. *J Clin Epidemiol*. https://doi.org/10.1016/j.jclinepi.2017.08.010
- Gems D, de Magalhães JP. 2021. The hoverfly and the wasp: a critique of the hallmarks of aging as a paradigm. *Ageing Res Rev*. https://doi.org/10.1016/j.arr.2021.101407
- GLP-1 RAs, SGLT2 inhibitors and noncardiovascular mortality in type 2 diabetes: insights from a meta-analysis. 2024. *Diabetes Metab Syndr*. https://www.sciencedirect.com/science/article/abs/pii/S1871402124000043
- Guyatt GH et al. 2008. GRADE: an emerging consensus on rating quality of evidence and strength of recommendations. *BMJ*. https://doi.org/10.1136/bmj.39489.470347.AD
- Halfaker A et al. 2012. The rise and decline of an open collaboration system: how Wikipedia's reaction to popularity is causing its decline. *Am Behav Sci*. https://doi.org/10.1177/0002764212469365
- Hernán MA, Robins JM. 2016. Using big data to emulate a target trial when a randomized trial is not available. *Am J Epidemiol*. https://doi.org/10.1093/aje/kwv254
- Ioannidis JPA. 2016. The mass production of redundant, misleading, and conflicted systematic reviews and meta-analyses. *Milbank Q*. https://doi.org/10.1111/1468-0009.12210
- Klein JR, Roodman A. 2005. Blind analysis in nuclear and particle physics. *Annu Rev Nucl Part Sci*. https://doi.org/10.1146/annurev.nucl.55.090704.151521
- Ko A et al. 2026. Risk for cancer with glucagon-like peptide-1 receptor agonists and dual agonists: a systematic review and meta-analysis. *Ann Intern Med*. https://doi.org/10.7326/ANNALS-25-02237
- Linux kernel documentation: Submitting patches. https://www.kernel.org/doc/html/latest/process/submitting-patches.html
- Mathes T, Klaßen P, Pieper D. 2017. Frequency of data extraction errors and methods to increase data extraction quality: a methodological review. *BMC Med Res Methodol*. https://doi.org/10.1186/s12874-017-0431-4
- MedDRA. https://www.meddra.org/
- Nadon NL et al. 2008. Design of aging intervention studies: the NIA Interventions Testing Program. *Age*. https://doi.org/10.1007/s11357-008-9048-1
- Neuen BL et al. 2024. Cardiovascular, kidney, and safety outcomes with GLP-1 receptor agonists alone and in combination with SGLT2 inhibitors in type 2 diabetes: a systematic review and meta-analysis. *Circulation*. https://doi.org/10.1161/CIRCULATIONAHA.124.071689
- NHGRI. Human Genome Project data sharing (Bermuda principles). https://www.genome.gov/about-genomics/policy-issues/Human-Genome-Project-Data-Sharing
- Qiu M et al. 2021. Safety of four SGLT2 inhibitors in three chronic diseases: a meta-analysis of large randomized trials of SGLT2 inhibitors. *Diab Vasc Dis Res*. https://doi.org/10.1177/14791641211011016
- Rees JMB et al. 2019. Factorial Mendelian randomization: using genetic variants to assess interactions. *Int J Epidemiol*. https://doi.org/10.1093/ije/dyz161
- Shojania KG et al. 2007. How quickly do systematic reviews go out of date? A survival analysis. *Ann Intern Med*. https://doi.org/10.7326/0003-4819-147-4-200708210-00179
- Silverii GA et al. 2025. GLP-1 receptor agonists and the risk for cancer: a meta-analysis of randomized controlled trials. *Diabetes Obes Metab*. https://doi.org/10.1111/dom.16489
- Song K et al. 2025. Comparative effects of SGLT2 inhibitors and incretin-based therapies on dementia risk in type 2 diabetes: a systematic review and meta-analysis. *Front Endocrinol*. https://doi.org/10.3389/fendo.2025.1695075
- Sterne JAC et al. 2016. ROBINS-I: a tool for assessing risk of bias in non-randomised studies of interventions. *BMJ*. https://doi.org/10.1136/bmj.i4919
- Sterne JAC et al. 2019. RoB 2: a revised tool for assessing risk of bias in randomised trials. *BMJ*. https://doi.org/10.1136/bmj.l4898
- Wali et al. 2026. Reassessing cancer risk with GLP-1 receptor agonists: a comprehensive meta-analysis of gastrointestinal malignancies. *Front Pharmacol*. https://doi.org/10.3389/fphar.2026.1736380
- Wang et al. 2024. The relationship between use of SGLT2is and incidence of respiratory and infectious diseases and site-specific fractures: a meta-analysis based on 32 large RCTs. https://doi.org/10.1007/s00228-024-03631-7
- Williamson PR et al. 2017. The COMET Handbook: version 1.0. *Trials*. https://doi.org/10.1186/s13063-017-1978-4
- Yang K et al. 2026. GLP-1 receptor agonists and noncardiometabolic outcomes: an umbrella review of meta-analyses. *JAMA Netw Open*. https://doi.org/10.1001/jamanetworkopen.2026.4722
- Zinman B et al. 2015. Empagliflozin, cardiovascular outcomes, and mortality in type 2 diabetes. *N Engl J Med*. https://doi.org/10.1056/NEJMoa1504720
