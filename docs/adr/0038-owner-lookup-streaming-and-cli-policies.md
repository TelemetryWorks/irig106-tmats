---
status: accepted
date: 2026-11-28
decision-makers: Joey
---

# Owner lookup, streaming and CLI policies

Owner direction, verbatim (selected items from the 2026-11-28 acceptance of
all outstanding recommendations):

- Q4 Pattern syntax: `*` matches exactly one whole name part or index (e.g. `R-*\TK1-*`). No partial wildcards or regex in 0.1.
- Q5 Lookup scope: exact literal lookup works for ANY name (vendor/unknown included, returning raw text). Wildcard patterns work only over the reviewed registry.
- Q9 The tmats host does zero per-packet allocation in steady state for non-setup packets (same rule as core L2-HOT-008).
- Q14 File order is first-fragment order. Takeover timing follows closure (INT-028 e).
- Q15 Flex reference: the original bbaggerman/irig106lib, pinned to one exact commit. Divergent revisions or forks are reported as known differences.
- Q16 Correct the L1-CH10-005 wording to match the archived editions (106-11 introduced setup on channel 0 only; 106-13/15: channel 0 is Format 1 only; 106-17+: Format 1 or 4).
- Q17 The ASCII/XML mixture finding comes only from Complete records. Non-Complete FRMT is kept as qualified evidence.
- Q18 `--strict` refuses on the listed bad-data kinds and that bad-data class, not on informational notes.
- Q22 Reader progress: retained core pool credits are capped so the reader can always progress. A run exceeding the cap fails with a typed "assembly budget exhausted" error and is reported; the reader continues.
- Q23 Stdin input only with an explicit `-` operand. Never an implicit fallback.
- Q24 Drop Criterion. Benchmarks use core's style (informational ignored tests in CI, no new dependency).

INT sign-offs are recorded under ADR-0033 in the interpretation register.
Q6/std is recorded separately in the #59 amendment to ADR-0015. This record
refines ADR-0036's retained-span resource policy and ADR-0037's strict scope;
raw extraction remains separate from semantic/governing access. INT-034
padding-only/leading-damage precedence and report formats remain open.
