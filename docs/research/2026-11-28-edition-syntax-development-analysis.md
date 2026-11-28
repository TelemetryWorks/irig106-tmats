# Development analysis: INT-009/010/012–017 — 2026-11-28

Eight existing interpretations share source-versus-interpretation boundaries:
scoped counter references, equality spelling, suspect example framing and
edition/wire declarations versus rules policy. Seven entries are accepted/
reconfirmed; INT-013 retains the owner's S1 SUSPECT status, not newly approved
real-data qualification. This written draft changes no rule or Design/Review
sign-off. Owner/independent analysis review and intensive executable tests
remain required. Inspected main 658c65b after #97.

## Sources, comparison method and limits

Fresh archive manifest fetched 2026-11-28; checked 32 unique PDF hashes:
fourteen Ch9 sources, fourteen packet chapters sharing the 106-04 volume,
and five RCC 123/124 companions. Re-extracted/read G\106, fragment/condition,
Appendix 9-C/9-E and located Format 1 version passages across those editions;
baseline header/Format 4 code spaces were read separately. No rendered-page
comparison or reference/compiler implementation is claimed. Text search is a
locator, not code recognition, framing repair or historical registry coverage.

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

### Source facts and historical bearings

| Citation/context | Located evidence / limit |
|---|---|
| 106-24R1 Ch9 Table 9-7 p. 9-117 | WFT/WFP Allowed when D\MNF\N > 1; WFP Range 1–D\MNF\N. The full same-parent counter is D-x\MNF\N-y-n-m, not an all-document count or WFP's default position. |
| Appendix 9-E E-2/E-3/E-5 and lexer example E-9 | Operator definition/precedence and grammar identify equality. Use E-6 p. E-3 as the agreed spaced occurrence and the explicit grammar '==' as the token authority; do not transplant C precedence or its lexer. |
| Appendix 9-C p. C-8; §9.4.2 p. 9-2 | Colon-ended MML/MNF example is kept as actual semicolon-framed input. Source examples do not authorize repairs or conforming-fixture claims. |
| Ch9 Table 9-2 G\106 p. 9-10 | Version/year wording, recommended leading zero and 0–99 range. One value can name multiple archived same-year editions; a publication date is not a new version code. |
| 106-05/07 Ch9 G\106 rows | Maximum field size 2 already exists; that size is not a 106-13 introduction or evidence for a year grammar absent from the text. |
| 106-13 / 15 G\106 | 13 lacks last-two-year-digits wording; 15 prints it on p. 9-10, before 17. No behavior gate is tied to a false 17 onset. |
| 106-04/05 Format 1 CSDW | Reserved whole word; not a declaration of a modern version merely because its low byte can be read. |
| 106-07 / 09 / 11 / 13 / 15 / 17 | Format 1 highest codes progress 07/08/09/0A/0B/0C, naming 07/09/11/13/15/17. SRCC appears in 07, FRMT in 09; layout is not inferred from the version byte. |
| 106-19 / 20 Ch11 | Highest code 0D names 106-19; 20 supplies no newer code. |
| 106-22 / 23 / 24 / 24R1 Ch11 | Highest code 0E names 106-22; F–FF reserved. Later reuse is a bounded archive observation, not an unlimited future-compatibility promise. |
| 106-24R1 Ch11 §11.2.1.1 e p. 11-6 / §11.2.7.5 Figure 11-43 p. 11-62 | Header Data Type Version 0A names 106-22; Format 4's table reserves 00–0B, names 0C as 106-17 and reserves 0D–FF. Neither supplies Format 1 fallback. Figure 11-41 is an index header, not that CSDW. |
| RCC 123-16 / RCC 124-13/17/19/22 | Version/recorder/derived/example context is supplemental, not authority to invent a missing code, alias, condition scope or repair. No reference source is copied/executed. |

Appendix 9-E and the recurring 9-C colon-ended pattern were located in the
inspected 17/19/20/22/23/24/24R1 text; missing older locators are not edition-
introduction proofs. Existing C-derived definitions in an older table do not
prove the modern grammar was available there. Each actual rule needs its
reviewed binding, applicable edition and source/coverage label.

