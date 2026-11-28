# Development analysis: assembly, closure and governing evidence — 2026-11-28

INT-011/028/029/030/031/032/034/045: eight approved interpretations whose
layers must remain separate. Inspected main 3a44c07 after #98. Only their
Analysis links are updated; no Design/Review, source quote or rule changes.
This is a written draft, not owner/independent analysis approval, executable
implementation coverage or permission to begin a full configuration timeline.
Q25 precedence is signed; Q26/report provenance is not changed here.

## Source reread and historical limits

Fresh archive manifest checked 2026-11-28. Re-hashed 32 unique archived PDFs:
Ch9 and packet chapters for 04/05/07/09/11/13/15/17/19/20/22/23/24/24R1,
sharing the full 04 volume, plus RCC 123-16 and four RCC 124 companions.
Read located joining/sequence, ordering/time, setup/change/event, framing and
case/blank/order passages. Text matching is only a locator: it is not a
packet classifier or evidence that a searched-for event does not exist.
Full-volume 04 reading is limited to its Ch9/setup/packet region; no modern
field/marker applicability is inferred from an absent old search hit.

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

### Source bearings, not a replacement historical router

| Archived source | Reading and availability boundary |
|---|---|
| 106-24R1 Ch11 §11.2.1.1 f p. 11-8 | Per-channel sequence counter; FF to 00 rollover. Channel evidence and actual observations matter, not adjacency in a filtered setup-only list. |
| Ch11 §11.2.7.2 pp. 11-50–51, Figure 11-34 | One setup may span consecutive packets with n+1; SRCC describes change within a session; changed setup precedes new/changed data; the event precedes the setup, not its end. No universal end marker or verified event identity is supplied by these passages. |
| Ch11 §11.2.3.2 p. 11-24 / §11.2.3.3 p. 11-26; Table 11-4 pp. 11-6–7 | First dynamic time packet and preceding static Format 1 ordering; explicit 11/12 type codes, separate reserved types. Time-source validity is separate from packet eligibility. |
| Ch10 §10.5.1/Table 10-9 p. 10-2 | Required initial setup/time/data order; does not by itself define a governing recovery or a backdated takeover rule. |
| Ch9 §9.4.1/9.4.2 p. 9-2 | Physical readability/projection, semicolon framing, intentional blanks, case-insensitivity and free attribute order; not blanket semantic equivalence. |
| 106-04/05 setup layouts | Whole reserved CSDW; no modern FRMT/RCCVER/SRCC reading is fabricated. Historical field availability remains the reviewed layout gate. |
| 106-07 Ch10 §10.6.7.2 pp. 10-71–72 / time §10.6.3.2 p. 10-46 | CH10VER and SRCC, but no FRMT; initial ordering present. Later commit/event wording is not present in this setup subsection; do not retroject it as a printed 07 requirement. |
| 106-09/11 Ch10 setup p. 10-68; time p. 10-44 | Located FRMT/SRCC and commit-before-data/preceding-event text; 12 remains reserved. |
| Archived 106-13 Ch10 time p. 10-44 (PDF 56), setup pp. 10-66–67 | First-time ordering, commit/event passages. Use archived e3740c7a… fingerprint above, not the irig106.org p. 10-41 pagination or a secondary mirror as primary evidence. |
| 106-15 Ch10 time p. 10-41, setup pp. 10-63–64 | Same relevant ordering/change evidence; 12 remains reserved. |
| 106-17 Ch11 time pp. 11-20/23, setup pp. 11-44–45 | Defined network-time Format 2; initial phase uses asserted session scope, not an inferred file boundary. |
| 106-19/20 Ch11 time pp. 11-23/25, setup pp. 11-46–47 | Located time/order/change evidence; version-code reuse is not a new closure/joining rule. |
| 106-22/23/24/24R1 Ch11 time pp. 11-24/26, setup pp. 11-50–51 | Same pertinent ordering/change evidence; neither periodic time nor event text prints the owner's universal SRCC=1 next-time terminator. |
| RCC 123-16 / RCC 124-13/17/19/22 | Supplemental packet/TMATS context; no heuristic event identity, end marker, semantic-equivalence or governing rule is adopted from a handbook or reference tool. |

