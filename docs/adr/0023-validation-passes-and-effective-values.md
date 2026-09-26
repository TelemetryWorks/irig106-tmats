---
status: accepted; passes 3 and 4 refined by 0026
date: 2026-09-26
decision-makers: Joey
---

# Validate in four passes over effective values

## Context and Problem Statement

ADR-0006 decided that validation is registry-driven, with severity policy and
user rules, and sketched the mechanism as "one generic pass per attribute"
plus hand-written cross-document rules. The team design review (priority T1)
showed the mechanism is insufficient: a pass that visits only the attributes
present cannot report one that is absent. `G\106` is "Required when: Always"
(Chapter 9 Table 9-2, 106-24R1); a file without it must be reported. Views and
validation also need to know whether a value was given, defaulted, missing,
invalid, or ambiguous — without the library inserting defaults into the
document (ADR-0002).

## Decision Drivers

* Report absences, not only problems with what is present
* One consistent notion of "the value of an attribute" for views and rules
* The stored bytes never change

## Considered Options

* **Four ordered passes, reading through an effective-value resolver and the
  condition evaluator of ADR-0022**
* One pass over present attributes plus ad-hoc absence rules (ADR-0006 as
  sketched)

## Decision Outcome

Chosen option: **four passes over effective values.**

1. **Present attributes** — each is known, allowed where it appears, and
   within its range and type.
2. **Presence** — every Required-when and R/R Ch 10 Status rule is evaluated
   for each occurrence it applies to; absent attributes are reported.
3. **Counters and indices** — each `\N` counter agrees with its entries, and
   indices run "with no missing values" (§9.5.1 a).
4. **Relationships** — links resolve and key values are unique.

Every finding names its pass. The link graph is built before the passes run.
Values are read through an **effective-value resolver** that returns one of
five states — **explicit** (value and location), **defaulted** (value and
citation), **missing**, **invalid** (raw text and reason), or **ambiguous**
(conflicting candidates) — and never writes to the document. Diagnostics gain
a **cannot evaluate** kind (ADR-0022). ADR-0006's decision — registry-driven
validation, severity policy, user rules, the Chapter 10 recorder profile, and
recommended-length findings as warnings — stands unchanged.

### Consequences

* Good: a missing required attribute is always reported; the first
  regression tests are "`G\106` absent is reported" and "`C-d\DPNO` absent is
  defaulted to 1".
* Good: views and validation agree on every value's state.
* Bad: the presence pass iterates rules per occurrence rather than only the
  items present; expected to be negligible at TMATS sizes and measured by the
  benchmarks (ADR-0021).
* Supersedes the mechanism sketched in ADR-0006. Diagram:
  `docs/diagrams/data-flow.svg`.
