# PR #45 independent review: retained-cut blocking and QM transition coverage

Date: 2026-11-25. Source: independent reviewer @thread:thr_qsvwmtnn8f,
relayed directly in this repository thread; exact reviewed PR #45 head
`db1dbf7c645930f52849bee742963dcc25f68244`.

## Review result

The reviewer reported CONCUR on INT-046, INT-047, INT-049, INT-050, INT-051
and INT-052, and CONCUR on INT-053 wording. INT-048 has narrow CONCERNS;
PR #45 remains CHANGES REQUESTED and must not be merged on that basis.
The unchanged entries' wording checks do not approve the amended PR or
satisfy development analysis, source freshness or executable tests.

The reviewer independently reproduced 139,968 members / 39,072 distinct,
consistent classes, all transition counts, the same R0 predicate, and no
identity upgrade of duplicate-SHA ambiguity. CI and drift checks were green
at that reviewed head. This is the reviewer's reported evidence, not new
runtime validation by this maintainer.

## Required retained-cut clarification

> Add an amendment note to INT-045 and INT-034 pointing to INT-048: Incomplete, or Ambiguous with a retained cut facet. State in INT-048 (and the matrix note) that the cut facet keeps Incomplete's full blocking force, so no ambiguous-acceptance option (including INT-031's labelled split, if approved) can make cut text usable or governing.

This clarification is added to INT-034/045/048 and the proposed INT-031
acceptance option, requirements and planned fixtures. It preserves the
owner's strict intent; it introduces no new owner question or matrix
classification. A compound Ambiguous label cannot launder a cut into usable
metadata. `cut_facet_blocks_ambiguous_acceptance` is planned, not executed.

## Required and optional QM fixture follow-ups

> Add a named fixture (e.g. qm_unresolved_then_valid_takeover_resolves_target): C1's takeover is unresolved, C2 has independently established valid takeover, and C3 must target C2. Keep C1's findings and the unresolved history. Add it to TEST-DATA and the L3-REG-016 fixture list.

The named required fixture is added to INT-053, the sequence contract,
TEST-DATA and L3-REG-016. Unresolved is not a permanent state when later
independently established valid takeover supplies a new governing identity;
a merely good, equal or Complete copy remains insufficient.

The reviewer also suggested unresolved → definite non-takeover coverage and
clarifying the two uses of D. Both are applied:
`qm_unresolved_persists_through_non_takeover` keeps the later target unresolved;
in the sequence table D means semantic CONTENT difference, not duplicate
SHA status. All eleven sequence fixtures stay within the existing bound of
three later copies. They are planned tests, not executed implementation.

## Artifact scope and remaining gate

No pairwise generator or member/class artifact changes are needed. The
product remains 139,968 / 39,072 and the compressed pairwise artifacts are
byte-identical to the reviewed head. Trace regeneration records the review
statuses; it is not runtime coverage. Updated-head recheck is still required
for INT-048's amendment and the fixture follow-ups before implementation.
