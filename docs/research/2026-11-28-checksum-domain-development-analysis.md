# Development analysis: INT-018/019/020/021/043 — 2026-11-28

Written maintainer analysis of five APPROVED checksum interpretations:
code-name recognition and raw exclusion, unavailable exclusion versus bad
stored values, uppercase hex, duplicates, and zero-padded algorithm 2.
The approved behavior is unchanged. Owner/independent analysis review and
intensive executable tests remain required before implementation. Analysis
is not final approval, a parser implementation or passing checksum coverage.
Inspected main e90aded after #92; Q25/Q26 and the earlier owner answers are
recorded. This group needs no new owner policy decision.

## Source evidence and limits

Re-fetched the archive manifest on 2026-11-28 and verified each cached or
newly fetched PDF against its current manifest SHA-256. Archive main was
10719995d63426c2fe62d1548acb21ae0238b44c when inspected. Thirty-two unique
PDFs cover fourteen archived Chapter 9 editions, thirteen separate Chapter 6
files, the shared 106-04 volume, RCC 123-16 and four RCC 124 handbooks.
Read pertinent text-layer paragraphs/table rows; searched the handbooks for
checksum/signature/digest context. No rendered-image comparison is claimed.
The cached pure-Python PDF tool emitted optional font-decoder warnings for
older Chapter 6 fonts; these are source-extraction limitations, not permission
to infer a missing rule. No new tool or project dependency was installed.

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

### Re-read passages and historical boundaries

| Archived edition/context | Located passage and consequence |
|---|---|
| 106-04 full volume | Ch9 §9.4.2 p. 9-2, PDF p. 180: attribute framing/case context. Ch6 .TMATS command material at pp. 6-79–6-80, PDF pp. 152–153, concerns transfer/storage; it is not authority for a SHA-256 algorithm. |
| 106-05 / 07 / 09 / 11 | Ch9 framing/case in the opening pages. Ch6 .TMATS material at printed pp. 6-57–58 (05) and 6-61–62 (07/09/11) describes WRITE/READ/SAVE/GET, not the later SHA-256 constant/exclusion rule. No historical algorithm is invented. |
| 106-13 | Ch6 §6.2.2.40 p. 6-30 explicitly defers the CHECKSUM algorithm/output to a subsequent release; transfer text continues on p. 6-31. This is unspecified historical behavior, not a different algorithm that tmats should guess. |
| 106-15 | Ch9 Table 9-2 p. 9-13 defines G\SHA, raw exclusion, integer designator and 2- plus 64 hex. Ch6 §6.2.2.40 f p. 6-31 specifies SHA-256, lower-case digits, constant 2- and name-to-semicolon exclusion. |
| 106-17 / 19 / 20 / 22 / 23 / 24 / 24R1 | Ch9 §9.4.2 p. 9-2 and Table 9-2 p. 9-13; Ch6 §6.2.3.11 f p. 6-22, with examples continuing on p. 6-23. The operative constant, lower-case 64 hex and raw exclusion wording agree in these inspected passages. |
| RCC 123-16 | Packet checksum discussion and reference TMATS code are supplemental context only. Packet header/data checksums are not G\SHA; implementation examples cannot override Chapter 9 framing or the approved lossless contract. |
| RCC 124-13 / 17 / 19 / 22 | No additional checksum/signature/digest definition was located by the text-layer searches. This is a search result, not proof that no image/text variant exists; no rule is supplied from silence. |

Baseline authority: 106-24R1 Ch9 Table 9-2 MESSAGE DIGEST/CHECKSUM
p. 9-13 and Ch6 §6.2.3.11 f pp. 6-22–23. Ch9 calls the range an integer,
hyphen and hex characters, and refers to the excluded colon-bearing item;
Ch6 fixes the output spelling and excludes from the code name to the following
semicolon. Ch9 §9.4.2 p. 9-2 separately establishes framing and case freedom.
Those texts explain the approved interpretations; neither source prescribes
which duplicate wins, permits repairing framing, or defines leading-zero
recovery by itself. The owner decisions supply those choices explicitly.

This is baseline behavior, not a claim that the later definition existed in
106-13 or that a labelled older basis has a complete historical registry.
INT-017/044 basis labels and actual coverage limitations remain visible.
Do not choose a hash algorithm from RCCVER or the recording's packet checksum.
The Chapter 6 example digest is not reproduced or used as a golden; the
independent byte/digest oracles below replace it for future synthetic tests.

