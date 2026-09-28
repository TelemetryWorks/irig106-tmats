---
status: accepted
date: 2026-09-27
decision-makers: Joey
---

# RCCVER is read literally; later editions' reuse of a code is a labelled note; the fallback is labelled policy

## Context and Problem Statement

ADR-0025, ADR-0028, INT-012, and INT-017 read RCCVER `0x0E` as "106-22 or
later". The team's spec alignment review (2026-09-27) objected: "The table
explicitly assigns 0x0E to 106-22 … The absence of newer codes does not
establish an unlimited future compatibility guarantee."

The archived editions (Chapter 11, Figure 11-34 and its predecessors) show:

| Edition | Last code | Then |
|---------|-----------|------|
| 106-17 | `0x0C = RCC 106-17` | `0x0D` through `0xFF` reserved |
| 106-19, 106-20 | `0x0D = RCC 106-19` | `0x0E` through `0xFF` reserved |
| 106-22, 106-23, 106-24, 106-24R1 | `0x0E = RCC 106-22` | `0x0F` through `0xFF` reserved |

So a recorder built to 106-20 writes `0x0D`, and one built to 106-23,
106-24, or 106-24R1 writes `0x0E`. That is a fact about the archived
editions, not a promise about later ones.

## Decision Drivers

* Report what the standard says, in its words
* Keep an observation about the archive apart from an inference
* Keep the validation fallback of ADR-0028 working, and visibly a policy

## Considered Options

* **The literal reading, a labelled note, and a labelled policy**
* "106-22 or later" (the previous wording)
* The literal reading alone

## Decision Outcome

Chosen option: **the literal reading, a labelled note, and a labelled
policy** — the owner's decision, 2026-09-27 (asked "say '106-22' literally,
with 'unchanged through 106-24R1' as a note?"; answered "Yes").

- **Declared**: each code reads as its table says — `0x0E` "RCC 106-22",
  `0x0D` "RCC 106-19"; reserved codes as unknown.
- **Note** (observation, labelled): `0x0E` — "unchanged through 106-24R1:
  106-23, 106-24, and 106-24R1 assign no newer code"; `0x0D` — "106-20
  assigns no newer code".
- **Fallback** (ADR-0028 option a, when `G\106` is missing or unrecognised):
  for `0x0E`, the rules of 106-24R1, labelled "policy: the newest archived
  edition that still uses this code" — the same effect as before, now named
  as a policy. For `0x0D` ("RCC 106-19"), likewise the rules of 106-20, the
  newest archived edition that still uses that code (owner, 2026-09-27:
  "Yes, use 106-20").

### Consequences

* Good: the report no longer claims more than the standard says.
* Neutral: validation behaviour for `0x0E` is unchanged.
* Refines the edition-code wording of ADR-0025 and ADR-0028, INT-012,
  INT-017, L1-CH10-006, `docs/ARCHITECTURE.md` sections 6.5 and 9, and
  `docs/TEST-DATA.md`; the same wording goes to `irig106-time` (T-9, its
  contract section 3.9) and to `irig106-types` when the shared mapping is
  fixed.
