---
status: proposed
date: 2026-09-25
decision-makers: Joey
---

# Correctness guard rails in the code and the tests

## Context and Problem Statement

The prototype's defects survived because its tests checked the code against
itself (a counter property that could not fail, a lenient-mode test that
accepted both outcomes) and nothing exercised arbitrary input. Proposed in
`docs/ARCHITECTURE.md` section 2, points 11–12; awaiting the owner's review of
the architecture.

## Decision Drivers

* Never panic on any input
* Index and span mistakes caught by the type system
* Tests that can fail

## Considered Options

* **Guard rails from the first commit:** `#![forbid(unsafe_code)]`; newtype
  identifiers (`ItemId`, `Span`); no panicking indexing; `#[non_exhaustive]`
  on public enums; an immutable `Send + Sync` document; fuzzing from the first
  commit; property tests (the scan never panics, a round trip is
  byte-identical, the index agrees with the item list); the D1–D7 regression
  tests
* Add them later, when the code stabilises

## Decision Outcome

Proposed option: **guard rails from the first commit.**

### Consequences

* Good: the classes of bug that sank the prototype are caught mechanically.
* Bad: some extra ceremony in early code (newtypes, fuzz harnesses).
