# Ledger criteria template (D67 prep)

Draft by A54 for Verification (A50). Council refs: D1 (prediction ledger), D62 (rolling audit), R2b.

**All examples below are FICTIONAL. They use the placeholder id `NCT00000000` and invented compounds. They are not forecasts and must never be scored.**

## Rules the template enforces

1. **Written before the answer exists.** A criterion is sealed (hashed and committed) before the outcome is public. Sealing is Verification's job, not covered here.
2. **Checkable by a machine.** Every criterion names one public record, one field in it, a comparator and a threshold. No judgement call at resolution time.
3. **Anchored to the registration.** A trial-outcome criterion (T1) points to the outcome measure the sponsor *registered* on ClinicalTrials.gov (title, timepoint, analysis population), with a hash of the registry snapshot taken at sealing. If the sponsor later changes the outcome, we still score against the sealed one.
4. **Fallback is mandatory.** Every criterion says what happens if the record is not available by the deadline. An unresolved criterion is never dropped from the ledger; it is scored by its fallback rule.
5. **Add-only.** A sealed criterion is never edited. Changes are new amendment entries that link to the original (for example the retraction exit, D61). Old versions stay readable.

## Template

```yaml
# One file per criterion. Fields marked REQUIRED must be filled before sealing.
id: LC-YYYY-NNNN                 # REQUIRED. Ledger criterion id, never reused.
type: T1                         # REQUIRED. T1 trial outcome | T2 process | T5 ITP paper
                                 # (T3/T4 to be defined by Verification.)
question: ""                     # REQUIRED. One plain-English sentence.
source:                          # REQUIRED. Where the answer will be read.
  registry: clinicaltrials.gov   # clinicaltrials.gov | crossref | pubmed | other
  record_id: NCT00000000         # Registry id, DOI or PMID.
  url: ""                        # Human-readable link.
field_path: ""                   # REQUIRED. Exact field to read in the record
                                 # (API path or a named table/row in the paper).
comparator: ">="                 # REQUIRED. One of: <, <=, ==, >=, >, exists, not_exists
threshold: null                  # REQUIRED unless comparator is exists/not_exists.
                                 # Number or ISO date only.
unit: ""                         # REQUIRED with threshold, e.g. "hazard ratio", "%", "date".
deadline: YYYY-MM-DD             # REQUIRED. Last date the record is checked (UTC).
fallback:                        # REQUIRED. Never empty.
  if_unresolved_by_deadline: ""  # e.g. "score as NO" or "score as UNRESOLVED (counts as miss)".
  if_record_withdrawn: ""        # e.g. "score as UNRESOLVED; keep entry".
registered_outcome:              # REQUIRED for T1, omit otherwise.
  title: ""                      # = protocolSection.outcomesModule.primaryOutcomes[].measure, word for word.
  timepoint: ""                  # = protocolSection.outcomesModule.primaryOutcomes[].timeFrame, word for word.
  analysis_population: ""        # Not a registry field before results. Copy from the
                                 # registered protocol/SAP and name the document;
                                 # compare later with resultsSection...outcomeMeasures[].populationDescription.
  analysis_population_source: "" # e.g. "Protocol v3, section 9.2" or "not registered".
  registry_version_date: YYYY-MM-DD
  snapshot_sha256: "<filled at sealing>"
approved_by: ""                  # REQUIRED. Verification agent id (A50 or delegate).
sealed_at: "<filled at sealing>" # Commit hash of the sealing commit.
amends: null                     # Id of the criterion this one amends, if any.
amendment_reason: null           # e.g. "retraction exit (D61)". Free text, one line.
```

## Example T1: trial outcome (FICTIONAL)

