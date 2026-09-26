---
status: accepted
date: 2026-09-26
decision-makers: Joey
---

# The registry holds source text and reviewed, executable interpretations

## Context and Problem Statement

ADR-0004 made the registry hold Chapter 9's seven usage-attribute fields per
attribute, with citations. The team design review (priority T1,
`docs/research/2026-09-26-team-design-review.md`) showed that is not enough,
and the archived 106-24R1 text confirms it:

- Some facts live only in the definition prose. `C-d\DPNO` (Table 9-11):
  "Specify how many times the trigger measurand must occur before the
  calculation is done. Default is 1." — there is no `Default:` field. The
  chapter has 107 `Default:` fields and further prose-only defaults.
- Conditions are written for people, not machines. `C-d\DPNO` is "Allowed
  when: C\DCT is "DER"" — `C\DCT` carries no occurrence index, so which `C`
  group it means must be interpreted. Other conditions depend on linked
  groups, on attributes that may be missing or invalid, or on defaulted
  values; the table does not say how.

## Decision Drivers

* Every executable rule traceable to the exact text it interprets
* Interpretations reviewed by people, and re-reviewed when the text changes
* No hidden assumptions about occurrences, links, missing values, or defaults

## Considered Options

* **Two layers per entry — the source text as printed, and a reviewed
  interpretation — with condition semantics defined once**
* Usage fields only, interpreted in code (ADR-0004 as written)
* Hand-written rules per attribute

## Decision Outcome

Chosen option: **two layers per entry.**

- **Source layer:** the table row exactly as printed — parameter name,
  code-name pattern, usage-attribute text, definition prose — with its
  citation, produced by the table extractor and never edited by hand.
- **Interpretation layer:** Allowed-when and Required-when as expressions in
  a small, defined condition language; the default and its origin (field or
  prose); typed ranges; links. Each interpretation records its author, an
  independent reviewer, the review date, notes wherever it is not literal,
  and a hash of the source text it interprets.
- **Two-person rule** (owner decision, 2026-09-26): author and reviewer are
  two different people; an interpretation drafted with tooling or an AI
  assistant still needs both, and the tool counts as neither.
- **Condition semantics, defined once:** each condition declares its
  occurrence scope (same occurrence, linked occurrence, or document-wide) and
  whether it follows a link; a missing or invalid operand makes the condition
  **cannot evaluate**, reported as its own finding and never silently true or
  false; each condition declares whether it sees defaulted values.
- **CI checks:** completeness (every table code name and prose default is in
  the registry or excluded with a reason); coverage (every source row is
  interpreted or marked "not yet interpreted"); freshness (a changed source
  hash flags its interpretation for re-review); the two-person rule.

Both layers are compiled into the checked-in static tables of ADR-0005. The
user overlay supplies entries of the same shape.

### Consequences

* Good: a reader can check any rule against the words it came from.
* Good: prose-only facts such as defaults are captured and proven captured.
* Good: a new edition's changed rows are found automatically.
* Bad: every interpretation needs two people; registry work is slower by
  design.
* Refines ADR-0004 (registry content) and ADR-0005 (the generation pipeline
  gains an interpretation and review step). Diagram:
  `docs/diagrams/registry-pipeline.svg`.
