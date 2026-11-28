# Development analysis: INT-061–066 — 2026-11-28

Six APPROVED interpretations share lossless preservation and explicit policy:
unindexed X mapping, registry vocabulary, vendor confirmation, and requested
ordering/newline/counter transformations. This written draft changes no
approved behavior, owner text or Design/Review sign-off. Owner/independent
analysis review and intensive executable tests remain required. Inspected
main 26cb450 after #96; no new owner decision is made here.

## Sources and historical limits

Fresh archive manifest fetched 2026-11-28; verified 19 PDF hashes and
re-extracted/read relevant Chapter 9 framing, V/X, counter and section-order
context across fourteen archived editions plus RCC 123-16 and RCC 124-13/
17/19/22. Text search only located passages; it is not private-code parsing,
source normalization or a rendered-image review. An absent locator is not
proof that a field/concept never existed. Historical definitions/arity still
need their own reviewed metadata, not an alias inferred from modern spelling.

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

### Baseline source bearings (106-24R1)

| Citation | Fact / interpretation boundary |
|---|---|
| §9.4.1–2 p. 9-2 | LF/CR can aid readability; attributes can appear in any order; names/values have delimiters and intentional blanks. No single standard canonical order/newline style or implicit source repair is specified. |
| Table 9-2 G\PN / G\DSI-n pp. 9-10–11 | G PN has no numeric index; G DSI has an inner source index without a G-x occurrence. Data Source ID links to V ID by textual source context, not equal occurrence numbers. |
| Table 9-7 D\MN p. 9-115 | Reviewed original name pattern for the X worked example, independent of document presence. |
| Table 9-7 MML\N / MNF\N / WP p. 9-117 | MML location count range 1–9999; MNF occupied word-position count range 1–8; explicit parent/child positions differ. Count is not highest index. |
| §9.5.13 p. 9-171 | Only V-x\ID/VN are standard headers; VN/path acronym is three characters, private suffix meanings are vendor-specific. The text does not define missing/duplicate/disagreeing header recovery. |
| §9.5.14 p. 9-172 | Right-of-ORGANIZATION existing-code matching, EXTENSION_CODE uniqueness, original-index matching and leftmost original arity, preservation on renumbering; X x need not be contiguous and cannot create originals. The text does not supply an index-less G mapping. |
| §§9.5.2–14 section order | G,T,R,M,P,D,B,S,Q,C,H,V,X is the source sequence for the owner-selected project profile, not a mandated serialization comparator. |

Owner sign-offs dated 2026-11-28 select INT-061/062/063 A, INT-064 A+D,
INT-065 A and INT-066 A. Final independent checks are recorded at #87
e0a0cce and the INT-064 amended recheck at 283ffd5. Sources provide facts;
the specific recovery/formatting permissions are owner policy. Their approval
is not completed analysis review or executable acceptance.

| Inspected historical/supplemental context | Positive observation / limit |
|---|---|
| 106-04 full volume | V ID/VN/private acronym form on p. 9-80 (PDF 258). Modern X form was not located in the Ch9 search. Do not invent an older X definition from current syntax. |
| 106-05 / 07 | V ID/VN and three-character/private-path context is located; modern X form not located. Source framing/order and original counter tables retain their actual layouts. |
| 106-09 / 11 / 13 / 15 | V/context and counter material located; no X-x/EXTENSION_CODE match in this search. Absence is a search/coverage limit, not a blanket rejection of retained X bytes. |
| 106-17 / 19 / 20 | Modern X form/index/uniqueness material located; V headers and counter scopes still need exact per-edition bindings. |
| 106-22 / 23 / 24 / 24R1 | Located V/X and counter rows plus modern section order. This does not make every private suffix or old renamed original a baseline registry code. |
| RCC 123-16 / RCC 124-13/17/19/22 | Supplemental counter/extension/vendor examples and recorder context cannot define the owner recovery policy, a private value grammar or a mandatory canonical transform. No reference source is copied or executed. |

