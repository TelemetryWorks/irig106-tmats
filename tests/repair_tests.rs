// tests/repair_tests.rs
//
// # Repair Engine Integration Tests
//
// ## Traceability:
//   L3-TEST-008: Repair idempotency tests
//   L2-REPAIR-001: Counter recomputation
//   L2-REPAIR-007: Dry-run mode

#![cfg(feature = "repair")]

use irig106_tmats::prelude::*;

// ═════════════════════════════════════════════════════════════════════════════
// Counter Recomputation (L2-REPAIR-001)
// ═════════════════════════════════════════════════════════════════════════════

#[test]
fn repair_fixes_g_dsi_counter() {
    let input = b"G\\PN:TEST;G\\106:17;G\\DSI\\N:99;G\\DSI-1:SRC1;";
    let mut doc = irig106_tmats::parse::parse(input).expect("parse failed");

    assert_eq!(doc.general.num_data_sources, Some(99));

    let report = repair(&mut doc, &RepairOptions::default());

    assert_eq!(doc.general.num_data_sources, Some(1));
    assert!(!report.actions_taken.is_empty());
    assert!(report.actions_taken.iter().any(|a| a.attribute_path == "G\\DSI\\N"));
}

#[test]
fn repair_fixes_r_n_counter() {
    let input = b"G\\PN:TEST;G\\106:17;R-1\\ID:REC;R-1\\N:10;R-1\\TK1-1:1;R-1\\TK1-2:2;";
    let mut doc = irig106_tmats::parse::parse(input).expect("parse failed");

    assert_eq!(doc.recorders[&1].num_channels, Some(10));

    let report = repair(&mut doc, &RepairOptions::default());

    assert_eq!(doc.recorders[&1].num_channels, Some(2));
    assert!(report.actions_taken.iter().any(|a| a.attribute_path == "R-1\\N"));
}

#[test]
fn repair_skips_correct_counters() {
    let input = b"G\\PN:TEST;G\\106:17;\
        G\\DSI\\N:2;G\\DSI-1:A;G\\DSI-2:B;\
        R-1\\ID:REC;R-1\\N:1;R-1\\TK1-1:1;";
    let mut doc = irig106_tmats::parse::parse(input).expect("parse failed");

    let report = repair(&mut doc, &RepairOptions::default());

    assert!(report.actions_taken.is_empty(), "no repairs should be needed");
}

// ═════════════════════════════════════════════════════════════════════════════
// Idempotency (L3-TEST-008)
// ═════════════════════════════════════════════════════════════════════════════

#[test]
fn repair_is_idempotent() {
    let input = b"G\\PN:TEST;G\\106:17;G\\DSI\\N:99;G\\DSI-1:SRC1;\
        R-1\\ID:REC;R-1\\N:50;R-1\\TK1-1:1;";
    let mut doc = irig106_tmats::parse::parse(input).expect("parse failed");

    // First repair
    let report1 = repair(&mut doc, &RepairOptions::default());
    assert!(!report1.actions_taken.is_empty());

    // Second repair — should have zero actions
    let report2 = repair(&mut doc, &RepairOptions::default());
    assert!(report2.actions_taken.is_empty(),
        "second repair pass should produce no changes");
}

// ═════════════════════════════════════════════════════════════════════════════
// Dry-Run Mode (L2-REPAIR-007)
// ═════════════════════════════════════════════════════════════════════════════

#[test]
fn repair_dry_run_does_not_modify() {
    let input = b"G\\PN:TEST;G\\106:17;G\\DSI\\N:99;G\\DSI-1:SRC1;";
    let doc = irig106_tmats::parse::parse(input).expect("parse failed");

    let report = repair_dry_run(&doc);

    assert!(report.was_dry_run);
    assert!(!report.findings.is_empty(), "should detect the counter mismatch");
    assert!(report.actions_taken.is_empty(), "dry-run should take no actions");

    // Original document unchanged
    assert_eq!(doc.general.num_data_sources, Some(99));
}

// ═════════════════════════════════════════════════════════════════════════════
// Repair Reduces Validation Errors (L3-REPAIR-008)
// ═════════════════════════════════════════════════════════════════════════════

