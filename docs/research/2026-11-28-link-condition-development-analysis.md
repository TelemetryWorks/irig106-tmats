# Development analysis: INT-001–008 — 2026-11-28

Written draft for eight APPROVED registry interpretations: three-source link
generation, source/target scopes and selectors, definition placeholders,
H/G test-item comparison, the FBCIN condition typo and same-C prose default.
Owner/independent analysis review and intensive executable tests remain required.
No approved behavior, Design/Review sign-off, source bytes, registry/code
implementation or requirement IDs change. Inspected main 9f94cdd after #94.

## Scope and source review

General link resolution remains 0.2 scope under L1-REG-005/L1-VIEW-006;
this analysis does not add R-to-time association evidence or bring the full
link graph into 0.1. The register, reviewed effective-value contracts and
bounded time-view states keep their existing release scopes. INT-008's
abbreviated condition/prose-default mechanism is context for that shared
resolver, not permission to implement a derived-parameter evaluator.

Re-fetched the archive manifest on 2026-11-28, verified 19 source PDF hashes,
and extracted/read pertinent definition/prose pages across fourteen archived
Chapter 9 editions plus RCC 123-16 and RCC 124-13/17/19/22. Whitespace/case
normalization was a search locator only, never a source edit or resolver.
Read row/continuation context separately; no rendered-image review is claimed.
A missing literal in the text layer is not proof that the concept is absent.
Unreviewed historical mappings remain unavailable, not guessed from spelling.

| Archived source | SHA-256 |
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

### Baseline source bearings (106-24R1)

| Source | Located authority / issue |
|---|---|
| §9.4.2 p. 9-2 | Framing, case-insensitive alphanumeric items and intentional blanks. No locale/Unicode fold, silent source rewrite or value-trim rule is created by that prose. |
| §9.5.1 a pp. 9-5–6 / b pp. 9-6–7 | Usage/keys versus relationship prose. The claim that all valid paths occur in Links fields is contradicted by the concrete rows below. Ties (g)/(h) describe R-to-B and R-to-S/Q. |
| Table 9-4 EV DATA LINK NAME p. 9-46; CHANNEL DATA LINK NAME / CDT p. 9-50 | R-side Links to P/B/S omit Q; same-channel CDT is the reviewed selector for CDLN. Event DLN scope is not inferred from CDLN's channel-index spelling. |
| Table 9-6 P-d\DLN p. 9-95 | Links to D-x\DLN and B-d\DLN; definition-table B index letter differs from Table 9-8. |
| Table 9-8 B-x\DLN / B-x\BT-i p. 9-124 | Incoming R/P/event links, plus 1553/A429 enumeration. Table spelling is B-x, not B-d. |
| Table 9-9 S-d\DLN p. 9-137 | UARTIN/MSGIN/ETHIN/FBCHIN accepted; no R subchannel-name edges in Links from. |
| Table 9-10 Q-d\DLN p. 9-146 | Incoming R CDLN/EV DLN; condition says MSGIN/1394IN/ETHIN/FBCIN. No literal FBCIN exists in the reviewed CDT enumeration. |
| Table 9-4 ANM p. 9-67, UCNM p. 9-78, MCNM p. 9-80, ENAM p. 9-81, CBM p. 9-82 | These rows define the particular subchannel/network names without Links to fields. CBM names CAN, which the inspected B/S/Q conditions do not accept. |
| §9.5.12 p. 9-171; §9.7 p. 9-177; Table 9-2 G\TA p. 9-10 | H\TA ties test item to G; H\ST-n is the other defined H attribute. G\TA is G's TEST ITEM, not B/S TA. IHAL context does not authorize invented H grammar. |
| Table 9-11 C-d\DCT p. 9-164; DPNO p. 9-167 | DCT lists DER. DPNO says Allowed when C\DCT is DER and places Default is 1 in Definition prose, not a Default usage field. DCT's own row is on p. 9-164 in the fingerprinted baseline; p. 9-165 continues conversion attributes, not that row. |

The citations bind these interpretations to the baseline rows, not every
historical layout. Source p. 9-164 for DCT corrects the broader earlier
review reference to p. 9-165; the earlier dated review record is unchanged.
No keyword meaning or owner behavior changes with this source-location note.