## INT-018: identify a name, then preserve its raw interval

The scanner's complete framed item and retained source locations precede
checksum recognition. Use INT-036 name projection, applicable name-edge
blank handling and INT-038 full-key identity; compare the literal G/SHA
case-insensitively per §9.4.2. This supplies a recognized item, never a
substring match in a previous value, a COMMENT or an X-carried original path.
Name projection must retain the first contributing G/g byte and final
semicolon's source locations. The hashed domain uses raw ranges around that
interval, including leading layout before the G and every byte after ';'.
Do not hash a reconstructed/projected document, or exclude leading layout
merely because it was omitted from the interpreted name.

For `g\s\r\nha` with real CR/LF inside the name, the raw spelling differs
from G\SHA. Keep the located noncanonical-name finding and applicable
internal-omission information, while excluding every raw byte in the selected
name-to-semicolon interval. The finding remains prominent in a stamp result;
recommend an explicit rename edit first, never a silent rename by stamp.

Compatible high-byte damage may conceal a further SHA name. L3-VAL-012's
ordered-readable-character/unknown-run rule preserves that evidence without
creating an exact lookup key. A known exact item alongside such damage does
not establish uniqueness. A leading BOM likewise blocks semantic lookup;
diagnostic token recognition for INT-043 warnings does not lift the block.

A preceding missing ';' leaves a later G\SHA-looking sequence inside the
previous value. Keep the actual scanner boundary and suggest-only framing
finding where its recognizer prerequisites hold; no regex repair is allowed.
An unterminated suffix is not an explicit lookup item under INT-035/019.
Raw extraction remains byte-exact even when semantic checksum use is blocked.

## INT-019: domain availability differs from value validity

Reason about independently established record bytes, item identity, framing
and exclusion before deciding whether a stored digest is usable. These are
separate facts, not a single 'valid SHA field' boolean. A complete terminated
item can have a bad value while still defining the exact bytes to exclude.
For `G\SHA:2-ABC;` or 63 hex digits, VERIFY is malformed, but authoritative
COMPUTE is allowed if the complete document/domain is otherwise established.
The same rule covers unknown algorithm tokens, unavailable value interpretation
and the no-hyphen placeholder. It is not a placeholder-specific exception.

Refuse authoritative COMPUTE for undefined/ambiguous domains: a missing
terminator, known truncation/cut, duplicate or compatible damaged SHA names,
blocking BOM, or enlarged value with evidence it swallowed a following item.
A bad digest length alone does not prove swallowed text or missing input.
For an ordinary complete document with no SHA item and no compatible damaged
presence evidence, the raw whole document is the input domain.
A separately labelled raw-byte hash is not an authoritative TMATS checksum
when the necessary domain is unavailable.

A final ';' cannot repair independently proven leading loss (Q25/INT-034).
Duplicate-induced aggregate Ambiguous keeps any blocking cut facet under
INT-048; neither a checksum nor caller ambiguity opt-in makes cut text usable.
Record all simultaneous malformed/duplicate/damage findings. This analysis
does not select a new universal precedence among visible status facets.

## INT-020: projected comparison never rewrites hashed bytes

For an established algorithm-2 item, interpret the digest value through the
approved INT-036 projection, validate 64 hex characters, then compare with
the digest of the independent raw domain. Uppercase hex may match, with the
required lower-case warning; real CR/LF omission keeps its located information.
A projected match is not a license to normalize raw COMMENT/layout bytes
outside the exclusion. Those bytes continue to affect the digest.

INT-055's raw decimal-integer syntax does not repair or reject this hexadecimal
field as a decimal number. G\SHA is explicitly in its representation exclusion
inventory. The checksum-specific projected value and INT-043 designator rule
remain authoritative; their scopes must not be replaced by a generic number
helper. Unknown value-blank handling stays unavailable per INT-041, never
trimmed into a match or labelled Invalid solely for lacking that rule.

`G\SHA:0;` means unverified, placeholder/no embedded digest, never match or
mismatch. Its complete exclusion permits COMPUTE; an explicit stamp may
replace it, using the established final-byte transaction and verification.
Missing item, placeholder, malformed value, unsupported algorithm and unavailable
value must remain distinguishable to callers without inventing new Rust enum
spellings or deciding JSON source-byte representation here.

