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
this contract; INT-011 records Joey's 2026-11-23 per-channel decision and
its amended wording awaits AI recheck. No fixture here
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
- No other reading diagnostic; D-3 and D-4 (correctly delimited) produce
  none.
- Writing the document back reproduces the input bytes exactly.
- Applying all 18 suggestions yields a document in which each of those
  counters is a separate attribute with a numeric value.

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
