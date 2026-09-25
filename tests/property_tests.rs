// tests/property_tests.rs
//
// # Property-Based Tests
//
// Uses proptest to verify invariants that must hold for all inputs.
//
// ## Traceability:
//   L3-TEST-005: Round-trip fidelity via proptest
//   L2-SERIAL-003: Parse→serialize→parse semantic identity

use proptest::prelude::*;

use irig106_tmats::prelude::*;

// ═════════════════════════════════════════════════════════════════════════════
// Strategies for generating TMATS data
// ═════════════════════════════════════════════════════════════════════════════

/// Generate a valid TMATS ASCII string with random but structurally correct content.
fn arb_tmats_ascii() -> impl Strategy<Value = Vec<u8>> {
    (
        "[A-Z][A-Z0-9_]{1,20}",        // program name
        prop::sample::select(vec!["04", "05", "07", "09", "11", "13", "15", "17"]),
        1..8u32,                         // number of channels
    )
        .prop_flat_map(|(pn, ver, num_ch)| {
            let channels = prop::collection::vec(
                (1..256u16, prop::sample::select(vec!["09", "19", "21", "30"])),
                num_ch as usize,
            );
            (Just(pn), Just(ver.to_string()), Just(num_ch), channels)
        })
        .prop_map(|(pn, ver, num_ch, channels)| {
            let mut buf = Vec::new();

            // G-group
            buf.extend_from_slice(format!("G\\PN:{pn};G\\106:{ver};").as_bytes());
            buf.extend_from_slice(format!("G\\DSI\\N:{num_ch};").as_bytes());
            for (i, (ch_id, _)) in channels.iter().enumerate() {
                let idx = i + 1;
                buf.extend_from_slice(
                    format!("G\\DSI-{idx}:SRC_{ch_id};G\\DST-{idx}:REC;G\\DSC-{idx}:U;").as_bytes()
                );
            }

            // R-group
            buf.extend_from_slice(format!("R-1\\ID:GEN_REC;R-1\\N:{num_ch};").as_bytes());
            for (i, (ch_id, cdt)) in channels.iter().enumerate() {
                let idx = i + 1;
                buf.extend_from_slice(
                    format!(
                        "R-1\\TK1-{idx}:{ch_id};R-1\\CDT-{idx}:{cdt};R-1\\CDLN-{idx}:LINK_{idx};R-1\\PDP-{idx}:UN;"
                    ).as_bytes()
                );
            }

            buf
        })
}

/// Generate a simple TMATS with just G-group attributes.
fn arb_simple_tmats() -> impl Strategy<Value = Vec<u8>> {
    (
        "[A-Z][A-Z0-9_]{1,16}",
        prop::sample::select(vec!["07", "09", "11", "13", "15", "17"]),
        prop::option::of("[A-Z0-9]{1,10}"),
    )
        .prop_map(|(pn, ver, tn)| {
            let mut buf = format!("G\\PN:{pn};G\\106:{ver};");
            if let Some(tn) = tn {
                buf.push_str(&format!("G\\TN:{tn};"));
            }
            buf.into_bytes()
        })
}

// ═════════════════════════════════════════════════════════════════════════════
// Property: Parse → Serialize → Parse round-trip (L3-TEST-005, L2-SERIAL-003)
// ═════════════════════════════════════════════════════════════════════════════

proptest! {
    #[test]
    fn prop_round_trip_simple(input in arb_simple_tmats()) {
        let doc = parse(&input).expect("initial parse failed");
        let serialized = serialize_to_vec(&doc).expect("serialize failed");
        let reparsed = parse(&serialized).expect("reparse failed");

        // Semantic equality on key fields
        prop_assert_eq!(
            doc.general.program_name.as_deref(),
            reparsed.general.program_name.as_deref(),
            "program name mismatch after round-trip"
        );
        prop_assert_eq!(
            doc.general.irig106_version.as_deref(),
            reparsed.general.irig106_version.as_deref(),
            "IRIG version mismatch after round-trip"
        );
        prop_assert_eq!(
            doc.general.test_number.as_deref(),
            reparsed.general.test_number.as_deref(),
            "test number mismatch after round-trip"
        );
    }

    #[test]
    fn prop_round_trip_with_channels(input in arb_tmats_ascii()) {
        let doc = parse(&input).expect("initial parse failed");
        let serialized = serialize_to_vec(&doc).expect("serialize failed");
        let reparsed = parse(&serialized).expect("reparse failed");

        // G-group
        prop_assert_eq!(
            doc.general.program_name.as_deref(),
            reparsed.general.program_name.as_deref(),
        );
        prop_assert_eq!(
            doc.general.data_sources.len(),
            reparsed.general.data_sources.len(),
            "data source count mismatch"
        );

        // R-group channels
        prop_assert_eq!(
            doc.recorders.len(),
            reparsed.recorders.len(),
            "recorder count mismatch"
        );
        for (&r_idx, r_group) in &doc.recorders {
            let repr = reparsed.recorders.get(&r_idx)
                .expect("missing R-group after round-trip");
            prop_assert_eq!(
                r_group.channels.len(),
                repr.channels.len(),
                "channel count mismatch in R-{}", r_idx
            );
            for (&ch_idx, ch) in &r_group.channels {
                let rech = repr.channels.get(&ch_idx)
                    .expect("missing channel after round-trip");
                prop_assert_eq!(
                    ch.channel_id, rech.channel_id,
                    "channel_id mismatch R-{}\\TK1-{}", r_idx, ch_idx
                );
                prop_assert_eq!(
                    ch.data_type.as_deref(), rech.data_type.as_deref(),
                    "data_type mismatch R-{}\\CDT-{}", r_idx, ch_idx
                );
            }
        }
    }
}