## INT-021/F1: multiplicity cannot be resolved by agreement

Count every occurrence in the one assembled document. Equal digest strings,
raw identity, matching hashes, case variants and projected collisions do not
choose a winner. Verification stays indeterminate; stamping refuses until
input ambiguity is fixed. Raw/source occurrence evidence remains intact.
A single record across fragment packets is one document: use the same channel,
sequence +1 modulo 256 and matching FRMT/RCCVER join evidence (ADR-0025/0031,
INT-011). Separate setup records/documents do not create scalar duplicates.

INT-048 makes assembly aggregate Ambiguous when field ambiguity exists;
INT-046's bounded identity exception cannot promote that to Complete.
This assembly consequence is separate from F1's checksum refusal, not a
claim that F1 alone defined all assembly classifications. Existing cut
facets retain their full blocking force.

The ADR-0029 exact-item removal is still allowed: bind an explicit item ID to
the input revision, remove that exact duplicate, rebuild/re-read the new
revision, then assess eligibility again before stamping. Never pick first/
last, silently drop equal entries, retarget a stale ID, or reuse the original
revision's findings/digest as proof of the changed result. Broad deletion
semantics remain their own reviewed edit scope.

## INT-043: numeric designator identity with mandatory warning

Read a nonempty ASCII digit token before the ASCII hyphen from the checksum's
approved projected value. Strip leading zeros as a canonical-string view,
retaining 0 for an all-zero token; compare with the string 2. No fixed-width
integer, big-integer conversion or source rewrite is needed. This is the
owner's explicit algorithm-token choice, not numeric identity inferred from
INT-038 alone or an extension of INT-055's decimal field limits.

For every located 02/002/noncanonical-2 token, emit NONCANONICAL-DESIGNATOR
warning even if verification is blocked by F1, damage, BOM or undefined
exclusion. Preserve every token location rather than deduplicating warnings.
Diagnostic recognition under a BOM does not define a valid exclusion.
A usable matching digest can yield positive verification only if all other
record/domain/uniqueness/value conditions hold. Uppercase hex adds INT-020's
warning independently of the designator warning or final verification result.

0-/00- and other all-digit designators are unsupported algorithms, not 2;
no-hyphen 0 is the distinct placeholder. +2-/2.0- are malformed. Leading
blank remains unavailable under INT-041; a high-byte en dash is Invalid under
INT-037, not an ASCII hyphen. Very long zero prefixes use string identity,
not a silent ten-digit cutoff borrowed from a different field definition;
actual resource bounds remain the reviewed resource contract.
Canonical stamping writes only 2- and lower-case hex, never an irig106.org
flex signature in G\SHA (L1-SUM-003). Flex and core packet-integrity evidence
remain independent; no successful digest upgrades core Mismatch/NotChecked,
and no core verdict upgrades a G\SHA result.

## Independent byte/digest and located-span oracles

Standard-library hashlib was run locally over the explicit preimages below,
not over parser-generated 'expected' output. The empty-preimage row is a primitive digest oracle only: a padding-only
setup is Incomplete under Q25 and does not gain checksum eligibility.
The full source fixture consists
of supplied raw intervals whose exclusion is independently specified; no
production scanner, TMATS checksum implementation or reference was run.
These are future fixture expectations, not executed repository tests.

| Fixed preimage (hex) | SHA-256 lower-case hex |
|---|---|
| `(empty)` | `e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855` |
| `475c504e3a544553543b` | `e4478640e0caef18c801adb453a53db25425b2a1246bddb80e208d2ff35c0a9e` |
| `0d0a200a475c504e3a544553543b` | `7f8b1b2594e2f9421664329073422e5b1902b724fc519e8a796d6142beafd818` |
| `434f4d4d454e543a6109623b` | `f6f55bfbe42d00c03aa12c88d1bfa223939e1be71103acfeaabc4af39ef7823b` |

For the projected-name fixture, raw source is CR LF SP then `g\s` CR LF
`ha:` then a supplied 66-character algorithm/digest value, `;`, LF,
`G\PN:TEST;`. Independently fixed raw exclusion is [3,78), not [0,78).
The preimage is exactly hex 0d0a200a475c504e3a544553543b above. Changing
input name/value bytes inside that known exclusion does not change the
preimage; changing the leading layout or following COMMENT/TAB bytes does.
Digest independence from excluded bytes does not bypass damage/uniqueness,
framing or other eligibility guards. Equality of digests never proves equality
of raw sources or governing identity.

