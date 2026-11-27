# X-extension association: family 2 batch 30

Date: 2026-11-27. Documentation-first derivation and proposed INT-061,
pending independent review/owner decision where stated. No runtime code.

## Source and accepted basis

Inspected 106-24R1 Chapter 9 section 9.5.14, printed p. 9-172, from the
[official Chapter 9 PDF](https://www.trmc.osd.mil/wiki/download/attachments/335389015/Chapter9.pdf?api=v2),
SHA-256 `dfc5ebfb3e6f02f58389126863d43a45d89c7e2d0533a1c690d9537531c740cb`.
The printed X form includes organization, ORIGCODE, extension code and indices.
ORIGCODE includes the original group and full original code path, potentially
several backslashes but no hyphens. X occurrence and the leading index tuple
must match the original; additional indices belong to the extension.

The worked example is D-1\MN-1-2:MEAS1; with
X-1\MYORG\D\MN\SGAIN-1-2:10.75;. This is a structural oracle from the
section, not a complete conforming TMATS file or a standard-defined sensor
conversion rule. Table 9-7 MEASUREMENT NAME p. 9-115 confirms the MN path
and two entry indices; NUMBER OF FRAGMENTS p. 9-117 supplies a nested-path
arity example. Table 9-2 PROGRAM NAME p. 9-10 prints unindexed G\PN.

ADR-0008 accepts first-class X attachment even with unknown extension meaning.
ADR-0026 supplies resolved/unresolved/ambiguous candidates without a winner.
Approved INT-038 explicitly uses canonical digit-string identity for X and
original indices; INT-039/048 retain duplicates and record ambiguity/cut
facets. INT-040 excludes X from creating D members and leaves private index
semantics unclassified. ASCII comparison comes from section 9.4.2 p. 9-2,
not from INT-036; the latter supplies name projection only.

## Bounded association contract

L2-EXT-001/002 and L3-EXT-001–008 derive the X portion of L1-EXT-003.
V grouping remains to derive. A partial child does not complete that parent
or cut the full 0.1 extension scope. Metadata used for claimed X support
must be independently sourced/reviewed; this is not an excuse to invent
private grammar or wait for the complete 0.2 registry.

For an unambiguous reviewed X decomposition and original layout, match the
complete original group/path, occurrence and ordered leading index tuple.
Original arity and component attachment come from reviewed name metadata;
do not guess from extension suffix length, a longest-prefix heuristic or a
plausible matching value. Keep extra extension indices distinct and opaque.

With prerequisites available, list every matching original source occurrence.
Zero means unresolved, one resolved to that item, several ambiguous. Do not
deduplicate identical originals or choose a first/last/best-checksum target.
A grammar/arity/encoding block means cannot-evaluate, not proof of no target.
Raw originals/extensions and their diagnostics remain retained in all cases.

Association is not validation or adoption. Keep record qualifications attached
and unchanged: a readable target cannot cure INT-048 Ambiguous or cut text.
Unknown extension values do not establish a clock reference or decode a
measurement conversion. Public pattern scope/syntax, incomplete/ambiguous
access and report-format decisions remain their own pending gates.

X occurrence values need not be contiguous, but that is not a blanket
exemption for original/carried/private nested indices. An orphan X creates
no original object, D member or counter entry. Namespace uniqueness and
private value/count rules still need their own reviewed definitions. No
renumber/edit operation or user-registry overlay is implemented by this
read-only association contract; association provenance must survive queries.

## Proposed INT-061: owner summary and options

The printed form has X-x and requires x to match an original index, while
ORIGCODE's group list includes G and G\PN has no numeric index at all.
The section's indexed D example does not supply a sentinel or omission rule
for that case. This is an explicit source gap, not proof that the standard
forbids every extension of an unindexed original.

**A — recommend:** keep the raw extension and available raw original evidence,
but leave the occurrence mapping unavailable/cannot-evaluate until a reviewed
representation exists. Do not invent x=0/1, read an index from a value or
claim either resolved or proven no-target. This protects consumers from a
false attachment, while callers can still inspect every byte.

**B:** define an explicit owner-chosen representation for unindexed targets,
with its own labelled policy, exact syntax, collision and compatibility
fixtures. This could enable them, but the cited section does not define that
convention. No sentinel or omitted-index form is selected in this PR.

INT-061 remains PROPOSED with A recommended. Owner sign-off and independent
AI check under ADR-0033 are required before implementing that case. Its
three planned Option-A fixtures are BLOCKED, not accepted conformance cases.
Indexed fields of groups without an occurrence and other ambiguous private
paths remain separate unsupported mappings until reviewed; this proposal
does not guess their treatment.

## Planned verification and gates

TEST-DATA supplies source-derived positive, mismatch, prefix-index, case/
normalization, unknown-value, orphan, duplicate, scope and blocked cases.
Original values are not match keys. Expected original IDs and spans are
listed independently of the resolver; round-trip preservation alone cannot
prove attachment. Whole-record qualifications and raw byte identity must
remain visible across every outcome.

A cross-fragment fixture states what makes ONE record: same channel,
sequence +1 and matching FRMT/RCCVER under ADR-0025/INT-011. Separate-record
fixtures never borrow a missing target. A leading encoding marker continues
to block semantic lookup under INT-037; matching cannot bypass it.

This PR only checks source citations/fingerprint, requirement links, trace
regeneration and whitespace locally. All new requirements are Draft and
fixtures planned, without executable markers. No Rust builds/tests, sample
access, benchmark, dependency install, cross-repository access or release
action occurs. Source freshness, development analysis and executable tests
remain before implementation. No existing owner hold is resolved, and #59
still needs its std answer plus rebase/recount before re-review.
