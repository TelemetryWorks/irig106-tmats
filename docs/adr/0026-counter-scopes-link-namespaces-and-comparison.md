---
status: accepted
date: 2026-09-26
decision-makers: Joey
---

# Counters declare their scope, links declare their namespace and cardinality, keys are unique per attribute, and comparison ignores case

## Context and Problem Statement

ADR-0023 made pass 3 check that indices run "with no missing values" and
pass 4 check that "links resolve and key values are unique" (Chapter 9
§9.5.1 a). The team design review (priority T4) showed those rules are too
coarse. Checked against 106-24R1 Chapter 9:

- "The values of "x" in "X-x" are not necessarily contiguous" (§9.5.14).
- 49 counters carry parent indices (for example `D-x\MNF\N-y-n`,
  `Q-d\NSF\N-i-n-m-o`) and count only within one parent combination.
- `P-d\DLN` "Links to: D-x\DLN, B-d\DLN", and `D-x\DLN` has "Links from:
  P-d\DLN". Both are keys ("Any attribute with a Links from: is a key and
  must be unique in the TMATS file"), yet they hold the same value by design.
- `B-x\DLN` is linked from both `R-x\CDLN` and `P-d\DLN`, so a recorder
  channel naming a PCM stream that carries bus data matches a P and a B
  group.
- `R-x\CDLN-n` lists "Links to: P-d\DLN, B-x\DLN, S-d\DLN", while `Q-d\DLN`
  lists "Links from: R-x\CDLN" — the two ends disagree.
- "For alphanumeric data items, including keywords, either upper or lower
  case is allowed; TMATS is not case sensitive" (§9.4.2); L1-READ-004 named
  only code names.

Also found: Appendix 9-C's own example ends 18 attributes with `:` instead
of `;` (page C-8; every edition since 106-17), for example
`D-1\MML\N-1-1:2: D-1\MNF\N-1-1-1:1: D-1\WP-1-1-1-1:14;`.

## Decision Drivers

* Report real problems without false conflicts on valid files
* Never pick one of several candidates silently (ADR-0002: lossless)
* Every rule traceable to the table row it interprets (ADR-0022)

## Considered Options

* **Declare counters and links in the interpretation layer; resolve links to
  resolved, unresolved, or ambiguous; keys unique per attribute; ASCII case
  folding for code names, keywords, and link values**
* Keep the blanket rules of ADR-0023 and special-case the known exceptions
  in code

## Decision Outcome

Chosen option: **declared counters and links** (owner decision, 2026-09-26).

- **Counters** declare the code pattern and index position they govern and
  their parent scope; contiguity from 1 to N is checked per parent-index
  combination unless the registry records a cited exception (the X group).
- **Links** declare their source, their target namespace (built from both
  the "Links to:" and "Links from:" fields; each disagreement settled in a
  reviewed interpretation), a selector where targets overlap (for
  `R-x\CDLN-n`, the channel data type `R-x\CDT-n` of the same channel), and
  their cardinality (exactly one, at most one, many). A link resolves to
  **resolved** (one candidate after the selector), **unresolved** (none), or
  **ambiguous** (several, all listed). An ambiguous link makes the values
  read through it ambiguous (ADR-0023).
- **Keys** are unique among the values of the same attribute, in the scope
  the registry declares; equal values across linked attributes are the link.
- **Comparison**: code names, keywords, and link values use ASCII case
  folding; stored bytes keep their spelling; blanks inside values are
  significant.
- **A colon where a semicolon is meant**: the scanner keeps the attribute
  exactly as read (the first colon ends the code name, ADR-0024) and, when a
  colon in a value is followed by a complete code name and its own colon,
  reports "possible `;` typed as `:`" as a warning with a suggested edit
  (ADR-0007). **Suspect** (owner, 2026-09-26): held in doubt until checked
  against real recordings (`docs/ROADMAP.md`, S1); the default severity and
  the trigger may change as a result.

### Consequences

* Good: valid files with noncontiguous X groups, nested counters, and linked
  P and D groups produce no false findings.
* Good: duplicate data-link names are reported where they matter — at every
  link that becomes ambiguous — and never resolved to the first match.
* Good: the standard's own Appendix 9-C example becomes a fixture that the
  reader handles without loss and with 18 precise suggestions.
* Bad: every counter and link needs a reviewed interpretation before pass 3
  or 4 can check it; uninterpreted ones are reported "not yet interpreted"
  (ADR-0022).
* Refines ADR-0022 (the interpretation layer gains counter and link
  declarations) and ADR-0023 (passes 3 and 4). Diagrams:
  `docs/diagrams/link-resolution.svg`, `docs/diagrams/link-graph.svg`.
