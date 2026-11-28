# Development analysis: remaining edit/mapping/default rules — 2026-11-28

INT-022–027/033 are the final seven entries without written analyses on main
7ac43ea after #99. All are owner-signed; this draft changes only Analysis
links, not Design/Review or behavior. Owner/independent analysis review and
intensive executable tests remain required. General removal edits stay 0.5;
packet cross-checking belongs to validator, and general registry/validation/
fragment reconstruction remains its later release scope. No feature is brought
into 0.1 merely by documenting its already-approved interpretation.

## Sources and boundaries

Fresh archive manifest checked 2026-11-28. Verified 45 unique cached PDFs:
fourteen Ch9, fourteen packet chapters sharing the full 04 volume, thirteen
separate Ch6 files, plus RCC 123-16 and four RCC 124 handbooks. Located and
reread affected X/deletion, command example, recorder conditions, conversion
links, fragment-position/example and packet-code rows across those sources.
Text search only locates passages; row continuation/context is read separately.
Earlier missing/unlocated vocabulary does not imply modern metadata, a false
condition or historical alias. No rendered/visual equivalence is claimed.

| Source asset | SHA-256 |
|---|---|
| [106-04 106-04.pdf](https://github.com/TelemetryWorks/rcc-106-standards/releases/download/106-04/106-04.pdf) | `a75ef16a77ecb4838c0c9678b7d483049e84c94d7a49a3edb7f7199ce98914c8` |
| [106-05 chapter9.pdf](https://github.com/TelemetryWorks/rcc-106-standards/releases/download/106-05/chapter9.pdf) | `ccdc77ccdd2f185a33e7ad1ed168375564432ae6d541172fef88ce0b783ea6b1` |
| [106-07 chapter9.pdf](https://github.com/TelemetryWorks/rcc-106-standards/releases/download/106-07/chapter9.pdf) | `ef8783a59dee91db9b1fd7ac1802617cb87f5f52e2ea5de395bc42224b8f66de` |
| [106-09 chapter9.pdf](https://github.com/TelemetryWorks/rcc-106-standards/releases/download/106-09/chapter9.pdf) | `461ae43340ff623212fdcd8bf1d6bc3fad12e479d0dc162e3f5534ae8e2e9ccc` |
| [106-11 chapter9.pdf](https://github.com/TelemetryWorks/rcc-106-standards/releases/download/106-11/chapter9.pdf) | `238b6469f5cac00e2d776b96869ed21414e1af6637e666b9f4cdf0a46a48d4e8` |
| [106-13 Chapter9.pdf](https://github.com/TelemetryWorks/rcc-106-standards/releases/download/106-13/Chapter9.pdf) | `33028a325390d013c4e694c8ead75b2ee0bc52786d2477ca795733b422bc2051` |
| [106-15 Chapter9.pdf](https://github.com/TelemetryWorks/rcc-106-standards/releases/download/106-15/Chapter9.pdf) | `f8c6ec06d389765ef783277e235ca1b8b9740fc88ca6299f464ca32ca24fb579` |
| [106-17 Chapter9.pdf](https://github.com/TelemetryWorks/rcc-106-standards/releases/download/106-17/Chapter9.pdf) | `9c726e5cd5e3883601fd70b18556ebef02a736c7a30e1909c33eccd12bebdaac` |
| [106-19 Chapter9.pdf](https://github.com/TelemetryWorks/rcc-106-standards/releases/download/106-19/Chapter9.pdf) | `0f0a7536e1858e31475535a592726f7943a085ccc2e7b3a033c20e8aa682047d` |
| [106-20 Chapter9.pdf](https://github.com/TelemetryWorks/rcc-106-standards/releases/download/106-20/Chapter9.pdf) | `add7a48f0ff0a1ef3840ecefcf9191dea2edb8605bc6de4dc981556d722c3019` |
| [106-22 Chapter9.pdf](https://github.com/TelemetryWorks/rcc-106-standards/releases/download/106-22/Chapter9.pdf) | `773bb32f54bf18e29a6cf4f5d5e998d094d668202889edb6ee56179bb424c6f7` |
| [106-23 Chapter9.pdf](https://github.com/TelemetryWorks/rcc-106-standards/releases/download/106-23/Chapter9.pdf) | `32b74561ac3e5842dfdff15a27c234ebcbb6817f8cfca3513eaf9897ace1b19a` |
| [106-24 Chapter9.pdf](https://github.com/TelemetryWorks/rcc-106-standards/releases/download/106-24/Chapter9.pdf) | `7e9253a9f2cbb2a55c9e3e702583c46fc9b0a8d69410928a491e9638036381c9` |
| [106-24R1 Chapter9.pdf](https://github.com/TelemetryWorks/rcc-106-standards/releases/download/106-24R1/Chapter9.pdf) | `dfc5ebfb3e6f02f58389126863d43a45d89c7e2d0533a1c690d9537531c740cb` |
| [rcc-123-16 123-16_IRIG_106_Chapter10_Programmers_Handbook.pdf](https://github.com/TelemetryWorks/rcc-106-standards/releases/download/rcc-123-16/123-16_IRIG_106_Chapter10_Programmers_Handbook.pdf) | `aedb41412f22283f2246e25c3b55befe5527af617ec83f8285e00b899f5191ab` |
| [rcc-124-13 124-13_TMATS_Handbook.pdf](https://github.com/TelemetryWorks/rcc-106-standards/releases/download/rcc-124-13/124-13_TMATS_Handbook.pdf) | `b83e071b43c1842cf5f84437a70b3275ff6af045a8ffa4a77f0def95ed2301aa` |
| [rcc-124-17 124-17_TMATS_Handbook.pdf](https://github.com/TelemetryWorks/rcc-106-standards/releases/download/rcc-124-17/124-17_TMATS_Handbook.pdf) | `8622c2789591ed891f02e0fce9121cf09fd3097b94412491dcdc62e2d00f7006` |
| [rcc-124-19 124-19_TMATS_Handbook.pdf](https://github.com/TelemetryWorks/rcc-106-standards/releases/download/rcc-124-19/124-19_TMATS_Handbook.pdf) | `f16ffedbc89b4bbb8f7c51e01706c9079330c4b1c753a87f1f3f37e3bf70ab5e` |
| [rcc-124-22 124-22_TMATS_Handbook.pdf](https://github.com/TelemetryWorks/rcc-106-standards/releases/download/rcc-124-22/124-22_TMATS_Handbook.pdf) | `99e47e0a945560466e628098717f254c30dcd16ab77fc045ee53abfa4376d228` |
| [106-05 chapter10.pdf](https://github.com/TelemetryWorks/rcc-106-standards/releases/download/106-05/chapter10.pdf) | `7aa84ef996df5ecd59da4159b82415ef2ba791a81dd20266c7d96a29adbf6bae` |
| [106-07 chapter10.pdf](https://github.com/TelemetryWorks/rcc-106-standards/releases/download/106-07/chapter10.pdf) | `50d125bc744001d803dcb8cadcb5c2038940ca980098afeb0ebcd25601d8e582` |
| [106-09 chapter10.pdf](https://github.com/TelemetryWorks/rcc-106-standards/releases/download/106-09/chapter10.pdf) | `1f93423591c43277009762c31c816358aead90bea9b570a4afc4c68e84ee0ec6` |
| [106-11 chapter10.pdf](https://github.com/TelemetryWorks/rcc-106-standards/releases/download/106-11/chapter10.pdf) | `8cef16928df5d3f93dfff6f95806e9923b41927413090b0844bf28313065c7a7` |
| [106-13 chapter10.pdf](https://github.com/TelemetryWorks/rcc-106-standards/releases/download/106-13/chapter10.pdf) | `e3740c7ac3099236f4e97cbe5b84c169340e6d4a4222769619e83ea0f8176fde` |
| [106-15 chapter10.pdf](https://github.com/TelemetryWorks/rcc-106-standards/releases/download/106-15/chapter10.pdf) | `cace968a4a796a56895eff36073840ae2db3a9c57dd33794d0c348e95fe0c204` |
| [106-17 chapter11.pdf](https://github.com/TelemetryWorks/rcc-106-standards/releases/download/106-17/chapter11.pdf) | `7297c782e0368e72ac783a72edccaa04eed28a8ea0e8ab7c61ea39379f516750` |
| [106-19 chapter11.pdf](https://github.com/TelemetryWorks/rcc-106-standards/releases/download/106-19/chapter11.pdf) | `ac06990ab1b98594cf70fd82815ae2d1efb6d7fb58bf329cf2dcc71a8b64ad9d` |
| [106-20 chapter11.pdf](https://github.com/TelemetryWorks/rcc-106-standards/releases/download/106-20/chapter11.pdf) | `e66370ce21b4ed44ed45f32e4785667398cf401e358f03e6f07494f06615748d` |
| [106-22 chapter11.pdf](https://github.com/TelemetryWorks/rcc-106-standards/releases/download/106-22/chapter11.pdf) | `e3e85ed09ddec95d49e9e1851ac4234b5df35b6340d9d91f355b1c598543d0a6` |
| [106-23 chapter11.pdf](https://github.com/TelemetryWorks/rcc-106-standards/releases/download/106-23/chapter11.pdf) | `ed30d02fe27cd89d9172ada824251fd6cd748bc87cb63c15f371152fa3274781` |
| [106-24 chapter11.pdf](https://github.com/TelemetryWorks/rcc-106-standards/releases/download/106-24/chapter11.pdf) | `d26e7926f4b2e7977df5e6210044cfc0d4c1191d45a66b71da2d530bf68de6a8` |
| [106-24R1 chapter11.pdf](https://github.com/TelemetryWorks/rcc-106-standards/releases/download/106-24R1/chapter11.pdf) | `0f63792dcac39f6010df371ee0afc46a396017918f4155cba9ebab6c2d6d263e` |
| [106-05 chapter6.pdf](https://github.com/TelemetryWorks/rcc-106-standards/releases/download/106-05/chapter6.pdf) | `e4e032eade65b2124301bdd56d267c3a21849c071c08e6f9b512d469de9dd0cd` |
| [106-07 chapter6.pdf](https://github.com/TelemetryWorks/rcc-106-standards/releases/download/106-07/chapter6.pdf) | `08b5e9b277d74361cfea59d3037cd93d5cccaef3e27936aeed34367ff0cb7498` |
| [106-09 chapter6.pdf](https://github.com/TelemetryWorks/rcc-106-standards/releases/download/106-09/chapter6.pdf) | `6fa953068df57a868a301cf8000478e0885bc972c6b5370d837a357987794719` |
| [106-11 chapter6.pdf](https://github.com/TelemetryWorks/rcc-106-standards/releases/download/106-11/chapter6.pdf) | `f4b66ff1c6636bbcf106db33b4de6c068034df7e1235b6979af73cf25f6787ba` |
| [106-13 chapter6.pdf](https://github.com/TelemetryWorks/rcc-106-standards/releases/download/106-13/chapter6.pdf) | `6cb908f9fb59959b277f57e6aeea5d2f9f9ccef42ad366b3dde2e13682227a9d` |
| [106-15 chapter6.pdf](https://github.com/TelemetryWorks/rcc-106-standards/releases/download/106-15/chapter6.pdf) | `602984a66afa370b2f38040f0571cb0627ea89d342c2ce9edc78cc27dba216f8` |
| [106-17 chapter6.pdf](https://github.com/TelemetryWorks/rcc-106-standards/releases/download/106-17/chapter6.pdf) | `b8a3bc7159cd58d02e8e4ac0e4602873a4110689d852b09e55a820e5fed0ae1b` |
| [106-19 chapter6.pdf](https://github.com/TelemetryWorks/rcc-106-standards/releases/download/106-19/chapter6.pdf) | `f5bf451e2d0567f10cda2bf03c927b3bf1387b07693019e1d6c27814248f0abe` |
| [106-20 chapter6.pdf](https://github.com/TelemetryWorks/rcc-106-standards/releases/download/106-20/chapter6.pdf) | `02124882a382f41e80fbe31bfb20d7ac6252416100b4c3cb41da974b7313e30f` |
| [106-22 chapter6.pdf](https://github.com/TelemetryWorks/rcc-106-standards/releases/download/106-22/chapter6.pdf) | `5b260104b4ee9fb70f4d65948d7723435af4a2d6b2f264a9a1d934be3ea3c49e` |
| [106-23 chapter6.pdf](https://github.com/TelemetryWorks/rcc-106-standards/releases/download/106-23/chapter6.pdf) | `855442db01cbfd503904bfe245c2c5bde3ef724b35b3fee8b1a0c71258767a03` |
| [106-24 chapter6.pdf](https://github.com/TelemetryWorks/rcc-106-standards/releases/download/106-24/chapter6.pdf) | `c05ab889aa1b4886bcebdb9b814913c709812fb563f055654a536396c9ae70f2` |
| [106-24R1 chapter6.pdf](https://github.com/TelemetryWorks/rcc-106-standards/releases/download/106-24R1/chapter6.pdf) | `762d96347855f2d19289b71af03d6acb618f6534825400767a760bfcd8b8c893` |

### Baseline source bearings and historical limitations

| Source | What it establishes; what it does not |
|---|---|
| 106-24R1 Ch9 §9.5.14 p. 9-172 | Preserve X associations during renumbering, match original indices, add no new original occurrence. It prints no original-removal policy; owner INT-022 supplies that policy. |
| Ch6 §6.2.3.11 pp. 6-22–23, example on p. 6-22 (PDF 28) | Literal G\DSI\N=18; occurs in command examples; it is not conforming colon framing or a scanner exception. |
| Ch9 Table 9-2 NUMBER OF DATA SOURCES G\DSI\N p. 9-11 | Complete name metadata for the positive guarded example; recognizing it is not accepting the malformed equals framing. |
| Ch9 §9.4.2 p. 9-2 | First-colon/semicolon framing, case and intentional blanks; not automatic punctuation repair or a generic semantic-equivalence rule. |
| Ch11 Table 11-4 pp. 11-7–8 | No 22 row, 23 labelled Analog Format 2 reserved; CAN single78 and Fibre Channel79/7A. The table is irregular; a reserved row is not an available valid TMATS format/admission mapping. |
| Ch9 Table 9-4 CDT p. 9-50; PDTF p. 9-50, UTF p. 9-77, FCTF p. 9-82 | Actual CDT and format fields need an edition-qualified reviewed map, not base+format arithmetic or a borrowed meaning from a similar name. |
| Table 9-4 TDTF p. 9-81; CDT p. 9-50 | Condition prints TSPIN; enumeration prints TSPIIN. The approved reading changes the condition metadata, not user data. |
| Table 9-4 RPS p. 9-51; MFF RPS p. 9-54; P-group definitions | P CDT references are erroneous; the meaningful local operand is same-channel R CDT. Keep doubled When source defect without creating a P field. |
| Table 9-11 DCN p. 9-158 and R AMN/DMN Links-to rows | Repeated AMN reference; only reviewed relationships form graph edges. Definition-edge dedup never deduplicates document items. |
| Table 9-7 MNF counter p. 9-117, WFP row p. 9-118 | Own scoped count, conditional position/range, default1 and each position once; these are distinct from index-set counting. Some earlier review references said p. 9-117 for WFP; the hash-identified row itself is on 9-118 (PDF 124). Behavior is unchanged. |
| Appendix 9-C pp. C-7–8, Table C-2 and D3 fragments | Two WP locations, count2, no WFP fields; explanatory M/L prose supplies no machine-readable position override. The standard example itself triggers the approved default-collision finding. |

Older Ch9 and handbook hits are not global aliases or production mapping
coverage. Preserve each original edition/code/reference list and availability;
review edition-specific conditions/graph/range metadata before executing it.
The command-example locator is present in inspected 07 and 15 and subsequent
Ch6 examples, not proof of identical framing in every earlier edition.
The older packet-code tables remain separate source profiles; no current
Table 11-4 family or reserved/defined status is retrojected into them.
RCC 123/124 context does not override the source standard or owner rule.

## INT-022: refusal includes uncertainty, not only proved attachments

Use INT-061/062's reviewed decomposition, registry vocabulary, instance/scope
and canonical-index evidence. A resolved X blocks default original deletion;
so do unresolved/Ambiguous/cannot-evaluate attachments, listing all possibly
attached items. Unique G-PN does not activate the unselected unique-path
Option C; unknown/private vocabulary cannot prove an empty attachment set.
Never infer a safe deletion from no convenient resolved result. Possible
attachment still needs the reviewed scope/evidence relationship; this is not
an indiscriminate rule that every X item blocks every unrelated original.

Target exact revision-bound item identities, not first/last value matches.
Refuse a stale plan; preserve the original source. Any cascade/orphan-preserving
operation is a separate explicit reviewed edit with a full deletion/evidence
set. Do not activate B as a default, manufacture a mapping, erase unknown X,
or silently cascade. Later renumber/removal association preservation is not
permission to edit during read, normalization or a checksum computation.
This 0.5 policy does not broaden 0.1's minimal stamp/exact-item transaction.

## INT-023: missing-delimiter reading plus a guarded suggestion

Read the command example exactly under INT-035. With no colon, keep the
region and located missing-delimiter finding, never a successful explicit
G-DSI count value or a repaired extra attribute. Only positively recognized
COMPLETE name before '=' and no colon anywhere in that item permit the
signed suggestion. Partial name, unknown arity, damaged name or unreviewed
private decomposition cannot satisfy positive recognition. Keep actual source
correspondence if reviewed name projection participates in recognition.

Suggestion is a located one-byte proposal, not an edit: raw replay, item
count/lookups, checksum preimage and independent findings stay unchanged.
A future explicit edit creates a new revision with its own reevaluation.
An '=' inside COMMENT/value, a later colon or an expression operator is not
this guard. INT-010's equality language and INT-013's colon-in-value warning
remain different contracts; no global replacement, fabricated colon or
blanket punctuation recovery. This analysis does not expand the source-cited
0.1 semicolon detector to arbitrary repair or imply complete historical
recognizer coverage.

## INT-024: reviewed mapping, not arithmetic or packet reading

Bind SAME-channel CDT plus its actual format attribute, edition/source basis
and reviewed mapping. Keep the baseline's irregular literal rows; 23 is the
reserved Analog Format2 row, not 20+2=22. This demonstrates why arithmetic is
unsafe, not that a reserved format is a usable expected mapping. CAN's single
78 and Fibre Channel79/7A similarly refute fixed families of eight.
Usable TMOUT has no expected recording packet type; unknown CDT is different
from that known no-type result.

Missing/Invalid/Ambiguous/unavailable operand or unavailable edition mapping
yields cannot-evaluate, all occurrences kept, never an inferred mismatch.
A determinate mismatch needs independently usable expected mapping and actual
observation. INT-055 applies only to reviewed decimal format-code definitions;
02 can match2 with warning where applicable, not via index rules or a generic
cast. State and range validity still apply. Preserve INT-044/017 intended basis
versus actual baseline vocabulary; do not claim old renamed-code support.

Tmats owns the mapping, irig106-validator owns setup-versus-recording checks.
Core owns packet finding, type/admission/integrity evidence. PacketFacts is
header facts only; PacketInfo sideband travels alongside, unchanged. No local
reader/checksum, reserved-type admission, recording rewrite, inferred time
association or new cross-repository dependency is introduced by this note.

## INT-025/026: repair definition references, not document values

TDTF's printed TSPIN condition reads TSPIIN in the reviewed definition.
Literal user CDT=TSPIN stays an unknown keyword; there is no global alias.
Bind its same recorder/channel CDT. Another channel's TSPIIN, a spelling in a
COMMENT or a sole available candidate does not supply that operand. Preserve
Missing/Invalid/Ambiguous/unavailable condition evidence, not false or borrowed
truth. Evaluating any R-N prerequisite uses INT-055 raw numeric rule, or an
explicitly supplied evaluated condition in a focused fixture.

For RPS and MFF-RPS, bind R CDT to the same recorder/channel. MFF's additional
m stays the field's own scope, not a reason to traverse P or borrow another
R occurrence. The printed P CDT/doubled When text stays provenance. Do not
create a P CDT field, silently change a document path or erase unknown input.
A true PCMIN condition permits applicability only: it does not validate the
polarity VALUE, insert a default, repair numbers or declare conformance.
Unavailable historical condition metadata remains unavailable; the approved
metadata correction is not proof all editions carry identical definitions.

## INT-027: deduplicate a graph edge, not source data

Retain the repeated AMN in the definition's source text/citations; the same
reviewed edge enters generated relationship metadata once. INT-003's three-
source rule still applies: check outgoing, incoming and prose evidence, not
majority vote. The baseline AMN/DMN Links-to review does not justify inventing
another R source from EV-MN, IPMN, BME or SMF name resemblance.

Document AMN/DMN/DCN occurrences, equal values and duplicate findings all stay.
No first/last target winner, target-set value dedup or canonical index collapse
removes occurrences. INT-038 canonical index identity applies to name indices,
not link values; source/target scope and placeholder arity remain INT-004.
A historical list difference needs its own source/review, not a modern graph
silently applied under a misleading old-edition label. This is later general
link-generation work, not a data-channel-to-clock evidence rule.

## INT-033: default collisions stay visible, caller order stays an assumption

For the SAME D location tuple, an actually absent WFP needs verified applicable
own MNF>1 and the available edition default binding before Defaulted1.
Missing/damaged presence, Invalid/Ambiguous/unavailable WFP or unusable count
never means actual absence. No row/type/explicit position is synthesized.
A malformed or duplicate explicit value is not replaced by default1.

With two real members and verified applicable default1 for both, report the
collision and ambiguous order, retaining separate Defaulted results/citations
and all source locations. WFP position VALUE uniqueness is distinct from
INT-040's structural index membership/count, and from INT-039's duplicate
single-field ambiguity. Swapping e indices, source order, WP word locations
or reading M/L prose cannot break the tie. Other D locations have independent
position domains; no global uniqueness across the group.

Explicit per-query caller ordering can support a labelled assumption without
changing source/default state, suppressing the collision or curing record
cut/ambiguity. It is never an ambient order for the next query or automatic
governing proof. No reconstruction/evaluator or general validation engine is
added to 0.1. The Appendix example is a source-shaped negative conformance
fixture; its being printed by the standard does not make it conforming.

## Independent oracles and planned coverage

- Literal `G\DSI\N=18;` is 11 bytes: name-like prefix [0,7), '=' [7,8),
  digits [8,10), terminator [10,11); no colon and no explicit lookup value.
  Positive complete-name evidence permits only the [7,8) suggestion. Raw
  bytes stay identical until a separate explicit edit.
- Supply D occurrence3, location (1,1,1), count2 and e1/e2 with WP113/121
  but no WFP, plus independently reviewed applicable default evidence.
  Expected positions [Defaulted1, Defaulted1], one colliding value group
  listing both occurrences and ambiguous reconstruction order. This does not
  prescribe finding rendering/severity or synthesize an explicit WFP item.
- Source-table irregularity is fixed independently: Analog2 row23 reserved,
  not22; CAN78; Fibre Channel0/1 at79/7A. A source-table label is not a valid
  data-format operand or a successful recording-conformance verdict.

The following 31 cases are PLANNED, not executed replacement-code tests.
References to later L1 parents retain those parents' release scope; they do
not add later editing/validation/reconstruction to the 0.1 product.

| Planned case | Independent expected evidence | Requirements / interpretations |
|---|---|---|
| `final_int_analysis_attached_original_refuses` | Reviewed resolved attachment blocks default removal of the exact original; list all attached X items and preserve the revision/output. | L1-WRT-003; INT-022 |
| `final_int_analysis_possible_attachment_refuses` | Unresolved/Ambiguous/cannot-evaluate attachment, including index-less G-PN, blocks by default with every possibly attached X; never claim none or guess a cascade. | L1-WRT-003; INT-022 |
| `final_int_analysis_explicit_cascade_is_separate` | A separately reviewed explicit cascade lists exact revision-bound original/X deletion targets; unselected orphan-preserving B is not the default and unknown targets never enter deletion set. | L1-WRT-007; INT-022 |
| `final_int_analysis_stale_or_cross_document_target` | A stale exact-item ID or X from another document cannot select an original/cascade; do not carry old positional identity into a new revision. | L1-WRT-007; INT-022 |
| `final_int_analysis_equals_source_oracle` | G-DSI-N source spelling with raw backslashes is 11 bytes; no colon => no explicit value, retained missing-delimiter finding, suggestion only for complete recognized name at [7,8). | L1-READ-003; INT-023 |
| `final_int_analysis_equals_guard_not_substring` | COMMENT:a=b; already has colon: no suggestion. Partial/unknown/private or damaged unrecognized name before equals cannot satisfy whole-name guard. | L1-READ-003; INT-023 |
| `final_int_analysis_equals_preserves_raw_and_digest` | An unapplied proposal never changes source, framed items, raw checksum domain or field availability; applying an explicit future edit makes a new revision. | L1-READ-001; INT-023 |
| `final_int_analysis_equals_no_global_replacement` | An equals inside an ordinary value or expression is not a delimiter recovery; do not confuse this guard with INT-010 spaced equality or INT-013 colon lookahead. | L1-READ-006; INT-023 |
| `final_int_analysis_equals_historical_source_not_grammar` | An example hit supplies a citation, not a conforming fixture, scanner exception or proof that all older editions define the same name/guard. | L1-READ-003; INT-023 |
| `final_int_analysis_packet_map_irregular` | Source table labels Analog Format2 as 23 reserved, not arithmetic 20+2=22; CAN single78 and Fibre Channel0/1 at79/7A remain literal reviewed facts, not permission to use a reserved format. | L1-CH10-007; INT-024 |
| `final_int_analysis_packet_map_unusable_inputs` | Missing/Invalid/Ambiguous/unavailable CDT or format yields cannot-evaluate with all occurrences; no guessed mismatch, even equal duplicates or a sole candidate. | L1-CH10-007; INT-024 |
| `final_int_analysis_packet_map_edition_limit` | Unreviewed old/renamed/undefined format mapping gives cannot-evaluate with intended/applied vocabulary basis; baseline coverage cannot claim historical conformance. | L1-CH10-007; INT-024 |
| `final_int_analysis_packet_map_tmout_and_unknown` | Usable TMOUT has no expected recorded packet type; unknown CDT is not an empty clean mapping; no data-to-clock association follows. | L1-CH10-007; INT-024 |
| `final_int_analysis_packet_map_numeric_and_core` | Use reviewed numeric-code rule for applicable decimal formats, including 02 warning; core supplies observed codes/trust unchanged, validator owns recording cross-check, no second reader/checksum. | L1-CH10-007; INT-024 |
| `final_int_analysis_tspi_condition_only` | Printed condition TSPIN reads as same-channel TSPIIN; user CDT=TSPIN stays unknown, and other channels cannot supply the operand. | L1-REG-004; INT-025 |
| `final_int_analysis_tspi_unknown_not_false` | Missing/Invalid/duplicate/unavailable CDT keeps condition cannot-evaluate, not false; all source evidence is retained. | L1-REG-004; INT-025 |
| `final_int_analysis_tspi_projection_and_numeric_prereqs` | Supplied reviewed CDT/condition or raw evaluation of R-N follows INT-036/037/041/055; no implicit repair or bypass from a truthy string. | L1-REG-004; INT-025 |
| `final_int_analysis_tspi_no_global_alias` | A literal TSPIN in unrelated values/definitions is not replaced; absent historical condition metadata stays unavailable rather than borrowing the modern alias. | L1-REG-004; INT-025 |
| `final_int_analysis_polarity_local_scopes` | RPS binds (recorder,channel); MFF-RPS binds same (recorder,channel), retaining own m. A PCMIN on another channel/recorder is not substituted. | L1-REG-004; INT-026 |
| `final_int_analysis_polarity_no_p_attribute` | No P-CDT document field or cross-link traversal is manufactured; the printed bad reference and doubled When remain source provenance. | L1-REG-004; INT-026 |
| `final_int_analysis_polarity_uncertainty` | Missing/Invalid/Ambiguous/unavailable same-channel CDT keeps both conditions cannot-evaluate; duplicates never first/last-win. | L1-REG-004; INT-026 |
| `final_int_analysis_polarity_not_value_or_numeric_repair` | Usable PCMIN permits the reviewed condition only, not every polarity value, clip/normalization, inferred default or valid whole-document verdict. | L1-REG-004; INT-026 |
| `final_int_analysis_conversion_graph_only` | Repeated AMN in the definition creates one relationship edge with duplicate source citation retained; two document AMN items remain two occurrences/findings. | L1-REG-005; INT-027 |
| `final_int_analysis_conversion_no_missing_edge_guess` | Only reviewed AMN/DMN Links-to plus incoming/prose sources form the graph; EV-MN/IPMN/BME/SMF lookalikes do not invent C-DCN edges. | L1-REG-005; INT-027 |
| `final_int_analysis_conversion_scope_and_candidates` | Preserve recorder/channel/member and C target scopes, canonical index identity, all equal target candidates and occurrence IDs; graph dedup is not value-based target dedup. | L1-REG-005; INT-027 |
| `final_int_analysis_conversion_historical_difference` | A different old reference list is retained with coverage limits and re-review; modern no-op treatment does not certify unchanged historical graph. | L1-REG-005; INT-027 |
| `final_int_analysis_positions_default_collision` | Source-shaped D3 two fragments and no WFP: given established applicable default binding, both Defaulted1 with citation, collision and ambiguous order; example is not conforming. | L1-VIEW-008; INT-033 |
| `final_int_analysis_positions_no_index_or_word_inference` | Swap e indices, source order, WP113/121 positions or explanatory M/L prose: no inferred reconstruction order; canonical index duplicates still retain every occurrence. | L1-VIEW-008; INT-033 |
| `final_int_analysis_positions_absence_vs_unusable` | Only established actual absence and verified own MNF>1 allow default1; present Invalid/Ambiguous/unavailable WFP or unknown applicability never becomes default. | L1-VIEW-004; INT-033 |
| `final_int_analysis_positions_scope_and_uniqueness` | Position uniqueness is per location tuple, not global across D group; count/structural membership INT-040 is separate from duplicate WFP values. | L1-VAL-002; INT-033 |
| `final_int_analysis_positions_caller_order_per_query` | Explicit labelled per-query caller order preserves Defaulted collision and record qualifications, changes no raw bytes and never becomes an ambient next-query order or automatic governing proof. | L1-VIEW-008; INT-033 |

Use bounded CI properties varying complete-name roles and equals/colon
placement; exact revision IDs and X attachment cardinality/uncertainty;
CDT/format state and same-channel scope; repeated graph references versus
document occurrences; and fragment order/index/default/value collisions.
Pin expectations independently from production recognizers/resolvers. A
fixture per applicable archived edition needs actual definition evidence;
unknown historical coverage is not silently a baseline pass.

Executed locally: fresh manifest/45 PDF hashes and targeted source rereads,
fixed 11-byte and source-table oracles, case/reference/Analysis-only checks,
trace generation/--check and diff --check. No parser/editor/mapper/reference
implementation was executed. No Rust build/test, real recording, install,
publishing or settings action; all source jobs finish before reporting.

All 66 entries now have written analyses, including S1's still-suspect note;
that inventory is not 66 completed owner/independent ANALYSIS approvals or
executable coverage. Source freshness, actual bindings and intensive tests
remain gates before each interpretation's code. The companion
[family-2 completion audit](2026-11-28-family2-completion-audit.md) separates
phase handoff from later component/release evidence. No family-3 code is
started here and no requirement or Design/Review status is upgraded.
