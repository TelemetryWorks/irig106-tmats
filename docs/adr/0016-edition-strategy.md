---
status: accepted
date: 2026-09-25
decision-makers: Joey
---

# Parse every edition; baseline 106-24R1; validate 106-04 through 106-24R1

## Context and Problem Statement

Real recordings span every IRIG 106 edition since Chapter 10 appeared:
106-04, 05, 07, 09, 11, 13, 15, 17, 19, 20, 22, 23, 24, and 24R1 (January
2025, current). TMATS itself is older (106-00 already has a Chapter 9). The
owner asked "is there a way to support multiple versions in the code?" and
"which should we realistically support and can we document the deltas".

The code-name syntax (§9.4.2) has not changed; attribute definitions have
(106-22 added the Q group via TG-171 and a CRC attribute via TG-173; 106-23
made "various corrections and enhancements" via TG-189; 106-24 changed only
the CRC parameter in Table 9-6; 106-24R1 did not change Chapter 9).

## Decision Drivers

* Every real file must parse
* Validation must use the rules of the file's own edition
* One registry, not one copy per edition

## Considered Options

* **Edition-independent parsing; one registry whose entries carry
  introduced/changed/removed editions; baseline 106-24R1**
* Support only the current edition
* Separate registries per edition

## Decision Outcome

Chosen option: **parse every edition; validate 106-04 through 106-24R1 from
one edition-tagged registry with 106-24R1 as the baseline**. The edition used
for validation comes from, in order: a caller override, `G\106` ("last 2
digits of the year", Table 9-2), then the Chapter 10 version in the
setup-record CSDW; the report says which source was used and whether they
disagree. Unknown or future editions are reported, not guessed. Pre-2004
files are parsed and validated against the oldest known edition with a
warning. Edition deltas are extracted from the archived Chapter 9 tables and
published as a generated `VERSION-DELTAS.md` (release 0.4).

### Consequences

* Good: a single code path for syntax; editions are data.
* Good: deltas are documented from the source, not from memory.
* Bad: transcribing and diffing fourteen editions' tables is large work,
  spread over releases 0.2–0.4.