The [historical layout review](2026-11-27-historical-csdw-layout-sources.md)
and [channel-use source review](2026-11-27-argument-contract-and-session-scope.md)
retain exact per-edition bindings. Setup-on-channel-0 starts in 106-11;
13/15 restrict channel-0 content to Format 1, 17+ permits Format 1/4.
Core owns these packet checks and legacy admission; tmats preserves evidence,
not a second router or a channel-based joining rule. Legacy distributed
candidates remain readable, but source-partition governance stays unavailable.

## INT-011: per-channel joining before any final classification

Maintain independent runs; equal channel, type 01, next sequence modulo 256
and matching decoded FRMT/RCCVER supply the approved within-phase join.
Compare literal fields, not equal intended fallback editions. Missing old
fields need their reviewed profile binding; do not invent modern zeroes.
Preserve each raw CSDW and every SRCC reading. SRCC is not a joining predicate
or an agreed record-level bit just because the other fields agree.

Other-channel ordinary traffic is provenance, not a join stop. A known
same-channel intervening packet stops the run even when a counter happens
to wrap back to a superficially adjacent value. INT-045's eligible time
boundary is a distinct layered reason and can be on any channel; this is
not a change to the per-channel adjacency predicate. Actual EOF stops joining;
input pause, budget refusal and unknown observations do not invent EOF.

Logical concatenation excludes EACH fragment's CSDW, headers, fill and packet
checksum. ADR-0036 uses borrowed non-contiguous DATA, with no required copy or
coalescing. Preserve span/packet origins across names, values, CRLF, BOM and
checksum boundaries. INT-037 offset zero is the assembled text's beginning,
not each packet's beginning; INT-055 raw numeric rules are not repaired at
seams. A valid join proves neither completeness nor an authoritative digest.

## INT-034: source facets, final outcomes and revisable observations

Boundary inspection uses exactly SP, 00–1F and DEL; it is not field trimming.
A semicolon followed only by that suffix can end otherwise Complete text at
actual EOF, labelled end of input. A substantive unterminated suffix cannot
be dropped to expose an earlier terminator; high bytes/BOM are not padding.
A present unknown/invalid/empty-valued attribute is not removed to manufacture
an empty document. Empty/layout-only bodies and terminated empty regions have
no substantive attributes under Q25: Incomplete, that reason, no truncation
claim. Semicolon itself is not a layout byte. Scanner findings remain.

Proven missing leading text retains a BLOCKING cut facet even if a final
semicolon, matching bytes/digest or duplicate-induced aggregate Ambiguous is
present. An ordinary input-start observation, nonzero first sequence or absent
metadata is not proof of loss. INT-048 established field ambiguity takes the
aggregate to Ambiguous WITH the cut facet: no ambiguity acceptance weakens
Incomplete's blocking force. Never choose a split from repeated scalars.

A gap is next != (previous+1) mod 256, not FF-to-00. Semicolon-ending gap or
profile-change split evidence retains competing readings; cut text retains
its cut. Missing required DATA bytes/bounds needs independently tied source
loss evidence. Do not invent missing run membership from a refused unknown
packet. If DATA is admitted with required extents available, core Mismatch,
NotChecked and secondary-integrity readings alone do not make text Incomplete;
pass them unchanged. Core admission/refusal/drain evidence and the assembler's
retention-limit error remain different operational facts, never fake cuts.
XML completeness remains unavailable, not a successful ASCII classification.

Open snapshots are explicit and versioned: extent/revision and invalidation
remain visible; a semicolon-ending v1 can change under v2 without finalizing
or automatically governing. Retained core pool credits are inside the budget
domain. Owner Q22 requires a verified progress cap/reserve and a typed assembly
budget exhausted failure if exceeded, with reader progress and truthful
ownership/release evidence. No numeric cap, one-block reserve, silent eviction,
required early copy or whole-heap bound is invented. This analysis does not
prove actual core APIs, reserve accounting or allocation behavior.

