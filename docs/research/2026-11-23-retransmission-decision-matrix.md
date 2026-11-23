# Exhaustive retransmission decision matrix: owner direction, not approval

Date: 2026-11-23. **DRAFT. No new interpretation or governing policy is
approved.** Joey agrees with a whole-byte-identical following copy keeping an
otherwise Complete seal Complete, labelled `identical retransmission`.
An SRCC=0 claim is checked: differing content is reported and treated as a
change. This table records that direction and conservative PROPOSED answers
for every remaining combination, with owner questions explicitly attached.
INT-045's signed rule remains unchanged until the new interpretation review.

## Sources and scope

- [106-24R1 Ch11](https://www.trmc.osd.mil/wiki/download/attachments/335389015/chapter11.pdf?api=v2)
  §11.2.7.2 pp. 11-50–51: fragmentation, SRCC change/unchanged meaning,
  changed-setup ordering, same-session ASCII/XML prohibition and minimum required attributes. §11.2.1.1 f p. 11-8:
  per-channel modulo-256 sequence, not a record delimiter.
- [106-24R1 Ch6](https://www.trmc.osd.mil/wiki/download/attachments/335389015/chapter6.pdf?api=v2)
  §6.2.3.11 f pp. 6-22–23 and Ch9 Table 9-2 p. 9-13: G\SHA byte domain.
  Ch9 §9.4.1–2 p. 9-2: characters/framing; raw equality is not normalization.
- INT-011 joins independently; INT-018 recognises projected G\SHA names but
  excludes RAW bytes, never text inside values. INT-019 blocks undefined/
  truncated authoritative domains. INT-020 allows upper-case hex; INT-043
  allows canonical-numeric algorithm 2 with noncanonical warnings. INT-021/F1
  makes duplicate G\SHA indeterminate and refuses stamping. No table row
  discards those findings, chooses a checksum duplicate or silently stamps.
- INT-034's remaining precise completeness precedence and INT-028/029/031/032
  governance are proposed. INT-039 scalar-value Ambiguous is not automatically
  the same thing as an assembly Ambiguous result; QD below makes this explicit.

The finite product is **139,968 members**: 3 × 6 × 36 × 3 × 4 × 3 × 3 × 2.
It covers the owner's eight dimensions, separating FRMT and RCCVER changes
and adding an explicit unavailable/indeterminate checksum status rather than
misnaming unknown algorithms/BOM/damage as malformed or verified.

Scope: a sealed ASCII candidate in the 0.1 design; following FRMT-different
means XML and is reported unsupported. This is not an XML-completeness parser.
Raw comparison can survive unsupported interpretation; semantic claims need
independent fixture evidence, not a baseline parser borrowed across formats.
Unmodelled absolute editions, resource failure, damaged-name recognition and
clock validity are not silently solved by this finite product. They remain
separate mandatory guard fixtures and source prerequisites.

## Explicit dimension members

| Dimension/code position | Every permitted member and meaning |
| --- | --- |
| srcc | `0`, `1`, `U` = following FIRST-fragment SRCC unreadable/untrusted |
| content | `B` = entire following TMATS text byte-identical; `H` = only G\SHA value bytes differ; `W` = only raw layout/whitespace differs; `D` = independently established semantic difference; `R` = other raw difference (e.g. case/order change or semantic comparison unavailable); `X` = following incomplete/cut, final comparison unavailable |
| sha_pair | First letter SEALED, second FOLLOWING: each is `N` absent, `V` verifies that candidate's fixed text domain, `F` mismatches, `M` malformed, `D` duplicate/F1, `U` unavailable/indeterminate for another reason |
| continuity | `C` = same channel, next candidate sequence +1 modulo 256; `G` = same channel, gap/other sequence; `O` = different channel |
| frmt_rccver | `SS` both same; `SD` same FRMT/different RCCVER; `DS` different FRMT/same RCCVER; `DD` both different |
| sealed_state | `T` = semicolon-ended supplied candidate, before the overlays below; `X` = cut mid-attribute, already Incomplete; `A` = supplied duplicate/split ambiguity, with INT-039 field evidence retained |
| separator | `T` time seal; `S` explicitly observed same-channel stop; `E` actual EOF of that supplied input |
| phase | `I` initial; `M` mid-session (the SEALED time-closed candidate was eligible SRCC=1; following SRCC is independent) |

All 36 checksum pairs are explicitly included:
`NN NV NF NM ND NU VN VV VF VM VD VU FN FV FF FM FD FU MN MV MF MM MD MU DN DV DF DM DD DU UN UV UF UM UD UU`.
They derive presence `none`, `following_only`, `sealed_only`, or `both` in
every expanded member. Present status is never inferred from presence alone.

Content classes are exclusive by precedence: X is selected whenever the
following candidate is cut; otherwise choose B, then H, then W, then D, then R.
B/H/W/D/R require a bounded, framed following extent
but do not assert semantic validity. H excludes changing item multiplicity;
W records a layout-only DELTA; both candidates retain all original bytes,
including padding or layout inside/outside the checksum exclusion. Layout
changes may affect interpreted intentional blanks: W does not prove semantic
equivalence. D therefore means other independently established semantic change. B compares TMATS text, not CSDW/packet headers. It never
means a matching prefix, normalized fields, multiset or checksum equality.
R closes the residual raw-difference class: it does not pretend that raw
case/order changes are semantic differences. Comparing reordered items or
unknown attributes requires a separate reviewed semantic-equivalence rule.

EOF is a real source boundary: the following candidate is a CROSS-INPUT
comparison, not bytes secretly arriving after EOF in the same input. No
same-session linkage or join across EOF is invented (QE). An explicit stop
similarly remains a stop even if a later sequence coincidentally compares +1.

## Every member, class, outcome and planned test is listed

Checked-in [members.jsonl.gz](retransmission-matrix/members.jsonl.gz) has one
record for EVERY member: all inputs, checksum presence, full sealed outcome
and basis, following outcome, findings, governing effect, owner questions,
equivalence-class ID and a unique NAMED planned test. Nothing is sampled.
The [class index](retransmission-matrix/classes.json.gz) collapses only records
with IDENTICAL complete output columns (excluding member/test identity).
Each class explicitly lists EVERY member ID in `member_ids`, with count and
full outputs. The member file also repeats the class ID for each complete row;
no membership is hidden behind a wildcard. [manifest.json](retransmission-matrix/manifest.json)
records every domain, exact counts and artifact SHA-256 fingerprints.

The stdlib [documentation expander](retransmission-matrix/spec.py) generates
these review artifacts, not library code or a runtime classifier. It parses
no TMATS and supplies no implementation test result. Read a chosen member or
list EVERY member of one class, without decompressing anything into the repo:

```bash
python3 docs/research/retransmission-matrix/spec.py --check
python3 docs/research/retransmission-matrix/spec.py --expand > /tmp/tmats-matrix.jsonl
python3 docs/research/retransmission-matrix/spec.py --class-id CLASS_ID
```

Member IDs list the eight code positions separated by dots. Its explicit
planned-test name is `retransmission_matrix_` plus those codes with underscores.
The compressed file contains the ACTUAL name for each member, not only this
formula. Class IDs hash the full proposed output; the generator checks collisions.

## Decision columns: compose these disjoint rules and overlays

The following tables explain the complete expanded outputs. R0 dominates;
otherwise R1, R2, R3 and R4 apply in that order, with ALL findings/questions
unioned. This is a DRAFT specification for review, not newly approved behavior.

| Rule | Exact members/condition | SEALED outcome and basis |
| --- | --- | --- |
| R0 | B with unequal checksum statuses; B with sealed X; H with absent checksum or unequal duplicate multiplicity; sealed X with sealed V; following X with following V | Explicit inconsistent-observation member. Recommend Incomplete if already cut, otherwise Ambiguous/hold; no fabricated usable following candidate. Q0, not a new parser verdict |
| R1X | sealed X, no R0 | Incomplete, original separator basis plus pre-existing Incomplete; no identity/digest rescue |
| R1A | sealed A, no R0 | Ambiguous, original separator basis plus pre-existing Ambiguous; never choose a scalar duplicate |
| R1T | sealed T with no subsequent applicable overlay | Complete on separator evidence; NOT semantic validity, integrity or governing authority |
| R2D | sealed T and sealed checksum D, no R0 | PROPOSED Ambiguous/hold, QD: F1 itself already makes checksum indeterminate, but assembly versus field ambiguity must be reviewed |
| R3B | sealed T; checksum not D; separator T; continuity C; versions SS; content B; no R0 | Owner-direction proposal: Complete, base time basis PLUS `identical retransmission`; QI limits need sign-off |
| R3V | same R3 boundary conditions; content H/W/D/R/X; sealed V; no R0 | Existing INT-045 Complete exception on sealed-text verification evidence only; retain both readings/finding |
| R3A | same R3 boundary conditions; content H/W/D/R/X; sealed N/F/M/U; no R0 | Existing INT-045 final Ambiguous with located `possible premature time-boundary seal` and both readings |

R1 separator bases: T/I = `initial time seal (INT-045)`; T/M =
`time-boundary seal (owner policy)`; S = `same-channel stop`; E =
`end of input (labelled)`. R3B adds, rather than hides, the separator basis.
A bounded following extent must be established independently (with source
provenance); matching a prefix cannot establish its own end. In particular,
SRCC=0 does not gain a time-closure rule through this comparison.
The proposed identity exception is intentionally restricted to OTHERWISE
Complete, bounded same-channel +1/same-FRMT/RCCVER time-seal cases. Broader
uses are not silently authorised; QB/QI ask the owner about them.

| R4 FOLLOWING column | Exact members (subject to R0) | Proposed outcome |
| --- | --- | --- |
| Cut comparison | content X | `continuation_unresolved_cut` when T/C/SS, otherwise `incomplete_unresolved_candidate`; partial bytes never prove identity |
| Same metadata | versions SS; content B/H/W/D/R | respectively identical retransmission / change SHA-value-only / change raw-layout-only / semantic change / other raw change, semantics unsettled |
| Different metadata | versions SD/DS/DD; content B/H/W/D/R | metadata change with the explicit raw-content subtype; QB, not presumed semantic equivalence |
| Different channel overlay | continuity O | retain independent-channel/unlinked label; no cross-channel governing winner |
| Different FRMT overlay | versions DS/DD | following XML unsupported; check same-session ASCII/XML-mixing violation using preserved provenance; do not claim a typed comparison or usable new description |
| F1 following overlay | following checksum D | duplicate-checksum indeterminate; retain all occurrences and stamping refusal |

For qualifying T/C/SS nonidentical copies, the following outcome also retains
`continuation_reading_retained_no_rejoin`; a change subtype is comparison
evidence, never a decision to dismiss the possible-continuation reading.

All H/W/D/R differences with SRCC=0 produce `unchanged_claim_differs_report_change`.
Metadata changes with SRCC=0 are reported separately. SRCC=1 plus B/SS reports
`change_claim_but_identical_text`, NOT an invented semantic change. U never
becomes SRCC=0. The byte-identity direction is checked, never inferred from
an unchanged bit or a verifying checksum. H/W/R are reported as RAW change
subtypes without silently approving INT-029's configuration equivalence.

Findings include ALL inherited source findings. Each present F/M/D/U checksum
adds its side-specific mismatch/malformed/F1/unavailable finding; D refuses
stamping and never verifies. Keep control-projection, noncanonical-name,
uppercase-hex acceptance and INT-043 noncanonical-designator warnings exactly
as their reviewed rules require. The table never inserts/fixes G\SHA.

Governing column: Complete means only `eligible_for_review_no_automatic_switch`;
Incomplete/Ambiguous block NEW adoption, with no silently selected old fallback.
Incomplete following, unsupported FRMT or unreadable SRCC add an explicit
following-usability limitation. QG remains for every coherent combination:
INT-028/031 adoption and old/new packet applicability are NOT approved by a
classification table. Inconsistent evidence instead blocks pending correction.

## Short owner questions: every affected member is flagged in the artifact

- **Q0:** Can inconsistent observations be rejected/held rather than fabricated
  as valid byte fixtures? Recommend yes, with named consistency/rejection tests.
  They remain listed/counted, never silently skipped or counted as parser coverage.
- **QD:** Does duplicate G\SHA or other field ambiguity force assembly Ambiguous,
  or only checksum/effective-value ambiguity? Recommend hold the ambiguity and
  never use equality/digest to upgrade pre-existing ambiguity; settle the layers.
- **QI:** Is the new exception limited to bounded otherwise-Complete T/C/SS
  candidates, independent of following SRCC 0/1/U? Recommend that limit; retain
  unreadable SRCC and incorrect changed/unchanged claims as separate findings.
- **QB/QE:** May identity/adoption extend across channel, FRMT/RCCVER change,
  sequence gaps, explicit stop or actual EOF? Recommend no automatic extension;
  retain raw identity as information and require explicit scope/source evidence.
- **QC/QP:** What governs after SHA-only/layout-only/other raw change or an incomplete
  comparison? Recommend report the raw change subtype, no speculative semantic
  reset; incomplete comparison remains unresolved, never presumed unchanged.
- **QH/QG:** Does an otherwise eligible integrity-failing copy govern, and what
  applies to buffered data? Recommend the reviewer's prominent-integrity-finding
  mode with opt-in strict quarantine ONLY after eligibility and applicability
  are reviewed. Do not waive F1, framing, trust or ambiguous record scope.
- **QS:** Does unreadable SRCC prevent using proven byte identity? Recommend
  retain equality as evidence, withhold automatic eligibility/governing inference;
  do not infer the bit. The matrix's proposed Complete label is not adoption.

## Implementation verification required, not performed here

L2-REG-007/L3-REG-014–015 require exhaustive generated enumeration of EVERY
member, not random sampling or a test that only executes this documentation
function. Each realizable member needs independent byte/provenance/digest
fixtures and observable assertions for every output column and no byte loss/
rejoin. Impossible observations need explicit consistency/rejection checks.
Open owner decisions must fail readiness visibly; do not encode them as passes.
Property-based mutation complements, never replaces, the full product.

Named owner fixtures: `srcc0_different_is_reported_change`,
`srcc1_identical_is_not_assumed_change`, `sha_only_following_not_sealed_proof`,
`sha_only_sealed_preserves_verified_exception`. Additional focused fixtures
cover identity with F1; SHA-only/layout-only deltas; partial-copy prefix match
versus mismatch; projected/case-varied G\SHA names versus text inside values;
unterminated G\SHA; uppercase hex and noncanonical `02-` warning; BOM/damaged
name; and EOF or unsupported-format boundaries. Digests use independent
Python hashlib, never the implementation or the unreproducible chapter example.

All prospective behavior stays in proposed INT-046. The documentation
coverage/expansion check is not an executed library/CLI test, completed
interpretation analysis, or evidence that any owner question is answered.
