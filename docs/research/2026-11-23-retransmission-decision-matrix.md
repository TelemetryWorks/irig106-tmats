# Exhaustive retransmission matrix: owner-signed baseline, AI check pending

Created 2026-11-23; updated 2026-11-25. Joey signed Q0/QD/QI/QB/QE/QC/QP/
QH/QG/QS on 2026-11-25. [Verbatim record and register mapping](2026-11-25-retransmission-owner-signoffs.md).
**Independent AI CONCUR at db1dbf7 covers INT-046/047/049/050/051/052
and INT-053 wording. INT-048's cut-facet amendment and the updated sequence
fixtures await recheck under ADR-0033. No runtime implementation or verification
is claimed.** [Exact-head review follow-up](2026-11-25-retained-cut-and-qm-review-followups.md).
QM is now owner-signed in
INT-053, with separate bounded sequence fixtures below; the pairwise product
is unchanged. [QM owner wording](2026-11-25-qm-governing-comparison-owner-decision.md).
QD is effective in this documented baseline NOW: established field ambiguity
forces assembly Ambiguous; identity or a digest can never upgrade it. QI is
limited to otherwise-Complete bounded time/+1/same-channel/same-profile copies,
with the INT-045 finding visible even on Complete. An SRCC=0 claim is checked.

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
  governance still have unresolved parts. INT-048 now makes established
  INT-039 field ambiguity assembly Ambiguous. QH/QG approves the integrity
  mode, not every general governing-setup or provisional-use rule.

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
and basis (also repeated in baseline columns), following comparison and
conditional assembly outcome, findings, governing effect, integrity policy,
owner-signed decision references and an empty pairwise open-question list,
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
formula. Class IDs hash the full current output; the generator checks collisions.

## Decision precedence: owner-signed baseline

R0 rejects/holds mutually inconsistent observations first. For coherent input,
retain prior evidence and apply **QD before QI or verified-digest exceptions**.
`sealed_outcome`/`sealed_basis` now equal the baseline columns. The former
`qd_proposal` column is removed: QD is the owner-signed baseline, not deferred.
Owner sign-off changes this documentation now; AI review still gates implementation.

| Rule | Exact condition (after R0) | SEALED outcome and basis |
| --- | --- | --- |
| R0 | B with unequal checksum statuses; B with sealed X; H with absent checksum or unequal duplicate multiplicity; sealed X with sealed V/F; sealed X with content H/W; following X with following V/F | Rejected/held inconsistent observations (INT-047). No valid-input baseline verdict; keep every nominal member and explicit rejection test, never parser coverage |
| R2D | Sealed checksum D, any supplied state/boundary/content | Assembly Ambiguous (INT-048), independently of a related follower; preserve all cut/identity/digest evidence and F1 indeterminate/refuse-stamp. No upgrade |
| R1A | Supplied sealed A, checksum not D | Preserve established field/split Ambiguous; no equality/digest upgrade |
| R1X | Supplied sealed X, checksum not D | Incomplete, with cut evidence; no prefix/identity rescue |
| R1T | Supplied sealed T without the time/C/SS guard | Complete on supplied boundary evidence; no cross-boundary identity exception or automatic adoption |
| R3B | Sealed T, no field ambiguity; time seal, continuity C, profile SS; content B | Complete, base closure label plus `identical retransmission` (INT-046); SRCC 0/1/U does not select the result; keep the located INT-045 finding |
| R3V | Same guard, nonidentical content, sealed V, no field ambiguity | Complete on existing sealed-text G\SHA classification evidence only; keep both readings and finding |
| R3A | Same guard, nonidentical content, sealed N/F/M/U | Final Ambiguous on INT-045 premature-seal evidence; both readings and finding retained |

