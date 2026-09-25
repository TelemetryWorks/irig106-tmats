// tests/ch10_tests.rs
//
// # Chapter 10 Payload Integration Tests
//
// ## Traceability:
//   L3-TEST-006: Ch10 integration tests

use irig106_tmats::prelude::*;

// ═════════════════════════════════════════════════════════════════════════════
// CSDW Encode/Decode (L2-CH10-001, L2-CH10-002)
// ═════════════════════════════════════════════════════════════════════════════

#[test]
fn csdw_round_trip() {
    let original = SetupRecordCsdw {
        ch10_version: 12, // 106-17
        config_change: true,
    };

    let bytes = original.encode();
    let decoded = SetupRecordCsdw::decode(&bytes).expect("decode failed");

    assert_eq!(decoded.ch10_version, 12);
    assert_eq!(decoded.config_change, true);
}

#[test]
fn csdw_version_mapping() {
    let csdw = SetupRecordCsdw { ch10_version: 12, config_change: false };
    assert_eq!(csdw.irig_version(), Some(Irig106Version::V106_17));

    let csdw = SetupRecordCsdw { ch10_version: 7, config_change: false };
    assert_eq!(csdw.irig_version(), Some(Irig106Version::V106_07));

    // Pre-106-07: version field is zero/undefined (L2-CH10-006)
    let csdw = SetupRecordCsdw { ch10_version: 0, config_change: false };
    assert_eq!(csdw.irig_version(), None);
}

#[test]
fn csdw_from_version() {
    let csdw = SetupRecordCsdw::from_version(Irig106Version::V106_17, false);
    assert_eq!(csdw.ch10_version, 12);
    assert_eq!(csdw.config_change, false);
}

#[test]
fn csdw_reserved_bits_zero() {
    let csdw = SetupRecordCsdw { ch10_version: 12, config_change: true };
    let bytes = csdw.encode();
    let word = u32::from_le_bytes(bytes);
    // Bits 9-31 should all be zero
    assert_eq!(word & 0xFFFFFE00, 0);
}

// ═════════════════════════════════════════════════════════════════════════════
// Payload Decode (L2-CH10-003)
// ═════════════════════════════════════════════════════════════════════════════

#[test]
fn decode_setup_payload_basic() {
    // Build a raw payload: CSDW + TMATS ASCII
    let csdw = SetupRecordCsdw::from_version(Irig106Version::V106_17, false);
    let tmats_bytes = b"G\\PN:PAYLOAD_TEST;G\\106:17;";

    let mut payload = Vec::new();
    payload.extend_from_slice(&csdw.encode());
    payload.extend_from_slice(tmats_bytes);

    let (decoded_csdw, doc) = decode_setup_payload(&payload)
        .expect("decode failed");

    assert_eq!(decoded_csdw.ch10_version, 12);
    assert_eq!(doc.general.program_name.as_deref(), Some("PAYLOAD_TEST"));
    assert_eq!(doc.source_version, Some(Irig106Version::V106_17));
}

#[test]
fn decode_setup_payload_too_short() {
    let result = decode_setup_payload(&[0, 0]);
    assert!(result.is_err());
}

#[test]
fn decode_setup_payload_csdw_only_no_tmats() {
    let csdw = SetupRecordCsdw::from_version(Irig106Version::V106_17, false);
    let encoded = csdw.encode();
    let result = decode_setup_payload(&encoded);
    assert!(result.is_err());
}

// ═════════════════════════════════════════════════════════════════════════════
// Payload Encode (L2-CH10-004, L2-INTEROP-004)
// ═════════════════════════════════════════════════════════════════════════════

#[test]
fn encode_setup_payload_round_trip() {
    let input = b"G\\PN:ENCODE_TEST;G\\106:17;R-1\\ID:REC;R-1\\N:1;R-1\\TK1-1:1;";
    let doc = parse(input).expect("parse failed");
    let csdw = SetupRecordCsdw::from_version(Irig106Version::V106_17, false);

    let payload = encode_setup_payload(&doc, &csdw).expect("encode failed");

    assert!(payload.total_len() > 4);

    // Decode the payload back
    let bytes = payload.to_bytes();
    let (decoded_csdw, decoded_doc) = decode_setup_payload(&bytes)
        .expect("decode round-trip failed");

    assert_eq!(decoded_csdw.ch10_version, csdw.ch10_version);
    assert_eq!(
        decoded_doc.general.program_name.as_deref(),
        Some("ENCODE_TEST"),
    );
    assert_eq!(decoded_doc.recorders[&1].channels[&1].channel_id, Some(1));
}

// ═════════════════════════════════════════════════════════════════════════════
// Config Change Detection (L2-CH10-007)
// ═════════════════════════════════════════════════════════════════════════════

#[test]
fn detect_config_change_identical() {
    let input = b"G\\PN:TEST;G\\106:17;";
    let doc1 = parse(input).expect("parse failed");
    let doc2 = parse(input).expect("parse failed");

    let changed = detect_config_change(&doc1, &doc2).expect("detect failed");
    assert!(!changed);
}

#[test]
fn detect_config_change_different() {
    let doc1 = parse(b"G\\PN:VERSION_A;G\\106:17;").expect("parse failed");
    let doc2 = parse(b"G\\PN:VERSION_B;G\\106:17;").expect("parse failed");

    let changed = detect_config_change(&doc1, &doc2).expect("detect failed");
    assert!(changed);
}
