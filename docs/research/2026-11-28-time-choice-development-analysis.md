# Development analysis: INT-054–060 — 2026-11-28

Seven APPROVED interpretations share the bounded time view: CHE context,
raw decimal syntax and representation dispatch, caller levels/eligibility,
evidence-only contradiction/conflict and explicit per-query choice. This
written draft changes no approved behavior or Design/Review sign-off. Owner/
independent analysis review and intensive executable tests remain required.
Inspected main 373cca3 after #95; no new owner decision is made here.

## Sources, historical scope and inventory

Re-fetched the archive manifest on 2026-11-28 and verified 19 PDF hashes:
fourteen archived Chapter 9 editions, RCC 123-16 and four RCC 124 handbooks.
Re-extracted/read the pertinent recorder/time/representation rows and context.
Read the baseline Range contexts alongside the existing reviewed exclusion
inventory; no rendered-image review or implemented historical registry is
claimed. Code/whitespace searches locate text, never define representation
from the appearance of a value. Missing historical locators do not prove a
field was absent from every recording or authorize a new default/alias.

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

### Baseline authority and owner policy

| 106-24R1 Chapter 9 source | Fact and boundary |
|---|---|
| §9.4.1–2 p. 9-2 | ASCII/nonprintable and blank/case rules underlie INT-036/037/041. They do not themselves choose the ten-digit limit, leading-zero warning or caller levels. |
| Table 9-4 NUMBER OF TRACKS/CHANNELS R-x\N p. 9-32 | Range 1–65536; operand for R\N > 0 applicability. Zero is out of range, not a usable false operand. |
| Table 9-4 NUMBER OF SOURCE BITS R-x\NSB p. 9-33 | Range 0–13, required when allowed under specified R\ID. Missing is not explicit zero; this is context, not a time-source association. |
| Table 9-4 TRACK NUMBER/CHANNEL ID R-x\TK1-n p. 9-48 | Range 1–65535. Entry index n is not the actual channel ID. |
| Table 9-4 CHE / DSI p. 9-49 | CHE says source must be enabled to generate packets; T/F, required when allowed, no printed default. Neither CHE nor DATA SOURCE ID selects a library time association. |
| Table 9-4 CDT p. 9-50 | TIMEIN is a channel-type keyword. Enumeration of a disabled TIMEIN is owner policy, not a new keyword interpretation. |
| Table 9-4 SHTF p. 9-51 | Numeric enumeration 0/1/2 is interpreted under INT-055; local text does not enable a packet secondary header. |
| Table 9-4 TTF/TFMT/TSRC pp. 9-71–72 | TTF format codes, TFMT letter/numeric arms and default A, TSRC I/E/R/X categories. These are local metadata, not actual associated channel IDs or packet-derived association proof. |
| §9.5.1 a pp. 9-5–6 and complete field definitions | Representation is declared by reviewed metadata, including mixed/symbolic arms; a digit-looking binary/string/hex value is not thereby decimal. |

INT-054/055 owner decisions and answers dated 2026-11-25, with final AI
CONCUR at #51 ebd5c98, supply the enumeration/numeric policies. INT-056–060
owner answers dated 2026-11-26 and final AI CONCUR at #52 3fcbd2c supply
caller acceptance, exact Quiet suppression, eligibility and no-inference rules.
The standard supplies field facts, not those acceptance/emission policies.
This analysis does not treat that policy as printed Chapter 9 text.

### Historical and supplemental comparison

