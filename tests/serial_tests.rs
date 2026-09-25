// tests/serial_tests.rs
//
// # Serialization Integration Tests
//
// Round-trip fidelity and serialization correctness tests.
//
// ## Traceability:
//   L3-TEST-001: Round-trip tests
//   L2-SERIAL-003: Round-trip fidelity
//   L2-SERIAL-002: Counter auto-computation

use irig106_tmats::prelude::*;

// ═════════════════════════════════════════════════════════════════════════════
// Round-Trip (L2-SERIAL-003, L3-TEST-001)
// ═════════════════════════════════════════════════════════════════════════════

#[test]
fn round_trip_minimal() {
    let input = b"G\\PN:TEST;G\\106:17;";
    let doc = parse(input).expect("parse failed");
    let output = serialize_to_vec(&doc).expect("serialize failed");

    let reparsed = parse(&output).expect("reparse failed");
    assert_eq!(
        doc.general.program_name.as_deref(),
        reparsed.general.program_name.as_deref(),
    );
    assert_eq!(
        doc.general.irig106_version.as_deref(),
        reparsed.general.irig106_version.as_deref(),
    );
}

#[test]
fn round_trip_with_r_group() {
    let input = b"G\\PN:TEST;G\\106:17;\
        R-1\\ID:MDR;R-1\\N:1;R-1\\TK1-1:5;R-1\\CDT-1:09;R-1\\CDLN-1:PCM1;";
    let doc = parse(input).expect("parse failed");
    let output = serialize_to_vec(&doc).expect("serialize failed");
    let reparsed = parse(&output).expect("reparse failed");

    assert_eq!(reparsed.recorders.len(), 1);
    assert_eq!(reparsed.recorders[&1].channels[&1].channel_id, Some(5));
    assert_eq!(
        reparsed.recorders[&1].channels[&1]
            .data_link_name
            .as_deref(),
        Some("PCM1"),
    );
}

#[test]
fn round_trip_multi_group() {
    let input = b"\
        G\\PN:ROUND_TRIP;G\\106:17;\
        G\\DSI\\N:1;G\\DSI-1:SRC1;G\\DST-1:REC;G\\DSC-1:U;\
        R-1\\ID:REC1;R-1\\N:1;R-1\\TK1-1:1;R-1\\CDT-1:09;R-1\\CDLN-1:PCM1;\
        P-1\\DLN:PCM1;P-1\\D1:1000000;P-1\\D2:NRZ-L;\
        C-1\\DCN:TEMP;C-1\\EU:DEGF;";

    let doc = parse(input).expect("parse failed");
    let output = serialize_to_vec(&doc).expect("serialize failed");
    let reparsed = parse(&output).expect("reparse failed");

    assert_eq!(reparsed.general.data_sources.len(), 1);
    assert_eq!(reparsed.pcm_formats[&1].bit_rate, Some(1_000_000.0));
    assert_eq!(
        reparsed.data_conversion[&1].eu_units.as_deref(),
        Some("DEGF")
    );
}

// ═════════════════════════════════════════════════════════════════════════════
// Counter Auto-Computation (L2-SERIAL-002, L3-SERIAL-003)
// ═════════════════════════════════════════════════════════════════════════════

#[test]
fn serialize_auto_computes_counters() {
    // Parse with explicit N=2 and 2 data sources
    let input = b"G\\PN:TEST;G\\106:17;G\\DSI\\N:2;\
        G\\DSI-1:A;G\\DSI-2:B;";
    let doc = parse(input).expect("parse failed");
    let output = serialize_to_vec(&doc).expect("serialize failed");
    let output_str = String::from_utf8_lossy(&output);

    // Serializer should emit G\DSI\N:2 computed from the map
    assert!(output_str.contains("G\\DSI\\N:2;"));
}

#[test]
fn serialize_fixes_incorrect_counter() {
    // Manually create a doc where the counter is wrong
    let input = b"G\\PN:TEST;G\\106:17;G\\DSI\\N:5;\
        G\\DSI-1:ONLY_ONE;";
    let doc = parse(input).expect("parse failed");

    // doc.general.num_data_sources == Some(5) but data_sources.len() == 1
    // Serializer should auto-compute from the map (L3-SERIAL-003)
    let output = serialize_to_vec(&doc).expect("serialize failed");
    let output_str = String::from_utf8_lossy(&output);

    // Should emit the actual count (1), not the declared count (5)
    assert!(output_str.contains("G\\DSI\\N:1;"));
}

// ═════════════════════════════════════════════════════════════════════════════
// Compact vs Pretty Formatting (L2-SERIAL-004)
// ═════════════════════════════════════════════════════════════════════════════

#[test]
fn serialize_compact_no_newlines() {
    let input = b"G\\PN:TEST;G\\106:17;";
    let doc = parse(input).expect("parse failed");

    let opts = SerializeOptions {
        format: irig106_tmats::serial::OutputFormat::Compact,
        ..Default::default()
    };

    let mut buf = Vec::new();
    irig106_tmats::serial::serialize_with_options(&doc, &mut buf, &opts).expect("serialize failed");
    let output = String::from_utf8(buf).unwrap();

    assert!(!output.contains('\n'));
    assert!(output.contains("G\\PN:TEST;"));
}

#[test]
fn serialize_pretty_has_newlines() {
    let input = b"G\\PN:TEST;G\\106:17;";
    let doc = parse(input).expect("parse failed");

    let opts = SerializeOptions {
        format: irig106_tmats::serial::OutputFormat::Pretty,
        ..Default::default()
    };

    let mut buf = Vec::new();
    irig106_tmats::serial::serialize_with_options(&doc, &mut buf, &opts).expect("serialize failed");
    let output = String::from_utf8(buf).unwrap();

    assert!(output.contains('\n'));
}

// ═════════════════════════════════════════════════════════════════════════════
// Extra Attribute Preservation (L2-MODEL-012, L2-SERIAL-003)
// ═════════════════════════════════════════════════════════════════════════════

#[test]
fn round_trip_preserves_unknown_attributes() {
    let input = b"G\\PN:TEST;G\\106:17;Z-1\\EXOTIC:mystery_value;";
    let doc = parse(input).expect("parse failed");
    let output = serialize_to_vec(&doc).expect("serialize failed");
    let output_str = String::from_utf8_lossy(&output);

    assert!(output_str.contains("Z-1\\EXOTIC:mystery_value;"));
}
