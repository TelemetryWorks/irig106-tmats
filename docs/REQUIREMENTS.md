# irig106-tmats Requirements

| Field | Value |
|-------|-------|
| Document | TMATS-REQ-001 |
| Version | 0.1.0 |
| Status | Draft |
| Crate | `irig106-tmats` |
| Author | TelemetryWorks |

## Summary

| Level | Count | Description |
|-------|-------|-------------|
| L1 | 13 | Capability requirements |
| L2 | 62 | Functional requirements |
| L3 | 107 | Design requirements |
| **Total** | **182** | |

## Standards Traceability Map

### Primary Normative Sources

| Section | Title | Crate Relevance |
|---------|-------|-----------------|
| Ch9 §9.4.1 | Physical Format | ASCII encoding, file extensions, CR/LF handling |
| Ch9 §9.4.2 | Logical Format | `code-name:data-item;` syntax, occurrence indexing, COMMENT |
| Ch9 §9.4.3 | XML Format | Tmats.xsd, divergences from code-name format |
| Ch9 §9.5.1 | Group Relationships | Figure 9-1, hierarchical linkage model |
| Ch9 §9.5.2 | General Information (G) | Table 9-1, G-group attributes |
| Ch9 §9.5.3 | Transmission Attributes (T) | Table 9-2, T-group attributes |
| Ch9 §9.5.4 | Recorder-Reproducer Attributes (R) | Table 9-3/9-4, R-group attributes |
| Ch9 §9.5.5 | Multiplex/Modulation Attributes (M) | M-group attributes |
| Ch9 §9.5.6.1 | PCM Format Attributes (P) | Table 9-5, P-group attributes |
| Ch9 §9.5.6.2 | PCM Measurement Description (D) | D-group attributes |
| Ch9 §9.5.6.3 | Bus Data Attributes (B) | Table 9-6, B-group attributes |
| Ch9 §9.5.7 | Message Data Attributes (S) | S-group attributes |
| Ch9 §9.5.8 | Data Conversion Attributes (C) | C-group attributes |
| Ch9 §9.6 | DDML | Data Description Markup Language |
| Ch9 §9.7 | IHAL | Instrumentation Hardware Abstraction Language |
| Ch9 Appendix E | TMATS Grammar | Yacc/Lex formal grammar |

### Cross-References

| Section | Title | Crate Relevance |
|---------|-------|-----------------|
| Ch10 Type 0x01 | Computer-Generated Data Format 1 | Setup record CSDW layout |
| RCC 123-20 §5.5.2 | Setup Record | CSDW fields, multi-packet TMATS |
| Ch4 | PCM Standards | P-group and D-group attribute semantics |
| Ch8 / MIL-STD-1553 | Bus Data | B-group attribute semantics |
| RCC 124-13 | TMATS Handbook | Operational guidance, examples |

---

## L1: Capability Requirements

### L1-PARSE: ASCII Parsing

The crate SHALL parse TMATS data in the code-name logical format defined
in Ch9 §9.4.2, producing a structured in-memory representation from raw
ASCII byte streams.

- **Traces to:** Ch9 §9.4.1, §9.4.2, Appendix E
- **Workflow:** READ, REPAIR
- **Implemented in:** `src/parse.rs`

### L1-MODEL: Domain Model

The crate SHALL provide a strongly-typed Rust domain model representing
all TMATS attribute groups and their hierarchical relationships as
defined in Ch9 §9.5.1 (Figure 9-1).

- **Traces to:** Ch9 §9.5.1 through §9.5.8, §9.6, §9.7
- **Workflow:** ALL
- **Implemented in:** `src/model.rs`

### L1-SERIAL: ASCII Serialization

The crate SHALL serialize a structured TMATS representation back to
spec-compliant code-name ASCII format per Ch9 §9.4.2.

- **Traces to:** Ch9 §9.4.1, §9.4.2
- **Workflow:** WRITE, REPAIR, GENERATE
- **Implemented in:** `src/serial.rs`

### L1-XML: XML Format Support

The crate SHALL parse and serialize TMATS data in the XML schema format
defined in Ch9 §9.4.3 (Tmats.xsd), including the documented divergences
from the code-name format.

- **Traces to:** Ch9 §9.4.3
- **Workflow:** CONVERT
- **Implemented in:** `src/xml.rs` (planned, feature-gated)

### L1-VALID: Validation Engine

The crate SHALL validate TMATS data against the normative attribute
tables (required fields, conditional requirements, value constraints,
keyword enumerations) defined in Ch9 §9.5 tables.

- **Traces to:** Ch9 §9.5.2–§9.5.8 tables, R-CH10/RO-CH10 tags
- **Workflow:** VALIDATE, REPAIR
- **Implemented in:** `src/validate.rs`

### L1-VERSION: Version-Aware Processing

The crate SHALL support TMATS attribute sets across IRIG 106 editions
(106-04 through 106-17+), handling attributes that were added, modified,
deprecated, or removed across versions.

- **Traces to:** Ch9 §9.5 (all editions), G\106 version attribute
- **Workflow:** ALL
- **Implemented in:** `src/types_bridge.rs` (version enum), `src/validate.rs` (version-filtered rules)

### L1-CH10: Chapter 10 Payload Integration

The crate SHALL decode and encode TMATS payloads for Chapter 10
Type 0x01 setup record packets, including CSDW interpretation,
producing/consuming raw byte buffers that irig106-ch10-reader and
irig106-write use for packet-level framing.

- **Traces to:** Ch10 Type 0x01, RCC 123-20 §5.5.2
- **Workflow:** READ, WRITE
- **Implemented in:** `src/ch10.rs`

