# Resolution criteria approvals

Owner: Verification (A50); kept by A66. Charter part 5, T5-A50-2, D1, D72:
Verification approves every resolution criterion before it is sealed.

Drafts come from Research's ledger (M1, A28) in the format of
`verification/prep/ledger-criteria-template.md`.

## Checklist (all must pass; one failure = refuse)

1. **One public record**: a registered ClinicalTrials.gov outcome, a DOI, a
   dataset release with a version, or an explicit published quantity (table,
   row, column named). No "the paper reports".
2. **One field, read by a machine**: `field_path` points to one value. No
   judgement at resolution time.
3. **Comparator and threshold**: one of `<, <=, ==, >=, >, exists,
   not_exists`; threshold is a number or ISO date, with a unit.
4. **Deadline**: an ISO date (UTC).
5. **Never reported = scored**: `fallback.if_unresolved_by_deadline` and
   `fallback.if_record_withdrawn` are filled, and neither drops the item.
6. **T1 items**: registered outcome title and time frame copied word for word
   from the registry, analysis population from a named protocol/SAP, registry
   version date.
7. **Blindness** (charter part 5): a dated Literature public-trace search,
   at most 7 days old at sealing, is attached or referenced.
8. **Data source licence**: the resolution source has an approve row in
   `licences.md`.

## Decisions

| Criterion id | Draft ref (PR / commit) | Date | Checks failed | Verdict | Reason |
|---|---|---|---|---|---|

No drafts received yet.
