# Bounded lookup-state requirements: family 2 batch 40

Date: 2026-11-27. Docs-only direct decomposition, not implementation.

## Missing trace supplied without expanding the release

The coverage audit identified bounded L1-VIEW-006 with no direct L2 child.
L2-VIEW-016/017 and L3-VIEW-065–070 now supply that parent chain; existing
exact lookup, duplicate and effective-value children remain under their
original parents. This closes a documentary primary-link gap, not all field,
source, access, semantic or executable readiness gaps.

The 2026-11-21 RELEASE-0.1 owner contract already brings bounded time-view
attribute lookup states forward; general link resolution remains 0.2.
Accepted ADR-0026 supplies resolved/unresolved/ambiguous and no arbitrary
winner. Approved INT-036/038/039 and existing candidate contracts supply
projected full-key identity, retained occurrences and scalar multiplicity.
No new owner choice, standard interpretation or clock-proof source is chosen.

## Source/contract inspection

Inspected L1-VIEW-006, RELEASE-0.1, ADR-0026 and the candidate, effective-value,
time-field and association requirements. Baseline field scope is already
cited to 106-24R1 Chapter 9 section 9.5.4 Table 9-4:

- TK1 TRACK NUMBER/CHANNEL ID p. 9-48: actual value is not entry index n.
- NSB NUMBER OF SOURCE BITS p. 9-33: recorder-level attribute, no entry n.
- DSI/CHE p. 9-49, SHTF p. 9-51 and TTF/TFMT/TSRC pp. 9-71–72 provide
  the existing reviewed field definitions/context; no new values or defaults
  are transcribed by this slice.

Chapter 9 source fingerprint remains
`dfc5ebfb3e6f02f58389126863d43a45d89c7e2d0533a1c690d9537531c740cb`.
[Official PDF](https://www.trmc.osd.mil/wiki/download/attachments/335389015/Chapter9.pdf?api=v2).
Source scope/name rules are inspected alongside existing requirements;
cardinality is an owner-directed composition, not a new claim that these
rows declare a data-channel-to-time-channel link.

## Three separate questions

1. **Candidate identity/cardinality:** within an available exact scoped
   lookup, zero identified items is unresolved, one resolved, several
   ambiguous. All original identities/locations remain, including identical
   repeats, invalid values and INT-038 canonical-spelling collisions. Scope
   is document/revision, recorder, path and the definition's full index tuple,
   never actual channel-ID value, another recorder or a guessed source link.
2. **Effective field value:** one item can be identity-resolved while its
   field is Invalid/unavailable. TFMT with no explicit item can have zero
   lookup candidates while a separately established applicable default is
   Defaulted A with provenance. That never fabricates an explicit occurrence.
   Multiple candidates remain ambiguous even if only one is valid.
3. **Clock association:** lookup success is not association_evidenced. The
   owner contract deliberately provides no positive TMATS data-to-clock
   evidence rule in 0.1; TSRC/DSI/CHE, source order and a lone candidate do
   not change that. Caller choices remain separate labelled assumptions.

These cardinality words apply to this bounded attribute lookup, not a
successful generic TMATS link resolver. L3-VIEW-002's empty raw collection
still proves neither an unrelated link verdict nor physical absence. Damaged
compatible names and unterminated prefixes remain related evidence under
INT-042; a presence predicate may be cannot-evaluate despite zero exact
candidates. The suffix/damaged bytes never become a synthetic explicit item.
Unavailable scope/key or marker-blocked interpreted lookup must not be
represented as usable empty success. No new fourth enum or Rust shape is
selected to represent these existing availability limitations.

## Qualifications and test boundary

A lookup does not change source/revision, default bytes, findings, field states,
record Ambiguous/Incomplete/cut facets or core integrity evidence. Proposed
INT-031 still gates general qualified-record access. Tests may receive
candidate evidence from permitted diagnostic inspection; they do not approve
that access. ADR-0037's raw extraction permission is not typed semantic use.

INT-056 suppression concerns bad-data emission on an assumption answer only;
original source/field diagnostics stay readable. A caller choice or Quiet
level cannot alter exact candidate count, erase duplicates, create an explicit
field or carry the choice to another query implicitly.

Eighteen planned cases independently specify keys, item identities and source
order, cardinality, effective/default evidence and qualifications. They cover
zero/one/many, invalid/empty values, entry versus actual ID, recorder-level
NSB, case/index collisions, defaulted absence, unavailable lookup, damaged/
unterminated evidence and separate document/revision contexts. Condition/default
cases receive evaluated applicable evidence rather than inventing a new
numeric/blank/default rule. Independent fixtures are not calculated through
the production lookup or a fabricated successful clock resolver.

## Verification and remaining work

Local work: source/contract inspection, trace regeneration/drift, identifier/
parent, case-link, fingerprint/inventory and whitespace checks. No Rust builds/
tests, installs, reference/sample access, packaging or release actions run.
Heavy checks stay in GitHub CI; prototype passes cannot verify these new
contracts. All new requirements remain Draft without Evidence.

The dated coverage audit stays historical. Its missing direct VIEW-006 trace
is addressed by this PR, while the broader partial/owner/source gaps remain.
#59 remains held until the owner's std answer; then rebase, regenerate and
recount before re-review. Unheld normalization detail, already-decided
assembly/resource and show-free CLI contracts are next, with no silent choices
for their pending portions. Report formats, strict scope, Criterion retention,
flex reference, pattern, INT-061/062/063, session/order/access/governance and
other recorded owner holds remain. No family-3 or parser code starts here.
