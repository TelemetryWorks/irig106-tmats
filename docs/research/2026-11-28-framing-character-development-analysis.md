# Development analysis: INT-035/036/037 — 2026-11-28

Written maintainer analysis, scoped to these three APPROVED interpretations.
Decision approval is unchanged. This analysis itself still needs owner and
independent review under ADR-0033; intensive executable tests are not written.
It is not permission to start parser code or a completed all-INT analysis.
Inspected tmats main d682f052f5dab6c045e30e5b4006c3947bc8a948.

## Fresh sources and edition coverage

Re-fetched the archive manifest on 2026-11-28 and recomputed each PDF's SHA-256
before inspecting relevant text. Reused bytes only after the hash matched.
106-04 is a full volume; the others are Chapter 9 or named handbooks. No
source PDF or real recording is committed. No other repository was checked
out. Text extraction located the printed sections; no raster review claimed.

| Source / release asset | SHA-256 |
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

For all 14 Chapter 9 editions, re-read sections 9.4.1/9.4.2, printed p. 9-2.
PDF page positions differ (106-04 p. 180 of the full volume; others pp. 4/6/8).
Each specifies ASCII, CR/LF for readability, discard of nonprintables before
configuration translation, code-name colon and data-item semicolon,
with no semicolon inside data (including comments). These are common source
facts, not evidence that malformed-input recovery was standardized.

| Inspected edition set | Relevant difference and consequence |
|---|---|
| 106-04/05/07/09 | No later intentional-blank sentence in the inspected logical-format text. Do not quote the later wording as if printed here |
| 106-11/13/15/17/19/20/22/23/24/24R1 | Intentional leading/trailing/embedded blanks expressly discussed; this does not authorize automatic value trimming |
| 106-24R1 E.9.c p. E-12 | C-6 DCN example has a blank before ':'. Supports L1-READ-006's surrounding-name treatment; trailing annotations after ';' are not conforming fixture bytes |

Common source rules support the approved interpretation across inspected
code-name text editions. Surrounding-name/value policy remains explicitly
the project's approved reading; no historical edition delta is invented.
Chapter 9 XML exceptions permit semicolons in XML text, but this analysis
is for code-name format; XML is unsupported under ADR-0015. Never apply the
colon/semicolon scanner to an XML body merely because its bytes contain ':'.

## Companion-source check and supplemental definitions

RCC 123-16 section 2.1, printed p. 2-1 (PDF p. 17), describes CODE:DATA and
optional CR/LF, corroborating framing/readability rather than line termination.
Its legacy size statement is not imported as a new global resource limit.
RCC 124-13 p. 2-2 (PDF p. 18), and 124-17/19/22 p. 2-6 (PDF pp. 24/25/26),
repeat colon/semicolon framing, intentional blanks and readability CR/LF.
They distinguish code-name format from XML; their examples/glossaries do not
supply a malformed-recovery, Unicode-conversion or empty-region algorithm.
Searches located relevant text across each handbook, then those passages
were read; this is not a claim of full semantic transcription of handbooks.

