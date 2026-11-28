# Development analysis: INT-041/042/044 — 2026-11-28

Written maintainer analysis for three APPROVED interpretations: read
availability/default evidence, physical-presence predicates and ambiguous
G\106 rules-basis fallback. The approved choices stay unchanged. This
analysis itself still needs owner/independent review and intensive executable
tests; it neither completes other INT analyses nor authorizes parser code.
Inspected main 233deae after #59 merged; Q25/INT-034 precedence and Q26/report
format remain owner-open. No new owner choice is selected here.

## Sources and freshness

Re-fetched the archive manifest on 2026-11-28 and checked local PDF bytes
against its SHA-256 entries before re-extracting/reading relevant pages.
Fourteen Chapter 9 editions, five companion handbooks and fourteen historical
packet-context readings cover 32 unique PDF assets: the 106-04 full volume
serves both Chapter 9 and Chapter 10. All source bytes are identified below.
Text search locates printed paragraphs/cells; table columns, continuation
pages and printed page numbers are read separately, not treated as repaired
TMATS. No rendered-image comparison, reference execution or new library code.

| Chapter 9 / companion source | SHA-256 |
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

Packet context re-read for the eligible Format 1 operand, not a new fallback
mapping. The shared 106-04 full-volume fingerprint is above; the remaining
packet chapters are:

| Packet source | SHA-256 |
|---|---|
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

The Format 1 section/figure/printed-page matrix remains the
[historical CSDW source review](2026-11-27-historical-csdw-layout-sources.md).
Here only its declaration-versus-policy and operand-eligibility implications
are rechecked. 106-04/05's inspected words are reserved, CH10VER/SRCC appear
in 106-07, FRMT in 106-09. Missing fields do not acquire a version by reading
reserved bits. The latest literal 0D/0E names are not the INT-017 policy's
106-20/106-24R1 targets. Format 4 and packet-header Data Type Version remain
other code spaces; they cannot supply a Format 1 RCCVER by numeric similarity.
This context does not mark INT-012/015/016/017 development analyses complete.

## Chapter 9 source bearings across editions

Every inspected Chapter 9 has the common logical-format/case/blank material
in section 9.4.2 pp. 9-2–3. It supplies retained textual facts, not the five
API states, damaged-name predicate recovery or duplicate-edition fallback.
The full blank sentence is already preserved at L3-VIEW-005; it does not
license generic value trimming. Approved INT-036 name projection and INT-055
RAW numeric rules stay separate, with INT-041 availability guards.

| Edition | G\106 / T-x\AN1 / R-x\TFMT-n row bearings | Relevant limitation/difference |
|---|---|---|
| 106-04 | Tables 9-1 p. 9-8 / 9-2 p. 9-13 / 9-3 p. 9-25 (full PDF 186/191/203) | Inspected TFMT lists letter formats and field size; no printed Default A/applicability column located |
| 106-05 | Tables 9-1 p. 9-8 / 9-2 p. 9-13 / 9-3 p. 9-28 | Same evidence boundary; TSRC is a separate field, not an inferred default/clock association |
| 106-07 | Tables 9-1 p. 9-9 / 9-2 p. 9-14 / 9-3 p. 9-33 | Inspected TFMT letters/field size and legacy recorder-status tags, not modern usage metadata |
| 106-09 | Tables 9-1 p. 9-9 / 9-2 p. 9-14 / 9-3 p. 9-40 | Same row-versus-usage distinction; no baseline default retrofitted |
| 106-11 | Tables 9-1 p. 9-9 / 9-2 p. 9-14 / 9-3 p. 9-36 | Inspected letter time formats include IRIG variant prose; no default inferred from it |
| 106-13 | Tables 9-3 p. 9-9 / 9-4 p. 9-12 / 9-5 p. 9-37 | Different table numbers; G106 MFS 2 alone does not supply a year grammar; TFMT still lacks the later usage column |
| 106-15 | Tables 9-2 p. 9-10 / 9-3 p. 9-18 / 9-4 p. 9-69 | TFMT prints Default A and TIMEIN applicability; AN1 prints explicit T-x ID presence condition; G106 prints two-digit-year guidance |
| 106-17 | Tables 9-2 p. 9-10 / 9-3 p. 9-18 / 9-4 pp. 9-69–70 | TFMT continuation carries enum/default; TTF also includes network time, not grounds to replace absent TFMT with an inferred value |
| 106-19 | Tables 9-2 p. 9-10 / 9-3 p. 9-18 / 9-4 p. 9-70 | Usage/default and network-format rows read with their own edition metadata |
| 106-20 | Same tables; printed chapter-relative pp. 10 / 18 / 70 | Intended policy edition versus current baseline vocabulary must remain labelled separately |
| 106-22 | Tables 9-2 p. 9-10 / 9-3 p. 9-18 / 9-4 pp. 9-71–72 | Read TFMT continuation, not a cropped page header alone |
| 106-23/24/24R1 | Tables 9-2 p. 9-10 / 9-3 p. 9-18 / 9-4 p. 9-72 | Baseline fixture bearings; inspected values/usage are not proof of universal historical coverage |