## INT-045: separate time-boundary layer, not a standard terminator

Initial adoption needs labelled caller-asserted session start, trustworthy
observations and static Format 1-only prefix before the first eligible time
packet. Any prior non-Format-1 packet is an ordering violation with withheld
adoption; Format 0/2/3/4 is not silently allowed as static setup. A first
untrusted time observation cannot be skipped in favor of a later trusted one.
Unknown/cropped starts or unavailable profile knowledge cannot prove a clean
prefix. A supplied file offset zero does not establish session start.

The time marker uses its reviewed edition profile: 11 is eligible in checked
07+ editions; 12 only from 17, reserved earlier. IPTS, secondary timestamps,
TFMT text and NTP/PTP inside other data never substitute. Source NONE is not
packet disqualification. Core trust/framing/required-integrity evidence must
establish the marker; this adds no local checksum or packet reader. Core's
evidence on channel misuse remains separate, including channel-0 time.

Mid-session eligibility uses FIRST-fragment SRCC=1, not last/OR/majority;
first0/later1 does not qualify. Repeated1 fragments do not restart the run.
The owner deliberately extends next-trusted-time closure beyond the printed
initial-ordering evidence; ordinary mid-session data is not the initial
prefix violation. Preserve earlier trust findings. Cut at the boundary stays
Incomplete or Ambiguous-with-cut, and later +1/same-profile text is continuous-
but-split, never joined backward. A later scalar is not a duplicate in the
sealed document merely because the run counter continues.

An otherwise Complete semicolon-ending seal is revisable to Ambiguous on
later same-channel +1/same FRMT/RCCVER, with both readings and located possible-
premature-seal finding. Independently verified G-SHA on sealed bytes alone,
or INT-046's approved bounded whole-byte identity, can retain otherwise
Complete classification WITH that finding. INT-048 field ambiguity/cut blocks
any upgrade; following-only checksum cannot verify the sealed copy. All
mid-session Complete results keep time-boundary seal (owner policy) basis.
A digest in continuation matching a joined hypothesis is diagnostic only,
not authoritative joined verification or permission to rejoin/use/govern.

No eligible time: open/live stays provisional within budget; actual EOF uses
INT-034. There is no lookahead timeout making a seal irrevocable and no
requirement to read the whole recording before using independently qualified
setup evidence. Consumers must preserve later revisions/findings before their
own publication contract; tmats does not invent a spool or I/O policy.

## INT-028: inventory order is not the takeover boundary

Compose approved closure/qualification/comparison/adoption evidence, never
latest Complete wins. Distinguish governing, ungoverned (no governing setup)
and unresolved governing/comparison target. Keep range/source provenance.
First-fragment order inventories interleaved runs; separately established
closure/takeover determines applicability. When a valid takeover closes at a
trusted time packet P, P is included; do not backdate to the last fragment.
TMATS text ends before P: including P in governing coverage is not including
P's bytes in the document or its hash. Earlier decoded ranges keep their
original setups, findings and any unresolved history.

INT-053 target is currently governing, never a non-takeover previous copy.
Complete/equality/SRCC/digest alone cannot restore an unresolved target; use
only a separately reviewed independent takeover proof. Supply such evidence
as an explicit test boundary input, not a fabricated production recovery rule.
Unverified F7 identity alone does not delay/block otherwise independently
valid takeover. Distributed/nonzero-channel candidates are retained/parsed,
but governing needs reviewed source partitions. Multiple simultaneous files
remain out of scope; a file boundary is not inferred session scope.

## INT-029/030: claims and event uncertainty remain evidence

