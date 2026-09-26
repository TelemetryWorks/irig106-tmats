---
status: accepted
date: 2026-09-26
decision-makers: Joey
---

# Derived parameters are parsed, validated, and described here; evaluated in `irig106-decode`

## Context and Problem Statement

Chapter 9 Appendix 9-E defines derived parameters: measurements calculated
from telemetry measurements, constants, and other derived measurements,
defined only in the C group (`C-d\DCT` = `DER`). They come in a function style
(`C-d\DPAT` = `N`: an operator, function, or custom-algorithm name with ordered
inputs `C-d\DP-n` and constants `C-d\DPC-n`) and a formula style
(`C-d\DPAT` = `A`: an expression in `C-d\DPA`), with a trigger measurand
(`C-d\DPTM`) and a number of occurrences (`C-d\DPNO`, default 1 in prose).
The grammar "strictly speaking, does not match the C language" (§E.7): its
precedence (Table E-6, consistent with the appendix's Yacc declarations)
binds `& ^ |` tighter than `* / %` and `+ -` tighter than `<< >>`.

The architecture treated `C-d\DPA` as a plain string. The team design review
(priority T2) asked for parsing, validation, dependencies, and trigger
semantics, with evaluation left to `irig106-decode`. It also found that a
value may contain a colon (`A<B || B<<C ? D : E`, §E.6.b).

## Decision Drivers

* Report derived-parameter defects as TMATS defects (syntax, names, cycles)
* One implementation of an unusual grammar, shared by every consumer
* Keep the library free of data, timing, and I/O (ADR-0010)

## Considered Options

* **Parse, validate, and describe here; evaluate in `irig106-decode`**
* Parse, validate, and also evaluate here
* Leave Appendix 9-E out of scope (treat `C-d\DPA` as a string)

## Decision Outcome

Chosen option: **parse, validate, and describe here; evaluate in
`irig106-decode`** (owner decision, 2026-09-26, on the recommendation that
followed the team's).

- A hand-written parser implements Table E-6 exactly for formula style; a
  binder handles function style. The output is an interpretable description
  (expression tree or bound call) with spans into the `C-d\DPA` bytes, the
  trigger (explicit, or the single input when there is one input and no
  trigger, §E.9.c), and the occurrences as an effective value (ADR-0023).
- Validation reports syntax errors, arity of the functions Table E-9 lists,
  unknown function names as custom algorithms (a warning — Table E-9 lists
  "selected" functions and §E.9.d uses a custom `NEWALG`), style misuse,
  unresolved measurement names, and cycles.
- A derivation graph beside the link graph resolves each derived
  measurement's dependencies, including other derived measurements.
- Errata, read through the two-person interpretation review (ADR-0022): `==`
  is the equality operator; `= =`, as printed in Table E-3, is accepted with a
  warning.
- A reference evaluator exists **only in the tests**, to prove the
  precedence by computing values (for example `2 + 3 & 1` is
  `2 + (3 & 1)`); it never ships.
- Evaluation, engineering-unit conversion, and the interpretation of
  floating-point bit patterns (Appendix 9-D) are non-requirement NR-007.
- Scanner rule: the first colon of an attribute ends the code name; later
  colons belong to the value; blanks around the code name are ignored for
  interpretation and kept in the bytes (the standard's own `C-6\DCN :DMC;`,
  §E.9.c).

### Consequences

* Good: consumers get one correct reading of the grammar and its dependency
  order without re-parsing.
* Good: the library stays pure; numeric policy lives with the evaluator.
* Bad: a parser to own and fuzz; custom algorithms can only be named, not
  checked.
* Refines ADR-0021 (scanner rule). Diagram:
  `docs/diagrams/derived-parameters.svg`.