In inspected 106-15/17/19, the valid-information sentence lies in the R/R
status discussion at p. 9-5; the Default definition continues on p. 9-6.
For 106-20 use chapter-relative 5–6; 106-22/23/24/24R1 have the pertinent
R/R continuation and Default explanation on p. 9-6. The sentence supports
separating physical specification from its validity obligation IN THAT
recorder-status context; it is not a formal universal definition of specified.
The explicit AN1 presence condition is the separate operative example.
Earlier inspected AN1 rows give field definitions/MFS, not that same printed
usage condition. Missing historical usage metadata cannot be filled by analogy.
Likewise no Default A was located in the inspected 04–13 TFMT rows; this is
not a proof about every absent/intermediate edition or an implicit old default.

G106's last-two-digits wording is already printed in inspected 106-15;
106-13 lacks it, and 04/05/07 field-size metadata cannot be mistaken for that
format rule. Approved INT-015's correction governs, not the stale historical
onset claim in ADR-0028's context. This draft does not invent a new numeric
spelling rule for edition declarations: use their reviewed INT-014/015
binding, not generic index canonicalization or a guessed field kind.

Companions: RCC 123-16 p. 2-4 separates G106's TMATS edition from the setup
record's recording version. RCC 124-13 p. 5-31 and 124-17/19/22 p. 5-45
provide G-group/version examples; their entry-type, condition/default and
example material was searched/read as context. They do not define our
unavailable API state, malformed-name truth table or duplicate G106 recovery.
Sample C/parser code was neither copied nor run; examples cannot choose an
edition winner, repair input or override a cited row/owner interpretation.

## INT-041: availability is a guard, not fabricated field data

Resolve only with reviewed field scope, cardinality, local interpretation and
basis/coverage evidence. An unavailable definition/required validity operand
is lack of knowledge, not proof of data Invalid or absence Missing. Keep raw
candidates and reasons; do not invent a sixth value-state or settle a public
Rust/result/rendering shape. Known multiplicity remains Ambiguous, and a
proved Invalid failure remains evidenced even when another constraint is
unavailable. Never filter to a usable candidate or let a default win.

A single actual locally usable value is Explicit, not a claim that its
Allowed/Required, cross-field, packet or document checks passed. Baseline
a means A case-insensitively with raw a kept. Present TFMT B on a non-TIMEIN
entry remains Explicit plus applicability finding, never absence. Present
empty/control-only data stays present; validity follows its reviewed field
rule rather than a blanket empty-value rule. Leading blank letter TFMT stays
unavailable under INT-041; numeric fields/codes use INT-055's RAW digit/control
check, including its uniform leading-blank guard and retained proved defects.
A CR/LF inside NSB is not repaired into usable digits by INT-036 projection.

Defaulted requires ACTUAL established absence, a verified default with
applicability true and an available applied rule/basis. Keep source citation,
Default-column versus prose origin and condition evidence; no fake input
value span and no insertion into the ordered document. For a supported field
actually absent but without a verified applicable default, Missing records
why: no default, false condition or unknown condition/basis. The last is not
proof of inapplicability; consumers must inspect the reason. Entirely missing
field metadata or blocked/unexamined lookup is the unavailable-read guard,
not successful resolution as Missing. Zero exact matches plus related damaged
presence evidence does not prove the actual absence needed for defaulting.

For baseline TFMT missing with evaluated TIMEIN applicability, Default A is
not an explicit source value; at TTF=2 the existing inconsistency finding
stays, never a guessed network format or override of packet evidence. A
physical-required finding, when its reviewed rule applies, remains independent
of a default. Neither default nor explicit format establishes a data-to-time
association. No global field-count/default cache may leak between recorder,
channel, document, revision or applied-basis/coverage contexts.