| Inspected context | Located evidence / limitation |
|---|---|
| 106-04 full volume | TK1/CHE/TFMT/DSI locators occur in Ch9; NSB/TTF/TSRC/SHTF were not located in this search. Do not fabricate absent source bits or packet/format fields from modern rules. |
| 106-05 | NSB/TK1/CHE/TFMT/TSRC/DSI locators occur; TTF/SHTF not located. A locator is not an applicability/default declaration. |
| 106-07 / 09 / 11 | TTF joins the located fields; SHTF not located. Historical table ranges/keywords require their own reviewed definition/basis. |
| 106-13 | All selected recorder/time code-name locators occur. The existing source review found no printed TFMT Default A in inspected 04–13 rows; modern defaults must not be silently backported. |
| 106-15 / 17 / 19 / 20 | The selected fields occur; inspected TFMT rows include the TIMEIN/default-A contract from 15 onward. The actual applied basis and available metadata remain explicit. |
| 106-22 / 23 / 24 / 24R1 | Selected fields occur with modern representation/usage context. This does not qualify every historical numeric field or all enum/code arms. |
| RCC 123-16 | Recorder/time reference context is not a second packet reader, ID parser or proof of a data-channel-to-time-channel association. |
| RCC 124-13 / 17 / 19 / 22 | No selected time-code locator was found in 124-13; located 17/19/22 recorder/XML time paths illustrate local metadata; they do not prescribe caller levels or supply a missing association field. No examples are copied as conforming raw fixtures. |

Historical TFMT/default details are cross-referenced in the
[effective/presence/basis analysis](2026-11-28-effective-presence-basis-development-analysis.md).
Older-profile absence/unreviewed metadata retains INT-041 availability and
INT-044 intended-versus-applied baseline labels. A source table existing is
not proof that its parser, defaults, ranges or edition deltas are implemented.

### Complete baseline Range-accounting boundary

Reconciled the PDF text's 740 literal Range hits with the reviewed
[every-Range/exclusion inventory](2026-11-26-int055-binary-exclusions.md):
726 colon labels plus five alternate labels are 731 label/forms; six extra
Range words occur inside Range values, two name the Range Commanders Council,
and one is Range View. Remove the two explanatory labels on p. 9-6 to obtain
729 table rows. This is documentation/source accounting, not grammar generation.

| Reviewed baseline inspection bucket | Rows | Consequence |
|---|---:|---|
| Floating point / positive floating point | 43 | Excluded, even with all-digit spelling. |
| Hex/structured/mixed hex | 10 | Excluded; existing G\SHA grammar remains separate. |
| Binary-labelled representations | 27 | Excluded, preserving widths/pattern arms. |
| Unlabelled 0/1 masks/patterns | 11 | Excluded, including the eight P-group rows formerly in the review bucket. |
| Character-length | 197 | Not decimal merely because contents are digits. |
| Date/time/IP | 10 | Separate representations. |
| Enumerations | 200 | Only an explicitly reviewed numeric-code arm uses INT-055. |
| Remaining integer/symbolic/mixed | 231 | Definition/arm review required; not automatically executable decimal rules. |
| Total table rows | 729 | Nothing is silently lost or newly declared implemented. |

Read the complete row/continuation, not just a prefix regex: e.g. P MF5's
binary value and B LBL's 8 Binary digits differ from Range: Binary; G\SHA's
structured hex and S MID's ASCII-or-Hex differ from Range: Hex. Binary words
in a decimal word-position Definition do not make that position a bit mask.
The prior [floating/hex inventory](2026-11-25-int055-owner-answers.md) and
binary inventory identify every excluded row with pages. Unknown dispatch or
mixed non-decimal arms stay unavailable/their own reviewed rule, not Invalid
because a generic decimal helper rejected them.

## INT-054: enumerate first, retain CHE as context

A confirmed scoped TIMEIN entry stays in the diagnostic time view with CHE F.
T, missing, invalid, ambiguous or unavailable CHE neither changes CDT's type
nor silently filters the entry. Under a proved applicable R\N > 0 condition,
missing CHE is Missing with the required-presence finding, not enabled/disabled
by default. An unusable R\N cannot establish that applicability; preserve
its uncertainty rather than assume positive or zero. Raw source stays intact.

