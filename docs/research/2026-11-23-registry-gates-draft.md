# Registry evidence gates and damaged-name matching — 2026-11-23

## Status and scope

Family 2 batch 9 derives source/review/development gate obligations from
existing decisions. It proposes no new standard interpretation. Requirements
and acceptance cases remain planned; no registry, extractor, matcher or gate
implementation is delivered here. INT-042 remains approved, with the owner's
new deterministic matching clarification recorded below.

General registry scope is unchanged. Full registry generation does not move
into 0.1; the bounded time view uses only its reviewed applicable subset.
Remaining family-2 derivations precede family 3's virtual-workspace restructure.

## Authorities inspected

- ADR-0004 and L1-REG-001 require edition-specific citations. ADR-0022
  requires both verbatim source and reviewed interpretation, plus visible
  completeness/coverage/freshness checks. Section 9.5.1 a of
  [106-24R1 Chapter 9](https://www.trmc.osd.mil/wiki/download/attachments/335389015/Chapter9.pdf?api=v2),
  pages 9-5–9-6, describes usage fields; definition prose remains part of
  the source layer. L2-REG-002 already preserves prose/default origins.
- ADR-0033 requires one owner sign-off and an independent AI check and
  preserves source hashes, written analysis and testing obligations. Its
  rule supersedes the historical two-human rule in ADR-0022.
- L1-REG-003 binds review and hash to source; L1-REG-006 and ADR-0027
  require registered interpretations. The register's development convention
  requires deep analysis and intensive tests beyond semantic approval.
- ADR-0005 requires checked-in generation with a drift check rather than
  downstream build-time generation. This batch derives refusal to activate
  unready interpretations; full generator design remains subsequent work.

No new hash algorithm, extraction schema, file layout or CI job is selected.
The hash input must have stable documented encoding and boundaries and cover
all cited source dependencies; a changed edition/reference is a dependency
change even if its text happens to match. Rehashing does not renew approval.

## Owner clarification: INT-042 diagnostic literal matching

Owner request, 2026-11-23, after approval/merge of #29: readable characters
must occur in order, and each high-byte run stands for zero or more unknown
bytes. L3-VAL-012 records this as a deterministic whole-literal compatibility
check, after INT-036 projection and ASCII case folding, without source repair.

Examples against ID: I followed by a high-byte run is compatible because the
run can cover D; I/run/D and ID/run are compatible even when the run covers
nothing. D/run/I is incompatible because readable order is wrong. An ordinary
terminated I or ID2 has no implicit gap; a proper unterminated EOF prefix
retains the earlier approved unknown-remainder guard. A high-byte run is
not restricted to replacing exactly its raw byte count.

This is diagnostic uncertainty only: `T-1\I\xFF:a;` cannot prove absence,
but never becomes an exact key or an Explicit value. INT-040 parent guards,
record boundaries, known exact-presence precedence and all previous INT-042
states remain. Implement using bounded matching states rather than guessed
replacement strings or exponential backtracking; no numeric resource budget
is chosen. Planned acceptance cases include positive and negative boundaries.

## Evidence and remaining work

New gate cases use synthetic source revisions and metadata, not fabricated
claims about actual edition changes, reviews or passing implementation tests.
They separate semantic approval, freshness, development analysis and required
verification; none may be silently promoted from a draft document/test name.
Current approved interpretations' incomplete analysis/tests remain visible.

Detailed extractor completeness, registry relationship reconciliation,
condition grammar, edition mapping, overlay policy, output determinism and
atomic generation remain separate derivations. Future implementation must
first complete applicable analysis and write independently sourced tests;
this record is a requirement-derivation note, not that completed analysis.