Rejected alternatives: turn every unsuccessful read into Missing/Invalid;
conflate unknown and false applicability; default over malformed/duplicate
presence; trim blanks into a keyword; suppress proved failures behind missing
metadata. They hide uncertainty or source evidence. The approved guard costs
callers an explicit availability check, with retained diagnostic evidence.

## INT-042: physical evidence, scoped uncertainty and damaged-name guards

Evaluate a reviewed is-specified binding only in its established document/
parent scope with lookup available. At least one ACTUAL exact match gives
true despite empty, Invalid or duplicate values; keep all independent state/
record findings. True is not a singular usable operand, conformance or access
permission. A defaulted value supplies no physical item. No match gives false
only when compatible related damage/suffix evidence is absent; otherwise
cannot-evaluate. Value == and != both need usable permitted operands, not
Python/Rust-style missing-value inequality yielding a convenient true.
Is-not-specified complements only true/false; unknown remains unknown.

An unterminated literal suffix/prefix is diagnostic evidence only, never an
INT-035 lookup item. A framed damaged name with high bytes is likewise not
repaired into a key. After approved name projection/ASCII case handling,
readable characters remain in order, whole literals have no implicit gaps,
and each maximal high-byte run stands for zero or more unknown bytes. This
is conservative diagnostic compatibility, not probabilistic recovery:
I<FF>D can hide nothing, I<FF> can hide D, but D<FF>I cannot become ID.
A run that might hide a separator makes component alignment UNKNOWN; apply
INT-040's unassigned compatible-scope guard rather than pick one split.
Do not enumerate hypothetical parents or matching completions to exclude
uncertain evidence. Known contradictory parent/literal facts still exclude
incompatible scopes; unknown positions impose no extra restriction.

One exact match is enough for true even with additional compatible damage;
it is not enough to prove that every other possible field/candidate is absent
or that any scalar value is unique. Known scalar duplicates still make its
value Ambiguous and INT-048 aggregate qualification remains. Blocked semantic
lookup (for example INT-037 leading marker) is cannot-evaluate before any
zero-match inference; raw diagnostic inspection is not that usable lookup.
INT-036 omitted controls may form a projected exact key with information;
high bytes cannot be omitted to form one. Canonical index equality follows
INT-038, with normalization/collision evidence kept, never field-value width.

One assembled setup is one document under INT-039, including validly joined
fragments; separate records cannot satisfy each other's presence. A cache
must retain document/revision/scope and invalidation. A suggestion, checksum,
sole candidate or nearby item never supplies missing predicate/link scope.
This covers only reviewed presence and value operands: it does not select
compound Boolean semantics, abbreviated-reference bindings or cycle policy.

Rejected alternatives: require a valid singular value for presence; treat
unknown as false; use a printable-prefix/value-text search as a damaged key;
greedily split high-byte damage; let a default make a required physical item
present. Approved A keeps inconvenient true-plus-invalid and unknown-absence
results instead of silently changing what the writer supplied.

## INT-044: separate declaration ambiguity from rules selection

Count actual scalar G106 OCCURRENCES within one document, not distinct raw
years, usable values or the number of edition candidates per year. Two
identical occurrences remain two; mixed recognised/unrecognised values do
not choose a winner or change the reason to unrecognised. Preserve every
occurrence/location and duplicate finding; source-field Ambiguous is never
relaxed by equality, G\SHA or a rules override. Diagnostic potential completions
are not invented actual G106 occurrences. One usable year naming several
editions is INT-014's different case, not duplicate G106.

An explicit valid override wins for the BASIS only, with its own label and
all source ambiguity retained. An unusable override blocks, rather than
silently falling through. Without override, known duplicate G106 uses the
specific ambiguous-G106 fallback reason plus count. Use exactly INT-017:
eligible Format 1 0E -> intended 106-24R1, 0D -> intended 106-20, lower codes
follow that existing policy, and absent/unusable eligible evidence uses the
baseline. This is project policy, not a rewritten CSDW literal or a mapping
borrowed from Format 4/header fields. Eligibility requires a reviewed Format 1
layout/profile and usable evidence, not any word with a familiar low byte.