An explicit caller filter may produce a filtered view, never mutate the source
candidate set or turn the sole survivor into association evidence. Disabled
context does not prohibit addressing an otherwise eligible scoped TIMEIN
candidate under INT-057; this analysis adds no CHE-based caller refusal or
new ID/format bad-data-class member. Independent CHE and applicability findings
remain available, as do record/cut qualifications. Enumeration does not prove
governing use or permission for the recorder to generate packets.

## INT-055: dispatch by definition, then check raw bytes

Apply only to reviewed decimal-integer field definitions and numeric code arms
of explicit enumeration tables. Do not infer a number from all-digit bytes,
or apply it to code-name indices, binary masks, floats, hexadecimal fields,
strings/dates/IP or G\SHA's structured designator/digest grammar. Letter TFMT
B stays on its letter path; CHE F is not zero/False parsed by a host boolean.
No new non-decimal grammar, registry coverage or numeric CLI-argument syntax
is approved by the field-value rule.

For this decimal branch require 1–10 raw ASCII digits before zero stripping.
Empty, signs, decimal/exponent notation, non-ASCII digits/high bytes and raw
controls are Invalid, without clipping/wrapping or projection repair. Real
CR/LF/TAB/NUL/DEL anywhere in the raw numeric value are not omitted into a
usable number. Preserve independent INT-036/037 findings/locations. A raw
0 CR LF 4 does not become Explicit 4; historical fixture names do not change
that current Invalid expectation. No guess of UTF-16/other encoding is made.

Uniform leading blank remains unavailable under INT-041 for NSB/TK1/R\N/
SHTF/TTF/numeric TFMT. Trailing/embedded blanks are Invalid; independently
proved failures survive the leading-blank guard. Do not trim 2 SP or 0 SP 2
into codes; a leading blank cannot hide a control, high byte or other proved
syntax failure. Letter/structured fields retain their separately reviewed
blank rules, not this decimal-only exception.

Strip leading zeros only after accepted digit syntax, retaining 0 for all-zero
input. The noncanonical-spelling warning survives a later range/membership
failure. Eleven zeros fail the limit before stripping. No overflow-prone u32
conversion: 9,999,999,999 fits u64, or compare canonical strings by length and
lexicographic order. The ten-digit cap is owner policy, not a Chapter 9 range.
Local range checks remain NSB 0–13, TK1 1–65535 and R\N 1–65536 in the cited
baseline. NSB 00 is Explicit zero with warning; missing NSB is not that value.
TK1 00 is Invalid range with its accepted-spelling warning; R\N 0 is Invalid,
so its >0 condition is cannot-evaluate rather than false.

Numeric TTF/TFMT 02 matches code 2 with warning; letter TFMT B keeps its own
case/raw evidence without a numeric warning. No format/default or packet
observation is rewritten: applicable absent TFMT may still Default A under
its verified rule, with a separate network-time inconsistency finding for
TTF=2 where required. A default never overrides packet evidence or proves an
association. Scalar duplicates stay Ambiguous before selecting among their
values; canonical spelling does not pick a winner.

## INT-056: levels act on choice, not field validity

For an otherwise eligible supplied target with bad ID/format, Strict refuses
with a typed bad-data reason. Warn is the default LEVEL and accepts a labelled
caller assumption with all bad-data findings. Quiet accepts the same labelled
assumption and suppresses EXACTLY that Warn attachment set, of ANY severity.
Unrelated findings are not suppressed. Fields/source diagnostics stay readable
with original Invalid/Ambiguous/unavailable states and reasons; no downgrade,
repair, automatic clean state or association proof follows.

Every Quiet result includes suppressed: N bad-data findings, even N=0. Count
actual Warn findings, not affected fields, categories or warning-only severity;
two distinct findings on one field can both count. Keep each query's level,
assumption/basis, target identity and suppression evidence on its dependent
answers. Do not accumulate prior-query counts or drop a zero marker. Retention
of source diagnostics is not permission to re-emit suppressed assumption
findings through another output path, nor a reason to erase them at source.
Concrete schema representation stays a separate reviewed artifact.