// ═════════════════════════════════════════════════════════════════════════════
// Property: Serialized counter matches model (L2-SERIAL-002)
// ═════════════════════════════════════════════════════════════════════════════

proptest! {
    #[test]
    fn prop_serialized_counters_correct(input in arb_tmats_ascii()) {
        let doc = parse(&input).expect("parse failed");
        let serialized = serialize_to_vec(&doc).expect("serialize failed");
        let reparsed = parse(&serialized).expect("reparse failed");

        // G\DSI\N should equal actual data source count
        if let Some(n) = reparsed.general.num_data_sources {
            prop_assert_eq!(
                n as usize,
                reparsed.general.data_sources.len(),
                "G\\DSI\\N counter mismatch after serialization"
            );
        }

        // R-x\N should equal actual channel count
        for (&r_idx, r) in &reparsed.recorders {
            if let Some(n) = r.num_channels {
                prop_assert_eq!(
                    n as usize,
                    r.channels.len(),
                    "R-{}\\N counter mismatch after serialization", r_idx
                );
            }
        }
    }
}

// ═════════════════════════════════════════════════════════════════════════════
// Property: Validation of generated docs produces no errors (L3-TEST-007)
// ═════════════════════════════════════════════════════════════════════════════

proptest! {
    #[test]
    fn prop_generated_validates_cleanly(input in arb_tmats_ascii()) {
        let doc = parse(&input).expect("parse failed");

        // Re-serialize to fix any counter issues, then validate
        let serialized = serialize_to_vec(&doc).expect("serialize failed");
        let clean_doc = parse(&serialized).expect("reparse failed");
        let report = validate(&clean_doc);

        // Should have zero errors (warnings are acceptable)
        // Note: generated data may have keyword issues since we use raw CDT codes,
        // so we only check that core structural validation passes
        let structural_errors: Vec<_> = report.diagnostics.iter()
            .filter(|d| d.severity == Severity::Error)
            .filter(|d| d.rule_id.starts_with("TMATS-V0") && d.rule_id.as_str() < "TMATS-V030")
            .collect();

        prop_assert!(
            structural_errors.is_empty(),
            "structural validation errors after round-trip: {:?}",
            structural_errors
        );
    }
}

// ═════════════════════════════════════════════════════════════════════════════
// Property: Ch10 payload encode → decode round-trip
// ═════════════════════════════════════════════════════════════════════════════

proptest! {
    #[test]
    fn prop_ch10_payload_round_trip(input in arb_simple_tmats()) {
        let doc = parse(&input).expect("parse failed");
        let csdw = SetupRecordCsdw::from_version(Irig106Version::V106_17, false);

        let payload = encode_setup_payload(&doc, &csdw).expect("encode failed");
        let bytes = payload.to_bytes();

        let (decoded_csdw, decoded_doc) = decode_setup_payload(&bytes)
            .expect("decode failed");

        prop_assert_eq!(decoded_csdw.ch10_version, csdw.ch10_version);
        prop_assert_eq!(
            doc.general.program_name.as_deref(),
            decoded_doc.general.program_name.as_deref(),
        );
    }
}

// ═════════════════════════════════════════════════════════════════════════════
// Property: into_owned preserves all data (L3-MODEL-016)
// ═════════════════════════════════════════════════════════════════════════════

proptest! {
    #[test]
    fn prop_into_owned_preserves_data(input in arb_tmats_ascii()) {
        let doc = parse(&input).expect("parse failed");
        let owned = doc.clone().into_owned();

        // Compare by re-serializing both
        let original_bytes = serialize_to_vec(&doc).expect("serialize original");
        let owned_bytes = serialize_to_vec(&owned).expect("serialize owned");

        prop_assert_eq!(
            original_bytes, owned_bytes,
            "into_owned() should not change serialized output"
        );
    }
}