### L1-GEN: TMATS Generation

The crate SHALL generate minimal spec-compliant TMATS records from
a channel inventory (channel IDs, data types, observed parameters),
enabling recovery when a Ch10 file has missing or corrupt TMATS.

- **Traces to:** Ch9 §9.5.2 Table 9-1, §9.5.4 R-CH10 requirements
- **Workflow:** GENERATE
- **Implemented in:** `src/generate.rs`

### L1-REPAIR: TMATS Repair

The crate SHALL detect and automatically correct common TMATS defects
(broken counters, missing required attributes, case normalization,
duplicates) while preserving all valid data.

- **Traces to:** Ch9 §9.5 normative tables, operational quality
- **Workflow:** REPAIR
- **Implemented in:** `src/repair.rs`

### L1-QUERY: Attribute Query and Navigation

The crate SHALL provide programmatic navigation of the TMATS group
hierarchy and resolution of cross-group references.

- **Traces to:** Ch9 §9.5.1 Figure 9-1, group linkage conventions
- **Workflow:** QUERY, VALIDATE
- **Implemented in:** `src/query.rs`

### L1-ERR: Error Reporting

The crate SHALL produce structured, actionable error and warning
diagnostics for all parsing, validation, and serialization failures,
with source location information.

- **Traces to:** General engineering quality
- **Workflow:** ALL
- **Implemented in:** `src/error.rs`

### L1-PERF: Performance

The crate SHALL parse TMATS data with zero-copy or minimal-allocation
strategies suitable for high-throughput telemetry processing pipelines.