The nine existing Strict/Warn/Quiet by Invalid/duplicate/unavailable fixtures
each have ID and format arms. New cases below also pin failure composition
and exact suppression. Duplicated scalar fields can make assembly Ambiguous
under INT-048: use the independent INT-031 access qualification/explicit
per-query opt-in where applicable; do not fabricate Complete records to test
bad-field acceptance. No level bypasses a blocking cut, framing/trust/scope
failure, eligibility refusal or genuine approved-evidence contradiction.
Accepting the local assumption choice is not a guarantee of usable fields,
a valid record, timestamp decode or governing setup.

## INT-057: target identity and ID value differ

The chosen candidate must exist as a scoped TIMEIN in the same recorder,
document/revision/query context. Non-TIMEIN, other recorder and nonexistent/
unresolvable ID-only choices refuse at every level with a typed reason.
An entry's canonical identity may be usable while its own TK1/format is bad;
that is INT-056, not permission to manufacture a usable ID. Entry index n
is not actual TK1; equal numeric TK1 across recorders cannot satisfy scope.
Duplicate ID-only candidates do not permit a first/last target winner.

CHE F remains context; shared DSI, NSB components, enabled/only-candidate
status or raw value similarity cannot establish eligibility for an unknown
entry. Raw/diagnostic enumeration and semantic access remain separate; no
assumption policy restores missing bytes or changes the governing setup.

## INT-058/059: evidence is not plausibility or candidate count

Contradiction refusal needs a SEPARATE owner-approved evidence INT proving a
different association for this data channel in scope, with its review/source/
development gates. INT-058 is the gate, not an evidence source. There is NO
such production evidence rule in 0.1. Order/sole candidate/TSRC/DSI/NSB/CHE,
observed time packets and checksum success never supply one. Bad target data
or a suggestively named vendor attribute is not contradictory association proof.
Do not introduce a packet-level proof while claiming mere local metadata work.

Zero, one or many unproven candidates stay association_unestablished with
candidates/reasons retained, not conflicting/evidenced. Competing admitted
evidence is the reserved conflicting case. Preclassified conflicting or
contradictory facts in composition fixtures test the receiving boundary only;
they cannot qualify a nonexistent baseline evidence path. This analysis
selects no precedence among future competing sources. A genuine established
contradiction cannot be bypassed by Strict/Warn/Quiet, but an unreviewed rule
cannot trigger that typed refusal.

Keep association state distinct from field Explicit/Defaulted/Missing/Invalid/
Ambiguous and INT-041 availability, L1-VIEW-006 lookup cardinality, record
qualification and caller-assumption acceptance. Selecting a target can yield
an assumption-bearing answer without upgrading the unestablished association.
Defaults or field agreement do not make that answer standards-evidenced.

## INT-060: no ambient target, even within one session

Every query carries its own choice explicitly. Warn default selects a level,
never a target. A prior accepted choice cannot supply the next query's absent
choice, even for equal source bytes in the same recording/setup/revision.
Dependent answers preserve their originating query's provenance; an independent
unassumed read stays unassumed. Reused immutable parsed metadata is not a
license to reuse the previous caller decision or suppression marker.

Revision/candidate/recorder identities are part of the result's binding, not
keys to silently retarget an old assumption. New setup revisions never inherit
a target based on equal ID/order. No new CLI flags, public Rust structs,
rendered schema shape, reset rule or session-wide configuration is chosen.
Caller Strict/Warn/Quiet is not extraction --strict, whose reviewed bad-data
class and delivery contract remain separate.

## Independent bounded oracles and planned coverage

For raw `R-1\NSB:0` CR LF `4;`, length is 13, name [0,7), value [8,12),
item [0,13). The value bytes 30 0d 0a 34 are Invalid under raw digit checking;
04 after INT-036 projection is not the numeric read. Raw replay/extraction
keeps every byte and the independent character findings.