The baseline is conditional on supplied state, framing and scope, not an
approval of all open INT-034 predicates. R2D now applies even when cut text
contains a known duplicate: aggregate Ambiguous does not discard the cut or
make the record usable. The retained cut facet keeps Incomplete's FULL
blocking force: no ambiguous-acceptance option, including proposed INT-031's
labelled split if approved, can make cut text usable or governing. INT-034
and INT-045 carry explicit amendment notes pointing to INT-048. Other established field ambiguity is represented by
supplied A. A cut AND other-field ambiguity requires named compound fixtures;
the flattened input state is not an assertion of exhaustive byte-level shapes.
Unknown/unreviewed cardinality must not be invented to trigger QD (INT-039).

R1 boundary labels remain time/I = `initial time seal (INT-045)`, time/M =
`time-boundary seal (owner policy)`, S = `same-channel stop`, E =
`end of input (labelled)`. Preserve these alongside QD's ambiguity basis.
The 6 coherent byte-identical D/D guard members remain Ambiguous; the 30
nonduplicate R3B guard members remain Complete. ALL keep identity evidence
AND `possible_premature_time_boundary_seal_both_readings`, not an erased finding.
Identity requires an independently bounded WHOLE extent; SRCC=0 acquires no
new time-closure rule, a matching prefix proves no end, and QB/QE permits no
automatic extension across channels, profiles, gaps, stops or actual EOF.

R0 includes cut/F on either candidate, not a proved mismatch under INT-019.
Cut-sealed/H or W cannot explain a bounded complete following copy. These
members remain listed/counted as consistency checks. Null baseline columns
mean no coherent valid-input verdict, not an accepted incomplete document.

| FOLLOWING comparison column | Members | Effect |
| --- | --- | --- |
| Cut | X | Comparison unresolved; a possible continuation reading never authorises a rejoin |
| Same metadata | SS; B/H/W/D/R | Identical retransmission / SHA-value-only change / layout-only change / independently established semantic change / other raw change, semantics unsettled |
| Changed metadata | SD/DS/DD | Metadata change with raw-content subtype; no automatic semantic equivalence |
| Different channel | O | Independent/unlinked information; never a cross-channel winner |
| Different FRMT | DS/DD | XML unsupported; check same-session ASCII/XML-mixing conformance from provenance |
| Duplicate following checksum | Following D | F1 indeterminate and following assembly Ambiguous under INT-048; new adoption blocked |

`following_assembly_outcome` applies the same QD precedence to that candidate
from supplied framing and known checksum ambiguity. It is conditional, not a
claim of a full validator: other following-field ambiguities need named fixtures.
Cut comparisons remain unresolved even if QD makes their assembly Ambiguous.
H/W/D/R with SRCC=0 report a differing-unchanged claim; SRCC=1 plus B/SS
retains its changed-claim/identical-text finding. Do not infer U as 0 or 1.
Raw layout/value deltas do not prescribe time-correlation resets (INT-050).

## Integrity mode and governing effect (owner-signed, not automatic switching)

INT-051 sets **prominent integrity findings by default** for otherwise eligible
initial and mid-session candidates. Integrity failure alone does not silently
quarantine or select the old setup. Strict quarantine is opt-in only after
its own review. The `integrity_policy` column states this explicitly.
All source findings remain, including F1, control/name projections and INT-043
noncanonical-designator warnings. Do not stamp, select duplicates or repair input.

QD Ambiguous and Incomplete block new adoption. Complete remains subject to
framing, trust, scope and applicability checks, with no automatic governing
switch. An unreadable following SRCC withholds its eligibility/governing
inference (INT-052), while equality may still support the bounded sealed-record
classification. The integrity mode does not waive BOM/damage, F1 or uncertain
assembly scope. General packet applicability/provisional publication rules
are not decided by accepting QH/QG's mode.

## Signed decisions and remaining review