The earlier [index/member analysis](2026-11-28-index-duplicate-counter-development-analysis.md)
and [time/numeric analysis](2026-11-28-time-choice-development-analysis.md)
keep their own source/coverage limits. Their context is not a completed
analysis of every dependency here. The profile's named v1 order is fixed
project policy, not dynamically replaced by an older edition's TOC.
Registry vocabulary is the labelled 24R1 baseline until reviewed deltas exist;
intended G\106/fallback edition and actual vocabulary remain separate facts.
No old DST/CDT or indexed/unindexed alias is approved by this analysis.

## INT-061: no fabricated occurrence for an index-less original

The standard allows G as an original group but does not say what X x means
for G\PN. Under selected A, preserve the X item, original/raw candidates
and missing-rule evidence, with cannot-evaluate. A unique PN occurrence does
not activate unselected C's labelled unique-path attachment; zero/one sentinel,
value/order-derived occurrence and the blanket-malformed alternative are not
chosen. This is uncertainty, not proof that G extensions are prohibited.

G\DSI-n still has no G-x outer occurrence. Its inner index does not silently
become the first-level original occurrence; that mapping remains unavailable
without its own reviewed binding. Ordinary indexed originals use their
separate definition/arity rule, never the value's numeric appearance.
No caller's desired attachment or transform request supplies missing grammar.

## INT-062: vocabulary before instance lookup

Use reviewed baseline original group/full path/index arity to decompose the
X name. EXTENSION_CODE uniqueness is part of proving a conforming split;
missing metadata, overlap or unproved uniqueness yields cannot-evaluate,
not a longest-prefix/greedy guess. This is more than finding a matching string
in the supplied document. Do not invent a concrete overlap definition that
the reviewed registry metadata has not established.

After a known split, search that same document for original occurrences.
No original means unresolved/orphan; one versus many follows the established
candidate/cardinality evidence, keeping every source location. Duplicate
values, order, checksum or a valid-looking target do not pick a winner.
An assembled multi-packet setup is one document; separate records cannot lend
their originals. Record/cut/encoding qualification stays attached.

The X-1/ D-1\MN-1-2 worked example uses the original's first occurrence and
ordered leading index tuple; canonical numeric identity accepts 01/1 while
preserving raw spelling/collision findings. Additional extension-private
indices are not interpreted as standard counters or D members. X never creates
an original/member; an orphan finding remains separate from counter evidence.

A's explicit cost remains: a private V code is not registry vocabulary even
when physically present. Standard V ID/VN differs from private V paths. No
hybrid document-defined vocabulary or path-presence fallback is active. A
missing baseline entry for an old renamed code remains unavailable with basis/
coverage evidence, not an automatic historical translation or no-target proof.
This analysis does not approve a complete registry or private parser.

## INT-063: observed namespace is not confirmed vendor authority

Retain every V occurrence, ID/VN header occurrence, private acronym namespace
and complete opaque suffix/value. Confirm attribution per namespace only
when one usable local VN agrees with that observed acronym. A good ABC header
does not relabel a disagreeing DEF namespace; a header in V-1 cannot fill
V-2. Source-ID/G-source candidates are separate textual context, not vendor
confirmation or an equality requirement between V x and G source index n.

Usable VN means one non-Ambiguous available INT-036 projected value of exactly
three characters under INT-037/041 guards, compared ASCII case-insensitively
per §9.4.2. Available ABCD is Invalid length and unconfirmed. Unreviewed
blank handling stays unavailable rather than being trimmed or manufactured
into a length verdict; high bytes are not folded into ASCII. Raw controls may
project away only under INT-036 with their located findings; raw source is
not changed. No added alphabet or INT-055 numeric rule is imposed on VN:
three-character 001 is not canonicalized to 1 merely because it is digits.