Supply an eligible R-1 entry 3 with actual TK1 42 and TIMEIN as the isolated
choice target. Entry identity 3 is not channel ID 3; another recorder's 42 is
not the same candidate. For Quiet-count composition use independently listed
Warn attachments b1/b2/b3 of information/warning/error severity at distinct
locations, plus unrelated n1: Quiet suppresses b1–b3 with N=3, keeps n1 and
all field/source diagnostics, level and assumption labels. With no Warn bad
attachments, N=0 still appears. These labels are synthetic identities, not
new finding codes or wire-schema keys.

| Planned case | Independent expected evidence | Requirements / interpretations |
|---|---|---|
| `time_policy_analysis_disabled_timein_retained` | Confirmed TIMEIN with CHE F remains enumerated and selectable if otherwise eligible; explicit caller filtering changes only its view, not source or association. | L3-VIEW-040; INT-054 |
| `time_policy_analysis_che_missing_presence` | Proved applicable R\N condition and absent CHE give Missing plus required-presence finding, never default enabled/disabled; unknown applicability retains its reason. | L3-VIEW-041; INT-054 |
| `time_policy_analysis_che_invalid_duplicate_context` | Invalid/duplicate/unavailable CHE remains located context without reclassifying CDT or silently filtering an entry; record qualification preserved. | L3-VIEW-042; INT-054 |
| `time_policy_analysis_raw_control_is_invalid` | Raw NSB 30 0d 0a 34 has value [8,12) and is Invalid, never Explicit 4; bytes and omission findings survive. | L3-VIEW-037; INT-055 |
| `time_policy_analysis_control_positions` | Each control at numeric value start/middle/end is Invalid, including TAB/NUL/lone CR/LF/DEL; no projection/encoding guess repairs it. | L3-VIEW-037; INT-055 |
| `time_policy_analysis_digit_limit_before_zeros` | Ten raw digits are within syntax; eleven zeros are Invalid before stripping. No clipping, wrap, arbitrary host-parser suffix acceptance or u32 overflow. | L3-VIEW-037; INT-055 |
| `time_policy_analysis_leading_blank_uniform` | Leading blank in NSB/TK1/R\N/SHTF/TTF/numeric TFMT remains unavailable; trailing/embedded blanks Invalid and other proven faults retained. Letter TFMT blank rule stays separate. | L3-VIEW-037; INT-055 |
| `time_policy_analysis_leading_blank_keeps_proved_failure` | Leading SP plus independently proven control/high byte/other syntax defect never erases Invalid evidence or turns it into a repaired numeric value. | L3-VIEW-037; INT-055 |
| `time_policy_analysis_non_ascii_and_signs` | Signs, decimal/exponent strings, Unicode digits/high bytes and empty decimal value are Invalid without repair, preserving raw and character diagnostics. | L3-VIEW-037; INT-055 |
| `time_policy_analysis_canonical_range_boundaries` | NSB 00 is Explicit 0 with warning, 014 invalid range with warning; TK1 65535/65536 and R\N 65536/65537 use their separate bounds. | L3-VIEW-038; INT-055 |
| `time_policy_analysis_rn_invalid_not_false` | R\N 0/Invalid, unavailable, Missing or duplicate/equal operands leave >0 cannot-evaluate, not assumed zero/positive or false; 01 succeeds with warning. | L3-VIEW-039; INT-055 |
| `time_policy_analysis_numeric_codes_letter_separate` | TTF/TFMT 02 and SHTF 00/01/02 match reviewed numeric codes with warning; letter TFMT B and CHE F do not enter a numeric/boolean repair branch. | L3-VIEW-038; INT-055 |
| `time_policy_analysis_no_absence_from_bad_numeric` | Present invalid/empty/ambiguous numeric data never becomes absence/default; missing NSB remains Missing with applicable required-presence finding, not Explicit zero. | L3-VIEW-039; INT-055 |
| `time_policy_analysis_binary_widths_excluded` | MFD TRA 00101, 32-bit DMSK, 64-bit AMSK, 81-bit UCSV, FW/X arms and P ADM FDP 0011 get no INT-055 zero-strip/warning/ten-digit limit; their own reviewed grammar or availability applies. | L3-VIEW-037; INT-055 |
| `time_policy_analysis_float_hex_text_excluded` | ITH -12.5 or digit-only float, MRC SPM 0001/0A3F, string/date/IP digit spellings stay outside INT-055; G/SHA retains its separate projected grammar. | L3-VIEW-037; INT-055 |
| `time_policy_analysis_dispatch_not_inventory_count` | All 729 table Range rows remain accounted for, but unknown/mixed representation metadata cannot become an executable decimal rule from a digit-looking value or bucket count. | L3-VIEW-037; INT-055 |
| `time_policy_analysis_strict_invalid_id_format` | Otherwise eligible target with independently Invalid ID and format arms: Strict typed refusal, original states/qualifications and source evidence unchanged. | L3-VIEW-052; INT-056 |
| `time_policy_analysis_warn_invalid_id_format` | Invalid ID/format arms: Warn default accepts labelled assumption with every bad-data attachment and no field/association upgrade. | L3-VIEW-052; INT-056 |
| `time_policy_analysis_quiet_invalid_id_format` | Invalid ID/format arms: Quiet labelled acceptance, exact suppressed count and level visible; Invalid and source findings preserved. | L3-VIEW-053; INT-056 |
| `time_policy_analysis_strict_duplicate_id_format` | Duplicate ID/format arms retain Ambiguous/record facets and supplied applicable access qualification; Strict refuses with typed bad-data reason. | L3-VIEW-052; INT-056 |
| `time_policy_analysis_warn_duplicate_id_format` | Duplicate ID/format arms with required independent ambiguity-access qualification: Warn labelled acceptance never chooses a field winner or makes assembly Complete/governing. | L3-VIEW-052; INT-056 |
| `time_policy_analysis_quiet_duplicate_id_format` | Duplicate ID/format arms: Quiet suppresses Warn attachments only, keeping every original occurrence, ambiguity/cut restrictions and count/level/assumption. | L3-VIEW-053; INT-056 |
| `time_policy_analysis_strict_unavailable_id_format` | Unavailable ID/format arms: Strict typed refusal keeps unavailable reason, not Invalid or repaired data. | L3-VIEW-052; INT-056 |
| `time_policy_analysis_warn_unavailable_id_format` | Unavailable ID/format arms: Warn labelled acceptance attaches all bad-data findings without restoring unavailable interpretation. | L3-VIEW-052; INT-056 |
| `time_policy_analysis_quiet_unavailable_id_format` | Unavailable ID/format arms: Quiet retains availability reasons/source diagnostics and exact suppression evidence, no fabricated usable field. | L3-VIEW-053; INT-056 |
| `time_policy_analysis_quiet_zero_marker` | Eligible explicit Quiet query with zero Warn bad-data attachments still has suppressed: 0 bad-data findings, level and assumption label. | L3-VIEW-053; INT-056 |
| `time_policy_analysis_quiet_any_severity_exact_count` | Distinct b1/info,b2/warning,b3/error Warn attachments suppress with N=3; unrelated n1 remains, all source/field findings readable; count findings not fields. | L3-VIEW-053; INT-056 |
| `time_policy_analysis_derived_marker_and_no_reemission` | Dependent answer keeps its own choice/level/count and source binding; original diagnostics stay accessible but suppressed assumption findings are not re-emitted by another result path. | L3-VIEW-049; INT-056/060 |
| `time_policy_analysis_all_levels_refuse_wrong_target` | Non-TIMEIN, other recorder, nonexistent or unresolvable ID-only target refuses with typed eligibility reason for every level; Quiet cannot suppress that refusal. | L3-VIEW-054; INT-057 |
| `time_policy_analysis_entry_identity_not_actual_id` | Existing canonical TIMEIN entry with bad TK1 may be explicitly addressed and use INT-056; no guessed actual ID, entry-index-as-ID or duplicate ID-only winner. | L3-VIEW-054; INT-057 |
| `time_policy_analysis_choice_keeps_cut_and_scope` | Any local level acceptance retains cut/trust/scope/access restrictions and does not make the setup usable/governing; bad field policy is not a framing override. | L3-VIEW-047; INT-056/057 |
| `time_policy_analysis_no_01_contradiction_path` | No production 0.1 evidence rule exists, so no local metadata, bad target or packet observation triggers the approved-evidence contradiction refusal. | L3-VIEW-055; INT-058 |
| `time_policy_analysis_future_proof_gate` | Injected separately reviewed conclusive contradictory evidence tests refusal composition at every level, not a new baseline association-proof rule or a guessed precedence among competing claims. | L3-VIEW-055; INT-058 |
| `time_policy_analysis_heuristics_never_proof` | Candidate order/count, CHE filtering, TSRC/DSI/NSB, equal formats, observed time packets and hashes never turn a choice into evidenced/contradictory association. | L3-VIEW-045; INT-058/059 |
| `time_policy_analysis_zero_one_many_unestablished` | Zero/one/many unproven TIMEIN candidates retain candidates/reasons and association_unestablished; no sole-survivor default or conflicting-by-count. | L3-VIEW-056; INT-059 |
| `time_policy_analysis_preclassified_conflict_only` | Synthetic conflicting input remains classified with evidence; composition is not acceptance of any actual 0.1 evidence source and no winner is chosen. | L3-VIEW-056; INT-059 |
| `time_policy_analysis_state_axes_stay_distinct` | Caller assumption/level is orthogonal to field states/availability, lookup cardinality, record qualification and association state; no default A or accepted choice upgrades another axis. | L3-VIEW-044; INT-056/059 |
| `time_policy_analysis_next_query_no_target_default` | After a successful explicit choice, a second query with omitted target stays unassumed; Warn level default does not supply that target. | L3-VIEW-057; INT-060 |
| `time_policy_analysis_revision_and_parallel_queries` | Equal metadata in a new revision or independent/interleaved query never inherits the earlier target/level/count; immutable parsed-data reuse is not ambient caller choice. | L3-VIEW-057; INT-060 |

