---
status: accepted
date: 2026-09-26
decision-makers: Joey
---

# Keep the TMATS edition and the recording-format version apart; label the validation basis

## Context and Problem Statement

ADR-0016 takes the validation edition from a caller override, then `G\106`,
then the setup record's Chapter 10 version, and reports "whether they
disagree"; it also validates pre-2004 files "against the oldest known
edition with a warning" while saying unknown editions are "reported, not
guessed". The team design review (priority T6) showed that these sources
answer different questions. Checked against the archived editions:

- `G\106` is the "Version of RCC IRIG 106 standard used to generate this
  TMATS file. The last 2 digits of the year should be used. Use a leading 0
  if necessary." "Range: 0 to 99" (Table 9-2, 106-24R1). `24` is 106-24 or
  106-24R1. The two-digit rule first appears in 106-17; 106-05 and 106-07
  give no format, and 106-13 only a maximum field size of 2.
- The setup-record CSDW's RCCVER says "which RCC release version applies and
  to which the following recorded data complies with"; its newest code in
  106-24R1 is still "0x0E = RCC 106-22". In 106-07 the byte was CH10VER
  ("the recorder requirements and following recorded data"); in 106-05 the
  whole word is "Reserved. (Bits 31-0)".

A TMATS file written to one edition may legitimately describe a recording
that complies with another, so a difference is not a conflict. And a check
against 106-04 of a file from 1996 is not validation against its edition.

## Decision Drivers

* Report what the file declares, exactly, without rewriting one declaration
  from the other
* Never present a check as something it is not
* Still give useful results when the edition is missing or unknown

## Considered Options

* **Two preserved declarations; a validation edition reported with its
  basis; a labelled fallback when `G\106` is missing or unrecognised;
  compatibility checks named as such**
* Keep ADR-0016's precedence chain and disagreement finding
* Validate nothing edition-specific without an override (option (b) of the
  owner's decision)

## Decision Outcome

Chosen option: **two declarations and a labelled basis, with option (a) for
the fallback** (owner decision, 2026-09-26).

- **TMATS edition declared**: `G\106`'s raw value and its reading — the
  candidate editions, or "unrecognised" (kept verbatim, never guessed).
- **Recording-format version declared**: per setup record, the raw version
  byte and its reading for its era — one edition, "106-22 or later"
  (`0x0E`), reserved, or "not declared" where the field did not exist.
- **No automatic conflict finding**: differing declarations are shown side
  by side; a user rule may flag them (ADR-0006).
- **Validation basis**: every validation report names the edition whose
  rules were applied and why — **override**, **declared** (`G\106`), or
  **fallback** with its reason. When `G\106` is missing or unrecognised and
  no override is given, the fallback is the edition read from RCCVER (for
  `0x0E`, the newest edition it covers), else the baseline 106-24R1, and
  validation continues.
- **Compatibility check**: for a declared edition the registry does not
  cover (before 106-04, or after the baseline), the library runs a
  compatibility check against a named edition (106-04, or the baseline) and
  reports it under that name, never as validation against the file's
  edition.
- The rest of ADR-0016 stands: parse every edition, one edition-tagged
  registry, baseline 106-24R1, generated edition deltas.

### Consequences

* Good: no false conflicts; every report says exactly which rules it
  applied and on what grounds.
* Good: files without a usable `G\106` are still checked, visibly as a
  fallback.
* Bad: reports carry more fields (two declarations, a basis); the JSON
  schema must hold them.
* Partly supersedes ADR-0016 (the precedence chain, the disagreement
  finding, and pre-2004 validation). Interpretations: INT-012, INT-014 to
  INT-017. Diagram: `docs/diagrams/edition-basis.svg`.
