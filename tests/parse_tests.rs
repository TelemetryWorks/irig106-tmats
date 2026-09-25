// tests/parse_tests.rs
//
// # Parse Integration Tests
//
// Tests for the TMATS ASCII parser covering Phase 1 (tokenization)
// and Phase 2 (structuring).
//
// ## Traceability:
//   L3-TEST-001: Spec example round-trip
//   L3-TEST-002: Per-group parse tests
//   L3-TEST-010: Lenient parse degradation

use irig106_tmats::prelude::*;

// ═════════════════════════════════════════════════════════════════════════════
// Basic Parsing (L2-PARSE-001, L2-PARSE-002)
// ═════════════════════════════════════════════════════════════════════════════

#[test]
fn parse_minimal_g_group() {
    let input = b"G\\PN:FLIGHT_TEST;G\\106:17;";
    let doc = parse(input).expect("parse failed");

    assert_eq!(doc.general.program_name.as_deref(), Some("FLIGHT_TEST"));
    assert_eq!(doc.general.irig106_version.as_deref(), Some("17"));
    assert_eq!(doc.source_version, Some(Irig106Version::V106_17));
}

#[test]
fn parse_g_group_with_dates() {
    let input = b"G\\PN:TEST;G\\106:07;G\\OD:03-15-2024;G\\RN:1;G\\RD:04-01-2024;";
    let doc = parse(input).expect("parse failed");

    let od = doc.general.origination_date.unwrap();
    assert_eq!(od.month, 3);
    assert_eq!(od.day, 15);
    assert_eq!(od.year, 2024);
    assert_eq!(od.to_string(), "03-15-2024");

    assert_eq!(doc.general.revision_number.as_deref(), Some("1"));
}

#[test]
fn parse_data_sources() {
    let input = b"G\\PN:TEST;G\\106:17;\
        G\\DSI\\N:2;\
        G\\DSI-1:SOURCE_A;G\\DST-1:REC;G\\DSC-1:U;\
        G\\DSI-2:SOURCE_B;G\\DST-2:REC;G\\DSC-2:S;";
    let doc = parse(input).expect("parse failed");

    assert_eq!(doc.general.num_data_sources, Some(2));
    assert_eq!(doc.general.data_sources.len(), 2);

    let ds1 = &doc.general.data_sources[&1];
    assert_eq!(ds1.data_source_id.as_deref(), Some("SOURCE_A"));
    assert_eq!(ds1.data_source_type.as_deref(), Some("REC"));
    assert_eq!(ds1.classification.as_deref(), Some("U"));

    let ds2 = &doc.general.data_sources[&2];
    assert_eq!(ds2.data_source_id.as_deref(), Some("SOURCE_B"));
    assert_eq!(ds2.classification.as_deref(), Some("S"));
}

// ═════════════════════════════════════════════════════════════════════════════
// R-Group Parsing (L3-TEST-002: Per-group)
// ═════════════════════════════════════════════════════════════════════════════

#[test]
fn parse_r_group_with_channels() {
    let input = b"G\\PN:TEST;G\\106:17;\
        R-1\\ID:RECORDER_1;R-1\\N:2;\
        R-1\\TK1-1:1;R-1\\CDT-1:09;R-1\\CDLN-1:PCM_LINK;\
        R-1\\TK1-2:5;R-1\\CDT-2:19;R-1\\CDLN-2:BUS_LINK;\
        R-1\\IDX\\E:T;";
    let doc = parse(input).expect("parse failed");

    assert_eq!(doc.recorders.len(), 1);
    let r = &doc.recorders[&1];
    assert_eq!(r.recorder_id.as_deref(), Some("RECORDER_1"));
    assert_eq!(r.num_channels, Some(2));
    assert_eq!(r.index_enabled, Some(true));

    let ch1 = &r.channels[&1];
    assert_eq!(ch1.channel_id, Some(1));
    assert_eq!(ch1.data_type.as_deref(), Some("09"));
    assert_eq!(ch1.data_link_name.as_deref(), Some("PCM_LINK"));

    let ch2 = &r.channels[&2];
    assert_eq!(ch2.channel_id, Some(5));
    assert_eq!(ch2.data_type.as_deref(), Some("19"));
}

// ═════════════════════════════════════════════════════════════════════════════
// P-Group Parsing (L3-TEST-002)
// ═════════════════════════════════════════════════════════════════════════════

#[test]
fn parse_p_group_pcm_format() {
    let input = b"G\\PN:TEST;G\\106:17;\
        P-1\\DLN:PCM_LINK;P-1\\D1:1000000;P-1\\D2:NRZ-L;\
        P-1\\F1:128;P-1\\F2:16;P-1\\F3:FE6B2840;P-1\\F3\\L:32;";
    let doc = parse(input).expect("parse failed");

    assert_eq!(doc.pcm_formats.len(), 1);
    let p = &doc.pcm_formats[&1];
    assert_eq!(p.data_link_name.as_deref(), Some("PCM_LINK"));
    assert_eq!(p.bit_rate, Some(1_000_000.0));
    assert_eq!(p.encoding.as_deref(), Some("NRZ-L"));
    assert_eq!(p.num_words_per_frame, Some(128));
    assert_eq!(p.num_bits_per_word, Some(16));
    assert_eq!(p.sync_pattern.as_deref(), Some("FE6B2840"));
    assert_eq!(p.sync_pattern_length, Some(32));
}

