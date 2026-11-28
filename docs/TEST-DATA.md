# Test Data and Test Oracles

Where real TMATS and Chapter 10 data comes from, how it is used, and what
reference tools we compare against. This project does **not** host or manage
a data set: real files are downloaded to a developer's machine and used
locally; CI runs on synthesized examples and fuzzing only.

## Policy

- **Real recordings never enter this repository or CI.** They are fetched by
  a developer into a local directory outside the repository. Tests that need
  them are opt-in and skip cleanly when the directory is absent (the exact
  mechanism is decided with the test architecture in the design phase).
- **Spec-sourced fixtures**: cite the archived Chapter 9 Appendix 9-C and
  Appendix 9-E (§E.6.b, §E.9.a–d) material used. Do not treat annotated
  Appendix 9-E example text as a conforming TMATS-file fixture: trailing
  annotations after a semicolon become the next region's code-name bytes.
  Use isolated, cited attribute/grammar fixtures with extraction documented;
  verbatim annotated blocks belong only in explicitly nonconforming tests,
  with original bytes retained. Existing Appendix 9-C suspect cases remain
  suspect rather than silently repaired.
- **CI uses synthesized fixtures**: small TMATS documents written for a
  specific behaviour, each citing the Chapter 9 clause it exercises, plus
  simulated variants of the conditions observed in real files (vendor quirks,
  non-conforming attributes, odd bytes), and fuzzing.
- **When a real file reveals a behaviour**, it is reproduced as a synthesized
  fixture in the same change, so CI keeps the lesson without the file.

## Owner-signed CHE and numeric field-value expectations

INT-054/055 record Joey's Option A decisions of 2026-11-25. The former
pending-OWNER blockers are removed and expected results below are specified.
INT-054/055 have final AI CONCUR at #51 ebd5c98; source freshness,
written development analysis and executable verification remain required.
These are PLANNED tests, not enabled runtime coverage or approval to implement.

Decimal-integer fields, including R\N condition operands and numeric-looking
TTF/SHTF/TFMT codes, use RAW nonempty ASCII digits, maximum ten before stripping
zeros. Leading zeros give a located warning and canonical comparison; 04=4,
02 matches code 2. Invalid syntax/range never repairs/clips/wraps input.
A leading blank is uniformly unavailable under INT-041; trailing/embedded
blanks are Invalid. Any raw control makes a number Invalid; INT-036 never
repairs it. Binary/masks, float/hex and all other non-decimal representations
are out of scope, even with numeric-only spelling. Numeric codes come from
explicit reviewed enumeration tables, never binary patterns; see the
[complete inventory](research/2026-11-26-int055-binary-exclusions.md). Character findings, missing/duplicate states, field-specific
ranges and independently supplied unit premises retain their contracts.
Raw binding does not assert a usable numeric value. See the new focused
fixture table below for lexical/range/condition expectations.

## Family 2 batch 1: foundation acceptance specifications

**Status: planned, not executable tests.** These checks accompany the first
L2/L3 foundation batch. None is evidence until implemented and run on the
replacement. They are not new interpretations of IRIG 106 or a claim that
the old prototype satisfies the replacement requirements.

For the restructure, the oracle is owner-decided ADR-0035 (overriding only
ADR-0034 W1), unchanged W2/W3, and the lockstep rule. For source storage,
it is accepted ADR-0002/0003 plus the
independent original byte vector. Byte-only fixtures below test storage,
not standards conformance: they deliberately need no invented attribute
meaning, keyword, default or delimiter-recovery policy.

### Restructure and host integration

| Planned check | Arrange and act | Required observation | Requirement |
| --- | --- | --- | --- |
| `virtual_workspace_retains_lockstep` | Inspect parsed root and CLI manifests, targets, workspace version and library dependency | No root package; members are crates/irig106-tmats and apps/irig106-tmats-cli; shared version and exact path dependency are inherited; CLI retains library and binary targets | L3-REL-001 |
| `host_runner_returns_to_caller` | A subprocess driver supplies arguments and two capture writers to `run`, then emits a sentinel after it returns; exercise success and failure invocations | Sentinel reached in both cases, output is in the supplied writers, returned class matches the documented command outcome | L3-CLI-001 |
| `thin_binary_matches_host_runner` | Run the same implemented invocation through the binary and host adapter with captured streams | Same reports, diagnostics and equivalent exit classification; inspect the thin main for duplicated command behavior | L3-CLI-002 |
| `command_and_renderer_can_be_hosted` | Call an implemented command directly, then render its report into memory without calling the runner | Report available independently; rendering uses the supplied writer and no process exit | L3-CLI-003 |

The manifest check must fail if only the library version changes or the CLI
pin ceases to be exact. During restructure, removing prototype code must not
be reported as implementing commands that remain scaffolds. Command outcome
fixtures and argument representations follow the command-specific batch;
only implemented entry points can supply acceptance evidence.

The I/O-boundary inspection for L3-IO-001 must inspect public APIs and their
reachable input/output acquisition paths. Record actual reviewed locations
before adding an Evidence line. A capture-writer test is useful but cannot
prove no hidden environment, file, network or process access exists.

### Source ownership, order and output

| Planned check | Arrange and act | Required observation | Requirement |
| --- | --- | --- | --- |
| `document_owns_original_bytes` | Create the document from a mutable byte vector, then overwrite/drop the caller vector | Document source remains exactly the independently retained original | L3-READ-001 |
| `raw_access_preserves_non_utf8` | Supply source containing `00`, `FF`, an invalid UTF-8 sequence and mixed line endings; inspect raw spans and request a decoded view | Raw slices remain byte-exact before and after the view request | L3-READ-002 |
| `ordered_items_retain_duplicates` | Use a scanner-approved fixture with repeated keys, unknown attributes and malformed retained material | Ordered enumeration preserves all supplied occurrences; a derived lookup cannot replace the item sequence | L3-READ-003 |
| `unedited_output_is_original_source` | Compare default output to the original byte vector, including retained separators, gaps and a malformed suffix | Exact byte equality, including final-newline presence or absence | L3-WRT-001 |
| `default_write_propagates_sink_failure` | Supply a writer that fails after a chosen prefix of a nonempty source | Error returned; no false successful completion or retry that changes the output bytes | L3-WRT-001 |

Ownership and raw-range checks can first use storage fixtures constructed
without parsing. Parser round trips and item classification remain blocked
on the next scanner requirements/interpretation batch; do not invent a
resynchronization rule merely to create a passing foundation test.

After the scanner exists, generated arbitrary byte vectors must compare
output directly to the original and exercise recovery findings as separately
specified. Empty input, repeated identical bytes and different newline
endings belong in the matrix; comparing only decoded fields is insufficient.

### Span arithmetic

| Planned check | Inputs | Required observation | Requirement |
| --- | --- | --- | --- |
| `span_constructor_checks_boundaries` | Empty range at zero/end; exact-end range; start beyond end; length beyond remaining source; values around `usize::MAX` | Valid ranges accepted, invalid/overflow ranges rejected without a panic or oversized allocation | L3-ROB-001 |
| `span_access_rechecks_actual_source` | Reuse a span constructed for a longer source against a shorter source; include an empty source and valid exact-end access | Safe slice or explicit rejection according to actual bounds; never unchecked indexing | L3-ROB-002 |

Future test markers must name the actual L3 obligations they check. Do not
attach them to these prose cases or to legacy tests that assert a different
contract. Coverage of these foundations does not complete their broader L1
parents: later feature batches still supply the remaining decomposition.

## Family 2 batch 2: scanner framing acceptance specifications

**Status: planned, not executable tests.** Framing recovery depends on
owner-approved INT-035 (Joey, 2026-11-21; independent AI concurrence
referenced to PR #13). Only framing
and ASCII comparison are covered here; raw code-name validity, character
meaning and permitted layout require subsequent derivations before parser
implementation. The case names below are prospective test names.

The valid fixture code COMMENT comes from 106-24R1 Chapter 9 section 9.4.2,
page 9-3; framing rules are in the same section, page 9-2. Malformed fixtures
are deliberately nonconforming recovery cases, not valid configurations.

| Planned check | Fixture or stimulus | Required observation | Requirement |
| --- | --- | --- | --- |
| `first_colon_and_semicolon_keep_raw_ranges` | `COMMENT:a:b;COMMENT:c;` | Two raw regions; first value is `a:b`; component and delimiter spans select exact original bytes | L3-READ-004, L3-READ-005 |
| `missing_colon_recovers_at_semicolon` | `BROKEN;COMMENT:ok;` | Retain first region with missing-colon finding; do not borrow the later colon; retain later COMMENT region | L3-READ-004, L3-READ-006, L3-READ-007 |
| `unterminated_suffix_is_retained` | `COMMENT:ok;BROKEN` and `G\SHA:2-incomplete` | Earlier material retained; suffix retained with finding and no explicit lookup value, consistent with INT-019; no fabricated delimiter | L3-READ-006, L3-READ-007 |
| `code_like_value_text_does_not_split` | `COMMENT:a COMMENT:b;` | One raw value containing the later colon/code-like text; no silent split or edit | L3-READ-005 |
| `quoted_semicolon_does_not_escape_framing` | Nonconforming `COMMENT:"a;b";` | Actual semicolon still ends the first raw region; quotes do not invent escape semantics or swallow the later malformed region | L3-READ-004, L3-READ-005, L3-READ-006 |
| `line_endings_are_not_terminators` | `COMMENT:a\r\nb;` and `COMMENT:a\rb;` | CR/LF never split the raw attribute; bytes remain in the raw span pending character interpretation | L3-READ-005 |
| `later_colon_cannot_validate_missing_separator` | Nonconforming `G\COM Time 12:30;` | Framing uses its colon but code-name-validity batch must report the invalid code name; never claim recovery of the intended item | L3-READ-004, L3-READ-007 |
| `empty_value_is_present` | `X:;` | Well-framed zero-length raw value, distinct from an absent item; unknown/invalid-code findings remain separate from framing | L3-READ-004 |
| `empty_code_name_is_reported` | `:v;` | Retain the complete region and value bytes, with an empty-code-name finding; no fabricated key | L3-READ-004, L3-READ-007 |
| `stray_value_semicolon_preserves_uncertainty` | Damaged intended value represented by `COMMENT:alpha;tail;` | Actual boundary splits candidates; later invalid/unknown-code/duplicate checks are the defence, not proof of intended meaning | L3-READ-004, L3-READ-007 |
| `eof_padding_is_not_an_unterminated_attribute` | A terminated attribute followed only by NUL, 0x1A or space padding | Retain padding silently, with no missing-terminator finding; character classification is a prerequisite | L3-READ-010 |
| `code_name_line_endings_require_character_policy` | CR/LF before/inside a code name and a lone CR | No line-ending terminator; retain bytes; character-batch classification must precede semantic lookup | L3-READ-005, L3-READ-008 |
| `encoding_markers_require_character_policy` | UTF-8 BOM and UTF-16 input | Retain original bytes, do not guess/transcode; finding/support behavior remains a character-batch prerequisite | L3-READ-007 |
| `empty_region_run_has_one_information_finding` | `;;;;` | Exactly one information-level finding with source span `[0, 4)`; preserve all four semicolons | L3-READ-009 |
| `two_empty_regions_have_one_information_finding` | `;;` | Exactly one information-level finding with source span `[0, 2)` | L3-READ-009 |
| `layout_only_terminated_region_has_information_finding` | `\r\n;` | Exactly one information-level finding with source span `[0, 3)` | L3-READ-009 |
| `mixed_empty_layout_run_has_one_information_finding` | `;;\r\n;` | One information finding over `[0, 5)`, merging the adjacent empty and layout-only terminated regions | L3-READ-009 |
| `nonempty_region_separates_empty_runs` | `;;COMMENT:x;;;` | Two information-level findings with spans `[0, 2)` and `[12, 14)`; never merge across COMMENT | L3-READ-009 |
| `empty_value_and_empty_name_are_not_empty_regions` | `X:;` and `:v;` | Neither belongs to an empty-region run; preserve existing empty-value/empty-name distinctions | L3-READ-004, L3-READ-009 |
| `unterminated_layout_at_eof_is_silent` | `\r\n` alone, then `;;;;\r\n` | Layout-only input has no finding; second input has one information finding over `[0, 4)` only; EOF suffix retained silently | L3-READ-009, L3-READ-010 |
| `code_name_blanks_do_not_change_raw_comparison` | `C-6\DCN :DMC;` (isolated attribute derived from Appendix 9-E section E.9.c, not the annotated example block) | Code-name view ignores surrounding blanks per L1-READ-006, then compares; raw span and comparator input behavior stay unchanged | L3-READ-008 |
| `ascii_comparison_changes_no_source` | Mixed ASCII letter case, spaces, punctuation and high bytes as comparator inputs | Only ASCII letters fold for comparison; lengths and all source bytes unchanged; no semantic-validity assertion | L3-READ-008 |
| `framing_retains_findings_and_source` | Malformed and well-framed regions interleaved | Every raw byte retained; findings point inside source and do not erase retained material | L3-READ-007 |
| `framing_cursor_terminates` | Empty source, runs of semicolons, missing-colon runs, unterminated suffix and generated arbitrary bytes | Cursor advances or finishes, never exceeds bounds or revisits a boundary; at most N+1 region steps | L3-ROB-003 |

Expected byte ranges are computed from hand-written fixture positions, not
from the scanner under test. The raw-source equality oracle is the original
input vector. Generated arbitrary bytes test framing/termination and do not
validate those bytes as compliant TMATS. Character-batch prerequisites,
from section 9.4.1 and the review:

- A suffix consisting solely of trailing NUL/0x1A/space padding at EOF does
  not get a missing-terminator finding. Retain it; do not use padding removal
  to hide an otherwise unterminated attribute containing other bytes.
- Specify CR/LF inside/before a code name and a lone CR; none is a terminator.
- Specify UTF-8 BOM and UTF-16 handling without guessing an encoding or
  transcoding original bytes.

Empty/layout-only **terminated** regions have the owner-decided run-level
information finding above; unterminated layout/padding is silent. These
prerequisites and code-name/duplicate/unknown-code findings must precede implementation; framing success alone is not
semantic validity. Use INT-035 markers on actual recovery tests
when they are implemented, alongside their L3 requirement markers.

### Planned suspect-terminator suggestion (not implemented)

Plan a future case with a line break inside a value followed by a complete
code name and colon, for example `COMMENT:first\nCOMMENT:second;`. This remains
one raw value under current framing. The future finding is suggestion-only:
never split, insert a terminator or apply an edit automatically. Complete
code-name recognition, requirement derivation and tests are future work;
this is neither a conforming two-attribute fixture nor an implemented check.
It supplements rather than silently changes L1-READ-007's existing pattern.

## Family 2 batch 3: character-policy acceptance specifications

**Status: planned, not executable tests.** INT-036/037 were approved by
Joey, 2026-11-22, under ADR-0033, with the independent AI recheck concurring
on PR #16 head `891f77f` (see the
[sign-off record](research/2026-11-22-int036-int037-owner-signoff.md)).
Approval is not implementation or passing tests; full development analysis
and executable checks remain outstanding. INT-035 remains approved,
including merged run findings and EOF silence. Full code-name grammar and
index parsing remain later work.

Sources: 106-24R1 Chapter 9 sections 9.4.1/9.4.2, page 9-2 (characters and
blanks), section 9.4.2 page 9-3 (COMMENT); RFC 20 sections 2 and 4.1/4.2
(ASCII positions); Unicode's BOM table. The draft analysis links each.
Escapes in the table denote actual fixture bytes, not backslash text to pass
to the parser. Raw and interpreted expectations must be independently set.

| Planned check | Fixture or stimulus | Expected observation under the approved policies | Requirement |
| --- | --- | --- | --- |
| `control_projection_preserves_raw_spans` | `CO\r\nMMENT:v;` | Interpreted name COMMENT; raw name span `[0, 9)`; contributing source offsets `[0,1,4,5,6,7,8]`; located information finding on omitted CR/LF `[2, 4)`; no invented raw contiguous spelling | L3-READ-011, L3-READ-012, L3-READ-017 |
| `crlf_before_name_is_not_a_delimiter` | `\r\nCOMMENT:v;` and `\rCOMMENT:v;` | Prefix controls omitted for name meaning; no control finding for leading layout; one attribute, full original bytes retained | L3-READ-011, L3-READ-013, L3-READ-017 |
| `controls_in_values_are_not_spaces` | `COMMENT:a\tb;` and `COMMENT:a\r\nb;` | Interpreted value ab under INT-036; raw control bytes retained and used for human display (label any escaping/decoding); no automatic space or terminator insertion | L3-READ-011, L3-READ-012 |
| `code_name_and_value_blanks_differ` | `  COMMENT :  value  ;` | Name COMMENT after surrounding-space trim; value keeps both leading and trailing spaces; comparator does not trim | L3-READ-013 |
| `embedded_name_space_is_not_repaired` | `COM MENT:v;` | Embedded ordinary space retained for later invalid-code finding; never silently change it to COMMENT | L3-READ-013 |
| `projected_empty_value_is_still_present` | `COMMENT:\x00;` versus `COMMENT:;` | Both have a present zero-length interpreted value; different raw value spans/bytes; NUL case alone has one information finding at `[8, 9)`; neither becomes absent | L3-READ-011, L3-READ-013, L3-READ-018 |
| `character_classes_keep_int035_run_policy` | `;\t\r\n\x7F;` | One information finding over `[0, 6)`; preserve bytes; no extra control findings | L3-READ-014 |
| `all_layout_eof_suffix_is_silent` | Terminated COMMENT followed by space/NUL/SUB/DEL/CR/LF | Retain suffix silently; never treat a colon/high byte as part of that layout-only class | L3-READ-014 |
| `high_bytes_are_not_silently_decoded` | COMMENT value containing FF or valid UTF-8 multibyte text | Located non-ASCII finding; raw bytes retained; affected field is Invalid with no usable interpreted value, with no implicit UTF-8/Latin-1 conversion | L3-READ-016 |
| `leading_encoding_marker_blocks_semantic_lookup` | Each of EF BB BF, FE FF, FF FE, 00 00 FE FF and FF FE 00 00 at assembled-text offset zero plus data | Located unsupported-marker finding and retained source; no semantic lookup, stripping or transcoding | L3-READ-015 |
| `partial_or_midstream_marker_is_not_a_leading_bom` | EF BB without BF; EF BB BF after a terminated COMMENT | Ordinary located high-byte findings, not a silently removed leading marker; retain source | L3-READ-015, L3-READ-016 |
| `unmarked_zero_patterns_do_not_select_encoding` | Unmarked ASCII bytes interleaved with NUL | No automatic UTF-16 selection; approved raw ASCII/control policy applies with one NUL information finding per affected field (plus internal-name omission findings where applicable), explicitly not proof of original encoding | L3-READ-011, L3-READ-015, L3-READ-018 |
| `name_edge_layout_stays_silent` | `\tCOMMENT\r\n:v;` | No control finding for name-edge layout; NUL is tested separately | L3-READ-017 |
| `nul_fields_have_one_information_finding_each` | `\x00\x00COMMENT:v\x00x\x00;` | Exactly one NUL information finding for name at `[0, 2)` and one for value identifying `[11, 12)` and `[13, 14)`; preserve all bytes | L3-READ-018 |
| `internal_name_nul_rules_both_apply` | `CO\x00MMENT:v;` | One NUL-field information finding and an internal-name omission information finding, both located at `[2, 3)`; no encoding guess | L3-READ-017, L3-READ-018 |
| `latin1_display_does_not_validate_field` | `COMMENT:Temp \xB0C;` (Latin-1 degree sign) | Value is Invalid with no usable interpreted value; raw bytes and explicitly labelled on-request `as_latin1`/lossy display remain available | L3-READ-012, L3-READ-016 |
| `split_marker_is_recognised_after_assembly` | Every recognized marker split at each internal byte boundary across Format 1 fragment packets, each with its own CSDW | Remove packet framing/CSDWs and assemble text first; same located unsupported-marker finding and blocked semantic lookup as unsplit text, with fragment provenance | L3-READ-015 |
| `marker_after_padding_is_midstream` | `\r\n\xEF\xBB\xBFCOMMENT:v;` and `\x00\xEF\xBB\xBFCOMMENT:v;` | High-byte findings and Invalid affected field, not a leading-marker finding or document-wide marker gate; NUL variant additionally has one NUL-field information finding; no padding skip/retry | L3-READ-015, L3-READ-016, L3-READ-018 |

The byte-class tests also enumerate all 256 possible byte inputs to the
projection/comparator helpers without asserting they are conforming field
values; actual colon/semicolon framing remains a separate operation. Check
that a high byte cannot disappear to make a different apparently valid key.
Use independent expected offsets rather than projection lengths as source
locations. The `[0, 6)` run comprises two terminated regions, including the
TAB/CR/LF/DEL bytes, not six separate diagnostics.

Known marker handling precedes semantic control omission; a BOM-marked
UTF-16/32-like stream must not become usable by deleting its zeros. Offset
zero is the first assembled text byte after Format 1 CSDW removal, never
packet/file offset zero. Unmarked lookalikes get the NUL-field findings
without an encoding guess; incomplete encoding detection remains a limitation.
No annotated Appendix 9-E text is a conforming-file fixture here.
Keep the future line-break hint on original bytes: losing CR/LF from an
interpreted value must not lose its raw location or implement that hint now.

## Family 2 batch 4: code-name acceptance specifications

**Status: planned cases, not executable tests.** Structural/recognition
requirements are drafted; INT-038 was approved by Joey, 2026-11-22, under
ADR-0033, with independent AI concurrence on PR #20 head `29707da` (see the
[sign-off record](research/2026-11-22-int038-owner-signoff.md)).
INT-035/036/037 remain approved. Approval is not implementation or passing
tests; full development analysis and executable checks remain outstanding.

Sources: 106-24R1 Chapter 9 sections 9.4.2 and 9.5.1 a/Table 9-1,
pages 9-2/3 and 9-5; Table 9-2 page 9-10; Table 9-4 page 9-48;
Table 9-7 pages 9-115/117; sections 9.5.12/13/14 pages 9-171/172.
The [baseline draft](research/2026-11-22-code-name-index-draft.md) identifies
rows and separates cited syntax from the owner-approved numeric identity policy.
Escapes below denote fixture bytes. Synthetic malformed/unknown examples
are deliberate stimuli, not claims that Chapter 9 defines those attributes.

| Planned check | Fixture or stimulus | Expected observation | Requirement |
| --- | --- | --- | --- |
| `comment_is_a_whole_name` | `comment:v;`, `COMMENT-1:v;`, `COMMENT\X:v;` | First is standalone COMMENT; no fabricated occurrence/path; other two are not COMMENT by prefix matching, remain retained | L3-READ-019 |
| `all_baseline_group_identifiers_are_recognised` | Each Table 9-1 identifier in both ASCII cases | Same group identity, original spelling retained; identifier recognition alone makes no claim of a valid complete attribute | L3-READ-020 |
| `literal_digits_are_not_indices` | `G\106:24;`, `R-1\TK1-3:42;` | G path [106], no occurrence; R occurrence 1, literal TK1, suffix index [3]; value 42 is not an index inferred from the name | L3-READ-021 |
| `nested_indices_keep_positions` | `D-1\MN-1-2:MEAS1;` | Occurrence 1; path MN with indices [1, 2]; raw index spans `[2, 3)`, `[7, 8)`, `[9, 10)` | L3-READ-022 |
| `multiple_path_levels_keep_index_attachment` | `D-1\MNF\N-2-3-4:1;` | Occurrence 1; MNF has no suffix indices; N has [2, 3, 4]; name spelling retained | L3-READ-022 |
| `index_order_and_arity_are_not_flattened` | MN suffixes `-1-2`, `-2-1`, `-1-2-3` | Different complete structures; third cannot match the two-entry-index definition by dropping its last token; no conformance claim for that extra-index name | L3-READ-021, L3-READ-022, L3-READ-026 |
| `missing_occurrence_is_not_defaulted` | `R\TK1-3:42;` | Never manufacture occurrence 1 or bind as `R-1\TK1-3`; raw name retained; definition matching cannot accept it as a complete instance of that cited pattern | L3-READ-021, L3-READ-024 |
| `decimal_index_spelling_preserves_numeric_identity` | `D-01\MN-001-02:a;D-1\MN-1-2:b;` | Under approved INT-038 both keys have canonical indices (1; 1, 2); per-token information findings for 01/001/02 plus a normalization-only full-key collision information finding listing both occurrences now; retain both items/order, never choose a winner | L3-READ-023, L3-READ-026, L3-READ-027, L3-READ-029 |
| `zero_index_is_not_range_approval` | `D-0\MN-1-0:v;` | Zero tokens structurally retained under INT-038; do not claim legal counter entries, silently add 1 or erase them | L3-READ-023, L3-READ-026 |
| `large_index_does_not_wrap_or_allocate_by_value` | A 40-digit nonzero decimal token at a standard index position | Full raw token retained; equality/hash use its canonical decimal string without fixed-width or big-integer conversion; failed optional bounded conversion explicit; no wrap/saturation/truncation or array up to that number | L3-READ-023 |
| `index_token_grammar_is_not_value_numeric_grammar` | Expected index replaced with empty token, `+1`, `1.0`, `1e2`, or template `x` | No match as a decimal index under INT-038; retained input and located name finding, no partial usable key or invented index | L3-READ-023, L3-READ-024 |
| `internal_blank_name_is_not_a_partial_key` | `G\COM Time 12:30;G\PN:ok;` | First raw name gets a finding; value is 30; never expose it as G\COM; second item retained/read independently | L3-READ-024 |
| `broken_components_preserve_following_items` | `D-1\MN-1-:v;G\PN:ok;` | No usable partial MN key; locate unfinished index, retain all bytes; next region still read | L3-READ-024 |
| `unknown_does_not_mean_malformed` | Synthetic `Z-1\EXPERIMENT-2:v;` | Unrecognized group/definition remains retained; no replacement with a known group; no syntax failure solely because no catalogue entry exists | L3-READ-020, L3-READ-025 |
| `private_names_are_not_forced_into_standard_keys` | H/private input and V attribute-string containing unfamiliar punctuation | Retain opaque raw portions where decomposition is uncertain; lack of a built-in pattern alone is not evidence to drop them or claim malformed syntax/conformance | L3-READ-025 |
| `extension_structure_is_not_link_resolution` | Isolated `X-1\MYORG\D\MN\SGAIN-1-2:10.75;` from section 9.5.14 | Keep path levels and indices where unambiguous; no resolved association, valid configuration or extension registry entry inferred by parsing | L3-READ-022, L3-READ-025, L3-READ-026 |
| `noncanonical_index_is_visible` | `R-00\TK1-01:v;`, compared with canonical tokens `0` and `1` | Located information findings on noncanonical token spans `[2, 4)` and `[9, 11)`; canonical strings 0 and 1; canonical 0/1 alone have no normalization finding; range validity remains separate | L3-READ-027 |
| `normalization_only_collision_is_reported_now` | `D-1\MN-1-2:a;D-01\MN-1-2:b;D-001\MN-1-2:c;` | Collision information finding lists all three occurrences/order now, in addition to token findings; no value agreement, winner or consolidation inferred; general duplicate classification still later | L3-READ-029 |
| `collision_group_includes_exact_repeats` | `D-1\MN-1-2:a;D-1\MN-1-2:a;D-01\MN-1-2:b;D-1\MN-1-2:a;` | One canonical-key group with two distinct raw index-spelling tuples; collision information lists all FOUR occurrences in order, including the three exact repeats; token finding for 01 remains separate | L3-READ-029 |
| `identical_index_spellings_do_not_trigger_normalization_collision` | Repeat `D-01\MN-1-2:a;` twice, then vary only code-letter case or value | Same raw index-spelling tuple throughout: no normalization-collision finding; each noncanonical 01 token still has its own information finding; general duplicate classification remains later | L3-READ-027, L3-READ-029 |
| `projected_index_controls_keep_locations` | `R-1\r\n0\TK1-1:v;` | INT-036 projection yields occurrence digits 10, canonical string 10 and internal-omission information finding on `[3, 5)`; raw occurrence span `[2, 6)` retained, no normalization finding solely for controls | L3-READ-023, L3-READ-024 |
| `canonical_string_equality_and_hashing` | Index digits `1234567890123456789012345678901234567890` and the same 40 digits prefixed with `00` in otherwise identical keys | Equality and hash consume the same canonical 40-digit string without any big-integer/fixed-width conversion; equal keys hash equally; padded token gets a finding; different identities need not have different hash outputs | L3-READ-023, L3-READ-027, L3-READ-029 |
| `index_blanks_are_not_local_trim` | `R- 1\TK1-1:v;`, `R-1 \TK1-1:v;`, and `R-1\TK1-1 :v;` | First two have internal blanks and located name findings, no repaired usable key; third has only whole-name trailing layout, handled under approved INT-036, not an internal-index failure | L3-READ-024, L3-READ-027 |
| `x_original_indices_use_numeric_identity` | `X-1\MYORG\D\MN\SGAIN-1-2:10.75;` versus `D-01\MN-01-02:MEAS1;`, then pad the corresponding X indices too | Corresponding occurrence/carried original indices compare by canonical strings 1/1/2 in both directions; padded tokens get findings; X/D remain different full keys with no full-key collision or resolved association inferred | L3-READ-028, L3-READ-027, L3-READ-029 |
| `character_findings_survive_key_parsing` | `CO\r\nMMENT:v;`, a NUL field and a high-byte name | Keep the approved control/NUL findings and raw spans; high-byte field is Invalid, not rescued by a partial ASCII key; leading markers retain INT-037 gating | L3-READ-024 |

Expected component positions and numeric identities must be specified without
calling the implementation under test. Compare projections against original
source correspondence, not shortened string offsets. Generate bounded case
variants, index permutations and digit lengths around conversion limits;
assert unchanged source, bounded work and continued framing after failures.
Table-derived sample attributes here exercise structure only, not all required
fields for a conforming file. Detailed unknown-code findings, duplicate
classification (apart from the normalization-only collision finding required
now), grammar for private extensions and counter/link validation remain
subsequent derivations; these cases do not complete those L1 parents.

The `\N` count and no-missing-values checks must count DISTINCT canonical
numeric identities, not raw spellings or repeated occurrences. Counter batch
6 below derives scoped requirements and planned checks, including the section
9.5.14 X noncontiguity exception; no executable checks are supplied yet. The
index cases here check identity/visibility only, not counter acceptance.

## Family 2 batch 5: duplicate and exact-lookup acceptance specifications

**Status: planned, not executable tests.** INT-039 was approved by Joey,
2026-11-22, under ADR-0033, with independent AI concurrence on PR #22 head
`b22d45c` (see the [sign-off record](research/2026-11-22-int039-owner-signoff.md)).
Earlier interpretations remain approved; full development analysis and
executable tests remain outstanding.
For scalar cases, the fixture supplies the cited definition binding; this
is not evidence of a completed production registry or reviewed value rules.

Sources: 106-24R1 Chapter 9 section 9.4.2 pages 9-2/3 (unique code names,
numbered entries, COMMENT and ordering); Table 9-2 PROGRAM NAME page 9-10;
ADR-0002/0003/0023, approved INT-038 and owner-approved F1/INT-021 (Joey, 2026-11-23; originally ADR-0034).
The [baseline draft](research/2026-11-22-scalar-duplicates-draft.md) separates
literal standard text from owner-approved duplicate recovery/COMMENT treatment.
Group \COM examples cite Table 9-2 COMMENTS, page 9-13, and Table 9-4
COMMENTS, page 9-84; assembly scope follows ADR-0025.
Escapes below denote bytes. Example attributes test candidate behavior, not
whether the whole fixture is a conforming configuration.

| Planned check | Fixture or stimulus | Expected observation under approved INT-039 | Requirement |
| --- | --- | --- | --- |
| `identical_scalar_repeats_are_ambiguous` | `G\PN:a;G\PN:a;` | Two retained candidates; one scalar-duplicate finding referencing both; singular value Ambiguous even though bytes agree; item spans `[0, 7)`, `[7, 14)`, value spans `[5, 6)`, `[12, 13)` | L3-READ-030, L3-READ-031, L3-VIEW-001, L3-VIEW-003 |
| `conflicting_scalar_repeats_keep_every_candidate` | `G\PN:a;G\PN:b;G\PN:a;` | Three candidates, exact repeats included; duplicate finding and Ambiguous list all in source order, no preferred value | L3-READ-031, L3-VIEW-003 |
| `invalid_candidate_is_not_filtered` | `G\PN:ok;G\PN:\xFF;` | High-byte value remains an invalid raw candidate under INT-037; singular slot is Ambiguous with both candidates and the non-ASCII finding, never explicit ok by filtering | L3-VIEW-001, L3-VIEW-003 |
| `projected_value_agreement_does_not_coalesce` | `G\PN:a\tb;G\PN:ab;` | INT-036 projects first value to ab, but raw items remain distinct; singular multiplicity is still Ambiguous without value-equivalence selection | L3-VIEW-003, L3-VIEW-004 |
| `no_match_is_not_present_empty` | Empty document versus `G\PN:;` | Exact raw lookup returns zero versus one candidate; latter value span `[5, 5)` remains present; no automatic default/validity claim | L3-VIEW-001, L3-VIEW-002 |
| `numbered_entries_are_separate_slots` | `G\DSI-1:Aircraft;G\DSI-2:Missile;` | Distinct complete keys; not duplicate slots merely because both match the DSI pattern; full counter/condition validity not asserted | L3-READ-030, L3-READ-031 |
| `repeated_numbered_slot_is_duplicate` | `G\DSI-1:Aircraft;G\DSI-1:Aircraft;` with scalar-per-slot definition binding | One numbered slot repeated; two candidates, duplicate finding and Ambiguous, not silently legal because the pattern is multiple-entry | L3-READ-031, L3-VIEW-003 |
| `duplicate_and_normalization_findings_coexist` | Repeated scalar slot with raw indices 1, 1, 01, 1 | Scalar-duplicate finding and Ambiguous list all four; INT-038 group information also lists all four and its noncanonical token finding remains separate | L3-READ-033, L3-VIEW-003 |
| `one_index_spelling_still_has_scalar_duplicates` | Same scalar slot spelled 01 twice | Duplicate finding and Ambiguous; per-token information twice; no INT-038 group collision with just one distinct raw spelling tuple | L3-READ-033, L3-VIEW-003 |
| `comment_collection_is_not_a_scalar_duplicate` | `COMMENT:first;COMMENT:second;` | Ordered two-item COMMENT collection retained without a duplicate-scalar finding merely for repeat; no concatenation or fabricated single semantic value | L3-READ-032, L3-VIEW-001 |
| `group_com_is_a_single_entry_duplicate` | `G\COM:first;\r\nG\COM:second;`; separately `R-1\COM:a;R-1\COM:b;R-2\COM:c;` | G slot and R-1 slot each have a duplicate finding listing both occurrences and an Ambiguous singular value; raw lookup returns every line, never concatenates; R-2 is a separate slot; standalone COMMENT exception does not apply | L3-READ-031, L3-READ-032, L3-VIEW-001, L3-VIEW-003 |
| `unknown_cardinality_is_not_assumed_singular` | Two identical synthetic `Z-1\PRIVATE:v;` items without a reviewed definition | Raw multiplicity visible where the key is structurally usable; scalar-duplicate conformance explicitly unclassified, not valid/singular by assumption; source retained | L3-READ-032, L3-VIEW-001 |
| `invalid_name_cannot_be_a_partial_lookup_key` | A high-byte code name or `G\PN extra:v;`, followed by `G\PN:ok;` | Bad-name item remains in source with findings, not fabricated into G\PN; exact lookup does not confuse it with the later well-named candidate | L3-VIEW-001 |
| `marker_block_is_not_missing_key` | Leading UTF-8/UTF-16/UTF-32 marker under INT-037 | Exact interpreted lookup reports the marker block and finding, not an empty-success/no-match claim; raw source remains available | L3-VIEW-001, L3-VIEW-002 |
| `order_and_severity_do_not_choose_values` | Permute repeated scalar items, then lower diagnostic severity | Candidate report order tracks input; multiplicity remains Ambiguous and raw emission remains byte-exact for each input | L3-VIEW-004 |
| `duplicates_cannot_be_hidden_by_a_default` | Repeated scalar fixture definition with an explicitly supplied test-only default | Ambiguous, never Defaulted; no Chapter 9 default is inferred or introduced by this synthetic metadata | L3-VIEW-004 |
| `candidate_groups_are_document_local` | Two separate documents each containing one G\PN item | Separate exact-lookups yield one item each; no cross-record grouping or setup-governance inference | L3-READ-030, L3-VIEW-001 |
| `cross_fragment_scalar_duplicate_is_document_local` | One assembled Format 1 candidate run: same channel, consecutive packets with sequence q then q+1 modulo 256, matching FRMT/RCCVER, fragment texts `G\PN:a;` and `G\PN:b;`, each packet with its own CSDW, under ADR-0025/0031 | ONE document with two scalar candidates; duplicate finding and Ambiguous list both; assembled item spans `[0, 7)` and `[7, 14)`, values `[5, 6)` and `[12, 13)`, with fragment-1/2 provenance; no packet-local reset; joining prerequisites do not prove completeness/governance, and repeated single-entry text may leave the run Ambiguous under ADR-0031 | L3-READ-030, L3-READ-031, L3-VIEW-001, L3-VIEW-003 |
| `duplicate_sha_keeps_f1_result` | Two G\SHA items (equal or different values) | Retain and list both; generic duplicate handling never yields positive integrity or permits stamping; F1 result remains indeterminate, checksum mechanics later | L3-READ-033 |

Planned generated cases permute small candidate sets and mix exact repeats,
case variants and canonical-equivalent indices. Expected lists are enumerated
from fixed fixture item identities/offsets, independently of any lookup index.
Compare raw lookup with the item store, including invalid values; never use
value success as a candidate filter. A unique raw candidate is not by itself
an Explicit semantic value. Pattern lookup, full default/type validation,
unknown-definition diagnostics, links, record assembly and counter rules are
not completed by these scenarios. No source fixture is changed automatically.

## Family 2 batch 6: counter acceptance specifications

**Status: planned, not executable tests.** INT-040 was approved by Joey,
2026-11-22, under ADR-0033, with independent AI recheck concurrence at
PR #24 head `176fdf4` (see the [sign-off record](research/2026-11-23-int040-owner-signoff.md)).
Earlier interpretations remain approved; full analysis and executable tests
remain outstanding.
Fixtures supply declared rule bindings/applicability for the stated checks;
short attribute snippets are not complete conforming configurations or
production registry evidence. Count-value typing/default resolution remains
its own reviewed prerequisite, not INT-038's index-token grammar.

Sources: 106-24R1 Chapter 9 section 9.5.1 a/Table 9-1 page 9-5; Table 9-7
NUMBER OF MEASUREMENT LOCATIONS, NUMBER OF FRAGMENTS and WORD POSITION,
page 9-117; section 9.5.14 page 9-172; ADR-0023/0026 and INT-038/039.
See the [baseline draft](research/2026-11-22-counter-membership-draft.md).
N below denotes a usable effective count only when explicitly stated.

| Planned check | Fixture or stimulus | Expected observation | Requirement |
| --- | --- | --- | --- |
| `nested_counter_scopes_do_not_merge` | D-x\MML\N-y-n rule with (x,y,n)=(1,1,1), N=2, members m={1,2}; another parent (1,1,2), N=1, members m={1} | Separate cardinalities 2 and 1; do not pool three members or mix the repeated m=1 across parents | L3-REG-001, L3-VAL-001 |
| `fragment_counter_is_one_level_deeper` | D-x\MNF\N-y-n-m with fixed (1,1,1,2), N=2 and WP fragment indices e={1,2}; a WP item under m=3 | Only the first scope's e identities count; different m is a different parent, not an extra fragment | L3-REG-001, L3-VAL-001 |
| `counter_counts_distinct_identities` | Within one declared scope, member index spellings 1, 01, 1, 2 with N=2 | Observed identities {1,2}; cardinality 2, all witnesses retained; approved normalization/duplicate findings remain, never whole-document success | L3-VAL-001, L3-VAL-006 |
| `several_properties_are_one_entry` | Reviewed rule names two child properties as witnesses for the same governed identity 1, plus identity 2 | Two distinct identities, not three property lines; no extra membership pattern inferred from a prefix | L3-REG-002, L3-VAL-001 |
| `invalid_payload_keeps_structural_membership` | `D-1\MNF\N-1-1-1:2;D-1\WP-1-1-1-1:\xFF;D-1\WP-1-1-1-2:4;`, with applicable reviewed fragment rule | e={1,2} structurally counted; first value stays Invalid/non-ASCII, with no overall validity claim; counter span `[0, 18)`, counter value `[16, 17)`, invalid value `[33, 34)` | L3-VAL-001, L3-VAL-005 |
| `ambiguous_member_does_not_inflate_count` | One scalar member slot repeated with different values at e=1, plus e=2, N=2 | Two identities; slot e=1 remains Ambiguous with all occurrences, not counted twice or selected by payload validity | L3-VAL-001, L3-VAL-006 |
| `cardinality_is_not_domain_coverage` | N=2 with observed {1,3}; separately N=3 with {1,3} | First has size 2 but domain/gap failure; second has maximum 3 but missing 2/count mismatch; no false pass | L3-VAL-002 |
| `zero_stays_located_out_of_domain_evidence` | N=2, observed {0,2} | Keep distinct observed count 2, but report index 0 and missing 1; never remove/renumber zero to manufacture success | L3-VAL-001, L3-VAL-002 |
| `permitted_zero_is_not_invented` | Explicit test rule permitting usable N=0 and complete empty membership; contrast an unavailable/missing N | First satisfies only that rule's empty domain; second cannot-evaluate, never guessed 0; no Chapter 9 zero default introduced | L3-VAL-002, L3-VAL-004 |
| `unusable_counter_is_not_inferred` | Missing, Invalid or duplicate/Ambiguous counter (including equal repeats), with observed members {1,2} | Counter check cannot-evaluate with reason/candidates; no inferred N=2 from count or maximum and no selected duplicate | L3-VAL-004, L3-VAL-005 |
| `incomplete_membership_does_not_pass` | Known required member evidence whose governed index/scope cannot be interpreted, plus one identified member | Retain partial observed set/evidence and report cannot-evaluate; do not claim completeness or assign an unrelated malformed field by guess | L3-REG-002, L3-VAL-004 |
| `x_exception_is_not_global` | X occurrences x={2,7} whose originals are known, beside an ordinary contiguous counter with a gap | No X occurrence contiguity failure solely for the gap; ordinary counter still fails; no X counter invented or extension-only indices exempted by analogy | L3-REG-002, L3-VAL-002 |
| `large_index_has_compact_domain_findings` | N=2, members 1 and a 40-digit positive decimal identity | Located excess-index/missing-2 evidence; no index-sized allocation, wrap/truncation or enumeration through that huge identity | L3-VAL-002, L3-VAL-003 |
| `gap_intervals_follow_supplied_members` | Valid rule N=9999 with member indices {1,9999} | Cardinality 2 and compact inclusive missing-index interval [2,9998], not thousands of fabricated member items; report original evidence | L3-VAL-003, L3-VAL-005 |
| `parsed_counter_outside_own_range_blocks` | Applicable NUMBER OF FRAGMENTS counter `D-1\MNF\N-1-1-1:9;`; separately NUMBER OF MEASUREMENT LOCATIONS `D-1\MML\N-1-1:0;` | Both parse as numbers but are Invalid under their own Table 9-7 ranges (1–8, 1–9999, p. 9-117); their own count evaluation cannot-evaluate, even if raw cardinality would match | L3-VAL-004 |
| `x_extension_does_not_create_d_member` | D fragment count N=2, ordinary WP members e={1,2}, plus `X-1\ORG\D\WP\FOO-1-1-1-3:v;` and `X-1\ORG\D\WP\FOO-1-1-1-2-7-9:v;` with extra extension-only indices | D set stays {1,2}, never gains 3 from X's embedded path; section 9.5.14 attachment with no matching D original is a separate link problem; extension-only indices 7/9 remain unclassified in this batch, without their own reviewed rule | L3-REG-002, L3-VAL-001 |
| `invalid_parent_domain_does_not_suppress_child` | Usable MML\N=1 and explicit m=0 or m=2; child MNF\N=2 in that parent scope and WP e={1,2}, with child rule applicable | Report the parent's zero/excess-domain violation; still evaluate that child's own count from explicit parent indices and get structural cardinality/domain match, without validating the parent or whole document | L3-REG-001, L3-VAL-002, L3-VAL-004 |
| `unusable_ancestor_count_does_not_suppress_child` | MML\N missing/Invalid/Ambiguous; explicit m=2, usable child MNF\N=2, WP e={1,2}, with child rule applicable | Ancestor count/domain check cannot-evaluate; child fragment count still evaluates in (x,y,n,m) from explicit indices, no guessed ancestor bound and no parent-validity claim | L3-REG-001, L3-VAL-004 |
| `zero_parent_spellings_share_one_scope` | `D-1\MNF\N-1-1-0:2;D-1\WP-1-1-00-1:4;D-1\WP-1-1-0-2:5;` | 0 and 00 mean the same parent m=0 under INT-038; one child scope with e={1,2}, not two scopes; retain normalization information and parent-domain violation; no parent repair | L3-REG-001, L3-VAL-001 |
| `malformed_governed_token_blocks_known_scope` | `D-1\WP-1-1-1-x:4;` or `D-1\WP-1-1-1-:4;` under declared WP rule | Literal group/path and token arity match, parents (1,1,1,1) readable, governed token invalid/empty: related diagnostic evidence blocks that scope's completeness (cannot-evaluate), contributes no fabricated identity/key; readable different parents affect only their own scope | L3-VAL-004 |
| `malformed_parent_is_unassigned_rule_evidence` | `D-1\WP-1-1-x-2:4;` or wrong-arity `D-1\WP-1-1-1-2-3:4;` under declared WP rule | Literal group/path match, but parent m unreadable or arity wrong: rule-level cannot-evaluate coverage with unassigned evidence, never assign to a guessed parent or certify complete rule coverage; every compatible scope reports partial/cannot-evaluate, NEVER pass even if its observed set already matches N | L3-VAL-004, L3-VAL-005 |
| `compatible_unassigned_evidence_never_passes` | Known fragment-counter scopes (1,1,1,1) and (1,1,1,2) each have usable N=2 and observed e={1,2}, plus `D-1\WP-1-1-x-2:4;` | Both scopes match readable (x,y,n)=(1,1,1); both report partial/cannot-evaluate, NEVER pass; retain already matching observed sets and unassigned rule-level evidence without choosing m or enumerating possible parents | L3-VAL-004, L3-VAL-005 |
| `incompatible_parent_does_not_inherit_unassigned_block` | Same unassigned `D-1\WP-1-1-x-2:4;`, alongside a known scope (1,1,2,1) | Readable n=1 contradicts n=2, so that scope is not blocked by this item; it must still satisfy all its own prerequisites/checks, with no whole-document success claim | L3-VAL-004 |
| `unrelated_malformed_text_does_not_block_counter` | `D-1\WPX-1-1-1-x:v;`, X extension text, an unframed name, or code-like text inside a value | No exact diagnostic literal group/path match as a separately framed D WP name: ignored for this D counter's membership/blocking only; retain all parser/unknown/link findings and never imply document validity | L3-REG-002, L3-VAL-004 |
| `wfp_value_uniqueness_is_outside_int040` | `D-1\WFP-1-1-1-1:1;D-1\WFP-1-1-1-2:1;`, with distinct e identities {1,2} | Entry identities stay distinct; FRAGMENT POSITION value uniqueness in Table 9-7 p. 9-118 is a separate VALUE rule, not an index-count rule or an INT-040 pass/fail claim; reviewed value validation remains later | L3-VAL-001, L3-VAL-005 |
| `counter_findings_keep_scope_and_citations` | Mismatch or cannot-evaluate in one parent scope while another has complete correct membership | Counters-and-indices pass, rule/edition/citation, parent tuple and counter/member locations are explicit; unrelated scope is not silently merged or certified as a whole document | L3-VAL-005 |
| `validation_does_not_repair_counters` | Noncanonical spellings, gaps, invalid values and mismatched N before/after a validation request | Byte-identical source and item order, unchanged declared N/indices and preserved earlier findings; no automatic filling, recomputation or renumbering | L3-VAL-006 |

Expected member sets, scopes and original locations must be independently
specified. Generated checks use small bounded sets with repeated spellings,
index permutations and unrelated scopes. Report structural equality separately
from payload validity. A missing-gap report does not fabricate source spans
for absent members; its counter/scope is evidence for the inferred interval.
No fixture here completes registry review, applicability/default evaluation,
link validation, the four-pass validator or broader release evidence.

## Family 2 batch 7: effective-value acceptance specifications

**Status: planned, not executable tests.** INT-041 was approved by Joey,
2026-11-23, under ADR-0033, with independent AI concurrence on PR #26 head
`0fc0bc8` (see the [sign-off record](research/2026-11-23-int041-owner-signoff.md)).
Full analysis and executable tests remain outstanding. Fixtures explicitly supply reviewed
field/scalar/type/default bindings, known scopes and edition basis except
where unavailable knowledge is the stimulus. These are not evidence of a
completed production registry, condition evaluator or channel/time API.

Sources: 106-24R1 Chapter 9 section 9.5.1 a page 9-6; Table 9-4 TIME FORMAT
page 9-72; Table 9-11 NUMBER OF OCCURRENCES page 9-167; Table 9-2 PROGRAM
NAME page 9-10; ADR-0022/0023/0028 and RELEASE-0.1.md's approved default
contract. See the [baseline draft](research/2026-11-23-effective-value-draft.md).
Escapes denote fixture bytes; short records are not full conforming files.

| Planned check | Fixture or stimulus | Expected observation | Requirement |
| --- | --- | --- | --- |
| `default_source_and_review_are_separate` | Cited TIME FORMAT row versus an unreviewed extracted row containing text Default A | Keep source/provenance for both; only a reviewed binding can authorize a default; source extraction alone is not executable permission | L3-REG-003, L3-REG-004 |
| `default_field_and_prose_origins_remain_distinct` | TIME FORMAT Default A and NUMBER OF OCCURRENCES prose default 1, with applicable reviewed fixture rules | Metadata and any Defaulted result cite the correct edition/table/row and field/prose origin; no regex scan through arbitrary comments establishes a default | L3-REG-003, L3-VIEW-008 |
| `explicit_a_differs_from_defaulted_a` | `R-1\TFMT-3:A;` versus no TFMT-3, with baseline TIMEIN/default applicability and edition basis established | Explicit A has original value span `[11, 12)`; absent-field result is Defaulted A with Table 9-4 p. 9-72 origin/applicability/basis and no input value span; each emits its original bytes unchanged | L3-VIEW-005, L3-VIEW-006, L3-VIEW-008, L3-VIEW-010 |
| `lowercase_tfmt_keeps_raw_case` | `R-1\TFMT-3:a;`, reviewed baseline enum rule | Explicit A under section 9.4.2 case-insensitive meaning; raw value `a` at `[11, 12)` and emitted bytes unchanged, never Defaulted merely because spelling differs | L3-VIEW-006 |
| `blank_enum_requires_reviewed_handling` | `R-1\TFMT-3: A;`, blank-handling rule unavailable | Unavailable/cannot-evaluate, NOT Invalid; preserve raw ` A` at `[11, 13)`, no trim, absence/default conversion or unreviewed blank rejection | L3-VIEW-007, L3-VIEW-011 |
| `present_non_timein_tfmt_is_not_absence` | `R-1\CDT-3:PCMIN;R-1\TFMT-3:A;`, reviewed local enum rules | Present TFMT is Explicit A locally plus an applicability finding; never ignored as absent, Defaulted or mistaken for a valid whole-channel configuration | L3-VIEW-006, L3-VIEW-010 |
| `ambiguous_cdt_makes_default_condition_unknown` | `R-1\CDT-3:TIMEIN;R-1\CDT-3:TIMEIN;`, TFMT absent | CDT remains Ambiguous even with identical values; TFMT is Missing with unknown-condition reason and cannot-evaluate finding citing both CDT candidates, never Defaulted A or a proven false condition | L3-VIEW-005, L3-VIEW-009 |
| `invalid_under_fallback_keeps_basis` | `R-1\TFMT-3:Z;`, local baseline rule selected under a supplied labelled fallback | Invalid with raw Z/reason AND fallback label, applied edition/citation and basis reason; no rewrite of declarations or false claim of declared-edition validity | L3-VIEW-005, L3-VIEW-007 |
| `bad_present_enum_does_not_default` | `R-1\TFMT-3:Z;` with the cited baseline enum rule | Invalid with original raw Z and reason, never Defaulted A | L3-VIEW-007 |
| `present_empty_is_not_absent` | `R-1\TFMT-3:;` and `R-1\TFMT-3:\x00;` | Both present; empty enumeration projection is Invalid under that rule, never absence/default; NUL case retains its information finding and raw byte | L3-VIEW-007 |
| `valid_empty_text_is_not_blanket_invalid` | Synthetic reviewed text rule explicitly allowing empty text, one present zero-length value | Explicit empty with its original zero-length span, not Missing or automatically Invalid; this test rule is not a new Chapter 9 default | L3-VIEW-006, L3-VIEW-007 |
| `zero_enumeration_is_not_absence` | `R-1\TFMT-3:0;` with baseline TIMEIN and the cited enum rule | Explicit enum 0 with raw provenance; no truthiness-based absence/default, no whole-record coherence claim | L3-VIEW-006, L3-VIEW-007 |
| `reviewed_no_default_yields_missing` | G\PN absent, binding reviewed as no default in Table 9-2 p. 9-10 | Missing with actual absence/no-default reason, no invented name or source location | L3-REG-004, L3-VIEW-009 |
| `unknown_default_condition_is_not_false` | Supported absent TFMT; default condition evaluates true, false or cannot-evaluate in separate stimuli | True with identified basis allows Defaulted; false and unknown give Missing with different reasons; preserve the unknown condition's cannot-evaluate finding, never call it false or use it to invent A | L3-VIEW-008, L3-VIEW-009 |
| `unestablished_default_basis_is_not_invented` | Supported absent field/default binding but no identified edition basis for applying it | Missing with the missing/indeterminate default-basis reason under RELEASE-0.1.md; no forged edition or default | L3-REG-004, L3-VIEW-009 |
| `unsupported_basis_does_not_borrow_default` | Requested edition has no supported reviewed field interpretation | Unavailable/cannot-evaluate read, raw access retained; do not borrow baseline A or claim data Invalid/field Missing merely from missing knowledge | L3-REG-004, L3-VIEW-011 |
| `unavailable_definition_is_not_data_invalidity` | Raw candidates present or absent, but no reviewed requested field binding | Unavailable reason with raw evidence, not a fabricated five-state answer or assertion that unknown data is malformed | L3-VIEW-011 |
| `unknown_validity_operand_blocks_explicit` | One type-parseable candidate whose reviewed range needs an unavailable operand, with no demonstrated failure | Cannot-evaluate read with operand reason and raw candidate, not guessed Explicit or Invalid | L3-VIEW-006, L3-VIEW-011 |
| `known_multiplicity_survives_unavailable_value_rule` | Reviewed scalar cardinality, two candidates, value-rule dependency unavailable | Ambiguous with both candidates under INT-039; no default or valid-only selection, no erasure of proved multiplicity | L3-VIEW-005, L3-VIEW-011 |
| `known_failure_survives_other_unavailable_rule` | One high-byte value already Invalid under INT-037, plus another unavailable local constraint | Keep the proved Invalid raw failure and dependency reason; do not erase it as mere unknown knowledge | L3-VIEW-007, L3-VIEW-011 |
| `fallback_basis_stays_labelled` | Applicable baseline default under a supplied identified fallback edition basis | Defaulted carries fallback and its reason/citation; neither G\106 nor RCCVER is rewritten or conflated | L3-REG-004, L3-VIEW-005, L3-VIEW-008 |
| `defaults_do_not_satisfy_physical_presence` | Baseline TIMEIN, absent TFMT, applicable R-status rule and verified A default | Defaulted A remains distinguishable from physically present; default does not suppress the separate missing-required-field obligation | L3-VIEW-010 |
| `network_time_inconsistency_does_not_rewrite_default` | Baseline TIMEIN, explicit TTF=2, TFMT absent with verified default applicability | Defaulted A plus the approved inconsistency finding/evidence boundary; no substitute format, clock association or packet override | L3-VIEW-008, L3-VIEW-010 |
| `default_scope_is_not_a_neighbour` | TIMEIN/default condition established only for another occurrence/channel | Do not borrow that condition to default this field; return this scope's actual Missing/unavailable reason, with no guessed parent | L3-REG-003, L3-REG-004, L3-VIEW-009 |
| `read_availability_is_not_marker_absence` | INT-037 leading marker blocks interpreted lookup | Retain marker block/raw bytes; never turn blocked lookup into zero candidates followed by Missing or Defaulted | L3-VIEW-011 |

Planned generated checks vary zero/one/many candidates, raw spelling and
zero-length projections with independent expected states and byte locations.
State reporting must not mutate source or absorb separate findings. Required
fields, cross-field consistency, general Allowed/Required conditions and
packet checks still need their own reviewed implementations; these scenarios
do not claim those validators exist or complete the parent requirements.

## Family 2 batch 8: condition-scope acceptance specifications

**Status: planned, not executable tests.** INT-042 Option A was approved by
Joey, 2026-11-23, under ADR-0033, with independent AI recheck concurrence on
PR #28 head `d69d275` (see the [sign-off record](research/2026-11-23-int042-owner-signoff.md)).
Full development analysis and executable tests remain outstanding. These cases
do not claim a complete registry, evaluator, presence validator or time-view API.
Fixtures supply reviewed predicate/scope/type bindings and edition basis
unless their absence is the stimulus. Short snippets are not full conforming
files. Escapes denote raw bytes; synthetic conditions/link results test the
contract without claiming new standard fields or association evidence.

Sources: 106-24R1 Chapter 9 section 9.5.1 a, page 9-6; Table 9-3 TRANSMIT
ANTENNA TYPE T-x\AN1, page 9-18; Table 9-4 TIME FORMAT R-x\TFMT-n, page
9-72; CHANNEL DATA TYPE R-x\CDT-n, page 9-50; approved INT-035/036/037/038/039/040/041
and ADR-0022/0023/0033. See the
[draft review and options](research/2026-11-23-condition-scope-draft.md).
The prefix/damaged-name cases below record clarifications requested with sign-off;
this PR does not claim those new fixture wordings have already been independently reviewed.

| Planned check | Fixture or stimulus | Expected observation | Requirement |
| --- | --- | --- | --- |
| `condition_metadata_retains_scope_and_source` | Cited T-1\AN1 presence rule and separately reviewed same-channel TFMT/CDT binding | Keep source text, edition/row, review identity, full bound scope and link/default policy; do not compile from citation alone | L3-REG-005, L3-REG-006 |
| `unreviewed_reference_is_not_a_wildcard` | A source abbreviation without an available reviewed operand mapping | Non-executable/cannot-evaluate with reference reason, never select occurrence 1 or search all occurrences | L3-REG-005, L3-REG-006, L3-VAL-007 |
| `specified_absent_is_false` | Established T-1 scope, reviewed T-1\AN1 predicate, no T-1\ID or compatible related damaged evidence | Physical-presence predicate false with known absence evidence and no fabricated value span; this is not an equality comparison with a Missing value | L3-VAL-008, L3-VAL-011 |
| `specified_empty_stays_true` | `T-1\ID:;` under that physical-presence binding | True presence with the actual zero-length candidate retained; its local value usability is a separate check | L3-REG-006, L3-VAL-008 |
| `specified_invalid_and_duplicate_stays_true` | `T-1\ID:\xFF;` and, separately, `T-1\ID:a;T-1\ID:b;` with reviewed scalar binding | True physical presence; retain Invalid high-byte evidence or Ambiguous scalar candidates plus duplicate finding respectively; never select a value | L3-VAL-008, L3-VAL-011 |
| `other_occurrence_does_not_supply_presence` | Established T-1 scope with only `T-2\ID:a;` | T-1 presence false; no borrowing from T-2 or document-wide fallback | L3-REG-005, L3-VAL-007, L3-VAL-008 |
| `presence_scope_unknown_is_not_absence` | Supplied linked-scope predicate with unresolved target, then ambiguous target candidates | Cannot-evaluate for each; keep target evidence/reason, never false absence or first/any-target selection | L3-VAL-007, L3-VAL-010 |
| `encoding_blocked_lookup_does_not_prove_absence` | Assembled text starts with UTF-8 BOM; requested physical-presence predicate | Cannot-evaluate under INT-037, raw bytes still accessible; blocked semantic lookup is not a known empty set | L3-REG-006, L3-VAL-007 |
| `unterminated_text_is_not_specified` | Established T-1 scope; only unterminated suffix `T-1\ID:a` | No explicit lookup item from suffix under unchanged INT-035; presence cannot-evaluate with suffix location/framing finding retained, never false or true; dependent T-1\AN1 is not judged not allowed from this condition | L3-REG-006, L3-VAL-008, L3-VAL-011 |
| `damaged_name_blocks_false_presence` | Established T-1 scope, only `T-1\ID\xFF:a;` | Zero exact matches but related damaged-name evidence; cannot-evaluate with raw name/location and failure retained, never repair to ID or judge T-1\AN1 not allowed | L3-REG-006, L3-VAL-008, L3-VAL-010 |
| `unterminated_literal_prefix_blocks_compatible_presence` | Established T-1 ID predicate, zero exact matches; `COMMENT:x;T-1\I` and `COMMENT:x;T-1\` at EOF in separate stimuli | Cannot-evaluate for predicates whose names each suffix could complete to, with compatible readable parents; T-1 ID cannot be declared absent or T-1\AN1 not allowed. No completed name/lookup item or enumeration of completions; incompatible T-2 parent remains excluded | L3-REG-006, L3-VAL-008, L3-VAL-010 |
| `high_byte_inside_literal_blocks_false_presence` | Established T-1 ID predicate; `T-1\I\xFFD:a;` and trailing-damage `T-1\ID\xFF:a;` separately | Both cannot-evaluate with located raw damage, neither exact presence nor false absence; high-byte position is not limited to the end, and no byte is removed to manufacture ID | L3-REG-006, L3-VAL-008, L3-VAL-010 |
| `projected_controls_preserve_exact_presence` | `T-1\I\r\nD:x;`, established T-1 ID predicate | True exact presence from INT-036 projection with raw CR/LF retained and internal-omission information; controls are not high-byte damage and do not block an established exact match | L3-REG-006, L3-VAL-008, L3-VAL-011 |
| `high_byte_run_covers_missing_literal_tail` | Established T-1 ID predicate; `T-1\I\xFF:a;` with zero exact matches | Cannot-evaluate: I followed by a high-byte run is compatible with ID because the run may stand for D; do not require literal D to be readable, repair the key or report false | L3-VAL-008, L3-VAL-012 |
| `high_byte_run_can_match_empty` | Same predicate with `T-1\I\xFF\xFED:a;` and, separately, `T-1\ID\xFF:a;` | Each cannot-evaluate with raw damage retained: the adjacent high bytes form one maximal run that can stand for zero unknown bytes; run length is not the number of inferred characters | L3-VAL-012 |
| `damaged_literal_readable_order_is_required` | T-1 ID predicate, only `T-1\D\xFFI:a;`, no other related evidence | False in this known scope: readable D before I cannot align to literal ID in order; the run cannot reorder/delete either readable character. Retain the independent damaged-name finding | L3-VAL-012 |
| `terminated_literal_has_no_implicit_gap` | T-1 ID predicate, only `T-1\I:a;` or `T-1\ID2:a;` in separate stimuli | False absent other related evidence: whole terminated literal does not match ID and no high-byte run supplies a gap. Contrast unterminated T-1\I at EOF, which blocks false under the approved prefix rule | L3-VAL-012 |
| `damaged_literal_matching_does_not_enumerate_replacements` | Supplied long diagnostic literal containing many alternating readable characters/high-byte runs, with compatible and incompatible reviewed target literals | Same whole-literal existential answer regardless of a run's possible expansion count; bounded matching states follow supplied lengths, no replacement enumeration/exponential backtracking or manufactured exact key | L3-VAL-012 |
| `high_byte_run_hiding_separator_is_unassigned` | T-1 ID presence predicate, zero exact matches; `T-1\xFFD:a;`, where the run could cover `\I` | Cannot-evaluate, not false: component split is UNKNOWN; retain wrong-arity-style unassigned evidence, compatible with every scope readable parts allow under INT-040. Never guess a component split or parent alignment to exclude a compatible scope; no new key or parent/split enumeration | L3-VAL-008, L3-VAL-012 |
| `exact_match_proves_presence_despite_related_damage` | `T-1\ID:x;T-1\ID\xFF:a;`, requested T-1 ID presence | True from the exact match, keeping damaged-name evidence/finding; neither damage nor an invalid value may erase established physical presence | L3-VAL-008, L3-VAL-011 |
| `presence_uses_canonical_name_identity` | `t-1\id:x;` and, separately, `T-01\ID:x;`, requested T-1 ID presence | Each matches: true through INT-036/038 projection, ASCII case folding and canonical index 1; raw spelling remains, and leading-zero case retains its normalization information finding (case folding alone needs no new finding) | L3-REG-006, L3-VAL-008, L3-VAL-011 |
| `damaged_child_parent_blocks_compatible_scopes` | Synthetic reviewed physical-presence predicate for D-1\WP-1-1-m-2, supplied fixed m scopes, no exact matches; `D-1\WP-1-1-x-2:4;` | Every scope (1,1,1,m) compatible with readable parents gives cannot-evaluate, never false; apply INT-040 without choosing x, guessing arity alignment or enumerating hypothetical parents; this tests diagnostic evidence, not a new Chapter 9 presence rule | L3-VAL-007, L3-VAL-008, L3-VAL-010 |
| `incompatible_damage_does_not_block_presence_absence` | Same supplied D-1 predicate/scope, only `D-2\WP-1-1-x-2:4;`; separately T-1 ID predicate with only `T-1\ID2:a;` | D parent 2 contradicts 1; printable ID2 is a different literal, not ID damage: false in each established scope absent other compatible evidence, independent findings retained | L3-VAL-008, L3-VAL-011 |
| `not_specified_preserves_unknown` | Reviewed negated-presence predicate; positive predicate true, false or cannot-evaluate (related suffix/damaged name) in separate stimuli | Negated results false, true, cannot-evaluate respectively; no unknown-to-true conversion or not-allowed claim based on unknown | L3-VAL-008, L3-VAL-010 |
| `other_document_does_not_supply_presence` | Current document has no T-1 ID or related damaged evidence; exact ID or related damaged name exists only in another document or separate setup record | Current presence false; neither actual item nor damage from another record contributes. Conversely fragments of ONE record (same channel, sequence +1, matching FRMT/RCCVER under ADR-0025/0031) share document scope; do not split at packet boundaries | L3-VAL-007, L3-VAL-008, L3-VAL-011 |
| `same_channel_value_not_other_channel` | `R-1\CDT-2:TIMEIN;`, evaluate TFMT-3 condition with reviewed same-channel binding | CDT-3 Missing gives cannot-evaluate, never true from channel 2 or false from missing comparison | L3-REG-005, L3-VAL-007, L3-VAL-009 |
| `value_comparison_true_false_unknown` | Same-channel CDT-3 is TIMEIN, PCMIN, absent, high-byte Invalid or repeated TIMEIN in separate fixtures | Equality to TIMEIN is true, false, cannot-evaluate, cannot-evaluate, cannot-evaluate respectively; duplicates retain both candidates, no positive default activation from uncertainty | L3-VAL-009, L3-VAL-010 |
| `missing_value_does_not_make_inequality_true` | Supplied reviewed inequality predicate whose required value is Missing, Invalid or Ambiguous | Each cannot-evaluate, never true because a usable equal value could not be found | L3-VAL-009 |
| `default_visibility_is_declared` | Synthetic reviewed value predicate with same usable Defaulted operand; policy explicit-only versus defaults-eligible | Explicit-only cannot-evaluate with policy reason; eligible policy compares usable value with default citation and labelled basis retained | L3-REG-005, L3-VAL-009 |
| `default_does_not_make_item_specified` | Synthetic reviewed presence predicate; no actual item but an available cited default | False physical presence, no insertion or source span for default; default visibility cannot rewrite physical presence | L3-REG-006, L3-VAL-008, L3-VAL-011 |
| `blank_rule_unavailable_is_not_false` | Value predicate with `R-1\TFMT-3: A;`, blank-handling unavailable | Cannot-evaluate, raw blank retained under INT-041; no trim or automatic mismatch/Invalid claim | L3-VAL-009, L3-VAL-010 |
| `condition_findings_name_invoking_pass` | Same unavailable dependency invoked from each of ADR-0023's four passes | Separate condition finding identifies actual invoking pass, rule/citation/scope, blocking reason, candidates/locations and edition basis; no fifth pass or fabricated absent-item span | L3-VAL-010 |
| `condition_results_keep_evidence_without_edits` | True, false and cannot-evaluate stimuli under labelled fallback edition, with earlier findings | Each retains operand provenance and fallback label; byte-for-byte emission, order and prior findings unchanged, no field deletion/default insertion or whole-document success claim | L3-VAL-010, L3-VAL-011 |

Future executable tests must assert these outcomes from independently supplied
fixtures and reviewed bindings, not regenerate expected answers through the
evaluator under test. Complete compound expressions, dependency cycles,
severity/deduplication, link construction and presence/profile enumeration
remain later requirements; these cases do not silently settle them.

### Planned F1 repair and uncertainty edges

Review follow-up to INT-021/F1; these are planned, not executable tests and
not evidence of completed checksum requirements. F1 was approved by Joey,
2026-11-23, under ADR-0033, with AI CONCUR at #30 head `13173b0` (see the
[owner approval record](research/2026-11-23-int021-int030-owner-signoff.md)). Sources: 106-24R1 Chapter 9 Table 9-2 MESSAGE DIGEST/CHECKSUM,
page 9-13; L3-VAL-012's reviewed diagnostic rule; ADR-0029's exact-target
transaction and final-byte stamp. INT-018/019/020 were reconfirmed by Joey,
2026-11-23, with independent AI CONCUR at main `30e42a3`; analysis/tests remain.

| Planned check | Fixture or stimulus | Expected observation |
| --- | --- | --- |
| `damaged_sha_name_is_indeterminate` | `G\SH\xFFA:2-<64 hex digits>;`, separately alone and beside one exact G\SHA item; placeholder expanded to actual hex bytes in future fixture | Verification indeterminate with all raw evidence; never drop high bytes to create a key or claim the exact item proves a unique SHA item |
| `exact_duplicate_removal_precedes_stamping` | Two G\SHA items; caller explicitly removes one using its exact current-revision item ID, then requests a final stamp | Repair permitted under ADR-0029 without guessed selection; rebuild/re-read the result; stamp only after ambiguity is resolved, as a possible final transaction step; still refuse if duplicates/damaged SHA evidence remain |

F7's first-record SRCC, fragment-disagreement, nearby-event candidate and
prior-ordering questions are analysis notes in INT-030 and the gate note,
not newly chosen event-identification or fragment-summary semantics.

## Family 2 batch 9: registry-gate acceptance specifications

**Status: planned, not executable tests.** These derive existing ADR-0004/
0005/0022/0027/0033 decisions and the interpretation register's development
gates; they propose no new Chapter 9 interpretation. The synthetic source,
review, analysis and test artifacts below are isolated gate fixtures, not
claims of current production registry coverage or completed development work.

Source structure: 106-24R1 Chapter 9 section 9.5.1 a, pages 9-5–9-6,
usage attributes and definition prose; Table 9-4 TIME FORMAT, page 9-72,
is a cited row example already used by batch 7. Mutation stimuli below are
hypothetical source revisions, not assertions of real standard changes.
See the [derivation record](research/2026-11-23-registry-gates-draft.md).

| Planned check | Fixture or stimulus | Expected observation | Requirement |
| --- | --- | --- | --- |
| `registry_citation_identifies_edition_and_row` | TIME FORMAT source fixture versus a citation containing only Table 9-4 without edition/row | Full edition/table/row/code-pattern identity retained; incomplete citation blocks executable readiness, no similarly named edition borrowed | L3-REG-007 |
| `registry_source_keeps_prose_dependencies` | Source row plus separately cited supporting prose; mutate only the prose text in a fixture | Both sources retained verbatim with dependency links; prose change is visible to freshness even if the usage fields are unchanged | L3-REG-007, L3-REG-010 |
| `registry_coverage_accounts_for_each_input_row` | Three supplied source rows classified executable, not yet interpreted and excluded with a reason; then one extra unaccounted row | Three accounted statuses remain distinct; added unaccounted row causes coverage failure, not silent omission or inferred semantics | L3-REG-008 |
| `registry_exclusion_requires_reason` | Source row excluded without a reason, then with a reason and source identity | Missing reason fails coverage check; identified reason accounts for exclusion without claiming executable support | L3-REG-008 |
| `registry_review_records_human_and_ai_roles` | Complete revision-bound fixture metadata, then owner sign-off missing, AI evidence missing or pending, or only a coordinator decision in separate fixtures | Complete roles/dates/references retained; each missing prerequisite blocks readiness, no coordinator or AI substitutes for owner approval | L3-REG-009, L3-REG-012 |
| `coordinator_reader_policy_needs_owner_and_ai_review` | Planned reader/validator behaviour tied to INT-021 (F1) or INT-030 (F7), with only the coordinator decision recorded, whether hand-written or generated | Implementation readiness blocked pending ADR-0033 owner sign-off AND independent AI check for each entry. This is a synthetic missing-review fixture, not current F1/F7 status: both now have owner approval and AI concurrence recorded. Prior checks on other interpretations or requirement wording alone never approve the fixture | L3-REG-009, L3-REG-012 |
| `legacy_owner_acceptance_still_needs_ai_review` | Synthetic pre-ADR-0033 owner-accepted entry with Review: pending, then a separate supplied complete-review fixture | Preserve historical owner acceptance but block implementation until independent AI check is recorded; do not grandfather pre-ADR-0033 entries or reuse unrelated AI checks; actual INT-018/019/020 now have their own reconfirmation and AI concurrence recorded | L3-REG-009, L3-REG-012 |
| `registry_ai_reference_is_revision_bound` | Owner-relayed AI CONCURS with PR and exact head versus only a moving branch URL | Immutable review reference accepted as evidence without fabricating a separately posted review; moving branch alone does not prove which interpretation/source revision was checked | L3-REG-009 |
| `registry_source_change_requires_rereview` | Reviewed source hash then changed source prose, source dependency edition or unavailable stored hash in separate stimuli | Every affected interpretation flagged stale/re-review; recomputing a digest alone does not renew approval, old sign-off remains historical | L3-REG-010 |
| `registry_digest_cannot_hide_source_changes` | Dependency strings whose naive concatenations coincide, or source differing only in case/layout | Documented encoding/boundaries distinguish hash input; no normalization that hides a source change; this fixture selects no digest algorithm | L3-REG-010 |
| `registry_unlisted_interpretation_never_executes` | Rule dependency names a nonexistent INT, then open/proposed/unready entries, plus a source-only entry | Executable rule blocked with precise dependency reason; source-only entry stays accounted and non-executable, no inline fallback interpretation | L3-REG-011, L3-REG-013 |
| `registry_all_interpretation_dependencies_are_checked` | Synthetic rule with two dependencies, only one ready | Block with the other dependency and source/review evidence listed; one approved entry does not authorize the whole rule | L3-REG-011, L3-REG-012 |
| `registry_approval_is_not_completed_development` | Approved/current fixture with draft analysis and named-but-unwritten focused/edge tests; then complete evidence supplied in an isolated fixture | First remains unready with every missing prerequisite; second can satisfy the evidence gate without claiming real registry completion. Drafts/planned cases never counted as analysis/test evidence | L3-REG-012 |
| `registry_test_linkage_requires_real_markers` | Register names a focused test but source has no matching executable test/interpretation marker | Report missing verification linkage, never infer passing coverage from a Markdown test name | L3-REG-011, L3-REG-012 |
| `registry_generation_refuses_unready_executable_rule` | Generator/check input asks to activate a rule whose source/review/analysis/test gate fails | Refuse activation with entry/dependency/source reason; source remains visibly non-executable, no silent fallback to a different rule, no source-text rewrite or downstream build-script generation | L3-REG-013 |

Future tests must supply independent source and review/evidence fixtures,
including each missing gate separately. A successful synthetic gate check
proves only that supplied readiness evidence is accounted for; it cannot
certify the standard's interpretation or production registry completeness.
No current build script, CI job, generator or parser test is changed here.

## Family 2 batch 10: checksum follow-up acceptance specifications

**Status: planned, not executable tests.** INT-018/019/020 were reconfirmed by
Joey, 2026-11-23, with independent AI CONCUR in the register review at main
30e42a3; INT-021/F1 and INT-030/F7 approvals remain recorded. Analysis and
tests are not complete. INT-043 Option B is approved (Joey, 2026-11-23; AI
CONCURS on #32 at 8941ca1). Cases remain planned, not executed behavior.

Sources: 106-24R1 Chapter 9 section 9.4.2 and Table 9-2 MESSAGE DIGEST/
CHECKSUM, page 9-13; Chapter 6 section 6.2.3.11 f, pages 6-22–6-23;
INT-035/036/037/038, reconfirmed INT-018/019/020 and approved F1. Snippets
are focused fixtures, not complete conforming TMATS files. Escapes mean raw
bytes; h denotes an independently generated 64-digit digest for the exact
raw preimage. Use Python hashlib, never production checksum code or Chapter
6's printed example digest as the expected oracle; its bytes/digest have not
been reproduced. Exact preimages/digests are in the
[derivation note](research/2026-11-23-checksum-followups-draft.md).

| Planned check | Fixture or stimulus | Expected observation | Requirement |
| --- | --- | --- | --- |
| `projected_sha_name_excludes_raw_span` | `\r\n g\s\r\nha:2-h;\nG\PN:TEST;`, h from the note's first preimage | Recognize projected G\SHA; raw spelling finding; exclude raw g through semicolon, including internal CR/LF; leading CR/LF/space and following newline are hashed, not trimmed | L3-SUM-001 |
| `missing_previous_semicolon_does_not_create_sha` | `COMMENT:x\r\nG\SHA:2-h;`, preceding terminator intentionally omitted | One COMMENT value under INT-035; no SHA exclusion inside it; located suspect-terminator/framing finding is suggestion-only, no repair or invented item | L3-SUM-001 |
| `x_and_comment_sha_text_is_not_item` | X carried path with G/SHA, and COMMENT text containing G\SHA in separate fixtures | Neither is the complete projected checksum code name; raw bytes hashed unchanged, no substring exclusion | L3-SUM-001 |
| `leading_bom_blocks_sha_verification` | UTF-8 BOM at assembled-text offset zero before an otherwise valid checksum fixture | Indeterminate semantic exclusion under INT-037; raw access retained, no BOM removal or positive verification | L3-SUM-003 |
| `eof_sha_refuses_authoritative_compute` | `G\SHA:2-` plus 64 digits at EOF without semicolon | Malformed verification; authoritative COMPUTE refused, no guessed end/exclusion, suggested terminator remains unapplied | L3-SUM-002, L3-SUM-003 |
| `mid_document_missing_terminator_fails_value_shape` | `G\SHA:2-h\r\nG\PN:TEST;` | The sole framed value includes G\PN:TEST and fails digest shape; swallowed-attribute evidence leaves the intended exclusion ambiguous, so authoritative COMPUTE is refused. Never split/repair or treat the later semicolon as proof of the intended boundary | L3-SUM-002, L3-SUM-003 |
| `complete_bad_digest_value_allows_compute` | Complete known input `G\SHA:2-ABC;G\PN:TEST;` and separately G\SHA:2- plus exactly 63 hex digits followed by `;G\PN:TEST;`; unique name, no damage/BOM/swallowed-item evidence | VERIFY malformed for value shape/length; authoritative COMPUTE ALLOWED using the same defined raw exclusion and the placeholder oracle preimage G\PN:TEST;. Do not infer truncation or missing delimiters from length alone | L3-SUM-002, L3-SUM-005 |
| `defined_exclusion_with_unusable_value_still_computes` | Unique terminated SHA item in complete known input with unsupported designator, high-byte Invalid value or blank-rule-unavailable value; no exclusion ambiguity | Stored-value verification status remains separate; authoritative COMPUTE allowed. Only undefined/ambiguous domain blocks it, not inability to compare the stored digest | L3-SUM-002, L3-SUM-004 |
| `truncated_setup_is_not_authoritative_checksum` | Known truncated setup record, with SHA terminating before the truncation | Malformed checksum verification and authoritative COMPUTE refusal; complete SHA syntax cannot make the record complete | L3-SUM-002, L3-SUM-003 |
| `malformed_sha_keeps_duplicate_finding` | Two complete SHA items plus a third unterminated SHA suffix | Retain the two framed duplicates/F1 evidence and the additional malformed/framing finding together; no positive result or authoritative checksum, no fabricated third lookup item | L3-SUM-002, L3-SUM-003 |
| `checksum_value_controls_keep_omission_information` | Independently correct h with CR/LF inserted between its hex digits, also with upper-case letters | Projected digits match; located omission information and applicable upper-case warning retained; emission unchanged | L3-SUM-004 |
| `checksum_value_blanks_are_unavailable` | Independently correct h with leading/embedded/trailing ASCII blanks in separate values | Unavailable until blank rule review; no trim, automatic mismatch or automatic Invalid verdict | L3-SUM-004 |
| `sha_zero_placeholder_is_unverified` | `G\SHA:0;G\PN:TEST;` | Unverified with placeholder/no-digest reason, never match/mismatch; defined exclusion permits COMPUTE under the general L3-SUM-002 rule (also true for complete malformed digest values), using the note's second preimage; explicit stamp may replace and must verify final output | L3-SUM-005 |
| `checksum_fixture_uses_independent_digest` | The note's exact byte preimages and hashlib digests | Expected digest comes from independent raw-byte oracle; do not derive expected output through the implementation under test or copy the printed Chapter 6 example | L3-SUM-001, L3-SUM-004 |
| `zero_padded_sha_algorithm_requires_decision` | Complete unique `G\SHA:2-h;`, `G\SHA:02-h;`, `G\SHA:002-h;` with h independently correct | Under owner-decided Option B, all identify algorithm 2 and may verify positively. Canonical 2- has no designator warning; 02/002 always have located warning NONCANONICAL-DESIGNATOR citing Chapter 6 section 6.2.3.11 f p. 6-22 and Chapter 9 Table 9-2 p. 9-13. Raw bytes unchanged. Semantic approval is recorded; analysis/executable tests remain pending | L3-SUM-004 |
| `sha_algorithm_identity_uses_canonical_string` | 40-digit token of 39 ASCII zeroes followed by 2, plus tokens 2, 02, 002, 000 | Strip leading zeros to canonical digit string, keep 0 for all-zero input; compare strings to 2, no fixed-width/big-integer conversion. First four identify algorithm 2; 000 is unsupported. No input rewriting | L3-SUM-004 |
| `noncanonical_sha_designator_always_warns` | 02/002 with matching digest, mismatching digest and upper-case matching digest in separate otherwise usable inputs | NONCANONICAL-DESIGNATOR warning always emitted for accepted noncanonical token, independent of match outcome; upper-case matching hex also retains INT-020 warning. Neither warning suppresses the other | L3-SUM-004 |
| `noncanonical_designator_warns_when_verification_blocked` | Located SHA candidates with readable 02/002 tokens: two duplicates; compatible name damage; leading BOM; unterminated/undefined exclusion in separate supplied raw-evidence fixtures | Every located noncanonical-2 token gets its own located NONCANONICAL-DESIGNATOR warning even though verification remains blocked. Retain all duplicate/damage/BOM/framing findings; no guessed key/exclusion or positive verdict. Diagnostic candidates remain distinct from explicit lookup items; never scan COMMENT/X value text as SHA | L3-SUM-003, L3-SUM-004 |
| `other_numeric_sha_designators_stay_unsupported` | 0-, 00-, 3-, 20-, 0020- with otherwise in-range hex content | Unknown/unsupported algorithm, never coerced to 2 or positively verified; distinguish 0- plus hex from the no-hyphen G\SHA:0; placeholder. Defined exclusion still allows COMPUTE | L3-SUM-002, L3-SUM-004 |
| `sha_designator_edge_classification` | Values ` 2-h`, `2–h` (UTF-8 en dash), `+2-h`, `2.0-h`, and `2-H` with independent correct upper-case hex in separate complete terminated items | Leading blank unavailable under blank rule; en dash high bytes Invalid per INT-037; +2/2.0 malformed; valid 2- with upper-case hex follows INT-020 (match with warning). Raw bytes unchanged; defined exclusion permits COMPUTE for all | L3-SUM-002, L3-SUM-004 |
| `sha_stamp_uses_canonical_designator` | Explicit stamp of eligible repaired/placeholder input, with final output re-read | Always writes `2-` plus lower-case hex, never `02-`/`002-`/`0-`; no stamp on ambiguous input and no positive verification of an unsupported original token | L3-SUM-005 |

## Family 2 batch 11: minimal stamp-transaction acceptance specifications

**Status: planned, not executable tests.** Derives ADR-0029, approved F1/
INT-021 and INT-043, and L1-SUM-004's already-approved 0.1 minimal transaction.
Sources: 106-24R1 Chapter 9 Table 9-2 MESSAGE DIGEST/CHECKSUM, page 9-13;
Chapter 6 section 6.2.3.11 f page 6-22. Supplied insertion/style bindings are
explicit fixture prerequisites, not a new fallback layout policy. Short
records do not assert full TMATS conformance. See the
[derivation/oracle note](research/2026-11-23-stamp-transaction-draft.md).

| Planned check | Fixture or stimulus | Expected observation | Requirement |
| --- | --- | --- | --- |
| `stamp_rejects_stale_ambiguous_or_overlapping_targets` | Stale/cross-document item ID, bare duplicate name, overlapping explicit ranges or two explicit edits to one item in separate stimuli | Reject whole explicit edit set with implicated targets/locations; no guessed retarget, partial edit or original mutation | L3-WRT-002 |
| `stamp_rebuilds_post_edit_revision` | Valid explicit change before stamping, with deliberately stale prior index/offset evidence | New revision/state and reread match actual edited bytes; no old offsets/reports used to hash or choose a SHA slot; original bytes/revision remain available | L3-WRT-003, L3-SUM-008 |
| `stamp_exact_duplicate_repair_is_allowed` | Two SHA items, caller removes exactly one current-revision item and requests final stamp | Permit explicit repair first; check the rebuilt result, stamp only once ambiguity is fixed; no first/last survivor chosen by stamp | L3-WRT-002, L3-WRT-003, L3-SUM-006 |
| `stamp_remaining_domain_ambiguity_refuses` | Post-repair revision still has duplicate/compatible damaged SHA evidence, blocking BOM, undefined exclusion or known truncation in separate fixtures | Refuse successful stamp with all evidence retained; exact-item presence alone never clears uncertainty | L3-SUM-006, L3-SUM-010 |
| `stamp_needs_established_slot_and_style` | Explicit insertion target and known CR/LF style, then unavailable/mixed style prerequisites in a separate fixture | First uses supplied document-style bytes; unavailable style/target reported, no hardcoded LF or implicit insertion-position fallback | L3-SUM-007 |
| `stamp_hashes_newlines_outside_inserted_item` | Note's exact bytes with CR/LF before and after inserted SHA item, independent expected digest | Both CR/LF sequences outside exclusion are hashed, so pre-insertion or projected-source hash cannot pass; final value canonical 2- plus lower-case hex | L3-SUM-007, L3-SUM-008, L3-SUM-009 |
| `stamp_accepted_noncanonical_input_emits_canonical_output` | Usable input 02/002 algorithm token, explicit stamp requested | Output writes canonical 2-; input NONCANONICAL-DESIGNATOR warning retained with input revision/location; no invented noncanonical warning for canonical new value | L3-SUM-008, L3-SUM-010 |
| `stamp_noncanonical_name_finding_stays_prominent` | Otherwise eligible slots `g\sha:2-h;` and `G\S\r\nHA:2-h;`, explicit value-only stamp in separate fixtures | Keep raw name spelling and INT-018's located noncanonical-name finding prominent in the stamp result; recommend an explicit rename edit first. No silent rename or new refusal solely for name spelling | L3-SUM-008, L3-SUM-010 |
| `stamp_never_writes_flex_into_sha` | Separately requested irig106.org flex compatibility value alongside an explicit SHA stamp | Flex remains separately labelled non-standard under L1-SUM-003; G\SHA contains only the SHA-256 2- digest, never the flex signature or an unlabelled substitution | L1-SUM-003, L3-SUM-008 |
| `stamp_final_verification_failure_is_atomic` | Valid staged edits with injected final reread/verification failure; separate target/emission/reconstruction failure stimuli | Failure, no successful verified-stamp result, original input unchanged; no assertion that external writer side effects are rolled back | L3-WRT-003, L3-SUM-009 |
| `stamp_refusal_keeps_per_occurrence_designator_warnings` | Located noncanonical-2 tokens across F1/damage/BOM/undefined-domain refusal fixtures | Warning for every located occurrence and all blockers retained; no positive integrity, guessed exclusion or coalescing of different source locations | L3-SUM-006, L3-SUM-010 |
| `stamp_preserves_unedited_source_and_findings` | Unknown attributes, raw case/layout and independent findings outside caller edits/SHA value | Bytes/order/spelling unchanged there; findings retain proper revision provenance; a verified stamp claims checksum integrity only | L3-WRT-003, L3-SUM-010 |

No fixture settles X deletion/renumbering (F2), automatic repairs (F3),
style fallback, general public edit API or file-publication behavior. Future
tests need independent expected bytes/digests and injected failures, not
expected answers derived through the stamping implementation itself.

## Family 2 batch 12: Format 1 payload/CSDW acceptance specifications

**Status: planned, not executable tests.** These use an explicitly identified
106-24R1 layout, not a guessed historical era. Sources: Chapter 11 section
11.1 page 11-1 (wire byte orientation), section 11.2.7.2 a/Figure 11-34 page
11-50 and RCCVER table page 11-51; ADR-0009/0015/0025/0028/0032. Payloads
start at the CSDW and are already bounded by the caller; snippets are not
whole packets or complete conforming records. Body/candidate completeness
is an independent later result. INT-012 is reconfirmed by Joey, 2026-11-23,
with AI concurrence at #34 head e352323; analysis/tests remain outstanding.
See the [source/review note](research/2026-11-23-setup-payload-draft.md).

| Planned check | Fixture or stimulus | Expected observation | Requirement |
| --- | --- | --- | --- |
| `short_csdw_has_no_fabricated_fields` | Payload lengths 0, 1, 2 and 3, known baseline layout/provenance | Located short-CSDW result retaining all available bytes; no padded word, host read past end or default flag/version | L3-CH10-001 |
| `payload_body_starts_after_four_bytes` | `0E 00 00 00` followed by raw `G\PN:x;\r\n\x00\x1A\xFF` | CSDW [0,4), body [4,len); body bytes unchanged, including high byte/padding. No trim, interpretation of padding as packet filler or completeness claim | L3-CH10-001, L3-CH10-002 |
| `csdw_fields_use_little_endian_wire_bytes` | `0E 00 00 00`, `0E 01 00 00`, `0E 02 00 00`, `0E 03 00 00` separately | RCCVER 0x0E in all; SRCC/FRMT respectively 0/0, 1/0, 0/1, 1/1. Same on any host; independent constants, not shared-decoder-derived expected values | L3-CH10-003, L3-CH10-005 |
| `body_after_csdw_is_not_byte_swapped` | Baseline word plus mixed text/high bytes, caller supplies exact body slice | Only CSDW interpreted as little endian; body remains its native raw byte order and retains input locations | L3-CH10-002, L3-CH10-003 |
| `xml_flag_never_enters_ascii_scanner` | `0E 02 00 00` followed by XML-looking text, ASCII-looking text or no bytes in separate fixtures | XML unsupported in each; full raw body retained, no code-name parsing, transcoding or fake empty document | L3-CH10-004 |
| `empty_ascii_fragment_is_not_complete_record` | Exactly `0E 00 00 00` | Decodable CSDW with empty [4,4) body; no complete/valid setup assertion or inferred required TMATS values | L3-CH10-001, L3-CH10-002, L3-CH10-004 |
| `reserved_bits_are_retained_not_sanitized` | `0E 03 00 80` | Raw word 0x8000030E and high reserved bit retained; FRMT=1, SRCC=1, RCCVER=0x0E unchanged. No new severity/validity verdict from this preservation test | L3-CH10-003, L3-CH10-006 |
| `rccver_baseline_literal_codes` | Separate baseline payloads with byte 0 set to 07,08,09,0A,0B,0C,0D,0E | Literal RCC 106-07,09,11,13,15,17,19,22 respectively; raw byte retained, no literal 106-20/23/24 invented | L3-CH10-007 |
| `rccver_reserved_stays_unknown_under_named_table` | Byte 0 is 00,06,0F or FF in separate baseline payloads | Reserved/unknown under this supplied baseline table; raw retained, no fabricated edition, implicit fallback or historical-field-presence claim | L3-CH10-007, L3-CH10-008 |
| `rccver_archive_note_is_not_declaration` | Raw 0E plus reviewed archive-note fixture | Literal RCC 106-22 stays distinct from labelled unchanged-through-106-24R1 observation; no unbounded future guarantee or newest-edition declaration | L3-CH10-007 |
| `local_srcc_is_not_session_validation` | Same readable CSDW supplied in first-record versus later-record contexts; two separate fragments differ only in SRCC | Local bit preserved in each; no invented event verdict, difference/first-record judgement or SRCC consensus. Assembly/session rules remain separate | L3-CH10-005 |
| `fragment_csdws_do_not_enter_assembled_text` | Two separately supplied baseline payloads, each with its own CSDW and arbitrary body split | Each adapter returns only its body range; later assembly gets original fragments/provenance, not an inserted CSDW in the text. No concatenation or complete-record decision occurs here | L3-CH10-002 |
| `split_bom_is_not_stripped_per_payload` | Body 1 ends EF, body 2 starts BB BF after its own CSDW | Both retain bytes; no fragment-level BOM removal or encoding guess. INT-037 evaluates markers after assembly, outside this decoder | L3-CH10-002 |
| `version_code_spaces_do_not_share_mapping` | Tagged Format 1 RCCVER 0D/0E, Format 4 RCCVER 0D/0E and packet-header Data Type Version 0A under their separately cited baseline tables | Format 1 literals RCC 106-19/22; Format 4 0D/0E reserved/unknown (its 0C is RCC-106-17); header 0A is RCC 106-22, while Format 1 0A is RCC 106-13. Never use a shared untagged version-byte lookup; guard does not implement Format 4 parsing | L3-CH10-007, L3-CH10-008 |
| `literal_rccver_is_not_fallback_policy` | Format 1 0E/0D, separately supplied reviewed fallback-policy fixture | Literal remains RCC 106-22/19. Optional selected rules 106-24R1/106-20 are separately labelled INT-017 POLICY, not declarations or part of INT-012 approval; test the policy through its own binding/readiness gate | L3-CH10-007, L3-CH10-008 |
| `version_metadata_sources_remain_distinct` | Packet data-type-version metadata differs from RCCVER; supplied TMATS G\106:24; with raw RCCVER 0E | Report supplied facts separately; no automatic disagreement finding, G\106 rewrite, fallback selection or time-channel inference | L3-CH10-008 |
| `unknown_layout_knowledge_does_not_prove_bad_data` | Payload bytes present but requested layout interpretation unavailable | Raw payload and limitation retained; unavailable interpretation, not zero-padded/header-default answer or unsupported-era bytes called malformed by guess | L3-CH10-001 |

Future tests must assert all cases against independent byte fixtures and the
shared-type adapter, with real unknown-code coverage; none may compute its
expected masks or version list by calling the decoder being tested. This
slice does not verify packet checksums, filler removal or the historical
layout mapping; those are not silently claimed by a green CSDW adapter test.

## Family 2 batch 13: declaration and rules-basis acceptance specifications

**Status: planned, not executable tests.** Sources: 106-24R1 Chapter 9 Table
9-2 G\106, page 9-10; Chapter 11 Figure 11-34/Format 1 table pages 11-50–
11-51, Format 4 Figure 11-43/table page 11-62 and packet-header Data Type
Version section 11.2.1.1 e page 11-6; ADR-0028/0032. INT-012 is reconfirmed.
INT-014/015/016/017 keep pending review; fixtures explicitly supply reviewed
bindings/policy to isolate these contracts. INT-044 Option B is approved (Joey, 2026-11-23; AI CONCURS on #36 at
6f6e080). Analysis/tests remain outstanding. No full edition-coverage claim.

| Planned check | Fixture or stimulus | Expected observation | Requirement |
| --- | --- | --- | --- |
| `edition_observations_survive_selection` | One G\106 year, separate Format 1 code, packet-header version and caller override, with provenance | Preserve each raw source/identity and candidate reading separately from selected override; never rewrite declarations or share untagged code-space mapping | L3-EDN-001, L3-EDN-003 |
| `one_year_candidates_are_not_scalar_duplicates` | One G\106:24; plus supplied reviewed INT-014 same-rules binding for 106-24/24R1 | Both candidate editions retained; shared-rules choice 24R1 labelled declared basis, not a fabricated directly declared revision or Ambiguous duplicate scalar | L3-EDN-001, L3-EDN-002, L3-EDN-004 |
| `unreviewed_year_binding_is_not_missing_data` | Raw G\106 candidate but required year/blank/edition binding unavailable | Unavailable reason with original candidate, no guessed year, missing/unrecognised classification or automatic fallback from absent knowledge | L3-EDN-002, L3-EDN-007 |
| `explicit_override_preserves_declarations` | Valid override with usable conflicting year/Format 1 code; separately unusable override | First selects override-labelled basis and retains source facts; unusable override reports blocked reason, no silent fallthrough or G\106 rewrite | L3-EDN-003 |
| `declared_basis_precedes_rccver_without_conflict` | Supplied usable G\106 rule basis 106-24R1, Format 1 literal 0D = RCC 106-19, no override | Declared basis retained, no automatic disagreement finding, RCCVER literal unchanged; no inferred 106-20 fallback | L3-EDN-004 |
| `fallback_policy_is_not_literal_rccver` | No override, G\106 missing or reviewed unrecognised, Format 1 0D then 0E; supplied reviewed INT-017 policy | Literal remains RCC 106-19/22; separate fallback rule edition is 106-20/24R1 with policy reason. These outputs do not extend INT-012 approval to INT-017 | L3-EDN-005 |
| `non_format1_codes_do_not_supply_fallback` | Only Format 4 reserved 0D/0E or header Data Type Version 0A metadata, no eligible Format 1 declaration; missing G\106/no override | Do not reinterpret either code space as a Format 1 operand; reviewed baseline fallback, if available, stays labelled policy with missing-eligible-RCCVER reason | L3-EDN-005, L3-EDN-007 |
| `baseline_fallback_stays_labelled` | Missing G\106 and no usable eligible Format 1 RCCVER, supplied baseline policy | Selected 106-24R1 fallback with reason, not a declared year/revision; effective defaults/findings retain that label | L3-EDN-005, L3-EDN-007 |
| `ambiguous_g106_uses_labelled_fallback` | G\106:19;G\106:24; with usable Format 1 0E, no override | Ambiguous with both occurrences/duplicate findings, no selected candidate; separate 106-24R1 basis labelled `fallback — reason: ambiguous G\106 (2 candidates)`, never missing/unrecognised | L3-EDN-001, L3-EDN-006 |
| `identical_g106_duplicates_still_fall_back` | G\106:24;G\106:24;, no override | Still Ambiguous with two occurrences/duplicate findings; same explicit fallback reason with 2 candidates, agreement information only, no collapse or declared basis | L3-EDN-006 |
| `override_keeps_ambiguous_declaration_evidence` | Either duplicate fixture with valid explicit override | Select override-labelled basis, keep every ambiguous raw candidate/duplicate finding; no implicit source repair or conversion to one declared item | L3-EDN-003, L3-EDN-006 |
| `ambiguous_fallback_uses_existing_policy` | Two G\106 occurrences; supplied reviewed INT-017 policy, eligible Format 1 0E, 0D, lower usable codes, then no usable eligible code in separate stimuli | Same policy outputs (24R1, 20, existing lower-code readings, else baseline), unchanged literal wire readings; every basis keeps the ambiguous-G\106 reason, never a new mapping | L3-EDN-005, L3-EDN-006 |
| `ambiguous_fallback_labels_defaults_and_findings` | Duplicate declaration fixture; applicable default and a separate invalid field under supplied fallback rules | Defaulted value and validation findings both retain applied edition, fallback label and ambiguity reason; no false declared-edition validity | L3-EDN-006, L3-EDN-007 |
| `single_damaged_or_blank_g106_is_not_duplicate` | One G\106 with high byte, then one with intentional blank, under supplied INT-037/041 bindings | High-byte value Invalid; blank handling unavailable until reviewed; keep reasons/raw evidence, do not invent duplicate ambiguity or missing data | L3-EDN-002, L3-EDN-006 |
| `ambiguous_fallback_is_prominent` | Duplicate fixture rendered as CLI text and JSON alongside a cannot-evaluate result | Fallback basis and full ambiguity reason at least as prominent as cannot-evaluate; all occurrences/findings retained and no declaration candidate selected | L3-EDN-006, L3-EDN-007 |
| `mixed_g106_duplicates_keep_ambiguity` | `G\106:zz;G\106:24;`, no override, supplied reviewed bindings/policy | Ambiguous first with every occurrence/duplicate finding; explicit ambiguous-G\106 fallback reason, n=2, not an unrecognised-declaration fallback or selected good candidate | L3-EDN-006 |
| `ambiguous_g106_json_reason_is_structured` | Two duplicate occurrences, then three including an exact repeat; JSON and text renderings | Stable machine code `ambiguous_g106`, numeric `candidates` 2 then 3, separate from display label; count is neither distinct years nor a string parsed from the human label | L3-EDN-006, L3-EDN-007 |
| `selected_basis_does_not_claim_rule_coverage` | Valid explicit selected edition whose requested reviewed definitions are unavailable | Keep selection/basis and coverage limitation distinct; no claim of validation or borrowing baseline definitions without a compatibility contract | L3-EDN-007 |
| `separate_records_keep_separate_edition_evidence` | Two successive records, or separate standalone documents, with one G\106 each | Separate declaration/basis requests, not one duplicate set or propagated version byte; preserve document/revision and record provenance | L3-EDN-001, L3-EDN-006, L3-EDN-007 |

Future tests must keep literal-code fixtures, declaration interpretations and
fallback policy fixtures independent. A source table lookup passing cannot
prove a fallback policy correct, and synthetic reviewed fixtures do not mark
pending interpretations implemented or production coverage complete.

## Family 2 batch 14: joining and provenance acceptance specifications

**Status: planned, not executable tests.** Sources: 106-24R1 Chapter 11
section 11.2.7.2/Figure 11-34 page 11-50 and section 11.2.1.1 f; ADR-0025,
ADR-0031 and INT-011. Fixtures supply a reviewed joining binding to isolate
this contract; INT-011 is reconfirmed by Joey, 2026-11-23, with AI CONCURS
on #38 at c623b15; analysis/tests remain. No fixture here
approves INT-034 or establishes complete/governing records just by joining.

For the exact map oracle below, supply fragment 1 text `G\COM:a` in a
packet beginning at file offset 100 with a 24-byte header, and fragment 2
text `b;` in a packet beginning at offset 136 with a 36-byte combined
primary/secondary header. Each has its own four-byte CSDW. They are adjacent
same-channel 0x01 packets, sequence 255 then 0, matching FRMT=0/RCCVER=0E.
Expected assembled bytes are `G\COM:ab;` (9 bytes); its value is [6,8).
Packet 1 has length 36: header 24 + CSDW 4 + text 7 + filler 1, no
packet data checksum; packet 2 begins immediately at 136. Packet 2 has
length 44: combined headers 36 + CSDW 4 + text 2 + filler 2, no packet
data checksum. Header checksums and slicing bounds are supplied verified;
mapping/join tests do not establish packet validity.

| Planned check | Fixture or stimulus | Expected observation | Requirement |
| --- | --- | --- | --- |
| `joining_requires_per_channel_sequence_evidence` | Two setup fragments with missing channel/sequence evidence, then complete per-channel evidence in a stream interleaved with another channel | Missing prerequisites stay unavailable; global adjacency is not required once the per-channel +1 and other joining predicates are established | L3-CH10-009, L3-CH10-010 |
| `other_channel_packets_do_not_stop_joining` | Channel 0 setup q=255 text `G\COM:a`, channel 3 PCM q=50, channel 7 time q=8, channel 0 setup q=0 text `b;`, matching FRMT/RCCVER | One channel-0 candidate `G\COM:ab;`; other-channel packets retained as provenance/information only, neither stops nor starts that run; no claim of completeness | L3-CH10-009, L3-CH10-010, L3-CH10-011, L3-CH10-016 |
| `two_setup_channels_have_independent_runs` | Pre-106-17 context: channel 1 setup q=10 `G\COM:a`, channel 2 setup q=20 `G\COM:c`, channel 1 setup q=11 `b;`, channel 2 setup q=21 `d;`; each channel's FRMT/RCCVER match | Two independent candidates: channel 1 `G\COM:ab;`, channel 2 `G\COM:cd;`; no mixing or cross-channel interruption; source byte maps retain only their own text, with interleaving provenance retained | L3-CH10-009, L3-CH10-010, L3-CH10-011, L3-CH10-013 |
| `same_channel_packet_stops_joining` | Channel 1 setup q=10, channel 1 non-setup q=11, channel 1 setup q=12; separately interrupt channel 1 while channel 2 continues q=20 -> 21 | First channel has two candidate runs, interruption/sequence evidence retained; no silent 10 -> 12 join. Channel 2 remains one independent run | L3-CH10-009, L3-CH10-011, L3-CH10-016 |
| `run_start_reasons_remain_evidence` | First observed fragment at start of supplied input, then restart after same-channel interruption or FRMT/RCCVER change | Every run has its own evidenced START reason alongside stop evidence; first reason `start of supplied input` does not prove intact beginning; other-channel traffic never causes a restart | L3-CH10-011, L3-CH10-016 |
| `setup_channel_restriction_is_not_joining` | Same-channel nonzero setup fragments with sequence +1, matching FRMT/RCCVER; supplied pre-106-17 and 106-17+ edition contexts | Both join under the same rule; channel-0 conformance is a separate edition-dependent check for the latter context, not rejection, mixing or loss of the candidate run | L3-CH10-009, L3-CH10-010, L3-CH10-011 |
| `joining_sequence_wrap_is_modulo_256` | Exact map fixture, then second sequence changed from 0 to 1 | 255 -> 0 joins; 255 -> 1 stops with discontinuity evidence, without declaring completeness or proving packet loss | L3-CH10-010, L3-CH10-011 |
| `joining_compares_channel_format_and_wire_version` | Independently change second same-channel fragment's FRMT, RCCVER, or data type; separately supply a different-channel setup fragment | Same-channel failed prerequisites stop that channel's join; different-channel fragment belongs to its own independent run and leaves the first open; never equate different RCCVER wire bytes via fallback editions | L3-CH10-009, L3-CH10-010, L3-CH10-011 |
| `srcc_disagreement_survives_joining` | Matching joining operands but SRCC=0 then 1 | No extra SRCC joining predicate; both raw CSDWs/readings retained, no invented aggregate change bit/event or completeness claim | L3-CH10-010, L3-CH10-013, L3-CH10-016 |
| `fragment_text_concatenation_adds_no_bytes` | Exact map fixture, plus metadata/sentinel header, CSDW, filler and packet-checksum slices outside supplied text bounds | Exactly nine assembled bytes `G\COM:ab;`, no inserted delimiter/layout or nontext packet bytes | L3-CH10-012 |
| `assembled_byte_map_has_exact_packet_offsets` | Exact map fixture; assembled offsets 0,6,7,8 | File offsets 128,134,176,177 respectively; offset 7 maps to fragment 2, not fragment 1's one-past-end; retain packet/recording identities | L3-CH10-013, L3-CH10-014 |
| `cross_fragment_ranges_keep_all_sources` | Exact map fixture; item [0,9), value [6,8), empty [7,7) | Item maps [128,135) then [176,178); value maps [134,135) then [176,177); empty range has no bytes; raw spans unaffected by display/projection | L3-CH10-014, L3-CH10-015 |
| `mapping_rejects_invalid_ranges` | Exact map fixture; reversed [8,6), range [0,10), byte offset 9, and overflowing external offset input | Invalid requests reported, never clamped/wrapped or mapped to filler/CSDW; no end-of-body byte invented | L3-CH10-014 |
| `empty_fragment_retains_metadata` | Three qualifying fragments with zero text in the middle | Empty fragment contributes no byte interval; metadata/CSDW/order retained, surrounding text concatenated exactly, no source bytes invented | L3-CH10-012, L3-CH10-013 |
| `split_marker_uses_assembled_offset_zero` | First text slice EF BB, next starts BF followed by code-name text; separately marker after one assembled padding byte | If a separate reviewed completeness decision permits scanning, first marker recognised at assembled offset zero per INT-037; padding case is midstream; joining itself decodes neither | L3-CH10-012, L3-CH10-016 |
| `joining_eof_is_not_completeness` | One joined candidate ending in `;` then EOF; separately a candidate ending inside a value | Both retain their evidenced START reason and EOF stop reason; classification, including possible leading truncation, requires INT-034 rather than assigning complete/incomplete from this joining test | L3-CH10-011, L3-CH10-016 |
| `joining_does_not_hide_untrusted_fragment` | Readable fragment, explicit unreadable/untrusted middle fragment, readable fragment | Trust/gap evidence retained; never silently concatenate readable neighbors as a proven intact record; no new recovery/classification policy | L3-CH10-011, L3-CH10-016 |
| `located_finding_spans_multiple_packets` | Exact map fixture with a supplied located finding over value [6,8); separately a Defaulted field with no source span | Finding retains assembled span and both packet ranges; Defaulted field keeps its citation/basis but no fictional packet span | L3-CH10-015 |
| `candidate_join_does_not_approve_interpretations` | Successful joining with fixture binding and duplicate scalar text across fragments | Candidate report retains membership/evidence; no claim that INT-011 review passed, INT-034 selected a split, checksum verified or configuration governs; INT-039 document-local duplicate evidence remains | L3-CH10-016 |

The existing `multi_packet_setup_record_is_assembled` end-to-end specification
still needs independent completeness, packet-slicing and digest oracles; these
focused cases do not replace it. The cross-fragment duplicate fixture already
records same channel, sequence +1 and matching FRMT/RCCVER as ONE candidate
run's prerequisites, without assuming that repeated scalar text proves its
completeness. No new test marker or implementation status is added here.

## Family 2 batch 15: finality, provisional evidence and limit specifications

**Status: planned, not executable tests.** Sources: 106-24R1 Chapter 9
section 9.4.2 page 9-2; Chapter 11 section 11.2.7.2 page 11-50 and
section 11.2.1.1 f; ADR-0031; reviewer consequences relayed by the owner
2026-11-23 after #38. INT-011 is reconfirmed (AI CONCURS on #38 at c623b15).
Joey signed INT-028/031 and INT-034's snapshot/padding/availability/continuity
choices on 2026-11-28. Q25 A signs padding-only/leading-loss precedence; its independent check remains;
final rule checks CONCUR at #87 e0a0cce; development/testing gates remain.

| Planned check | Fixture or stimulus | Expected observation | Requirement |
| --- | --- | --- | --- |
| `closed_run_classification_retains_evidence` | Supplied complete, incomplete and ambiguous classification bindings for distinct runs | Retain each channel/run, raw extent, START/stop evidence, verdict and all competing readings/spans; never imply semantic approval from fixture success | L3-CH10-017, L3-CH10-019 |
| `eof_boundary_is_complete_and_labelled` | Trusted ASCII run `COMMENT:x;`, normal `start of supplied input` START reason, no independent gap/damage/duplicate evidence, then supplied EOF | Complete, basis `end of input`, labelled; neither EOF nor ordinary START alone downgrades it or proves leading truncation | L3-CH10-017, L3-CH10-018 |
| `same_channel_stop_supplies_followed_packet_basis` | Trusted boundary-ending run, then a non-setup packet on that same channel; supplied reviewed classification binding | Closed-run report uses same-channel stop evidence; if otherwise complete, basis `followed by another packet`; independent conformance findings remain | L3-CH10-018 |
| `other_channel_packet_cannot_finalize_run` | Channel 0 run `COMMENT:x;` remains open while other-channel data packets continue, no EOF | Still open; other-channel provenance/information retained, never final Complete merely from another channel's packet | L3-CH10-018, L3-CH10-020 |
| `open_boundary_is_provisional_not_complete` | Open channel run currently ends in `;`; temporarily no more packets available | PROVISIONAL `text ends at ;, run still open`, observed extent/channel retained; temporary input starvation is not EOF, finality or a join stop | L3-CH10-020, L3-CH10-021 |
| `provisional_extension_never_closes_run` | Observe open `COMMENT:x;`, then append qualifying same-channel fragment `COMMENT:y;`, separately an in-value suffix | Same run extends without a manufactured boundary; updated extent/evidence retained; suffix is not discarded to reuse the older boundary observation | L3-CH10-020 |
| `provisional_use_policy_remains_explicit` | Streaming irig106-time consumer asks to use open setup data; no reviewed provisional-use binding, separately a future opt-in binding | Without binding, no claim of complete/governing authority; opt-in snapshot revision, invalidation and already-emitted-output expectations are open acceptance items, not a selected implementation | L3-CH10-021 |
| `xml_completeness_stays_unavailable` | FRMT=1 run, body happens to end in `;`, then EOF | Unsupported/unknown completeness, not ASCII Complete inferred from one byte; bytes/evidence remain | L3-CH10-017 |
| `competing_split_evidence_is_not_repaired` | Supplied ambiguous classification for repeated single-entry fields or a gap at `;` | All candidates, possible split points and raw bytes retained; neither an INT-044 fallback nor EOF erases independent split evidence or selects a winner | L3-CH10-019 |
| `padding_and_known_leading_damage_need_review` | Boundary text plus trailing CR/LF/NUL padding; separately explicit leading-loss evidence | Retain raw padding/source evidence; Q25 A gives padding-only Incomplete/no loss claim, and proven leading loss blocking cut even under final semicolon/Ambiguous. No invented trim or inferred leading loss from ordinary input start. Different from normal input-start/clean exact-semicolon EOF case | L3-CH10-017, L3-CH10-018 |
| `aggregate_open_run_budget_is_enforced` | Supplied finite byte-accounting model; two channels each below budget individually, their combined retained charge exceeds it | Typed limit error with channel/run/limit/charge evidence, no hidden per-channel-only test, eviction or cross-channel merge | L3-ROB-004, L3-ROB-005 |
| `budget_boundary_and_overflow_are_typed` | Supplied independently charged state C=60, budget B=64, delta=4 then 5 in separate trials; separately overflowing charge arithmetic | Exact boundary fits; over-boundary operation reports typed limit error; overflow cannot wrap into acceptance or panic. These are contract stimuli, not a production accounting oracle | L3-ROB-004, L3-ROB-005 |
| `metadata_and_snapshots_are_charged` | Empty-text fragments and additional channel/run/provenance metadata; separately assembler-retained provisional copies under supplied model | Charge grows even without text growth and includes retained copies; declared byte bound cannot omit metadata. Concrete accounting/defaults remain review work | L3-ROB-004 |
| `limit_error_is_not_eof_or_data_invalidity` | Open runs on channels 1/2, operation on channel 1 exceeds aggregate budget | Typed operational failure; identify accepted/rejected data and ownership path, no silent loss of either run, fabricated EOF/completeness/governance or checksum success | L3-ROB-005, L3-ROB-006 |

Resource-policy defaults, exact memory accounting/allocator overhead, failure
atomicity and resume behavior must be reviewed before executable tests claim
a hard memory bound. No numeric production limit is proposed by the synthetic
B=64 case. Full timelines remain 0.2; provisional pre-EOF metadata is an open
0.1 consumer contract, not silently implemented by a timeline or a fourth
final state. Existing whole-record acceptance cases remain required.

## INT-034 Q25 A selected precedence — 2026-11-28

[Verbatim owner sign-off](research/2026-11-28-int034-q25-owner-signoff.md). Owner-choice holds are
removed for these cases; the new independent check and full development
analysis/review/tests remain. All cases are PLANNED, not executed.

| Planned case | Independently specified expectation | Requirements |
|---|---|---|
| `q25_padding_only_is_incomplete_without_loss_claim` | Closed zero-text candidate, SP/CR/LF/NUL/1A/DEL-only text, and terminated empty regions (;; / CR LF ;) separately (the latter contain no substantive attributes under Q25, not because semicolon is a layout byte): Incomplete, reason no substantive attributes; raw bytes kept, no inferred truncation; existing scanner information/silence rules unchanged | L3-CH10-028 |
| `q25_present_empty_or_unknown_not_padding_only` | COMMENT:; or an unknown framed X:; remains a substantive retained item, not padding-only through value/validity filtering; other classification/validation guards remain independent | L3-CH10-028 |
| `q25_leading_loss_blocks_ambiguity_opt_in` | Independently evidenced lost prefix plus duplicate G PN or SHA: aggregate Ambiguous retains blocking cut/location; labelled per-query ambiguity opt-in never makes it usable/governing | L3-CH10-029 |
| `q25_final_semicolon_keeps_leading_cut` | Known missing leading text but observed suffix ends ';': cut remains; equality/digest evidence cannot remove it or bypass the separate checksum truncation refusal | L3-CH10-029 |
| `q25_input_start_does_not_prove_loss` | Otherwise complete closed substantive run at ordinary start of supplied input, including arbitrary first sequence: no leading-loss/cut inferred from START alone; unrelated findings remain | L3-CH10-029 |
| `q25_provisional_never_hides_known_leading_loss` | Later independently proven prefix loss invalidates an earlier versioned snapshot visibly; known cut never becomes usable/governing through provisional or ambiguity-only access | L3-CH10-029 |
| `q25_raw_extraction_is_not_semantic_access` | Available raw text from a proven-cut candidate remains byte-exact under default extract with findings; semantic/governing acceptance stays blocked, no source reconstruction or new strict-class choice | L3-CH10-029, L3-CLI-031 |

## INT-045: layered time-boundary acceptance specifications

**Status: planned, not executable tests.** Initial Option A is signed by
Joey, 2026-11-23, with AI CONCURS on #40 at 93eb6e2. The mid-session
extension with the premature-seal guard is approved; independent AI CONCURS
on #41 at 79b94eb. Analysis/tests remain outstanding.
INT-011 joining predicates/approval are unchanged. Sources: [source investigation and exact editions/pages](research/2026-11-23-initial-time-boundary-proposal.md).
These cases supply reviewed packet-type/classification bindings to isolate
this approved closure contract; no implementation or broader governance
approval is claimed. Parents identify affected obligations,
not a completed L2/L3 derivation or a new requirement allocation.

| Planned check | Fixture or stimulus | Proposed expected observation | Parent obligations |
| --- | --- | --- | --- |
| `initial_first_time_seals_before_dispatch` | CALLER-ASSERTED, labelled session start, trusted matching channel-0 0x01 fragments q=10/11 with `COMMENT:a` + `b;`, then trusted 0x11 on channel 42, then data | Initial boundary before the first time packet, text `COMMENT:ab;`, distinct initial-time evidence; apply supplied completeness binding, then parse eligible TMATS and dispatch the SAME time packet before subsequent data; no rewind or marker bytes in TMATS | L1-CH10-004 |
| `initial_time_type_is_edition_specific` | Known clean initial prefix; separately first marker 0x11 or 0x12 under supplied 106-15 versus 106-17/24R1 packet bindings, and reserved 0x10/0x13–17 | 0x11 eligible throughout; 0x12 reserved/ineligible in 106-15, eligible from 106-17; other reserved codes never close positively. Valid structurally trusted source-NONE 0x11 still counts; semantic clock availability is separate | L1-CH10-004, L1-CH10-005 |
| `non_time_timestamps_do_not_close_setup` | Secondary-header timestamp, IPTS, Ethernet-contained NTP/PTP, TFMT text, or unknown time-family packet before the first recognised time packet | None is this closure signal; non-setup traffic in the initial prefix is an ordering problem, not permission to choose an end marker or assume a time-source link | L1-CH10-004, L1-CH10-005 |
| `initial_marker_requires_start_order_and_trust` | Absent caller assertion/cropped session start; prior PCM or unreadable packet; corrupt/unchecked marker; time before any setup; or unsupported edition binding | Keep evidence/bytes, report unavailable boundary/adoption or ordering/missing-setup problem; no clean-prefix assertion, invented document, backfill or silent repair. No positive result merely because a type byte resembles 0x11 | L1-CH10-004, L1-CH10-005 |
| `initial_time_does_not_repair_incomplete_setup` | Trusted first time follows cut attribute, gapped/untrusted setup fragments, XML, or unresolved duplicate/split evidence | Boundary evidence does not fabricate Complete/ASCII/unique authority, checksum success or repaired text; INT-034 still supplies the independent verdict or unavailable reason | L1-CH10-004 |
| `no_initial_time_uses_eof_or_provisional` | Clean exact-semicolon TMATS followed by actual EOF, then same bytes on a paused live source; source-NONE metadata but no time packet | Actual EOF may give Complete labelled end-of-input under INT-034 while recording separately lacks required time; live case remains open/provisional with explicit budget and typed limit error if needed. Pause/limit/NONE never manufactures EOF | L1-CH10-004, L1-CH10-005, L1-ROB-001 |
| `initial_runs_and_later_documents_do_not_mix` | Pre-106-17 two setup channels before 0x11, each with its own fragments; separately initial `G\PN:a;`, first time, then later `G\PN:b;`; separately repeated G\PN before first time | Seal initial candidates separately without selecting governance; later setup is a separate candidate, not a duplicate added to the initial document. Repeats within one candidate keep every occurrence/duplicate and possible split; first-time boundary chooses no winner | L1-CH10-004 |
| `mid_session_time_seals_srcc_update` | After initial closure, channel 0 SRCC=1 candidate `COMMENT:update;`, then trusted recognised 0x11 on channel 42; separately applicable 0x12 | Separate closure layer seals update immediately before that time packet; classify under INT-034 and retain evidence before dispatch. Other unchanged-channel DATA alone is not a terminator. General governance remains separately reviewed | L1-CH10-004, L1-CH10-005 |
| `mid_session_cut_update_is_incomplete` | Channel 0 SRCC=1 q=10 text `COMMENT:up`, periodic trusted time on channel 42, then channel 0 q=11 text `date;`, same FRMT/RCCVER | First candidate is INCOMPLETE with located cut finding over retained [0,10) text; do not wait for a later marker or concatenate to `COMMENT:update;` to make it complete | L1-CH10-004 |
| `post_boundary_plus_one_is_continuous_but_split` | Same cut fixture; separately a boundary-ending first candidate with a same-channel +1 fragment after the marker | Record continuous-but-split relation with both candidate/phase IDs, q=10/11, FRMT/RCCVER agreement, marker and source spans; cut text stays INCOMPLETE, otherwise Complete semicolon-ending text becomes finally AMBIGUOUS unless its own G\SHA verifies; never backward rejoin or erase findings | L1-CH10-004 |
| `semicolon_seal_continuation_is_finally_ambiguous` | Channel 0 first-fragment SRCC=1 q=10 `COMMENT:update;`, trusted time on channel 42, then channel 0 q=11 `COMMENT:more;`, matching FRMT/RCCVER and no verifying G\SHA | Initial seal report uses `time-boundary seal (owner policy)`; FINAL outcome AMBIGUOUS, both separate-record and prematurely-split readings retained, located `possible premature time-boundary seal` finding links marker and following fragment; no rejoin. Report revision can reach irig106-time's spool before publication | L1-CH10-004 |
| `verified_sha_keeps_sealed_candidate_complete` | Same continuity, sealed text `COMMENT:update;G\SHA:2-71cfce9820e746cd612c6bf46e324469f55df274bbb1b7ac53fafe6fd30dc39a;`; independent SHA-256 oracle hashes only `COMMENT:update;` | Reviewed, unambiguous match over the SEALED text keeps Complete, basis `time-boundary seal (owner policy)` plus explicit verified-G\SHA classification evidence; continuity/both readings/finding retained, never hash appended text or stamp to force a match. Minimal fixture makes no required-attribute/conformance claim | L1-CH10-004, L1-SUM-002 |
| `srcc_eligibility_comes_from_first_fragment` | First fragment SRCC=1 then later SRCC=0; separately SRCC=0 retransmission fragments followed by time, including a later fragment saying 1 | First candidate eligible for the owner extension from FIRST fragment; disagreement retained. SRCC=0-first retransmission stays outside, even with later 1; no majority/last-fragment upgrade or fabricated restart | L1-CH10-004, L1-CH10-005 |
| `premature_seal_signals_do_not_replace_verified_sha` | Matching continuation plus missing minimum-required evidence, continuation-looking text or high/low packet fill; separately missing/mismatching/malformed/unsupported/indeterminate G\SHA | Support signals labelled with citation and strength (content heuristic, fill weak); none substitutes for reviewed, unambiguous verified G\SHA or allows rejoining. Otherwise Complete candidate is finally AMBIGUOUS, findings/locations preserved; unavailable registry knowledge is not a missing-attribute verdict | L1-CH10-004, L1-SUM-002 |
| `continuation_sha_is_diagnostic_only` | Seal `COMMENT:up` as cut text; later same-channel +1 fragment contains `date;G\SHA:2-71cfce9820e746cd612c6bf46e324469f55df274bbb1b7ac53fafe6fd30dc39a;`, whose digest would match the unadopted joined hypothesis `COMMENT:update;` | May note the hypothetical match as diagnostic information only; sealed candidate stays Incomplete with its cut finding, never gets the sealed-text exception or a rejoin. For an otherwise Complete prefix with matching continuation, a continuation-only digest likewise cannot remove final Ambiguous | L1-CH10-004, L1-SUM-002 |
| `all_non_format1_initial_packets_violate_order` | For each type 0x00/02/03/04 and ordinary data, insert it between caller-asserted session start and first time | Every non-Format-1 packet gives an ordering finding and withheld adoption; being Computer-Generated does not exempt Formats 0/2/3/4 | L1-CH10-005 |
| `session_starts_are_asserted_and_independent` | Caller assertions identify two session starts in one supplied recording stream; separately a file boundary, offset 0, sync, q=0 or SRCC=1 without an assertion | Separate labelled initial phase per assertion; no inferred session start from any of the latter signals; source session/recording wording retained | L1-CH10-004, L1-CH10-005 |
| `untrusted_first_time_cannot_be_replaced` | Caller-asserted initial phase, setup text, untrusted first 0x11, then trusted 0x11 | Withhold adoption with the first candidate's trust finding/evidence; later trusted time never takes over as a repaired initial-first boundary | L1-CH10-004, L1-CH10-005 |
| `channel_zero_time_has_conformance_finding` | Structurally trusted 0x11/eligible 0x12 on channel 0 in 106-17+ | Retain closure/type evidence AND separate channel-use conformance finding; do not silently report compliant use, reinterpret as setup, or count a secondary timestamp instead | L1-CH10-004, L1-CH10-005 |

Further adversarial fixtures must test source/session resets and a fragment
arriving after the proposed seal without merging it backward, plus exact
packet/provenance boundaries. No late malformed continuation is repaired as
earlier text. The mid-session time boundary is explicit owner policy, not a
claim of a printed universal terminator. Event identity remains unverified
under INT-030; a nearby 0x02 does not satisfy it. SRCC=0-first retransmissions
remain outside the extension, and fragment-SRCC disagreement retains its
separate findings. Eligibility comes from the first fragment, not aggregation.
The verified-G\SHA exception is classification evidence only, not proof that
minimum required attributes are satisfied. Missing-attribute support cites
106-24R1 section 11.2.7.2 p. 11-51; continuation content is heuristic and
packet fill is weak evidence. The digest fixture uses independent Python
`hashlib.sha256`, never the library under test or Chapter 6's example digest.

## Planned exhaustive retransmission matrix (owner-signed INT-046–053)

Owner direction dated 2026-11-23 requires every combination, not a sample.
The [matrix and owner questions](research/2026-11-23-retransmission-decision-matrix.md)
list all 139,968 members and their individual planned test names in the
compressed artifact. All are PLANNED, not existing tests. Joey signed the
pairwise decisions on 2026-11-25; the independent AI check remains pending and
blocks implementation readiness. QM/INT-053 has separate bounded sequence
fixtures below; the pairwise product is unchanged.
The documentation expander checks only the artifact.
Sources: 106-24R1 Ch11 §11.2.7.2 p. 11-50 SRCC; Ch6 §6.2.3.11 f pp. 6-22–23
and Ch9 Table 9-2 p. 9-13 checksum; INT-011/018–021/034/039/043/045 and owner-signed INT-046–053.

| Named planned test | Fixture / explicit matrix member | Owner-signed assertions / recorded independent AI check | Requirements |
| --- | --- | --- | --- |
| `retransmission_matrix_exhaustive_coverage` | Every tuple in the versioned domains, including inconsistent observations and every class member; 139,968 currently | Independently exercise all observable output columns; no sampled substitute, missing case, silent owner choice or generator-as-oracle. Count realizable byte fixtures separately from observation consistency/rejection checks | L3-REG-014 |
| `srcc0_different_is_reported_change` | `0.D.NN.C.SS.T.T.I`: bounded following text changes `G\PN:one;` to `G\PN:two;` despite SRCC=0 | Changed claim discrepancy; sealed Ambiguous under existing premature-seal guard; both readings retained, no rejoin. No automatic governing switch (QG) | L3-REG-015 |
| `srcc1_identical_is_not_assumed_change` | `1.B.NN.C.SS.T.T.M`: same complete raw text, following first SRCC=1 | Complete on signed QI, `time-boundary seal (owner policy)` plus `identical retransmission`; report changed claim against identical evidence, no semantic-change assumption; keep the located INT-045 premature-seal finding visible (QI/QG) | L3-REG-015 |
| `sha_only_following_not_sealed_proof` | `0.R.NV.C.SS.T.T.I`: following adds its own well-formed verifying checksum to the sealed text | Not byte-identical. Following-only verification never verifies sealed candidate or cures guard: sealed Ambiguous; report raw change despite SRCC=0; preserve both candidates (QC/QG) | L3-REG-015 |
| `sha_only_sealed_preserves_verified_exception` | `0.R.VN.C.SS.T.T.I`: following removes the sole checksum; sealed checksum verifies sealed bytes | Sealed Complete ONLY on INT-045's existing sealed-text-verification basis; following absence is explicit, raw change reported; no joined digest/adoption inference (QC/QG) | L3-REG-015 |
| `identical_duplicate_sha_never_verifies` | `0.B.DD.C.SS.T.T.I`: identical text contains duplicate G\SHA | F1 indeterminate with every occurrence, stamping refusal; retain byte-identity evidence AND the located INT-045 premature-seal finding. INT-045 baseline Ambiguous with this related follower; owner-signed QD forces assembly Ambiguous independently of the follower and takes precedence over QI; never positive verification or a selected duplicate | L3-REG-015 |
| `duplicate_sha_does_not_hide_premature_seal` | All 144 previously coherent duplicate-sealed-SHA / nonidentical T/C/SS members (138 coherent now, six R0 cut-following/F observations) | Located premature-seal finding and both readings retained regardless of R2D; coherent baseline Ambiguous, six inconsistent rows explicitly R0 | L3-REG-014 |
| `duplicate_sha_without_related_follower_is_ambiguous` | `0.D.DN.G.SS.T.T.I`, plus different-channel/profile/explicit-stop/EOF variants | Owner-signed QD baseline Ambiguous, even without related follower; F1 stays indeterminate/refuse-stamp. No equality or digest upgrade | L3-REG-014 |
| `cut_observation_cannot_claim_mismatch_or_only_sha_layout_delta` | Every sealed X with checksum F or content H/W, plus following X with checksum F, including intersections | Explicit R0 consistency/rejection test, not parser coverage: checksum F must instead be U and bounded H/W cannot explain the cut | L3-REG-014 |
| `other_field_ambiguity_blocks_identity_and_digest` | Repeated definition-bound G\PN within one candidate (INT-039), including identical repeats and sealed G\SHA verification, with byte-identical following text | Supplied A remains assembly Ambiguous under INT-048; no first/last winner or Complete upgrade; retain every item/digest/identity finding | L3-REG-015 |
| `cut_duplicate_keeps_cut_evidence_and_ambiguity` | `0.D.DN.G.SS.X.T.I` plus cut text containing repeated G\PN | Aggregate Ambiguous under strict QD WITH A RETAINED CUT FACET; retain Incomplete's full blocking force, both raw occurrences and cut location. No ambiguous acceptance can make it usable/governing | L3-REG-015 |
| `cut_facet_blocks_ambiguous_acceptance` | Cut text with duplicate SHA or another established field ambiguity, then request the owner-signed INT-031 labelled ambiguous-split acceptance | Acceptance cannot make the retained-cut candidate usable or governing, under the signed option; preserve the cut facet and all findings | L3-REG-015 |
| `duplicate_g106_fallback_does_not_cure_assembly_ambiguity` | Duplicate G\106 with INT-044 labelled fallback | Assembly Ambiguous remains; fallback selects diagnostic rule basis, never a winning declaration or governing setup | L3-REG-015 |
| `following_duplicate_is_assembly_ambiguous` | `0.D.ND.G.SS.T.T.I`, then cut-following variant X | Following candidate assembly Ambiguous under QD even if sealed candidate is Complete; block following adoption and retain incomplete comparison evidence | L3-REG-015 |
| `integrity_findings_default_no_silent_quarantine` | Otherwise eligible initial and mid-session setup with checksum F/M, plus separate F1/framing/trust/ambiguous-scope controls | Default prominent findings; no implicit quarantine or silent old fallback. Controls remain blocking; quarantine opt-in requires separate review | L3-REG-015 |
| `retransmission_boundary_and_projection_mutations` | Mutate named witnesses: partial matching prefix, BOM/damaged/projected names, raw exclusion/leading layout, value-contained G\SHA, missing terminator, uppercase hex, `02-`, layout inside/outside exclusion, channel/FRMT/RCCVER/EOF | Preserve dependency findings, exact raw bytes/spans and every candidate; no normalized equality, silent rejoin or unsupported-format interpretation. Compare independent digests, not production output | L3-REG-015 |

The four owner examples are named regressions in addition to all per-member
names; neither these illustrative text fragments nor documentation outputs
are conforming full setup fixtures. Future byte witnesses must supply all
fragments, independently established extents and relevant provenance. Use
Python hashlib for independent digests. An added/removed checksum belongs to
R, not H (H changes only existing value bytes). Proper tuple constraints found
during witness construction must be reported, reviewed and tested, not skipped.

### Planned QM sequences (INT-053 owner-signed, independent AI CONCUR at #45 73396bc)

Joey's QM decision, 2026-11-25, compares each copy with the CURRENTLY
GOVERNING setup; an unresolved takeover leaves later comparison targets
unresolved, never guessed. Immediate-previous equality is information only.
Preserve every earlier finding and historical governing setup for already
decoded data. [Full sequence contract, explicit targets and control evidence](research/2026-11-23-retransmission-decision-matrix.md#multiple-following-copies-owner-signed-qm-sequence-contract).

Keep the 139,968-member / 39,072-class pairwise product unchanged. The cases
below use an established G0 and up to THREE later copies; this is a fixture
bound, not a limit on recording length/history or arbitrary-length coverage.
All are PLANNED, not executed parser/consumer tests. Control/adoption evidence
must be independently established under reviewed rules, not inferred from
Complete, a checksum, an SRCC bit or previous-copy equality.

| Named planned test | Sequence obligation | Requirements |
| --- | --- | --- |
| `qm_B_then_D_uses_governing_target` | C1 identical to G0 but definitely not adopted; C2 differs from G0. C2 still targets G0; retain C1 findings | L3-REG-016 |
| `qm_H_then_B_previous_identity_is_information_only` | C1 is SHA-value-only change from G0, definitely unadopted; C2 equals C1. Governing comparison is H(G0), not B(C1) as an eligibility/classification input | L3-REG-016 |
| `qm_third_copy_only_checksum_does_not_select_target` | Two unadopted copies then C3 with the only checksum: no retroactive verification or inferred target switch. Addition of a checksum item is R, not H | L3-REG-016 |
| `qm_changed_srcc_claims_never_select_target` | Three copies with 0/1/U claims and fixed control evidence; preserve claim/discrepancy findings without switching target from bits | L3-REG-016 |
| `qm_unresolved_governing_blocks_later_target` | Changed warn-mode integrity-failing C1 with unresolved takeover, then identical C2: target remains unresolved, neither G0 nor C1 selected | L3-REG-016 |
| `qm_ambiguous_copy_does_not_prove_old_target` | QD Ambiguous C1 with unresolved actual takeover, then C2: preserve ambiguity and unknown target, never infer that the old setup governed from an adoption block | L3-REG-016 |
| `qm_valid_takeover_updates_governing_target` | Independently established valid G1 takeover; C2 may equal old G0 yet must compare with G1. Keep G0 for earlier decoded packets | L3-REG-016 |
| `qm_unresolved_then_valid_takeover_resolves_target` | C1 takeover unresolved; C2 independently established valid takeover; C3 targets C2. Preserve C1 findings and unresolved history; never infer takeover from apparent goodness/equality | L3-REG-016 |
| `qm_unresolved_persists_through_non_takeover` | C1 takeover unresolved; C2 definitely no takeover; C3 target remains unresolved, neither C2 nor guessed G0; preserve findings/evidence | L3-REG-016 |
| `qm_previous_identity_mutation_preserves_decisions` | Fix G/candidate/adoption evidence; mutate only immediate-predecessor equality. Eligibility, governing and classification unchanged, with each history's findings retained | L3-REG-016 |
| `qm_findings_and_prior_decoding_provenance_are_persistent` | G0 decoding, G1 valid takeover, then rejected/unresolved copy: preserve every earlier located finding, governing description and packet-to-setup identity | L3-REG-016 |

## Planned time-view metadata batch 16 (0.1 bounded subset)

These are PLANNED tests for L2-VIEW-005–007 / L3-VIEW-012–022, not existing
interfaces or executed coverage. Sources: 106-24R1 Ch9 section 9.5.4,
Table 9-4 TRACK NUMBER/CHANNEL ID p. 9-48; CHANNEL DATA TYPE p. 9-50;
SECONDARY HEADER TIME FORMAT p. 9-51; TIME DATA TYPE FORMAT pp. 9-71–72;
TIME FORMAT and TIME SOURCE p. 9-72. The [batch note](research/2026-11-25-time-view-metadata-requirements.md)
records the source fingerprint, approved INT-035–039/041 and scope.

Unless varied, fixtures supply a reviewed baseline definition/basis and
record/source context plus the required condition evidence. Fragments below
isolate assertions; they are not claimed to be complete conforming setups.
Preserve separate counter/presence/applicability findings. Known scalar
duplicates also impose INT-048 assembly ambiguity: local field reads below
are diagnostic, never permission to govern or bypass retained-cut blocking.

**Numeric dependency (owner follow-up to #50):** Parsed R\N and numeric
code expectations now follow owner-signed INT-055, with final independent AI
CONCUR at #51 ebd5c98. Existing supplied-condition unit arms remain isolated; add parsed
integration arms with valid scoped R\N=1 (or case-specific invalid/unknown
operand), never assuming missing applicability true. Numeric code arms use
the new syntax; raw-only binding arms do not claim conversion coverage.

| Named planned test | Stimulus / independent expectation | Requirements |
| --- | --- | --- |
| `time_view_entry_index_is_not_channel_id` | **Parsed expectation: INT-055; independent final CONCUR at #51 ebd5c98.** Entry n=3 has TK1=42 and CDT=TIMEIN; return scope n=3 and actual ID 42, never ID 3 | L3-VIEW-012, L3-VIEW-014 |
| `time_view_never_borrows_other_scope` | **UNIT: supplied evaluated R\N applicability; no numeric text parsing.** R-1 entry 3 has TIMEIN with absent TFMT; R-2 entry 3 explicitly has TFMT=B. R-1's own applicable default is A, never borrowed B; repeat across document/revision boundaries | L3-VIEW-012, L3-VIEW-016 |
| `time_view_projected_indices_keep_evidence` | **Parsed expectation: INT-055; independent final CONCUR at #51 ebd5c98.** `r-01\cdt-003:timein;R-1\TK1-3:42;`: same numeric scope with raw spelling and noncanonical-index information retained; no rewrite or case-sensitive failure | L3-VIEW-012, L3-VIEW-013 |
| `time_view_unknown_cdt_is_not_confirmed_timein` | **UNIT: supplied evaluated R\N applicability; no numeric text parsing.** CDT absent, high-byte Invalid, duplicate TIMEIN or blank-unavailable in separate cases, with a present TFMT: Confirm no TIMEIN by inference; retain each state/reason, raw candidates and format evidence | L3-VIEW-013 |
| `time_view_unusable_id_is_not_fabricated` | **Parsed expectation: INT-055; independent final CONCUR at #51 ebd5c98.** Confirmed TIMEIN with TK1 absent, 0, or duplicate 42/43: Retain entry scope and Missing/Invalid/Ambiguous identity; never n/0/first/last substitution | L3-VIEW-014 |
| `time_view_actual_id_selection_keeps_all_candidates` | **Parsed expectation: INT-055; independent final CONCUR at #51 ebd5c98.** Two entries (or recorder scopes) each explicitly identify 42; query with insufficient scope: Return all scoped candidates/selection limitation; no merge or conformance claim of global uniqueness. No match fabricates nothing | L3-VIEW-015 |
| `time_view_formats_bind_to_entry_not_extras` | **UNIT: raw, located binding only; no numeric state assertion.** Adjacent entries carry differing TTF/TFMT/TSRC; raw unrelated recorder extras also present: each raw field binds to its full entry key; unrelated extras do not supply metadata or disappear. Any arm asserting TTF Explicit/Invalid follows INT-055 | L3-VIEW-016 |
| `time_view_format_roles_are_distinct` | **Parsed numeric-code expectation: INT-055; independent final CONCUR at #51 ebd5c98.** TIMEIN entry has TTF=2, TFMT=2, TSRC=E and SHTF=1: Keep four roles and independent locations; neither numeric 2 nor E becomes a channel ID, packet-type byte or reset command | L3-VIEW-017 |
| `time_view_duplicate_field_does_not_default` | **UNIT: supplied evaluated R\N applicability; no numeric text parsing.** One entry contains TFMT=A and TFMT=B, then canonical-index collision spellings of the same TFMT key: Field Ambiguous with every occurrence and normalization/collision findings; no first/last/default A rescue; assembly ambiguity retained | L3-VIEW-018 |
| `time_view_explicit_and_default_a_are_distinct` | **UNIT: supplied evaluated R\N applicability; no numeric text parsing.** Explicit TFMT=A versus absent TFMT with known TIMEIN/baseline; repeat explicit lowercase a: Explicit raw value/span versus Defaulted A with citation/basis and no input span; same original bytes before/after view | L3-VIEW-018, L3-VIEW-019 |
| `time_view_blank_and_high_byte_keep_existing_policy` | **UNIT: supplied evaluated R\N applicability; no numeric text parsing.** TFMT=` A` versus a value containing 0xFF: Blank-unavailable per INT-041, not automatic Invalid/trim; high-byte Invalid per INT-037; no default replacement | L3-VIEW-018 |
| `time_view_unknown_condition_or_edition_never_borrows_a` | **UNIT: supplied evaluated R\N applicability; no numeric text parsing.** Absent TFMT with ambiguous CDT or unavailable reviewed edition rule: Missing/default-unavailability or unavailable read with precise reason, not assumed A or data Invalid merely from unknown rules | L3-VIEW-019 |
| `time_view_default_keeps_independent_findings` | **Parsed numeric-code expectation: INT-055; independent final CONCUR at #51 ebd5c98.** TIMEIN, TTF=2, no TFMT, with the RELEASE-0.1 network-time inconsistency finding: Defaulted A stays labelled and the independent finding remains; never override packet evidence, infer UTC or suppress required-presence findings | L3-VIEW-017, L3-VIEW-019 |
| `time_view_local_format_on_non_timein_is_not_absent` | **UNIT: supplied evaluated R\N applicability; no numeric text parsing.** Non-TIMEIN entry explicitly contains letter TFMT=B: Not a confirmed TIMEIN entry; local field remains Explicit with applicability finding, not silently absent | L3-VIEW-013, L3-VIEW-018 |
| `time_view_missing_source_does_not_default` | **UNIT: supplied evaluated R\N applicability; no numeric text parsing.** Confirmed TIMEIN with absent TSRC/TTF: Missing with reason and independent required-presence findings; do not infer external/internal source, default format or an association | L3-VIEW-016, L3-VIEW-018 |
| `time_view_shtf_on_non_timein_does_not_enable_header` | **Parsed numeric-code expectation: INT-055; independent final CONCUR at #51 ebd5c98.** A non-TIMEIN entry has SHTF=2; compare absent and unusable cases: Expose that entry's own field state; no TIMEIN prerequisite, enabled-header inference or neighbouring-channel fallback | L3-VIEW-020 |
| `time_view_provenance_and_revision_are_preserved` | **UNIT: supplied evaluated R\N applicability; no numeric text parsing.** Assembled field crosses fragments; repeat on standalone input and an edited revision: Original spans and supplied fragment map retained; no packet provenance on standalone text and no stale results after revision change | L3-VIEW-021 |
| `time_view_retains_qualification_without_adoption` | **UNIT: supplied evaluated R\N applicability; no numeric text parsing.** Incomplete, cut-plus-ambiguity, other assembly ambiguity or unsupported XML context: Preserve limitation; raw/diagnostic access never becomes unqualified usable metadata, a governing setup or an evidenced clock association | L3-VIEW-022 |
| `time_view_fallback_basis_is_visible` | **UNIT: supplied evaluated R\N applicability; no numeric text parsing.** A field failure or applicable default under supplied labelled fallback edition: Keep fallback edition/basis and field evidence, never a claimed explicit G\106 declaration or input location for a default | L3-VIEW-019, L3-VIEW-021 |

The expected field facts are sourced independently from the cited rows and
approved interpretations. Future Rust fixtures must name the relevant
requirements/interpretations in markers and exercise the real API, including
byte identity after viewing. This table does not create executable tests or
complete the NSB, association, general channel/link or measurement views.



## Planned source-bit batch 17 (explicit NSB only)

PLANNED acceptance cases for L2-VIEW-008/009 and L3-VIEW-023–030; no source-bit
implementation or passing runtime coverage is claimed. Sources: 106-24R1
Ch9 section 9.5.4 Table 9-4 NUMBER OF SOURCE BITS p. 9-33, TRACK NUMBER/
CHANNEL ID p. 9-48; Ch11 section 11.2.1.1 b(1), p. 11-5. The [batch note](research/2026-11-25-source-bit-requirements.md)
records the inspected sources and independent bit-string vector checks.

Unless varied, use a supported reviewed baseline definition and explicit
recorder/document/revision scope, applicable R\ID condition and usable full
TK1 operand. Values shown as hex below are numeric fixture notation, not
an asserted TMATS lexical format. Partial text snippets are not conforming
full-setup fixtures. Retain independent source/assembly/usage findings.

| Named planned test | Independent stimulus / expectation | Requirements |
| --- | --- | --- |
| `nsb_explicit_zero_preserves_full_id` | **Parsed expectation: INT-055; independent final CONCUR at #51 ebd5c98.** Explicit NSB=0, full ID 42: source width 0/value 0, local width 16/value 42; retain the actual declaration, never confuse it with absence | L3-VIEW-024, L3-VIEW-026 |
| `nsb_high_bits_not_low_bits` | **Parsed expectation: INT-055; independent final CONCUR at #51 ebd5c98.** NSB=4, C=0xA02A: source 10, local 42, widths 4/12, full 41002 unchanged | L3-VIEW-026 |
| `nsb_upper_count_and_id_boundaries` | **Parsed expectation: INT-055; independent final CONCUR at #51 ebd5c98.** NSB=13, C=0xFFFF: source 8191, local 7, widths 13/3, full 65535 retained | L3-VIEW-024, L3-VIEW-026 |
| `nsb_zero_local_component_is_not_full_zero` | **Parsed expectation: INT-055; independent final CONCUR at #51 ebd5c98.** NSB=13/C=0xA000 gives source 5120/local 0; NSB=1/C=0x8000 gives source 1/local 0. No reserved-full-ID or TK1-range error is manufactured for the derived zero | L3-VIEW-025, L3-VIEW-026 |
| `nsb_absence_is_not_explicit_zero` | Absent NSB with independently supplied usable full-ID evidence and specified R\ID: retain Missing plus the required-presence finding (Table 9-4 p. 9-33, Required when: Allowed), no fabricated components/count and no presumed explicit zero. This absence check needs no numeric conversion; any combined parsed TK1 variant follows INT-055 | L3-VIEW-027 |
| `nsb_bad_count_never_clips_or_wraps` | **Parsed expectation: INT-055; independent final CONCUR at #51 ebd5c98.** NSB=-1 (invalid syntax), 14 (invalid range), or another demonstrated invalid numeric value under a supported definition: no split, no modulo/clamp/default; raw Invalid evidence remains | L3-VIEW-024, L3-VIEW-027 |
| `nsb_duplicate_even_equal_blocks_split` | **Parsed expectation: INT-055; independent final CONCUR at #51 ebd5c98.** NSB=4 repeated identically, then 4/5: Ambiguous with every occurrence, no chosen count or split; assembly ambiguity remains | L3-VIEW-024, L3-VIEW-027 |
| `nsb_blank_or_unsupported_rule_is_unavailable` | **Parsed expectation: INT-055; independent final CONCUR at #51 ebd5c98.** Leading-blank NSB count retains its existing INT-041 unavailable outcome (L3-VIEW-024); an unavailable historical definition/layout likewise stays unavailable rather than proving data Invalid. Neither trims, assumes baseline or invents zero | L3-VIEW-024, L3-VIEW-027 |
| `nsb_unknown_applicability_does_not_enable_split` | **Parsed expectation: INT-055; independent final CONCUR at #51 ebd5c98.** Explicit local count with unestablished specified-R\ID condition evidence: no guessed applicability; retain the field and blocking condition evidence | L3-VIEW-023, L3-VIEW-024 |
| `nsb_recorder_scope_is_not_borrowed` | **Parsed expectation: INT-055; independent final CONCUR at #51 ebd5c98.** Recorder 1 lacks NSB, recorder 2 has 4; repeat across record/revision. Recorder 1 does not borrow 4; entry-suffixed R-1\NSB-3 is not the native recorder-level declaration | L3-VIEW-023, L3-VIEW-027 |
| `nsb_canonical_recorder_keeps_raw_spelling` | **Parsed expectation: INT-055; independent final CONCUR at #51 ebd5c98.** R-01\NSB binds selected canonical recorder 1, with the spelling/normalization information retained | L3-VIEW-023, L3-VIEW-028 |
| `nsb_actual_id_not_entry_index` | **Parsed expectation: INT-055; independent final CONCUR at #51 ebd5c98.** Entry n=3 has TK1=41002 and NSB=4: split actual C=0xA02A into 10/42, never split n=3 | L3-VIEW-025 |
| `nsb_unusable_id_is_not_coerced` | **Parsed expectation: INT-055; independent final CONCUR at #51 ebd5c98.** Missing, duplicate or out-of-range TK1=0/65536 with valid NSB: retain full-ID state, no n/packet-ID/local-ID substitution or split | L3-VIEW-025, L3-VIEW-027 |
| `nsb_local_id_collision_does_not_merge_channels` | **Parsed expectation: INT-055; independent final CONCUR at #51 ebd5c98.** NSB=4, full IDs 0xA02A/0xB02A both have local 42, sources 10/11. Keep both full/scoped identities; no channel merge or evidenced clock association | L3-VIEW-029 |
| `nsb_provenance_and_qualification_survive_read` | **Parsed expectation: INT-055; independent final CONCUR at #51 ebd5c98.** Declaration/ID spans in assembled or standalone input, labelled edition basis, then revision change and retained-cut/ambiguous context: preserve evidence, raw bytes and limitations without stale results or governing promotion | L3-VIEW-028, L3-VIEW-029 |
| `nsb_all_valid_partitions_match_bitstring_oracle` | Future CI arithmetic test streams C=1–65535 × explicit N=0–13; partition independent 16-character strings, assert 917,490 pairs, widths/ranges and exact reconstruction. Parse/scope assertions use focused cases above | L3-VIEW-026, L3-VIEW-030 |

The all-valid-partitions oracle uses already numerical operands, not raw
NSB/TK1 field strings; its 917,490 arithmetic cases are unchanged. Parsed
binding variants now have INT-055 expectations; development/source/test gates remain
required even where a fixed mathematical vector can be checked independently. The missing-NSB required-presence assertion above
uses absence and the specified-R\ID predicate, not numeric acceptance.

CHE enumeration expectations follow owner-signed INT-054 below; the
independent check has final CONCUR at #51 ebd5c98. No caller filter runs implicitly.
DSI source context is in the remaining-derivations list, not implemented
here or used as a clock reference. No association/correlation or governing
inference follows from common multiplexer source/local values.


## Planned DSI-context batch 18

PLANNED tests for L2-VIEW-010 and L3-VIEW-031–036, not executed coverage.
Source: 106-24R1 Chapter 9 section 9.5.4 Table 9-4 DATA SOURCE ID
R-x\DSI-n, p. 9-49; [batch note](research/2026-11-25-dsi-context-requirements.md).
Queries take supplied recorder/entry scope using approved code-name-index
identity. Every row is an isolated UNIT fixture receiving a scoped evaluated
R\N > 0 result (true unless varied), not parsing R\N or assuming it at runtime.
In particular, the missing-DSI required-presence assertion receives true
applicability. Parsed-R\N integration arms now use INT-055 expectations (R\N=1 gives
true; invalid/missing/ambiguous operands cannot establish true). These units neither test numeric condition evaluation nor
choose CHE enumeration. Use supported reviewed field/condition evidence for
semantic assertions; unavailable results never guess blank/length rules.
Fragments are not complete conforming recording setups.

| Named planned test | Independent stimulus / expectation | Requirements |
| --- | --- | --- |
| `dsi_context_binds_native_entry_scope` | **UNIT: supplied evaluated R\N applicability; no numeric text parsing.** R-1\DSI-3:SRC_A; with other recorder/entry DSI values: return only supplied (1,3) context. Recorder R-1\ID and G\DSI-n do not fill the slot | L3-VIEW-031 |
| `dsi_context_canonical_indices_preserve_raw` | **UNIT: supplied evaluated R\N applicability; no numeric text parsing.** r-01\dsi-003:SRC_A; addresses canonical (1,3), retaining raw spelling and normalization information, without reading a TK1 numeric value | L3-VIEW-031, L3-VIEW-035 |
| `dsi_context_duplicates_are_not_coalesced` | **UNIT: supplied evaluated R\N applicability; no numeric text parsing.** Equal DSI duplicates, then differing duplicates in the same scope: field Ambiguous with every occurrence; preserve INT-048 assembly consequences | L3-VIEW-032 |
| `dsi_context_absence_never_borrows_or_defaults` | **UNIT: supplied evaluated R\N applicability; no numeric text parsing.** Missing native DSI, with nearby entry/recorder labels present: preserve Missing and independently established required-presence findings, never substitute a label/default | L3-VIEW-032 |
| `dsi_context_empty_is_not_absent` | **UNIT: supplied evaluated R\N applicability; no numeric text parsing.** R-1\DSI-3:;: retain a present zero-length value and its supported local state or unavailable reason, never fabricate absence or an empty success document | L3-VIEW-032, L3-VIEW-033 |
| `dsi_context_projection_keeps_raw_evidence` | **UNIT: supplied evaluated R\N applicability; no numeric text parsing.** DSI text contains a control or high byte: apply approved INT-036/037, keep original bytes/findings and labelled display access; no raw display is relabelled a usable semantic value | L3-VIEW-033 |
| `dsi_context_unreviewed_rule_stays_unavailable` | **UNIT: supplied evaluated R\N applicability; no numeric text parsing.** Unknown blank/length or edition-definition prerequisite for a DSI semantic read: raw context retained with unavailable reason, not trimmed/truncated or guessed Explicit/Invalid | L3-VIEW-033 |
| `dsi_context_suggestive_name_is_not_clock_link` | **UNIT: supplied evaluated R\N applicability; no numeric text parsing.** Text such as TIMEIN, CLOCK_A or SRC_A matches another entry's label: no time-source selection, numeric-ID conversion, NSB/TSRC substitution or evidenced association | L3-VIEW-034, L3-VIEW-036 |
| `dsi_context_equal_values_keep_scopes` | **UNIT: supplied evaluated R\N applicability; no numeric text parsing.** Same locally read DSI string in distinct scoped entries: preserve each identity and any independent link/key findings, never merge or assert global conformance | L3-VIEW-034 |
| `dsi_context_provenance_and_qualification_survive` | **UNIT: supplied evaluated R\N applicability; no numeric text parsing.** Standalone or fragmented source, new revision, retained-cut/ambiguous/unsupported context: preserve source/basis/spans and limitations, no fabricated packet locations or stale cross-revision value | L3-VIEW-035 |
| `dsi_context_does_not_require_clock_resolution` | **UNIT: supplied evaluated R\N applicability; no numeric text parsing.** Native context is locally readable while a separate time association or general link result is unresolved/ambiguous: retain DSI context and independent findings without resolving either by guess | L3-VIEW-036 |

CHE and numeric expectations are owner-signed in INT-054/055, with final
independent CONCUR at #51 ebd5c98. DSI length-counting and generic link selection remain
outside these planned context fixtures. All
new requirements remain Draft until real executable verification exists.

## Planned CHE and numeric syntax batch 19

Owner-signed INT-054/055; independent AI final CONCUR at #51 ebd5c98. Requirements:
L2-VIEW-011/012, L3-VIEW-037–042. Sources and owner text are in the
[decision note](research/2026-11-25-che-numeric-syntax-owner-decisions.md).
Use reviewed baseline definitions and scoped applicability. Expected states
are local; independent applicability, assembly and default findings remain.
Every case preserves raw bytes, source locations and warning evidence.

| Named planned test | Independent stimulus / expected result | Requirements |
| --- | --- | --- |
| `che_disabled_timein_is_still_enumerated` | R-1\N:1; R-1\CDT-1:TIMEIN; CHE F vs T: both enumerate the same entry, CHE Explicit false/true with own location; never automatic filtering or packet-generation permission | L3-VIEW-040, L3-VIEW-042 |
| `che_missing_duplicate_invalid_preserve_entry` | Confirmed TIMEIN: absent CHE Missing plus required-presence finding when Allowed; duplicate F/F or T/F Ambiguous with all occurrences/assembly hold; CHE Q Invalid; unavailable definition stays unavailable. Entry remains inspectable, no fabricated enable value | L3-VIEW-041, L3-VIEW-042 |
| `che_caller_filter_is_explicit_and_nonmutating` | Caller explicitly filters disabled from its result, compared with unfiltered view: base still contains disabled entry and every finding; bytes/revision/scopes unchanged; ambiguous/cut setup never gains governing eligibility | L3-VIEW-040, L3-VIEW-042 |
| `numeric_ascii_syntax_and_digit_limit` | NSB empty, +4, -1, 4x, 4.0, non-ASCII digit U+0664, 4 followed by blank or embedded blank: Invalid; 0000000004 (10 digits) Explicit 4 with warning, 00000000004 and 00000000000 (11 digits) Invalid before canonicalisation; no clipped success | L3-VIEW-037 |
| `numeric_zero_padding_keeps_warning_and_raw` | NSB 04 and 00: Explicit 4/0 with located warnings and unchanged raw strings; 014: Invalid range with warning retained; canonical 4 and 0 have no zero-padding warning | L3-VIEW-038 |
| `numeric_ranges_follow_canonicalisation` | NSB 0/13 valid, 14 invalid; TK1 1/65535 valid, 0/65536 invalid; R\N 1/65536 valid, 0/65537 invalid. Repeat with zero-padding within ten digits: same mathematical outcome, warning retained, no wrap | L3-VIEW-038 |
| `numeric_codes_accept_02_without_changing_letters` | Supported TIMEIN: TTF 02 and TFMT 02 match code 2 Explicit with warning; SHTF 00/01/02 match 0/1/2 with warning. Letter TFMT B stays Explicit B with raw spelling, no numeric-code warning; no default replacement | L3-VIEW-037, L3-VIEW-038 |
| `numeric_leading_blank_keeps_int041_exception` | Leading blank in NSB ` 4`, TK1 ` 42`, R\N ` 1`, SHTF ` 1`, TTF ` 2`, TFMT ` 2`: all unavailable under uniform INT-041; existing letter TFMT ` A` unchanged. Trailing/embedded blanks in each numeric field/code are Invalid. Never trim/default; retain independent proved failures | L3-VIEW-037, L3-VIEW-039 |
| `numeric_rn_condition_never_guesses` | R\N=01: Explicit 1/warning, condition >0 true for TK1/CDT/SHTF/DSI/CHE; 0/65537/+1 invalid => cannot-evaluate, not false; missing or duplicate 1/1 => cannot-evaluate with Missing/Ambiguous evidence; unavailable rule likewise. No borrowed count; DSI absence finding only when Allowed established | L3-VIEW-039 |
| `numeric_projection_keeps_character_findings` | NSB `0\r\n4` (actual CR/LF bytes) is Invalid on RAW bytes, never Explicit 4; repeat TAB, NUL, lone CR/LF and each control at start/middle/end of a numeric value. Retain located character/NUL findings, raw bytes and no default. A high byte remains INT-037 Invalid. INT-036 projection cannot make any arm usable | L3-VIEW-037, L3-VIEW-038 |
| `numeric_exclusions_dispatch_by_definition` | Float R-x\ITH-n=-12.5, R-x\TC6=12.5 and hex R-x\MRC\SPM=0A3F, plus digits-only variants: no INT-055 validity verdict or digit-limit/zero-padding warning solely from that rule. Dispatch to the field's own reviewed representation rule; unavailable if it is unreviewed, not a fabricated Explicit/Invalid. Binary R-x\MFD\TRA-n-m=00101 keeps all five bits, never decimal 101; R-x\DMSK-n-m with 32 digits, R-x\AMSK-n-m with 64 digits and R-x\UCSV-n-m with 81 digits incur NO INT-055 ten-digit rejection or zero-padding warning. Include FW/X arms, D-x\WFM-y-n-m-e, P-d\ADM\FDP-n=0011 (four preserved bits, never decimal 11), and digits in character/date/IP fields. Dispatch by reviewed representation; no INT-055 verdict even for all-digit spellings. Retain raw width/bytes and existing G\SHA rules separately | L3-VIEW-037, L3-VIEW-038 |

These are named PLANNED tests. Future property tests should independently
mutate digit runs, signs, blanks, lengths, range boundaries and control/high
bytes, assert canonical equivalence only for admitted raw strings, and check
warnings, raw identity and unusable-condition propagation. No full grammar
coverage or implementation readiness is claimed by this table.

## Planned association/assumption batch 20 (bounded time view)

PLANNED acceptance specifications for L2-VIEW-013/014 and L3-VIEW-043–051,
not runtime tests or an association resolver. Authority: owner direction
2026-11-21 and owner answers 2026-11-26 (INT-056–060, final CONCUR
at #52 3fcbd2c) in RELEASE-0.1. Inspected sources: 106-24R1 Chapter 9 section
9.5.4 Table 9-4 NSB p. 9-33, TK1 p. 9-48, DSI/CHE p. 9-49, CDT
p. 9-50, SHTF p. 9-51, TTF/TFMT/TSRC pp. 9-71–72; [batch note](research/2026-11-26-association-assumption-requirements.md).
None of those fields is treated as a data-channel -> time-channel reference.

Unless varied, supply reviewed baseline metadata and an explicitly scoped
setup document/revision, source data-channel identity and caller-supplied
context basis. No fixture infers governing context. Parsed numeric arms use
approved INT-055; isolated composition units receive effective field/state
results rather than claiming numeric parser coverage. Preserve independent
presence/counter findings in snippets that are not complete conforming setups.

| Named planned test | Independent stimulus / expectation | Requirements |
| --- | --- | --- |
| `association_no_timein_keeps_unestablished_reason` | Scoped setup contains no confirmed TIMEIN entry: association_unestablished with no fabricated candidate and an explicit reason; do not call the association evidenced, resolved or data Invalid merely from absence | L3-VIEW-043, L3-VIEW-044, L3-VIEW-045 |
| `association_only_timein_is_not_proof` | One locally usable TIMEIN entry, scoped TK1 and Explicit TFMT=B; repeat with absent applicable TFMT Defaulted A: retain the respective field evidence, association_unestablished and that candidate; no automatic selection | L3-VIEW-044, L3-VIEW-045 |
| `association_multiple_candidates_never_first_or_lowest` | Several TIMEIN entries without proof, vary input order and actual TK1 values independently of entry indices: unestablished, retain every candidate/scope; never pick first/lowest/nearest, or call the count alone an evidence conflict | L3-VIEW-044, L3-VIEW-045 |
| `association_enable_state_is_only_context` | One enabled and one disabled confirmed TIMEIN, then an explicit caller-filtered projection with one remaining: CHE context survives; no evidenced association or loss of underlying candidate uncertainty solely from enable state/filtering | L3-VIEW-045, L3-VIEW-047 |
| `association_equal_labels_and_source_bits_are_not_proof` | Data/time entries share DSI text or explicit NSB-derived source component in the same recorder: preserve context/scopes, no merge or association proof; repeat distinct labels and recorders without changing evidence support | L3-VIEW-043, L3-VIEW-045 |
| `association_source_category_and_format_are_not_ids` | TIMEIN TSRC=E or I and TTF/TFMT/SHTF declarations numerically resemble another TK1: no clock-selection reference from the category or format; explicit/defaulted evidence stays labelled | L3-VIEW-045, L3-VIEW-047 |
| `association_same_actual_id_retains_scopes` | Same actual TK1 appears on independent recorder scopes, plus entry n differs from actual ID: insufficient query scope retains all candidates/limitation, never uses n or merges equal IDs | L3-VIEW-043, L3-VIEW-047 |
| `association_unknown_extension_is_raw_not_evidence` | Unknown vendor/X attribute has CLOCK_A-like name or a target-looking numeric value: preserve raw occurrence and unknown findings; no supported proof provider, association_evidenced or hidden field interpretation | L3-VIEW-046, L3-VIEW-047 |
| `association_unreviewed_proof_semantics_are_unavailable` | Supplied alleged relation evidence lacks a reviewed interpretation, source or applicability: retain origin/raw evidence and unavailable reason; no affirmative baseline proof fixture or guessed extension semantics | L3-VIEW-046 |
| `association_stale_proof_never_crosses_revision` | Evidence claims a target from another recording/setup/revision or wrong channel scope: cannot establish current association; keep mismatch/reason and historical evidence, never silently transplant it | L3-VIEW-043, L3-VIEW-046, L3-VIEW-051 |
| `association_field_ambiguity_is_not_association_state` | Duplicate CDT/TFMT or unusable TK1 in one possible candidate, supplied as field-state evidence: preserve Ambiguous/Invalid/unavailable and assembly qualifications; do not convert field-state names to association_evidenced or select a winner | L3-VIEW-044, L3-VIEW-047 |
| `association_conflict_composition_keeps_all_evidence` | Isolated unit receives a preclassified association_conflicting result and its competing scoped evidence: keep state, every candidate/reason and locations; no preferred winner. This tests composition, not production admission/classification of proof | L3-VIEW-044, L3-VIEW-047 |
| `assumed_time_target_never_upgrades_association` | Explicit caller selects one possible TIMEIN without proof, with caller basis: every dependent view answer is labelled assumed and carries choice/basis/scope; underlying unestablished remains unestablished; level defaults to Warn and is recorded | L3-VIEW-048, L3-VIEW-049, L3-VIEW-057 |
| `assumed_choice_keeps_supplied_conflict` | Isolated provenance unit receives a preclassified conflict, with no approved-rule different-association predicate supplied for this choice, then an otherwise eligible choice: retain conflict and level-qualified findings, never association_evidenced. This is not a production evidence-admission/contradiction path | L3-VIEW-048, L3-VIEW-050 |
| `assumption_missing_choice_or_basis_is_not_inferred` | Choice or caller basis is not supplied, or selected actual ID names several scopes: no guessed first target/basis or unqualified assumed answer; retain precise missing/ambiguous limitation | L3-VIEW-048 |
| `assumption_does_not_upgrade_bad_target` | Caller chooses a scoped TIMEIN entry with Invalid/duplicate/unavailable ID/format: apply INT-056 Strict/Warn/Quiet (matrix below), accept only at Warn/Quiet as labelled assumption, retain original field states and setup qualification; Quiet suppresses exactly Warn's bad-data findings of any severity, never field/source diagnostics, and records `suppressed: N bad-data findings` even 0 | L3-VIEW-047, L3-VIEW-050 |
| `assumption_provenance_survives_derived_view_outputs` | Select/copy/render a view answer derived from a caller choice: retain assumption label, supplied basis, selected level, source/target/setup/revision; independent unassumed query does not inherit that earlier choice | L3-VIEW-049, L3-VIEW-051 |
| `assumption_cannot_bypass_setup_qualification` | Incomplete/cut-plus-ambiguity/unsupported setup context with caller choice: retain full eligibility limitations and findings, no governing selection or provisional-use permission | L3-VIEW-047, L3-VIEW-050 |
| `association_and_assumption_keep_source_locations` | Standalone versus assembled input with supplied fragment mapping: preserve original item/default basis/spans and fragment provenance; no fabricated packet location on standalone or input span for a default | L3-VIEW-047, L3-VIEW-049 |
| `association_revision_change_requires_new_scope` | New setup/revision repeats TK1/DSI, with a prior association/assumption answer: no silent reuse. Require new applicable evidence or newly scoped explicit choice; keep earlier answers attached to their original scope | L3-VIEW-043, L3-VIEW-051 |
| `association_views_never_edit_or_drop_input` | Vary candidate and caller-selection order, unknown fields and duplicate spelling; before/after read bytes, item order, multiplicity and canonical input diagnostics remain identical; result bad-data finding emission follows its explicit level, with no stamp/normalisation/edit side effect | L3-VIEW-047, L3-VIEW-051 |


### Owner-signed caller-choice fixture matrix (INT-056–060)

All cases are PLANNED; final interpretation CONCUR is at #52 3fcbd2c. The nine bad-data
fixtures enumerate every level x bad-data-kind combination. EACH has both
ID and format arms (18 focused members): a canonical scoped TIMEIN entry is
already identified independently of its bad TK1/format value. No nonexistent
ID-only request is made to look like an identified entry. Keep original
input diagnostics, states, raw bytes, association state and setup limits.
Quiet suppresses EXACTLY Warn's bad-data findings, of ANY severity; they
stay readable from the target's field results AND source diagnostics. Every
Quiet result includes `suppressed: N bad-data findings`, including N=0.

| Named planned test | Independent stimulus / expected result | Requirements |
| --- | --- | --- |
| `assumption_strict_invalid_target` | Existing same-recorder scoped TIMEIN entry; separate bad-ID and bad-format arms, state Invalid. Strict refuses with a typed invalid-target-data reason; no accepted choice; recorded level Strict. Original field states/reasons and canonical input diagnostics unchanged; no association/governing upgrade | L3-VIEW-052, L3-VIEW-053 |
| `assumption_warn_invalid_target` | Existing same-recorder scoped TIMEIN entry; separate bad-ID and bad-format arms, state Invalid. Warn (also repeat with omitted level) accepts as a labelled caller assumption with ALL bad-data findings attached; recorded level Warn. Original field states/reasons and canonical input diagnostics unchanged; no association/governing upgrade | L3-VIEW-052, L3-VIEW-053 |
| `assumption_quiet_invalid_target` | Existing same-recorder scoped TIMEIN entry; separate bad-ID and bad-format arms, state Invalid. Quiet accepts as a labelled caller assumption suppressing EXACTLY the bad-data findings Warn attaches, of ANY severity; target field results AND source diagnostics still expose them. Recorded level Quiet and `suppressed: N bad-data findings` report the exact suppressed count (no omitted zero marker). Original field states/reasons and canonical input diagnostics unchanged; no association/governing upgrade | L3-VIEW-052, L3-VIEW-053 |
| `assumption_strict_duplicate_target` | Existing same-recorder scoped TIMEIN entry; separate bad-ID and bad-format arms, state Ambiguous with every occurrence. Strict refuses with a typed duplicate-target-data reason; no accepted choice; recorded level Strict. Original field states/reasons and canonical input diagnostics unchanged; no association/governing upgrade | L3-VIEW-052, L3-VIEW-053 |
| `assumption_warn_duplicate_target` | Existing same-recorder scoped TIMEIN entry; separate bad-ID and bad-format arms, state Ambiguous with every occurrence. Warn (also repeat with omitted level) accepts as a labelled caller assumption with ALL bad-data findings attached; recorded level Warn. Original field states/reasons and canonical input diagnostics unchanged; no association/governing upgrade | L3-VIEW-052, L3-VIEW-053 |
| `assumption_quiet_duplicate_target` | Existing same-recorder scoped TIMEIN entry; separate bad-ID and bad-format arms, state Ambiguous with every occurrence. Quiet accepts as a labelled caller assumption suppressing EXACTLY the bad-data findings Warn attaches, of ANY severity; target field results AND source diagnostics still expose them. Recorded level Quiet and `suppressed: N bad-data findings` report the exact suppressed count (no omitted zero marker). Original field states/reasons and canonical input diagnostics unchanged; no association/governing upgrade | L3-VIEW-052, L3-VIEW-053 |
| `assumption_strict_unavailable_target` | Existing same-recorder scoped TIMEIN entry; separate bad-ID and bad-format arms, state unavailable with reason. Strict refuses with a typed unavailable-target-data reason; no accepted choice; recorded level Strict. Original field states/reasons and canonical input diagnostics unchanged; no association/governing upgrade | L3-VIEW-052, L3-VIEW-053 |
| `assumption_warn_unavailable_target` | Existing same-recorder scoped TIMEIN entry; separate bad-ID and bad-format arms, state unavailable with reason. Warn (also repeat with omitted level) accepts as a labelled caller assumption with ALL bad-data findings attached; recorded level Warn. Original field states/reasons and canonical input diagnostics unchanged; no association/governing upgrade | L3-VIEW-052, L3-VIEW-053 |
| `assumption_quiet_unavailable_target` | Existing same-recorder scoped TIMEIN entry; separate bad-ID and bad-format arms, state unavailable with reason. Quiet accepts as a labelled caller assumption suppressing EXACTLY the bad-data findings Warn attaches, of ANY severity; target field results AND source diagnostics still expose them. Recorded level Quiet and `suppressed: N bad-data findings` report the exact suppressed count (no omitted zero marker). Original field states/reasons and canonical input diagnostics unchanged; no association/governing upgrade | L3-VIEW-052, L3-VIEW-053 |
| `assumption_refuses_non_timein_target` | Candidate in same recorder has confirmed non-TIMEIN CDT: typed non-TIMEIN eligibility refusal at Strict/Warn/Quiet; no accepted assumption | L3-VIEW-054 |
| `assumption_refuses_other_recorder` | Target TIMEIN belongs to another recorder, even with equal numeric TK1: typed recorder-scope refusal at every level | L3-VIEW-054 |
| `assumption_refuses_nonexistent_id` | ID-only selection has no scoped candidate: typed nonexistent-target refusal, no fabricated entry/ID or first-match fallback | L3-VIEW-054 |
| `assumption_no_01_contradiction_rule` | Actual 0.1 registry has no association-evidence rule. An otherwise eligible choice cannot trigger evidence-contradiction refusal; state remains unestablished with level-qualified assumption result | L3-VIEW-055, L3-VIEW-056 |
| `assumption_heuristics_never_contradict` | Vary order, only candidate, TSRC/DSI/NSB and observed time packets against chosen target: never treat any as evidence or invoke contradiction refusal; retain context/other findings | L3-VIEW-055 |
| `assumption_future_evidence_requires_own_int` | Admission-gate unit supplies hypothetical provider metadata missing its own INT, owner signature, independent check or claim/scope definition in separate arms: cannot establish a different association or trigger contradiction. Conditional unit for a separately established approved-rule different-association predicate requires refusal at every level, retaining the rule identity; it is NOT a baseline proof fixture or approval of a hypothetical source | L3-VIEW-046, L3-VIEW-055 |
| `association_unproven_multiplicity_is_unestablished` | Two or more unproven scoped TIMEIN candidates, all metadata otherwise usable: association_unestablished, never conflicting/evidenced by count; retain each candidate | L3-VIEW-056 |
| `association_conflict_requires_competing_evidence` | Missing/unreviewed/heuristic relation claims cannot produce conflicting. Isolated supplied-state composition with competing-evidence classification preserves conflicting and provenance; no actual 0.1 proof path is asserted | L3-VIEW-044, L3-VIEW-056 |
| `assumption_next_query_needs_explicit_choice` | Query A explicitly selects a target; query B omits target in the SAME setup/revision/session: B is not assumed to use A's target and no remembered choice is filled in. A retains its own provenance | L3-VIEW-049, L3-VIEW-057 |
| `assumption_warn_default_is_not_target_default` | After a Quiet query, query B explicitly supplies target but omits level: recorded Warn DEFAULT with all bad-data findings. Query C omits target: no target from either Warn default or previous queries | L3-VIEW-052, L3-VIEW-057 |

| `assumption_quiet_suppressed_count_zero` | Eligible target with usable ID/format, explicit Quiet and Warn bad-data set empty: labelled assumption, level Quiet, marker `suppressed: 0 bad-data findings` MUST appear; no clean-data implication or field-state upgrade; field/source diagnostics unchanged | L3-VIEW-052, L3-VIEW-053 |
| `assumption_quiet_suppressed_count_nonzero` | Same target/choice yields Warn bad-data set of three independently named findings: information, warning and error. Quiet suppresses exactly those three, marker `suppressed: 3 bad-data findings`; unrelated warning/error findings remain on the result. All three suppressed findings remain readable from target field results AND source diagnostics; states/reasons unchanged | L3-VIEW-052, L3-VIEW-053 |
| `assumption_quiet_marker_survives_derived_result` | Copy/select/render an accepted Quiet result with N=0 and N=3 in separate arms: marker, count, level, assumption label and unchanged field states remain visible; do not drop zero or relabel Quiet as clean. Suppression never hides an independent typed refusal | L3-VIEW-049, L3-VIEW-053 |

The bad-data level matrix never bypasses eligibility or contradiction guards.
Accepting a choice at Warn/Quiet is not a field validity or setup adoption
verdict. No signed proof rule is fabricated by the conditional gate unit.

Future generated checks should permute metadata/order/scopes and caller
choices without synthesizing proof; independently assert no evidenced result
from those mutations alone, unchanged raw bytes, and retained assumption
labels/qualifications on dependent results. There is no field-code invention,
synthetic positive-proof claim, evidence weighting rule or fixed Rust symbol
in this test plan. Meaningful executable markers and source-based oracles
must accompany implementation later; all requirements here remain Draft.

## Planned lossless-output batch 21

PLANNED tests for L2-WRT-003/004 and L3-WRT-004–011, refining existing
L3-WRT-001. Sources: ADR-0002/0003/0010/0029, L1-WRT-001/002 and
106-24R1 Chapter 9 sections 9.4.1–9.4.2 pp. 9-2–3; [batch note](research/2026-11-26-lossless-output-requirements.md).
Use independent original byte vectors, not normalized text or a second
serializer. Retained-document fixtures do not require every parse attempt
to return a document, nor claim malformed inputs conform to the standard.

| Named planned test | Independent stimulus / expected result | Requirements |
| --- | --- | --- |
| `unedited_output_keeps_full_byte_vector` | Known original bytes with attribute gaps/layout: output both as bytes and through supplied sink equals the entire input vector, not only interpreted items | L3-WRT-004, L3-WRT-009 |
| `unedited_output_empty_and_layout_only` | Retained empty, CR/LF, lone-CR and layout-only source: exact original sequence, no synthetic item/newline or fabricated success document on a rejected parse | L3-WRT-004, L3-WRT-005 |
| `unedited_output_keeps_bad_delimiters_and_cut_suffix` | Missing-colon region, empty name/value, extra semicolons and nonterminated suffix: every retained byte remains; findings/qualification persist, no missing-semicolon repair | L3-WRT-004, L3-WRT-005 |
| `unedited_output_keeps_unknown_vendor_extension_items` | Unknown/native-looking, V/X and COMMENT items interspersed with known keys: same input order and bytes, no registry-driven omission or schema reconstruction | L3-WRT-004 |
| `unedited_output_never_coalesces_duplicates` | Equal/conflicting scalar repeats and canonical-index collision spellings: preserve every spelling/occurrence and source position, no chosen winner or deduplication | L3-WRT-004, L3-WRT-006 |
| `unedited_output_keeps_controls_high_bytes_and_padding` | Retained source contains NUL/0x1A/space EOF padding, internal controls and high bytes: output equals raw vector even when interpreted reads discard/control-find or reject high bytes | L3-WRT-004, L3-WRT-005 |
| `unedited_output_unsupported_marker_keeps_source` | Retained-source document with leading unsupported marker or format qualification: copy source, not lossy/Latin-1/UTF conversion; no semantic support/conformance or governing claim | L3-WRT-005 |
| `unedited_output_owns_source_not_mutated_caller_buffer` | Construct owned document from known vector, then modify/release caller buffer: output equals the original known document bytes, not later caller content | L3-WRT-004 |
| `unedited_output_queries_do_not_change_bytes` | Perform raw/effective lookups, canonical-name/index comparisons and labelled display reads in varied order: replay source unchanged, no cache/model reconstruction | L3-WRT-006 |
| `unedited_output_defaults_and_fallback_do_not_materialize` | Applicable absent TFMT default and labelled edition fallback: no inserted TFMT/G\106 field, no rewritten explicit declaration, same full source bytes | L3-WRT-006, L3-WRT-010 |
| `unedited_output_quiet_assumption_does_not_change_source` | Explicit Quiet choice emits its suppression marker/set but retains source diagnostics: output is original vector; suppression never deletes source attributes or repairs invalid numeric data | L3-WRT-006 |
| `unedited_output_assembly_body_is_not_packets` | Supplied assembled TMATS body with fragment map/CSDW metadata: serialize body only, not CSDWs, packet headers/fillers or recording framing; provenance/qualification remains metadata | L3-WRT-005, L3-WRT-009 |
| `writer_partial_acceptance_keeps_order` | Supported partial-write adapter with deterministic accepts 1/2/3 bytes: complete vector exactly once in order or explicit failure under its contract; no dropped/duplicated suffix or truncated success | L3-WRT-007 |
| `writer_no_progress_is_not_unbounded_retry` | Mock destination makes no progress under a fallible adapter contract: terminate with observable failure, never a busy-loop/success; leave document intact, no new retry count/default policy | L3-WRT-007, L3-WRT-008 |
| `writer_failure_before_first_byte_is_visible` | Caller sink fails before accepting anything: return failure/cause, source/revision/findings unchanged, no fabricated complete-output success | L3-WRT-008 |
| `writer_failure_after_prefix_has_no_rollback_claim` | Sink accepts known prefix then fails, with error at final boundary as a separate arm: no success even if a full prefix escaped, error observable, source unchanged, no promise sink reverted | L3-WRT-007, L3-WRT-008 |
| `writer_explicit_retry_uses_fresh_caller_sink` | After reported partial failure caller explicitly retries to a fresh sink: complete original vector, no retained hidden offset or implicit append into the failed external destination | L3-WRT-004, L3-WRT-008 |
| `writer_destination_is_caller_supplied` | Recording fake/byte destination is supplied directly: no library-selected path, stdout/stderr, network/environment or packet destination; existing no-I/O inspection remains separate evidence | L3-WRT-009 |
| `normalization_not_requested_means_replay` | Mixed case/order/newlines, inconsistent counters and repair suggestions with no normalization/edit request: exact source bytes, no counter repair, case fold, newline insertion or sorting | L3-WRT-006, L3-WRT-010 |
| `normalization_is_not_requested_by_view_mode_or_verify` | Tree/pretty presentation preference or checksum verification only: cannot authorize source normalization or stamp; default serialization still byte-identical | L3-WRT-010 |
| `normalization_request_scope_is_separate` | Isolated operation-dispatch unit receives explicit request for an already-reviewed transform: retain request/scope separately from default replay, never add unrequested repairs. This fixture supplies no canonical transformation algorithm or normalized-byte oracle | L3-WRT-011 |
| `normalization_unreviewed_transform_is_not_guessed` | Requested transform lacks reviewed semantics/support: do not invent canonical order/counter bytes or report a successful normalized result; source unchanged, limitation/error visible under the eventual reviewed API | L3-WRT-011 |
| `unedited_replay_generated_vectors_match_original` | Future CI property cases supply bounded independent byte vectors and reader results with retained documents, vary read-only operations and sink paths: complete original-vector equality and source immutability for each; no typed-field/hash-only oracle | L3-WRT-004, L3-WRT-005, L3-WRT-006, L3-WRT-007 |

For generated tests, fix/document a finite seed/vector/operation budget in the
test harness and publish it with results; do not run a local exhaustive input
space. Short-write cases apply only to adapters whose supplied interface
allows partial acceptance. Failure checks observe source and escaped prefix
independently; they do not pretend external side effects are transactional.
All normalization transformations still need their own reviewed rules and
independent byte oracles before those features are implemented. Nothing here
marks the full normalization L1, writer family or release complete.

## Planned core-backed slicing batch 22 (corrected boundary)

PLANNED cases for L2-CLI-003/004 and L3-CLI-004–013, re-pointed under
ADR-0036. Use header-only PacketFacts (irig106-types), borrowed DATA spans and core
PacketInfo integrity/trust sideband ALONGSIDE PacketFacts; no local packet
finder, header/profile validator or checksum algorithm is under test.
Chapter 11 facts/reference values remain in the [original source note](research/2026-11-26-packet-slicing-requirements.md);
the [decision/span note](research/2026-11-26-core-reader-span-rework.md)
records the corrected ownership and exact maximum-case arithmetic. Values
below are internal evidence, not report formats or new core API field names.

| Named planned test | Independent stimulus / expected result | Requirements |
| --- | --- | --- |
| `core_is_only_packet_finder_and_reader` | Instrumented core adapter supplies facts/DATA: no local sync scan, header parser, checksum call or fallback reader; missing compatible core capability remains a dependency gap | L3-CLI-004, L3-CLI-012 |
| `packetfacts_header_only_packetinfo_alongside` | Same header-only PacketFacts with two distinct core PacketInfo sidebands (secondary_integrity/RDG-008 readings, data-checksum state, admission evidence and ranges): each sideband passes unchanged alongside the same header facts, never inserted as new PacketFacts fields or inferred from headers; span/CSDW logic sees the same DATA | L3-CLI-004, L3-CLI-005, L3-CLI-010 |
| `core_primary_trust_passes_through_unchanged` | Supplied core primary evidence/qualifications survive adapter and assembler exactly; no stronger verified verdict from TMATS or duplicate check of source header bytes | L3-CLI-004, L3-CLI-010 |
| `core_rdg008_both_readings_are_preserved` | Core evidence includes RDG-008 byte/word readings 0x0024/0x1410 and its own outcome/qualifications: preserve both and outcome unchanged, no preferred reading or local checksum calculation | L3-CLI-005, L3-CLI-010 |
| `core_data_notchecked_has_no_local_fallback` | Core data evidence NotChecked with checksum-related metadata present or absent: pass NotChecked unchanged; never calculate packet data checksum or synthesize verified/failed/absent from flags | L3-CLI-009, L3-CLI-010 |
| `core_future_data_evidence_is_not_reinterpreted` | Opaque supplied core data-integrity evidence beyond NotChecked: preserve its exact values/qualification through slicing/assembly; no API enum/schema is invented by this fixture | L3-CLI-009, L3-CLI-010 |
| `core_data_extents_are_not_rebuilt_from_headers` | Core supplies DATA references excluding header/filler/checksum with nontrivial source origins: use those spans, never guessed O+24/36 or a new local P/H/C parse | L3-CLI-006, L3-CLI-008 |
| `borrowed_span_local_bounds_do_not_wrap` | Invalid requested logical subrange/overflow or D<4: bounded rejection, no clamp/wrap/fabricated CSDW; check local span contract only, never revalidate packet headers | L3-CLI-006, L3-CLI-008 |
| `core_profile_limits_versions_remain_core_facts` | Core supplies packet profile/size/version qualifications, including unsupported context: keep them without local cap/version tables or G\106/RCCVER fallback; source limits remain reference facts | L3-CLI-007 |
| `csdw_straddles_core_block_boundary` | DATA starts with 0E 01 00 00, split after byte 1/2/3 across different backing blocks: raw four bytes/scalar decode unchanged, logical text starts at 4 once, zero payload coalescing | L3-CLI-008, L3-CLI-012, L3-CLI-013 |
| `text_fragment_straddles_core_block_boundary` | CSDW followed by G\PN:x; text split within its name/value/semicolon in separate arms: same logical text/source mappings, no CSDW skip at a new block or inserted delimiter, zero payload copy | L3-CLI-008, L3-CLI-011, L3-CLI-012 |
| `csdw_and_text_cross_multiple_short_spans` | Several spans shorter than four bytes, plus valid zero-length metadata intervals where supplied: consume exactly first four logical bytes once, retain only remaining text spans; do not require any span to hold a contiguous CSDW/text | L3-CLI-006, L3-CLI-008, L3-CLI-012 |
| `empty_text_span_does_not_prove_complete_setup` | Total DATA D=4 split across blocks: valid logical CSDW, empty text only; D=0/1/2/3 cannot fabricate CSDW. Neither case asserts record completeness/default metadata | L3-CLI-008 |
| `other_core_packet_types_do_not_become_tmats` | Core facts say type 0x00/02/03/04 or time packet: no Format-1 CSDW decode or text append merely from text-like DATA; keep core evidence for its actual type | L3-CLI-004, L3-CLI-008 |
| `assembly_block_boundaries_do_not_stop_runs` | Joined same-channel packet facts satisfy existing INT-011/045 prerequisites; text crosses blocks within each packet: block edges never create joins/stops/closure or new CSDWs, and no payload allocation is required | L3-CLI-011, L3-CLI-012 |
| `borrowed_provenance_maps_blocks_and_packets` | Logical text range crosses both block and fragment boundaries with distinct core source origins: preserve every ordered subrange, owner/block identity, packet metadata and unchanged evidence | L3-CLI-011 |
| `span_owners_live_until_borrows_end` | CI API/lifetime tests retain references while block recycling/drop would invalidate them: safe ownership/borrowing prevents use-after-recycle, no hidden copy claimed as zero-copy success | L3-CLI-011, L3-CLI-012 |
| `maximum_setup_uses_256_borrowed_data_spans` | Core facts for baseline maximum P=134217728,H=24,C=0 supply D=134217704 as first span 524264 then 255 spans 524288: assembler text length 134217700, original references/owners preserved, zero extra payload copy/coalescing, data NotChecked unchanged. Large backing only in bounded CI; local math allocates no payload | L3-CLI-007, L3-CLI-012, L3-CLI-013 |
| `span_count_fixture_is_not_a_production_cap` | Smaller payload split into 257 spans within supplied resource budget: 256-span coverage does not create an undocumented span-count rejection or contiguous-buffer fallback | L3-CLI-007, L3-CLI-012 |
| `core_packet_integrity_is_not_tmats_sha` | Core integrity evidence and TMATS G\SHA/flex results differ or are unavailable: keep domains separate; no local packet verification fills NotChecked and no core packet verdict proves TMATS completeness/governance | L3-CLI-009, L3-CLI-010 |
| `borrowed_input_never_requires_owned_document_first` | Span-backed slicing/assembly is exercised directly without constructing a contiguous owned Document; any explicit owned standalone/materialization API is separate and not invoked behind this path | L3-CLI-012, L3-CLI-013 |
| `core_span_integration_matrix_preserves_evidence` | Future bounded CI varies span partitions, packet metadata/evidence and source origins: same logical CSDW/text bytes and unchanged core evidence, zero payload-copy counter, no reimplementation of core reader/checksum oracles | L3-CLI-005, L3-CLI-006, L3-CLI-010, L3-CLI-013 |

DATA is already core's body view; tmats does not receive trailer bytes just
to checksum them. Core data verification is an owner-scheduled feature
request, not a second local implementation or mandatory new release blocker.
All fixtures are planned; no compatible core API/version, runtime ownership
safety or large-case performance is claimed as tested here. Report/rendered
formats now follow owner Q26/ADR-0039; concrete acceptance remains planned. Standard checksum facts and
example values remain reference evidence, not TMATS acceptance algorithms.

## Planned CLI-mounting batch 24

These fixtures refine L1-CLI-009's accepted reusable runner/command/render
boundary and the post-#55 host-dispatch clarification. They are planned,
not executable coverage, and select no text/JSON output or exit mapping.
Use supplied writers and explicit core facts/sideband/span descriptors;
spies inspect calls, bytes and retained evidence, not rendered layouts.

| Planned test | Independent input and expected boundary result | Requirements |
|---|---|---|
| `mounted_runner_uses_supplied_arguments` | Host process arguments intentionally differ from the supplied invocation; runner uses supplied arguments under the later reviewed argument contract | L3-CLI-022 |
| `mounted_runner_uses_supplied_streams` | Host-supplied memory streams receive runner writes; process stdout/stderr are not substituted | L3-CLI-022 |
| `mounted_runner_success_returns_to_host` | Subprocess host driver calls a real successful invocation, then records a sentinel after return; process-exiting code cannot pass | L3-CLI-022, L3-CLI-023 |
| `mounted_runner_failure_returns_to_host` | Real unsuccessful invocation returns its classification and host executes a later sentinel; no numeric mapping or rendered diagnostic is asserted | L3-CLI-022, L3-CLI-023 |
| `standalone_and_mounted_runner_share_execution` | Same reviewed command inputs traverse the same public runner/computation; standalone only supplies process inputs and maps status at its outer boundary | L3-CLI-023 |
| `mounted_command_without_main_or_runner` | A real reviewed individual command executes without entering main or whole-runner parsing and returns its report without direct printing | L3-CLI-024 |
| `mounted_command_reuses_runner_computation` | Equivalent reviewed inputs through individual command and runner reach the shared computation; no second command implementation in host | L3-CLI-023, L3-CLI-024 |
| `mounted_renderer_uses_supplied_writer` | Renderer receives an actual command report and supplied memory writer; writes there without acquiring process streams; output format is not pinned here | L3-CLI-025 |
| `mounted_renderer_failure_returns_to_host` | Supplied writer fails after a bounded prefix; reviewed writer failure remains visible, no delivery success claimed, host continues | L3-CLI-025 |
| `mounting_host_dispatches_packet_types` | Instrumented host uses core facts to route setup DATA to TMATS and other DATA to their consumers; adapter performs no dispatch or sync scan | L3-CLI-026 |
| `mounting_adapter_format_guard_is_defensive` | Deliberately misroute non-Format-1 facts with text-like DATA; host receives an observable typed wrong-input-kind refusal carrying the supplied data type, never silent success/no-op; no CSDW/text interpretation or append, alternate dispatch, raw-header parse or trust upgrade; no spelling/rendering assertion | L3-CLI-026, L3-CLI-027 |
| `mounting_format1_is_not_csdw_frmt` | Format-1 facts with either CSDW FRMT bit pass the packet-kind guard; FRMT-specific support/interpretation stays separate, never ASCII forced by the guard | L3-CLI-027 |
| `mounting_format_guard_does_not_admit_packet` | Format-1 facts match but sideband remains qualified/untrusted/NotChecked; matching the guard never establishes trust, completeness or governance | L3-CLI-027, L3-CLI-029 |
| `mounting_source_route_is_not_packet_dispatch` | A recording-selected source is read by core once; per-packet dispatch never calls the source-prefix selector on each DATA payload | L3-CLI-026, L3-CLI-029 |
| `mounting_time_consumer_preserves_tmats_boundary` | Host sends recognised trusted time DATA to time consumer and supplies qualified boundary observation to assembler; applicable INT-045 closure still occurs | L3-CLI-028 |
| `mounting_untrusted_time_keeps_qualification` | Host supplies time observation with original untrusted qualification; dispatch never promotes it to a trusted closure or lets a later packet silently take over | L3-CLI-028, L3-CLI-029 |
| `mounting_same_channel_stop_is_not_filtered` | Host routes same-channel non-fragment packet to another consumer while preserving interruption observation; INT-011 run stops under its existing rule | L3-CLI-028 |
| `mounting_other_channel_packet_keeps_provenance` | Interleaved other-channel observation remains provenance without stopping INT-011 join; any INT-045 time closure is evaluated separately | L3-CLI-028 |
| `mounting_session_start_remains_caller_asserted` | Host supplies or omits labelled session-start provenance; mounting does not infer it from offset zero, filename, sequence or SRCC | L3-CLI-028 |
| `mounting_borrowed_spans_and_sideband_survive` | Fragment DATA split across blocks retains source identity/ordered spans and PacketInfo sideband alongside header-only PacketFacts; no coalescing, extra traversal or payload copy imposed by mounting | L3-CLI-029 |
| `mounting_guard_keeps_observation_path` | A guarded misrouted fragment call returns an observable typed wrong-input-kind refusal carrying the supplied data type to the host, never silent success/no-op, and creates no text append; separately supplied packet observations still reach assembly with their original qualifications; no spelling/rendering assertion | L3-CLI-027, L3-CLI-028 |

Expected packet-level outcomes come from INT-011/045 and existing core/span
requirements, not a new classifier in the fixture. No local packet or
secondary/data-checksum verification oracle is permitted. Executable tests
must cover the actual reviewed commands when implemented; scaffold/help
success does not demonstrate every command's mountability. Full report
formats and numeric exit mappings now follow Q26/ADR-0039 and the report
acceptance rows below; this earlier mounting batch does not test rendering.

## Planned extraction and safe-output batch 25

These are planned acceptance cases for L2-CLI-008/009, not executable tests.
Extraction receives one explicitly selected record/candidate and available
raw text extent. ADR-0037 records the owner's 2026-11-27 policy, removing
#57's three owner holds: raw extract under findings by default, every finding
reported; --strict refuses the listed bad-data kinds/class, not informational
notes (owner Q18, ADR-0038). Known cut prefixes are valid raw
extraction inputs, not missing whole-document failures. Stdout/pipes are
allowed; unproven regular identity and existing unrelated -o files require
--force; same-handle writes, no temp-file+rename. Known input aliases refuse.
These are policy decisions, not passing implementation evidence. Other
argument forms need reviewed definitions; Q26/ADR-0039 supplies report policy; file-acquisition guarantees
cover tool-opened destinations, not shell redirection.

| Planned test | Independent input and expected boundary result | Requirements |
|---|---|---|
| `extract_selected_text_is_byte_exact` | Supplied text `COMMENT:a;` is delivered exactly, with no newline, terminator change, label or wrapper | L3-CLI-030, L3-CLI-031 |
| `extract_fragment_text_excludes_each_csdw` | Two packet text fragments `COMMENT:a;` and `COMMENT:b;` yield literal `COMMENT:a;COMMENT:b;`; exclude both CSDWs and every header/trailer byte | L3-CLI-030 |
| `extract_csdw_split_is_removed_once` | Synthetic arbitrary-span partition: four logical CSDW bytes straddle spans; expected text excludes four bytes total, not four per span. With core 0.2 default 512 KiB blocks the CSDW is always in the first block; this is not a production-layout claim | L3-CLI-030, L3-CLI-037 |
| `extract_text_split_does_not_change_bytes` | A text fragment crosses several span boundaries including inside a name/value; compare to the same independent literal | L3-CLI-030, L3-CLI-037 |
| `extract_layout_and_case_survive` | Mixed case, leading/inter-item layout and CR/LF are replayed exactly; no canonical case, order or newline conversion | L3-CLI-031 |
| `extract_comment_control_is_view_only` | Mechanical source `COMMENT:a\tb;` with a real TAB: INT-036 interpreted value is `ab` but extraction keeps `a\tb` byte-for-byte; the same applies to an omitted CR/LF/control; default extract allowed with all findings; strict allows this information-only case and reports the note, with bytes unchanged (owner Q18) | L3-CLI-031 |
| `extract_numeric_control_invalid_keeps_raw` | Mechanical source `R-1\NSB:0\r\n4;` with real CR/LF: INT-055 raw-byte numeric check returns Invalid, never repaired to 4; extraction keeps every control/raw byte; default extract allowed with all findings, --strict refuses when findings exist | L3-CLI-031 |
| `extract_assembled_leading_bom_is_not_stripped` | Same-channel +1 fragments with matching FRMT/RCCVER: first text starts EF, following text starts BB BF then `COMMENT:a;` (after each packet CSDW); leading assembled EF BB BF is detected under INT-037 at text offset zero, never stripped from extraction; default extract allowed with all findings, --strict refuses when findings exist | L3-CLI-030, L3-CLI-031 |
| `extract_raw_unknown_vendor_extension_survive` | Supplied raw text contains unknown, V and X items; none is dropped or rendered from a typed model | L3-CLI-031 |
| `extract_raw_damage_is_not_repaired` | Mechanical sink input contains duplicate/malformed items, high bytes and an unterminated suffix; default exact replay with all findings; strict refuses listed bad-data kinds, other findings follow owner Q18's reviewed bad-data-class rule, while informational notes alone do not refuse | L3-CLI-031 |
| `extract_raw_padding_is_not_packet_fill` | Record text contains trailing preserved NUL/space bytes; replay them, while packet filler remains excluded by core DATA extents | L3-CLI-030, L3-CLI-031 |
| `extract_read_only_views_do_not_stamp` | Read-only views/findings/checksum verification precede extraction; original text, including original checksum spelling/value, stays unchanged | L3-CLI-031 |
| `extract_selected_identity_never_substituted` | Two distinct supplied records have different literal texts; the explicit eligible selection alone supplies output and provenance | L3-CLI-032 |
| `extract_unresolved_selection_never_becomes_empty_success` | Missing/unresolved selection cannot become success with an empty document or a different selected record; exact public error spelling is not pinned | L3-CLI-032 |
| `extract_distinct_records_never_concatenated_implicitly` | One selected record plus another available record outputs only the selected text; multi-record output policy remains undecided | L3-CLI-032 |
| `extract_short_writes_preserve_order` | Bounded sink accepts positive partial writes under existing writer contract; delivered bytes equal independent literal in order | L3-CLI-033, L3-CLI-037 |
| `extract_sink_failure_keeps_source` | Sink fails before/after a bounded prefix; failure stays observable, no full-delivery claim, source bytes and provenance unchanged | L3-CLI-033, L3-CLI-035 |
| `output_same_path_is_refused_before_truncate` | Destination is input path; open without truncating, compare handle identity, host observes refusal before any truncate/write, input bytes unchanged | L3-CLI-034, L3-CLI-036 |
| `output_relative_absolute_input_alias_is_refused` | Distinct relative/absolute spellings name the input; no destructive open, input unchanged | L3-CLI-034, L3-CLI-035 |
| `output_symbolic_link_input_alias_is_refused` | Supported platform symlink destination identifies input; refuse before modifying either input contents or replacing its file | L3-CLI-034, L3-CLI-035 |
| `output_hard_link_input_alias_is_refused` | Supported platform hard-link destination identifies input despite path inequality; refuse before destructive open | L3-CLI-034, L3-CLI-035 |
| `output_unestablished_input_safety_never_opens_destructively` | Unproven regular identity without --force: no destructive acquisition or successful output; the explicit-force case is covered by owner-policy fixtures | L3-CLI-034, L3-CLI-036 |
| `output_acquisition_failure_never_redirects` | Destination acquisition fails; observable failure returns, no automatic stdout/alternate-file output or input repair | L3-CLI-035, L3-CLI-036 |
| `output_distinct_sink_success_keeps_input` | Isolated distinct regular-file output, with ADR-0037 authorization (--force when existing): non-truncating open, compare input identities, truncate/write through the SAME handle (no pathname reopen); input snapshot remains identical | L3-CLI-035, L3-CLI-037 |
| `output_failure_returns_to_mounting_host` | Host calls shared command with failing sink/acquisition; failure is observable and host executes subsequent sentinel | L3-CLI-033, L3-CLI-036 |
| `extract_oracle_is_independent` | Expected text comes from literal source/fragment descriptors; fixture never invokes production extraction/writer to calculate expected bytes | L3-CLI-037 |

Filesystem cases need isolated temporary fixtures and platform-identity/
acquisition analysis, including race behavior; instrument open/check/truncate
ordering and assert no pathname reopen before truncation. String checks alone cannot
verify the obligation. Unsupported link creation is recorded as unavailable
coverage, not a safety pass. ADR-0037 now selects extract's overwrite and
finding policy; other grammar, cleanup/durability and report layout remain
separate. Preserve source/format observations without inventing core byte
extents or parsing unsupported bodies; end-to-end implementation still needs
meaningful tests, including the owner-policy cases below.

## Planned raw pattern lookup batch 26

These are planned cases for L2-VIEW-015/L3-VIEW-058–064, not executable
coverage. Query notation below describes bindings, not a public string
syntax: use the cited R-x\TK1-n pattern from 106-24R1 Ch9 Table 9-4
TRACK NUMBER/CHANNEL ID p. 9-48. Named items A/B/C are independent fixture
identities with explicit literal source ranges. Values are raw candidates,
not interpreted TK1 verdicts. Existing findings remain located/observable.
Owner Q4/5 selects reviewed-registry-only whole-part/index * in 0.1,
with ANY-name raw exact lookup (2026-11-28, ADR-0038). Owner-choice holds
are removed; source/design review and executable checks still gate code. Diagnostic candidate fixtures do not approve incomplete/ambiguous
access: see the [open item](ROADMAP.md#streaming-setup-record-follow-ups)
and [owner-signed INT-031](INTERPRETATIONS.md#int-031). Any supplied record
qualification, including Ambiguous, cut facets and Incomplete, stays attached
and unchanged when candidates are returned.

| Planned test | Independent source/bindings and expected candidate result | Requirements |
|---|---|---|
| `pattern_bound_recorder_keeps_all_entries` | A=`R-1\TK1-3:a;`, B=`R-2\TK1-1:b;`, C=`R-1\TK1-1:c;`; bind x=1, n unbound: [A,C] in source order, not [C,A] | L3-VIEW-058, L3-VIEW-060 |
| `pattern_unbound_occurrence_is_not_default_one` | Same A/B/C; both positions unbound: [A,B,C]; bind x=3: empty collection, never reuse recorder 1 | L3-VIEW-059, L3-VIEW-063 |
| `pattern_literal_path_is_not_substring` | TK1 query does not match literal TK10, another path segment or TK1-like bytes inside a COMMENT/value; original items remain retained | L3-VIEW-058, L3-VIEW-061 |
| `pattern_literal_case_is_ascii_only` | Name `R-1\tK1-3` (path separator backslash then ASCII lowercase t, NOT a TAB) matches the TK1 pattern under Ch9 section 9.4.2 p. 9-2; replace K (4B) with high byte CB: no match or high-byte/locale/Unicode folding, raw damage retained | L3-VIEW-058, L3-VIEW-059 |
| `pattern_full_arity_never_matches_extra_index` | A complete key under the cited pattern matches; extra/missing index or path component is not guessed into that pattern; retain unusable raw source | L3-VIEW-058 |
| `pattern_bound_index_uses_canonical_index_identity` | A=`R-01\TK1-003:a;`, B=`R-1\TK1-3:b;`; bind x=1,n=3 returns [A,B] with raw spellings and INT-038 normalization/collision findings; record remains INT-048 Ambiguous with qualification attached and unchanged | L3-VIEW-059, L3-VIEW-060, L3-VIEW-063 |
| `pattern_name_projection_keeps_internal_omission` | Name `r-01\T\r\nK1-003` contains real CR/LF; approved projection matches x=1,n=3 and retains internal-omission/normalization findings and raw bytes | L3-VIEW-059, L3-VIEW-061 |
| `pattern_long_index_never_uses_field_digit_limit` | Entry index is 40 ASCII digits; same literal canonical-string binding matches without integer conversion or INT-055 ten-digit cutoff; index range validity is not asserted | L3-VIEW-059 |
| `pattern_absent_occurrence_is_not_invented` | Name without the required occurrence cannot match by supplying occurrence 1 implicitly | L3-VIEW-058, L3-VIEW-059 |
| `pattern_exact_repeats_are_not_deduplicated` | Three repeated source occurrences of one complete key return all three IDs in order, with INT-039 duplicate and INT-038 collision findings where applicable; record remains INT-048 Ambiguous with qualification attached and unchanged | L3-VIEW-060, L3-VIEW-063 |
| `pattern_fully_bound_agrees_with_exact_and_literal_oracle` | Fully bind x/n: pattern and exact lookup both return independently listed IDs/ranges in source order; neither path calculates the expected list | L3-VIEW-060, L3-VIEW-064 |
| `pattern_empty_value_is_a_present_candidate` | Matching `R-1\TK1-3:;` returns one raw item with an empty value range; not zero matches or a valid/defaulted TK1 | L3-VIEW-061, L3-VIEW-062 |
| `pattern_invalid_numeric_value_is_not_filtered` | Matching name with value `+2` or raw control bytes remains a raw candidate despite any INT-055 Invalid verdict; no value repair/query filtering | L3-VIEW-061 |
| `pattern_high_value_byte_keeps_candidate` | Matching name with value byte FF retains candidate/raw byte and INT-037 Invalid field evidence; not a leading document marker | L3-VIEW-061 |
| `pattern_damaged_name_is_not_fabricated` | Name includes a high byte and has no complete usable key: keep raw item/diagnostics but do not invent a match; collection alone makes no absence claim | L3-VIEW-061, L3-VIEW-062 |
| `pattern_leading_bom_is_unavailable_not_empty` | Leading assembled EF BB BF blocks interpreted matching under INT-037; expose unavailable reason/finding and raw source, never successful zero matches | L3-VIEW-062 |
| `pattern_unavailable_binding_is_not_zero_matches` | Query lacks usable reviewed pattern/binding metadata: observable unavailable reason, not a manufactured empty success | L3-VIEW-062 |
| `pattern_suffix_is_not_item_but_can_block_presence_false` | Unterminated compatible suffix is not returned as an item under INT-035; independent INT-042 predicate keeps cannot-evaluate rather than inferring false from empty query | L3-VIEW-062 |
| `pattern_cross_fragment_duplicates_share_document` | Same channel, sequence +1, same FRMT/RCCVER form ONE ADR-0025/INT-011 record; matches on each fragment both appear with packet/body provenance and duplicate evidence; record remains INT-048 Ambiguous with qualification attached and unchanged | L3-VIEW-060, L3-VIEW-061, L3-VIEW-063 |
| `pattern_other_document_never_supplies_missing_match` | No match in supplied document; a later separate record/document contains the key: query does not borrow it or choose a governing record | L3-VIEW-063 |
| `pattern_defaults_never_create_source_items` | A view cache contains an effective/defaulted value; raw pattern retrieval still returns only actual source occurrences and never mutates source bytes | L3-VIEW-061, L3-VIEW-063 |
| `pattern_permutation_partition_oracle_preserves_multiplicity` | Bounded CI permutations/span partitions: independently permuted literal IDs/ranges determine result; retain every occurrence and source bytes, not production-matcher-derived expectations | L3-VIEW-064 |

Encoding availability, raw candidates and effective/association/presence
answers remain distinct. No query glob/regex syntax, private-code grammar,
link inference, renderer/schema/error spelling or output policy is chosen.
No fixture or executable test marker is claimed as implemented by this table.

## Planned build-compatibility inspection batch 27

These are planned inspection/CI acceptance cases for L2-REL-002/003/004,
not executed Rust tests or Evidence. Inspect actual package/configuration
binding and completed artifacts at one exact review head. Negative cases
are future acceptance-oracle examples, not mutations performed by this PR.
No prototype build is presented as redesigned-library acceptance.
Owner Q6 (Joey, 2026-11-28) removes the std-choice hold: default-on,
always-required std; "not a no_std promise" (ADR-0015 amendment). The exact
no-default-features + explicit std consumer declaration stays unchanged.

| Planned acceptance case | Evidence required or independent rejection condition | Requirements |
|---|---|---|
| `compat_msrv_inherits_both_packages` | Resolve actual library/CLI TOML inheritance to rust-version 1.85 under ADR-0035; comments or virtual-root-only metadata cannot pass | L3-REL-002 |
| `compat_msrv_detects_one_package_mismatch` | Library inherits 1.85 but CLI declares a different floor: mismatch remains visible, not hidden by the library manifest | L3-REL-002 |
| `compat_msrv_runs_real_185_check` | Completed Rust 1.85 check includes both redesigned packages and resolved production dependencies/configuration; actual rustc -V output, compiler/toolchain/features and head are identifiable | L3-REL-003 |
| `compat_stable_is_not_msrv_evidence` | Stable succeeds while the 1.85 job is missing/failed: MSRV is unsatisfied; do not infer success or silently raise the floor | L3-REL-003, L3-REL-007 |
| `compat_dependency_msrv_failure_is_not_waived` | Dependency check fails on 1.85: retain compatibility gap, do not omit CLI/supported features or substitute publishing toolchain | L3-REL-003 |
| `compat_wasm_targets_plain_library` | Completed CI check explicitly targets wasm32-unknown-unknown and redesigned library with recorded features, not native/default host or CLI | L3-REL-004 |
| `compat_wasm_bindings_do_not_substitute` | Old wasm-bindgen/prototype feature or wrapper-only success does not satisfy the plain-library target obligation | L3-REL-004, L3-REL-008 |
| `compat_wasm_runs_from_first_new_code` | Once redesigned library exists, every-change check is present and actually executed; a permanently skipped job is missing evidence | L3-REL-004, L3-REL-007 |
| `compat_three_platforms_build_and_test` | Completed Windows/Linux/macOS results at the same head cover both packages and implemented tests, not only a matrix declaration | L3-REL-005 |
| `compat_one_platform_failure_stays_unsatisfied` | Two platforms succeed, third fails/cancels/is absent: that platform is not accepted through another platform's pass | L3-REL-005, L3-REL-007 |
| `compat_scaffold_tests_do_not_prove_commands` | Help/version-only scaffold passes: substantive redesigned command/library checks are still required once those areas exist | L3-REL-005 |
| `compat_platform_fixture_unavailable_is_not_pass` | Platform cannot create a required fixture/link: record unavailable coverage without approving an output-safety waiver | L3-REL-005 |
| `compat_consumer_declaration_is_exact` | Isolated consumer keeps version 0.1, default-features=false, features=[std]; no defaults/all-features substitution; reviewed public parse/time-view calls compile | L3-REL-006 |
| `compat_workspace_feature_leak_is_not_consumer_success` | Workspace all-features succeeds but isolated exact consumer fails: consumer obligation remains unsatisfied | L3-REL-006, L3-REL-007 |
| `compat_consumer_uses_reviewed_source` | Before publication, source override separately binds unchanged dependency entry to reviewed head; inspect actual resolved source/version, not a placeholder | L3-REL-006 |
| `compat_consumer_waits_for_actual_time_view` | Before view exists, requirement is not falsely claimed implemented; after it exists, consumer must call the real reviewed API, not an empty main | L3-REL-006 |
| `compat_old_head_artifact_does_not_verify_new_head` | Matching command/OS with another source SHA does not satisfy current acceptance | L3-REL-005, L3-REL-007 |
| `compat_each_check_retains_own_qualifications` | Native success cannot substitute for WASM/MSRV/consumer; retain targets/features, job state and command for every obligation | L3-REL-007 |
| `compat_compile_does_not_claim_bindings_or_no_std` | Plain-WASM or std consumer compiles: bindings, no_std and semantic parser coverage remain separate/unproved | L3-REL-004, L3-REL-008 |
| `compat_source_override_is_not_dependency_publication` | Local reviewed source builds: types/core publication/API prerequisites and qualified-record owner gates remain outstanding unless separately satisfied | L3-REL-006, L3-REL-008 |
| `compat_std_default_is_required_and_supported` | Actual default-feature redesigned builds enable required std on supported native/MSRV/library-WASM targets; declaration alone does not count as executed acceptance | L3-REL-003, L3-REL-004 |
| `compat_std_explicit_without_defaults_is_supported` | Isolated positive build disables defaults and enables exactly std, preserving the owner consumer declaration; workspace feature unification cannot hide a missing required feature | L3-REL-003, L3-REL-006 |
| `compat_std_absent_is_unsupported` | Build omitting std is documented as unsupported and excluded from supported acceptance even if compilation succeeds; no compile-time failure is mandated by the owner support policy. Record actual outcome if run, never claim positive no_std support; keep plain-WASM and positive explicit-std consumer distinct | L3-REL-003, L3-REL-008 |

Future Evidence must name inspected configuration plus the actual run/job
artifacts; these rows alone provide no evidence. The current docs-only CI
can check documentation drift and the prototype, but cannot execute an
unimplemented redesign. Heavy builds/tests remain in bounded GitHub CI;
local checks for this batch stay lightweight. No feature, workflow or
manifest is changed, and no publishing/package operation is performed.

## Planned library guard-rail/coverage inspection batch 28

These cases refine L1-ROB-002/003 and L2-ROB-004/005. They are planned
inspection/CI acceptance specifications, not executed tests or Evidence.
Use CI-scripted cargo clippy runs on isolated fixture crates under the actual
lint settings, not prohibited operations inserted into this requirements PR
or production library. Do NOT add trybuild/ui_test or any dev dependency
without owner approval.

| Planned acceptance case | Independent failure/success evidence required | Requirements |
|---|---|---|
| `guard_unwrap_used_is_rejected` | Syntactically valid non-test library unwrap operation fails with unwrap_used, not a grep hit or unrelated compiler error | L3-ROB-007, L3-ROB-009 |
| `guard_expect_used_is_rejected` | Typed expect operation in non-test library code fails with expect_used | L3-ROB-007, L3-ROB-009 |
| `guard_panic_is_rejected` | Explicit panic macro in non-test library code fails with panic lint | L3-ROB-007, L3-ROB-009 |
| `guard_todo_is_rejected` | Non-test library todo macro fails with todo lint | L3-ROB-007, L3-ROB-009 |
| `guard_unimplemented_is_rejected` | Non-test library unimplemented macro fails with unimplemented lint | L3-ROB-007, L3-ROB-009 |
| `guard_unreachable_is_rejected` | Non-test library unreachable macro fails with unreachable lint | L3-ROB-007, L3-ROB-009 |
| `guard_debug_macro_is_rejected` | Non-test library dbg macro fails with dbg_macro lint | L3-ROB-007, L3-ROB-009 |
| `guard_stdout_print_is_rejected` | Non-test library stdout print macro fails with print_stdout lint | L3-ROB-007, L3-ROB-009 |
| `guard_stderr_print_is_rejected` | Non-test library stderr print macro fails with print_stderr lint | L3-ROB-007, L3-ROB-009 |
| `guard_comments_strings_are_not_operations` | Benign comments/string literals containing prohibited spellings do not fail as typed operations; no text scanner substitutes for Clippy | L3-ROB-007 |
| `guard_workspace_table_must_bind_library` | Table exists in virtual root but library fails to inherit it: configuration is not accepted as effective enforcement | L3-ROB-008 |
| `guard_scoped_test_allowance_keeps_production_denied` | Test helper/CLI-main exception is scoped; separately checked non-test library still rejects the same prohibited operation, including in an included submodule | L3-ROB-008, L3-ROB-009 |
| `guard_failure_cannot_be_skipped_or_ignored` | Lint job skipped or command failure swallowed: enforcement obligation remains unsatisfied, not an overall-green pass | L3-ROB-009 |
| `coverage_library_below_floor_fails` | Controlled completed library result below 80% fails the actual gate; retain source/configuration and tool result | L3-ROB-010, L3-ROB-012 |
| `coverage_library_at_floor_still_needs_other_checks` | Completed library measurement meeting 80% satisfies only line floor, not lints, semantic fixtures or owner gates | L3-ROB-010, L3-ROB-012, L3-ROB-013 |
| `coverage_missing_measurement_is_not_success` | Threshold flag exists but measurement/artifact is missing or failed: no library acceptance claimed | L3-ROB-010, L3-ROB-012 |
| `coverage_cli_score_cannot_replace_library` | CLI has high line coverage while library is below 80%: library obligation fails, not averaged into a passing score | L3-ROB-010, L3-ROB-011 |
| `coverage_scope_and_omissions_remain_visible` | Untested library files or conditional/generated code cannot be silently excluded; record source scope, features, exclusions and unmeasured configurations | L3-ROB-011 |
| `coverage_old_head_or_prototype_report_is_not_evidence` | Coverage report belongs to a different SHA/prototype rather than reviewed implementation: no current acceptance; inspect real command/tool/source identity | L3-ROB-011 |
| `coverage_gate_result_is_not_rounded_display` | Below-floor enforcement is checked from actual gate result, not a rounded display or green unrelated job; ignored command failure cannot pass | L3-ROB-012 |
| `guard_coverage_pass_does_not_approve_interpretation` | Lints/line floor pass while a semantic fixture, arbitrary-input property or owner/interpretation gate is unresolved: keep that independent obligation unsatisfied | L3-ROB-013 |

All negative lint fixtures require the intended diagnostic, not just any
nonzero exit. Coverage evidence must name actual library scope/configuration
and completed matching-head result. No new coverage exclusion list, numeric
resource budget, test framework or full ADR-0020 acceptance is selected.
Owner decisions now select std/pattern/extraction/report/access policies;
their development/review/test gates remain independent of these planned checks.

## Planned performance-evidence inspection batch 29

These are planned acceptance cases for L2-PERF-001/002, not executed
benchmarks or new numeric gates. Fixtures need independent expected-work
inventories under reviewed semantics. Heavy synthesized runs belong in
bounded GitHub CI; local/sample work requires the existing authorization
and data/compute plan. No reference tool or recording is accessed here.
Owner Q24 drops Criterion in the restructure; informational ignored CI
benchmarks use no new dependency. Q9 requires zero per-packet steady-state
allocation by the tmats host for
non-setup packets (core L2-HOT-008 analogy); no such threshold is set here.

| Planned acceptance case | Independent evidence or rejection condition | Requirements |
|---|---|---|
| `perf_parser_consumes_expected_source` | Literal source and expected item inventory establish full work before timing acceptance; preserved bytes alone do not hide missing parsed items | L3-PERF-001, L3-PERF-002 |
| `perf_empty_early_result_is_not_fast_parse` | Parser returns early/empty while fixture contains expected items: reject timing as incomplete work, not an optimization success | L3-PERF-001 |
| `perf_parser_result_is_observable` | Timed operation consumes a real result and records actual scope; an optimized-away/no-op path cannot count as parsing | L3-PERF-002 |
| `perf_parser_scope_lists_included_work` | Record acquisition/allocation/index/checksum/view/render inclusion so differently scoped runs cannot silently compare | L3-PERF-002, L3-PERF-004 |
| `perf_recording_scan_includes_later_setups` | Reviewed synthetic recording has later setup records/updates; independently listed inventory must all be accounted for, not only the first packet | L3-PERF-001, L3-PERF-003 |
| `perf_fragments_are_not_records` | Several fragments form one reviewed candidate/record: distinguish fragment/run/record counts with source extents and qualification intact | L3-PERF-001, L3-PERF-003 |
| `perf_interleaved_channels_keep_reviewed_work` | Synthetic per-channel runs/interleaving follow INT-011/045 fixtures; benchmark cannot reduce work by dropping another channel or inventing a completion | L3-PERF-001, L3-PERF-003 |
| `perf_core_is_the_only_packet_reader` | Recording benchmark uses core facts/sideband/DATA and TMATS slicing; local sync scanner/checksum fallback or a second traversal oracle is not accepted | L3-PERF-003 |
| `perf_recording_and_text_byte_denominators_differ` | Large recording with smaller setup text reports actual processed byte domains; parser text rate is not labelled full-file scan throughput | L3-PERF-004 |
| `perf_borrowed_assembly_mode_is_identified` | Recording traversal uses ADR-0036 borrowed spans without coalescing/extra payload copies; any owned parse-source construction is a separate parser workload. Paired untimed allocator/copy instrumentation reports allocation counts and payload-copy bytes, with counter scope and no new numeric threshold | L3-PERF-003, L3-PERF-004 |
| `perf_coalescing_or_copying_scan_is_not_accepted` | A run coalesces/copies setup payloads instead of ADR-0036 borrowed-span assembly: reject it as recording-scan evidence even if fast/correct; untimed test-allocator and explicit copy counters remain reported, without a new allocation threshold | L3-PERF-003, L3-PERF-004 |
| `perf_reproducible_input_build_identity` | Record source SHA, input fingerprint/size, compiler/tool versions, profile/Cargo features, compile-time target features, hardware/OS/architecture and run settings | L3-PERF-005 |
| `perf_unlike_cache_or_acquisition_modes_are_labelled` | Warm in-memory run versus file acquisition/cold mode cannot masquerade as equal-scope timing; actual modes retained | L3-PERF-004, L3-PERF-005 |
| `perf_reference_uses_same_file` | Input fingerprints and reference version/command show same-file comparison; another input or unreported scope cannot yield an equivalent-work speedup claim | L3-PERF-005, L3-PERF-006 |
| `perf_reference_first_packet_only_is_partial` | idmptmat consumes first packet while tmats traverses later setups: record partial/non-equivalent work, never unqualified whole-recording ratio | L3-PERF-006 |
| `perf_reference_standalone_rejection_is_not_timing_success` | Reference cannot accept standalone TMATS: unsupported outcome stays visible, not a successful zero-time parse | L3-PERF-006 |
| `perf_reference_timeout_is_not_speedup` | Reference nontermination hits the explicit bound: retain timeout/extent and stop owned process, no fabricated completed rate | L3-PERF-006, L3-PERF-007 |
| `perf_reference_semantic_disagreement_is_investigated` | Independent inventory disagrees: investigate cited rules, retain mismatch; reference output alone cannot settle correctness | L3-PERF-001, L3-PERF-006 |
| `perf_run_limits_and_cleanup_are_explicit` | Future plan bounds input/time/memory/repetitions and stops only its owned jobs on completion/limit; missing plan is not authorization for background runs | L3-PERF-007 |
| `perf_missing_sample_evidence_remains_missing` | Synthetic CI exists but authorized sample/reference work unavailable: sample obligation remains unverified; do not upload local-use data or silently run heavy local work | L3-PERF-005, L3-PERF-007 |
| `perf_initial_measurement_is_not_guessed_budget` | First measured baseline is retained for later explicit budget review; historical 100 Mbps context is not promoted into an accepted pass/fail threshold | L3-PERF-008 |
| `perf_speed_does_not_waive_semantics_or_owner_holds` | Fast run with missing source/candidates, unapproved interpretation or unresolved qualified-record policy cannot count as full acceptance | L3-PERF-001, L3-PERF-008 |

Future Evidence names completed, matching-source runs and their actual
workload/limitations. These rows select no public report format, timing
threshold, benchmark dependency, reference-tool patch or numeric timeout.
Prototype benchmark comments are not requirement verification markers for
this new contract, and timing does not approve proposed ADR-0021.

## Planned X-extension association batch 30

Source: 106-24R1 Ch9 section 9.5.14 p. 9-172; original D-x\MN-y-n is
Table 9-7 MEASUREMENT NAME p. 9-115, D-x\MNF\N-y-n-m is NUMBER OF
FRAGMENTS p. 9-117. G\PN is Table 9-2 PROGRAM NAME p. 9-10. These
fixtures test structural attachment, not whole-file conformance or private
extension-value semantics. A/B below are independent source item identities.
Record qualification/access restrictions remain intact. Joey signed INT-061/062
A on 2026-11-28. Selected A expectations are now owner-decided; formerly
alternative inputs below assert A's unavailability/refusal instead. Remaining
independent final review, development analysis/review and tests still gate code. Indexed candidate tests
can receive explicit reviewed decomposition metadata; they do not sign off
runtime vocabulary selection. Under owner-signed registry-vocabulary A,
registry-name matching plus EXTENSION_CODE uniqueness determines the split,
not a free-form greedy parser or target presence in the document.

| Planned test | Independent source/binding and expected evidence | Requirements |
|---|---|---|
| `x_section_example_attaches_to_original` | A=`D-1\MN-1-2:MEAS1;`, X=`X-1\MYORG\D\MN\SGAIN-1-2:10.75;`; resolved to A, raw bytes/locations retained; no Sensor Gain value decoder asserted | L3-EXT-001, L3-EXT-002, L3-EXT-004 |
| `x_extra_indices_remain_extension_private` | Same A, X ends SGAIN-1-2-7-8: first [1,2] binds A; [7,8] stays private, not another original index/counter declaration | L3-EXT-002, L3-EXT-006 |
| `x_nested_origcode_uses_full_path` | A=`D-1\MNF\N-1-1-1:2;`, X=`X-1\ORG\D\MNF\N\NOTE-1-1-1-8:v;`; reviewed three-index arity/path binds A, final 8 remains private | L3-EXT-001, L3-EXT-002 |
| `x_wrong_group_or_path_never_matches_value` | Another code/path holds MEAS1 or looks like a prefix: it is not a target solely through value/prefix similarity | L3-EXT-002, L3-EXT-004 |
| `x_occurrence_mismatch_is_not_repaired` | Only original D-2 exists, X occurrence is 1: available matching gives unresolved, no substitution of nearest/only occurrence | L3-EXT-002, L3-EXT-004 |
| `x_carried_index_order_is_exact` | A indices [1,2], X leading [2,1]: no match; a matching subsequence later in private indices cannot repair it | L3-EXT-002 |
| `x_too_few_carried_indices_is_unavailable` | Original arity two is reviewed, X supplies one usable carried index: cannot-evaluate malformed binding, not a fabricated second index | L3-EXT-001, L3-EXT-002 |
| `x_canonical_indices_keep_raw_findings` | A=`D-01\MN-01-02:m;`, X=`X-1\MYORG\D\MN\SGAIN-1-2:v;`: canonical positions agree, raw spellings/INT-038 findings remain; X/D are different full keys | L3-EXT-003 |
| `x_long_index_has_no_field_digit_limit` | Reviewed indexed name contains a 40-digit identity; compare canonical strings without integer conversion or INT-055's field-value ten-digit rule | L3-EXT-003 |
| `x_ascii_case_is_not_unicode_folding` | Lowercase original-path spelling compares under ASCII case rules; a high-byte look-alike never folds into a matching ASCII original path | L3-EXT-003 |
| `x_name_projection_retains_omission_finding` | A real internal control omitted under INT-036 leaves canonical matching possible only through approved projection; original source and located omission remain | L3-EXT-003, L3-EXT-005 |
| `x_orphan_is_not_new_original_or_counter_member` | Reviewed X targets an absent original: unresolved, all bytes retained, no D member/counter entry created (INT-040) | L3-EXT-004, L3-EXT-006 |
| `x_duplicate_originals_are_all_candidates` | A and B are repeated original scalar keys, even with equal values: ambiguous [A,B] in source order, no winner, record INT-048 Ambiguous attached/unchanged | L3-EXT-004, L3-EXT-005 |
| `x_unknown_extension_value_does_not_erase_attachment` | Unique matching original exists, private value unknown/contains high bytes: structural attachment retains raw evidence/field findings, never interprets it as a clock/measurement rule | L3-EXT-004, L3-EXT-005, L3-EXT-006 |
| `x_noncontiguous_occurrences_are_not_created` | Originals/related X occurrences use 1 and 3: section 9.5.14's X occurrence rule does not demand 2 or synthesize original 2; other index rules stay separate | L3-EXT-006 |
| `x_unknown_arity_never_uses_greedy_prefix` | Reviewed registry code-name/arity metadata missing or split uniqueness unproved: cannot-evaluate with retained raw items, not guessed longest-prefix match or false absence; the standard's registry-match decomposition rule is not treated as absent | L3-EXT-001, L3-EXT-004 |
| `x_separate_record_cannot_supply_original` | Target only exists in another setup record/document: query never borrows it or chooses a governing record | L3-EXT-005 |
| `x_cross_fragment_target_shares_one_document` | Same-channel sequence +1 fragments with matching FRMT/RCCVER form ONE ADR-0025/INT-011 record: original and X across fragments retain target IDs and packet/body provenance | L3-EXT-005 |
| `x_attachment_preserves_cut_and_ambiguous_qualification` | Diagnostic source has Ambiguous and/or retained cut/Incomplete facets: any returned attachment keeps them and their blocking force, without approving access/adoption | L3-EXT-005 |
| `x_leading_marker_blocks_semantic_attachment` | Assembled text begins EF BB BF: INT-037 availability gate remains; keep raw evidence and reason, not resolved/empty semantic success | L3-EXT-001, L3-EXT-005 |
| `x_unindexed_original_mapping_unavailable` | **Owner-signed INT-061 A:** raw X appears to target unindexed G\PN; retain plausible raw evidence and cannot-evaluate mapping, not resolved or proven no-target | L3-EXT-007 |
| `x_unindexed_zero_one_never_default` | **Owner-signed INT-061 A:** hypothetical X-0/X-1 forms for G\PN cannot introduce an implicit sentinel/default occurrence or source rewrite | L3-EXT-007 |
| `x_unindexed_value_never_supplies_occurrence` | **Owner-signed INT-061 A:** numeric-looking G\PN value/order supplies no missing source index; no association invented from its data | L3-EXT-007 |
| `x_unindexed_unique_path_option_c` | **A selected; C not active:** exactly one G\PN still leaves occurrence mapping cannot-evaluate with raw evidence; unique path does not authorize C attachment or an invented sentinel | L3-EXT-007 |
| `x_unindexed_duplicate_path_option_c` | **A selected; C not active:** duplicate G\PN stays field Ambiguous/all listed; X mapping cannot-evaluate with qualification retained; no duplicate winner | L3-EXT-005, L3-EXT-007 |
| `x_unindexed_nonconforming_option_d` | **A selected; D not active:** preserve X and cannot-evaluate occurrence mapping; do not impose D's blanket malformed/nonconforming verdict merely because G\PN has no index | L3-EXT-007 |
| `x_registry_decomposition_survives_absent_target` | **Owner-signed INT-062 A:** reviewed 106-24R1 baseline registry contains D\MN pattern/arity, but no matching original item is present; registry split remains defined, then instance lookup reports unresolved/orphan, not absent grammar | L3-EXT-001, L3-EXT-004 |
| `x_document_only_code_does_not_define_registry_split` | **Owner-signed INT-062 A:** a document-only/private code is present but absent from reviewed registry metadata; presence cannot create a registry code definition or heuristic split; retain raw/cannot-evaluate evidence | L3-EXT-001 |
| `x_extension_overlap_prevents_heuristic_split` | **Owner-signed INT-062 A:** fixture metadata establishes EXTENSION_CODE overlap with an existing registry code name; retain violation/raw evidence and cannot-evaluate split, do not pick a longest prefix or rename source | L3-EXT-001, L3-EXT-006 |
| `x_private_target_registry_only_cost` | **Owner-signed INT-062 A:** private V original is present but outside standard registry vocabulary (section 9.5.13); X remains cannot-evaluate. Standard V ID/VN do not grant private-name grammar | L3-EXT-001 |
| `x_baseline_registry_keeps_edition_basis` | **Owner-signed INT-062 A:** supplied declaration/override/fallback evidence and INT-044 reason stay unchanged; identify registry as 106-24R1 until deltas exist, not a fake older catalog | L3-EXT-001, L3-EXT-005 |
| `x_legacy_renamed_original_is_unavailable` | **Owner-signed INT-062 A:** coordinator's legacy R\DST/R\CDT example has no reviewed baseline metadata for the legacy original: cannot-evaluate, raw bytes kept, no automatic translation; no historical rename date asserted | L3-EXT-001 |
| `x_hybrid_no_vocabulary_uses_labelled_document_names` | **A selected; hybrid C not active:** document-only original in a group lacking registry vocabulary remains cannot-evaluate; never emit document-defined vocabulary as an adopted basis | L3-EXT-001 |
| `x_hybrid_existing_vocabulary_never_falls_back` | **A selected; hybrid C not active:** reviewed registry vocabulary exists but requested code is missing: cannot-evaluate, no document-name fallback | L3-EXT-001 |
| `x_hybrid_v_private_boundary_needs_owner` | **A selected; historical test name retained:** standard V ID/VN vocabulary never authorizes private names as decomposition metadata; private original remains cannot-evaluate even when present | L3-EXT-001 |
| `x_association_oracle_is_not_resolver_output` | Independently listed target IDs/path/arity/spans determine expected outcomes; production resolver/round trip alone is not the oracle; all bytes and qualifiers unchanged | L3-EXT-008 |

No public query syntax, concrete result/error names, report layout, private
value/counter semantics, namespace-uniqueness validator or edit/renumber API
is selected here. V-group derivations remain separate; a partial X child
does not complete L1-EXT-003. Future metadata coverage and executable tests
must satisfy the claimed 0.1 support rather than silently reduce its scope.

## Planned V-group context batch 31

Source: 106-24R1 Ch9 section 9.5.13 p. 9-171 (V ID/VN/private form) and
Table 9-2 DATA SOURCE ID p. 9-11. These are structural/context fixtures,
not whole-file conformance or private-value decoders. Header composition
can receive evaluated field states under reviewed metadata; it does not
settle unreviewed blanks/required-presence rules. Eight INT-063 A/definition expectations are owner-decided (Joey,
2026-11-28); final rule check CONCURS at #87 e0a0cce; development/test gates apply.

| Planned test | Independent source/context and expected evidence | Requirements |
|---|---|---|
| `v_standard_headers_are_not_private_suffixes` | V-1\ID and V-1\VN are standard headers; V-1\ABC\ID is a private suffix and cannot replace the local ID header | L3-EXT-009, L3-EXT-010 |
| `v_private_items_keep_observed_namespace` | Readable V-1\ABC\GAIN with opaque value stays in observed ABC inventory, with original item/span/value; no gain decoder claimed | L3-EXT-009, L3-EXT-015 |
| `v_private_nested_digits_do_not_create_counters` | Private suffix contains paths, N or -1-2; retain opaque bytes without a standard counter/member/index grammar | L3-EXT-009, L3-EXT-015 |
| `v_context_with_agreeing_evaluated_headers` | Supplied usable ID=SRC and VN=ABC agree with observed ABC namespace; retain explicit header/source context. Any vendor-confirmation policy remains conditional on INT-063 | L3-EXT-010, L3-EXT-014 |
| `v_header_only_occurrence_is_retained` | ID/VN headers exist with zero private items: retain occurrence context and empty private collection, not no V occurrence or a fabricated payload | L3-EXT-010 |
| `v_canonical_occurrence_keeps_raw_findings` | V-01 and V-1 share canonical occurrence identity; retain raw spellings/normalization findings and repeated header candidates rather than deduplicate | L3-EXT-010, L3-EXT-011 |
| `v_ascii_acronym_comparison_preserves_raw` | ABC/abc literal/evaluated identifiers compare by ASCII case rule; high-byte look-alike never folds into ASCII; raw source remains | L3-EXT-011 |
| `v_textual_source_ids_are_not_numeric_indices` | Usable source label 01 does not equal G source label 1; INT-038/055 cannot strip its zeros as a decimal value | L3-EXT-011, L3-EXT-012 |
| `v_source_link_uses_text_not_occurrence` | V occurrence 2 has usable ID=SRC, G\DSI-1 is SRC: matching textual source context can resolve; x need not equal n | L3-EXT-012 |
| `v_missing_g_source_keeps_local_label` | Usable local ID exists but no equal G source candidate: keep label and unresolved source link distinct, no synthesized G item | L3-EXT-012 |
| `v_duplicate_g_sources_keep_all_candidates` | Multiple G\DSI-n occurrences have same textual source identity: keep ambiguous candidates/all source IDs in order, no winner or record upgrade | L3-EXT-012, L3-EXT-013 |
| `v_ambiguous_local_id_never_selects_source` | Reviewed local ID state is Ambiguous, even with equal candidate text: preserve state/all evidence; no source chosen by equality | L3-EXT-010, L3-EXT-012 |
| `v_other_occurrence_cannot_supply_headers` | V-2 has headers missing on V-1: retain V-1 missing context; do not borrow V-2's vendor/source even if a private path matches | L3-EXT-010, L3-EXT-013 |
| `v_separate_record_cannot_supply_context` | Context exists only in another setup document/record: no cross-record source/vendor binding | L3-EXT-013 |
| `v_cross_fragment_context_has_packet_provenance` | Same channel, sequence +1 and matching FRMT/RCCVER give ONE ADR-0025/INT-011 record; headers/private items span packets, retaining item/body/packet mappings | L3-EXT-013 |
| `v_record_qualifications_never_upgrade` | Inventory/source candidates remain available diagnostically only under reviewed access; Ambiguous/Incomplete/cut/encoding qualifications and blocking force stay attached | L3-EXT-013 |
| `v_missing_vn_keeps_namespace_unconfirmed` | **Owner-signed INT-063 A:** usable path namespace but VN Missing: preserve items/context, unavailable confirmed attribution, no default or invented required-presence finding | L3-EXT-010, L3-EXT-014 |
| `v_duplicate_vn_never_picks_vendor` | **Owner-signed INT-063 A:** repeated known-scalar VN, even equal: keep Ambiguous/all headers; no first/last/path-selected confirmed vendor | L3-EXT-010, L3-EXT-014 |
| `v_path_header_mismatch_keeps_both` | **Owner-signed INT-063 A:** evaluated VN=ABC and private path=XYZ: preserve both, unavailable confirmed attribution with located reason, no raw reassignment/repair | L3-EXT-014 |
| `v_invalid_unavailable_vn_keeps_namespace` | **Owner-signed INT-063 A:** evaluated Invalid or unavailable VN does not hide observed items or become an inferred vendor; preserve distinct state/reason | L3-EXT-010, L3-EXT-014 |
| `v_unknown_private_duplicates_stay_unclassified` | Repeated opaque private name retains every raw occurrence; unknown private cardinality does not create a proved scalar duplicate merely from shape | L3-EXT-010, L3-EXT-015 |
| `v_vn_projected_three_chars_ascii_match` | **Owner-signed INT-063 A:** VN a<TAB>Bc (real TAB) projects under INT-036 to aBc, exactly three ASCII characters; compares to ABC path without rewriting raw value; existing diagnostics remain | L3-EXT-011, L3-EXT-014 |
| `v_vn_wrong_length_is_invalid_unconfirmed` | **Owner-signed INT-063 A:** available projected empty, two- or four-character VN is Invalid (not Missing/generic unavailable), so confirmed attribution is unconfirmed; raw bytes retained | L3-EXT-011, L3-EXT-014 |
| `v_vn_blank_handling_does_not_trim` | **Owner-signed INT-063 A:** three-character value space+AB has leading blank; INT-041 unreviewed blank handling remains unavailable, never trim it or invent a numeric/strict alphabet rule | L3-EXT-011, L3-EXT-014 |
| `v_vn_high_byte_is_not_usable` | **Owner-signed INT-063 A:** raw three-byte VN containing FF is Invalid under INT-037, not a usable acronym through Latin-1/Unicode/high-byte folding; unconfirmed and raw evidence unchanged | L3-EXT-011, L3-EXT-014 |
| `v_grouping_oracle_is_not_implementation_output` | Independently listed IDs/header states/source candidates and literal spans determine expected inventory; no production grouping path calculates the oracle | L3-EXT-016 |

No vendor decoder/plug-in, generic user overlay, acronym alphabet, required-
presence rule, result schema, exact rendered label or repair policy is chosen.
Observed raw grouping is not confirmed interpretation, clock proof or a
waiver of INT-061/062/063 and the other owner/implementation gates.

## Planned dependency/artifact inspection batch 32

These are planned evidence/acceptance cases for L2-REL-005/006, not executed
policy checks, generated SBOMs, verified attestations or permission to release.
Inputs can supply synthetic graph/artifact facts and independently expected
failure reasons. The intended applicable check must fail; an unrelated tool
error is not proof that policy/artifact enforcement worked.

| Planned acceptance case | Independent evidence or rejection condition | Requirements |
|---|---|---|
| `dep_actual_graph_not_manifest_strings` | Resolved library/CLI graph, features/targets, lock/source identity and policy revision match the reviewed head; a direct manifest list alone is insufficient | L3-REL-009 |
| `dep_transitive_or_feature_edge_not_omitted` | Finding lies in a checked transitive/optional graph edge: actual policy result retained; another feature graph cannot silently hide it | L3-REL-009, L3-REL-010 |
| `dep_advisory_source_context_is_recorded` | Completed command/tool result identifies advisory source/snapshot/time or its unavailability; no invented freshness window or clean status from missing data | L3-REL-009 |
| `dep_denied_finding_fails_acceptance` | Applicable non-exempt denial is present: failed acceptance with intended reason, not a swallowed command failure or unrelated-network-error pass | L3-REL-010 |
| `dep_duplicate_warning_is_not_new_ban` | Existing multiple-version warning remains distinguishable from denial; this contract does not silently tighten policy | L3-REL-010 |
| `dep_internal_path_is_not_external_exception` | Accepted exact-version workspace path does not become an unlisted external path/git/registry-source allowance | L3-REL-010 |
| `dep_exception_reason_is_scoped` | Listed exception is matched to identifier/dependency/context/reason; unrelated replacement dependency cannot inherit its prototype rationale | L3-REL-011 |
| `dep_prototype_xml_ignore_retirement_is_checked` | Prototype XML code is removed in future restructure: review/removal of its prototype-only advisory ignores accompanies that change, not blanket retention | L3-REL-011 |
| `dep_past_pass_does_not_approve_new_dependency` | Old pass or existing Criterion entry cannot approve retention: owner Q24 drops Criterion; new dependencies still need approval | L3-REL-009, L3-REL-011 |
| `artifact_binary_digest_matches_released_bytes` | Actual downloaded binary bytes/digest match its intended source/version/target and checksum/provenance association | L3-REL-012, L3-REL-014 |
| `artifact_same_filename_stale_digest_is_rejected` | Rebuilt/different bytes retain filename but stale evidence: acceptance fails rather than trusting the name | L3-REL-012 |
| `artifact_sbom_is_per_binary_build_context` | Matching target/features/resolved build identity associates CycloneDX SBOM with intended binary; library-only requirements list is insufficient | L3-REL-013 |
| `artifact_other_target_sbom_is_not_substitute` | Windows or another-version graph supplied for intended Linux build: mismatch/missing evidence remains unsatisfied, no cross-target substitution | L3-REL-012, L3-REL-013 |
| `artifact_missing_sbom_is_not_green_build_success` | Binary exists and workflow is green, required per-binary SBOM absent: L1 release-artifact obligation remains unsatisfied | L3-REL-013 |
| `artifact_attestation_is_for_intended_file` | Builder provenance verification covers intended .crate/binary/SBOM subject bytes and source/build identity, not another digest or signing-step name; manifest/bundle validation does not imply their individual attestation | L3-REL-014 |
| `artifact_wrong_source_or_changed_payload_fails` | Verification/evidence gives another repository/head or modified payload: preserve mismatch/failure, not accepted build provenance | L3-REL-014, L3-REL-015 |
| `artifact_user_recipe_has_applicable_result` | Documented verification path and actual completed matching-file result are available; docs/upload alone cannot claim verification | L3-REL-015 |
| `artifact_missing_or_failed_verification_stays_unsatisfied` | Required artifact/verification missing, failed or not run: no inference from release notes or unrelated green job | L3-REL-014, L3-REL-015 |
| `artifact_distribution_does_not_gain_publish_authority` | Unified trusted-publishing.yml uses a non-publishing self-attesting builder and crates-io-gated publish job; artifact acceptance cannot add a stored credential, token fallback or maintainer bootstrap | L3-REL-016 |
| `artifact_signed_build_is_not_tmats_integrity` | SBOM/provenance verifies build evidence only; no positive G\SHA, trusted recorder data, semantic coverage or qualified-record-use permission follows | L3-REL-016 |
| `dep_release_uses_published_types_core` | Release graph resolves compatible types 0.2/core 0.2 APIs from crates.io and exercises them; reject unpublished/path/git/[patch]/source overrides, including a CI-only override; internal packaged CLI resolves its matching published library | L3-REL-009, L3-REL-010 |
| `artifact_unified_release_set_is_complete` | Both .crate files, all binary archives, SBOMs, exactly one SHA256SUMS and one irig106-tmats-<version>.intoto.jsonl bundle; missing/extra manifest or bundle fails | L3-REL-012, L3-REL-016 |
| `artifact_builder_attests_only_own_output` | Nested signer independently guards v* tag push, checks github.sha/verified output and rejects caller-selected inputs; PR/manual runs never sign; no top-level attest job | L3-REL-014 |
| `artifact_publish_repackage_cmp_before_upload` | Non-compiling publish needs gate/build, validates checksums and cmp's cargo package --locked --no-verify output with attested .crate; changed bytes block upload | L3-REL-016 |
| `artifact_served_crate_bytes_and_registry_checksum` | New and already-published packages must match attested bytes and registry checksum; retry never skips comparisons; a mismatch fails | L3-REL-015, L3-REL-016 |
| `artifact_draft_resume_checks_metadata_and_assets` | Wrong tag/title/body/draft state or asset name/size/digest blocks publication; check again after publish, never replace an asset or modify published release | L3-REL-016 |
| `artifact_verifier_checks_served_release` | Verify actual downloaded release assets with repository/tag/builder signer constraints, not only staging artifacts; tampering fails | L3-REL-015 |

No workflow/policy/credential/settings action is performed by these cases.
Existing deny.toml and RELEASING.md govern, including coordinator/owner-only
bootstrap and TP-only real releases. New requirements remain Draft until
matching graph/distribution evidence is reviewed; no new Evidence is claimed.

## Planned release follow-up checks carried from #65

These extend the batch-32 acceptance specification, not executed release
checks. The org standard governs any difference from the local summary.

| Planned acceptance case | Independent evidence or rejection condition | Requirements |
|---|---|---|
| `artifact_gate_uses_remote_annotated_tag` | Remote API shows annotated tag peeling to github.sha on main with intended CI green at that commit; lightweight/local-only/wrong-commit tag or unrelated CI fails | L3-REL-012 |
| `artifact_sbom_generator_is_pinned_locked` | Exact cargo-cyclonedx pin and --locked installation match recorded evidence; floating tool selection is rejected | L3-REL-013 |
| `artifact_publish_reuses_build_toolchain` | publish and publish-path-check install/check build's recorded rust-version; a newer stable resolved after approval is rejected | L3-REL-013 |

## Planned edition-independent reading batch 33

These are synthetic structural/evidence cases for L1-EDN-001, not a claim of
historical/future conformance or an implemented reader. Source framing uses
106-24R1 Ch9 sections 9.4.1/9.4.2 p. 9-2 and approved INT-035–039.
The metadata matrix supplies missing/known/unknown/older/future labels to the
same byte input without inventing numeric historical mappings. Internal
states/spans are asserted; owner Q26/ADR-0039 now supplies the report policy, with renderer/schema work still unimplemented.

| Planned test | Independent input and expected result | Requirements |
|---|---|---|
| `edition_read_without_declaration` | Synthetic `COMMENT:x;`: one framed item, source/item [0,10), name [0,7), value [8,9); no G\106 prerequisite | L3-EDN-008 |
| `edition_read_metadata_does_not_reframe` | Same bytes above with each supplied metadata label: identical source, spans, order and scanner findings; semantic findings are not equated | L3-EDN-008, L3-EDN-013 |
| `edition_read_unknown_g106_kept_raw` | `G\106:zz;COMMENT:x;`: raw values and exact replay retained; no guessed year or mapping; declaration interpretation tested separately | L3-EDN-008, L3-EDN-009 |
| `edition_read_changed_declaration_has_own_offsets` | `G\106:24;COMMENT:x;` versus `G\106:unknown;COMMENT:x;`: independent item spans [0,9)/[9,19) versus [0,14)/[14,24); no assertion that byte-changing edits keep offsets | L3-EDN-008, L3-EDN-013 |
| `edition_read_unregistered_name_is_retained` | Synthetic `UNREGISTERED_TEST:x;`: registry does not know this name; raw item [0,20), name [0,17), value [18,19); unknown definition never drops bytes | L3-EDN-009 |
| `edition_read_unregistered_order_and_repeats` | Synthetic unregistered items interleaved with COMMENT: all occurrences in source order, replay exact; unknown multiplicity never guessed scalar | L3-EDN-009, L3-EDN-010 |
| `edition_read_no_historical_alias` | Supplied unavailable definition: raw key/value retained; no renamed-code alias or baseline range/default inferred | L3-EDN-009, L3-EDN-011 |
| `edition_read_recovery_independent_of_basis` | `:v;COMMENT:x;tail` under each metadata label: empty-name finding, intact COMMENT and retained suffix; no suffix explicit lookup item | L3-EDN-010 |
| `edition_read_projection_does_not_rewrite_source` | `CO\r\nMMENT:x;` with real CR/LF under every metadata label: INT-036 projection and internal-omission information, raw bytes unchanged | L3-EDN-010 |
| `edition_read_high_byte_never_disappears` | COMMENT value contains 0xFF: INT-037 interpretation limits retained with exact raw bytes; metadata does not transcode/repair | L3-EDN-009, L3-EDN-010 |
| `edition_read_duplicate_uses_reviewed_metadata` | Known scalar definition supplied twice: INT-039 retains all occurrences; absent multiplicity metadata cannot invent historical scalar behavior | L3-EDN-010 |
| `edition_read_semantic_unavailability_separate` | Readable structural item with no applicable definition: raw candidates retained, no usable typed meaning invented or conformance claim | L3-EDN-011 |
| `edition_read_fallback_is_not_historical_validation` | Given an already evaluated labelled basis, raw reading does not relabel it as the declared/validated historical edition; no INT-017 mapping is exercised | L3-EDN-011 |
| `edition_read_preserves_record_qualification` | Supply Complete, Incomplete, Ambiguous and Ambiguous-with-cut qualifications separately: each remains attached unchanged; no governing authorization | L3-EDN-012 |
| `edition_read_documents_never_merge` | Two separately identified inputs retain their own source/revision/qualification; equal labels or keys never concatenate their attribute sets | L3-EDN-012 |
| `edition_read_xml_gate_remains` | Supplied XML format is unsupported through the setup adapter, not silently routed as ASCII because the declaration is unknown | L3-EDN-012 |
| `edition_read_limit_failure_not_empty_success` | Same bytes/context with an exhausted supplied budget: existing typed limit contract remains observable; no empty successful document | L3-EDN-008, L3-EDN-013 |
| `edition_read_oracle_is_independent` | Fixture oracle lists exact input bytes/spans and metadata variants independently; a production-scanner-derived oracle is rejected as evidence | L3-EDN-013 |

No public report format, historical code semantics, default, registry delta
or owner interpretation is approved by these cases. Future implementation
must replace planned rows with executable tests and trace markers.

## Planned complete-record enumeration batch 34

These planned tests assert internal command computation for L1-CLI-003,
not report layouts, public record ordinals or command eligibility. Inputs
supply independently evaluated final assembly results and core source facts;
assembly-specific integration cases retain their existing interpretation
gates. No synthetic packet is claimed conforming merely by its channel.
Source: 106-24R1 Ch11 section 11.2.7.2 p. 11-50; ADR-0025/0036.

| Planned test | Independent input and expected result | Requirements |
|---|---|---|
| `records_all_complete_not_first_only` | Complete candidates A/B/C at first-fragment offsets 100/500/900, with intervening data observations: entries A/B/C exactly once | L3-CLI-038, L3-CLI-040 |
| `records_one_fragmented_document_one_entry` | A consists of three fragments with explicit ordered membership: one A entry retaining all spans, not three packet entries | L3-CLI-039 |
| `records_fragmented_and_single_both_retained` | Multi-fragment A followed by single-fragment B: two entries with separate body extents and provenance | L3-CLI-038, L3-CLI-039 |
| `records_identical_separate_copies_not_deduplicated` | Distinct bounded A/B have identical text and digest: both remain; equality is information, no governing/eligibility upgrade | L3-CLI-039, L3-CLI-043 |
| `records_order_uses_first_fragment_not_finalization` | Supplied Complete A begins at 100 and ends at 800; independent B begins at 300 and ends at 400: internal order A/B, despite completion order B/A | L3-CLI-040 |
| `records_not_sorted_by_channel_or_timestamp` | Supplied source order A/B disagrees with both channel and timestamp order: retain A/B and their original provenance | L3-CLI-040 |
| `records_complete_list_does_not_absorb_noncomplete` | A Complete, B Incomplete, C Ambiguous, D provisional: Complete entries contain A; B/C/D evidence and distinct states remain, no text borrowed into A | L3-CLI-041 |
| `records_cut_facet_retains_blocking_force` | Ambiguous-with-cut B retains cut and all findings; no Complete entry or usable/governing setup manufactured | L3-CLI-041, L3-CLI-043 |
| `records_revised_time_seal_uses_final_outcome` | A formerly observed as Complete now supplied finally Ambiguous after continuation: no stale Complete A entry; revision history and premature-seal finding retained | L3-CLI-041 |
| `records_unknown_or_xml_format_not_reparsed` | Supplied unsupported-format candidate remains unsupported with source facts; no ASCII reparse or format guessing | L3-CLI-041 |
| `records_no_complete_is_not_empty_document` | Successfully traversed extent contains only evaluated non-Complete candidates: no Complete entries, evidence retained, no synthesized empty document | L3-CLI-038, L3-CLI-041 |
| `records_failure_is_not_clean_empty_scan` | Acquisition/traversal fails before any Complete record: failure distinct from successful zero-result inventory | L3-CLI-038, L3-CLI-042 |
| `records_late_failure_does_not_claim_full_inventory` | Complete A observed before supplied core failure at 700 in a larger extent: A identity retained with traversal failure/extent, no claim of absent later records or publication policy | L3-CLI-042 |
| `records_limit_never_silently_drops_channel_run` | Supplied typed aggregate-run limit error: preserved with context, no retry as text, no silently forgotten run or invented success | L3-CLI-042 |
| `records_selection_does_not_renumber_internal_identity` | Same A/B set passed to two command consumers with explicit A versus B context: selected identity changes, underlying inventory/provenance does not | L3-CLI-043 |
| `records_separate_documents_do_not_create_duplicate` | A and B each contain one scalar field: enumeration retains two scoped documents, never combines their fields into an INT-039 duplicate group | L3-CLI-039 |
| `records_complete_is_not_governing_or_integrity_pass` | Evaluated Complete A with integrity findings/unresolved governing effect: those facts stay attached; no checksum match or adoption inferred | L3-CLI-041, L3-CLI-043 |
| `records_oracle_uses_independent_candidate_sets` | Expected set/order/membership listed independently from supplied facts; production enumeration or a local replacement packet reader is not an oracle | L3-CLI-044 |

Owner-signed INT-028/031 governs access and Q14 fixes first-fragment order;
report rendering follows owner Q26/ADR-0039; other unselected public forms need reviewed request tables. No executable tests or packet reader are
added by these specifications.

## Planned checksum command computation batch 35

Internal computation only, given explicitly selected eligible records.
Incomplete/ambiguous access follows owner-signed INT-031; refused-result
propagation tests can inject evaluated outcomes rather than access such text.
No output schema, selection default or numeric exit code is fixed here.
Sources: existing L3-SUM-001–010; INT-018–021/043; ADR-0014/0029.

| Planned test | Independent input and expected result | Requirements |
|---|---|---|
| `checksum_command_keeps_selected_record` | Different eligible A/B sources: explicitly select B; computation and findings refer only to B, not first A or joined A+B | L3-CLI-045, L3-CLI-051 |
| `checksum_command_rejects_stale_context_substitution` | Distinct revisions of the same document: supplied revision remains attached; no cached earlier digest presented for it | L3-CLI-045 |
| `checksum_command_recording_domain_is_text_only` | Eligible assembled text from borrowed spans: independent SHA oracle excludes headers/CSDWs/filler/packet checksum by using the specified text domain | L3-CLI-045, L3-CLI-051 |
| `checksum_command_malformed_value_compute_allowed` | Complete G\SHA:2-ABC; with defined exclusion: compute allowed and matches independent hashlib oracle, verify malformed; no command-level conflation | L3-CLI-046, L3-CLI-047 |
| `checksum_command_placeholder_is_not_verification` | Complete G\SHA:0;: compute allowed under established domain, verification stays placeholder/unverified, not match | L3-CLI-046, L3-CLI-047 |
| `checksum_command_undefined_domain_stays_refused` | Inject missing-terminator/truncation/damage/BOM refusal: no authoritative result or silently substituted raw hash | L3-CLI-046 |
| `checksum_command_duplicate_refusal_keeps_all_findings` | Inject F1 duplicate outcome with occurrence locations: indeterminate/refusal and every occurrence preserved, no selected winner or positive match | L3-CLI-046, L3-CLI-047 |
| `verify_command_preserves_outcome_partition` | Independently supplied match/mismatch/absent/unknown/malformed/indeterminate/unavailable/placeholder results remain distinct with their source context | L3-CLI-047 |
| `verify_command_noncanonical_2_keeps_warning` | Eligible 02-/002- algorithm-2 cases with independently calculated matching digest: match plus located warning, raw spelling unchanged | L3-CLI-047 |
| `verify_command_blocked_keeps_noncanonical_warning` | Inject blocked result plus located noncanonical-2 warning: both remain visible internally; no fabricated verified result | L3-CLI-047 |
| `checksum_command_flex_is_explicit_and_separate` | Explicit flex request/options remain attached to non-standard result; standard request cannot be satisfied by flex | L3-CLI-048 |
| `checksum_command_flex_never_stamps_gsha` | Flex result passed through computation leaves input and G\SHA unchanged; no standard-verification claim | L3-CLI-048, L3-CLI-049 |
| `stamp_command_duplicate_is_not_auto_repair` | Inject duplicate/refused transaction: no successful stamped result, input/findings/occurrences retained, no implicit remove | L3-CLI-049 |
| `stamp_command_explicit_exact_item_repair_then_stamp` | Permitted exact-item duplicate removal in supplied revision-bound transaction precedes re-read and stamp; wrapper never selects the item itself | L3-CLI-049, L3-CLI-050 |
| `stamp_command_keeps_noncanonical_name_finding` | Eligible g\sha: slot: successful value stamp keeps name finding and rename recommendation; raw name not silently changed | L3-CLI-049 |
| `stamp_command_final_bytes_verify_independently` | Transaction inserts a separator/newline outside exclusion: command result matches independent final-byte SHA and library re-read verification, canonical 2-/lower-case hex | L3-CLI-050, L3-CLI-051 |
| `stamp_command_verification_failure_is_not_success` | Inject final verification failure after emission: original input unchanged and failure retained, no successful stamped-result value | L3-CLI-049, L3-CLI-050 |
| `stamp_command_result_is_not_delivery_success` | Verified in-memory result followed by supplied destination delivery failure: computation result never suppresses that failure or authorizes in-place writes | L3-CLI-050 |
| `checksum_command_oracles_not_wrapper_mocks_only` | Integration evidence uses eligible actual documents/independent digests; an injected call/result alone is not SHA correctness or flex compatibility coverage | L3-CLI-051 |

Flex's exact source/algorithm and independent compatibility derivation remain
outstanding. Output delivery and ambiguous/incomplete access cases do not
remove their owner gates; new tests are planned, not executed coverage.

## Planned flex compatibility acceptance batch 36

These are source-qualification/acceptance obligations, not executed vectors.
Owner Q15 selects original bbaggerman/irig106lib; the [exact qualification
pin/hashes](research/2026-11-28-owner-policy-signoffs.md#flex-qualification-pin)
removes the reference-choice hold. Numerical values remain UNQUALIFIED pending
CI reference outputs, pinned-source/API analysis and independent arithmetic.
ADR-0014 still requires bit-for-bit compatibility including D7. Reference runs
are bounded, GitHub CI only, synthetic inputs only, no new project dependency.
Record outputs/revision/hashes, never paste reference source into this repo.
References: L1-SUM-003, ADR-0014/0018 and the 2026-09-25 tool investigation.
Synthetic filter tokens below are not assertions of Chapter 9 field meaning.

| Planned test | Independent input/evidence and expected obligation | Requirements |
|---|---|---|
| `flex_reference_snapshot_is_reproducible` | Pin revision/files/function/options/environment and fixed vectors; moving branch or historical prose alone fails readiness | L3-SUM-014 |
| `flex_raw_source_not_effective_projection` | Controls, leading-zero tokens and raw spelling differ from typed views: use qualified reference preparation, source unchanged; do not assume INT projections equivalent | L3-SUM-011 |
| `flex_reference_line_boundaries_are_explicit` | Qualified cases with CR/LF, delimiters, empty/layout-only input and no final semicolon have source-derived handling; no parser repair guessed | L3-SUM-011, L3-SUM-014 |
| `flex_uppercase_is_reference_byte_operation` | Mixed-case bytes use the reference's defined preparation, not unrelated locale/Unicode folding; no source rewrite | L3-SUM-011 |
| `flex_default_comment_and_group_filters` | Qualified complete COMMENT/group COM/G/V cases use the recorded default selection, with expected reference values fixed independently | L3-SUM-012 |
| `flex_r_identity_filter_is_exact_not_prefix_family` | Listed R filter tokens and neighboring nonmatching tokens retain the pinned source's exact selection behavior, never a guessed family-wide exclusion | L3-SUM-012 |
| `flex_d7_dpoc4_is_not_silently_fixed` | Qualified DPOC1/2/3 versus DPOC4 and MPOC4 vectors pin the accepted exclusion slip; no corrected symmetry | L3-SUM-012, L3-SUM-015 |
| `flex_each_inclusion_option_has_reference_vectors` | Every supported reference inclusion option and accepted combination has independently fixed results; public CLI syntax remains separate | L3-SUM-012, L3-SUM-014 |
| `flex_arithmetic_matches_independent_reference` | Qualified lines check initialization/packing/odd length/reduction/final sum with a separately derived arithmetic oracle, not production functions | L3-SUM-013, L3-SUM-015 |
| `flex_long_line_and_carry_boundaries` | Bounded source-derived length/carry/overflow cases agree across qualified host environments or expose a documented readiness gap; no host-width accident accepted | L3-SUM-013, L3-SUM-015 |
| `flex_line_permutation_preserves_sum` | Reorder independently qualified complete contributing lines: same reference sum; do not generalize to fragments or malformed text | L3-SUM-013, L3-SUM-015 |
| `flex_repeated_line_is_not_set_deduplication` | Repeated qualified contributing line participates twice according to reference accumulation; fixed vector detects accidental deduplication | L3-SUM-013, L3-SUM-015 |
| `flex_byte_domain_gaps_block_positive_claims` | NUL/high-byte/odd/truncated stimuli without defined reference behavior remain recorded gaps, never fabricated positive compatibility vectors | L3-SUM-011, L3-SUM-014 |
| `flex_options_and_source_identity_survive_wrapper` | Result retains non-standard identity and the actual options/source/revision; no exact public serialization chosen | L3-SUM-016 |
| `flex_is_not_sha_or_packet_integrity` | Flex result neither enters G\SHA nor substitutes for standard verification or core data-checksum evidence; all states stay separate | L3-SUM-016 |
| `flex_mocked_wrapper_is_not_algorithm_coverage` | Wrapper propagation succeeds but no pinned arithmetic/filter vectors exist: flex implementation readiness remains unsatisfied | L3-SUM-014, L3-SUM-015 |

## Planned checksum-evidence independence follow-up from #68

These test supplied independent outcomes, not an admission bypass. A real
integration test must first satisfy its reviewed source/access gates; it may
not read a rejected packet just to manufacture a G\SHA result.

| Planned test | Independent input and expected result | Requirements |
|---|---|---|
| `checksum_core_data_mismatch_and_gsha_match_stay_separate` | Given core PacketInfo data-checksum Mismatch and independently evaluated G\SHA match: both retained unchanged, no packet-integrity upgrade or forced G\SHA mismatch | L3-CLI-010, L3-CLI-047 |
| `checksum_core_data_notchecked_and_gsha_match_stay_separate` | Given NotChecked and independently evaluated G\SHA match: NotChecked never becomes checked/match, G\SHA result unchanged | L3-CLI-010, L3-CLI-047 |
| `checksum_core_evidence_never_repairs_gsha_failure` | Supplied core Mismatch/NotChecked paired with G\SHA mismatch/indeterminate: preserve each state/findings, no promotion of either or access authority | L3-CLI-010, L3-CLI-047 |

## Planned suspect-terminator batch 37

Planned internal warning/suggestion checks for L1-READ-007, still suspect
under ROADMAP S1. Sources: 106-24R1 Ch9 section 9.4.2 p. 9-2, Appendix
9-C p. C-8, and ADR-0007/0026. G\PN's reviewed name is Table 9-2 PROGRAM
NAME p. 9-10; synthetic COMMENT values below are intentional byte fixtures,
not an assertion that all such values are errors. Use independently supplied
recognizer coverage where private-name grammar is pending.

| Planned test | Independent input and expected result | Requirements |
|---|---|---|
| `suspect_colon_keeps_one_original_item` | `COMMENT:before: G\PN:x;`: one original item [0,23), value [8,22), warning at [14,15), proposal replaces only that byte; no G\PN lookup item | L3-READ-034, L3-READ-035, L3-READ-036 |
| `suspect_colon_no_blank_still_matches` | `COMMENT:before:G\PN:x;`: no optional blank, complete reviewed name/colon still qualifies; source unchanged | L3-READ-034 |
| `suspect_colon_multiple_candidates_stay_one_value` | Two independently located recognized name/colon patterns in one COMMENT value: two proposals, no recursive splitting or applied edits | L3-READ-035, L3-READ-037 |
| `suspect_colon_mixed_case_uses_whole_name_rule` | Lowercase g\pn lookahead with own colon qualifies under ASCII comparison, preserving spelling and raw edit location | L3-READ-034, L3-READ-037 |
| `suspect_colon_control_projection_maps_raw_span` | Following G\P<CR><LF>N: recognized under INT-036 projection: locate the original triggering colon, retain omission information and every control byte | L3-READ-035, L3-READ-037 |
| `suspect_colon_time_and_url_are_not_code_names` | Separate COMMENT values contain Time 12:30 or https://example.test: no complete recognized code name with own colon, no targeted warning | L3-READ-034 |
| `suspect_colon_partial_name_or_no_own_colon` | Following G\ or G\PN without its own colon: predicate not satisfied; no prefix match | L3-READ-034 |
| `suspect_colon_actual_name_delimiter_not_candidate` | `G\PN:x;`: its actual name/value delimiter is not inside the value, no targeted warning | L3-READ-034 |
| `suspect_colon_does_not_cross_semicolon` | `COMMENT:before:;G\PN:x;`: no lookahead across first item's semicolon, no targeted warning or merged item | L3-READ-034, L3-READ-036 |
| `suspect_colon_eof_never_invents_delimiter` | `COMMENT:before: G\PN` at EOF: no own colon/terminated lookup item invented, framing/suffix evidence retained | L3-READ-034, L3-READ-036 |
| `suspect_colon_intentional_lookalike_stays_possible` | Author intentionally quotes complete G\PN:x text after a colon: heuristic may warn but never claims knowledge of intent, changes source or closes S1 | L3-READ-037, L3-READ-038 |
| `suspect_colon_default_output_and_hash_domain_unchanged` | Read/inspect suggestions then default replay/hash on an eligible input: exact original bytes/domain and unchanged item inventory, proposals unapplied | L3-READ-036 |
| `suspect_colon_unknown_grammar_is_not_guessed` | Supplied unresolved V/X recognition metadata: no fabricated complete-name evidence; coverage limitation explicit rather than a claimed all-name pass | L3-READ-034, L3-READ-038 |
| `suspect_colon_appendix_targeted_locations_only` | Source code-name excerpt retains documented colon-ended patterns: target proposals independently listed; other literal-example findings not suppressed or mistaken for extra targeted warnings | L3-READ-037 |
| `suspect_colon_s1_not_cleared_by_regressions` | Synthetic/source-excerpt checks pass: real-recording confirmation still outstanding, no new severity decision or removed suspect mark | L3-READ-038 |
| `suspect_colon_limit_failure_is_not_no_candidates` | Bounded colon-rich input triggers supplied recognition/finding budget failure: preserve typed failure/limits, not clean complete no-candidate status or silent dropping | L3-READ-039 |

No report renderer, automatic repair, private grammar, future line-break
heuristic or sample-data run is introduced by these planned checks.

## Detector clarifications carried after #70

| Planned test | Independent input and expected result | Requirements |
|---|---|---|
| `suspect_colon_skips_unterminated_suffix` | `COMMENT:before: G\PN:x` at EOF contains a lookalike but no terminating semicolon: retained suffix, no detector warning/proposal or new lookup item | L3-READ-035, L3-READ-036 |
| `suspect_colon_cut_qualification_never_upgraded` | Given permitted diagnostic inspection of a terminated value within supplied cut-qualified text: suggestion retains cut/Incomplete or Ambiguous-with-cut qualification, never usable/governing and never an INT-031 bypass | L3-READ-035, L3-READ-036 |
| `suspect_colon_appendix_vocabulary_is_required` | Independently listed Appendix 9-C names/patterns include complete D-1\MML\N-1-1, D-1\MNF\N-1-1-1 and D-1\WP-1-1-1-1 forms and the rest of the [source-fixed pattern/literal inventory](research/2026-11-28-appendix9c-recognizer-inventory.md): recognition coverage is required for every listed full shape; XML relative/defective annotations do not become accepted names | L3-READ-034, L3-READ-037 |
| `suspect_colon_missing_appendix_name_is_coverage_failure` | Remove one required reviewed excerpt pattern from a synthetic recognizer catalog: readiness visibly fails, never counts a missed warning as clean no-candidate output | L3-READ-034, L3-READ-038 |

## Planned session-evidence batch 38

Synthetic supplied CSDW/session/trigger evidence only; no invented packet
identity or body access. Sources: 106-24R1 Ch11 section 11.2.7.2 Figure
11-34 pp. 11-50–51 and approved INT-030/F7. Core routing/channel checks and
owner-signed INT-031 access gates remain independent.

Owner Q16/17 (2026-11-28, ADR-0038) selects archived L1 edition wording
and Complete-only mixture finding. Non-Complete FRMT remains qualified
evidence without the formal finding; source/executable gates remain.

| Planned test | Independent input and expected result | Requirements |
|---|---|---|
| `session_ascii_xml_same_scope_reports_mixture` | Given Complete ASCII and Complete XML records in supplied S: retain source/qualification/scope evidence; Complete ASCII plus Complete XML produces the mixture finding; broader candidate scope is not selected | L3-CH10-022 |
| `session_formats_in_separate_scopes_not_mixture` | ASCII in supplied S1, XML in S2: no cross-scope mixture; scopes/provenance retained, not merged by file proximity | L3-CH10-022 |
| `session_caller_scope_keeps_assertion_label` | S is caller asserted rather than observed control commands: check retains that basis, never certifies .RECORD/.STOP was seen | L3-CH10-022 |
| `session_xml_observation_does_not_parse_body` | Readable XML FRMT with unsupported body: retain format evidence without ASCII/XML body parsing or eligibility upgrade | L3-CH10-022, L3-CH10-023 |
| `session_unknown_scope_is_not_clean_pass` | Readable ASCII/XML observations but no established shared session: retain unavailable scope, no guessed grouping or clean full-scope result | L3-CH10-023 |
| `session_unknown_frmt_is_not_default_ascii` | Unreadable format evidence: retain unknown and coverage limitation, no ASCII default | L3-CH10-023 |
| `session_partial_extent_never_proves_global_no_mixture` | Known supplied prefix contains only ASCII, later traversal unavailable: no observed mixture limited to prefix, never whole-recording conformance | L3-CH10-023 |
| `session_frmt_conflict_keeps_record_qualification` | Conflicting format observations within supplied Ambiguous candidate: all evidence retained, no winning FRMT or repaired boundary | L3-CH10-023 |
| `session_f7_unidentified_event_is_indeterminate` | Established SRCC=1 trigger, event identity unverified: indeterminate missing basis, no pass/fail claim | L3-CH10-024 |
| `session_nearby_format2_event_is_candidate_only` | Same trigger with nearby generic Format 2 recording event: candidate provenance retained, result still indeterminate | L3-CH10-024 |
| `session_no_guessed_event_match_is_not_missing_event` | No packet matches a guessed event type: no missing-event verdict while identity/prior scope remain unreviewed | L3-CH10-024 |
| `session_first_record_srcc1_is_separate_evidence` | Caller supplies first-record context and SRCC=1: separately located observation plus F7 uncertainty, not automatic missing-event conclusion | L3-CH10-025 |
| `session_fragment_srcc_disagreement_is_not_aggregated` | Same FRMT/RCCVER but SRCC 0/1/unreadable across fragments: retain every reading; no OR/AND/first/last general summary or event verdict | L3-CH10-025 |
| `session_srcc0_never_proves_byte_identity` | SRCC=0 with independently differing content evidence: bit retained, no unchanged-byte assertion from it alone | L3-CH10-025 |
| `session_core_channel_evidence_passes_through` | Supplied core legacy/edition admission and integrity evidence retained with PacketFacts, not reclassified by a local threshold or event/format result | L3-CH10-026 |
| `session_oracle_has_no_reader_or_scope_guess` | Independently listed observations/context/expected results; production scope inference or invented packet identity cannot establish fixture correctness | L3-CH10-027 |

F7's determinate presence/absence check remains BLOCKED on verified identity,
reviewed prior-ordering, source freshness, analysis and executable tests.
Per-edition parent/channel wording needs source reconciliation; these cases
do not select a packet routing rule or authorize new access/governing policy.

## Session-scope follow-ups after #71 (Q16/17 signed 2026-11-28)

| Planned test | Independent input and expected result | Requirements |
|---|---|---|
| `session_complete_ascii_incomplete_xml_scope_pending` | Complete ASCII plus Incomplete XML candidate in explicit S: Owner-selected A retains qualified XML evidence without formal mixture finding; no qualification upgrade | L3-CH10-022, L3-CH10-023 |
| `session_same_bytes_without_session_assertion` | Same supplied recording bytes/facts contain ASCII/XML observations, no actual session evidence and no caller assertion: unavailable scope, never inferred one-session or no-mixture | L3-CH10-022, L3-CH10-023 |
| `session_same_bytes_with_labelled_whole_recording_assertion` | Exact same bytes/facts plus explicit caller assertion that this recording is one session: scope available with asserted basis retained; formal mixture finding applies only to Complete ASCII/XML in that asserted scope, not inferred command observations | L3-CH10-022, L3-CH10-023 |

## Planned argument-parser mechanism batch 39

Internal requests/usage results only, not new public grammar or rendering.
Mechanism fixtures may supply a synthetic reviewed table; product coverage
requires every actual implemented rule to have reviewed independent cases.
Authority: accepted ADR-0013 and ADR-0034 W3, L1-CLI-009.

| Planned test | Independent input and expected result | Requirements |
|---|---|---|
| `args_only_supplied_tokens_are_consumed` | Supplied tokens produce the table-defined request regardless of unrelated process arguments/environment; no global argv read | L3-CLI-052 |
| `args_parser_no_source_or_stream_acquisition` | Parser request/usage paths use bounded in-memory tokens only; file/stdin/output/process-exit hooks are never reached | L3-CLI-052, L3-CLI-055 |
| `args_reviewed_show_raw_request` | Reviewed show --raw operand form yields that internal request with the exact operand; no file acquisition, rendering or input-route claim | L3-CLI-053, L3-CLI-054 |
| `args_missing_required_operand_is_usage` | Reviewed row requires a value, supplied sequence ends first: usage classification, no executable partial request | L3-CLI-053 |
| `args_unknown_under_table_is_not_guessed` | Supplied token is unaccepted by the fixture's reviewed grammar: usage failure, no nearest command/option repair | L3-CLI-053 |
| `args_declared_conflict_is_not_executed` | Fixture table explicitly defines a conflict: conflicting input yields usage; this does not define product duplicate/precedence policy | L3-CLI-053, L3-CLI-055 |
| `args_operand_identity_not_tmats_projection` | Supplied path/value tokens preserve identity unless the reviewed grammar explicitly decodes them; INT-036/038/055 does not silently rewrite arguments | L3-CLI-054 |
| `args_decoder_failure_keeps_usage_result` | Reviewed value decoder refuses an operand: no guessed edition/record/clock target or command execution | L3-CLI-053, L3-CLI-054 |
| `args_empty_and_truncated_use_reviewed_rows` | Bounded empty/truncated token lists produce their table-defined usage/request outcome, with no automatic undocumented command default | L3-CLI-053, L3-CLI-057 |
| `args_usage_reaches_host_without_exit` | Rejected request returns its classification to the calling host, which continues running; no output-text/exit-number assertion | L3-CLI-055 |
| `args_success_does_not_hide_later_failures` | Parsed valid request later hits acquisition/library/delivery failure: preserve the failure, no claim of successful command from parse success | L3-CLI-055 |
| `args_binary_and_host_share_parser` | Identical supplied arguments at both adapters construct identical request/usage outcomes before rendering, not two separately maintained parsers | L3-CLI-056 |
| `args_individual_command_needs_no_global_parse` | Explicit request calls an individual command without main/global argv parsing or process lifetime changes | L3-CLI-056 |
| `args_help_version_have_no_hidden_exit` | Reviewed help/version table entries return their request/outcome; no parser-side print/exit or claim of full command coverage | L3-CLI-052, L3-CLI-055 |
| `args_each_implemented_row_has_independent_cases` | Inventory of actual implemented reviewed command/flag/value/error rules maps to independent positive/negative tests; missing cases or unreviewed active rows fail readiness | L3-CLI-057 |
| `args_synthetic_table_is_not_product_coverage` | Mechanism fixtures pass on a synthetic table while product rows are absent/unreviewed: product readiness still unsatisfied, not inferred from parser unit success | L3-CLI-057 |

Held grammar rules need their own review before activation. No report-format,
numeric CLI syntax, separator/abbreviation, non-Unicode policy or new default
is selected; no dependency or executable parser is added by these cases.

## Planned extract owner-policy cases (ADR-0037, 2026-11-27)

**Owner Q18, 2026-11-28:** strict refuses the listed bad-data kinds/class,
not informational notes. Owner-choice holds are removed; selected expected
results remain PLANNED, not executed coverage. ADR-0037's original text is
history, refined by ADR-0038; no severity-derived class predicate.

Extract row spellings --force and --strict are owner-approved (2026-11-27);
the rest of argument grammar remains on hold.

No finding is an always-refuse extraction class. For EACH of data-checksum
mismatch, secondary-header failure, INT-045 cut, INT-048 ambiguity, INT-037
marker and INT-055 invalidity, and for ALL SIX combined, instantiate both
default and --strict fixtures below: default exact bytes plus every finding;
strict refusal without successful extraction. Use synthetic retained extents
and unchanged core/record evidence; no repaired/guessed byte ranges.

| Planned test | Independent input and expected result | Requirements |
|---|---|---|
| `extract_findings_default_each_kind` | Six individually named variants as above: extraction proceeds with exact original bytes and every finding, no semantic/governing upgrade | L3-CLI-031, L3-CLI-060, L3-CLI-061 |
| `extract_findings_strict_each_kind` | Same six variants with --strict: observable refusal, all findings retained, no successful byte-delivery claim | L3-CLI-060, L3-CLI-061 |
| `extract_all_findings_default_and_strict` | All six kinds combined: default extracts exact bytes and reports all; strict refuses, no always-refuse kind fabricated | L3-CLI-060, L3-CLI-061 |
| `extract_cut_prefix_is_available_raw_data` | Supplied original cut prefix under Incomplete or Ambiguous-with-cut: default extracts the retained prefix, never demands an invented complete suffix; strict refuses on its findings | L3-CLI-030, L3-CLI-060 |
| `extract_strict_unlisted_information_scope_pending` | Only an information finding outside the six listed bad-data kinds: Owner-selected Q18 permits byte-exact strict extraction and reports the informational note; no all-findings refusal | L3-CLI-060, L3-CLI-061 |
| `extract_strict_without_findings_is_allowed` | Explicit selected synthetic input has no findings: --strict does not itself refuse; exact bytes, normal destination policy | L3-CLI-060 |
| `extract_findings_never_enter_raw_stream` | Default emits exact bytes on selected sink and retains every diagnostic separately; no labels/diagnostic text inserted into extracted bytes, no layout/stream mapping pinned | L3-CLI-031, L3-CLI-060 |
| `extract_existing_unrelated_refuses_without_force` | Existing unrelated regular destination: observable default refusal without truncation/write/rename | L3-CLI-034, L3-CLI-058 |
| `extract_existing_unrelated_force_same_handle` | Same destination with --force: non-truncating open, identity check, truncate/write via SAME handle; no reopen or temp-file+rename | L3-CLI-034, L3-CLI-058 |
| `extract_known_input_alias_force_still_refuses` | Same-path/symlink/hard-link known input alias plus --force: refusal before destructive access, input unchanged | L3-CLI-034, L3-CLI-058, L3-CLI-059 |
| `extract_stdout_and_pipe_identity_not_required` | Synthetic stdout and pipe sinks have no regular-file identity: allowed, exact bytes, ordinary fallible delivery; no proof-failure refusal | L3-CLI-059 |
| `extract_unproven_regular_refuses_without_force` | Inject regular-file identity unavailable (network-share analogue): refusal before truncate/write, no fabricated proof | L3-CLI-059 |
| `extract_unproven_regular_force_allows_without_proof_claim` | Same facts plus --force: owner-authorized write through same non-truncating opened handle, identity remains unproven and is never relabelled checked | L3-CLI-034, L3-CLI-059 |
| `extract_force_does_not_override_strict` | --force plus --strict with a listed INT-055 invalidity finding: strict refusal, unrelated destination not truncated/written; flags govern separate decisions | L3-CLI-058, L3-CLI-060 |
| `extract_force_write_failure_keeps_failure_visible` | Authorized in-place unrelated-file write fails after prefix: observable partial delivery/failure, no rename fallback or success claim | L3-CLI-033, L3-CLI-058 |
| `extract_no_extent_is_not_a_findings_refusal_class` | No selected raw extent available at all: observable operational unavailability, never guessed core boundaries or fabricated bytes; distinct from available cut prefix | L3-CLI-032, L3-CLI-060 |

These are planned checks, not current CLI behavior. Source/development,
independent review and executable testing remain; the three #57 owner holds
are removed. General semantic/governing access is not raw extract permission.

## Planned bounded lookup-state batch 40

These supply L1-VIEW-006's direct 0.1 trace with internal exact-candidate
states, not a general link engine. Table 9-4 (106-24R1) TK1 p. 9-48,
DSI/CHE pp. 9-49, SHTF p. 9-51, TTF/TFMT/TSRC pp. 9-71–72 and NSB p.
9-33 supply the reviewed scope; INT-036/038/039/041/042 supply existing key,
multiplicity and value/presence behavior. Availability/record qualifications
are supplied explicitly where their interpretation remains gated.

| Planned test | Independent input and expected result | Requirements |
|---|---|---|
| `bounded_lookup_zero_exact_candidates` | Available scoped TFMT lookup with no matching item: unresolved, zero candidates; no proof of generic-link or clock association resolution | L3-VIEW-066 |
| `bounded_lookup_one_item_is_identity_resolved` | One R-1\TFMT-3:B;: resolved to its exact identity/location, with separate reviewed field result | L3-VIEW-065, L3-VIEW-066 |
| `bounded_lookup_equal_repeats_are_ambiguous` | Two identical TFMT occurrences at the same key: ambiguous, both IDs/locations retained, never deduplicated | L3-VIEW-066 |
| `bounded_lookup_invalid_candidate_not_filtered` | TFMT B plus a second same-key invalid value: two candidates/ambiguous, not resolved to B; both findings retained | L3-VIEW-066, L3-VIEW-068 |
| `bounded_lookup_one_empty_or_invalid_stays_present` | Single TFMT with zero-length or invalid value in separate cases: identity resolved, value Invalid/unavailable per its existing rule, never zero candidates | L3-VIEW-068 |
| `bounded_lookup_actual_id_is_not_entry_index` | Entry 3 has TK1 value 42: TFMT-3 lookup binds entry 3, never TFMT-42 or an item in another entry | L3-VIEW-065 |
| `bounded_lookup_recorder_context_not_borrowed` | Same entry index under R-1/R-2 with different formats: each exact recorder query retains only its own items | L3-VIEW-065 |
| `bounded_lookup_recorder_level_nsb_has_no_entry` | Recorder-level R-1\NSB:0; queried under its reviewed arity: no fabricated -3 suffix or per-entry source-bit item | L3-VIEW-065 |
| `bounded_lookup_case_and_index_collisions_keep_occurrences` | R-1\TFMT-3:B; and r-01\tfmt-03:B;: same canonical full key, ambiguous with all raw spellings and INT-038 information retained | L3-VIEW-066 |
| `bounded_lookup_reorder_changes_listing_not_winner` | Reorder same-key candidates with fixed identities: cardinality remains ambiguous, listing follows retained source order, no first/last winner | L3-VIEW-066, L3-VIEW-070 |
| `bounded_lookup_default_is_not_a_candidate` | TFMT absent with supplied already evaluated applicable baseline default-A condition: lookup unresolved/zero explicit items, effective field Defaulted A with citation; source unchanged | L3-VIEW-068 |
| `bounded_lookup_unavailable_prerequisite_not_empty_success` | Supplied unresolved scope/key or blocked interpreted lookup (e.g. INT-037 marker): limitation retained, no invented usable zero-candidate state | L3-VIEW-067 |
| `bounded_lookup_damage_zero_matches_not_absence_proof` | Zero exact TFMT matches plus compatible R-1\TF\xFFMT-3:A; evidence: retain no exact item plus damage; never infer physically unspecified/default applicability; presence cannot-evaluate per INT-042 | L3-VIEW-067 |
| `bounded_lookup_suffix_not_explicit_item` | R-1\TFMT-3:A at EOF: suffix retained but not a candidate; zero exact matches never prove absence or a default condition | L3-VIEW-067 |
| `bounded_lookup_documents_and_revisions_stay_scoped` | Same canonical key in separate documents/revisions: never combined into one result or borrowed to fill a missing field | L3-VIEW-065, L3-VIEW-069 |
| `bounded_lookup_qualification_not_upgraded` | Permitted diagnostic candidate evidence under Complete/Incomplete/Ambiguous-with-cut separately: lookup never changes qualification, removes cut or grants INT-031 access/governing use | L3-VIEW-069 |
| `bounded_lookup_assumption_does_not_change_candidates` | Supplied association/caller-choice context and Quiet level: original field/source evidence and lookup cardinality unchanged; assumption emission policy is separate | L3-VIEW-069 |
| `bounded_lookup_oracle_is_independent` | Fixed source/key/candidate/state fixtures, including supplied condition evidence, independent of production lookup or a positive clock-association invention | L3-VIEW-070 |

All are planned checks, not executed tests. No new default, public query
syntax, report schema or incomplete/ambiguous access option is approved.

## Planned normalization-policy batch 41

Joey signed INT-064 A+D / INT-065 A / INT-066 A on 2026-11-28; independent
pre-checks concurred. Their owner-choice holds are removed; source freshness,
written analysis/review and intensive executable tests remain prerequisites. Definitions/byte oracles come from reviewed
source regions and explicitly supplied condition/member evidence; no implicit
policy is calculated through the production normalizer.

| Planned test | Independent input/expected obligation | Requirements |
|---|---|---|
| `normalize_request_without_policy_is_not_default_repair` | Missing/unready policy returns unavailable/refusal with original unchanged; default output still exact replay, never guessed canonical profile | L3-WRT-012 |
| `normalize_stale_revision_or_conflicting_plan_refuses` | Wrong revision, conflicting exact targets or unresolved composition: observable refusal, no partial successful normalization or source mutation | L3-WRT-012 |
| `normalize_order_explicit_bijection` | **Owner-signed INT-064 A:** source COMMENT:b;COMMENT:a;, explicit region order [second,first] gives exact COMMENT:a;COMMENT:b;, no byte changes inside either region | L3-WRT-013, L3-WRT-017 |
| `normalize_order_missing_or_repeated_identity` | **Owner-signed INT-064 A:** omission/repetition in requested full region order refuses, never deduplicates, drops unknown content or invents a copy | L3-WRT-013 |
| `normalize_order_preserves_raw_regions` | **Owner-signed INT-064 A:** complete selected regions with mixed case/duplicates/opaque bytes retain every byte exactly once; anchored prefix/suffix mapping must be positively established | L3-WRT-013, L3-WRT-017 |
| `normalize_order_region_mapping_unavailable` | **Owner-signed INT-064 A:** no unambiguous complete-region partition: order request refuses rather than treating a malformed suffix as a complete item or guessing ownership | L3-WRT-012, L3-WRT-013 |
| `normalize_group_profile_section_order` | **Owner-signed INT-064 A+D:** supplied positively grouped region IDs X1,R1,G1,T1,M1,P1,D1,B1,S1,Q1,C1,H1,V1,R2 yield G1,T1,R1,R2,M1,P1,D1,B1,S1,Q1,C1,H1,V1,X1 within one anchor-free run; raw regions intact | L3-WRT-013, L3-WRT-017 |
| `normalize_group_profile_stable_within_group` | **Owner-signed INT-064 D:** source R regions with entry indices 10 then 2 retain that order and duplicate occurrences; D does not perform B's numeric/full-key sort or deduplicate | L3-WRT-013 |
| `normalize_group_profile_anchors` | **Owner-signed INT-064 D:** supplied regions R1,G1,COMMENT,R2,G2 reorder to G1,R1,COMMENT,G2,R2; COMMENT/opaque/malformed anchors stay in place, no false claim of global group order across anchors | L3-WRT-013 |
| `normalize_group_profile_private_and_unknown_placement` | **Owner-signed INT-064 D:** complete positively grouped V/X regions preserve private bytes and within-group order; unknown/unavailable group classification anchors rather than guessing placement or resolving private attachment | L3-WRT-013, L3-WRT-017 |
| `normalize_newlines_layout_only` | **Owner-signed INT-065 A:** explicitly identified CRLF/lone-CR/lone-LF layout ranges convert to the caller's LF or CRLF target; CRLF is one token, all other bytes unchanged | L3-WRT-014 |
| `normalize_newlines_field_controls_untouched` | **Owner-signed INT-065 A:** real CR/LF within COMMENT value or internal code name stays raw; only positively classified layout converts, no INT-036 projection written back | L3-WRT-014 |
| `normalize_newlines_unknown_layout_unavailable` | **Owner-signed INT-065 A:** opaque/unclassified range prevents claiming complete requested conversion; no inferred trim/platform default or silent partial success | L3-WRT-012, L3-WRT-014 |
| `normalize_newlines_padding_is_not_linebreak` | **Owner-signed INT-065 A:** NUL/0x1A/space padding is retained; explicit newline conversion never deletes padding or inserts a missing semicolon | L3-WRT-014 |
| `normalize_counter_contiguous_distinct_members` | **Owner-signed INT-066 A:** evaluated applicable D MNF/N scope has child identities 1/01 and 2: k=2, proposed canonical value 2, every duplicate/spelling finding retained; invalid WP VALUE remains a structural member. Supplied reviewed MML/N membership descriptors likewise count canonical identities once with their own range, preserving duplicates; no MNF range import | L3-WRT-015, L3-WRT-017 |
| `normalize_counter_gap_refuses` | **Owner-signed INT-066 A:** child identities {1,3}: refuse counter rewrite; B would use 2 with gap findings, never silently write maximum 3 as a member count | L3-WRT-015 |
| `normalize_counter_invalid_old_value_explicit_rewrite` | **Owner-signed INT-066 A:** one complete identified target with malformed old value, established contiguous members {1,2}, accepted cited range: explicit counter operation proposes 2, old validation failure remains in old revision | L3-WRT-015, L3-WRT-016 |
| `normalize_counter_compatible_damage_blocks` | **Owner-signed INT-066 A:** compatible unassigned/damaged member evidence makes count incomplete: refuse, never count only readable children as complete | L3-WRT-015 |
| `normalize_counter_target_or_range_missing_refuses` | **Owner-signed INT-066 A:** absent/duplicate target, unavailable applicability/scope/range or count outside cited range refuses; no target insertion, clipping or renumbering | L3-WRT-015 |
| `normalize_checksum_is_not_implicitly_restamped` | Authorized changed-output fixture retains G\SHA bytes unless stamp separately requested; input match not reused as output proof, verify final domain independently | L3-WRT-016 |
| `normalize_source_and_old_findings_remain` | Original bytes/revision/item identities/findings stay available after proposed success/refusal; new revision has its own correspondence/reevaluated results, no automatic governing promotion | L3-WRT-016 |
| `normalize_explicit_compound_plan_has_coordinates` | Explicit reviewed composition order and source correspondence required; unresolved order/newline/counter overlap never runs in a guessed sequence | L3-WRT-012, L3-WRT-016 |
| `normalize_oracles_are_exact_bytes_not_typed_values` | Expected full output and untouched-region identities fixed independently; matching typed values/multiset or calling production writer again is not proof | L3-WRT-017 |

The default lossless writer and ADR-0037 extraction remain unchanged. These
are bounded library transform contracts, not an apply-all/general 0.5 edit
API or a 0.2+ CLI clean-up command. The 0.1 normalization obligation is not
claimed complete merely by adding gated children.

## Planned retention/limit evidence batch 42

Internal evidence tests use bounded synthetic ownership/span descriptors and
explicit fixture-model charges. They do not choose production charging,
allocator overhead, numeric defaults, failure atomicity, resume API or the
owner-signed non-setup host allocation rule (Q9). Source: L2-ROB-003/L3-ROB-004–006,
ADR-0036 and existing per-channel/provenance contracts.

| Planned test | Independent fixture and expected evidence | Requirements |
|---|---|---|
| `retention_inventory_covers_all_categories` | Supplied runs, owned/borrowed text, spans/fragments, provenance, snapshot and error storage all appear with domain/owner/size evidence; no hidden category | L3-ROB-014, L3-ROB-016 |
| `retention_small_slice_keeps_backing_owner_visible` | Two text bytes borrow a much larger retained owner: used length and backing capacity remain distinct, zero copy is not zero retained memory | L3-ROB-014, L3-ROB-016 |
| `retention_empty_fragments_still_have_metadata` | Empty text from several fragments/channels retains their metadata/provenance/owner references and declared charges, never a zero-length loophole | L3-ROB-014, L3-ROB-016 |
| `retention_shared_owner_keeps_all_references` | Two runs point to one supplied owner identity: inventory preserves both references and sharing; no charging formula inferred from it | L3-ROB-014, L3-ROB-015 |
| `retention_closing_one_reference_is_not_free` | One shared run closes while another/snapshot retains the owner: owner still live, no dangling reference or falsely claimed full release | L3-ROB-015 |
| `retention_handoff_is_not_deallocation` | Completed record/caller takes a retained span: ownership/domain transfer distinguished from physical release; no claim of total application heap reduction | L3-ROB-015, L3-ROB-016 |
| `retention_snapshot_and_error_state_are_in_scope` | Assembler-owned snapshot/error reference still retains owner/metadata after main run changes: visible under the declared model, no budget escape | L3-ROB-014, L3-ROB-018 |
| `retention_unknown_capacity_is_not_zero` | Backing capacity/overhead needed for claimed bound is unavailable: readiness limitation exposed, not assumed zero or a hard memory guarantee | L3-ROB-016 |
| `retention_model_does_not_claim_rss_bound` | Fixture covers open-run bytes but excludes caller outputs/allocator unknowns: scope/limitations retained, no global process/RSS claim | L3-ROB-016 |
| `retention_limit_equality_is_not_excess` | Fixture limit 64, charged 50, proposed charge 14: arithmetic fits exactly; no other constraints supplied, so byte comparison alone is not excess | L3-ROB-017 |
| `retention_excess_is_not_full_admission` | Same limit/state, proposed full charge 15: cannot claim full operation admitted within 64; actual acceptance/disposition follows the separately supplied failure-policy result | L3-ROB-017, L3-ROB-018 |
| `retention_integer_overflow_is_typed_limit` | Synthetic integer max 255, charged 254, delta 2: typed arithmetic/limit failure evidence, no wrap/panic or enormous allocation | L3-ROB-017, L3-ROB-018 |
| `retention_channels_share_aggregate_limit` | Fixture charges from several independent runs combine under one supplied limit; do not enforce each channel as if it had a separate full budget | L3-ROB-016, L3-ROB-017 |
| `retention_failure_disposition_is_exact` | Injected failure record lists accepted fragment/prefix and rejected remainder with origins/ownership: no lost/duplicated ranges or fabricated bytes | L3-ROB-018, L3-ROB-019 |
| `retention_failure_ledger_does_not_choose_atomicity` | Separately supplied all-rejected versus explicit-prefix failure records exercise common evidence validation only; neither is approved as the production failure contract or a safe retry promise | L3-ROB-018 |
| `retention_error_payload_copy_cannot_escape_budget` | Proposed full rejected-payload copy hidden in error path: inventory/claimed-domain check detects unaccounted retention, no false bounded-state success | L3-ROB-014, L3-ROB-018 |
| `retention_limit_is_not_joining_or_validity_signal` | Limit failure with an open run: preserve its actual state/source context, never invent EOF/stop/Complete, malformed findings or governing adoption | L3-ROB-018 |
| `retention_fixture_model_is_not_production_evidence` | Synthetic charge totals pass but real ownership/allocation model is unverified: implementation hard-bound readiness still unsatisfied | L3-ROB-016, L3-ROB-019 |
| `retention_core_refusals_and_drains_stay_core_owned` | Supplied core HardLimit / BudgetTooSmall / TargetLimit refusals and drain reports retain identity, extent and context unchanged; distinct from injected tmats assembler-retention failure, never reclassified | L3-CLI-042, L3-ROB-018 |
| `retention_pool_credits_are_in_domain_not_counted_twice` | Retained DATA spans hold supplied core BlockPool credits: inventory includes them inside the budget domain, not a borrowed/caller exclusion; overlapping core/assembler byte totals are not added twice | L3-ROB-014, L3-ROB-016, L3-ROB-018 |
| `retention_open_run_holds_pool_to_b_reader_stall` | Owner Q22: synthetic B credits with independently supplied verified progress reserve R; open run attempts retaining beyond B-R. Refuse/fail/report run with typed assembly budget exhausted error before consuming reserve, preserve truthful ownership/disposition and core evidence, reader continues; no silent eviction or invented EOF; early copy was not selected as the progress policy, with no blanket ban inferred. Actual API reserve proof remains required | L3-ROB-016, L3-ROB-019 |

Operational failure is not a new always-refuse extraction finding. Raw
extract still requires an available selected extent; no guessed byte ranges,
silent eviction, local spill, partial-acceptance policy or source truncation
is authorized. Tests are planned, not executed memory/performance evidence.

## Planned checksum-domain analysis cases — 2026-11-28

[Written analysis](research/2026-11-28-checksum-domain-development-analysis.md)
for approved INT-018/019/020/021/043. These are PLANNED cases, not passing
implementation/reference tests. Source fingerprints, historical applicability
limits, independently fixed preimages/digests and raw exclusion [3,78) are
in the note. The empty-preimage digest is not a padding-only eligibility rule.
Owner/independent analysis review and intensive executable tests remain.

| Planned case | Independent expected evidence | Requirements / interpretations |
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

## Planned owner Q26 report/show acceptance — 2026-11-28

Owner Q26 A/ADR-0039 removes the report-format hold for CLI-002/006 and
show. These are planned executable cases, not implemented schema artifacts
or passing renderer tests. Concrete schema/catalogue/request-table review
remains development work. No new dependency or severity threshold is chosen.

| Planned test | Supplied scenario and expected result | Requirements |
|---|---|---|
| `q26_raw_show_preserves_original_bytes` | Selected raw text includes TAB, BOM, high bytes, unknown and duplicate items; exact source bytes are delivered, no interpreted projection or JSON wrapper. Findings/qualifications remain available separately | L3-CLI-068 |
| `q26_tree_keeps_unknown_and_duplicate_items` | Reviewed structure plus opaque private items and equal duplicates: every source occurrence remains identifiable, no guessed link or merged node | L3-CLI-069 |
| `q26_channel_summary_keeps_disabled_time_context` | R-entry summary includes disabled TIMEIN with CHE/DSI/TSRC as context; no candidate silently filtered and no association inferred | L3-CLI-070 |
| `q26_report_does_not_upgrade_qualifications` | Supplied Invalid/Ambiguous/unavailable fields and cut-qualified record remain qualified in tree/summary and JSON; display is not permission to govern | L3-CLI-070, L3-CLI-073 |
| `q26_default_text_and_explicit_format_values` | Omitted format selects readable text; --format json and --format text select the pinned peer values; byte-exact mode applicability is explicit in the reviewed request table | L3-CLI-071 |
| `q26_unknown_format_returns_usage` | Unsupported format value follows the usage-refusal path before command execution; no invented --json alias or silently ignored option | L3-CLI-071, L3-CLI-076 |
| `q26_result_and_operational_streams_separate` | Independent stdout/stderr sinks see results versus operational diagnostics; stderr text never corrupts JSON/raw payload | L3-CLI-072 |
| `q26_json_ascii_escaping_preserves_unicode` | **Coordinator reading of the owner's "matching irigtime" (Q26 A), pending owner confirmation (2026-11-28)**; proposed expectation follows. Supplied report string café plus U+1F680: JSON stdout contains ASCII bytes only, with Unicode escapes for é and a surrogate pair for U+1F680; parsed string equals the supplied string. Raw show/extract retain original bytes, including high bytes; JSON encoding is not source repair | L3-CLI-072, L3-CLI-078 |
| `q26_text_json_share_supplied_facts` | Fixed command facts/findings and qualifications appear in both representations without re-parsing or candidate reselection | L3-CLI-072 |
| `q26_schema_preserves_finding_identity_and_locations` | Independently specified finding codes/severity/source spans and ambiguous_g106 with numeric candidates survive schema validation; unknown locations are not fabricated | L3-CLI-073 |
| `q26_unavailable_not_zero_or_missing_location` | Explicit unavailable reason and retained cut survive; unavailable is not zero/false/empty success, and unknown source location is distinguishable from offset zero | L3-CLI-073 |
| `q26_assumption_default_fallback_are_distinct` | Explicit, assumed, defaulted and fallback facts retain their distinct basis, citations and caller-choice level; no next-query default | L3-CLI-074 |
| `q26_quiet_zero_marker_is_present` | Quiet with zero attached Warn bad-data findings still carries suppressed: 0 bad-data findings, caller level and assumption label in both representations | L3-CLI-074 |
| `q26_quiet_suppresses_exact_warn_attachment_set` | Quiet with N>0 suppresses exactly Warn attachments of every severity, reports the count, preserves target states/source diagnostics and cannot upgrade validity or command outcome | L3-CLI-074 |
| `q26_additive_schema_change_increments_minor` | Independently specified compatible additive schema change increments minor, preserving major and predecessor identity; crate version is not the schema version | L3-CLI-075 |
| `q26_breaking_schema_change_increments_major` | Changed field type/meaning or incompatible shape increments major; peer schema_revision alone cannot satisfy the policy | L3-CLI-075 |
| `q26_runner_returns_all_five_exit_classes` | Fixed success/failed-check/usage/input/output outcomes return 0/1/2/3/4 in runner/direct paths; mounted host survives and format changes no criterion | L3-CLI-076 |
| `q26_json_success_is_complete` | Exit 0 delivers one complete schema-conforming report from independent expected facts, not a renderer-generated oracle | L3-CLI-077 |
| `q26_json_failed_check_is_complete` | Exit 1 still delivers complete schema-conforming JSON with findings/qualifications, never partial because the check failed | L3-CLI-077 |
| `q26_writer_or_flush_failure_returns_output_class` | Injected write or flush failure returns 4; consumers discard output even if bytes appear complete. No atomic stdout/rollback promise | L3-CLI-077 |
| `q26_independent_oracles_reject_mutated_reports` | Independently specified expected facts/raw bytes and reviewed schema/goldens reject missing suppression counts, altered qualifications, wrong version/types and corrupt payloads; renderer agreement alone is insufficient | L3-CLI-078 |

## Planned show-free command orchestration batch 43

These are bounded integration cases using explicit reviewed internal requests,
source/revision descriptors and independent stage oracles. No CLI syntax,
rendered findings/assumption labels/suppression marker, numeric exits or new
record-selection/access policy is specified. Policy-dependent real-command
arms remain gated, not silently enabled by a successful injected adapter.

| Planned test | Supplied scenario and expected internal evidence | Requirements |
|---|---|---|
| `command_chain_preserves_selected_source` | Two sources have equal text but different identities; explicit selection reaches computation with its own identity, never a value-based replacement | L3-CLI-062 |
| `command_chain_preserves_selected_revision` | Selected input and distinct proposed output revisions stay separate; stamp verification refers to the explicit output, not input proof | L3-CLI-062 |
| `command_chain_does_not_pick_identical_copy` | Two equal scoped candidates retain their separate bindings; requested candidate is used, no latest/first/digest winner | L3-CLI-062 |
| `command_chain_raw_extract_keeps_control_bytes` | Use existing byte-exact extract fixture with controls/BOM: handoff bytes remain original, with existing findings; no projection/serialization inserted | L3-CLI-063 |
| `command_chain_checksum_keeps_raw_domain` | Use independently fixed checksum-domain fixture: orchestration supplies raw selected source and established exclusion, not interpreted text or flex | L3-CLI-063 |
| `command_chain_keeps_core_and_sha_independent` | Supplied core Mismatch/NotChecked and independent G\SHA result remain separate through command result, without trust/qualification upgrade | L3-CLI-063 |
| `command_chain_keeps_limit_owners_distinct` | Inject core admission refusal/drain evidence versus tmats limit failure: unchanged source/stage ownership, no alternate route or double-counted bytes | L3-CLI-063, L3-CLI-065 |
| `command_chain_next_call_has_no_clock_default` | First query supplies a caller choice; next supplied query omits it: no remembered choice, INT-060 state unchanged | L3-CLI-064 |
| `command_chain_next_call_has_no_edition_override` | First supplied request has an override; next has none: apply its existing labelled basis policy, never inherit previous override | L3-CLI-064 |
| `command_chain_failure_does_not_leak_selection` | Failed first request selected source A; next explicit request selects B: no reused A binding or stale request options | L3-CLI-064 |
| `command_chain_prior_result_keeps_binding` | Host retains result A while running B: A source/revision/qualification evidence unchanged; no concurrency promise | L3-CLI-064 |
| `command_chain_acquisition_failure_is_not_empty` | Injected source acquisition failure differs from acquired empty text; no successful empty report, no fabricated source | L3-CLI-065 |
| `command_chain_traversal_failure_keeps_extent` | Injected core traversal failure preserves visited/unvisited extent and incomplete inventory, never asserts no later records | L3-CLI-065 |
| `command_chain_access_failure_is_not_adoption` | Inject refused access under supplied reviewed policy: no computation claiming semantic/governing success; enforces the owner-signed INT-031 per-query/cut constraints | L3-CLI-065 |
| `command_chain_computation_and_delivery_differ` | Supplied sink fails after successful computation: command result and delivery failure stay distinguishable; no success-of-delivery claim or retry policy | L3-CLI-065 |
| `command_chain_runner_matches_direct_computation` | Same explicit request/source/context through runner handoff and direct command path: equivalent pre-render semantic data/qualifications and host stays alive | L3-CLI-066 |
| `command_chain_failure_returns_to_host` | Shared command returns injected failure through both paths; host can perform next explicit call, no process exit or silent success | L3-CLI-066 |
| `command_chain_mock_is_not_product_acceptance` | Synthetic adapters pass but real core/text paths or command policies unverified: full product acceptance remains unproven, missing paths visible | L3-CLI-067 |
| `command_chain_never_falls_back_to_stdin` | Absent explicitly supplied source or acquisition failure never reads stdin or chooses it as fallback; explicit '-' selects stdin; omitted operand/acquisition failure never does, no new error spelling/exit chosen | L3-CLI-062, L3-CLI-065 |
| `command_chain_mounted_consumes_host_routes_only` | Host supplies a routed Format-1 fragment and required non-fragment observations while another consumer gets other DATA: adapter consumes only supplied routes, no own dispatch/header reconstruction/claim on other packets; standalone host dispatch reaches same adapter with typed wrong-kind guard | L3-CLI-062, L3-CLI-026, L3-CLI-027, L3-CLI-028 |
| `command_chain_extract_flags_match_direct_policy` | Runner and direct extract receive identical --force/--strict settings plus destination/identity evidence. Existing unrelated file and unproven regular identity refuse without --force; known alias refuses with or without it; each refusal happens before truncate/write. Authorized unrelated/unproven --force arms follow the same-handle sequence. Strict listed-kind/no-finding arms use the common approved rule; strict allows informational notes alone and refuses typed bad-data class; all findings/flags retained. Outcomes/forwarded flags match before rendering | L3-CLI-064, L3-CLI-066, L3-CLI-034 |

No record-order default, strict predicate, stamp destination exception or
flex oracle is chosen. Extraction keeps ADR-0037's separate permission;
semantic/governing access stays gated by owner-signed INT-031. Test observations
are internal handoffs and outcomes, not a public report/schema design.
No test in this table has been executed against replacement code.

## Legacy independent-check batches 3/4 selected-policy follow-ups

Joey signed the recommended choices on 2026-11-28. Owner-choice holds
are removed, including Q25 A; its new independent check and development gates remain. Remaining
independent checks, source/development analysis/review and tests still apply;
no executable coverage is claimed.

| Planned case | Independent source/evidence and expected selected-policy boundary | Interpretation |
|---|---|---|
| `extension_removal_requires_explicit_reviewed_plan` | **Owner-signed F2 A:** ordinary removal refuses while extensions attach or may attach with unresolved/ambiguous/cannot-evaluate evidence, listing all possible X items (including INT-061 A G\PN); any cascade is a separately explicit revision-bound edit; unresolved attachment never becomes a guessed cascade or proof of no attachment | INT-022 |
| `equals_suggestion_needs_complete_recognized_name` | Owner-signed F3 A suggests only positively recognized full name before '=' with no colon; always keep raw/missing-delimiter finding, never apply | INT-023 |
| `packet_mapping_unusable_operands_are_not_mismatch` | Missing/Invalid/duplicate/unavailable CDT or format: cannot-evaluate with all evidence, not a mismatch; usable baseline versus unavailable historical mapping distinguished | INT-024 |
| `tspiin_condition_does_not_repair_tspin` | TDTF p. 9-81 condition reading TSPIIN; a document's CDT=TSPIN is still unknown | INT-025 |
| `duplicate_conversion_source_is_no_op` | C DCN references R AMN twice but only AMN/DMN have R-side Links-to; no fabricated EV/IPMN/BME/SMF source | INT-027 |
| `governing_umbrella_composes_approved_guards` | Complete copy with ambiguity/unreadable SRCC/unresolved takeover never adopts from completion alone; approved INT-045..053 constraints preserved | INT-028 |
| `governing_complete_is_not_takeover` | A later Complete candidate has no established valid control transition: no latest-Complete replacement of current/unresolved governing evidence | INT-028, INT-053 |
| `governing_unresolved_is_not_no_setup` | Distinguish unresolved comparison/control target from known ungoverned packet range; no guessed previous setup | INT-028, INT-031, INT-053 |
| `srcc_raw_subtypes_do_not_infer_semantics` | SHA-only/layout-only/other raw difference crossed with SRCC 0/1/unreadable: preserve claim/subtype evidence, no generic case/order equivalence, reset or automatic takeover | INT-029, INT-050 |
| `governance_assumptions_are_per_query` | Prior ambiguity opt-in or assigned description cannot silently supply next query; cut facet always blocks semantic use, Complete integrity alone follows INT-051 | INT-031, INT-032, INT-048, INT-051, INT-060 |
| `appendix_fragment_defaults_find_example_collision` | The standard's D-3 example itself has two fragments and no WFP; both default 1 with finding/ambiguous order, never infer from index or C-2 prose | INT-033 |
| `completeness_rollover_is_not_gap` | Source sequences FE/FF/00/01 on same channel retain +1 continuity; no false gap at FF->00 | INT-034, INT-011 |
| `completeness_padding_precedence_remains_open` | **Owner-signed Q25 A (precedence arm unblocked by owner Q25 A; fixture name is historical):** ';' plus suffix bytes from hex {20,00–1F,7F} retains semicolon boundary/raw bytes. A high byte is not padding; X:a plus padding keeps substantive unterminated suffix. Padding-only/no substantive attributes is Incomplete without a truncation claim; proven leading loss keeps blocking cut through a final semicolon and aggregate Ambiguous; ordinary input start proves no loss; no field-value trim | INT-034, INT-035, INT-036 |
| `completeness_integrity_is_not_source_unavailability` | Otherwise available well-framed text with core Mismatch/secondary-integrity findings is distinct from unavailable DATA bounds; no integrity-only Incomplete quarantine; resource refusal remains operational | INT-034, INT-051 |

## Legacy independent-check batch 1 follow-up fixtures

Independent review at main c682e05 (2026-11-27); these are PLANNED, not
executed tests or implementation approval. INT-005/006 amendments await
independent recheck. General link/H semantics retain their release scope;
this does not bring general resolved links forward into the bounded time view.

| Planned case | Independent premise and expected result | Interpretation |
|---|---|---|
| `definition_placeholder_arity_must_match` | Definition B-d\DLN matches B-x\DLN with the same path/one index position; differing index arity cannot match merely because placeholder letters are ignored | INT-004 |
| `definition_placeholders_do_not_equal_document_indices` | Definition placeholder equivalence never equates actual document B-1 and B-2 occurrences; preserve raw/canonical index identities independently | INT-004, INT-038 |
| `link_target_unusable_cdt_keeps_all_groups` | Same-channel CDT separately Missing, Invalid, duplicate equal/unequal, or unavailable (leading blank): R\CDLN target selection cannot-evaluate, retaining every P/B/S/Q candidate and every CDT occurrence/finding, never resolved or no-target from uncertainty | INT-005, INT-039, INT-041 |
| `link_target_fibre_channel_uses_int007` | Usable same-channel FBCHIN admits S/Q by INT-007's condition reading; explicit CDT=FBCIN retains unknown-keyword/Invalid evidence and cannot-evaluate selection, never silently repaired | INT-005, INT-007 |
| `h_test_item_ascii_match_keeps_raw` | H\TA:TeSt;G\TA:test; compares ASCII-insensitively with original bytes/locations kept; B/S TA fields do not supply a G target and high-byte look-alikes do not fold | INT-006, INT-036, INT-037 |
| `h_test_item_leading_blank_is_unavailable` | Leading blank on either H\TA or G\TA makes that value unavailable under INT-041; no trim or invented match. Other unreviewed blank-bearing forms stay unavailable rather than choosing a blank rule | INT-006, INT-041 |
| `h_test_item_duplicate_occurrences_are_ambiguous` | Multiple H\TA, multiple G\TA, or both, including identical repeats: Ambiguous with EVERY occurrence retained, never first/last or an equality winner; record qualification unchanged | INT-006, INT-039, INT-048 |
| `derived_default_labels_prose_origin` | Same-occurrence DER condition and absent DPNO: default 1 cites Table 9-11 Definition prose p. 9-167, distinguishable from table Default usage metadata; no other occurrence supplies the condition | INT-008 |

## Legacy independent-check batch 2 source follow-ups

Planned only; INT-010/015 source corrections need exact-head recheck.
These do not satisfy development-analysis or executable-test gates.

| Planned case | Source and expected boundary | Interpretation |
|---|---|---|
| `equality_spacing_cites_e6_not_e3` | 106-24R1 E-3 p. E-2 has ==; E-6 p. E-3 has spaced = =; grammar remains == and spaced form warns, no behavior change | INT-010 |
| `g106_year_wording_is_present_in_10615` | Archived 106-15 year wording/range already present; MFS 2 also in 106-05/07/13. Do not gate recognition-format source metadata on 106-17 or treat MFS 2 as new in 106-13 | INT-015 |
| `fallback_intended_basis_is_not_applied_vocabulary` | Intended fallback basis 106-13, available registry 106-24R1 baseline: retain both and availability limitation, never claim edition-specific 106-13 validation; policy code mapping unchanged | INT-017 |

## Planned historical CSDW source-word fixtures

Source: [14-edition layout matrix](research/2026-11-27-historical-csdw-layout-sources.md).
Explicit layout premises only, not automatic identification. These are planned
checks of existing requirements, not executed decoder evidence.

| Planned case | Independent word/source and expected boundary | Requirements |
|---|---|---|
| `csdw_10604_10605_no_declared_fields` | All-reserved 32-bit word; preserve all bytes and four-byte text offset; no declared CH10VER/SRCC/FRMT, even when raw bits 8/9 are set | L2-CH10-001, L2-CH10-002 |
| `csdw_10607_srcc_without_frmt` | Explicit 106-07 layout; low byte 07, SRCC 1; bit 9 is reserved, not XML; retain its raw evidence without inventing a reserved-bit refusal policy | L2-CH10-002 |
| `csdw_10609_introduces_frmt` | Explicit 106-09 layout: bit 9=1 is XML unsupported with original bytes retained; bit 8 independently SRCC, low byte 08 literal 106-09 | L2-CH10-002 |
| `csdw_zero_byte_does_not_identify_layout` | Same zero low byte supplied with different reviewed layout premises: reserved/not-declared readings retain their premise, no modern/legacy layout guessed from byte alone | L2-CH10-001, L2-CH10-002 |
| `csdw_unavailable_layout_never_guesses` | No established supported layout or contradictory supplied context: gap observable, no fabricated modern fields/empty successful text; final unavailable-context API/policy still requires review | L2-CH10-001 |
| `csdw_10611_source_layout` | Explicit 106-11 section/figure from matrix; independently set bit 9, bit 8 and low byte with all original word/body bytes retained; verify defined field positions and cited table separately from fallback policy | L2-CH10-001, L2-CH10-002 |
| `csdw_10613_source_layout` | Explicit 106-13 section/figure from matrix; independently set bit 9, bit 8 and low byte with all original word/body bytes retained; verify defined field positions and cited table separately from fallback policy | L2-CH10-001, L2-CH10-002 |
| `csdw_10615_source_layout` | Explicit 106-15 section/figure from matrix; independently set bit 9, bit 8 and low byte with all original word/body bytes retained; verify defined field positions and cited table separately from fallback policy | L2-CH10-001, L2-CH10-002 |
| `csdw_10617_source_layout` | Explicit 106-17 section/figure from matrix; independently set bit 9, bit 8 and low byte with all original word/body bytes retained; verify defined field positions and cited table separately from fallback policy | L2-CH10-001, L2-CH10-002 |
| `csdw_10619_source_layout` | Explicit 106-19 section/figure from matrix; independently set bit 9, bit 8 and low byte with all original word/body bytes retained; verify defined field positions and cited table separately from fallback policy | L2-CH10-001, L2-CH10-002 |
| `csdw_10620_source_layout` | Explicit 106-20 section/figure from matrix; independently set bit 9, bit 8 and low byte with all original word/body bytes retained; verify defined field positions and cited table separately from fallback policy | L2-CH10-001, L2-CH10-002 |
| `csdw_10622_source_layout` | Explicit 106-22 section/figure from matrix; independently set bit 9, bit 8 and low byte with all original word/body bytes retained; verify defined field positions and cited table separately from fallback policy | L2-CH10-001, L2-CH10-002 |
| `csdw_10623_source_layout` | Explicit 106-23 section/figure from matrix; independently set bit 9, bit 8 and low byte with all original word/body bytes retained; verify defined field positions and cited table separately from fallback policy | L2-CH10-001, L2-CH10-002 |
| `csdw_10624_source_layout` | Explicit 106-24 section/figure from matrix; independently set bit 9, bit 8 and low byte with all original word/body bytes retained; verify defined field positions and cited table separately from fallback policy | L2-CH10-001, L2-CH10-002 |
| `csdw_10624r1_source_layout` | Explicit 106-24R1 section/figure from matrix; independently set bit 9, bit 8 and low byte with all original word/body bytes retained; verify defined field positions and cited table separately from fallback policy | L2-CH10-001, L2-CH10-002 |

No record-governance, checksum trust upgrade, layout default or new historical
validator severity is inferred. Present-but-reserved code values and absent
fields stay distinguishable. Baseline L3 bit cases remain unchanged.

## INT-035/036/037 development-analysis edge oracles

Source: [written analysis](research/2026-11-28-framing-character-development-analysis.md).
PLANNED only; analysis owner/independent review and executable tests remain.
These framing fixtures do not assert private/unknown code-name validity.

| Planned case | Independent input/expected boundary | Interpretation |
|---|---|---|
| `analysis_empty_run_spans` | ';;' -> [0,2); CR LF ';' -> [0,3); ';;COMMENT:x;;;' -> [0,2) and [12,14); silent EOF layout is outside the runs | INT-035, INT-036 |
| `analysis_first_colon_only` | COMMENT:a:b; has one raw value a:b, never two items or code-like recovery | INT-035 |
| `analysis_empty_fields_distinct` | :v; gives empty-name finding; X:; gives present zero-length raw value; neither an empty-region run | INT-035 |
| `analysis_control_name_span` | CO CR LF MMENT:a TAB b; raw name [0,9), value [10,13); projected COMMENT/ab; internal omission [2,4), no raw rewrite | INT-036 |
| `analysis_nul_per_field` | Two NULs per name/value: one NUL finding per field, listing every location, plus internal-name omission if independently applicable | INT-036 |
| `analysis_numeric_projection_is_not_repair` | NSB raw 0 CR LF 4 remains Invalid; never repaired through character projection, extraction unchanged | INT-036, INT-055 |
| `analysis_marker_every_split` | Every 2/3/4-byte full signature from the analysis at every fragment partition after CSDW slicing: same true-offset-zero finding, all bytes preserved | INT-037 |
| `analysis_partial_marker_not_full` | EF, EF BB, FE alone, or 00 00 FE at EOF is not a full marker; existing high-byte/framing evidence remains | INT-037 |
| `analysis_marker_after_layout_is_midstream` | CR LF or NUL then EF BB BF is midstream; no skipped padding, BOM stripping or encoding guess | INT-037 |
| `analysis_high_byte_sweep` | Every 80–FF byte in name/value, including B0 and valid-looking UTF-8 sequences, prevents silently usable affected fields | INT-037 |

The analysis additionally requires bounded property/fuzz cases and a distinct
source-qualified fixture for EACH of 106-04/05/07/09/11/13/15/17/19/20/22/23/
24/24R1. Shared test bodies cannot erase edition evidence. No real recording
was accessed; local-only sample checks are still outstanding where available.

## INT-038/039/040 development-analysis edge oracles

Source: [written analysis](research/2026-11-28-index-duplicate-counter-development-analysis.md).
PLANNED only; written analysis owner/independent review and intensive
executable tests remain pending. These independently specified boundaries
supplement the existing approved-interpretation cases, not executed coverage.

| Planned case | Independent input/expected boundary | Interpretation |
|---|---|---|
| `index_identity_40_digit_pair` | 01234567890123456789012345678901234567890 and its 40-digit nonzero suffix have equal canonical identity/hash input, original spellings retained; no integer conversion | INT-038 |
| `index_zero_is_not_field_value_zero` | Arbitrarily long all-zero INDEX -> identity 0; an eleven-digit numeric FIELD VALUE is Invalid under INT-055; neither grants range validity | INT-038 |
| `index_collision_lists_every_occurrence` | D-1\MN-1-2, D-01\MN-01-02, D-1\MN-1-2: one collision group listing all three, not just two distinct spellings | INT-038 |
| `index_case_only_has_no_spelling_collision` | Case-only key variants with the same digit tuples: no INT-038 collision finding, scalar duplicate still Ambiguous | INT-038 |
| `index_internal_control_keeps_raw_tuple` | R-1 CR LF 0\TK1-1 and R-10\TK1-1: projected identity 10, internal-omission evidence and distinct raw-token collision retained, no projection-based loss of witnesses | INT-038 |
| `scalar_valid_invalid_candidates_stay_ambiguous` | Known scalar with one usable and one Invalid/empty occurrence: both kept, no winner/default | INT-039 |
| `scalar_comment_and_group_com_differ` | Repeated standalone COMMENT is collection; repeated G/R COM same slot is Ambiguous even when equal; raw all retained | INT-039 |
| `scalar_fragment_scope_is_one_document` | Explicit same-channel +1 modulo 256, same FRMT/RCCVER and no closure: cross-fragment G PN duplicates; separate records do not | INT-039 |
| `counter_invalid_payload_is_member_evidence` | N=2 with WP identities 1/2 and first payload FF: structural set {1,2}, Invalid value remains; counter [0,18), value [16,17), bad payload [33,34) | INT-040 |
| `counter_duplicate_properties_count_once` | WP/WI properties and exact/01 member repeats contribute one e identity; field ambiguity and collision findings still retained | INT-040 |
| `counter_malformed_parent_blocks_compatible_scopes` | D-1\WP-1-1-x-2 blocks all compatible (1,1,1,m), even known set {1,2} already matches N=2; x=2 parent scope unaffected by this evidence | INT-040 |
| `counter_wrong_arity_never_guesses_alignment` | Matching literal family with wrong index arity stays rule-level unassigned; only unambiguous readable positions constrain compatible scopes | INT-040 |
| `counter_unrelated_damage_is_not_member` | Unrelated literal family, code-like value text, unframed suffix and X original-path text are not D-member evidence; other findings remain | INT-040 |
| `counter_bad_parent_does_not_hide_child` | m=0/00 is one scope; parent-domain violation/unusable MML does not suppress a child with explicit indices and independently usable MNF | INT-040 |
| `counter_range_is_not_mfs` | Older printed MFS 1 is not baseline MNF range 1–8; applied baseline versus intended historical basis is labelled, unsupported edition mappings stay unavailable | INT-040 |
| `counter_cardinality_is_not_contiguity` | N=2 set {1,3} and N=3 set {1,3} both fail their respective checks; no max/count substitution, no eager array allocation | INT-040 |

The invalid-payload oracle uses the existing corrected byte sequence:
`D-1\MNF\N-1-1-1:2;D-1\WP-1-1-1-1:<FF>;D-1\WP-1-1-1-2:4;`
(`<FF>` is one raw byte). Counter [0,18), counter value [16,17), invalid
payload [33,34). Future implementations must assert these independent spans.
Each historical fixture uses its own cited table metadata; early maximum
field sizes are not modern ranges. Canonical identity, duplicate ambiguity,
structural membership and all record qualifications remain separate results.

## Owner-sign-off acceptance edges — 2026-11-28

Source: [verbatim owner sign-offs](research/2026-11-28-interpretation-owner-signoffs.md).
PLANNED, not executed tests. Owner-choice holds for these selected policies
are removed; final rule checks CONCUR at #87 e0a0cce; source freshness, written
analysis/review and executable tests still gate implementation. INT-022 is
0.5 scope; none of these cases selects rendered output. Q25 A now separately signs
INT-034 padding-only/leading-loss precedence, with its independent check pending.

| Planned case | Independently specified expected result | Interpretation |
|---|---|---|
| `owner_f2_attached_delete_refuses` | Reviewed unique attachment, or unresolved/ambiguous/cannot-evaluate possible attachment to exact original: ordinary deletion refuses and lists attached/possibly attached X items; a cascade needs a separate explicit edit, never implicit removal | INT-022 |
| `owner_f3_equals_suggestion_guard` | Recognized complete G\DSI\N=18; with no colon: preserve bytes/missing-delimiter finding and offer only the suggested delimiter edit; unknown/incomplete name or any colon: no such suggestion | INT-023 |
| `owner_packet_mapping_unusable_is_not_mismatch` | Missing/Invalid/Ambiguous/unavailable CDT or format, and unsupported edition mapping separately: cannot-evaluate with evidence, no mismatch; reviewed usable map supplies expected code, never arithmetic family guessing | INT-024 |
| `owner_tspiin_condition_not_user_repair` | Standard TSPIN condition is read TSPIIN; literal document CDT=TSPIN remains unknown; no field-byte rewrite | INT-025 |
| `owner_polarity_condition_is_channel_local` | RPS/MFF RPS use same recorder/channel R CDT; missing/bad/duplicate/unavailable local CDT never borrows another channel and remains cannot-evaluate | INT-026 |
| `owner_conversion_edge_not_item_dedup` | Repeated definition link yields one graph edge; repeated raw document items remain multiple with their duplicate/ambiguity findings | INT-027 |
| `governing_sealing_time_packet_is_inclusive` | Validly taking-control candidate closes at trusted time packet P: new setup governs P and later eligible packets, never packets between its last fragment and P retroactively | INT-028 |
| `governing_unresolved_needs_independent_rule` | Complete candidate with unresolved target: no restoration without separately reviewed independent takeover proof; supplied such proof restores target and preserves earlier uncertainty/findings | INT-028, INT-053 |
| `governing_f7_indeterminate_is_not_independent_veto` | Otherwise independently valid takeover with unverified event: no block/delay solely for F7; keep its indeterminate finding; no claim of event presence/absence | INT-028, INT-030 |
| `governing_nonzero_legacy_source_has_no_guessed_partition` | Admitted legacy distributed/nonzero setup: retain/parse candidate but withhold governing source assignment until a reviewed partition rule exists | INT-028 |
| `owner_inventory_first_fragment_takeover_closure` | A starts, B starts, B closes, A closes: inventory A then B; eligibility/governing follows separately established closure/takeover, never inventory position | INT-028 |
| `owner_descriptive_change_is_still_raw_change` | Independently proven comment/date-only change: not semantic configuration change, but raw subtype and SRCC discrepancy remain; unknown/private changes never assumed descriptive | INT-029, INT-050 |
| `owner_ambiguous_access_per_query_only` | Non-cut Ambiguous text: default refuses description; explicit labelled opt-in permits assumption access for that query only; next query without opt-in refuses; no automatic takeover or ambiguity upgrade | INT-031 |
| `owner_cut_refuses_even_ambiguity_opt_in` | Incomplete or Ambiguous with cut facet: per-query acceptance never makes it usable/governing; diagnostic inventory and separately approved raw extract remain possible | INT-031, INT-048 |
| `owner_ungoverned_assignment_is_not_session_default` | Explicit assigned description on one ungoverned-range query is labelled assumption; next query gets no remembered setup and no declared-governing upgrade | INT-032 |
| `owner_fragment_default_collision_not_index_order` | Two WFP positions default 1: retain defaults and collision/ambiguous ordering; explicit caller order labelled, never inferred from fragment indices or Appendix 9-C prose | INT-033 |
| `owner_provisional_snapshot_version_and_invalidation` | Explicit opt-in snapshot S1 binds exact extent/version; later text yields S2 and visible S1 invalidation without erasing S1 evidence or automatically governing; known cut never usable | INT-034 |
| `owner_boundary_padding_preserves_raw_bytes` | Otherwise complete substantive text ending ';' followed by each SP/00–1F/7F suffix remains terminated, bytes retained; high bytes are not layout; Q25 A classifies padding-only as Incomplete/no loss claim and retains proven leading cut; ordinary input start proves no loss | INT-034 |
| `owner_sequence_rollover_is_continuity` | Same channel FF -> 00 with matching FRMT/RCCVER and no layered closure joins; missing/other sequence not repaired and other-channel traffic never breaks continuity | INT-011, INT-034 |
| `owner_available_integrity_failure_is_not_cut` | Admitted available DATA with core Mismatch/NotChecked/secondary readings remains available text, findings unchanged; independently missing required run bytes add incomplete-source facet, operational refusal alone does not invent a cut | INT-034, INT-051 |

## ADR-0038 selected policy acceptance — 2026-11-28

Owner-choice holds below are removed. All cases are PLANNED, not measured
or executed; [policy/pin record](research/2026-11-28-owner-policy-signoffs.md).
Q26 report rendering now follows ADR-0039; Q25 precedence is recorded in
INT-034 with FINAL CONCUR at #91 5f4f3c4. Concrete development/verification remains planned.
Q18 class membership beyond its six listed kinds still needs an explicit
reviewed mapping and independent fixtures before additional-member refusal
coverage can be claimed. Current rows do not invent such members from severity.

| Planned test | Independent expected result / boundary | Requirements |
|---|---|---|
| `pattern_star_is_one_whole_part_or_index` | Reviewed R-*\TK1-* matches complete recorder/channel indices only; R-1\*-3 matches one reviewed literal part, never a multi-part EV/PM path, value text or arbitrary suffix | L3-VIEW-058, L3-VIEW-071 |
| `pattern_partial_star_and_regex_not_supported` | R-1\T*-3 and regex-like index expressions never execute partial/regex matching; unsupported patterns visibly refuse/unavailable, no guessed glob expansion | L3-VIEW-071 |
| `exact_unknown_vendor_raw_lookup_is_not_registry_limited` | Retained private/unknown literal name returns all raw occurrences and locations by exact lookup; wildcard scope cannot remove raw access or invent a typed value/private index grammar | L3-VIEW-071, L3-VIEW-001 |
| `pattern_private_vocabulary_remains_unavailable` | Missing reviewed pattern metadata cannot be supplied by private source presence; unavailable matching stays distinct from a reviewed empty candidate collection | L3-VIEW-062, L3-VIEW-071 |
| `host_non_setup_steady_state_zero_allocations` | Warm both standalone and mounted paths, then count a bounded sustained non-setup packet stream including pool/refill/bookkeeping outside timer: every expected observation consumed, zero host per-packet allocations; setup/rendering measured separately | L3-PERF-009 |
| `host_allocation_counter_detects_bookkeeping_regression` | Inject one real allocation in steady-state non-setup host bookkeeping: acceptance fails even if parser allocations remain zero and output bytes agree | L3-PERF-009 |
| `file_order_interleaved_finalization_is_first_fragment` | Supplied A-first/B-second independent runs finish B then A: inventory A,B; closure/takeover separately evaluated, no channel sort or backdating | L3-CLI-040 |
| `flex_exact_original_pin_and_hashes` | Qualification evidence matches original repository, exact revision and inspected source hashes; wrong revision/fork is a known difference, never silently substituted or accepted as the pinned vector oracle | L3-SUM-014, L3-SUM-015 |
| `channel_rule_edition_evidence_is_passed_through` | Supplied core evidence for 106-11 nonzero setup, 106-13/15 channel-0 non-Format-1 and 106-17+ channel-0 other than Format-1/4 remains unchanged in tmats; no second classifier or threshold inferred from G106 | L3-CH10-026 |
| `mixture_only_complete_records_contribute` | Complete ASCII plus Incomplete XML in explicit S: qualified XML evidence without mixture finding; changing only XML qualification to Complete produces finding, no XML body parsing | L3-CH10-022, L3-CH10-023 |
| `extract_strict_bad_data_not_severity_inference` | Each listed bad-data category/class refuses strict and default extracts exactly with findings; unrelated informational note alone permits strict extraction. Message text/severity alone never establishes class membership | L3-CLI-060, L3-CLI-061 |
| `retention_cap_leaves_verified_reader_reserve` | Independently supplied B/R progress evidence: retain at most B-R distinct pool credits; shared references never double-count bytes or falsely free an owned credit; next core admission retains its verified reserve | L3-ROB-016, L3-ROB-019 |
| `retention_over_cap_fails_run_reader_continues` | One additional retained credit exceeds cap: typed assembly-budget error with run/channel/spans, truthful release/disposition and retained diagnostic source; next core-routed observations still consumed, no silent eviction, fake Complete/EOF or intact-run retry | L3-ROB-016, L3-ROB-018, L3-CLI-042 |
| `retention_unverified_reserve_is_not_progress_proof` | Unknown required core credit/caller-held ownership prevents an unconditional progress claim; supplied synthetic reserve alone is not actual API proof or permission to exclude those owners | L3-ROB-016, L3-ROB-019 |
| `stdin_explicit_dash_only` | Explicit '-' selects supplied stdin source; no operands, failed path acquisition or absence of a filesystem path never reads stdin automatically; runner/direct paths agree before rendering | L3-CLI-062, L3-CLI-065 |
| `benchmark_uses_ignored_ci_without_criterion` | Restructured manifest has no Criterion; bounded informational ignored-test benchmark job records workload/toolchain/timing plus untimed allocation/copy evidence, not a new throughput acceptance gate or dependency | L3-PERF-003, L3-PERF-005, L3-PERF-009 |

## INT-041/042/044 development-analysis edge oracles

Source: [written analysis](research/2026-11-28-effective-presence-basis-development-analysis.md).
PLANNED, not executed tests. Owner/independent analysis review and intensive
executable checks remain; Q25 precedence is handled separately, and Q26
report policy is now recorded in ADR-0039.
Bindings/scope/conditions are supplied and reviewed explicitly; these rows
do not invent historical registry metadata, reference scope or governance.

| Planned case | Independent expected boundary | Interpretation |
|---|---|---|
| `availability_default_needs_absence_rule_and_true_condition` | No raw TFMT, known baseline binding/TIMEIN applicability: Defaulted A with source/citation/condition, no input value span or inserted item; remove any prerequisite and never invent A | INT-041 |
| `availability_missing_false_is_not_unknown` | Actually absent supported TFMT with false versus unknown applicability: both Missing with DISTINCT reasons; unknown CDT operand retains condition finding, not confirmed false | INT-041 |
| `availability_present_empty_never_defaults` | One empty or control-only candidate remains present; local validity follows definition, never default by empty-string truthiness | INT-041 |
| `availability_proved_bad_or_duplicate_survives_gap` | Known scalar multiplicity remains Ambiguous; demonstrated high-byte/range failure remains visible when another rule is unavailable; no candidate winner | INT-041 |
| `availability_numeric_raw_controls_not_repaired` | NSB raw 0 CR LF 4 is Invalid under INT-055, retaining control/source findings; no projection-first Explicit 4 | INT-041 |
| `availability_tfmt_default_keeps_packet_conflict` | Evaluated TIMEIN/TTF=2 plus absent TFMT: baseline Defaulted A with existing inconsistency finding and unchanged packet evidence; no association inference | INT-041 |
| `availability_damaged_name_blocks_default` | Reviewed baseline R-1\TFMT-1 binding and evaluated TIMEIN applicability, no exact match, but R-1\TFMT\xFF-1:a; (one raw FF byte): compatible damaged-name presence is cannot-evaluate. The effective read is unavailable/cannot-evaluate with raw item/locations retained, neither Defaulted A nor plain Missing; never repair the name to create an explicit field | INT-041, INT-042 |
| `presence_empty_invalid_duplicate_values_still_true` | Available exact T-1\ID items with empty/Invalid/duplicate values: true presence; all candidate/field/record findings remain and no usable singular ID inferred | INT-042 |
| `presence_exact_with_related_damage_still_true` | One actual exact T-1\ID plus compatible damaged name: true, both evidence sets kept; reversing order changes no predicate answer or value-validity claim | INT-042 |
| `presence_default_is_not_physical` | Supplied reviewed default result with no raw item does not make that field physically specified; no synthetic input span or requirement satisfaction | INT-042 |
| `presence_high_byte_has_independent_witnesses` | I<FF>D -> ID via empty replacement; I<FF> -> ID via D; T-1<FF>D can hide separator plus I; D<FF>I has no ID witness; all remain diagnostic, not keys | INT-042 |
| `presence_terminated_literal_has_no_unread_tail` | Terminated T-1\I:a; without damage is not ID; unterminated T-1\I at EOF can complete to ID and blocks false without becoming a lookup item | INT-042 |
| `presence_scope_projection_and_record_are_distinct` | t-01\id projects to T-1\ID with normalization information; contradictory recorder occurrence/other setup never contributes; valid cross-fragment scope stays one document | INT-042 |
| `presence_marker_block_precedes_zero_match` | INT-037 leading marker blocks semantic lookup: cannot-evaluate with raw bytes/reason, never false/default from zero usable matches | INT-042 |
| `presence_unknown_negation_and_inequality` | Cannot-evaluate remains so under is-not-specified; missing/unusable value operands make both equality and inequality cannot-evaluate, never guessed Boolean | INT-042 |
| `fallback_counts_items_not_years_or_candidates` | G106 24/zz/24 in one document -> n=3, ambiguous reason; one 24 with two edition candidates is one source item, not duplicate; separate records never combine | INT-044 |
| `fallback_override_never_upgrades_qualified_record` | Valid override changes basis only; duplicate source/assembly Ambiguous and any cut stay. Unusable override blocks selection rather than falling through | INT-044 |
| `fallback_wire_literal_policy_and_eligibility` | Supplied eligible Format 1 0D/0E yields unchanged policy targets, literal readings kept; wrong/absent layout or other code-space operand never supplies eligibility | INT-044 |
| `fallback_intended_edition_is_not_coverage` | Intended policy 106-20 plus current baseline vocabulary retains actual coverage limit; unavailable field binding cannot claim a historical default/Invalid result or implicit alias | INT-044 |
| `fallback_basis_survives_default_and_invalid` | Baseline default and separately demonstrated Invalid under ambiguous-G106 fallback both carry ambiguous reason/count and applied-basis metadata; no report-shape or severity choice | INT-041, INT-044 |
| `historical_usage_is_not_silently_backfilled` | 04–13 inspected TFMT/AN1 rows versus later explicit usage metadata: each fixture uses its own source/binding; no inherited modern default/condition or old field-size-as-range | INT-041, INT-042, INT-044 |

The source note fixes literal byte-span/completion witnesses independently.
Every touched archived edition needs its own source-qualified fixture, with
unsupported coverage explicit. No annotated standard/handbook sample is a
whole-file conformance oracle. No real recording was accessed in this cloud
analysis; local-only checks remain outstanding where available.

## Real Chapter 10 recordings (local use only)

Order of work (owner decision, 2026-09-25): the irig106.org vendor sample
recordings first, because they cover the widest spread of vendor quirks,
then the owner's program files as they become available.

- **IRIG 106 sample data files** — laboratory recordings from recorder vendors,
  each with a TMATS setup record:
  <https://www.irig106.org/wiki/sample_data_files>.
  The site warns that they "provide examples of varying degrees of Chapter 10
  compliance", which makes them useful for non-conforming-input handling.
  Listed at the time of writing: Calculex (PCM at several rates, video and
  voice), Enertec (video and 1553), Heim (PCM/video/analog/UART, 16PP194 bus,
  CAN bus, Chapter 7 PCM), JDA Systems, NetAcquire, Smartronix (PCM with 1553,
  HD and SD video), Sypris, TTC (eight 1553 channels, ARINC 429), Wideband
  Systems (Ethernet, UART, video, PCM, 1553, ARINC 429, discrete), Wyle, and
  files synthesized with Data Bus Tools' FLIDAS.
- **IRIG 106 synthetic data file** — a fully synthesized flight recording
  (valid TMATS, time, video, and a 1553 navigation channel, about 150 MB) with
  an interface control document:
  <https://www.irig106.org/wiki/synthetic_data_files>.
- Some downloads require an irig106.org account.
- **Program recordings** supplied by the project owner, as they become
  available, under the same local-only rule.

## Test oracles (reference implementations)

- **`irig106lib`** — the long-standing C library for reading, writing, and
  parsing IRIG 106 data, including TMATS (BSD-3-Clause):
  <https://github.com/bbaggerman/irig106lib>.
- **`irig106utils`** — command-line utilities built on it, including
  `idmptmat` ("Read and print out the TMATS record in various formats"):
  <https://github.com/bbaggerman/irig106utils>.
- **irig106.org software downloads** (Windows builds, including
  `igDisplayTMATS`): <https://www.irig106.org/wiki/software_download>.

- **`idmptmat` in the standard itself** — the complete `idmptmat` source is
  published in the RCC 123 Chapter 10 Programmers' Handbook (Appendix C,
  "Example Program – Decode TMATS", in 123-16; Appendix D in 123-20), mirrored
  in `rcc-106-standards`.
- **`libirig106`** — a later C library whose README says it is "Forked from
  Bob Baggerman's `irig106lib`" (Chapter 10/11 parsing and generation; last
  pushed 2022-02): <https://github.com/atac/libirig106>. GitHub does not
  detect its license; confirm the license file before relying on it.
- **`igDisplayTMATS` 3.1** (Qt GUI, October 2022) — raw, summary, and tree
  views plus signature checks. Only Windows binaries are published; its
  source is not in `irig106lib`, `irig106utils`, or the retired SourceForge
  repository.

Planned use: differential tests that compare this library's reading of each
sample file's TMATS with `idmptmat`'s output, recording the exact upstream
commit or version compared against. Comparison is semantic (channels, types,
data sources, hierarchy), not textual. A disagreement is investigated against
Chapter 9, not assumed to be our bug or theirs.

## Known defects in the reference tools, and the tests that guard `tmats`

Every defect found in a reference tool becomes a named regression test in
this project, built from a synthesized fixture, so the same mistake cannot
reach `tmats` (UC-17). Found so far by reading the source (upstream
`master`, 2026-09-25):

| # | Tool | Defect | Guarding test for `tmats` |
|---|------|--------|---------------------------|
| D1 | `idmptmat` (`vDumpChannel`, `vDumpTree`) | The loop over G data sources advances with `psuFirstGDataSource->psuNext` instead of the current node's `psuNext`, so any TMATS with **two or more** `G\DSI-n` repeats the second data source forever (the program never ends). | Summary and tree output for documents with 1, 2, 3, and many data sources terminate and list each source exactly once, in order. |
| D2 | `idmptmat` | Reads only the **first packet** of a Chapter 10 file; later setup records (configuration changes) are never shown, and standalone TMATS files are not accepted. | A recording with several setup records reports each one; a plain TMATS text file is accepted. |
| D3 | `idmptmat` | Its signature option is compiled out with the comment `// SIGNATURE GENERATION ISN'T CORRECT. FIX LATER.` | `G\SHA` compute/verify tested against independently computed SHA-256 vectors (UC-15), not against another tool's output. |
| D4 | `idmptmat` tree view | Labels the channel type `(R-x\DST-n)` even when the value came from `R-x\CDT-n` (`irig106lib` stores both in one field). | Every displayed value is labelled with the code name it was actually read from. |
| D5 | `irig106lib` `enI106_Tmats_IRIG_Signature` | When `G\SHA` is the **last** item, it hashes `length − start` bytes from the beginning instead of the `start` bytes before `G\SHA`, producing a wrong digest. | `G\SHA` placed first, in the middle, last, and absent all give the digest defined by Chapter 6 §6.2.3.11 f. |
| D6 | `irig106lib` `enI106_Tmats_IRIG_Signature` | Finds `G\SHA` by case-insensitive substring search over the whole text, so the characters `G\SHA` inside another item's value (for example a comment) are treated as the checksum item. | Only a real `G\SHA` code name is excluded; the same text inside a value is hashed. |
| D7 | `irig106lib` flex signature | The R-group exclusion list names `MPOC4` twice and omits `DPOC4`, although `R-x\DPOC4` exists in 106-23 Chapter 9 — almost certainly a typo, so `DPOC4` changes the flex signature while `DPOC1`–`DPOC3` do not. | Our flex signature reproduces `irig106lib`'s behaviour exactly, because its only purpose is compatibility; the test pins the quirk with a `DPOC4` fixture and says so, rather than fixing it silently. |

Open items: `irig106lib` comments that `R-x\DST-n` existed only in 106-04 and
was replaced by `R-x\CDT-n`; confirm against the 106-04 and 106-05 Chapter 9
in the archive and record it in the edition deltas.

## Required acceptance tests

Tests that must exist, by name, before the requirement they verify can be
shown as Implemented in `docs/TRACE-MATRIX.md`. They are specified here
during the design phase and written with the code they test; no placeholder
or ignored test is added in the meantime, because a marker on it would make
the trace matrix report the requirement as verified.

### `multi_packet_setup_record_is_assembled`

Verifies that one setup record spanning several consecutive packets is
assembled into one complete record ("A single setup record may span multiple
consecutive packets", Chapter 11 §11.2.7.2, 106-24R1). Requirements:
L1-CH10-001, L1-CH10-004, L1-CH10-006, L1-CLI-003, L1-CLI-008, L1-SUM-001.
Pictured in
`docs/diagrams/setup-record-assembly.svg`.

*Input* — a synthesized recording, in file order:

1. A PCM packet on channel 3.
2. Data type `0x01`, channel `0x0000`, sequence number `0xFE`, no secondary
   header, CSDW with FRMT = 0 (ASCII), SRCC = 0, RCCVER = `0x0E`; TMATS text
   fragment 1 is the first part of a document that includes a correct `G\SHA`
   for the whole document, split in the middle of an attribute.
3. Data type `0x01`, channel `0x0000`, sequence number `0xFF`, **with** a
   secondary header (packet-flags bit 7 = 1), the same CSDW, fragment 2, 3
   bytes of `0x00` filler, and a 16-bit data checksum (flags bits 1–0 = 10).
4. Data type `0x01`, channel `0x0000`, sequence number `0x00` (rollover),
   the same CSDW, fragment 3, `0xFF` filler, and an 8-bit data checksum.
5. A PCM packet on channel 3.

Every header and secondary-header checksum is correct.

*Expected*:

- Exactly one complete setup record, whose TMATS body is byte-for-byte
  fragment 1 + fragment 2 + fragment 3 — no header, secondary header, CSDW,
  filler, or checksum bytes.
- One CSDW summary: ASCII, no configuration change, RCCVER `0x0E` read as
  "RCC 106-22" with the note "unchanged through 106-24R1" (ADR-0032).
- The provenance map sends a body offset inside fragment 2 to packet 3 and
  the right offset within it, and an attribute split across fragments 1 and 2
  to both packets.
- The document parses without diagnostics; the attribute split across the
  packet boundary is one attribute.
- `G\SHA` verifies as a match over the assembled body.
- `tmats` reports one setup record, not three.

*Companion cases* (same fixture builder): the same record with a sequence gap
(`0xFE`, `0x00`) is reported as two incomplete records with a diagnostic;
fragment 2 with a different RCCVER stops that channel's joining and is reported;
a SAME-CHANNEL PCM packet between fragments 1 and 2 stops that join,
while an OTHER-channel PCM packet is provenance/information only and never
stops it; completeness still needs INT-034; a corrupted header
checksum on fragment 2 is reported and the record is not assembled from
untrusted lengths.

### `appendix_9c_example_reports_suspected_semicolons`

Verifies that the standard's own code-name example (106-24R1 Appendix 9-C) is
read without loss and that its 18 colon-for-semicolon errata are each
reported. Requirements: L1-READ-001, L1-READ-006, L1-READ-007. **Suspect**
(owner, 2026-09-26): the finding and the diagnostic are held in doubt until
checked against real recordings (`docs/ROADMAP.md`, S1).

*Input* — the Appendix 9-C code-name example transcribed verbatim from the
archived 106-24R1 Chapter 9 (Distribution A), including page C-8.

*Expected*:

- Exactly 18 "possible `;` typed as `:`" warnings, one at each of these
  attributes, each with a suggested edit replacing the colon after the value
  by `;`:
  `D-1\MML\N-1-1`, `D-1\MNF\N-1-1-1`, `D-1\MNF\N-1-1-2`, `D-1\MML\N-1-2`,
  `D-1\MNF\N-1-2-1`, `D-1\MML\N-1-3`, `D-1\MNF\N-1-3-1` to
  `D-1\MNF\N-1-3-6`, `D-1\MML\N-1-4`, `D-1\MNF\N-1-4-1`, `D-2\MML\N-1-1`,
  `D-2\MNF\N-1-1-1`, `D-2\MML\N-1-2`, `D-2\MNF\N-1-2-1`.
- Correctly delimited D-3/D-4 code-name excerpts produce no targeted
  suspect-colon warning. Preserve any other findings from literal example
  text/prose separately; this is not a blanket no-diagnostics/conformance claim.
- Writing the document back reproduces the input bytes exactly.
- A separately authorized later general-edit test can apply the 18 explicit
  proposals and check the resulting separated fields; the 0.1 detector itself
  never applies suggestions or declares the literal example conforming.

*Companion cases*: `C-1\DPA:A?B:C;` and the Appendix 9-E expressions
produce no such warning; a value ending `: G\COM:x;` is reported (the G group
has no occurrence index).

### `appendix_9c_channel_2_trace`

Verifies the path of `docs/TMATS-IN-CHAPTER-10.md` section 7: from channel
ID 2 of the standard's own example to measurement 82AJ01 in engineering
units, with every state. Requirements: L1-VIEW-002, L1-VIEW-004,
L1-VIEW-006, L1-VAL-005, L1-EDN-002, L1-SUM-002.

*Input* — the Appendix 9-C code-name example transcribed verbatim from the
archived 106-24R1 Chapter 9 (the same fixture as
`appendix_9c_example_reports_suspected_semicolons`).

*Expected*:

- The channel view for channel ID 2 gives `R-1\CDT-1` PCMIN, `R-1\DSI-1`,
  and `R-1\CDLN-1` `PCM1` as explicit; `R-1\CHE-1`, `R-1\PDTF-1`, and
  `R-1\PDP-1` as missing, each with a required-attribute finding.
- The format link resolves to `P-2` only; the view gives exactly the values
  of section 7.1 (bit rate 2,000,000, 10-bit words with words 121 and 122 of
  6 and 4 bits, 64 × 277-word minor frames, the 30-bit pattern, the ID
  counter).
- The measurement link resolves to `D-3`; 82AJ01 has one location with two
  fragments at words 113 and 121, frame 5, frame interval 32, full words;
  both transfer orders defaulted to msb first; both positions defaulted to
  1, with a finding and an ambiguous fragment order (INT-033).
- The conversion link resolves to `C-7`, with every value explicit.
- `G\106` missing is reported; the validation basis is a fallback to
  106-24R1, labelled; `G\SHA` is absent (not an error).
- No channel has `R-x\CDT-n` TIMEIN.

### `setup_record_completeness_outcomes`

Verifies the three outcomes of ADR-0031 and INT-034, one case each, all on
channel `0x0000` with an ASCII CSDW. Requirements: L1-CH10-004,
L1-CH10-008.

| Case | Input | Expected |
|------|-------|----------|
| a | a record in two fragments ending with `;`, then a SAME-CHANNEL time packet (channel conformance checked separately) | complete, "followed by another packet" |
| b | the same record as the last packets of the input | complete, "end of input" (labelled) |
| c | the same record with its last fragment cut inside an attribute, at the end of input | incomplete; no description |
| d | three fragments with the middle one missing (sequence jumps by 2) and an attribute split across the gap | incomplete; no description |
| e | two complete records back to back, sequence continuous, same CSDW, the second starting again with `G\106` | ambiguous; the split point and both readings reported; neither governs by default |
| f | a sequence gap where the text before it ends with `;` | ambiguous |

### `srcc_compares_configuration_not_bytes`

Verifies INT-029's comparison. Each case is a governing record followed by
a second complete record. Requirements: L1-CH10-008, L1-READ-004.

| Case | Second record | Expected |
|------|---------------|----------|
| a | the same attributes in another order, SRCC 0 | same configuration; information only |
| b | the same attributes with keywords in lower case, SRCC 0 | same configuration; information only |
| c | one value changed, SRCC 0 | changed configuration; unannounced-change finding |
| d | identical bytes, SRCC 1 | identical; change-bit-with-nothing-changed finding |
| e | reordered only, SRCC 1 | same configuration; change-bit-with-nothing-changed finding |
| f | one malformed attribute added, SRCC 0 | not comparable; textual differences; no SRCC finding |

## Planned content-routing batch 23

These are acceptance specifications for L2-CLI-005/006, not executable
coverage. ADR-0019 supplies the retained selector; ADR-0036 supplies the
core-only reader boundary. Chapter 11 (106-24R1) section 11.2.1.1 a p. 11-5
and section 11.1 p. 11-1 supply initial sync bytes 25 EB (hex).
All buffers here are bounded synthetic inputs. Router spies observe calls,
forwarded bytes and failures; they do not implement packet finding.

| Planned test | Independent input and expected boundary result | Requirements |
|---|---|---|
| `input_sync_prefix_routes_recording` | 25 EB followed by arbitrary bytes selects core exactly once; no text-reader call; no claim that core accepts it | L3-CLI-014, L3-CLI-017 |
| `input_reversed_sync_routes_text` | EB 25 is not the source-derived prefix; unchanged bytes reach text reader, not core | L3-CLI-014, L3-CLI-018 |
| `input_sync_only_does_not_prove_packet` | Exactly 25 EB selects recording; injected core rejection is preserved, not converted to text success | L3-CLI-014, L3-CLI-019 |
| `input_empty_source_is_not_acquisition_failure` | Successful zero bytes select text with no invented document; failed acquisition stays failure and invokes neither consumer | L3-CLI-015, L3-CLI-019 |
| `input_one_byte_never_fabricates_sync` | Each of 256 one-byte sources selects text; no padded byte or out-of-range read | L3-CLI-015 |
| `input_prefix_sweep_has_one_recording_pair` | Every two-byte prefix, independently enumerated: only literal 25 EB selects recording; exact unchanged prefix forwarded | L3-CLI-014, L3-CLI-021 |
| `input_extension_never_selects_mode` | Same bytes under .tmats, .ch10 and no suffix always choose the same route; swap bytes while retaining the name and the route follows bytes | L3-CLI-016 |
| `input_later_sync_does_not_trigger_scan` | Standalone bytes with 25 EB at offset 1 or later select text; no core search or automatic recovery | L3-CLI-014, L3-CLI-016 |
| `input_text_bytes_survive_route_probe` | Text source with CR/LF, high bytes, NUL and malformed tail reaches text reader byte-for-byte; reader interpretation remains separate | L3-CLI-018, L3-CLI-020 |
| `input_bom_is_not_stripped_to_find_sync` | EF BB BF before 25 EB selects text; router neither skips BOM nor changes source mapping | L3-CLI-014, L3-CLI-018 |
| `input_layout_is_not_skipped_to_find_sync` | Leading spaces/CR/LF before 25 EB select text; no prefix repair or trim | L3-CLI-014, L3-CLI-018 |
| `input_core_failure_never_retries_text` | Recording prefix plus injected core failure preserves failure; text-consumer call count stays zero | L3-CLI-017, L3-CLI-019 |
| `input_text_failure_never_retries_core` | Nonmatching prefix plus injected TMATS failure preserves failure; core-consumer call count stays zero | L3-CLI-018, L3-CLI-019 |
| `input_core_unavailable_is_dependency_gap` | Recording route with unavailable core adapter never substitutes local scanner/checksum/reader or text | L3-CLI-017, L3-CLI-019 |
| `input_core_evidence_survives_routing` | Inject qualified admission, RDG-008 sideband and data NotChecked alongside header-only PacketFacts; route never upgrades or adds fields | L3-CLI-017, L3-CLI-019 |
| `input_route_does_not_adopt_setup` | Successful routing/consumer call cannot itself turn Incomplete/Ambiguous or untrusted evidence into a usable governing setup | L3-CLI-019 |
| `input_probe_preserves_source_origin` | Selected consumer receives offset-zero bytes and same source identity, not bytes starting after the two-byte probe | L3-CLI-018, L3-CLI-020 |
| `input_router_keeps_borrow_owner_alive` | Borrowed source remains live through selected consumer; no mandatory full-file copy or library-owned file open | L3-CLI-017, L3-CLI-020 |
| `input_routing_oracle_is_independent` | Prefix expectations are literal source-derived bytes; injecting opposite consumer failures proves selection does not reuse parser success as an oracle | L3-CLI-021 |

These tests do not decide a public error spelling, report schema or numeric
exit code. Core packet fixtures and TMATS reader fixtures separately verify
those components; the routing sweep is not parser coverage. No executable
markers are added until the rebuilt implementation exists.

## Known defects in the prototype, and the tests that guard the rebuild

The team's spec alignment review (`docs/research/2026-09-27-spec-alignment-review.md`)
reproduced five defects in the prototype (tag `prototype-0`) against
106-24R1. Each is known from the 2026-09-25 prototype review and becomes a
regression test for the rebuilt library, as D1–D7 do for the reference
tools. They describe the prototype, not the design.

| # | Prototype defect | Standard | Regression test |
|---|------------------|----------|-----------------|
| P1 | `P-d\D1` read as bit rate, `D2` as encoding, `F1`–`F3` misread; D1 and F2 dropped when serialized | Table 9-6: D1 PCM code, D2 bit rate, F1 common word length, F2 word transfer order, F3 parity | `pcm_d_and_f_attributes_keep_their_table_9_6_meanings` (L1-READ-001, L1-REG-001) |
| P2 | `G\DST-1:STO` rejected as an invalid keyword | Table 9-2, `G\DST-n`: "STO Storage" | `g_dst_accepts_every_table_9_2_keyword` (L1-REG-001, L1-VAL-001) |
| P3 | CSDW bit 9 treated as reserved; `[12, 2, 0, 0]` re-encoded as `[12, 0, 0, 0]` | Figure 11-34: "FRMT. Bit 9 is the setup record format"; bits 31–10 reserved | `setup_record_format_bit_survives_decode_and_encode` (L1-CH10-002, L1-CH10-003) |
| P4 | RCCVER `0x0B` with `G\106:17` reported as source version 106-15 | ADR-0028: two declarations, neither derived from the other | `rccver_does_not_replace_g106` (L1-EDN-005) |
| P5 | Lenient reading of `G\PN:TEST;G\106:17;BROKEN` returned an empty document with no diagnostics | L1-READ-001, L1-READ-003 (preservation) | `lenient_reading_keeps_attributes_before_a_malformed_tail` (L1-READ-001, L1-READ-003) |

## Errata in the standard's own examples

Found while checking the design against the archived standard. Each is kept
verbatim in fixtures and handled by a diagnostic, never corrected silently.
Each also has an entry in the interpretation register
(`docs/INTERPRETATIONS.md`), which holds every interpretation, not only
errata.

| # | Where | Erratum | Handling |
|---|-------|---------|----------|
| E1 (INT-013) | Chapter 9 Appendix 9-C, page C-8 (106-24R1); the same 18 in 106-17, 106-19, 106-20, 106-22, 106-23, 106-24 | 18 attributes after D-group counters end with `:` instead of `;`, for example `D-1\MML\N-1-1:2: D-1\MNF\N-1-1-1:1: D-1\WP-1-1-1-1:14;` | L1-READ-007; `appendix_9c_example_reports_suspected_semicolons`. **Suspect** (ROADMAP S1). |
| E3 (INT-023) | Chapter 6 §6.2.3.11, `.TMATS WRITE` and `.TMATS READ` examples (106-24R1) | `G\DSI\N=18;` — `=` where §9.4.2 requires `:` | Read as a missing delimiter and kept exactly; whether to suggest `:` is open (follow-up F3). |
| E4 | Chapter 6 §6.2.3.11, example setup file (106-24R1) | `G\SHA:0;` — not "integer followed by "-" followed by hex characters" (Table 9-2) | Verification reports unverified with placeholder/no-embedded-digest reason; a defined exclusion permits COMPUTE, and explicit stamping may replace it (L3-SUM-002/005). |
| E2 (INT-010) | Chapter 9 Appendix 9-E, Table E-3 | The equality operator printed `= =`; the grammar gives `==` | ARCHITECTURE section 5.2 (T2): `==` is the operator, `= =` accepted with a warning. |

## Checks to run against real data

The owner's doubts to settle when the local sample recordings are read
(`docs/ROADMAP.md`, "Suspect findings to confirm against real data"):

- **S1** — count every "possible `;` typed as `:`" warning and every value
  containing `: ` followed by a code name across all samples; review each
  one; record whether real TMATS writers produce the pattern, whether any
  legitimate value triggers it, and whether a warning is the right default.
  Record the result in `docs/research/` and update the ROADMAP entry.

## Standards

The RCC 106 standards and handbooks are mirrored, with original URLs and
SHA-256, in <https://github.com/TelemetryWorks/rcc-106-standards>
(`python scripts/fetch.py --edition 106-24R1 --chapter 9`). Standards from
other bodies that IRIG 106 references — MIL-STD-1553B, ARINC 429, IEEE 1588,
IEEE 754 — are cited, not mirrored.