### Historical comparison and supplemental evidence

| Inspected editions/context | Positive observations and limitation |
|---|---|
| 106-04 full volume, Ch9 pp. 9-1–80 (PDF pp. 179–258) | R PCM link uses R-x\PDLN-n; H prose on p. 9-79 (PDF 257) names H-x\TA / H-x\ST-n. Do not turn those into baseline CDLN or index-less H names with an invented alias. |
| 106-05 / 07 | G/H/link framing and older table definitions are located; no DPNO literal located in the selected definition search. 07 has CDLN; a missing match is not an edition-introduction proof. No baseline Q target is backported. |
| 106-09 / 11 / 13 | DPNO and Definition-prose Default is 1 are located, with older table layouts: 09 p. 9-106 (PDF 108), 11 p. 9-92 (PDF 94), 13 p. 9-76 (PDF 80). Explicit modern usage fields cannot be inferred when the older layout does not print them. |
| 106-15 / 17 / 19 / 20 | DPNO/default prose and H\TA are located. DPNO p. 9-150 in 15, p. 9-151 in 17, p. 9-152 in 19; 20 text uses chapter-relative pagination. No Q-d\DLN/FBCIN literal was located in this search. |
| 106-22 / 23 | Q DLN at p. 9-145 and DPNO at p. 9-166. Each text layer has one FBCIN, in the Q condition; CDT says FBCHIN. |
| 106-24 / 24R1 | Q DLN p. 9-146, DPNO p. 9-167; the same one-FBCIN condition versus FBCHIN CDT discrepancy. |
| RCC 123-16 | Recorder/TMATS reference context, not authoritative link semantics or a replacement registry. No prototype/reference implementation supplies an otherwise uncited relation. |
| RCC 124-13 / 17 / 19 / 22 | General-information/link examples were located (e.g. PDF pp. 29/37 in 13, 41/43/56 in 17, 42/44/57 in 19, 43/45/58 in 22). They illustrate source naming, not new Q targets, H cardinality, missing defaults or an automatic alias policy. |

These are passage/search observations, not a generated edition-delta registry.
Historical source freshness does not mean historical resolver coverage exists.
INT-017/044 keep intended basis separate from applied baseline vocabulary;
missing binding is unavailable, not proof of Invalid or no link. 106-04's
indexed H declaration is a concrete arity difference, not an index-letter
placeholder variation. Defer its actual mapping to reviewed edition work.
Appendix/handbook examples may carry annotations and are not copied into
conforming TMATS fixtures. All proposed oracles here are synthetic/scoped.

## INT-001: recover the Q edge without fabricating its selector

Keep the R-side P/B/S statement, Q-side incoming CDLN/event statement and
§9.5.1 b(h) prose as independent evidence, even when they disagree. The
approved Q relation is supported by the incoming row and prose; using only
the outgoing R row loses a supported target. This is a registered reading,
not a general majority-vote algorithm for missing metadata.

The relation declares a possible namespace edge. Candidate resolution still
needs the source value, source scope, reviewed selector/cardinality and usable
target evidence. It does not make every R link point to Q or discard P/B/S.
CDLN selection under INT-005 consumes the same channel's CDT. Do not silently
reuse that selector for EV DLN based only on the shared suffix n: event and
channel index positions need their own reviewed binding. Retain that coverage
limit rather than invent an event-to-channel association.

## INT-002: prose edges are typed, not suffix matching

The approved names/target groups are ANM->B, UCNM->S, MCNM/ENAM->S or Q.
Tie each to the cited row and recorder/channel scope; a similarly named item
in another recorder or document cannot supply the missing source context.
When S and Q both contain eligible matching names, keep all candidates and
ambiguity, even if their values agree. Group/index/source order does not pick
the winner. Source-side and target-side missing Links fields remain visible.

CBM has no declared B/S/Q target in the reviewed baseline. An unmatched CAN
subchannel name is not an unresolved link for a relation that is not defined;
retain the name, raw evidence and any independent findings. This is not
permission to invent a new Q/CAN path from future editions, similar keywords,
packet type or the fact that the code name looks like another subchannel name.
Bus-type/other target constraints remain independently reviewed validation;
the group mapping does not invent a bus-instance selector from B\BT alone.