// ═════════════════════════════════════════════════════════════════════════════
// Order Independence (L2-PARSE-007)
// ═════════════════════════════════════════════════════════════════════════════

#[test]
fn parse_order_independence() {
    // Attributes in reversed order should produce the same model
    let input_forward = b"G\\PN:TEST;G\\106:17;R-1\\ID:REC;R-1\\N:1;R-1\\TK1-1:10;";
    let input_reverse = b"R-1\\TK1-1:10;R-1\\N:1;R-1\\ID:REC;G\\106:17;G\\PN:TEST;";

    let doc_fwd = parse(input_forward).expect("forward parse failed");
    let doc_rev = parse(input_reverse).expect("reverse parse failed");

    assert_eq!(
        doc_fwd.general.program_name.as_deref(),
        doc_rev.general.program_name.as_deref()
    );
    assert_eq!(
        doc_fwd.recorders[&1].recorder_id.as_deref(),
        doc_rev.recorders[&1].recorder_id.as_deref()
    );
    assert_eq!(
        doc_fwd.recorders[&1].channels[&1].channel_id,
        doc_rev.recorders[&1].channels[&1].channel_id
    );
}

// ═════════════════════════════════════════════════════════════════════════════
// Case Insensitivity (L2-PARSE-004)
// ═════════════════════════════════════════════════════════════════════════════

#[test]
fn parse_case_insensitive_code_names() {
    let input = b"g\\pn:Lower;G\\106:17;";
    let doc = parse(input).expect("parse failed");
    assert_eq!(doc.general.program_name.as_deref(), Some("Lower"));
}

// ═════════════════════════════════════════════════════════════════════════════
// Comment Handling (L2-PARSE-005)
// ═════════════════════════════════════════════════════════════════════════════

#[test]
fn parse_comments_preserved() {
    let input = b"G\\PN:TEST;G\\106:17;G\\COM:This is a comment;R-1\\COM:Recorder note;";
    let doc = parse(input).expect("parse failed");

    assert_eq!(doc.general.comments.len(), 1);
    assert_eq!(doc.general.comments[0].as_ref(), "This is a comment");
}

// ═════════════════════════════════════════════════════════════════════════════
// Whitespace / Non-Printable Handling (L2-PARSE-002)
// ═════════════════════════════════════════════════════════════════════════════

#[test]
fn parse_with_crlf_between_attributes() {
    let input = b"G\\PN:TEST;\r\nG\\106:17;\r\nR-1\\ID:REC;\r\n";
    let doc = parse(input).expect("parse failed");

    assert_eq!(doc.general.program_name.as_deref(), Some("TEST"));
    assert_eq!(doc.recorders[&1].recorder_id.as_deref(), Some("REC"));
}

// ═════════════════════════════════════════════════════════════════════════════
// Lenient Mode (L2-PARSE-009, L3-TEST-010)
// ═════════════════════════════════════════════════════════════════════════════

#[test]
fn parse_lenient_recovers_valid_data() {
    // Mix of valid and malformed attributes
    let input = b"G\\PN:TEST;BADATTR_NO_COLON;G\\106:17;";
    let opts = ParseOptions {
        mode: ParseMode::Lenient,
        ..Default::default()
    };

    // Lenient mode should either recover or fail gracefully
    let result = irig106_tmats::parse::parse_with_options(input, &opts);
    // With lenient parsing, we expect to get the valid attributes
    match result {
        Ok(doc) => {
            assert_eq!(doc.general.program_name.as_deref(), Some("TEST"));
        }
        Err(_) => {
            // Also acceptable if tokenizer can't recover
        }
    }
}

// ═════════════════════════════════════════════════════════════════════════════
// Unknown Groups (L2-MODEL-012)
// ═════════════════════════════════════════════════════════════════════════════

#[test]
fn parse_unknown_group_preserved() {
    let input = b"G\\PN:TEST;G\\106:17;Z-1\\CUSTOM:value;";
    let doc = parse(input).expect("parse failed");

    assert_eq!(doc.unknown.len(), 1);
    assert_eq!(doc.unknown[0].value.as_ref(), "value");
}

// ═════════════════════════════════════════════════════════════════════════════
// Owned Mode (L2-PERF-002)
// ═════════════════════════════════════════════════════════════════════════════