Missing/equal-duplicate/invalid/unavailable VN or path mismatch keeps attribution
unconfirmed with evidence, not a guessed path-first/header-first winner or a
new required-presence/default finding. Private values stay opaque, not clock
formats, measurement definitions or a generic scalar/contiguity grammar.
Inventory/confirmation never upgrades field/record validity, semantic access,
checksum or governing use. Header/source ambiguity survives any display grouping.

## INT-064: explicit permutation plus the named D profile

Default replay bypasses normalization even when ordering metadata exists.
A's mechanism validates a complete-region plan as a revision-bound bijection;
missing/repeated/stale/cross-document identities and unknown partition refuse.
Replay whole raw regions, including their assigned leading layout, retaining
explicit prefix/EOF anchors. No hidden deduplication, index renumbering,
uppercase rewrite or repair is implied. This is not an arbitrary patch API.

The named Chapter 9 group order, v1 profile is D: stable G,T,R,M,P,D,B,S,Q,C,
H,V,X within each reorderable run, with standalone COMMENT and opaque/
malformed/unknown-group barriers anchored. Complete positively grouped V/X
regions can be placed without knowing their private meaning or confirming
attachment/vendor; unavailable group/region classification anchors instead.
Keep the distinction between caller A plans and D's particular grouping/
barrier rules; do not turn D into a full-key sort or a default serializer.

A alone supplies no built-in canonical order; A+D meets the signed policy.
Anchors may prevent global group order and D preserves within-group source
order, so it is not the standard's unique canonical form. Profile identity
and source/revision correspondence remain reviewable. A transform's successful
permutation is not a conformance verdict on unknown/invalid source content.

## INT-065: positively established layout only

Require an explicit LF or CRLF target, never a platform default. CRLF is one
logical newline token even across borrowed-span/assembled-fragment boundaries;
source mapping still identifies the contributing raw bytes. Convert only
positively established layout ranges outside name/value content. Approved
name-edge layout may be included only with that known role; internal name
controls, COMMENT/value CR/LF and opaque regions are not whitespace cleanup.
A line break followed by apparent code-name text inside a value is still
value content unless the actual framing proves otherwise (INT-035).

Unknown layout stays unavailable rather than guessed, and field content can
retain mixed newline bytes: do not claim a global raw LF/CRLF convention.
No conversion deletes NUL/0x1A padding, strips BOM/high bytes, trims blanks,
adds a missing semicolon, or repairs INT-055 Invalid numeric controls.
Semantic projection is not an output plan. It remains useful for views while
the original and any proposed output retain their exact distinct bytes.

## INT-066: a safe rewrite needs independent member evidence

Require an explicit unique well-framed existing counter target, reviewed
applicability/parent/member scope, complete member evidence, contiguous
distinct identities 1..k, no compatible unassigned evidence and a range that
accepts k. Then write canonical decimal k. Missing/duplicate/unknown targets,
gaps, unknown evidence or out-of-range result refuse without source change.
Do not write the highest index, a partial count, a clipped maximum, new target
or synthetic/renumbered children. Use MML's own 1–9999 range, not MNF's 1–8.

Member identity comes from INT-038/040, not member VALUE validity. Spelling
variants/repeated properties count the same structural member once while
all collision/duplicate/invalid-value findings remain. X attachments never
create D membership. Compatible damaged/unassigned evidence blocks complete
membership for every compatible parent scope; do not select one convenient
scope to permit rewriting. Known child-scope inspection under INT-040 is not
by itself authorization to transform or silently repair a bad parent.

A malformed old counter VALUE may be explicitly replaced only when its exact
physical target and independently established scope/member evidence are known.
This does not validate the old value or become a generic number repair. A
missing defaulted counter is not an existing target. Preserve old findings
bound to the original revision and assess the changed output separately.
No private X/V counter semantics are inferred from a trailing index or N.

## Compound/revision boundaries