Keep intended policy edition and actual available registry vocabulary/coverage
separate. Current 0.1 vocabulary is the 106-24R1 baseline until edition deltas
exist; do not claim historical field rules were applied merely because the
intended basis names 106-20 or an older edition. If the needed binding/coverage
is unavailable, INT-041 blocks that read; missing coverage alone cannot prove
a historical Invalid/default from a baseline alias. Independently proved
common failures still remain under INT-041, with their actual basis/citation.
A reviewed baseline check retains its
actual baseline basis/coverage limitation. All derived defaults AND findings,
including Invalid results, keep the selection reason/label. Selection mutates
neither declaration nor source and certifies no global conformance.

INT-048 still makes duplicated fields assembly Ambiguous; a retained cut facet
keeps its blocking force. INT-031's per-query ambiguity-only access is separate
from basis selection, never implicit and never cut-text permission. Q25 remains
open; no precedence is settled by fallback. The existing ambiguous_g106/numeric
candidates JSON obligation is approved already, but this draft chooses no new
Q26 schema, display punctuation, label layout, severity or exit-code rule.

Rejected A would withhold all automatic basis selection for duplicates while
ADR-0028 permits fallback for missing/unrecognised declarations. Owner B avoids
that asymmetry without pretending a duplicate writer selected an edition. Its
cost is prominent, inescapable policy/coverage qualifications on every result.

## Independent planned oracles and remaining verification

All cases below are PLANNED, not executed parser/validator tests. Supply
reviewed field/condition/basis bindings explicitly; TFMT cases consume evaluated
R-N/CDT conditions rather than silently invent numeric parsing or link scope.
Independently fix raw byte intervals, candidates, scope and expected evidence.

| Planned case | Independent boundary / expected evidence |
|---|---|
| availability_default_needs_absence_rule_and_true_condition | No raw TFMT, known baseline binding/TIMEIN applicability: Defaulted A with source/citation/condition, no input value span or inserted item; remove any prerequisite and never invent A |
| availability_missing_false_is_not_unknown | Actually absent supported TFMT with false versus unknown applicability: both Missing with DISTINCT reasons; unknown CDT operand retains condition finding, not confirmed false |
| availability_present_empty_never_defaults | One empty or control-only candidate remains present; local validity follows definition, never default by empty-string truthiness |
| availability_proved_bad_or_duplicate_survives_gap | Known scalar multiplicity remains Ambiguous; demonstrated high-byte/range failure remains visible when another rule is unavailable; no candidate winner |
| availability_numeric_raw_controls_not_repaired | NSB raw 0 CR LF 4 is Invalid under INT-055, retaining control/source findings; no projection-first Explicit 4 |
| availability_tfmt_default_keeps_packet_conflict | Evaluated TIMEIN/TTF=2 plus absent TFMT: baseline Defaulted A with existing inconsistency finding and unchanged packet evidence; no association inference |
| availability_damaged_name_blocks_default | Reviewed baseline R-1\TFMT-1 binding and evaluated TIMEIN applicability, no exact match, but R-1\TFMT\xFF-1:a; (one raw FF byte): compatible damaged-name presence is cannot-evaluate. The effective read is unavailable/cannot-evaluate with raw item/locations retained, neither Defaulted A nor plain Missing; never repair the name to create an explicit field |
| presence_empty_invalid_duplicate_values_still_true | Available exact T-1\ID items with empty/Invalid/duplicate values: true presence; all candidate/field/record findings remain and no usable singular ID inferred |
| presence_exact_with_related_damage_still_true | One actual exact T-1\ID plus compatible damaged name: true, both evidence sets kept; reversing order changes no predicate answer or value-validity claim |
| presence_default_is_not_physical | Supplied reviewed default result with no raw item does not make that field physically specified; no synthetic input span or requirement satisfaction |
| presence_high_byte_has_independent_witnesses | I<FF>D -> ID via empty replacement; I<FF> -> ID via D; T-1<FF>D can hide separator plus I; D<FF>I has no ID witness; all remain diagnostic, not keys |
| presence_terminated_literal_has_no_unread_tail | Terminated T-1\I:a; without damage is not ID; unterminated T-1\I at EOF can complete to ID and blocks false without becoming a lookup item |
| presence_scope_projection_and_record_are_distinct | t-01\id projects to T-1\ID with normalization information; contradictory recorder occurrence/other setup never contributes; valid cross-fragment scope stays one document |
| presence_marker_block_precedes_zero_match | INT-037 leading marker blocks semantic lookup: cannot-evaluate with raw bytes/reason, never false/default from zero usable matches |
| presence_unknown_negation_and_inequality | Cannot-evaluate remains so under is-not-specified; missing/unusable value operands make both equality and inequality cannot-evaluate, never guessed Boolean |
| fallback_counts_items_not_years_or_candidates | G106 24/zz/24 in one document -> n=3, ambiguous reason; one 24 with two edition candidates is one source item, not duplicate; separate records never combine |
| fallback_override_never_upgrades_qualified_record | Valid override changes basis only; duplicate source/assembly Ambiguous and any cut stay. Unusable override blocks selection rather than falling through |
| fallback_wire_literal_policy_and_eligibility | Supplied eligible Format 1 0D/0E yields unchanged policy targets, literal readings kept; wrong/absent layout or other code-space operand never supplies eligibility |
| fallback_intended_edition_is_not_coverage | Intended policy 106-20 plus current baseline vocabulary retains actual coverage limit; unavailable field binding cannot claim a historical default/Invalid result or implicit alias |
| fallback_basis_survives_default_and_invalid | Baseline default and separately demonstrated Invalid under ambiguous-G106 fallback both carry ambiguous reason/count and applied-basis metadata; no report-shape or severity choice |
| historical_usage_is_not_silently_backfilled | 04–13 inspected TFMT/AN1 rows versus later explicit usage metadata: each fixture uses its own source/binding; no inherited modern default/condition or old field-size-as-range |