## Planned edge and property coverage

Each row is a future independently specified fixture, plus bounded CI property
coverage over case, controls, spans, values and edition labels. Do not use
Chapter 6's unreproduced example digest or production exclusion as the oracle.
Source excerpts are not automatically conforming whole TMATS fixtures.

| Planned case | Independent expected evidence | Requirements / INT |
|---|---|---|
| `sum_analysis_name_case_all_variants` | All ASCII letter-case combinations identify one complete G/SHA name; raw spelling/locations retained; only canonical spelling lacks its spelling finding. | L3-SUM-001; INT-018 |
| `sum_analysis_projected_name_raw_span` | The specified [3,78) raw exclusion preserves leading layout in the preimage and internal CR/LF within exclusion; no source rewrite. | L3-SUM-001; INT-018 |
| `sum_analysis_value_and_x_are_not_sha` | COMMENT value text and X-carried G/SHA path do not create checksum items or exclusions. | L3-SUM-001; INT-018 |
| `sum_analysis_missing_previous_semicolon` | Later apparent SHA remains previous value; no inserted boundary; recognizer-based suggestion only where reviewed vocabulary is present. | L3-SUM-001; INT-018 |
| `sum_analysis_damaged_name_keeps_all_blockers` | Exact SHA plus compatible damaged name prevents unique exclusion/positive verification; raw candidate evidence and warnings retained. | L3-SUM-003; INT-018/021 |
| `sum_analysis_bom_diagnostic_does_not_admit` | Blocking BOM plus located 02 token keeps warning but never restores lookup/exclusion/positive verification. | L3-SUM-003; INT-018/043 |
| `sum_analysis_bad_value_defined_exclusion` | Separate complete 2-ABC and 63-digit items: VERIFY malformed, authoritative COMPUTE allowed for established document/exclusion. | L3-SUM-002; INT-019 |
| `sum_analysis_unknown_or_unavailable_value_compute` | Defined complete exclusion permits COMPUTE despite unsupported token or unavailable value; no positive VERIFY inferred. | L3-SUM-002; INT-019/020/043 |
| `sum_analysis_unterminated_and_cut_refuse` | Exact EOF suffix or independently known truncation blocks authoritative checksum; later semicolon/equality cannot erase leading cut. | L3-SUM-002; INT-019 |
| `sum_analysis_swallowed_attribute_is_not_length_guess` | Evidence of enlarged value swallowing a following item blocks authoritative domain; malformed length alone does not infer loss. | L3-SUM-002; INT-019 |
| `sum_analysis_no_sha_hashes_complete_raw_document` | Complete known source with no exact/compatible SHA evidence hashes its complete raw bytes; no implicit placeholder insertion. | L3-SUM-002; INT-018/019 |
| `sum_analysis_uppercase_hex_warns_on_match` | Matching uppercase digest receives lower-case warning with raw bytes intact. | L3-SUM-004; INT-020 |
| `sum_analysis_controls_are_value_projection_only` | Real CR/LF in digest disappear only from the interpreted value; omission information remains and outside-exclusion controls remain hashed. | L3-SUM-004; INT-020 |
| `sum_analysis_blank_is_not_numeric_repair` | Checksum value blank stays unavailable; neither trimming nor INT-055 decimal rejection is used for this excluded hex field. | L3-SUM-004; INT-020/043 |
| `sum_analysis_placeholder_compute_without_verify` | Complete G/SHA:0 has placeholder status and defined exclusion; COMPUTE allowed, no match/mismatch; explicit final-byte stamp separate. | L3-SUM-005; INT-020 |
| `sum_analysis_duplicate_equal_or_different` | Every exact/projected scalar occurrence survives, whether equal or different; VERIFY indeterminate and stamp refuses. | L3-SUM-003; INT-021 |
| `sum_analysis_duplicate_cut_is_still_blocking` | Assembly Ambiguous retains cut; no equality, checksum or caller ambiguity opt-in grants usability. | L3-SUM-003; INT-019/021 |
| `sum_analysis_duplicates_cross_fragments_not_records` | Same channel/+1 modulo 256/same FRMT-RCCVER establishes one record and duplicate scope; independently assembled records do not share scalar slots. | L3-SUM-003; INT-021 |
| `sum_analysis_exact_removal_then_requalify` | Exact revision-bound duplicate removal is allowed; rebuild before stamp and retain original-revision findings separately. | L3-WRT-002, L3-WRT-003, L3-SUM-006; INT-021 |
| `sum_analysis_zero_padded_algorithm_matches` | 2/02/002 identify 2 by canonical string; every noncanonical token warns even on mismatch or blocked verification. | L3-SUM-004; INT-043 |
| `sum_analysis_long_algorithm_no_integer_conversion` | A long zero-prefix ending 2 follows canonical string identity within resource bounds, not u32 overflow or an unrelated decimal-field limit. | L3-SUM-004; INT-043 |
| `sum_analysis_algorithm_shape_distinctions` | 0-/00-/3- unsupported; no-hyphen 0 placeholder; +2-/2.0- malformed; leading blank unavailable; non-ASCII dash Invalid. | L3-SUM-004; INT-020/043 |
| `sum_analysis_multiple_warnings_never_short_circuit` | Two located noncanonical-2 items each keep warning plus F1/assembly ambiguity; uppercase hex warning remains distinct. | L3-SUM-004; INT-020/021/043 |
| `sum_analysis_stamp_is_canonical_not_flex` | Stamp output algorithm is 2 with lower-case hex; noncanonical name finding remains; flex never occupies G/SHA; core integrity unchanged. | L3-SUM-005; INT-018/020/043 |
| `sum_analysis_edition_evidence_is_not_aliasing` | Baseline rule/basis and source coverage remain labelled; 106-13 deferred algorithm is not treated as a historical SHA-256 definition. | L3-SUM-004; INT-018/019/020/043 |