Transforms require explicit sequencing and checked old-to-new correspondence;
no guessed order between order/newline/counter operations or stale offset
retargeting after a permutation. Original source is immutable; no partial
guessed compound result is success. Retain exact unaffected bytes and source
qualifications. If bytes outside G\SHA exclusion change, don't reuse input
verification as output proof or silently restamp. Do not declare a checksum
stale or valid solely because an operation ran: establish its actual domain/
result under the separate checksum contracts. Stamp is separate, explicit and
final when requested; F1/damage/cut and other eligibility guards still apply.

These are text-library transformations, not a Chapter 10 recording rewriter,
filesystem acquisition/atomic-replace promise, automatic source/ID repair or
an association-proof rule. No new public Rust structs, CLI spelling, renderer
schema or hidden buffering/copying policy is chosen.

## Independent synthetic byte/count oracles

- Region input R2=`R-2\ID:r;`, G=`G\PN:g;`, R1=`R-1\ID:s;` has raw spans
  [0,9), [9,16), [16,25). With a known partition/no barriers, profile D emits
  exactly `G\PN:g;R-2\ID:r;R-1\ID:s;`: stable R2-before-R1, not key sort.
- For `R-1\ID:r;` CR LF `G\PN:g;`, length 18, supplied positive layout
  [9,11) converts to LF, yielding 17 exact bytes. Separately, CR/LF inside
  `COMMENT:a` CR LF `b;` stays intact; that example is field content, not the
  first oracle's suffix. A split CR|LF with the same logical layout mapping
  still produces one LF, not two.
- Existing `D-1\MNF\N-1-1-1:bad;` with independently complete members
  e=1,2 may have old value [16,19) explicitly replaced by `2`, range 1–8.
  Other source bytes and old findings remain; this is not validation of bad.
  e=1,3 refuses, as does k=9 for MNF; independently complete MML m=1..9 uses
  its distinct cited range and cannot inherit the MNF refusal.

