---
status: accepted
date: 2026-09-25
decision-makers: Joey
---

# Reference tools are oracles, not authorities; real data stays local

## Context and Problem Statement

irig106.org's `idmptmat` (its source is published in RCC 123) and
`irig106lib` are the closest things to a reference implementation, and
irig106.org publishes real vendor sample recordings. Reading their source
found seven defects (D1–D7 in `docs/TEST-DATA.md`), including an endless loop
on two or more data sources and a wrong checksum range. Real flight data is
usually restricted, and some downloads need an account.

The owner: "We want to test for the known but in idmptmat so that does not
happen to our cli tmats tool", and "We will have links to the data samples and
test oracles and they will be downloaded locally for test then we will
simulate the different examples for our CICD pipeline and fuzzing tests
instead of trying to manage the data set."

## Decision Drivers

* Tests assert the standard, not another tool's behaviour
* Known defects in other tools must never reach ours
* No managed data set; no restricted data in CI

## Considered Options

* **Semantic differential tests against the reference tools, run locally on
  downloaded data; every found defect becomes a named regression test built
  from a synthesized fixture; CI uses synthesized fixtures and fuzzing**
* Mirror the sample data in a repository and run it in CI
* Trust the reference tools' output as expected results

## Decision Outcome

Chosen option: **oracles, not authorities**. A disagreement with a reference
tool is settled by the standard. Real recordings (irig106.org vendor samples
first, then program files) are downloaded and used locally only; whatever a
real file reveals is reproduced as a synthesized fixture in the same change.

### Consequences

* Good: CI is fast, deterministic, and free of restricted data.
* Good: D1–D7 are guarded forever.
* Bad: real-data coverage depends on developers running the local suite.