Property families: split the same known raw preimage/exclusion across span and
fragment boundaries, vary name case, digest hex case, omitted value controls,
zero-prefix lengths and source edition labels; preserve the fixed raw-domain
oracle and independent occurrence sets. Mutate bytes outside exclusion to
change the expected digest, and mutate excluded bytes to retain it only when
eligibility/identity is independently known. Combine two or more blockers
without losing any occurrence or finding. CI stays bounded; no local fuzz,
reference qualification or dependency is introduced. Real recordings remain
local-only and were not accessed.

## Family-two audit and next work

Q25/Q26 and previously signed answers activate their selected contracts, not
implementation. In current requirements the precedence arm is unblocked;
CLI-002/006/show now have L2/L3 children and planned report tests. Negative
fixtures with deliberately missing approval remain negative fixtures, not
stale holds to remove. Dated research notes remain history. The ASCII-only
addendum's coordinator provenance/confirmation label is retained as reviewed;
this checksum analysis neither changes it nor opens a new decision.

Remaining derivation/development work includes reviewed schema artifacts,
non-UTF-8 source-byte representation, precise request/mode applicability,
and the explicit Q18 bad-data-class mapping beyond the listed six kinds.
Those cannot be inferred from rendering, severity or this checksum group.
Other approved INT groups still need written analysis, owner/independent
analysis review and appropriate implementation-family tests. Historical CSDW
layout and Appendix 9-C vocabulary notes exist; they are not unperformed
maintainer tasks or substitutes for the other analyses.

Family two is not complete. The current ordered roadmap requires it before
family three. Do not start a concurrent restructure: it retires the prototype,
changes source/test-marker roots and touches manifest/CI/structure/trace
contracts still being assessed. Future sequencing may isolate independent
notes, but this PR does not waive that gate or begin ADR-0035's restructure.

## Executed checks and readiness

Locally executed: fresh manifest lookup, all listed PDF hash checks, bounded
text extraction/passages and supplemental-source search, explicit independent
hashlib byte-oracle checks, planned-row/count/link checks, trace regeneration
and --check, diff --check. No library/checksum/CLI/reference implementation,
Rust tests/builds, local heavy campaign, new tool/dependency or publishing.
All extraction jobs finish before reporting. Source extraction limitations
remain documented and need independent review rather than invented certainty.

Only these five Analysis fields link this written draft. Their Design/Review
sign-offs are unchanged; owner and independent analysis review remain pending.
This adds no executed redesigned coverage and does not complete the other
INT analyses. Require exact-head independent review; do not merge automatically.
