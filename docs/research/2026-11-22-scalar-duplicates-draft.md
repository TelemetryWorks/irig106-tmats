# Scalar duplicates and exact lookup draft — 2026-11-22

Baseline analysis for family 2 batch 5 and proposed INT-039. Not a completed
cross-edition analysis, owner sign-off, independent AI result or executable
test report. INT-035/036/037/038 remain approved unchanged.

## Sources inspected and governing decisions

Baseline: RCC 106-24R1 Chapter 9, January 2025.
[Official PDF](https://www.trmc.osd.mil/wiki/download/attachments/335389015/Chapter9.pdf?api=v2)
([archive](https://github.com/TelemetryWorks/rcc-106-standards/releases/download/106-24R1/Chapter9.pdf)).

Section 9.4.2, page 9-2, describes unique code names, single-entry data,
numbered multiple entries and arbitrary source order. It gives G\DSI-1/2/3
examples. Table 9-2 PROGRAM NAME G\PN, page 9-10, supplies a simple scalar
example. These passages motivate identifying a slot by its complete name,
not by the attribute pattern with every index erased. They do not define
which repeated value wins or whether equal repeats count as one value.

Section 9.4.2, page 9-3, permits interjected COMMENT items. Its purpose
motivates a collection treatment; the text does not explicitly settle
repeated COMMENT cardinality. That treatment is therefore proposed for owner
review, not claimed as a direct standard mandate. Private H/V/X attributes
cannot all be assumed scalar from the shape of an unfamiliar name.

ADR-0002/0003 retain bytes, order and item identities. ADR-0023 defines the
five effective-value states. ADR-0026 requires all ambiguous link candidates
and declares key namespaces; this batch does not implement that link graph.
ADR-0034 F1 independently makes duplicate SHA integrity indeterminate and
refuses stamping. INT-038 already requires canonical decimal-string identity,
per-token information and canonical-key collision information with repeats.

## Proposed INT-039 and alternatives

Recommend a conservative singular-value rule: two or more occurrences of a
reviewed scalar slot remain ambiguous even if bytes or interpreted values
agree. A duplicate finding locates every item. Do not filter invalid values
to make a remaining value appear unique. Do not default over multiplicity.
Raw exact lookup remains a candidate collection, not a semantic verdict.

Alternatives are first/last-wins, accepting identical repeats as one value,
or selecting the only valid candidate. First/last-wins depends on source
order. Coalescing equal values hides that multiple declarations were present
and requires attribute-specific agreement logic. Valid-only selection hides
a damaged declaration. Conservative ambiguity costs convenience: benign equal
repeats block singular use until an explicit input correction. That cost
requires owner sign-off; no repair API is selected or implemented here.

Recommend treating standalone COMMENT as a retained ordered collection,
without a duplicate-scalar finding merely because several comments occur.
Alternative: apply unique-name diagnostics to COMMENT too. That is simpler,
but reads an intended comment-insertion facility as one scalar declaration.
This proposal makes no such exemption for every descriptive value or every
unknown attribute; reviewed definition cardinality remains the boundary.

Unknown/unreviewed cardinality stays visibly unclassified for scalar-duplicate
conformance, with all raw repeats available. This is not a claim that the
input is valid. A known multiple-entry pattern has a separate slot for each
full index tuple, and can still repeat one of those slots. Identity ignores
only the already-approved case/control projection and index spelling rules.
Raw values, validity or enumeration equivalence do not establish identity.

## Composition with existing uncertainty rules

A scalar duplicate can have only one raw index-spelling tuple: duplicate
findings still apply although INT-038's collision information does not.
Conversely, unknown cardinality cannot suppress an applicable INT-038
collision finding. Both findings preserve every occurrence when applicable.

Within one document/record, raw candidate order follows original item order.
Different documents are never silently joined. Marker gating and malformed
name rules remain in force: no usable key is invented from invalid name
bytes. A value that is invalid still has a raw candidate if its name has a
usable key. Empty raw value and absent item remain different observations.

Generic scalar Ambiguous is not a positive checksum result, nor permission
to change the coordinator's F1 decision. No packet identity, complete-record
proof, link selection, time association, default or caller assumption is
inferred by counting matching attributes.

## Planned verification and remaining work

TEST-DATA batch 5 covers exact repeat and conflicting values, mixed valid/
invalid candidates, empty versus absent, distinct and repeated indexed slots,
case/zero-padding grouping, COMMENT collections, unknown cardinality, source
order permutations, invalid names and independent normalization findings.
Expected candidate IDs/spans must come from fixed fixture positions, not
from the parser or index under test. Compare lookup results with a bounded,
independently enumerated item list; preserve source identity on every case.

These are planned checks, not executed tests. Full cross-edition/handbook/
sample analysis, definition binding and intensive verification remain gates.
Severity/profile mapping, pattern-query grammar, complete effective-value
resolution, registry/links, counter scopes and later family-2 areas remain
outstanding before family 3's restructure; partial parent rollups prove none
of those complete. No prototype result supplies evidence for this redesign.