| Planned case | Independent expected evidence | Requirements / interpretations |
|---|---|---|
| `ext_transform_analysis_unindexed_g_unavailable` | X naming G/PN plus a unique existing PN retains raw candidates and cannot-evaluate; no zero/one sentinel, value/order guess, unselected unique-path C or blanket-malformed D. | L3-EXT-007; INT-061 |
| `ext_transform_analysis_g_inner_index_not_outer` | G/DSI-n does not supply an invented G-x occurrence mapping; missing rule stays unavailable, with raw evidence. | L3-EXT-007; INT-061 |
| `ext_transform_analysis_known_code_orphan_not_grammar` | Known reviewed original pattern with no instance decomposes first and reports orphan/unresolved; another document cannot supply the original. | L3-EXT-004; INT-062 |
| `ext_transform_analysis_registry_not_document_vocabulary` | A physically present private V code or old renamed name cannot define the split, activate hybrid C or silently alias to the baseline; both intended basis and actual vocabulary stay visible. | L3-EXT-001; INT-062 |
| `ext_transform_analysis_overlap_no_greedy_split` | Unavailable registry/arity or established overlap/nonunique split remains cannot-evaluate with all source evidence, never longest-prefix recovery. | L3-EXT-001; INT-062 |
| `ext_transform_analysis_original_indices_private_tail` | Source D-1 MN-1-2 and X-1 MYORG D MN SGAIN-01-02 use canonical leading original identities, preserving raw collisions; extra private indices do not become counters/D members. | L3-EXT-003; INT-062 |
| `ext_transform_analysis_target_duplicates_keep_qualification` | Equal/invalid duplicate original occurrences stay ambiguous with all locations; attachment success cannot remove record/cut/encoding or field evidence. | L3-EXT-005; INT-062 |
| `ext_transform_analysis_vendor_agreement_is_local` | One usable ABC VN confirms only agreeing ABC namespace; DEF remains unconfirmed, and a header in another V occurrence cannot be borrowed. | L3-EXT-014; INT-063 |
| `ext_transform_analysis_vendor_missing_duplicate` | Missing/equal duplicate VN retains every source/header/private item with unconfirmed attribution, no required/default finding invented. | L3-EXT-010; INT-063 |
| `ext_transform_analysis_vendor_wrong_length` | Available ABCD VN is Invalid length and attribution unconfirmed, not a generic unavailable result or silent trim. | L3-EXT-014; INT-063 |
| `ext_transform_analysis_vendor_blank_high_byte_guards` | Unreviewed blank handling stays unavailable; high-byte VN is not folded; no guessed valid/invalid result from a trim. | L3-EXT-014; INT-063 |
| `ext_transform_analysis_vendor_projection_raw_preserved` | INT-036-projected three-character VN can compare ASCII case-insensitively with located control findings and unchanged source; no uppercase rewrite. | L3-EXT-011; INT-063 |
| `ext_transform_analysis_vendor_digits_not_numbers` | Usable three-character 001 VN/acronym stays 001; no numeric zero stripping, added alphabet restriction or fabricated source binding from V/G occurrence equality. | L3-EXT-011; INT-063 |
| `ext_transform_analysis_source_context_not_vendor_proof` | G source-ID candidates and V attribution remain separate textual evidence; missing/ambiguous source context never reassigns a private namespace or proves time association. | L3-EXT-012; INT-063 |
| `ext_transform_analysis_default_replay_unchanged` | Available metadata/profile never activates ordering/newline/counter transforms without an explicit request; every raw region remains source order. | L3-WRT-012; INT-064/065/066 |
| `ext_transform_analysis_order_bijection` | Known complete-region A plan retains every region exactly once and its raw bytes, explicit prefix/EOF anchors and revision correspondence. | L3-WRT-013; INT-064 |
| `ext_transform_analysis_order_bad_or_stale_plan` | Missing/repeated/cross-document/stale region identities or unknown partition refuse; no partial guessed result or deduplication. | L3-WRT-012; INT-064 |
| `ext_transform_analysis_group_profile_stable` | Fixed R2/G/R1 byte oracle emits G/R2/R1 in 25 bytes under D; no R-index/full-key sort or source rewrite. | L3-WRT-013; INT-064 |
| `ext_transform_analysis_group_profile_barriers` | D sorts only within runs between standalone COMMENT/opaque/malformed/unknown-group barriers, keeping anchors and within-group order; no global-order claim. | L3-WRT-013; INT-064 |
| `ext_transform_analysis_private_group_placement_not_meaning` | Complete positively grouped V/X regions keep raw private bytes/relative order without confirmation/attachment; unavailable group classification anchors rather than decodes. | L3-WRT-013; INT-064 |
| `ext_transform_analysis_newline_explicit_target` | Missing LF/CRLF target does not select a platform default; the supplied [9,11) layout oracle converts CRLF to one LF with all other bytes exact. | L3-WRT-014; INT-065 |
| `ext_transform_analysis_newline_field_content_kept` | COMMENT/value and internal-name CR/LF stay exact under layout normalization; code-looking text after a value line break is not a new boundary. | L3-WRT-014; INT-065 |
| `ext_transform_analysis_newline_seam_one_token` | CR and LF split across borrowed spans/assembled fragments but adjacent in proved logical layout remain one token, preserving contributing source positions without required coalescing. | L3-WRT-014; INT-065 |
| `ext_transform_analysis_unknown_layout_not_guessed` | Unproved layout ranges remain unavailable; mixed field newlines prevent a false globally uniform raw-newline claim. | L3-WRT-014; INT-065 |
| `ext_transform_analysis_padding_bom_not_cleanup` | NUL/1A padding, BOM/high bytes and blanks are not dropped or transcoded by newline/profile operations; unknown partition refuses rather than strips. | L3-WRT-014; INT-065 |
| `ext_transform_analysis_numeric_controls_not_repaired` | Numeric value 0 CR LF 4 remains raw/Invalid during layout-only conversion; no INT-036 projection used as output repair. | L3-WRT-014; INT-065 |
| `ext_transform_analysis_counter_exact_old_value` | Known unique MNF target with old value bad at [16,19), complete e=1,2 and applicable metadata permits explicit replacement by 2; old Invalid finding remains input-bound. | L3-WRT-015; INT-066 |
| `ext_transform_analysis_counter_gap_refuses` | Complete observed e=1,3 or 2,4 cannot be rewritten to highest or cardinality; no inserted/renumbered child to make it pass. | L3-WRT-015; INT-066 |
| `ext_transform_analysis_counter_each_range` | MNF k=0/9 refuses its 1–8 range; complete MML m=1..9 and boundary k=9999 use its own 1–9999 range, k=10000 refuses with no clipping; all other prerequisites remain, no borrowed MNF cap. | L3-WRT-015; INT-066 |
| `ext_transform_analysis_counter_missing_duplicate_target` | Missing/defaulted, duplicate/equal or unidentifiable counter target refuses; no insertion, first/last selection or new name repair. | L3-WRT-015; INT-066 |
| `ext_transform_analysis_counter_canonical_members` | Members with 1/01/2 spellings contribute distinct identities 1,2, not three; preserve duplicate/collision/invalid-value findings and every source occurrence. | L3-WRT-015; INT-066 |
| `ext_transform_analysis_counter_compatible_damage` | Damaged/unassigned member evidence blocks complete evidence for every compatible scope; no convenient parent guess or swallowed suffix ignored. | L3-WRT-015; INT-066 |
| `ext_transform_analysis_counter_x_not_member` | X attaching to or orphaning a D/WP target creates no D member and no private counter semantics; separate link findings remain observable. | L3-WRT-015; INT-062/066 |
| `ext_transform_analysis_counter_unknown_policy_refuses` | Unavailable applicability/range/scope or private count grammar cannot be guessed from trailing indices/N or a successful raw lookup. | L3-WRT-015; INT-066 |
| `ext_transform_analysis_compound_correspondence` | Explicit operation sequence retains checked source/output correspondence; stale offsets after permutation or unresolved overlaps refuse without a partial guessed result. | L3-WRT-012; INT-064/065/066 |
| `ext_transform_analysis_hash_domain_separate_stamp` | Changed outside-exclusion bytes do not reuse input verification/restamp; output domain/result must be established. An explicit final stamp retains F1/damage/cut guards. | L3-WRT-016; INT-064/065/066 |
| `ext_transform_analysis_original_and_qualifications_persist` | Original bytes/findings/record qualifiers remain input-bound; successful formatting/attachment does not mean valid fields, usable setup or governing/time association. | L3-WRT-016; INT-061/063/064/065/066 |