## INT-003: three sources, one reviewed disagreement record

Generate relationship declarations from outgoing fields, incoming fields and
prose ties, preserving which sources could state each relation. A missing
statement in an applicable source is not silent consensus. Require the
relation's interpretation entry and readiness gates; generation fails on an
unregistered one-sided relationship rather than making a best guess. Source
text, edition and mismatch evidence are retained even for an approved edge.

Do not treat 'all valid paths are documented' as an extractor completeness
proof. Prose-only H/G ties have no H definition table to fill in. An absent
column/source is represented as such, not repaired into an invented Links to.
Nor should two source strings be sufficient to activate an unreviewed edge:
INT-001/002/006 carry specific owner choices, not a general voting shortcut.
The generation invariant is about metadata edges, not deduplicating repeated
TMATS document items or resolving their scalar conflicts.

## INT-004: definition placeholders are not document identities

Compare definition references by group, literal code path and equal number/
positions of indices. B-d\DLN and B-x\DLN name the same definition shape;
B-d\DLN and B-x\DLN-i do not. Different groups or literal path parts also
stay different. Placeholder equivalence cannot erase an actual index, a nested
scope position, unknown private grammar or the 106-04 H first-level index.

Document indices follow INT-038's canonical digit-string identity with raw
spelling/collision evidence. Actual occurrence 1 and 2 are not equal because
the definition used x versus d; actual 01 and 1 share numeric index identity,
not because placeholder letters are interchangeable. Link VALUES such as
01 and 1 are names, not automatically numeric indices: this interpretation
never strips their zeros. The cross-group link still compares its reviewed
values/namespaces; it does not require equal P/R/B occurrence numbers.

## INT-005: same-channel CDT selects before candidate cardinality

For reviewed usable CDLN/CDT context, accepted target groups are PCMIN->P;
1553IN/429IN->B; UARTIN->S; MSGIN/ETHIN/FBCHIN->S or Q; 1394IN->Q.
Other reviewed channel types have no data-link target. This prevents a PCM
channel with equal P and B link names from being misresolved to the B group
that describes a bus embedded inside that PCM stream. Matching the value
before applying the selector is not enough; group overlap is intentional.

Missing, Invalid, duplicate/Ambiguous (equal repeats included), or unavailable
same-channel CDT yields cannot-evaluate selection, retaining all candidates,
all CDT occurrences and findings. It is different from a known usable type
whose reviewed mapping has no target. Do not borrow another channel's CDT,
infer PCMIN from the existence of P data, or return empty/no-target because
the selector cannot be evaluated. A target with invalid/unavailable identity
cannot become a usable match merely to avoid an unresolved result.

The Q/FBCHIN path explicitly depends on INT-007. A document CDT=FBCIN is
an unknown keyword under that reviewed enumeration, not corrected by the
standard-condition reading. No unsupported edition becomes Invalid merely
because it lacks a reviewed CDT definition. Usable type and available mapping
are separate prerequisites. General link cardinality, source namespace and
record qualifications remain intact after the type filter.

## INT-006: the H/G tie needs its full value and record evidence

The baseline test-item relation is H\TA to G\TA, not any attribute ending
in TA. B/S TA values are not substitute keys. Compare projected usable text
ASCII case-insensitively, keeping raw bytes/locations. No locale/Unicode or
high-byte folding is permitted; INT-036/037 handle character evidence.
Leading blank remains unavailable under INT-041. Other unreviewed blank-bearing
link values do not get silently trimmed or compacted into a match.

Multiple H\TA or G\TA scalar occurrences are Ambiguous even when equal;
list every one, never first/last wins. INT-048's assembly ambiguity and any
cut/encoding/availability facets remain attached. Link lookup is not permission
to govern, verify a digest or bypass INT-031's semantic-access/cut limits.
Raw H content is retained regardless of the interpreted tie's availability.