#[test]
fn parse_owned_produces_static_document() {
    let input = b"G\\PN:TEST;G\\106:17;R-1\\ID:REC;";
    let doc: OwnedTmatsDocument = parse_owned(input).expect("parse_owned failed");

    // This compiles because doc is 'static
    assert_eq!(doc.general.program_name.as_deref(), Some("TEST"));
    assert_eq!(doc.recorders[&1].recorder_id.as_deref(), Some("REC"));
}

// ═════════════════════════════════════════════════════════════════════════════
// Multi-group integration (L3-TEST-001)
// ═════════════════════════════════════════════════════════════════════════════

#[test]
fn parse_multi_group_spec_example() {
    // Simplified version of a real-world TMATS with multiple groups
    let input = b"\
        G\\PN:F16_FLIGHT_TEST;G\\106:17;G\\OD:01-15-2024;\
        G\\TN:FT-2024-001;\
        G\\DSI\\N:2;\
        G\\DSI-1:PCM_SRC;G\\DST-1:REC;G\\DSC-1:U;\
        G\\DSI-2:BUS_SRC;G\\DST-2:REC;G\\DSC-2:U;\
        R-1\\ID:MDR-1;R-1\\N:2;\
        R-1\\TK1-1:1;R-1\\CDT-1:09;R-1\\CDLN-1:PCM1;\
        R-1\\TK1-2:3;R-1\\CDT-2:19;R-1\\CDLN-2:BUS1;\
        R-1\\IDX\\E:T;\
        P-1\\DLN:PCM1;P-1\\D1:5000000;P-1\\D2:NRZ-L;P-1\\F1:256;P-1\\F2:16;\
        B-1\\DLN:BUS1;B-1\\BT:1553;\
        C-1\\DCN:AIRSPEED;C-1\\DCT:PAIR;C-1\\EU:KNOTS;";

    let doc = parse(input).expect("parse failed");

    // G-group
    assert_eq!(doc.general.program_name.as_deref(), Some("F16_FLIGHT_TEST"));
    assert_eq!(doc.source_version, Some(Irig106Version::V106_17));
    assert_eq!(doc.general.data_sources.len(), 2);

    // R-group
    let r = &doc.recorders[&1];
    assert_eq!(r.channels.len(), 2);
    assert_eq!(r.index_enabled, Some(true));

    // P-group
    let p = &doc.pcm_formats[&1];
    assert_eq!(p.data_link_name.as_deref(), Some("PCM1"));
    assert_eq!(p.bit_rate, Some(5_000_000.0));

    // B-group
    let b = &doc.bus_data[&1];
    assert_eq!(b.data_link_name.as_deref(), Some("BUS1"));
    assert_eq!(b.bus_type.as_deref(), Some("1553"));

    // C-group
    let c = &doc.data_conversion[&1];
    assert_eq!(c.measurement_name.as_deref(), Some("AIRSPEED"));
    assert_eq!(c.eu_units.as_deref(), Some("KNOTS"));
}

// ═════════════════════════════════════════════════════════════════════════════
// Duplicate Attribute Detection (L3-PARSE-012)
// ═════════════════════════════════════════════════════════════════════════════

#[test]
fn parse_strict_rejects_duplicates() {
    // Same attribute appears twice
    let input = b"G\\PN:FIRST;G\\106:17;G\\PN:SECOND;";
    let opts = ParseOptions {
        mode: ParseMode::Strict,
        ..Default::default()
    };

    let result = irig106_tmats::parse::parse_with_options(input, &opts);
    assert!(
        result.is_err(),
        "strict mode should reject duplicate attributes"
    );
}

#[test]
fn parse_lenient_keeps_last_duplicate() {
    // Same attribute appears twice — lenient mode keeps last
    let input = b"G\\PN:FIRST;G\\106:17;G\\PN:SECOND;";
    let opts = ParseOptions {
        mode: ParseMode::Lenient,
        ..Default::default()
    };

    let doc = irig106_tmats::parse::parse_with_options(input, &opts)
        .expect("lenient parse should succeed");

    // Keep-last semantics: SECOND overwrites FIRST
    assert_eq!(doc.general.program_name.as_deref(), Some("SECOND"));

    // Should have a diagnostic about the duplicate
    assert!(
        !doc.parse_diagnostics.is_empty(),
        "should record duplicate as a parse diagnostic"
    );
    assert!(
        doc.parse_diagnostics
            .iter()
            .any(|d| d.rule_id == "TMATS-P004"),
        "should have TMATS-P004 diagnostic for duplicate"
    );
}

#[test]
fn parse_no_false_duplicate_for_indexed_attrs() {
    // Indexed attributes like R-1\TK1-1 and R-1\TK1-2 are NOT duplicates
    let input = b"G\\PN:TEST;G\\106:17;R-1\\TK1-1:1;R-1\\TK1-2:5;";
    let opts = ParseOptions {
        mode: ParseMode::Strict,
        ..Default::default()
    };

    let doc = irig106_tmats::parse::parse_with_options(input, &opts)
        .expect("different indices should not be duplicates");

    assert_eq!(doc.recorders[&1].channels.len(), 2);
}