[RFC 20](https://www.rfc-editor.org/rfc/rfc20.html), sections 2 and 4.1/4.2,
re-read 2026-11-28: the ASCII table locates controls, SP and DEL separately.
INT-036's exact omission set 00–1F/7F and special SP handling is the approved
project interpretation, not a claim SP/DEL are identical control classes.
No locale classification function or Unicode whitespace category substitutes.

[Unicode BOM FAQ](https://www.unicode.org/faq/utf_bom.html#BOM), byte-order-mark
table, re-read 2026-11-28, confirms the five signatures: UTF-8 EF BB BF;
UTF-16 FE FF / FF FE; UTF-32 00 00 FE FF / FF FE 00 00. This establishes
signature bytes only. Chapter 9 does not prescribe a BOM policy; INT-037's
retention/blocking behavior is the owner-approved product policy. Matching
a marker is not proof that the remainder is valid text in that encoding.

## INT-035: framing and recovery analysis

Frame original bytes before projecting names or values. A semicolon limits
one candidate region; only its first colon separates raw name/value. The
physical partition is independent of line breaks and reviewed vocabulary.
A missing colon cannot borrow the next region's colon. Recovery retains the
malformed region, then resumes after its actual terminator. No suggestion
changes this partition; missing-terminator and checksum-domain evidence stay.

A later colon inside a malformed value can look like a delimiter, and a
stray semicolon can look like a genuine boundary. The scanner cannot prove
writer intent. Keep raw source and code-name/unknown/duplicate findings;
never call recovered candidates the intended original attributes. Suspect
terminator detection is suggestion-only, with its separate vocabulary gate.

Empty/layout-only terminated regions form one merged run finding. The exact
span excludes any silent unterminated layout at EOF. ':v;' is malformed
empty name, not an empty region; 'X:;' is present empty value, not absent.
A substantive unterminated suffix remains raw-only, never an explicit lookup
item. This does not choose record completeness/semantic-access policies.

Rejected alternatives still matter: line-based resynchronization invents
boundaries; best-guess colon insertion alters input; using known code-like
text as a split can reinterpret ordinary COMMENT content. They remain
unselected, not hidden fallback implementations.

## INT-036: projections, source correspondence and findings

Omit ASCII 00–1F/7F only in the interpreted view. Preserve source intervals
for every projected byte, so a shorter key cannot be used as a raw edit
location or SHA domain. Boundary discovery and raw extraction use original
bytes. Trim SP only around names under L1-READ-006; value SP remains real.
Internal name SP is not repaired, and TAB in a text value disappears rather
than becoming SP. Leading/trailing name layout has no internal-omission
finding, but NUL still triggers its separate per-field information rule.

Controls between printable name bytes produce the located omission finding;
NUL occurrences aggregate once per affected name/value field, not once per
byte or document. A name can need both independent kinds of finding. Empty
projected value remains physically present. Padding regions are not fields.
High bytes are not silently discarded to make a key or value usable.

Approved INT-055 deliberately checks numeric RAW bytes: 0 CR LF 4 is Invalid,
not 04/4 after projection. Character findings and raw spans survive. This
exception is not evidence to change general text projection or to trim
numeric blanks. Typed numeric/default/applicability behavior remains separate.

Alternative control-to-space mapping changes meaning; dropping SP destroys
intentional blanks; keeping controls in keys defeats the approved discard
reading. Lossy display decoding is separately labelled, never a semantic fix.

## INT-037: marker ordering, fragmentation and limits

Check the assembled text's true offset zero after CSDW removal but before
projection/padding. Recognize each full signature across arbitrary fragment
boundaries; CSDWs never participate. FF FE also catches UTF-32LE's prefix,
without a claim to have identified the full encoding. A prefix truncated at
EOF is not a full marker; ordinary framing/high-byte evidence remains.

Do not skip initial CR/LF/NUL to retry marker matching. A later signature is
midstream; affected high-byte fields are Invalid rather than a whole-document
leading-marker verdict. Exact leading markers block semantic lookups but
retain raw bytes for extraction and separate checksum rules. Display views
as_latin1/lossy cannot promote Invalid into semantic success.

UTF-16 without a marker can resemble ASCII interleaved with controls. Per-field
NUL findings expose many cases but do not prove encoding or detect every one.
Automatic conversion, stripping or zero-pattern guessing would add unapproved
input policy and change checksum/span evidence. No such alternative is added.

## Independent edge oracles and intensive-test plan

All cases are PLANNED, not parser execution. Expected bytes/spans must be
hand-specified or independently derived, not generated by the production
scanner/projection being tested. Printable byte examples below are synthetic
framing inputs, not claims that X is a standard attribute code.

| Named planned case / family | Independent input/expected result |
|---|---|
| analysis_empty_run_spans | ';;' -> [0,2); CR LF ';' -> [0,3); ';;COMMENT:x;;;' -> [0,2) and [12,14); EOF layout excluded |
| analysis_first_colon_only | 'COMMENT:a:b;' has one value containing ':', never two items |
| analysis_empty_fields_distinct | ':v;' empty-name finding; 'X:;' physically present zero-length value; neither empty-region run |
| analysis_control_name_span | CO CR LF MMENT:a TAB b; retains raw name [0,9), value [10,13); interpreted COMMENT / ab; omission [2,4) |
| analysis_nul_per_field | Two NULs in a name and two in its value: two per-field NUL findings total, with every location, plus independent internal-name finding if applicable |
| analysis_numeric_projection_is_not_repair | NSB 0 CR LF 4 raw bytes -> Invalid under INT-055; raw extraction unchanged |
| analysis_marker_every_split | Every full 2/3/4-byte marker split at each byte, and each multi-fragment partition after CSDWs, yields the same leading-marker evidence |
| analysis_partial_marker_not_full | EF / EF BB / FE alone / 00 00 FE at EOF are not complete markers; do not suppress existing framing/high-byte findings |
| analysis_marker_after_layout_is_midstream | CR LF or NUL then EF BB BF: not leading; no padding skip or auto-strip |
| analysis_high_byte_sweep | Every 80–FF byte in name/value, including Latin-1 B0 and valid-looking UTF-8 runs, has no silently usable affected field |

Intensive follow-through must cover the existing named INT tests, these
edge oracles and interactions with INT-019/038/039/048/055. Generate bounded
byte sequences with separators/controls/high bytes and assert raw replay,
valid located intervals, projection idempotence with correspondence, recovery
progress/termination and no prefix promotion. Keep field/blocking findings
independent of raw extraction and core integrity, never cure cut facets.

Require a fixture for EACH inspected edition's 9.4.1/9.4.2 evidence (04,05,07,
09,11,13,15,17,19,20,22,23,24,24R1), with edition/source hash explicit. The
common rules may share a test body but the edition inputs cannot disappear
behind a claim of one baseline fixture. Appendix examples must be isolated
with annotations removed only in separately labelled synthetic fixtures;
nonconforming examples stay nonconforming. Real sample checks remain local-
only and unavailable in this cloud assignment; none was accessed or claimed.

## Conclusion and gates remaining

The inspected sources support keeping approved INT-035/036/037 unchanged.
This written analysis replaces the 'not yet written' link for these three
only; it does NOT mark the analysis owner/AI-reviewed, tests executed or
implementation ready. Source snapshots, methods and limitations are explicit.
Analysis/tests for all other approved INTs remain separate remaining work.

Executed locally: 19 PDF SHA checks, bounded source-text extraction/search,
manual passage inspection and documentation/trace consistency checks. No
Rust build, parser test, fuzz run, real-recording or reference-tool execution.
No dependency installation, new default, repair, Unicode support, output
format or source-availability/governing policy is introduced. Existing CI
checks validate this docs change, not the future intensive-test plan.