Bounded CI properties vary span/fragment partitions, case/index spellings,
registry availability, V header/path agreement, prefix/barrier placement and
member scopes/contiguity. Expected full output bytes, untouched-item sets,
old/new mappings, refusal facts and source findings are independently fixed;
the production splitter/vendor matcher/normalizer cannot generate its own
oracle. Real recordings stay local-only and were not accessed. No private
attribute meaning, alternative policy or arbitrary-length resource default
is supplied by these synthetic descriptors.

## Executed checks and remaining gates

Executed locally: fresh manifest/19 PDF hashes, targeted passage/TOC and
counter context reads, fixed raw-span/order/newline/count oracle checks,
matching planned cases/source/requirement links, exact six-Analysis-only
register diff, trace generation/--check and diff --check. Source jobs finish
before reporting. No splitter/registry/vendor resolver/normalizer/reference
or Rust suite was implemented or executed. No local heavy campaign, new tool/
dependency, publishing, credentials/settings or prototype retirement.

Only INT-061–066 Analysis fields link this draft. Owner/independent analysis
review, source/metadata coverage and intensive executable tests remain required.
The requirements' stale INT-063 proposed header is aligned to its already
recorded owner A/final concurrence; no new rule or approval is invented.
Other dependency analyses remain separate work. Family two remains incomplete;
family three has not started. Continue related analyses and remaining concrete
schema/request/strict-class derivations without silently activating alternatives.
