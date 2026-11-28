# Family-2 completion audit — 2026-11-28

## Verdict and snapshot

**The scoped structural decomposition and written-analysis inventory are now
complete, subject to this final PR's independent and owner review.** No missing
scoped primary L1-to-L2/L3 chain remains. This is a documentary closure
recommendation, not implementation acceptance, blanket analysis sign-off or
authorization to start family 3 in this PR. The exact-head review/merge and
accepted phase handoff remain before the mechanical ADR-0035 restructure.

Inspected main 7ac43ea after #99, plus this PR's seven Analysis links and
planned cases. Normative requirements are unchanged. Earlier audits remain
[dated history](2026-11-27-family2-unheld-queue-reassessment.md), not a current
list of missing children/owner holds. Q25/Q26, std, normalization, X/V, caller
access, pattern scope, host allocation, file order, reference choice, mixture,
stdin, extraction and retained-credit direction are recorded; do not re-ask
decided policy or equate a final rule check with executed implementation.

## Rechecked scope and documentary reach

Reused the earlier 50-parent release inventory, including minimum stamp
prerequisites under L1-WRT-007, and added the already-owner-approved 0.1 host
allocation requirement L1-PERF-002 (Q9), for 51 parents. This is reconciliation,
not scope expansion. Shared parents can contain later-release portions; the
counts below describe reachable contracts, not promotion of those parts to 0.1.

| Check | Inspected result |
|---|---|
| Scoped parents with no direct L2 or no reachable L3 | None |
| Scoped reachable children | 100 L2 / 330 L3 (299 T, 31 I) |
| Scoped L3 absent from named TEST-DATA table references | L3-IO-001 only: API/reachable-I/O Inspection, not a fabricated Rust fixture |
| All-repository denominator | 108 L2 + 352 L3 + 27 countable L1 leaves = 487; zero verified |
| Written analysis inventory after this PR | All 66 entries linked; zero unwritten; all 66 still lack executed interpretation test markers |

The register still explicitly marks owner/independent ANALYSIS review and
intensive tests pending. Preserve that distinction: prior PR APPROVE evidence
is available, but this audit does not silently clear every analysis gate.
INT-013 retains S1 SUSPECT; written source analysis is not real-data clearance.

