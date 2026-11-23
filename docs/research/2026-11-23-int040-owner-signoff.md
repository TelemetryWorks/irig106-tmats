# INT-040 owner sign-off — dated 2026-11-22, recorded 2026-11-23

Owner: Joey. Under ADR-0033, the owner approved INT-040 as written in
[PR #24](https://github.com/TelemetryWorks/irig106-tmats/pull/24) at final head
`176fdf4ebe0a69f83da1b12202a462274fe91d8e`, merged at
`4f74f9420f468f5d6aecafe1475399ec5a72cdec`. The sign-off is explicitly dated
2026-11-22. The owner relayed that the independent AI recheck **CONCURS**
with the clarifications; this relay is recorded on 2026-11-23. This PR/head
and owner relay are the AI-check reference, not a claim of a separately
posted GitHub review.

## Approved interpretation

Count distinct canonical identities of structurally identified entries even
when their payload is Invalid or their scalar slot is Ambiguous. Keep every
witness and independent finding. A matching structural count is not a valid
configuration. The final reviewed draft includes counter-own-range blocking,
independent child checks despite parent-domain/ancestor-counter problems,
canonical 0/00 parents, X attachment exclusion and WFP value-rule separation.
It also specifies exact diagnostic membership boundaries for damaged names.
No default, guessed parent identity, repaired counter or synthetic member is
introduced by approval.

## Compatible-scope tightening requested with sign-off

A scope whose readable parent tokens are compatible with unassigned member
evidence reports **partial / cannot-evaluate, NEVER pass**. This holds even
when its independently observed set already matches a usable counter.
For `D-1\WP-1-1-x-2`, every scope (1,1,1,m) is compatible: x=1, y=1 and n=1
are readable facts, but the location parent m is not known. Preserve those
partial sets and unassigned evidence; never pick m to recover a passing scope.

Compare only canonical parent facts mapped unambiguously by the reviewed
pattern. A contradictory readable parent position excludes a scope from
this item's block: for example (1,1,2,1) contradicts readable n=1. An unknown
position imposes no restriction. With wrong arity, never guess an alignment
to exclude a scope. Evaluate the guard against existing/evaluated scopes;
do not enumerate all possible values of unreadable parents. Other families
and unrelated findings keep their own rules, not a whole-document pass.

This tightening accompanies the approval. The independent concurrence above
applies to #24's final head; this record does not claim the added tightening
wording or this new PR has already received independent review.

## Remaining evidence

Approval records the interpretation decision, not implementation, completed
analysis or passing tests. Requirements remain drafted and TEST-DATA cases
remain planned. Full development analysis and intensive executable tests
remain gates. The earlier dated counter-membership draft/review follow-up is
preserved as history; its pending sign-off descriptions are superseded here.
Remaining family-2 requirements still precede family 3's ADR-0035 restructure.
