// tests/validate_query_tests.rs
//
// # Validation and Query Integration Tests
//
// ## Traceability:
//   L3-TEST-003: Version-specific validation
//   L3-TEST-007: Generation round-trip (generate → validate → zero errors)
//   L2-VALID-001..006
//   L2-QUERY-004, L2-QUERY-005

use irig106_tmats::prelude::*;

// ═════════════════════════════════════════════════════════════════════════════
// Validation: Required Attributes (L2-VALID-001)
// ═════════════════════════════════════════════════════════════════════════════

#[test]
fn validate_missing_program_name() {
    let input = b"G\\106:17;";
    let doc = parse(input).expect("parse failed");
    let report = validate(&doc);

    assert!(report.has_errors());
    assert!(report.diagnostics.iter().any(|d| d.rule_id == "TMATS-V001"));
}

#[test]
fn validate_missing_version() {
    let input = b"G\\PN:TEST;";
    let doc = parse(input).expect("parse failed");
    let report = validate(&doc);

    assert!(report.has_errors());
    assert!(report.diagnostics.iter().any(|d| d.rule_id == "TMATS-V002"));
}

#[test]
fn validate_complete_minimal_passes() {
    let input = b"G\\PN:TEST;G\\106:17;";
    let doc = parse(input).expect("parse failed");
    let report = validate(&doc);

    // Should have no errors (may have warnings)
    assert_eq!(report.errors, 0);
}

// ═════════════════════════════════════════════════════════════════════════════
// Validation: Counter Consistency (L2-VALID-005)
// ═════════════════════════════════════════════════════════════════════════════

#[test]
fn validate_counter_mismatch_g_dsi() {
    let input = b"G\\PN:TEST;G\\106:17;G\\DSI\\N:3;G\\DSI-1:ONLY_ONE;";
    let doc = parse(input).expect("parse failed");
    let report = validate(&doc);

    assert!(report.diagnostics.iter().any(|d| d.rule_id == "TMATS-V010"));
}

#[test]
fn validate_counter_correct_g_dsi() {
    let input = b"G\\PN:TEST;G\\106:17;G\\DSI\\N:1;G\\DSI-1:SRC;";
    let doc = parse(input).expect("parse failed");
    let report = validate(&doc);

    assert!(!report.diagnostics.iter().any(|d| d.rule_id == "TMATS-V010"));
}

#[test]
fn validate_counter_mismatch_r_n() {
    let input = b"G\\PN:TEST;G\\106:17;R-1\\ID:REC;R-1\\N:5;R-1\\TK1-1:1;R-1\\TK1-2:2;";
    let doc = parse(input).expect("parse failed");
    let report = validate(&doc);

    assert!(report.diagnostics.iter().any(|d| d.rule_id == "TMATS-V011"));
}

// ═════════════════════════════════════════════════════════════════════════════
// Validation: Cross-Group References (L2-VALID-006)
// ═════════════════════════════════════════════════════════════════════════════

#[test]
fn validate_broken_cross_group_ref() {
    let input = b"G\\PN:TEST;G\\106:17;\
        R-1\\ID:REC;R-1\\N:1;R-1\\TK1-1:1;R-1\\CDLN-1:NONEXISTENT_LINK;";
    let doc = parse(input).expect("parse failed");
    let report = validate(&doc);

    assert!(report.diagnostics.iter().any(|d| d.rule_id == "TMATS-V020"));
}

#[test]
fn validate_valid_cross_group_ref() {
    let input = b"G\\PN:TEST;G\\106:17;\
        R-1\\ID:REC;R-1\\N:1;R-1\\TK1-1:1;R-1\\CDLN-1:PCM1;\
        P-1\\DLN:PCM1;";
    let doc = parse(input).expect("parse failed");
    let report = validate(&doc);

    assert!(!report.diagnostics.iter().any(|d| d.rule_id == "TMATS-V020"));
}

// ═════════════════════════════════════════════════════════════════════════════
// Query: Channel Resolution (L2-QUERY-004)
// ═════════════════════════════════════════════════════════════════════════════

