# INT-038 owner sign-off — 2026-11-22

Owner: Joey. Under ADR-0033, the owner approved INT-038 as amended in
[PR #20](https://github.com/TelemetryWorks/irig106-tmats/pull/20) at head
`29707da26fd43ec9a871d41bfd6213afdfa3f965`, merged at
`3cc3658aef954b2d3d8aa29dcae8b09ff72c3794`. The owner relayed that the
independent AI recheck **CONCURS**. This PR/head and the 2026-11-22 owner
relay are the AI-check reference, not a claim of a separately posted GitHub
review.

## Approved interpretation

Numeric identity uses the canonical decimal digit string, without fixed-width
or big-integer conversion for equality/hashing. Preserve every raw spelling;
report noncanonical-spelling and collision information findings now. Read
digits from the approved INT-036 projection, with its omission findings and
blank handling unchanged. Use the same numeric identity for X/original index
comparisons under 106-24R1 Chapter 9 section 9.5.14, page 9-172; index equality
alone does not resolve an association. Zero's structural readability is not
range approval. Scoped counter requirements/checks over distinct numeric
identities remain explicitly deferred to the counter batch.

## Collision-group clarification requested with sign-off

L3-READ-029 fires when a canonical full-key group has two or more DISTINCT
raw index-spelling tuples. List EVERY occurrence in that group in input
order, exact repeats included. Distinctness is over corresponding index
tokens' original-source spellings; code-letter case or value differences
alone do not create a distinct index-spelling tuple. A group with one raw
tuple repeated does not trigger this normalization-collision finding, while
per-token noncanonical-spelling findings remain applicable.

For example, raw names A, A, B, A whose indices spell 1, 1, 01, 1 all share
a canonical key: the group finding lists all four occurrences, not just A
and B representatives. Repeating only spelling 01 does not meet the distinct-
spelling trigger, but each 01 token still gets its own information finding.
General duplicate classification remains later; nothing is merged or selected.

This nit accompanies the owner's approval. The AI concurrence above is at
#20's exact head, not a claim that the new clarification wording or this
sign-off PR has already received independent review.

## Remaining evidence

Approval records the interpretation decision, not implementation, completed
analysis or passing executable tests. L2/L3 requirements remain drafted;
TEST-DATA cases remain planned. Full cross-edition/handbook/sample analysis
and intensive tests remain development gates. Earlier dated draft/review
records are preserved as history; their pending sign-off descriptions are
superseded by this record. Remaining family-2 batches still precede family
3's ADR-0035 restructure.