| Scoped L1 | Current primary L2 children | Reachable L3 |
|---|---|---:|
| L1-READ-001 | L2-READ-001, L2-READ-002, L2-READ-007 | 5 |
| L1-READ-002 | L2-READ-010 | 4 |
| L1-READ-003 | L2-READ-004, L2-READ-006, L2-READ-009, L2-READ-012, L2-READ-013 | 15 |
| L1-READ-004 | L2-READ-005 | 1 |
| L1-READ-005 | L2-READ-011 | 3 |
| L1-READ-006 | L2-READ-003, L2-READ-008 | 5 |
| L1-READ-007 | L2-READ-014, L2-READ-015 | 6 |
| L1-CH10-001 | L2-CH10-001 | 2 |
| L1-CH10-002 | L2-CH10-002 | 4 |
| L1-CH10-004 | L2-CH10-004, L2-CH10-005, L2-CH10-006, L2-CH10-007 | 15 |
| L1-CH10-005 | L2-CH10-008, L2-CH10-009 | 6 |
| L1-CH10-006 | L2-CH10-003 | 2 |
| L1-EDN-001 | L2-EDN-003, L2-EDN-004 | 6 |
| L1-EDN-002 | L2-EDN-002 | 5 |
| L1-EDN-005 | L2-EDN-001 | 2 |
| L1-VIEW-001 | L2-VIEW-001, L2-VIEW-015 | 10 |
| L1-VIEW-004 | L2-VIEW-002, L2-VIEW-003, L2-VIEW-004, L2-VIEW-011 | 12 |
| L1-VIEW-005 | L2-VIEW-008, L2-VIEW-009 | 8 |
| L1-VIEW-006 | L2-VIEW-016, L2-VIEW-017 | 6 |
| L1-VIEW-007 | L2-VIEW-005, L2-VIEW-006, L2-VIEW-007, L2-VIEW-010, L2-VIEW-012, L2-VIEW-013 | 26 |
| L1-VIEW-008 | L2-VIEW-014 | 9 |
| L1-REG-006 | L2-REG-006, L2-REG-007 | 6 |
| L1-EXT-003 | L2-EXT-001, L2-EXT-002, L2-EXT-003, L2-EXT-004 | 16 |
| L1-WRT-001 | L2-WRT-001, L2-WRT-003 | 7 |
| L1-WRT-002 | L2-WRT-004, L2-WRT-005, L2-WRT-006 | 8 |
| L1-SUM-001 | L2-SUM-001 | 2 |
| L1-SUM-002 | L2-SUM-002 | 3 |
| L1-SUM-003 | L2-SUM-004, L2-SUM-005 | 6 |
| L1-SUM-004 | L2-SUM-003 | 5 |
| L1-IO-001 | L2-IO-001 | 1 |
| L1-ROB-001 | L2-ROB-001, L2-ROB-002, L2-ROB-003, L2-ROB-006, L2-ROB-007 | 12 |
| L1-ROB-002 | L2-ROB-004 | 3 |
| L1-ROB-003 | L2-ROB-005 | 4 |
| L1-CLI-001 | L2-CLI-005, L2-CLI-006 | 8 |
| L1-CLI-002 | L2-CLI-018 | 3 |
| L1-CLI-003 | L2-CLI-010, L2-CLI-011 | 7 |
| L1-CLI-004 | L2-CLI-008 | 7 |
| L1-CLI-005 | L2-CLI-012, L2-CLI-013 | 7 |
| L1-CLI-006 | L2-CLI-019, L2-CLI-020, L2-CLI-021 | 8 |
| L1-CLI-007 | L2-CLI-009 | 5 |
| L1-CLI-008 | L2-CLI-003, L2-CLI-004 | 10 |
| L1-CLI-009 | L2-CLI-001, L2-CLI-002, L2-CLI-007, L2-CLI-014, L2-CLI-015, L2-CLI-016, L2-CLI-017 | 23 |
| L1-PERF-001 | L2-PERF-001, L2-PERF-002 | 8 |
| L1-REL-001 | L2-REL-001 | 1 |
| L1-REL-002 | L2-REL-002 | 3 |
| L1-REL-003 | L2-REL-003 | 2 |
| L1-REL-004 | L2-REL-004 | 2 |
| L1-REL-005 | L2-REL-005 | 3 |
| L1-REL-006 | L2-REL-006 | 5 |
| L1-WRT-007 | L2-WRT-002 | 2 |
| L1-PERF-002 | L2-PERF-003 | 1 |

The formerly missing report/show parents CLI-002/006 have children after
#92; REL-002/003/004 have children after merged #59; bounded VIEW-006 has
its direct children. This closes those identified missing-child findings.
Parent links and fixture names alone do not prove every branch, source
binding, output shape or runtime outcome is correct.

## What remains, and which phase it blocks