| Codes | Owner-signed entry, Joey 2026-11-25 | Decision |
| --- | --- | --- |
| Q0 | INT-047 | Reject/hold contradictions; count separately from parser coverage |
| QD | INT-048 | Immediate assembly Ambiguous for field ambiguity, before identity/digest; owner overrides deferral |
| QI | INT-046 | Bounded otherwise-Complete identity exception; visible INT-045 finding, all SRCC observations |
| QB/QE | INT-049 | No automatic extension; equality outside scope is information only |
| QC/QP | INT-050 | Raw change subtype, no speculative reset; incomplete comparison unresolved |
| QH/QG | INT-051 | Prominent-integrity-finding default after eligibility checks; quarantine requires separate review |
| QS | INT-052 | Equality evidence does not infer unreadable SRCC, eligibility or governance |

Artifact `owner_decisions` flags refer to this table; manifest maps codes to
entries. `owner_questions` is empty for this PAIRWISE product: the choices
above are signed, not open. The independent check CONCURS on INT-046/047/049/050/051/052 and
INT-053 wording at db1dbf7; INT-048 and updated fixture follow-ups await
recheck. Implementation readiness remains blocked. QM/INT-053 is owner-signed as a separate sequence layer
outside the pairwise row domains; no runtime sequence coverage is claimed.

## Multiple following copies: owner-signed QM sequence contract

Joey signed QM on 2026-11-25 (INT-053; independent AI check pending). Each
later copy compares with the **currently governing setup**, G: the original
sealed record if it governs, or the latest copy that validly took control.
A received copy does not become G merely because it is newest, Complete,
SRCC=0/1, byte-identical or checksum-verifying. Adoption evidence comes from
the separately reviewed transition/applicability rules, not this table.

| Established control evidence for a copy C | Comparison target for later copies |
| --- | --- |
| C validly took control | C, with its identity and supporting evidence |
| C definitely did not take control | The existing G, which may itself be unresolved; C is not promoted |
| Whether C took control is unresolved | Unresolved; retain the evidence and possible identities, never choose one |

A changed copy failing integrity in warn mode, or an Ambiguous copy, must
leave the target unresolved WHEN its takeover is unresolved. Do not convert
QD's adoption prohibition into proof of the recorder's actual old setup.
Nor does prominent-finding mode prove that the changed setup took control.
An unresolved target stays unresolved until reviewed evidence actually
establishes governing identity; a later apparently good/equal copy alone
cannot repair it. Report local framing/ambiguity findings independently.

Raw equality to the **immediately previous** copy is a separate INFORMATION
comparison, naming both copies. It never changes eligibility, governing or
classification. In particular, previous-copy equality cannot substitute for
an unavailable G comparison or invoke QI. If G is known, apply the pairwise
contract to the independently identified G/copy comparison with its guards;
do not treat the immediately previous copy as G by position.

Preserve every earlier copy's located findings without overwrite. Retain old
governing setup identities/descriptions for data already decoded under them;
a later takeover or unresolved target does not retrospectively relabel that
data. Retaining historical G does not make it the current comparison target.

**Bounded verification choice:** keep the pairwise product unchanged at
139,968 members / 39,072 classes. QM needs state and historical evidence,
so it has separate named fixtures with an established original G0 and at
most **three later copies**, not a Cartesian multiplication or a runtime
implementation. This is a fixture bound, not a recording/history limit or
claim of exhaustive arbitrary-length coverage. B/H/D below always identify
their comparison target explicitly; no arrow silently switches targets. In
this SEQUENCE table, D means a semantic content difference (content code D),
not a duplicate checksum (SHA-status code D); duplicate SHA is stated by name.

