---
status: accepted; three questions open as owner follow-ups F1–F3
date: 2026-09-26
decision-makers: Joey
---

# Edits are transactions over defined targets, and a stamp hashes the final bytes

## Context and Problem Statement

ADR-0007 made edits a patch list over the original bytes, applied only by
the caller; ADR-0014 computes `G\SHA` over the original bytes and suggests a
stamp edit when it is stale. The team design review (priority T7) asked that
the contracts be completed before the APIs are fixed. Checked against
106-24R1:

- "The entire contents of the TMATS file except the characters from
  "G\SHA:" to the following ";" (inclusive) shall be used to calculate the
  checksum" (Chapter 9 Table 9-2); "all text between the "G\SHA" and the
  following semicolon, inclusive, shall be discarded for the purposes of
  digest calculation" and the digest is "64 lower-case hexadecimal
  characters" (Chapter 6 §6.2.3.11 f). A line break written with an inserted
  item lies outside that range and is hashed.
- Neither text defines two `G\SHA` items or one with no following
  semicolon; `g\sha` is the same code name (§9.4.2).
- §9.5.14 says a renumbered original's extension "could be updated to
  preserve the link", and nothing about removing the original.
- Nothing in our design defined overlapping patches, stale item IDs, edits
  to duplicated attributes, or renumbering collisions; L1-WRT-006 required
  generated output to pass validation while leaving required input
  unspecified.

## Decision Drivers

* An edit either happens completely and correctly or not at all
* No edit is ever applied to a guessed target
* A stamped checksum verifies by construction

## Considered Options

* **Transactions — validate, apply atomically, rebuild, verify — over
  targets bound to a revision, with defined rejections; the stamp as the
  transaction's last step over the final bytes**
* Apply edits one at a time as they come (ADR-0007 as sketched)
* Compute the stamp before inserting the item

## Decision Outcome

Chosen option: **transactions with a final-bytes stamp** (owner direction,
2026-09-26: apply T7, leaving three questions open).

- **Targets**: an edit names an item by an ID bound to one document
  revision, or by code name. The whole set is rejected, with a finding
  naming the edits and items, for: two edits touching the same bytes or
  item; an ID from another revision (never re-targeted); a code name that
  matches several items unless the edit names one item or says "every
  occurrence"; renumbering onto an index in use unless the same set moves
  the occupant. Counters change only when the set says so; otherwise a
  counter edit is suggested.
- **Transaction**: validate the whole set (targets, the rules above, and
  well-formed new text — code names parse, values contain no `;`); apply it
  atomically as a new revision, leaving the original untouched and changing
  nothing on failure; rebuild the affected derived state (index, link graph,
  derivation graph, effective values); verify by re-reading the emitted
  bytes.
- **Stamp**: the last step of a transaction. Emit the final bytes with the
  `G\SHA` item in place, using the document's own separator and line ending;
  compute SHA-256 over every byte outside the item; write the value inside
  the item; verify.
- **`G\SHA` policy decided here**: `G\SHA` is the code name in any case,
  and the same characters inside a value are not (D6); an item with no
  following semicolon is an error, verification reports "malformed", and a
  terminator is suggested; upper-case hex that otherwise matches is a match
  with a warning. Each is a register entry (INT-018 to INT-020).
- **Generator**: with sufficient input the output validates for the
  selected edition; with insufficient input the result is an **incomplete
  draft** with missing-input findings, never presented as valid.
- **Open, not decided** (owner follow-ups, `docs/ROADMAP.md`): **F1** two or
  more `G\SHA` items; **F2** removing an attribute that X extensions point
  to; **F3** suggesting `:` where `=` was typed, as in Chapter 6's own
  examples. Until decided, the implementation must not choose a behaviour
  for them; they are register entries with the status "open" (INT-021 to
  INT-023).

### Consequences

* Good: no partial edits, no edits to guessed targets, and a stamp that
  verifies by construction — the team's newline case included.
* Good: the same transaction serves editing, fix suggestions, the
  generator, and `tmats stamp`.
* Bad: a verify pass after every transaction costs a re-read; TMATS is small
  and the benchmarks measure it.
* Refines ADR-0007 (edits) and ADR-0014 (stamping). Diagram:
  `docs/diagrams/edit-transaction.svg`.
