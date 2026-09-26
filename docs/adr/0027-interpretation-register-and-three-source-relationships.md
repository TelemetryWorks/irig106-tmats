---
status: accepted
date: 2026-09-26
decision-makers: Joey
---

# Keep an interpretation register, and generate relationships from three sources

## Context and Problem Statement

ADR-0022 gave every registry entry a reviewed interpretation, and ADR-0026
built link declarations from both the "Links to:" and "Links from:" fields.
The team design review (priority T5) asked that inconsistencies in the
standard be recorded rather than the tables assumed mechanically complete.
Checked against 106-24R1 Chapter 9:

- §9.5.1 b says "All valid paths are documented in "Links to:" and "Links
  from:" attributes", yet `R-x\CDLN-n` omits Q from "Links to:" while
  `Q-d\DLN` lists it under "Links from:", and the R group's sub-channel and
  network names (`R-x\ANM-n-m`, `UCNM`, `MCNM`, `ENAM`, `CBM`) have no
  "Links" field at all although §9.5.1 b (g, h) names them as tie sources.
  Two directions of the fields are not enough; the prose is a third source.
- The Q group's condition names the channel type `FBCIN`, which Table 9-4
  does not define (it defines `FBCHIN`).
- The H group has no table: "The only H group attributes defined in this
  standard are" `H\TA`, which "ties the H group to the G group", and
  `H\ST-n` (§9.5.12). The architecture wrongly said its ties would be added
  "when its tables are transcribed".

Interpretations were being recorded across the ROADMAP, the architecture,
ADRs, and `docs/TEST-DATA.md`, with no one place to review them and no
check that each is tested.

## Decision Drivers

* Every departure from a literal reading of the standard is visible,
  reviewed, and tested
* No relationship is lost because one source omits it
* The design phase gathers what T1–T5 found before any code

## Considered Options

* **A register document now (`docs/INTERPRETATIONS.md`), generated from the
  registry's interpretation files later; relationships from three sources,
  with a generator check**
* Wait for the registry and record interpretations only in its files
* Generate relationships from the "Links" fields alone

## Decision Outcome

Chosen option: **the register now, generated later** (owner decision,
2026-09-26).

- **The register** holds one entry per inconsistency, gap, or silence the
  design had to decide: a permanent `INT-NNN` identifier, the sources quoted
  verbatim, the behaviour, the reason, the design status (accepted or
  suspect), the two-person review (ADR-0022), the origin, and a focused
  test. Tests name their entry with `/// Interpretations: INT-NNN` above
  `#[test]`; `scripts/build-trace-matrix.py` lists each entry with its tests
  and reports entries without one. Code may rely only on interpretations
  listed there. Once interpretation files exist, the register is generated
  from them and CI checks it is current.
- **Relationships** are generated from "Links to:", "Links from:", and the
  ties of §9.5.1 b, taking reviewed definitions only. A relationship stated
  by fewer than all the sources that could state it needs a register entry;
  the registry generator fails without one.
- **The extractor never assumes completeness**: it also reports links naming
  undefined code names, conditions naming unknown attributes or keywords,
  and attributes defined only in prose.
- **The H group**: `H\TA` and `H\ST-n` are built in; `H\TA` links to `G\TA`
  (INT-006); every other H attribute is organisation-defined, supplied
  through the user overlay, and preserved when undefined — the pattern of
  ADR-0008.

### Consequences

* Good: one place to review every interpretation, with its evidence and its
  test; reviewers of a new edition see which ones its changes touch.
* Good: R → Q links, sub-channel ties, and the H → G tie are not lost.
* Bad: the register is hand-maintained until the registry exists; the
  trace-matrix report keeps it honest in the meantime.
* Extends ADR-0022 (the register indexes its interpretations), ADR-0026
  (a third source for link declarations), and ADR-0008 (the H group).
  Register: `docs/INTERPRETATIONS.md`.