#[test]
fn query_resolve_channel_with_pcm() {
    let input = b"G\\PN:TEST;G\\106:17;\
        R-1\\ID:REC;R-1\\N:1;R-1\\TK1-1:5;R-1\\CDLN-1:PCM_LINK;\
        P-1\\DLN:PCM_LINK;P-1\\D1:1000000;";
    let doc = parse(input).expect("parse failed");

    let config = resolve_channel(&doc, 5).expect("channel not found");
    assert_eq!(config.channel_id, 5);
    assert!(matches!(config.format, Some(irig106_tmats::query::FormatRef::Pcm(_))));
}

#[test]
fn query_resolve_channel_with_bus() {
    let input = b"G\\PN:TEST;G\\106:17;\
        R-1\\ID:REC;R-1\\N:1;R-1\\TK1-1:3;R-1\\CDLN-1:BUS_LINK;\
        B-1\\DLN:BUS_LINK;B-1\\BT:1553;";
    let doc = parse(input).expect("parse failed");

    let config = resolve_channel(&doc, 3).expect("channel not found");
    assert!(matches!(config.format, Some(irig106_tmats::query::FormatRef::Bus(_))));
}

#[test]
fn query_resolve_nonexistent_channel() {
    let input = b"G\\PN:TEST;G\\106:17;R-1\\ID:REC;R-1\\N:1;R-1\\TK1-1:1;";
    let doc = parse(input).expect("parse failed");

    assert!(resolve_channel(&doc, 99).is_none());
}

// ═════════════════════════════════════════════════════════════════════════════
// Query: Data Source Enumeration (L2-QUERY-005)
// ═════════════════════════════════════════════════════════════════════════════

#[test]
fn query_enumerate_data_sources() {
    let input = b"G\\PN:TEST;G\\106:17;\
        G\\DSI\\N:2;G\\DSI-1:SRC_A;G\\DST-1:REC;G\\DSI-2:SRC_B;G\\DST-2:TEL;";
    let doc = parse(input).expect("parse failed");

    let sources = enumerate_data_sources(&doc);
    assert_eq!(sources.len(), 2);
    assert_eq!(sources[0].1.data_source_id.as_deref(), Some("SRC_A"));
    assert_eq!(sources[1].1.data_source_id.as_deref(), Some("SRC_B"));
}

// ═════════════════════════════════════════════════════════════════════════════
// Query: Channel Enumeration
// ═════════════════════════════════════════════════════════════════════════════

#[test]
fn query_enumerate_channels() {
    let input = b"G\\PN:TEST;G\\106:17;\
        R-1\\N:2;R-1\\TK1-1:1;R-1\\TK1-2:5;\
        R-2\\N:1;R-2\\TK1-1:10;";
    let doc = parse(input).expect("parse failed");

    let channels = enumerate_channels(&doc);
    assert_eq!(channels.len(), 3);
}

// ═════════════════════════════════════════════════════════════════════════════
// Generate + Validate (L3-TEST-007)
// ═════════════════════════════════════════════════════════════════════════════

#[cfg(feature = "generate")]
#[test]
fn generate_validates_with_zero_errors() {
    use irig106_tmats::generate::*;

    let doc = TmatsBuilder::new(Irig106Version::V106_17)
        .program_name("GEN_TEST")
        .add_channel(ChannelInventoryEntry {
            channel_id: 1,
            data_type: DataTypeCode::Pcm,
            subchannel_count: None,
            observed_bit_rate: Some(1_000_000.0),
            observed_sample_rate: None,
            packet_count: Some(1000),
        })
        .add_channel(ChannelInventoryEntry {
            channel_id: 3,
            data_type: DataTypeCode::Mil1553Format1,
            subchannel_count: None,
            observed_bit_rate: None,
            observed_sample_rate: None,
            packet_count: Some(500),
        })
        .build()
        .expect("generation failed");

    assert_eq!(doc.general.program_name.as_deref(), Some("GEN_TEST"));
    assert_eq!(doc.recorders[&1].channels.len(), 2);
    assert_eq!(doc.pcm_formats.len(), 1); // PCM channel
    assert_eq!(doc.bus_data.len(), 1);    // 1553 channel

    let report = validate(&doc);
    assert_eq!(report.errors, 0, "generated TMATS should have zero validation errors: {:?}",
        report.diagnostics);
}