Compare with the actual INT-053 target, retaining INT-050 raw subtype and only
independently established semantic class. Raw identical, SHA-value-only,
layout-only, other raw difference and unresolved comparison are distinct.
SRCC=0 with differing bytes is checked/reported as change, not assumed
unchanged; SRCC=1 with identical bytes retains the claim/equality evidence,
not a guessed semantic reset. Unreadable bit stays unreadable. Case-insensitive
keywords and free attribute order do not establish whole-document semantic
equivalence; private/unknown fields cannot be folded into a guessed class.
Proven descriptive-only comment/date changes are non-semantic under the
owner's rule, but only reviewed definitions can establish that premise.
Raw changes and findings remain; none chooses governance or cures ambiguity.

F7's named configuration-change event has no verified packet identity here.
A nearby Format 2 recording-event packet is a candidate only; do not assert
presence or absence by guessing a type. First-record SRCC=1, per-fragment
SRCC disagreement and the prospective meaning of prior stay separate.
First-fragment SRCC for INT-045 closure does not define a general event-check
aggregation. Offset proximity, RTC ordering or reference code is not an
approved identity/prior rule. Historical absence of a searched phrase is not
proof that an old recording violates a later requirement. F7 approval is
approval to report indeterminate pending evidence, not resolved analysis of
which event the writer meant.

## INT-031/032: semantic access is not extraction or governing proof

Fail closed by default on Incomplete/Ambiguous description building; do not
silently carry old setup forward. An explicit labelled per-query ambiguity-
only acceptance never removes the actual qualification, accepts cut text or
establishes takeover. The next query without that choice gets no remembered
permission. Integrity alone on otherwise eligible Complete uses INT-051's
prominent-finding default, not this refusal predicate or silent quarantine.

Ungoverned ranges retain locations, with no nearest/only/future setup default.
An explicit per-query caller-assigned description makes every derived answer
an assumption, not a declared setup or a fixed session default. Unsupported
XML retains its own limitation. Raw extraction is separately owner-authorized
byte-exact delivery under findings; it is not a usable description. Strict
extraction uses the reviewed bad-data contract, not an invented any-note
failure threshold. Text/JSON labels and source diagnostics keep Q26 provenance;
this draft defines no new wire field, exit precedence or severity.

## Independent bounded oracles and planned cases

Three synthetic text fragments (each supplied AFTER its own CSDW):
`G\PN:a` (6 bytes), `b;R-1\ID:r;` (11), and `\r\n\x00\x1a ` (5).
Logical text length is 22; first value crosses the seam, and retained
padding does not change its last substantive semicolon. Give each packet
text origin +4 relative to its supplied DATA origin: assembled [5,9) maps
to fragment 1 [9,10) and fragment 2 [4,7), preserving all four bytes `ab;R`.
CSDW bytes are excluded independently for EACH fragment. These are supplied
DATA-relative origins, not guessed fixed file/header offsets.

For sequence continuity enumerate all 256 prior counters with independently
listed expected successor: FF->00, 00->01, FE->FF. Also retain a known
same-channel intervening observation through a full counter cycle; numeric
coincidence does not hide it. Vary other-channel order independently.

For takeover, supplied input positions P0=start(A), P1=last fragment(A),
P2=ordinary data, P3=eligible trusted time boundary, P4=later data distinguish
inventory order from boundary. GIVEN independently established valid takeover,
new setup covers P3/P4, never P2 retroactively. A different witness B can
start after A and close first: inventory A/B; no completion-order sort.
This is a range oracle with supplied proof, not a general takeover algorithm.

The cases below remain PLANNED. Source hashes/byte arithmetic are executed;
no assembler, decoder, timeline, access API or packet reader is tested here.

