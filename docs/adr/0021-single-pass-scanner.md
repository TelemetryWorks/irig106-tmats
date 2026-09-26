---
status: proposed; scanner colon and blank rule added by ADR-0024
date: 2026-09-25
decision-makers: Joey
---

# A single-pass scanner that indexes, checksums, and reports as it reads

## Context and Problem Statement

The legacy tools copy the TMATS text, split it with `strtok` into fixed 2 KB
line buffers, upper-case a copy of every line, match code names against
hand-written per-group decoders, and walk linked lists to print; the checksum
re-reads the text on a separate path. The owner asked for an architecture
that is "better, faster, less error prone". Proposed in
`docs/ARCHITECTURE.md` section 2, points 1–4, 8–9, and 15; awaiting the
owner's review of the architecture.

## Decision Drivers

* Read the bytes once, copy nothing
* Constant-time lookups without linked lists
* Errors that never abort reading

## Considered Options

* **One scan (`memchr` for `:` and `;`) that records spans, folds each code
  name once into a compact key (group, occurrence, interned path, indices),
  builds the index, computes the `G\SHA` digest, and records diagnostics with
  locations — all in the same pass; values parsed only when a view asks**
* Tokenize, then parse, then index in separate passes

## Decision Outcome

Proposed option: **the single-pass scanner.** There is no separate "strict"
parser: strictness is a policy applied to the diagnostics. Performance is
measured with benchmarks on the sample recordings from day one, reported next
to `idmptmat` on the same files; the numeric budget is set after the first
measurement.

### Consequences

* Good: no copies, no second read for the checksum, no list-walking defects.
* Good: one code path for lenient and strict use.
* Bad: the scanner is the most intricate component and needs the heaviest
  fuzzing.
* Diagrams: `docs/diagrams/data-flow.svg`, `docs/diagrams/legacy-vs-new.svg`.