Only H\TA and H\ST-n are defined built-ins in the baseline prose. Other
organisation-defined H paths remain raw/unknown absent reviewed overlay
metadata, not guessed fixed cardinals, defaults or relationships. IHAL's
historical 'never implemented' context does not mean H source bytes may be
dropped. This analysis introduces neither an overlay implementation nor an
automatic historical H-x-to-H alias.

## INT-007: repair a printed condition, never the user's keyword

The four inspected 106-22/23/24/24R1 text layers each contain one FBCIN in
the Q DLN condition; CDT enumerates FBCHIN and S's condition uses FBCHIN.
Read that PARTICULAR standard condition as FBCHIN under the approved INT,
preserving original source text and interpretation evidence. Do not globally
replace FBCIN in documents, COMMENT, private fields or unrelated metadata.
A literal user CDT=FBCIN remains unknown under the reviewed enumeration.

If the Q definition or its reviewed condition is unavailable for an edition,
return unavailable/cannot-evaluate, not a successful correction or inferred
Q membership. Source-case normalization alone does not turn a misspelling
into a valid enumeration keyword. The corrected condition still uses the
source channel's usable CDT; duplicate/equal CDT does not become one operand.

## INT-008: same occurrence and prose origin are independent facts

C\DCT in the baseline C-d\DPNO condition binds to that same C occurrence,
not any DER in the document or a linked conversion with a matching name.
Default 1 comes from Definition prose, with that provenance and actual
edition/basis recorded. It is not a fabricated Default usage field or an
explicit value written into source. Missing metadata is not a no-default rule.

Default only with real absence and established applicable condition/default
under INT-041/042. Missing/Invalid/Ambiguous/unavailable same-C DCT cannot
prove DER; retain all operand evidence and withhold the automatic default.
A usable non-DER operand has a distinct false/not-applicable reason. Explicit,
invalid and duplicate DPNO items keep their own evidence; defaulting never
fills over them. Compatible damaged presence also blocks proving absence.

Two C occurrences with DER/NON remain separate when their numeric indices,
item order or raw case vary. Index 01 versus 1 follows INT-038 and can expose
same-slot ambiguity; it cannot make another C occurrence's DCT supply a value.
This analysis chooses no new DPNO numeric range from a printed character-size
cell and no new evaluation algorithm. The earlier prose default in archived
09/11/13 does not retroactively create modern usage metadata in those tables.

## Independent synthetic oracles and planned coverage

The source/target expectations below are handwritten from the approved
relations, not from a generator, production resolver or reference library.
Synthetic snippets assert isolated mechanisms, not whole-file conformance.
Use actual supplied reviewed definitions/condition bindings when implemented;
a synthetic adapter alone is not general registry/link acceptance.

- With R-2 CDLN-7 BUS and its CDT-7 PCMIN, P-9 DLN BUS and B-4 DLN BUS,
  the eligible group is P and its candidate is P-9, not B-4 or inferred P-7.
  A second distinct eligible P candidate makes the result ambiguous; do not
  deduplicate by equal value or pick source order.
- For raw `H\TA:TaNK;G\TA:tank;`, source length is 20 bytes; H name [0,4),
  H value [5,9), first item [0,10), G name [10,14), G value [15,19), second
  item [10,20). Interpreted value comparison agrees without rewriting any byte.
- In `C-1\DCT:DER;C-2\DCT:NON;`, absence of DPNO has a conditional prose
  default only for C-1 under the supplied applicable baseline binding. This
  does not insert a raw DPNO item into either occurrence.

