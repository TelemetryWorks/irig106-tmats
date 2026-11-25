# Family 2 batch 18: scoped DSI context and numeric-fixture dependencies

Date: 2026-11-25. Requirements/PLANNED cases only, not implementation or
completed development analysis. This follows merged #49 at
`0b86124da24b77e11428d747ee1b408245d17c19` and the owner's follow-up about
required NSB presence and pending numeric field-value syntax.

## Scope and primary source

Derive DATA SOURCE ID context with L2-VIEW-010 and L3-VIEW-031–036 under
L1-VIEW-007's bounded time-view contract. Queries use a supplied canonical
recorder/entry scope; they need no parsed TK1 operand or channel-enumeration
policy. Reuse L1-VIEW-004 support and approved INT-035–039/041. No new
interpretation, stable Rust type or lookup-selection grammar is introduced.

Inspected [106-24R1 Chapter 9](https://www.trmc.osd.mil/wiki/download/attachments/335389015/Chapter9.pdf?api=v2),
section 9.5.4 Table 9-4 DATA SOURCE ID, printed p. 9-49, on 2026-11-25:
R-x\DSI-n is an entry-scoped identifier with its own usage/link fields. The
row records a character range, Allowed R\N>0, Required when Allowed, Links
from G\DSI-n and Links to M-x\ID. Those are not a time-channel reference.
RELEASE-0.1 and the #48 follow-up already require source context without
clock-reference semantics. General relationship resolution is separate work.

The Chapter 9 fingerprint is the one verified in batch 16:
`dfc5ebfb3e6f02f58389126863d43a45d89c7e2d0533a1c690d9537531c740cb`.
Source inspection is not runtime coverage or mechanical registry completeness.
This batch does not invent character-limit counting or blank/link-value
normalisation. Unsupported reviewed-definition prerequisites use INT-041's
unavailable contract while raw context remains inspectable.

## DSI context obligations

Bind the native R-x\DSI-n field by document/revision and canonical x,n,
never by a suggestive label, neighbouring value or prototype extras.
Recorder R-x\ID and G-group DSI entries stay distinct native fields. Keep
all matching raw occurrences, projection findings and effective states;
duplicates remain Ambiguous, with INT-048 assembly consequences. An absent
DSI acquires no default or borrowed recorder label.

A locally usable context label is not proof that a link resolved, a clock
was chosen, an NSB component names that source or a setup governs. Equal
labels do not merge entries; independent link/key findings remain visible.
Preserve byte identity, revision, source locations, supplied fragment mapping
and eligibility limitations. No plausible name rescues retained-cut input.

The eleven named planned cases cover native/canonical scope, duplicates,
absence/empty, strict character projection, unavailable definitions,
suggestive/equal labels, provenance and unresolved independent relationships.
They may inspect an entry directly even while its actual TK1 interpretation
is unavailable. Parsed-ID lookup and selection of enumerated entries remain
their separately gated operations; no CHE-dependent filtering is specified.

## #49 required-presence follow-up

Table 9-4 NUMBER OF SOURCE BITS, p. 9-33, has Allowed when R\ID is specified
and Required when Allowed. `nsb_absence_is_not_explicit_zero` now also asserts
a required-presence finding when that reviewed predicate is established,
alongside Missing/unsplit evidence and no invented explicit zero. This
presence/absence assertion needs no numeric-field conversion; any parsed
TK1 variant of its full-ID setup remains subject to the blocker below.
An unavailable condition is not silently turned true to force that finding.

## Pending owner decision: NSB/TK1 field-value syntax

The owner has not decided sign handling, leading zeros, blanks or a digit
limit for NUMERIC FIELD VALUES of NSB/TK1. Do not borrow INT-038's accepted
numeric CODE-NAME INDEX identity or a host language's integer parser.
No acceptance of '+', '-', zeros, whitespace or an arbitrary digit cap is
selected by this PR. Preserve the existing mathematical ranges separately.

TEST-DATA labels the four batch-16 TK1-dependent cases and fourteen batch-17
parsed numeric cases BLOCKED. This includes `nsb_bad_count_never_clips_or_wraps`,
TK1 identity/range cases and the parsed arms of vector/provenance fixtures.
Any other end-to-end case that parses those fields inherits the blocker even
if its standalone field assertion is not individually tagged. Known states,
canonical name keys and already numerical arithmetic operands can still be
specified independently; they are not passing parser/integration coverage.

The 917,490-pair future numerical oracle and fixed mathematical vectors are
unchanged. No exhaustive runtime check is executed here. Resolve the owner
choice, record its interpretation and obtain the ADR-0033 independent check
before implementing the dependent parser/integration expectations.

## Remaining boundaries and checks

CHE enumeration/filter handling remains pending exactly as recorded after
#48; no disabled-channel policy is asserted here. Association uncertainty,
caller assumptions, remaining view requirements and other family-2 work
continue separately. Full timelines/measurement views stay in 0.2.

Local checks cover requirement IDs/parents, planned case links, explicit
blocker annotations, the required-presence assertion, trace drift and
whitespace. No Rust build/test runs locally. Substantive checks use existing
GitHub CI; run IDs/results are recorded in the PR. Source inspection and
planned case names do not verify these requirements or complete development
analysis, source freshness or executable implementation gates.