**E-3 spacing caveat:** the current hash-identified PDF text extraction and
[official PDF text reader](https://www.trmc.osd.mil/wiki/download/attachments/335389015/Chapter9.pdf?api=v2#page=238)
render spaces between E-3's equals signs, unlike the joined transcription
recorded in the earlier independent check. No rendered-print claim or source
rewrite follows. E-6 plus the explicit grammar suffice for approved INT-010;
this analysis does not depend on E-3's exact extracted spacing or change the
historical review quote. Independent source/citation review retains that caveat.

### Shared-rule and year-candidate evidence

Fresh 24 and 24R1 PDF hashes differ as listed. Comparing all 252 per-page
text bodies after removing their running Telemetry Standards/Chapter 9 header
lines gives 252 exact matches, even before whitespace normalization. This
supports the already-reviewed shared-Ch9-rules choice for year 24; it is not
a generic rule that text extraction proves visual/semantic PDF equivalence,
a production edition-delta table or proof of parser conformance.

The current manifest locates 106-24 and 106-24R1, 106-96 and 106-00, and no
106-25 edition. Edition identity/year and retrieval/publication date are
separate. Bare manifest labels are discovery metadata: scanned/non-telemetry
records or no available Ch9 definition are not proof of supported TMATS rules.
A candidate name need not imply complete historical vocabulary/validation.
Preserve source-qualified availability rather than silently treating every
archive document as a fully supported TMATS profile.

## INT-009: bind the parent before reading the counter

For D-x\WFP-y-n-m-e or WFT, bind the count to the same (x,y,n,m) and omit
only the reviewed fragment position e from that metadata reference. This is
not generic suffix chopping or a literal lookup for malformed unindexed
D\MNF\N in document text. Another measurement/location/record cannot supply
the count. INT-038 canonical digit identity applies to parent indices, with
raw/collision findings; INT-055's field digit cap is not an index limit.

Use the scoped effective counter evidence. Missing, Invalid, Ambiguous
(equal duplicates included) or unavailable operands yield cannot-evaluate,
never true because some other count is >1, or false because the operand is
missing. In the baseline, zero/nine fail MNF's 1–8 range; do not compare a
proved Invalid nine as a usable >1. No WFP default-1 or observed highest
fragment index invents an MNF default/count. Default use needs declared
scope/policy, not this abbreviation rule.

For a usable own count 1, >1 is false; for 2, true. WFP's upper range operand
is the SAME scoped count. An inapplicable present field remains raw/local
field evidence with its applicable finding, not ignored as absent. This
condition result is not whole-scope conformance, record usability or permission
to recompute counters. The latter remains explicit INT-066 policy.

## INT-010: equality within its reviewed expression language

Under the available reviewed Appendix 9-E grammar, == is equality and = =
is accepted with the approved warning, preserving operator/raw source spans.
This does not authorize changing a TMATS delimiter, assignment-like example,
COMMENT or quoted operand by global string replacement. Read lexical role
before an operator rule; all original bytes remain default replay.

The extension fixes these forms, not every arbitrary whitespace/token repair.
Other lexical whitespace/projection details use their reviewed grammar and
INT-036/037 context, never a guessed strip-all-spaces normalizer. Numeric
constants in this expression language include its own decimal/hex/floating
forms; INT-055's decimal FIELD rule is not a replacement expression lexer.
Nor does the Appendix define the separate usage-condition grammar by analogy.
Keep unusual E-6 precedence and associativity/source dependencies explicit;
no C/host-language evaluator is introduced in production.

Derived expression parsing/evaluation remains its documented later scope;
0.1 raw reading does not gain an evaluator or expression-rewriting mode.
Unknown historical grammar coverage remains unavailable, not Invalid merely
because a baseline parser or common integer helper was borrowed.

## INT-012: literal codes have a code space and source table

For an eligible extracted Format 1 version field, use its approved literal
mapping, preserving raw byte/table/provenance. 0D says RCC 106-19 and 0E says
RCC 106-22. Notes name the observed reuse through 20 / 24R1 respectively;
do not replace those declarations with 20/24R1 or say 22-or-later.
A future/reserved code stays unknown; no 0F is invented for 23 or 24.

Header version 0A, Format 1 RCCVER 0A and Format 4 RCCVER 0A are different
facts: 22, 13 and reserved under the cited baseline tables. Format 4 0C=17
happens to agree with Format 1; that does not make one decoder/table universal.
Format 4 0D remains reserved, unlike Format 1 0D=19. This is a code-space
guard, not a new Format 4 reader: the host/core ownership and typed Format 1
adapter refusal remain; no header/Format 4 byte substitutes into INT-017.

## INT-013: retained suspect suggestion, never framing recovery

The actual first colon delimits the name; an actual semicolon ends the item.
In the C-8 sample MML value, later colons/code-looking names remain in that
ONE value until its terminator. Do not split into three explicit lookup items,
repair counters or make the excerpt a conforming TMATS fixture. Literal
annotation text in Appendix examples can become the next region's name.

The L1-READ-007 detector tests a colon INSIDE an established value, optional
blanks and a positively recognized COMPLETE following name with its own
colon, within that value's bounds. It does not require a line break. The
name/value delimiter itself and ordinary time/value colons do not satisfy
that predicate. Require the reviewed Appendix 9-C vocabulary/arity, including
MML/MNF/WP; no partial group-looking prefix or unreviewed private split.

Each positive case proposes a one-byte colon-to-semicolon edit with raw
location/revision evidence. The proposal stays unapplied; source, item count,
lookups and raw checksum intervals stay unchanged, with independent checksum
eligibility/qualification gates still applied by their owners. Unterminated
suffixes are skipped. Supplied cut-qualified diagnostics retain their blocking
facets and cannot make the record usable. Do not activate the separately
planned line-break-without-colon suggestion under this detector's name.

S1 remains suspect pending the authorized real-recording/false-positive review.
The title's 18 is a historical extraction-sensitive observation, not a required
match count or proof every warning is a real typo. No sample/reference access
or successful real-data qualification is claimed here.

## INT-014/015: candidate years are not field occurrences or syntax repair

Under an available reviewed G\106 binding, year 24 retains BOTH 106-24 and
106-24R1 candidates. Reviewed shared-rule evidence permits naming newest 24R1
as rules applied, not pretending the writer declared that exact revision.
Different/unproved rule equivalence needs its own review rather than an
arbitrary newest choice. A single scalar with two candidate editions differs
from two scalar occurrences, which remain Ambiguous under INT-039/048/044.

The existing year-reader rule reads 4 and 04 as year 04, retaining raw spelling
and the note for omitted recommended leading zero. Literal name component
106 is not an index, and a year-reader is not INT-038 index canonicalization.
Generic numeric parsing alone does not recognize editions: actual G\106
representation/range/lexical findings come from its reviewed definition and
any applicable INT-055 dispatch, not an assumed grammar from digits alone.
This analysis pins existing year-reader behavior without widening the set of
accepted spellings or suppressing independent field-state evidence.

Only archived year candidates are recognized by that policy; 106-05, 2005,
IRIG 106-05 and revision-label spellings are not silently converted to 05/24.
When reviewed as unrecognised, preserve the exact declaration and apply the
separate labelled fallback contract; do not strip prefixes, truncate to the
last digits or guess a century. Unavailable/high-byte/blank/unknown binding
is not automatically proof of an unrecognised data value or missing field.

The last-two-digits wording exists in inspected 15, not first in 17. Maximum
field size 2 already exists in 05/07 and does not prove the missing old format.
24R1's January 2025 publication does not create 106-25 or make G\106=25 its
code. A numeric zero is not a universal missing sentinel: under an available
year binding, 00 can locate archived 106-00, unlike a wire version 00's
not-declared meaning. Candidate identity does not establish historical-rule
coverage; override/default/availability and basis labels stay separate.

## INT-016: field presence precedes latest-table interpretation

04/05 all-reserved words carry no version declaration; never extract a modern
recording-version field from reserved raw bits. 07 introduces CH10VER and
SRCC, with FRMT only appearing in 09. Profile/layout availability and trusted
source extents come from their reviewed integration contracts, not from
looking at a raw low byte or choosing a G\106 edition.

After eligible field extraction, defined codes use the newest approved table
as INT-016 specifies. Do not confuse that policy with claiming every code
was assigned in an older table. 00–06 report not declared/reserved, not a
new error; F–FF are unknown edition. Independent core integrity/conformance
facts remain unchanged; this group chooses no new cross-declaration conflict
or trust rule, no zero-padding of a short CSDW and no invented packet fields.
Version declares recording format, never the TMATS text edition.

## INT-017: labelled policy is not a wire declaration

| Format 1 code | Literal reading | Intended fallback edition |
|---|---|---|
| 07 | 106-07 | 106-07 |
| 08 | 106-09 | 106-09 |
| 09 | 106-11 | 106-11 |
| 0A | 106-13 | 106-13 |
| 0B | 106-15 | 106-15 |
| 0C | 106-17 | 106-17 |
| 0D | 106-19 | 106-20 (owner policy) |
| 0E | 106-22 | 106-24R1 (owner policy) |

Do not implement newest-still-uses-code by searching any later table for an
old code: later tables keep old entries, but 07–0C still use the named intended
basis, and 0D is not 24R1 merely because that table includes it. The fixed
approved reuse epochs/policy differ from arbitrary future compatibility.

With no override and established missing/unrecognised G\106, eligible
Format 1 RCCVER supplies this fallback; no usable eligible RCCVER supplies
the labelled baseline policy instead. A readable header/Format 4 code is not
an eligible replacement. A usable G\106 basis or valid explicit override has
its own precedence; unusable override blocks rather than falling through.
A G\106/wire-version difference alone is not a conflict finding.

INT-044's already-approved ambiguous-G\106 case uses the SAME policy and a
distinct ambiguous reason counting all occurrences, even mixed zz/24 or equal
24/24 values; no declaration winner is picked. This dependency is not a new
fallback reason or mapping introduced here. All declared facts/findings stay.

Until reviewed edition deltas exist, the actual registry vocabulary is the
24R1 baseline while the label records the policy's intended edition. Do not
claim actual 13/20 rules were applied merely from that intended label, or turn
missing coverage into a proved Invalid/default. Defaults/findings keep the
fallback basis; no rule choice overwrites G\106, CSDW or packet provenance.

## Independent synthetic oracles and planned coverage

- Raw `A==B` has operator [1,3); `A= =B` has operator [1,4). With the supplied
  reviewed formula grammar both denote equality, the latter with warning;
  original bytes and operator locations differ and are never normalized in replay.
- Raw `D-1\MML\N-1-1:2: D-1\MNF\N-1-1-1:1: D-1\WP-1-1-1-1:14;`
  is 54 bytes and ONE item: name/value delimiter 13, value [14,53), terminator
  53. Given independent full-name recognition, suspect replacement spans are
  [15,16) and [34,35); recognized names [17,32) and [36,50) stay inside that
  value, not new lookup items. This is a source-shaped negative conformance
  example, not proof the writer intended either suggested edit.
- `G\106:24;G\106:zz;` consists of [0,9), [9,18), values [6,8), [15,17).
  It has two scalar occurrences, not one year with two edition candidates;
  under INT-044 ambiguity reason/candidates=2 remains separate from any field
  fault and fallback/override basis. No renderer/schema layout is invented.

| Planned case | Independent expected evidence | Requirements / interpretations |
|---|---|---|
| `edition_syntax_analysis_counter_own_parent` | Count 1 in location m=4 and count 2 in m=5 yield false/true >1 only in their own (x,y,n,m); WFP upper bound uses that same count, never another location/record. | L3-REG-005; INT-009 |
| `edition_syntax_analysis_counter_canonical_scope` | Parent 01/1 identity follows INT-038 with collision evidence; equal duplicate scoped counts stay Ambiguous, not an operand winner. | L3-REG-005; INT-009 |
| `edition_syntax_analysis_counter_unusable_is_unknown` | Missing/Invalid/Ambiguous/unavailable MNF, including out-of-range 0/9, gives cannot-evaluate, not false or a comparison of an unusable numeric nine. | L3-VAL-009; INT-009 |
| `edition_syntax_analysis_position_default_not_count` | WFP default position 1 or observed highest fragment never creates MNF default/count; a false Allowed condition does not discard a present field. | L3-VAL-011; INT-009 |
| `edition_syntax_analysis_equality_both_spellings` | Fixed A==B/[1,3) and A= =B/[1,4) raw operators share equality meaning under supplied grammar; spaced form warns, neither rewrites source. | L1-DER-002; INT-010 |
| `edition_syntax_analysis_operator_role_not_substring` | COMMENT or a reviewed quoted-operand token is not rewritten by finding = =; no assignment/framing repair or strip-all-spaces operator recovery. | L1-DER-002; INT-010 |
| `edition_syntax_analysis_expression_grammar_own_constants` | Full Appendix expression constants/precedence use their reviewed language, not INT-055 decimal FIELD syntax, host C precedence or usage-condition grammar. | L1-DER-002; INT-010 |
| `edition_syntax_analysis_unavailable_old_grammar` | Missing historical formula grammar stays unavailable/raw, not Invalid from a borrowed baseline parser or a new 0.1 evaluator. | L1-DER-002; INT-010 |
| `edition_syntax_analysis_literal_wire_mapping` | Eligible Format 1 07–0E codes map to the exact listed editions; 0D is 19, 0E is 22, F–FF unknown, with raw/table identity preserved. | L3-CH10-007; INT-012 |
| `edition_syntax_analysis_reuse_is_bounded_note` | Reuse notes identify inspected 20/23/24/24R1 only, never declare 22-or-later or invent code F for a later edition. | L3-CH10-008; INT-012 |
| `edition_syntax_analysis_three_code_spaces` | Same byte 0A reads header=22, Format1=13, Format4=reserved under the cited baseline tables; Format4 0D never borrows Format1=19. | L3-CH10-008; INT-012 |
| `edition_syntax_analysis_format4_guard_not_reader` | A Format4/header fact cannot enter the Format1 fallback path or bypass the observable typed Format1 adapter refusal; no new reader is qualified. | L3-CLI-027; INT-012 |
| `edition_syntax_analysis_colon_oracle_not_three_items` | 54-byte source-shaped sample remains one item/value [14,53); only proposals [15,16)/[34,35), never explicit MNF/WP lookups or applied edits. | L3-READ-035; INT-013 |
| `edition_syntax_analysis_complete_name_required` | Only supplied complete reviewed names/arity can satisfy the lookahead predicate; missing Appendix9C coverage is unready, not silently skipped or prefix-guessed. | L3-READ-034; INT-013 |
| `edition_syntax_analysis_ordinary_colons_negative` | Ordinary Time 12:30 and group-looking partial/private syntax do not trigger without the whole-name/own-colon predicate; do not reinterpret the item delimiter. | L3-READ-034; INT-013 |
| `edition_syntax_analysis_suffix_skip_cut_preserved` | Remove final terminator: no established value for this detector, suffix skipped. Separately supplied cut-qualified terminated text keeps its cut/qualification on any diagnostic suggestion. | L3-READ-035; INT-013 |
| `edition_syntax_analysis_s1_not_closed_by_count` | Source/synthetic tests or an extraction-sensitive count 18 never close S1, prove intended repairs or enable the separate future line-break suggestion. | L3-READ-038; INT-013 |
| `edition_syntax_analysis_24_candidate_set` | One year 24 retains 24 and 24R1 candidates; reviewed shared-rule evidence permits naming newest applied rules, not a precise declared R1 identity. | L3-EDN-002; INT-014 |
| `edition_syntax_analysis_year_leading_zero` | Available reviewed year binding reads 4/04 as year04 with raw spellings and missing-recommended-zero note; independent field syntax/range findings are not erased or replaced with index canonicalization. | L3-EDN-002; INT-014 |
| `edition_syntax_analysis_year_zero_not_wire_zero` | Available year binding can locate archived 00/96 identities while keeping historical rule coverage unavailable where needed; year00 is not wire00 not-declared. | L3-EDN-001; INT-014/016 |
| `edition_syntax_analysis_rule_equivalence_required` | Multiple candidate editions with different/unproved shared rules are not arbitrarily newest-selected; source-text comparison is not a generic semantic-PDF equality oracle. | L3-EDN-002; INT-014 |
| `edition_syntax_analysis_scalar_vs_same_year` | One 24 item/two candidates differs from duplicate/equal 24 items; all occurrences remain Ambiguous with n=2 and the distinct fallback reason, no collapse. | L3-EDN-006; INT-014/017 |
| `edition_syntax_analysis_year_format_onset` | Source 15 wording is recognized as present before17; 05/07 MFS2 is not new in13 and does not itself supply the missing old grammar. | L3-EDN-002; INT-015 |
| `edition_syntax_analysis_other_forms_raw_unrecognised` | Under reviewed reader, 106-05/2005/IRIG 106-05/revision-label forms are preserved unrecognised, not prefix-stripped, last-digit-truncated or used as a data-repair rule. | L3-EDN-001; INT-015 |
| `edition_syntax_analysis_publication_not_edition_year` | 24R1 January2025 publication does not create archive106-25 or make declaration25 its code; manifest retrieval dates never select a year. | L3-EDN-002; INT-015 |
| `edition_syntax_analysis_unavailable_binding_not_bad_value` | Unknown historical binding/blank/high-byte interpretation retains original availability/field evidence, not guessed unrecognised/missing data to force fallback; applicable numeric dispatch is not a year-recognition shortcut. | L3-EDN-002; INT-015 |
| `edition_syntax_analysis_pre07_no_field` | 04/05 all-reserved CSDW contains no recording-version declaration, even if low raw bits look like a modern code; original reserved/core evidence retained. | L3-CH10-008; INT-016 |
| `edition_syntax_analysis_low_reserved_no_new_error` | Eligible 00–06 field codes are not-declared/reserved with raw evidence, never a new tmats error or zero-to-G106 inference. | L3-EDN-001; INT-016 |
| `edition_syntax_analysis_profile_before_latest_lookup` | Reviewed field presence/extents precede newest-table lookup; raw low byte or G106 cannot choose a historical layout, fabricate short-word bytes or invent metadata fields. | L3-CH10-008; INT-016 |
| `edition_syntax_analysis_fallback_fixed_epochs` | Missing/unrecognised G106/no override and eligible Format1: 07–0C intended named editions, 0D->20, 0E->24R1 as policy; later-table presence of old codes never maps them all to24R1. | L3-EDN-005; INT-017 |
| `edition_syntax_analysis_actual_vocabulary_label` | Intended13/20 fallback versus actual baseline24R1 vocabulary remains explicit; missing historical coverage cannot claim old rules, a default or proved Invalid. | L3-EDN-005; INT-017 |
| `edition_syntax_analysis_declared_override_precedence` | Usable G106 or valid explicit override retains its proper precedence/label and all wire/source facts; unusable override blocks rather than falls through, with no automatic version-conflict finding. | L3-EDN-003; INT-017 |
| `edition_syntax_analysis_no_eligible_rccver_baseline` | No usable eligible Format1 RCCVER yields labelled baseline policy only when other fallback prerequisites hold; readable header/Format4 bytes are not substitutes or declarations. | L3-EDN-005; INT-017 |
| `edition_syntax_analysis_mixed_duplicate_reason` | zz/24 or equal24/24 keeps every occurrence and ambiguous reason count2 before individual value recognition; same INT017 policy, not missing/unrecognised reason or a winner. | L3-EDN-006; INT-017 |
| `edition_syntax_analysis_qualifications_not_upgraded` | Basis/candidate/operator/suggestion success never upgrades cut/ambiguity/core integrity, invents a time association or authorizes source normalization. | L3-CH10-008; INT-009/012/013/017 |

Bounded CI properties vary canonical parent spellings, operand states and
locations independently; raw operator spelling/lexical role with fixed grammar;
colon-rich source spans with independently listed recognizer candidates; year/
field occurrence/rules basis; and identical wire bytes across explicitly
separate code spaces/profiles. Never let the production recognizer, decoder,
selector or parser generate its own expected answer. Real recordings remain
local-only and were not accessed; S1 qualification still needs its review.

## Executed checks and remaining gates

Executed locally: fresh manifest/32 PDF hashes, targeted source passage and
code-space reads, whole 252-page text-body comparison after running-header
removal, archive-candidate metadata checks, fixed literal byte/scope/code
oracles, matching planned cases/links and exact register-diff checks, trace
generation/--check and diff --check. Source jobs finish before reporting.
No parser/decoder/reference implementation, Rust suite or local heavy campaign,
new tool/dependency, publishing, credentials/settings or prototype retirement.

Only these eight Analysis fields link the written draft; seven accepted/
reconfirmed designs and S1's suspect status are unchanged, as are all Review
sign-offs. Owner/independent analysis review, source/actual binding coverage
and intensive executable tests remain required. Current requirement pointers
for INT-008/009's completed rule checks and owner-signed INT-031 access are
aligned without changing Statements or completing development gates.
Family two remains incomplete; family three has not started. Continue the
remaining related analyses and concrete schema/request/strict-class work.