| Planned case | Independent expected evidence | Requirements / interpretations |
|---|---|---|
| `assembly_governing_analysis_interleaved_runs` | A1/B1/A2/B2 with per-channel +1 and matching profiles yields A1+A2 and B1+B2; the inventory follows first-fragment order, not closure order. | L3-CH10-009; INT-011/028 |
| `assembly_governing_analysis_known_interruption_survives_rollover` | Every 0..255 sequence rolls to its next modulo 256; a known same-channel intervening packet stops joining even if a later observed counter happens to equal +1 after a full wrap. | L3-CH10-010, L3-CH10-011; INT-011 |
| `assembly_governing_analysis_csdw_not_rules_equivalence` | Same-channel +1 but different FRMT or literal RCCVER stops joining; equivalent fallback editions do not make unequal wire fields agree; SRCC disagreement is retained separately. | L3-CH10-010; INT-011 |
| `assembly_governing_analysis_seam_bytes_and_locations` | The fixed three-fragment oracle maps assembled [5,9) to two original packet ranges; no CSDW, filler, checksum or newline enters text; invalid ranges never clamp. | L3-CH10-012, L3-CH10-014; INT-011 |
| `assembly_governing_analysis_bom_and_numeric_seams` | Split EF/BB/BF still forms the offset-zero marker after CSDWs; a split numeric CR/LF stays raw and Invalid under INT-055, never repaired by assembly. | L3-CH10-012; INT-011 |
| `assembly_governing_analysis_closure_inclusive_not_backdated` | Supplied independently valid takeover seals at time packet P3: P3 can use new setup; P2 before closure is not retroactively governed by it; inventory order alone cannot adopt. | L1-CH10-008; INT-028 |
| `assembly_governing_analysis_unresolved_recovery_requires_proof` | Complete C2 with unresolved target does not resolve it by equality/SRCC/digest; a separately reviewed independent takeover proof can resolve to C2 for later C3, keeping earlier history. | L3-REG-016; INT-028 |
| `assembly_governing_analysis_f7_not_takeover_veto` | With all independent takeover guards established, F7 indeterminate alone neither blocks nor delays; no event-presence or event-absence claim follows. | L1-CH10-005; INT-028/030 |
| `assembly_governing_analysis_legacy_partition_not_guessed` | Admitted distributed/nonzero-channel legacy candidates retain bytes and independent runs; no governing source partition is inferred from channel, TSRC, DSI or order. | L1-CH10-008; INT-028 |
| `assembly_governing_analysis_governing_domains_distinct` | Established governing G0, ungoverned ranges and unresolved target evidence stay distinct; Complete alone and a successful candidate inventory do not select G0 or a new setup. | L1-CH10-008; INT-028 |
| `assembly_governing_analysis_srcc_claim_cross_product` | For raw identical/SHA-only/layout-only/other/independently semantic/unresolved comparisons, preserve first SRCC 0/1/unreadable without forcing a semantic reset or governing transition. | L1-CH10-008; INT-029 |
| `assembly_governing_analysis_descriptive_requires_definitions` | Only independently proven comment/date-only changes are non-semantic; raw differences remain, and private/unknown fields cannot be declared descriptive by name or case-folding. | L1-CH10-008; INT-029 |
| `assembly_governing_analysis_previous_copy_not_target` | A non-takeover C1 never becomes target for C2 by position; unresolved takeover keeps target unresolved; raw previous-copy equality remains information only. | L3-REG-016; INT-029 |
| `assembly_governing_analysis_equal_does_not_cure_duplicates` | Equal repetitions of a scalar or SHA retain every occurrence and aggregate Ambiguous; SRCC=0, equality or a digest never upgrades it. | L3-CH10-019; INT-029/034 |
| `assembly_governing_analysis_incomplete_comparison_no_subtype` | A prefix/cut following copy cannot prove byte identity or a SHA/layout-only subtype; preserve unresolved comparison, not unchanged. | L1-CH10-008; INT-029 |
| `assembly_governing_analysis_event_unknown_stays_indeterminate` | A nearby Format 2 recording-event packet is a located candidate only; the configuration-change event cannot be marked present or absent without a reviewed identity rule. | L3-CH10-024, L3-CH10-025; INT-030 |
| `assembly_governing_analysis_event_first_srcc_and_disagreement` | First-record SRCC=1 and per-fragment 1/0 disagreement are separate evidence; successful FRMT/RCCVER joining never proves SRCC agreement or event identity. | L3-CH10-025; INT-030 |
| `assembly_governing_analysis_event_prior_needs_own_rule` | Offset proximity or equal/decreasing RTC cannot define prior or choose event scope while identity and prior-ordering interpretations are unavailable. | L3-CH10-025; INT-030 |
| `assembly_governing_analysis_event_no_update_is_not_proof` | An observed generic event without a setup, or an update without a verified event, retains actual observations and unavailable check; no fabricated update or chosen event packet. | L3-CH10-025; INT-030 |
| `assembly_governing_analysis_access_default_no_old_carry` | Incomplete or Ambiguous candidate gives no description by default; do not silently carry old setup forward; cut bytes and findings remain inspectable/extractable under separate contracts. | L3-CH10-019; INT-031 |
| `assembly_governing_analysis_ambiguity_choice_per_query` | Non-cut Ambiguous: labelled explicit per-query acceptance does not change classification, establish takeover or persist into a second query without the choice. | L1-VIEW-008; INT-031 |
| `assembly_governing_analysis_cut_beats_all_acceptance` | Leading/middle/trailing proven cut, including aggregate duplicate-Ambiguous, remains unusable despite ambiguity opt-in, identical bytes, verified digest or final semicolon. | L3-CH10-029; INT-031/034 |
| `assembly_governing_analysis_integrity_not_incomplete` | Admitted, bounded otherwise eligible Complete text with core Mismatch/NotChecked or secondary readings retains evidence; integrity alone does not turn it Incomplete or silently quarantine it. | L3-CH10-019; INT-031/034 |
| `assembly_governing_analysis_assumed_description_is_per_query` | No initial usable setup or unsupported XML produces qualified ungoverned range; explicitly assigned description labels every derived answer assumed and does not persist. | L1-VIEW-008; INT-032 |
| `assembly_governing_analysis_no_nearest_or_future_setup` | Ungoverned input never borrows the nearest, only, later or old setup; a later valid takeover does not backfill earlier ranges or erase previous findings. | L1-CH10-008; INT-032 |
| `assembly_governing_analysis_extraction_not_semantic_access` | Owner-authorized byte-exact extraction can retain cut/ambiguous/encoding/integrity bytes and findings without creating a description or governing setup; strict extraction uses its separate bad-data contract. | L3-CLI-031; INT-031/032 |
| `assembly_governing_analysis_eof_terminator_and_padding` | The three-fragment oracle ends in semicolon plus CR/LF/NUL/1A/SP: at actual EOF otherwise Complete with end-of-input basis; input start alone is not leading loss. | L3-CH10-018; INT-034 |
| `assembly_governing_analysis_padding_only_not_truncation` | Empty/layout-only and terminated empty regions have no substantive attributes: Incomplete with that reason, not a truncation claim; invalid/unknown/present-empty attributes are not filtered away to make this case. | L3-CH10-028; INT-034 |
| `assembly_governing_analysis_boundary_padding_not_value_trim` | SP/control/DEL final-boundary check never discards an unterminated substantive suffix, high byte, marker or intentional value blanks; numeric control Invalid is not repaired. | L3-CH10-019; INT-034 |
| `assembly_governing_analysis_gap_vs_finality` | Sequence gap or profile change with semicolon-ending prefix exposes Ambiguous split readings; cut text retains cut/Incompleteness even if field ambiguity aggregates to Ambiguous; XML remains unknown completeness. | L3-CH10-017; INT-034 |
| `assembly_governing_analysis_snapshots_versioned_not_stops` | Open semicolon-ending v1 remains provisional; extension v2 invalidates the older observation visibly but creates no join stop, EOF, governance or usable cut snapshot. | L3-CH10-020, L3-CH10-021; INT-034 |
| `assembly_governing_analysis_availability_not_checksum` | Unavailable required run DATA/bounds can support cut only with independently tied source evidence; refused unknown packets are not fabricated fragments; core integrity and admission states remain unchanged. | L3-CLI-042; INT-034 |
| `assembly_governing_analysis_resource_failure_not_eof` | Retained pool credits and metadata enter the reviewed budget domain; exceeding progress cap gives typed assembly budget exhausted, releases/tracks actual ownership, and reader continues without synthetic Complete/EOF. | L3-ROB-016; INT-034 |
| `assembly_governing_analysis_initial_assertion_order_and_trust` | Asserted session start plus clean static-Format1 prefix and eligible first trusted time provides boundary; missing assertion, any non-Format1 prefix, or untrusted first time withholds adoption; later good marker cannot replace the first. | L3-CH10-018; INT-045 |
| `assembly_governing_analysis_time_marker_profile_guard` | 0x11 is eligible in checked 07+ profiles; 0x12 is reserved before17 and defined from17. IPTS/secondary timestamps/TFMT/NTP-in-data never substitute; NONE clock source is not marker disqualification. | L3-CH10-018; INT-045 |
| `assembly_governing_analysis_time_layer_any_channel` | Ordinary other-channel traffic never stops INT-011, but trusted eligible time on any channel closes under INT-045; marker bytes stay outside TMATS and can be governed at the inclusive takeover point. | L3-CH10-011, L3-CH10-018; INT-011/028/045 |
| `assembly_governing_analysis_mid_srcc_first_fragment` | First SRCC=1 seals at next trusted recognised time; first0/later1 is not eligible; repeated1 does not restart candidate; disagreement remains evidence and mid Complete carries owner-policy basis. | L3-CH10-018; INT-045 |
| `assembly_governing_analysis_late_continuity_revises_result` | Semicolon-ending seal followed by same-channel +1/same FRMT/RCCVER becomes Ambiguous with both readings and located premature-seal finding; no rejoin or timeout-finality. | L3-CH10-018; INT-045 |
| `assembly_governing_analysis_bounded_exception_finding_kept` | Verified sealed-only G-SHA or approved bounded whole-byte identity can keep otherwise Complete seal Complete, with premature-seal finding; duplicates/cut never upgraded; following-only digest cannot supply exception. | L3-CH10-018; INT-045 |
| `assembly_governing_analysis_continuation_digest_diagnostic_only` | Digest in continuation that matches hypothetical joined bytes is diagnostic only: no adopted join, authoritative joined verification, waived cut or takeover. | L3-CH10-018; INT-045 |
| `assembly_governing_analysis_no_time_no_manufactured_end` | No eligible time: live run remains bounded/provisional; actual EOF uses labelled INT-034. Input pause, source NONE, rejected marker or resource exhaustion is never EOF. | L3-CH10-020; INT-034/045 |