Literal byte-oracle checks (\xFF denotes ONE raw byte, not four printable
characters) are independent of any future parser/matcher:

| Input bytes | Fixed intervals / diagnostic witness |
|---|---|
| T-1\I\xFF:a; | Raw name [0,6), high byte [5,6), value [7,8); replacing only that run with D gives T-1\ID |
| T-1\I\xFFD:a; | Raw name [0,7), high byte [5,6), value [8,9); empty-run witness gives T-1\ID |
| T-1\xFFD:a; | Raw name [0,5), high byte [3,4), value [6,7); run witness is backslash plus I, so component alignment is unknown, never repaired |
| COMMENT:x;T-1\I | Only the first item terminates; related suffix [10,15) is not an explicit field/value |
| G\106:24;G\106:zz;G\106:24; | Three framed items [0,9), [9,18), [18,27); all three count, not two distinct values |

These witnesses prove only the stated possible character completion, not a
recovered key, value, scope, final record outcome or permitted access. A
small literal witness check is not the production compatibility algorithm.

Intensive executable work must cover every focused register case plus these
edges, with independent raw spans/candidate IDs and one source-qualified
fixture per archived edition touched. Add bounded properties over exact versus
damaged names, every high byte/maximal-run position, literal-prefix lengths,
canonical indices, candidate order/multiplicity and isolated document/revision/
basis changes. Actual matches make physical presence monotone under added
related damage; adding uncertainty alone cannot turn unknown into proved false.
Do not reuse the eventual matcher/resolver/selector to generate its own oracle.
Source tests cover Default-column and reviewed prose-default origin separately.
Per-edition missing coverage remains explicit, not a passing baseline stand-in.

Use independently listed high-byte replacement witnesses and raw intervals for
the named source shapes; no parser/matcher is implemented by this note. Future
fuzz work remains bounded in GitHub CI. Real recordings stay local-only and
were not accessed; source examples are not whole-file conformance fixtures.
No new dependency, numeric resource budget, compound-expression interpretation,
rendered result format or owner precedence decision is approved.

## Conclusion / executed checks versus gates

The inspected sources support keeping INT-041/042/044 as approved, with the
recorded historical coverage/context caveats. Only these three Analysis fields
link to this written draft; Design/Review sign-offs are unchanged. Written
analysis review by owner/independent reviewer and intensive executable tests
remain; dependency INTs' other analyses are not completed by context reads.

Executed locally: fresh manifest/PDF hash checks, bounded PDF text extraction/
passage reading and literal byte/witness/count/source-link/trace checks only.
No Rust build, parser/validator/reference execution, library test, heavy local
campaign, new install or dependency. All source jobs finish before reporting.
No executable/verified redesigned coverage is added. Independent exact-head
review is required; this group clears neither Q25 nor Q26 or implementation
readiness by itself.