#[test]
fn repair_reduces_validation_errors() {
    let input = b"G\\PN:TEST;G\\106:17;\
        G\\DSI\\N:5;G\\DSI-1:SRC;\
        R-1\\ID:REC;R-1\\N:99;R-1\\TK1-1:1;";
    let mut doc = irig106_tmats::parse::parse(input).expect("parse failed");

    let report = repair(&mut doc, &RepairOptions::default());

    assert!(report.post_repair_errors <= report.pre_repair_errors,
        "repair should not increase errors: pre={}, post={}",
        report.pre_repair_errors, report.post_repair_errors);
}

// ═════════════════════════════════════════════════════════════════════════════
// Case Normalization (L2-REPAIR-003)
// ═════════════════════════════════════════════════════════════════════════════

#[test]
fn repair_normalizes_keyword_case() {
    let input = b"G\\PN:TEST;G\\106:17;\
        G\\DSI\\N:1;G\\DSI-1:SRC;G\\DST-1:rec;\
        R-1\\ID:REC;R-1\\N:1;R-1\\TK1-1:1;R-1\\PDP-1:un;R-1\\CDLN-1:L;\
        P-1\\DLN:L;P-1\\D1:1000000;P-1\\D2:nrz-l;P-1\\F1:128;P-1\\F2:16;P-1\\F3:FE;";
    let mut doc = irig106_tmats::parse::parse(input).expect("parse failed");

    let report = repair(&mut doc, &RepairOptions {
        auto_fix_case: true,
        ..Default::default()
    });

    // Should have normalized lowercase keywords
    let case_actions: Vec<_> = report.actions_taken.iter()
        .filter(|a| a.rule_id == "TMATS-R003")
        .collect();
    assert!(!case_actions.is_empty(), "should have case normalization actions");

    // Verify the actual values were changed
    let packing = doc.recorders[&1].channels[&1].data_packing_option.as_deref();
    assert_eq!(packing, Some("UN"), "PDP should be normalized to uppercase");

    let encoding = doc.pcm_formats[&1].encoding.as_deref();
    assert_eq!(encoding, Some("NRZ-L"), "encoding should be normalized to uppercase");
}

#[test]
fn repair_case_normalization_skipped_when_disabled() {
    let input = b"G\\PN:TEST;G\\106:17;\
        R-1\\ID:REC;R-1\\N:1;R-1\\TK1-1:1;R-1\\PDP-1:un;";
    let mut doc = irig106_tmats::parse::parse(input).expect("parse failed");

    let report = repair(&mut doc, &RepairOptions {
        auto_fix_case: false,
        ..Default::default()
    });

    let case_actions: Vec<_> = report.actions_taken.iter()
        .filter(|a| a.rule_id == "TMATS-R003")
        .collect();
    assert!(case_actions.is_empty(), "should not normalize when disabled");

    // Value should still be lowercase
    assert_eq!(doc.recorders[&1].channels[&1].data_packing_option.as_deref(), Some("un"));
}

// ═════════════════════════════════════════════════════════════════════════════
// Orphan Detection (L2-REPAIR-005)
// ═════════════════════════════════════════════════════════════════════════════

#[test]
fn repair_detects_orphan_channels() {
    let input = b"G\\PN:TEST;G\\106:17;\
        R-1\\ID:REC;R-1\\N:1;R-1\\TK1-1:1;R-1\\CDLN-1:NONEXISTENT_LINK;";
    let doc = irig106_tmats::parse::parse(input).expect("parse failed");

    let report = irig106_tmats::repair::repair_dry_run(&doc);

    let orphan_findings: Vec<_> = report.findings.iter()
        .filter(|f| f.rule_id == "TMATS-R005")
        .collect();
    assert!(!orphan_findings.is_empty(),
        "should detect orphan CDLN reference to NONEXISTENT_LINK");
    assert!(!orphan_findings[0].auto_fixable,
        "orphan detection should not be auto-fixable");
}

#[test]
fn repair_no_orphans_when_links_resolve() {
    let input = b"G\\PN:TEST;G\\106:17;\
        R-1\\ID:REC;R-1\\N:1;R-1\\TK1-1:1;R-1\\CDLN-1:PCM1;\
        P-1\\DLN:PCM1;P-1\\D1:1000000;P-1\\D2:NRZ-L;P-1\\F1:128;P-1\\F2:16;P-1\\F3:FE;";
    let doc = irig106_tmats::parse::parse(input).expect("parse failed");

    let report = irig106_tmats::repair::repair_dry_run(&doc);

    let orphan_findings: Vec<_> = report.findings.iter()
        .filter(|f| f.rule_id == "TMATS-R005")
        .collect();
    assert!(orphan_findings.is_empty(), "should have no orphans when links resolve");
}