Generate bounded CI properties over fragment cuts (including every text-byte
seam), counter rollover, per-channel interleavings, SRCC states, source/profile
availability, qualification/cut facets and explicit caller choices. Expected
answers come from reviewed source/owner tables and fixed oracles, never the
production joiner/selector itself. Keep the existing 139,968-member pairwise
matrix and its 39,072 classes unchanged; it does not cover every timeline.
Bounded QM sequence fixtures retain recovery only with independent proof.
Local sample recordings were not accessed; real-data checks stay local-only.

## Readiness and executed checks

Executed: fresh manifest/source hashes, targeted source rereads, exact text
seam/range/counter arithmetic, matching case lists and requirement/link checks,
eight Analysis-only register diffs, trace generation/--check and diff --check.
Source extraction finished before reporting; no background job remains.
No Rust build/tests, reference execution, dependency/tool install, publishing,
credentials/settings change or prototype restructure occurred.

Rule sign-offs remain unchanged. Owner/independent ANALYSIS review, actual
source/profile/integration bindings and intensive executable tests still gate
implementation. The separate [retransmission analysis](2026-11-28-retransmission-development-analysis.md)
retains INT-046–053 evidence; [checksum analysis](2026-11-28-checksum-domain-development-analysis.md)
retains domain/verification guards. Neither completed review nor live packet
eligibility can be inferred from linking this note.

Entries without written analysis fall 15 to 7; no requirement is added and
zero of 487 remain verified. Family two is still incomplete; family three
has not started. Remaining seven analyses are INT-022–027/033; concrete
schema/request/strict-class derivations and independent review/tests remain
separate from writing these notes.