| Planned case | Independently specified expected evidence | Requirements / interpretations |
|---|---|---|
| `link_analysis_q_edge_from_incoming_and_prose` | R outgoing P/B/S plus Q incoming/prose retains the approved Q edge and every source disagreement; no R-only graph loses Q. | L1-REG-005; INT-001 |
| `link_analysis_q_is_candidate_not_automatic_target` | Declared Q edge does not choose Q without reviewed source scope/selector/cardinality; P/B/S candidates stay available. | L1-VIEW-006; INT-001/005 |
| `link_analysis_event_scope_not_channel_by_suffix` | EV DLN gets its approved Q relation; CDLN channel selection is not copied to an event merely because its index has the same spelling. | L1-REG-005; INT-001 |
| `link_analysis_recorder_document_separation` | Equal DLNs in a different recorder/document never lend the source channel or unrelated scope operand. | L1-REG-005; INT-001/005 |
| `link_analysis_subchannel_target_groups` | Handwritten ANM->B, UCNM->S, MCNM/ENAM->S/Q expectations retain all source IDs and applicable bindings. | L1-REG-005; INT-002 |
| `link_analysis_s_q_equal_names_stay_ambiguous` | Eligible S and Q matching the source value remain multiple candidates even if every value is identical; no group/order winner. | L1-VIEW-006; INT-002 |
| `link_analysis_can_has_no_invented_link` | Baseline CBM/CANIN keeps raw name and unrelated findings but does not emit unresolved for a nonexistent B/S/Q relation. | L1-REG-005; INT-002 |
| `link_analysis_subchannel_no_generic_suffix_join` | Unknown vendor path ending NM/DLN does not acquire the registered subchannel relation or a guessed bus-instance selector. | L1-REG-005; INT-002 |
| `link_analysis_unregistered_one_sided_edge_fails` | Supplied one-sided relationship with no applicable interpretation fails generation/readiness; another edge being approved is insufficient. | L1-REG-006; INT-003 |
| `link_analysis_three_source_provenance` | Outgoing/incoming/prose memberships and each missing applicable statement survive generation; no repaired source column. | L1-REG-005; INT-003 |
| `link_analysis_prose_only_h_is_not_missing_table_repair` | H/G tie uses its registered prose evidence without inventing an H table or Links column. | L1-REG-006; INT-003/006 |
| `link_analysis_metadata_edges_not_document_dedup` | Consolidating reviewed metadata evidence never coalesces equal raw TMATS scalar occurrences or removes their ambiguity. | L1-REG-006; INT-003 |
| `link_analysis_definition_placeholder_equal_arity` | B-d DLN reference matches B-x DLN definition shape with equal group/path/arity, preserving source spellings. | L1-REG-005; INT-004 |
| `link_analysis_placeholder_wrong_arity_or_path` | Extra/missing index position, different literal path or group prevents definition-shape equivalence. | L1-REG-005; INT-004 |
| `link_analysis_document_indices_not_placeholders` | Occurrence 1 and 2 stay different; 01/1 follows INT-038 with raw/collision evidence, not definition-letter substitution. | L1-REG-005; INT-004 |
| `link_analysis_link_values_are_not_numeric_indices` | DLN values 01 and 1 are not made equal by the code-name index rule; historical H-x versus index-less H is not arity-equivalent. | L1-REG-005; INT-004 |
| `link_analysis_pcm_embedded_bus_uses_p` | Independent R-2/channel-7 PCMIN context plus P-9/B-4 equal BUS values selects P-9 namespace candidate, never B-4 or P-7 by index. | L1-VIEW-006; INT-005 |
| `link_analysis_all_reviewed_cdt_routes` | Handwritten PCMIN/1553IN/429IN/UARTIN/MSGIN/ETHIN/FBCHIN/1394IN route table excludes unrelated groups without changing raw items. | L1-REG-005; INT-005/007 |
| `link_analysis_missing_cdt_cannot_select` | Missing same-channel CDT retains every candidate group and cannot-evaluate, not resolved/empty or borrowed CDT. | L1-REG-005; INT-005 |
| `link_analysis_invalid_unavailable_cdt_cannot_select` | Independently Invalid versus unavailable CDT each prevents filtering while retaining operand reasons, candidates and findings. | L1-REG-005; INT-005 |
| `link_analysis_equal_duplicate_cdt_cannot_select` | Two scalar CDT occurrences with equal PCMIN are Ambiguous and cannot select P as if one operand existed. | L1-REG-005; INT-005 |
| `link_analysis_valid_no_target_not_unknown_selector` | A reviewed usable type with no link target differs visibly from missing/invalid/unsupported selector metadata. | L1-REG-005; INT-005 |
| `link_analysis_h_g_ascii_match_keeps_spans` | TaNK/tank values match with the fixed 20-byte source spans, unchanged bytes and the declared H/G namespace. | L1-VIEW-006; INT-006 |
| `link_analysis_h_g_blank_and_high_byte_guards` | Leading blank unavailable, other unreviewed blanks untrimmed, and high-byte text not Unicode/locale folded into a usable match; all evidence retained. | L3-VIEW-005; INT-006 |
| `link_analysis_h_g_duplicates_no_winner` | Repeated equal H TA or G TA remain Ambiguous with every occurrence; B/S TA never substitutes. | L3-VIEW-003; INT-006 |
| `link_analysis_h_unknown_fields_preserved` | Unreviewed private H fields remain raw/unknown without invented cardinality/default/link semantics; record/cut qualification stays attached. | L1-REG-006; INT-006 |
| `link_analysis_fbcin_is_source_condition_only` | Reviewed Q condition is interpreted FBCHIN while its original FBCIN source text remains; document CDT=FBCIN still has unknown-keyword/unusable-operand evidence. | L1-REG-005; INT-007 |
| `link_analysis_fbcin_scope_no_global_replace` | COMMENT/private values containing FBCIN stay exact; no blanket replacement or repair into a keyword. | L1-REG-006; INT-007 |
| `link_analysis_q_profile_unavailable_not_backported` | Unavailable historical Q binding or unsupported edition cannot acquire a baseline Q target or an invented alias from the typo interpretation. | L1-REG-005; INT-007 |
| `link_analysis_dpno_same_c_prose_default` | C-1 DER/C-2 NON, absent DPNO: default applies only to the supplied applicable same-C binding and retains Definition-prose provenance, not inserted source. | L3-REG-003; INT-008 |
| `link_analysis_dpno_operand_unusable_no_borrow` | Missing/Invalid/Ambiguous/unavailable same-C DCT prevents default; another occurrence DER never supplies it. | L3-REG-004; INT-008 |
| `link_analysis_dpno_present_or_damaged_blocks_absence` | Present invalid/empty/duplicate DPNO or compatible damaged presence never becomes a defaulted absence; keep original field/record evidence. | L3-VIEW-005; INT-008 |
| `link_analysis_dpno_canonical_scope_collision` | C-01/C-1 DCT occurrences use canonical index identity with collision/duplicate evidence, not an operand/default winner. | L3-REG-004; INT-008 |
| `link_analysis_historical_prose_not_usage_column` | Archived DPNO prose default remains distinguishable from modern Default usage fields; missing historical applicability metadata stays unavailable. | L3-REG-003; INT-008 |

