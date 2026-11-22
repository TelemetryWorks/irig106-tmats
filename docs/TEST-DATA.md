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
requirements are drafted; INT-038's decimal identity needs owner sign-off
and independent AI review under ADR-0033. INT-035/036/037 remain approved.
Full development analysis and tests still precede implementation.

Sources: 106-24R1 Chapter 9 sections 9.4.2 and 9.5.1 a/Table 9-1,
pages 9-2/3 and 9-5; Table 9-2 page 9-10; Table 9-4 page 9-48;
Table 9-7 pages 9-115/117; sections 9.5.12/13/14 pages 9-171/172.
The [baseline draft](research/2026-11-22-code-name-index-draft.md) identifies
rows and separates cited syntax from proposed numeric identity policy.
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
| `decimal_index_spelling_preserves_numeric_identity` | `D-01\MN-001-02:a;D-1\MN-1-2:b;` | Under proposed INT-038 both keys have canonical indices (1; 1, 2); per-token information findings for 01/001/02 plus a normalization-only full-key collision information finding listing both occurrences now; retain both items/order, never choose a winner | L3-READ-023, L3-READ-026, L3-READ-027, L3-READ-029 |
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
numeric identities, not raw spellings or repeated occurrences. Their scoped
requirements and executable checks are explicitly deferred to the counter
batch, including the section 9.5.14 X noncontiguity exception. This batch
checks index identity/visibility only and must not claim counter acceptance.

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
fragment 2 with a different RCCVER ends the record and is reported; a PCM
packet between fragments 1 and 2 ends the record; a corrupted header
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
| a | a record in two fragments ending with `;`, then a time packet | complete, "followed by another packet" |
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
| E4 | Chapter 6 §6.2.3.11, example setup file (106-24R1) | `G\SHA:0;` — not "integer followed by "-" followed by hex characters" (Table 9-2) | Verification reports malformed (L1-SUM-002). |
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
