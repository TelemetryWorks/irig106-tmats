---
status: accepted
date: 2026-09-25
decision-makers: Joey
---

# No automatic repair: suggested edits, applied only by the caller

## Context and Problem Statement

The prototype's `repair()` changed documents by default. For example, when
`R-1\N:8` declared eight channels but six were defined, it rewrote the counter
to 6 — destroying the best evidence that the file was truncated. It also
inserted counters without reporting them. The owner asked whether an option
to repair should exist at all, and whether it would be a feature in the new
design.

## Decision Drivers

* Flight-test metadata is evidence; the library must not change it silently
* Users still need help fixing files
* Byte-faithful output (ADR-0002)

## Considered Options

* **Findings carry suggested edits; a general edit API applies the ones the
  caller chooses**
* Automatic repair with opt-out (the prototype)
* No fix help at all

## Decision Outcome

Chosen option: **suggested edits with explicit apply**. There is no `repair()`
module: where a fix is unambiguous, a finding carries a suggested edit; edits
are a patch list over the original bytes, applied only by an explicit call.
Counter mismatches are always reported, never corrected automatically. A
stale `G\SHA` after an edit is reported with a suggested stamp edit
(ADR-0014).

### Consequences

* Good: nothing changes without a decision by the caller; undo and diff fall
  out of the patch list.
* Good: the same edit API serves editing, generation, and fixes.
* Bad: fixing a file takes two steps (review, then apply). A CLI flag to apply
  all suggestions can come later on the same pieces.
* Diagram: `docs/diagrams/edits-and-checksum.svg`.