Properties vary source order, ASCII case, recorder/document/C occurrence,
independent target namespaces, repeated values and canonical index spellings.
Keep reviewed definition provenance fixed when varying document values; vary
CDT usability independently from the candidate set. The independent graph
oracle is a handwritten source/target/evidence set, never generated output
compared with itself. Exact-byte replay is unchanged for every mutation.
Future per-edition fixtures need actual reviewed definitions, not treating
this historical search inventory as an executable edition map.

## Executed checks and next work

Executed locally: fresh manifest lookup and 19 PDF hash checks; bounded source
text extraction/read and supplemental searches; exact source-literal counts;
fixed byte-span and handwritten route-set sanity checks; matching planned
case/requirement/link checks; exact eight-Analysis-only register diff; trace
regeneration/--check and diff --check. Source jobs finish before reporting.
No graph generator/resolver, keyword validator, parser, reference, derived
expression evaluator or Rust suite was run. No new dependency/tool, publishing,
credential/settings operation or additional runtime code.

Only INT-001–008 Analysis fields link this draft. Design/Review sign-offs
remain unchanged; owner/independent analysis review and intensive executable
verification remain. Historical compatibility mappings and registry coverage
are not completed by passage reads; no new owner decision is needed for the
approved baseline choices here. The new historical H/PDLN observations remain
explicit source limits for later edition work, not silently chosen aliases.

Family two remains incomplete. Continue approved analyses in related groups
and remaining schema/request/strict-class derivations; family three has not
started. The future full graph retains its documented 0.2 scope, and no time
association or expanded 0.1 semantic-access permission is invented.