- **Traces to:** Ecosystem performance requirements
- **Workflow:** READ
- **Implemented in:** `src/parse.rs`, `src/model.rs` (Cow<'a, str>)

### L1-INTEROP: Ecosystem Integration

The crate SHALL integrate with irig106-types for shared domain types
and expose APIs consumable by irig106-write, irig106-cli,
irig106-studio, irig106-validator, and irig106-ch10-reader.

- **Traces to:** TelemetryWorks crate architecture
- **Workflow:** ALL
- **Implemented in:** `src/lib.rs`, `src/types_bridge.rs`

---

## L2: Functional Requirements

### L1-PARSE Decomposition

| ID | Requirement | Spec Ref | Tested By |
|----|-------------|----------|-----------|
| L2-PARSE-001 | Tokenize input into (code-name, data-item) pairs using `:` and `;` delimiters | Ch9 §9.4.2 | `parse_tests::parse_minimal_g_group` |
| L2-PARSE-002 | Discard non-printable characters (except CR/LF) from input | Ch9 §9.4.1 | `parse_tests::parse_with_crlf_between_attributes` |
| L2-PARSE-003 | Decompose code names into group prefix, attribute path, occurrence indices | Ch9 §9.4.2 | `parse_tests::parse_r_group_with_channels` |
| L2-PARSE-004 | Treat all code names and keyword values as case-insensitive | Ch9 §9.4.2 | `parse_tests::parse_case_insensitive_code_names` |
| L2-PARSE-005 | Recognize COMMENT attributes and preserve content | Ch9 §9.4.2 | `parse_tests::parse_comments_preserved` |
| L2-PARSE-006 | Accept integer, decimal, and scientific notation per §9.5 tables | Ch9 §9.5 | `parse_tests::parse_p_group_pcm_format` |
| L2-PARSE-007 | Produce identical results regardless of attribute order | Ch9 §9.4.2 | `parse_tests::parse_order_independence` |
| L2-PARSE-008 | Support incremental feeding of byte chunks for streaming I/O | Ch9 §9.4.1 | (planned) |
| L2-PARSE-009 | Support lenient mode that continues past malformed attributes | Operational | `parse_tests::parse_lenient_recovers_valid_data` |

### L1-MODEL Decomposition

| ID | Requirement | Spec Ref | Tested By |
|----|-------------|----------|-----------|
| L2-MODEL-001 | Represent all G-Group attributes per Ch9 §9.5.2 Table 9-1 | Ch9 §9.5.2 | `parse_tests::parse_data_sources` |
| L2-MODEL-002 | Represent all T-Group attributes per Ch9 §9.5.3 | Ch9 §9.5.3 | `src/model.rs` TGroup (core attrs) |
| L2-MODEL-003 | Represent all R-Group attributes per Ch9 §9.5.4 | Ch9 §9.5.4 | `parse_tests::parse_r_group_with_channels` |
| L2-MODEL-004 | Represent all M-Group attributes per Ch9 §9.5.5 | Ch9 §9.5.5 | `src/model.rs` MGroup (core attrs) |
| L2-MODEL-005 | Represent all P-Group attributes per Ch9 §9.5.6.1 | Ch9 §9.5.6.1 | `parse_tests::parse_p_group_pcm_format` |
| L2-MODEL-006 | Represent all D-Group attributes per Ch9 §9.5.6.2 | Ch9 §9.5.6.2 | `src/model.rs` DGroup (core attrs) |
| L2-MODEL-007 | Represent all B-Group attributes per Ch9 §9.5.6.3 | Ch9 §9.5.6.3 | `parse_tests::parse_multi_group_spec_example` |
| L2-MODEL-008 | Represent all S-Group attributes per Ch9 §9.5.7 | Ch9 §9.5.7 | `src/model.rs` SGroup (core attrs) |
| L2-MODEL-009 | Represent all C-Group attributes per Ch9 §9.5.8 | Ch9 §9.5.8 | `parse_tests::parse_multi_group_spec_example` |
| L2-MODEL-010 | Represent \N counter attributes as typed counts | Ch9 §9.5 | `parse_tests::parse_data_sources` |
| L2-MODEL-011 | Preserve cross-group linkages per Figure 9-1 | Ch9 §9.5.1 | `validate_query_tests::query_resolve_channel_with_pcm` |
| L2-MODEL-012 | Preserve unrecognized attributes as raw pairs | Operational | `parse_tests::parse_unknown_group_preserved` |
| L2-MODEL-013 | Represent DDML structures per Ch9 §9.6 | Ch9 §9.6 | (planned) |
| L2-MODEL-014 | Represent IHAL structures per Ch9 §9.7 | Ch9 §9.7 | (planned) |

### L1-SERIAL Decomposition

| ID | Requirement | Spec Ref | Tested By |
|----|-------------|----------|-----------|
| L2-SERIAL-001 | Emit attributes in `code-name:data-item;` format | Ch9 §9.4.2 | `serial_tests::round_trip_minimal` |
| L2-SERIAL-002 | Auto-compute and emit correct \N counter values | Ch9 §9.5 | `serial_tests::serialize_auto_computes_counters` |
| L2-SERIAL-003 | Parse→serialize round-trip produces semantically identical output | Ch9 §9.4.2 | `serial_tests::round_trip_multi_group` |
| L2-SERIAL-004 | Support compact and human-readable output modes | Operational | `serial_tests::serialize_compact_no_newlines` |
| L2-SERIAL-005 | Write to any `std::io::Write` and `Vec<u8>` | Operational | `serial_tests::round_trip_minimal` |

### L1-XML Decomposition

| ID | Requirement | Spec Ref | Tested By |
|----|-------------|----------|-----------|
| L2-XML-001 | Parse TMATS XML documents conforming to Tmats.xsd | Ch9 §9.4.3 | `tests/xml_tests.rs` |
| L2-XML-002 | Serialize domain model to valid TMATS XML | Ch9 §9.4.3 | `tests/xml_tests.rs` |
| L2-XML-003 | Handle XML divergences: per-link C-groups, no counters, expanded keywords, XML dates | Ch9 §9.4.3 a–d | `tests/xml_tests.rs` |
| L2-XML-004 | Support ASCII ↔ XML conversion via shared domain model | Ch9 §9.4.3 | `tests/xml_tests.rs` |
| L2-XML-005 | Validate against Tmats.xsd structure in strict mode | Ch9 §9.4.3 | (partial) |

### L1-VALID Decomposition

| ID | Requirement | Spec Ref | Tested By |
|----|-------------|----------|-----------|
| L2-VALID-001 | Enforce required attributes per §9.5 tables (R, RO, R-CH10, RO-CH10, RO-PAK) | Ch9 §9.5 | `validate_query_tests::validate_missing_program_name` |
| L2-VALID-002 | Evaluate "Required when" conditional requirements | Ch9 §9.5 | (planned) |
| L2-VALID-003 | Verify keyword attributes contain only enumerated values | Ch9 §9.5 | `tests/version_registry_tests.rs` |
| L2-VALID-004 | Enforce MFS (Maximum Field Size) constraints | Ch9 §9.5 | `tests/version_registry_tests.rs` |
| L2-VALID-005 | Verify \N counter values match actual group instance counts | Ch9 §9.5 | `validate_query_tests::validate_counter_mismatch_g_dsi` |
| L2-VALID-006 | Verify all cross-group references resolve | Ch9 §9.5.1 | `validate_query_tests::validate_broken_cross_group_ref` |
| L2-VALID-007 | Verify date fields conform to MM-DD-YYYY | Ch9 §9.5.2 | `tests/version_registry_tests.rs` |
| L2-VALID-008 | Enforce R-CH10 and RO-CH10 mandatory attributes for Ch10 sources | Ch10/Ch9 | (planned) |
| L2-VALID-009 | Classify findings as Error, Warning, or Info | Operational | `validate_query_tests::validate_missing_program_name` |
| L2-VALID-010 | Support validation profiles: Full, Ch10Required, Custom | Operational | `src/validate.rs` ValidationProfile |

### L1-VERSION Decomposition

| ID | Requirement | Spec Ref | Tested By |
|----|-------------|----------|-----------|
| L2-VERSION-001 | Detect IRIG 106 version from G\106 or CSDW iCh10Ver | Ch9 §9.5.2, Ch10 | `parse_tests::parse_minimal_g_group` |
| L2-VERSION-002 | Maintain attribute registry per IRIG 106 version | Ch9 §9.5 | `tests/version_registry_tests.rs` (23 attrs) |
| L2-VERSION-003 | Accept deprecated attributes from earlier versions with warnings | Ch9 §9.5 | `src/version.rs` is_deprecated_for() |
| L2-VERSION-004 | Provide migration diagnostics between versions | Ch9 §9.5 | `tests/version_registry_tests.rs` |
| L2-VERSION-005 | Allow callers to override auto-detected version | Operational | `src/parse.rs` ParseOptions |

### L1-CH10 Decomposition

| ID | Requirement | Spec Ref | Tested By |
|----|-------------|----------|-----------|
| L2-CH10-001 | Decode Type 0x01 CSDW (iCh10Ver, bConfigChange) | RCC 123-20 §5.5.2 | `ch10_tests::csdw_round_trip` |
| L2-CH10-002 | Encode valid Type 0x01 CSDW | RCC 123-20 §5.5.2 | `ch10_tests::csdw_round_trip` |
| L2-CH10-003 | Extract TMATS ASCII payload following CSDW | RCC 123-20 §5.5.2 | `ch10_tests::decode_setup_payload_basic` |
| L2-CH10-004 | Produce complete payload (CSDW + TMATS bytes) for irig106-write | RCC 123-20 §5.5.2 | `ch10_tests::encode_setup_payload_round_trip` |
| L2-CH10-005 | Support mid-stream TMATS during network streaming | RCC 123-20 §5.5.2 | (planned) |
| L2-CH10-006 | Handle zero-filled CSDW for pre-106-07 files | RCC 123-20 §5.5.2 | `ch10_tests::csdw_version_mapping` |
| L2-CH10-007 | Compare documents and report bConfigChange | RCC 123-20 §5.5.2 | `ch10_tests::detect_config_change_different` |

### L1-GEN Decomposition

| ID | Requirement | Spec Ref | Tested By |
|----|-------------|----------|-----------|
| L2-GEN-001 | Accept channel inventory (IDs, data types, metadata) | Operational | `validate_query_tests::generate_validates_with_zero_errors` |
| L2-GEN-002 | Produce G-group with required attributes | Ch9 §9.5.2 | `validate_query_tests::generate_validates_with_zero_errors` |
| L2-GEN-003 | Produce R-group entries satisfying R-CH10 requirements | Ch9 §9.5.4 | `validate_query_tests::generate_validates_with_zero_errors` |
| L2-GEN-004 | Produce stub P/B/S groups linked from R-group channels | Ch9 §9.5 | `validate_query_tests::generate_validates_with_zero_errors` |
| L2-GEN-005 | Annotate which attributes were inferred vs. provided | Operational | (planned) |
| L2-GEN-006 | Expose builder pattern API | Operational | `validate_query_tests::generate_validates_with_zero_errors` |
| L2-GEN-007 | Validate generated document before returning | Ch9 §9.5 | `validate_query_tests::generate_validates_with_zero_errors` |

### L1-REPAIR Decomposition

| ID | Requirement | Spec Ref | Tested By |
|----|-------------|----------|-----------|
| L2-REPAIR-001 | Recompute all \\N counter values from actual counts | Ch9 §9.5 | `tests/repair_tests.rs` |
| L2-REPAIR-002 | Detect and resolve duplicate attributes | Ch9 §9.4.2 | `src/parse.rs` (detect), (resolve planned) |
| L2-REPAIR-003 | Normalize keyword values to canonical case | Ch9 §9.5 | `tests/repair_tests.rs` |
| L2-REPAIR-004 | Insert missing R-CH10 required attributes with defaults | Ch9 §9.5.4 | (planned) |
| L2-REPAIR-005 | Detect orphan attributes referencing non-existent parents | Ch9 §9.5.1 | `tests/repair_tests.rs` |
| L2-REPAIR-006 | Produce detailed repair report with before/after values | Operational | `tests/repair_tests.rs` |
| L2-REPAIR-007 | Support dry-run mode | Operational | `tests/repair_tests.rs` |

### L1-QUERY Decomposition

| ID | Requirement | Spec Ref | Tested By |
|----|-------------|----------|-----------|
| L2-QUERY-001 | O(1) lookup by full code-name string | Operational | (planned, lazy index) |
| L2-QUERY-002 | Iterate all instances of a given group with occurrence indices | Ch9 §9.5.1 | `validate_query_tests::query_enumerate_channels` |
| L2-QUERY-003 | Walk hierarchy from G-group through linked groups per Figure 9-1 | Ch9 §9.5.1 | `validate_query_tests::query_resolve_channel_with_pcm` |
| L2-QUERY-004 | Resolve Ch10 channel ID to complete config chain (R→P→D→C) | Ch9 §9.5.1 | `validate_query_tests::query_resolve_channel_with_pcm` |
| L2-QUERY-005 | Enumerate declared data sources with types and classification | Ch9 §9.5.2 | `validate_query_tests::query_enumerate_data_sources` |
| L2-QUERY-006 | Compute structured diff between two documents | Operational | `src/query.rs` diff() |

### L1-ERR Decomposition

| ID | Requirement | Spec Ref | Tested By |
|----|-------------|----------|-----------|
| L2-ERR-001 | Parse errors include byte offset and line number | Operational | `src/error.rs` ParseError |
| L2-ERR-002 | All errors represented as Rust enum with per-category variants | Operational | `src/error.rs` TmatsError |
| L2-ERR-003 | Collect all errors rather than failing on first | Operational | `parse_tests::parse_lenient_recovers_valid_data` |
| L2-ERR-004 | Each error includes code-name, constraint, and actual value | Operational | `src/error.rs` Diagnostic |
| L2-ERR-005 | Support rendering with source spans for terminal display | Operational | (planned, miette) |

### L1-PERF Decomposition

| ID | Requirement | Spec Ref | Tested By |
|----|-------------|----------|-----------|
| L2-PERF-001 | Zero-copy borrowed mode referencing input buffer | Operational | `parse_tests::parse_minimal_g_group` |
| L2-PERF-002 | Owned mode with no lifetime dependencies | Operational | `parse_tests::parse_owned_produces_static_document` |
| L2-PERF-003 | O(n) allocations where n is attribute count | Operational | (planned, benchmark) |
| L2-PERF-004 | Handle TMATS up to multi-packet assembly sizes | Ch10 | (planned) |
| L2-PERF-005 | Deferred group construction for callers who don't need all groups | Operational | (planned) |

### L1-INTEROP Decomposition

| ID | Requirement | Spec Ref | Tested By |
|----|-------------|----------|-----------|
| L2-INTEROP-001 | Use shared types from irig106-types | Ecosystem | `src/types_bridge.rs` |
| L2-INTEROP-002 | Expose parse functions: `&[u8]` → `Result<TmatsDocument>` | Ecosystem | `parse_tests::parse_minimal_g_group` |
| L2-INTEROP-003 | Expose serialize functions: `&TmatsDocument` → `Write`/`Vec<u8>` | Ecosystem | `serial_tests::round_trip_minimal` |
| L2-INTEROP-004 | Expose payload builder: `(SetupRecordCsdw, Vec<u8>)` for irig106-write | Ecosystem | `ch10_tests::encode_setup_payload_round_trip` |
| L2-INTEROP-005 | Serde Serialize/Deserialize on model types | Ecosystem | (feature-gated) |
| L2-INTEROP-006 | Core modules usable in no_std + alloc for WASM | Ecosystem | (planned) |
| L2-INTEROP-007 | wasm-bindgen-compatible API subset | Ecosystem | `src/wasm.rs` |
| L2-INTEROP-008 | Document irig106-write payload byte-level contract | Ecosystem | `src/ch10.rs` doc comment |

---

## L3: Design Requirements

### L3-ARCH: Crate Architecture

| ID | Requirement | Implemented In |
|----|-------------|----------------|
| L3-ARCH-001 | Module layout: parse, model, serial, xml, validate, version, ch10, generate, repair, query, error | `src/lib.rs` |
| L3-ARCH-002 | Feature flags: xml, serde, wasm, std, rich-errors, generate, repair, full | `Cargo.toml` |
| L3-ARCH-003 | Internal dependency DAG: error ← model ← parse/serial/validate/query; ch10 ← parse+serial | `docs/ARCHITECTURE.md` |
| L3-ARCH-004 | Public re-exports for key types at crate root | `src/lib.rs` |
| L3-ARCH-005 | Prelude module for ergonomic imports | `src/lib.rs` prelude |

### L3-PARSE: Parser Internals

| ID | Requirement | Implemented In |
|----|-------------|----------------|
| L3-PARSE-001 | Two-phase parse: Phase 1 (tokenize) → Phase 2 (structure) | `src/parse.rs` tokenize() + structure_document() |
| L3-PARSE-002 | Tokenizer: single-pass byte-level state machine, no regex, no backtracking | `src/parse.rs` tokenize() |
| L3-PARSE-003 | Tokenizer input trait for &[u8], Read, mmap | (planned) |
| L3-PARSE-004 | Inline filter non-printable bytes during scanning | `src/parse.rs` tokenize() |
| L3-PARSE-005 | CodeName struct: GroupPrefix, occurrence, SmallVec path | `src/model.rs` CodeName |
| L3-PARSE-006 | GroupPrefix enum: G/T/R/M/P/D/B/S/C/Unknown(char) | `src/types_bridge.rs` GroupPrefix |
| L3-PARSE-007 | Cow<'input, str> for zero-copy strings, validated 7-bit ASCII | `src/parse.rs` |
| L3-PARSE-008 | Case-fold at comparison time via to_ascii_uppercase, preserve original | `src/parse.rs` first_path_name_upper() |
| L3-PARSE-009 | Phase 2 routes by GroupPrefix to independent group builders | `src/parse.rs` structure_document() |
| L3-PARSE-010 | GroupBuilder trait: accept(), finish() | (planned, currently inline fns) |
| L3-PARSE-011 | Lenient mode wraps errors in collection sidecar | `src/parse.rs` structure_document() |
| L3-PARSE-012 | Duplicate attribute detection in strict/lenient modes | `src/parse.rs` structure_document() |
| L3-PARSE-013 | ParseOptions struct: mode, target_version, max_attributes, max_value_size | `src/parse.rs` ParseOptions |
| L3-PARSE-014 | RawAttribute struct: code_name, raw_code_name, value, byte_offset, line | `src/model.rs` RawAttribute |

### L3-MODEL: Domain Model Internals

| ID | Requirement | Implemented In |
|----|-------------|----------------|
| L3-MODEL-001 | TmatsDocument root: all group collections + unknown + version + diagnostics | `src/model.rs` TmatsDocument |
| L3-MODEL-002 | AttrValue enum: Keyword, Text, Integer, Decimal, Scientific, Date, Boolean, Empty | `src/model.rs` AttrValue |
| L3-MODEL-003 | TmatsDate: month/day/year with MM-DD-YYYY and XML format parsing | `src/model.rs` TmatsDate |
| L3-MODEL-004 | GGroup struct: program_name, irig106_version, dates, data_sources, POC, comments, extra | `src/model.rs` GGroup |
| L3-MODEL-005 | DataSourceDecl: data_source_id, type, classification, extra | `src/model.rs` DataSourceDecl |
| L3-MODEL-006 | RGroup struct: recorder_id, media_type, channels, drives, index/events enabled, extra | `src/model.rs` RGroup |
| L3-MODEL-007 | RChannel struct: channel_id, data_type, data_link_name, packing, enabled, extra | `src/model.rs` RChannel |
| L3-MODEL-008 | PGroup struct: data_link_name, bit_rate, encoding, frame structure, subframes, extra | `src/model.rs` PGroup |
| L3-MODEL-009 | DGroup struct: measurement_list_name, measurements, extra | `src/model.rs` DGroup |
| L3-MODEL-010 | BGroup struct: data_link_name, bus_type, messages, extra | `src/model.rs` BGroup |
| L3-MODEL-011 | CGroup struct: measurement_name, conversion_type, pair_sets, eu_units, extra | `src/model.rs` CGroup |
| L3-MODEL-012 | ConversionPairSet: raw_value, eu_value | `src/model.rs` ConversionPairSet |
| L3-MODEL-013 | IndexMap<u32, T> for repeated groups, non-contiguous index support | `src/model.rs` all group collections |
| L3-MODEL-014 | DataLinkName newtype for cross-group reference resolution | `src/model.rs` DataLinkName |
| L3-MODEL-015 | OwnedTmatsDocument = TmatsDocument<'static> type alias | `src/model.rs` |
| L3-MODEL-016 | into_owned() on every model type converting Cow::Borrowed to Owned | `src/model.rs` all structs |
| L3-MODEL-017 | TGroup struct: transmitter_id, carrier_frequency, modulation, power, antenna, extra | `src/model.rs` TGroup |
| L3-MODEL-018 | MGroup struct: baseband_signal_type, modulation_sense, subcarriers, extra | `src/model.rs` MGroup |
| L3-MODEL-019 | SGroup struct: data_link_name, message_definitions, extra | `src/model.rs` SGroup |
| L3-MODEL-020 | Every group struct has `extra: Vec<RawAttribute<'a>>` for unrecognized attributes | `src/model.rs` all structs |

### L3-SERIAL: Serializer Internals

| ID | Requirement | Implemented In |
|----|-------------|----------------|
| L3-SERIAL-001 | Visitor pattern: G, T, R, M, P, D, B, S, C, unknown order | `src/serial.rs` serialize_with_options() |
| L3-SERIAL-002 | TmatsEmitter trait: emit_attr(), emit_comment() with Compact/Pretty impls | `src/serial.rs` TmatsEmitter |
| L3-SERIAL-003 | Counter auto-computation from IndexMap::len() before emitting | `src/serial.rs` serialize_g_group() etc. |
| L3-SERIAL-004 | Code-name reconstruction from model, not stored raw strings | `src/serial.rs` serialize_r_group() etc. |
| L3-SERIAL-005 | Value formatting rules: Integer, Decimal, Date, Boolean, Keyword | `src/serial.rs` |
| L3-SERIAL-006 | SerializeOptions: format, line_ending, emit_comments, canonical_keyword_case | `src/serial.rs` SerializeOptions |
| L3-SERIAL-007 | Vec<u8> pre-allocation from estimated attribute count × avg size | `src/serial.rs` serialize_to_vec() |

### L3-XML: XML Internals

| ID | Requirement | Implemented In |
|----|-------------|----------------|
| L3-XML-001 | quick-xml backend for no-alloc streaming parse | `src/xml.rs` |
| L3-XML-002 | XML element-to-group mapping via Tmats.xsd type names | `src/xml.rs` |
| L3-XML-003 | Counter attribute suppression during XML serialization | (planned) |
| L3-XML-004 | Keyword expansion: abbreviated ↔ full form | (planned) |
| L3-XML-005 | Static keyword expansion registry | (planned) |
| L3-XML-006 | Date conversion: MM-DD-YYYY ↔ YYYY-MM-DD | (planned) |
| L3-XML-007 | Per-link C-group expansion during XML serialization | (planned) |
| L3-XML-008 | Per-link C-group collapse during XML parsing | (planned) |

### L3-VALID: Validation Engine Internals

| ID | Requirement | Implemented In |
|----|-------------|----------------|
| L3-VALID-001 | Rule registry: Vec<Box<dyn ValidationRule>> | `src/validate.rs` default_rules() |
| L3-VALID-002 | ValidationRule trait: id, description, severity, spec_ref, applicable_versions, check | `src/validate.rs` ValidationRule |
| L3-VALID-003 | Built-in rule categories: Required, Conditional, Keyword, MFS, Counter, CrossRef, Date, Numeric | `src/validate.rs` (5 rules implemented) |
| L3-VALID-004 | ValidationContext: target_version, source, profile, attribute_registry | `src/validate.rs` ValidationContext |
| L3-VALID-005 | Diagnostic struct: severity, rule_id, message, spec_ref, attribute_path, expected, actual, fix | `src/error.rs` Diagnostic |
| L3-VALID-006 | Severity enum: Error, Warning, Info | `src/error.rs` Severity |
| L3-VALID-007 | ValidationProfile enum: Full, Ch10Required, Custom | `src/validate.rs` ValidationProfile |
| L3-VALID-008 | Version-filtered execution: skip rules not applicable to detected version | `src/validate.rs` validate_with_options() |
| L3-VALID-009 | Parallel rule execution via rayon (std feature) | (planned) |
| L3-VALID-010 | ValidationReport: diagnostics, version, counts | `src/validate.rs` ValidationReport |

### L3-VERSION: Version Registry Internals

| ID | Requirement | Implemented In |
|----|-------------|----------------|
| L3-VERSION-001 | Irig106Version enum: V106_04 through V106_17 | `src/types_bridge.rs` |
| L3-VERSION-002 | AttrMeta struct: code_name_pattern, introduced_in, deprecated_in, required_tag, MFS, keywords | (planned) |
| L3-VERSION-003 | RequiredTag enum: Required, RequiredOptional, RequiredCh10, etc. | (planned) |
| L3-VERSION-004 | ValueType enum: Keyword, Text, Integer, Decimal, Scientific, Date, Boolean, FreeForm | (planned) |
| L3-VERSION-005 | AttrMetaRegistry: entries by code-name, by group, by version | (planned) |
| L3-VERSION-006 | Build-time registry generation from declarative TOML/JSON source | (planned) |
| L3-VERSION-007 | Declarative source-of-truth file derived from §9.5 tables | (planned) |
| L3-VERSION-008 | Version detection priority: override → G\106 → CSDW → heuristic | `src/parse.rs` structure_document() |
| L3-VERSION-009 | CSDW-to-Version static mapping table | `src/types_bridge.rs` from_csdw_version() |
| L3-VERSION-010 | Migration diff: set differences between version registries | (planned) |

### L3-CH10: Chapter 10 Payload Internals

| ID | Requirement | Implemented In |
|----|-------------|----------------|
| L3-CH10-001 | SetupRecordCsdw struct: ch10_version (u8), config_change (bool) | `src/ch10.rs` SetupRecordCsdw |
| L3-CH10-002 | CSDW decode: u32 LE, bitmask extraction | `src/ch10.rs` decode() |
| L3-CH10-003 | CSDW encode: pack fields into u32 LE, zero reserved bits | `src/ch10.rs` encode() |
| L3-CH10-004 | Payload extraction: slice at offset 4 after CSDW | `src/ch10.rs` decode_setup_payload() |
| L3-CH10-005 | SetupRecordPayload: csdw + tmats_bytes + total_len + to_bytes | `src/ch10.rs` SetupRecordPayload |
| L3-CH10-006 | Config change detection via canonical serialization comparison | `src/ch10.rs` detect_config_change() |
| L3-CH10-007 | Pre-106-07 CSDW: zero → version None, fall back to G\106 | `src/ch10.rs` + `src/types_bridge.rs` |
| L3-CH10-008 | PayloadEncoding enum: Ascii, Xml | `src/ch10.rs` PayloadEncoding |

### L3-GEN: TMATS Generator Internals

| ID | Requirement | Implemented In |
|----|-------------|----------------|
| L3-GEN-001 | ChannelInventoryEntry: channel_id, data_type, observed rates, packet count | `src/generate.rs` |
| L3-GEN-002 | TmatsBuilder: version, program_name, test_number, channels | `src/generate.rs` |
| L3-GEN-003 | Builder validation: ≥1 channel, no duplicates, version required | `src/generate.rs` build() |
| L3-GEN-004 | G-group generation: PN, 106, OD, DSI\N, DSI-n | `src/generate.rs` build() |
| L3-GEN-005 | R-group generation: ID, N, TK1-n, CDT-n, CDLN-n, PDP-n | `src/generate.rs` build() |
| L3-GEN-006 | Format group stubs: PCM→P, 1553/ARINC→B, Message→S | `src/generate.rs` build() |
| L3-GEN-007 | GenerationMetadata: generated, inferred, missing attribute sets | (planned) |
| L3-GEN-008 | DataTypeCode → GroupPrefix static mapping table | `src/types_bridge.rs` tmats_group() |

### L3-REPAIR: Repair Engine Internals

| ID | Requirement | Implemented In |
|----|-------------|----------------|
| L3-REPAIR-001 | RepairEngine struct: rules, options | `src/repair.rs` |
| L3-REPAIR-002 | RepairRule trait: id, description, can_auto_fix, detect, apply | `src/repair.rs` |
| L3-REPAIR-003 | RepairFinding: rule_id, description, current/proposed values, auto_fixable | `src/repair.rs` |
| L3-REPAIR-004 | RepairAction: rule_id, action_type, attribute_path, old/new values | `src/repair.rs` |
| L3-REPAIR-005 | RepairOptions: dry_run, duplicate_strategy, auto_fix flags | `src/repair.rs` |
| L3-REPAIR-006 | Counter recomputation logic | `src/repair.rs` |
| L3-REPAIR-007 | RepairReport: findings, actions, pre/post error counts | `src/repair.rs` |
| L3-REPAIR-008 | Repair-then-validate pipeline | `src/repair.rs` |

### L3-QUERY: Query API Internals

| ID | Requirement | Implemented In |
|----|-------------|----------------|
| L3-QUERY-001 | HashMap<String, AttrRef> index for O(1) code-name lookup | (planned, lazy) |
| L3-QUERY-002 | AttrRef enum: group, occurrence, field_path | (planned) |
| L3-QUERY-003 | ChannelConfig: channel_id, recorder, format (Pcm/Bus/Message), measurements, conversions | `src/query.rs` ChannelConfig |
| L3-QUERY-004 | Group iterator: impl Iterator<Item = (u32, &T)> via TmatsGroup trait | (planned) |
| L3-QUERY-005 | TmatsGroup trait: group_prefix() implemented for all 9 groups | `src/model.rs` TmatsGroup |
| L3-QUERY-006 | Diff algorithm: serialize canonical → compare → added/removed/modified | `src/query.rs` diff() |
| L3-QUERY-007 | Lazy index construction via OnceLock on first lookup | (planned) |

### L3-ERR: Error Internals

| ID | Requirement | Implemented In |
|----|-------------|----------------|
| L3-ERR-001 | TmatsError enum: Parse, Validate, Serialize, Ch10, Xml, Generate, Repair | `src/error.rs` TmatsError |
| L3-ERR-002 | ParseError struct: kind, byte_offset, line, code_name, message | `src/error.rs` ParseError |
| L3-ERR-003 | ParseErrorKind enum: InvalidDelimiter, MalformedCodeName, InvalidValue, etc. | `src/error.rs` ParseErrorKind |
| L3-ERR-004 | TmatsErrors collection: Vec<TmatsError> with IntoIterator | `src/error.rs` TmatsErrors |
| L3-ERR-005 | Display format: `[SEVERITY] CODE: message (at byte X, line Y) → spec_ref` | `src/error.rs` Display impls |
| L3-ERR-006 | miette SourceSpan under "rich-errors" feature | (planned) |
| L3-ERR-007 | Error code convention: TMATS-P/V/S/C/G/R/X### | `src/error.rs` |

### L3-PERF: Performance Internals

| ID | Requirement | Implemented In |
|----|-------------|----------------|
| L3-PERF-001 | Lifetime parameterization: TmatsDocument<'a>, Cow<'a, str> | `src/model.rs` all types |
| L3-PERF-002 | into_owned() → TmatsDocument<'static> | `src/model.rs` into_owned() |
| L3-PERF-003 | Allocation count tracking in debug benchmarks | (planned) |
| L3-PERF-004 | SmallVec<[PathSegment; 4]> for code-name paths | `src/model.rs` CodeName |
| L3-PERF-005 | Pre-sized collections from \N counter or heuristic | (planned) |
| L3-PERF-006 | Lazy group construction: Deferred variant with resolve() | (planned) |
| L3-PERF-007 | Criterion benchmark suite: parse_small/medium/large, serialize, validate, query | (planned) |

### L3-INTEROP: Ecosystem Integration Internals

| ID | Requirement | Implemented In |
|----|-------------|----------------|
| L3-INTEROP-001 | parse(), parse_owned(), parse_with_options() signatures | `src/parse.rs` |
| L3-INTEROP-002 | serialize(), serialize_to_vec(), serialize_with_options() signatures | `src/serial.rs` |
| L3-INTEROP-003 | decode_setup_payload(), encode_setup_payload() signatures | `src/ch10.rs` |
| L3-INTEROP-004 | validate(), validate_with_options() signatures | `src/validate.rs` |
| L3-INTEROP-005 | TmatsBuilder::build() → Result<OwnedTmatsDocument> | `src/generate.rs` |
| L3-INTEROP-006 | repair(), repair_dry_run() signatures | `src/repair.rs` |
| L3-INTEROP-007 | Serde derives: cfg_attr(feature = "serde") on all model types | `src/model.rs` |
| L3-INTEROP-008 | WASM bindings: parse_tmats, validate_tmats, get_channel_config | (planned) |
| L3-INTEROP-009 | irig106-write contract doc comment on encode_setup_payload | `src/ch10.rs` SetupRecordPayload |
| L3-INTEROP-010 | irig106-types import list: DataTypeCode, ChannelId, Irig106Version, GroupPrefix | `src/types_bridge.rs` |

### L3-TEST: Testing Strategy

| ID | Requirement | Implemented In |
|----|-------------|----------------|
| L3-TEST-001 | Spec example round-trip tests | `tests/parse_tests.rs`, `tests/serial_tests.rs` |
| L3-TEST-002 | Per-group parse tests (G, T, R, M, P, D, B, S, C) | `tests/parse_tests.rs` |
| L3-TEST-003 | Version-specific validation tests | `tests/validate_query_tests.rs` |
| L3-TEST-004 | Fuzz testing via cargo-fuzz | (planned) |
| L3-TEST-005 | Property-based tests via proptest | (planned) |
| L3-TEST-006 | Ch10 integration tests with real packet bytes | `tests/ch10_tests.rs` |
| L3-TEST-007 | Generation → validate → zero errors | `tests/validate_query_tests.rs` |
| L3-TEST-008 | Repair idempotency tests | `tests/repair_tests.rs` |
| L3-TEST-009 | XML/ASCII conversion round-trip | `tests/xml_tests.rs` |
| L3-TEST-010 | Lenient parse degradation tests | `tests/parse_tests.rs` |
| L3-TEST-011 | WASM smoke tests | (planned — requires wasm-pack) |
| L3-TEST-012 | Benchmark regression CI gate | (planned — CI config needed) |

---

## Implementation Status

| Category | Total | Implemented | Planned |
|----------|-------|-------------|---------|
| L1 Capabilities | 13 | **13** | 0 |
| L2 Functional | 62 | **57** | 5 |
| L3 Design | 107 | **96** | 11 |
| Tests | 105 | **105** (99 #[test] + 6 proptest) | WASM smoke, CI gate |
| Fuzz targets | 3 | **3** | — |
| Benchmarks | 8 | **8** | — |
| Repair rules | 4 | **4** | — |
| Validation rules | 13 | **13** | — |

### Remaining L2/L3 Work

| ID | Description | Status |
|----|-------------|--------|
| L2-PARSE-008 | Streaming/incremental parse | Planned (TmatsInput trait) |
| L2-VALID-002 | Conditional "Required when" evaluation | Planned (needs full attr registry) |
| L2-VERSION-002 | Complete attribute registry (all ~200+ attrs) | Partial (23 of ~200+) |
| L3-PARSE-003 | TmatsInput trait for Read/mmap | Planned |
| L3-PARSE-010 | GroupBuilder trait extraction | Planned (currently inline fns) |
| L3-PERF-003 | Allocation count tracking in benchmarks | Planned |
| L3-PERF-005 | Pre-sized collections from \N hint | Planned |
| L3-PERF-006 | Lazy/deferred group construction | Planned |
| L3-QUERY-001 | O(1) code-name HashMap index | Planned (lazy via OnceLock) |
| L3-QUERY-004 | Generic group iterator via TmatsGroup trait | Planned |
| L3-VALID-009 | Parallel rule execution via rayon | Planned |
| L3-VERSION-006 | Enable build.rs → include!() path | Ready (uncomment in version.rs) |
| L3-INTEROP-006 | no_std core compatibility verification | Planned |
| L3-TEST-011 | WASM smoke tests via wasm-pack | Planned |

---

## Revision History

| Version | Date | Changes |
|---------|------|---------|
| 0.1.0 | 2026-03-23 | Initial requirements from architecture session |
| 0.2.0 | 2026-03-23 | Added L1-GEN, L1-REPAIR; expanded L3 from 33→107 |
| 0.3.0 | 2026-03-23 | Version registry, expanded validation (13 rules), property tests |
| 0.4.0 | 2026-03-23 | XML module, WASM bindings, build.rs codegen, fuzz targets; all 13 L1 implemented |