Future bounded CI properties vary raw numeric bytes, length/zero prefixes,
known representation arms, ranges and enum membership independently; vary
bad-field kind/severity/locations under all three caller levels; permute
candidate order/CHE context without creating evidence; interleave explicit
and unassumed queries across recorder/document/revision identities. Preserve
raw bytes and original findings at every step. Real recordings remain local-
only and were not accessed. No test uses the production numeric parser,
chooser or result renderer to calculate its own expected answers.

## Executed checks and remaining gates

Locally executed: fresh manifest/PDF hash checks; bounded text extraction/read,
Range label/count reconciliation with reviewed inventories; independent raw
span/digit-bound/attachment-count sanity checks; matching planned rows and
source/requirement links; exact seven-Analysis-only register diff; trace
regeneration/--check and diff --check. Source jobs finish before reporting.
No numeric parser, time view, chooser, evidence engine, CLI, reference or Rust
suite was implemented/executed, no heavy local campaign, new dependency/tool,
publishing, credential/settings change or prototype retirement.

Only INT-054–060 Analysis fields link this written draft. Their Design/Review
and all owner wording remain unchanged. Owner/independent analysis review,
source/metadata coverage and intensive executable tests remain prerequisites.
This source accounting neither completes other INT analyses nor selects a
new rule for mixed/unsupported representation, future association evidence or
JSON source-byte rendering. Family two remains incomplete; family three has
not started. Continue related analyses and remaining concrete schema/request/
strict-class derivations under the existing review gates.