| Named planned test | Bounded sequence and established control evidence | Required observation |
| --- | --- | --- |
| `qm_B_then_D_uses_governing_target` | C1=B(G0), definitely no takeover; C2=D(G0) | C2 targets G0, not C1 by position; report D and preserve C1 findings; do not invent C2 adoption |
| `qm_H_then_B_previous_identity_is_information_only` | C1=H(G0), definitely no takeover; C2=B(C1) | Previous B is information only; governing comparison remains H(G0), never classified as unchanged because C1 repeated |
| `qm_third_copy_only_checksum_does_not_select_target` | C1/C2 no SHA and definitely no takeover; C3 adds the only SHA, verifying C3 | G0 remains the target until valid takeover evidence; C3's SHA verifies neither G0 nor earlier copies. Adding an item is R, not H |
| `qm_changed_srcc_claims_never_select_target` | C1/C2/C3 carry changed/unchanged/unreadable first-fragment claims against fixed control evidence | Target follows control evidence, never the bits; retain changed-claim discrepancy/unreadable findings |
| `qm_unresolved_governing_blocks_later_target` | C1=D(G0) with failed integrity in warn mode AND unresolved takeover; C2=B(C1) | Target unresolved, neither G0 nor C1 selected; previous equality is information only, governing comparison unavailable |
| `qm_ambiguous_copy_does_not_prove_old_target` | C1 has QD field ambiguity and unresolved actual takeover; C2 follows | Keep QD Ambiguous and unresolved target; adoption prohibition is not proof that G0 governed later recorded data |
| `qm_valid_takeover_updates_governing_target` | C1 has independently established valid takeover as G1; C2 differs from G1 even if equal to old G0 | Target G1; G0 equality is not a governing comparison. Historical G0 still identifies previously decoded data |
| `qm_unresolved_then_valid_takeover_resolves_target` | C1 takeover unresolved; C2 has independently established valid takeover; C3 follows | Target becomes C2 and C3 compares with C2; unresolved is not permanent. Preserve C1 findings and the unresolved history; no inference from C2 merely being Complete/equal/good |
| `qm_unresolved_persists_through_non_takeover` | C1 takeover unresolved; C2 definitely did not take control; C3 follows | Target remains unresolved, never promoted to C2 or restored to a guessed G0; keep the non-takeover evidence and prior findings |
| `qm_previous_identity_mutation_preserves_decisions` | Fix G, candidate C and adoption evidence; vary only whether an unadopted immediate predecessor equals C | Only previous-comparison information changes; eligibility, governing and classification are unchanged, with each history's findings retained |
| `qm_findings_and_prior_decoding_provenance_are_persistent` | Decode under G0; valid G1 takeover; then a rejected or unresolved C2 | Retain all prior findings and both governing descriptions/packet references; do not overwrite or relabel prior decoding |

All cases are PLANNED and require markers for INT-053 and L3-REG-016 plus
applicable dependencies. They do not decide the remaining adoption algorithm,
permit an unreviewed strict-quarantine mode or claim runtime verification.

## Implementation verification required, not performed here

L2-REG-007/L3-REG-014–015 require exhaustive generated enumeration of EVERY
member, not random sampling or a test that only executes this documentation
function. Each realizable member needs independent byte/provenance/digest
fixtures and observable assertions for every output column and no byte loss/
rejoin. Impossible observations need explicit consistency/rejection checks.
Open owner decisions must fail readiness visibly; do not encode them as passes.
Property-based mutation complements, never replaces, the full product.
CI runs `spec.py --check` alongside the trace drift check; this checks checked-in
documentation only, not library behavior or the planned QM sequence tests.

Named owner fixtures: `srcc0_different_is_reported_change`,
`srcc1_identical_is_not_assumed_change`, `sha_only_following_not_sealed_proof`,
`sha_only_sealed_preserves_verified_exception`. Additional focused fixtures
cover identity with F1; SHA-only/layout-only deltas; partial-copy prefix match
versus mismatch; projected/case-varied G\SHA names versus text inside values;
unterminated G\SHA; uppercase hex and noncanonical `02-` warning; BOM/damaged
name; and EOF or unsupported-format boundaries. Digests use independent
Python hashlib, never the implementation or the unreproducible chapter example.

All owner-signed pairwise behavior is recorded in INT-046–052, AI check pending. The documentation
coverage/expansion check is not an executed library/CLI test, completed
interpretation analysis, or a substitute for independent AI review and development gates.