```yaml
id: LC-0000-0001
type: T1
question: >
  FICTIONAL EXAMPLE. Will the registered primary outcome of trial NCT00000000
  show a hazard ratio below 0.90 for the drug arm?
source:
  registry: clinicaltrials.gov
  record_id: NCT00000000
  url: https://clinicaltrials.gov/study/NCT00000000
field_path: resultsSection.outcomeMeasuresModule.outcomeMeasures[type=PRIMARY].analyses[paramType="Hazard Ratio (HR)", groupIds=<drug, placebo>].paramValue  # checked vs API v2 metadata 2026-10-19; select by paramType + groupIds, never by index
comparator: "<"
threshold: 0.90
unit: hazard ratio, drug vs placebo
deadline: 2099-12-31
fallback:
  if_unresolved_by_deadline: score as NO
  if_record_withdrawn: score as UNRESOLVED; keep entry
registered_outcome:
  title: "Time to first occurrence of a fictional composite event"
  timepoint: "Up to 60 months"
  analysis_population: "Intention-to-treat, all randomised participants"
  analysis_population_source: "Fictional protocol v1, section 9"
  registry_version_date: 2099-01-01
  snapshot_sha256: "<filled at sealing>"
approved_by: A50
sealed_at: "<filled at sealing>"
amends: null
amendment_reason: null
```

## Example T2: process, results posted by a date (FICTIONAL)

```yaml
id: LC-0000-0002
type: T2
question: >
  FICTIONAL EXAMPLE. Will trial NCT00000000 have results posted on
  ClinicalTrials.gov by 2099-06-30?
source:
  registry: clinicaltrials.gov
  record_id: NCT00000000
  url: https://clinicaltrials.gov/study/NCT00000000
field_path: protocolSection.statusModule.resultsFirstPostDateStruct.date  # checked vs API v2 metadata 2026-10-19; count only if resultsFirstPostDateStruct.type == ACTUAL
comparator: "<="
threshold: 2099-06-30
unit: date
deadline: 2099-07-07   # one week of slack for registry lag
fallback:
  if_unresolved_by_deadline: score as NO (no results posted counts as NO)
  if_record_withdrawn: score as NO; keep entry
registered_outcome: null   # not T1
approved_by: A50
sealed_at: "<filled at sealing>"
amends: null
amendment_reason: null
```

## Example T5: ITP paper (FICTIONAL)

The NIA Interventions Testing Program (ITP) publishes lifespan results in mice by compound and sex.

```yaml
id: LC-0000-0003
type: T5
question: >
  FICTIONAL EXAMPLE. Will the first ITP paper reporting compound "Fictinib"
  show an increase in median lifespan of at least 10% in male mice, pooled across sites?
source:
  registry: crossref
  record_id: "<DOI of the first ITP paper reporting Fictinib, recorded when published>"
  url: ""
field_path: "Paper, main lifespan table: row 'Fictinib, males, pooled sites', column '% change in median lifespan'"
comparator: ">="
threshold: 10
unit: "% change in median lifespan vs control"
deadline: 2099-12-31
fallback:
  if_unresolved_by_deadline: score as UNRESOLVED (counts as miss)
  if_record_withdrawn: score as UNRESOLVED; keep entry; if the paper is retracted after resolution, add an amendment entry (retraction exit, D61)
registered_outcome: null   # not T1
approved_by: A50
sealed_at: "<filled at sealing>"
amends: null
amendment_reason: null
```

## Example amendment: retraction exit (FICTIONAL)

```yaml
id: LC-0000-0004
type: T5
question: >
  FICTIONAL EXAMPLE. Amendment of LC-0000-0003 after the source paper was retracted.
source:
  registry: crossref
  record_id: "<retraction notice DOI>"
  url: ""
field_path: "update-to (Crossref retraction record)"
comparator: exists
threshold: null
deadline: 2099-12-31
fallback:
  if_unresolved_by_deadline: keep LC-0000-0003 score unchanged
  if_record_withdrawn: keep LC-0000-0003 score unchanged
registered_outcome: null
approved_by: A50
sealed_at: "<filled at sealing>"
amends: LC-0000-0003
amendment_reason: retraction exit (D61)
```

## Open points for Verification

1. ClinicalTrials.gov API v2 paths checked on 2026-10-19 against https://clinicaltrials.gov/api/v2/studies/metadata. The registry has no analysis-population field before results, so T1 takes it from the registered protocol/SAP.
2. `threshold` and `unit` are separate fields (A50 ruling).
3. T5 has no registry record before publication; the DOI is filled at publication, which is outside the "sealed before" rule. Proposal: seal the question and table/row spec, record the DOI later as an add-only amendment.
4. Base rate per criterion (D70) is not a field yet; add `base_rate` and `reference_class` if the council adopts D70.