| Remaining item | Actor and evidence | Boundary before family 3 or later code |
|---|---|---|
| Final seven-analysis PR and completion audit | Independent reviewer at exact head; owner reviews/merges and accepts documentary handoff | Remaining family-2 closure step. Do not mark this unreviewed draft accepted or begin restructure here |
| Analysis-readiness records | Owner and independent reviewer, with exact analysis/revision evidence; maintainer keeps source/test gates explicit | Before implementation relies on each INT, not a demand to implement every INT before layout work |
| Concrete CLI/query request tables | Maintainer drafts against selected spellings/defaults/scopes; review mode/format applicability, selectors, token/value/repetition/error rules without guessing unselected grammar | Before affected parser/command integration; can be prepared separately from mechanical layout. Q26 is decided, not an owner-format hold |
| Schema/finding catalogue/rendered contract | Maintainer plus schema review: concrete artifacts, version/type/code/location/unavailable/assumption/suppression facts and non-UTF-8 source representation | Before renderer/JSON implementation. ASCII-only JSON remains a coordinator reading pending owner confirmation; do not upgrade provenance or invent byte decoding |
| Strict bad-data membership | Maintainer drafts explicit mapping/fixtures; review any addition beyond the six listed kinds under L3-CLI-060 | Before claiming additional-member strict coverage; informational notes are not refusals and severity is not a classifier |
| Resource/accounting integration | Maintainer with actual core API/ownership evidence, reviewed finite charging/progress-reserve design, typed failure/recovery and reference lifetime accounting | Before retention/limit implementation and bounded claims; Q22 cap direction is already decided. No invented numeric cap/default or local second reader |
| Historical layout/vocabulary qualification | Existing source notes plus actual reviewed profile/registry bindings and independent fixtures; S1 real-data review remains local-only | Before corresponding semantic recognition/decoder claim. Source notes now exist; do not require a complete later-release edition-delta registry merely to move crates |
| Flex source/arithmetic/vectors | Maintainer qualifies the recorded bbaggerman pin in bounded synthetic GitHub CI, records outputs/hashes without copying reference source | Before flex implementation/compatibility acceptance, not workspace layout; no new dependency or moving reference |
| F7 identity/prior and independent takeover/source partitions | Separately reviewed evidence/interpretations if available | Until then use the approved indeterminate/unavailable outcomes; these are not permission to guess, nor mandatory identification before layout or the bounded time-view metadata subset |
| Exact consumer/platform/performance/coverage | GitHub CI on redesigned code, real dependency API checks, zero-allocation/borrowed-span evidence and independent semantic tests | Later component/release gates. Green prototype CI and source arithmetic are not that acceptance; not prerequisites to have all redesigned code already working before restructure |
| Published dependencies and release pipeline | Coordinator/core maintainer with owner go-ahead; tmats release evidence from published types/core 0.2 and org trusted-publishing binary variant | Before integration/release acceptance; this audit does not verify live publication state or perform cross-repository publishing |

These are explicit component readiness obligations, not new unbounded
family-2 requirement batches. Concrete request/schema/membership work still
needs reviewed definitions and may expose a real new policy question; flag
one if found. Do not silently choose behavior or advertise full product
readiness from documentary reach. No additional absent scoped primary chain
was identified by this audit.

## Family-3 recommendation

After review/merge of this final group and accepted family-2 closure, the
next PR can be the mechanical ADR-0035 virtual workspace/restructure:
crates/irig106-tmats and apps/irig106-tmats-cli, lockstep workspace version,
exact-version internal path dependency, W2 prototype retirement and Q24
Criterion removal. Carry W3's reusable CLI/thin-main boundary and default-on,
required std contract. Update path-sensitive trace/CI/package documentation;
keep the prototype history at prototype-0 and split review-sized work if needed.

No new reader, TMATS interpretation, renderer, public grammar or semantic
implementation should be smuggled into that PR. It need not wait for flex
vectors, publication, full coverage or all parser tests: those require later
code. Component-specific analysis/source/readiness gates continue to apply.
A separate documentation-only request/schema proposal can be drafted in
parallel after this review, but do not start another branch editing shared
indexes before its predecessor merges. No restructure/parser work is started
by this audit; the current roadmap's pre-handoff restriction is respected.

## Checks and limits

Executed locally: parsed actual Parent fields and named table references,
recounted scoped descendants/methods, checked all analysis links and exact
register-field diffs, regenerated/checked trace and checked whitespace.
No Rust compilation/tests, API reachability inspection, published-package
compatibility, real-recording check, reference run, memory benchmark or
release execution was performed. GitHub CI checks this exact PR separately;
its success still does not execute the planned replacement-code fixtures.
