// tests/version_registry_tests.rs
//
// # Version Registry and Registry-Driven Validation Tests
//
// ## Traceability:
//   L3-TEST-003: Version-specific validation tests
//   L2-VALID-003: Keyword validation
//   L2-VALID-004: MFS enforcement
//   L2-VALID-007: Date format validation
//   L3-VERSION-005: AttrMetaRegistry
//   L3-VERSION-010: Migration diff

use irig106_tmats::prelude::*;

// ═════════════════════════════════════════════════════════════════════════════
// Registry Basics (L3-VERSION-005)
// ═════════════════════════════════════════════════════════════════════════════

#[test]
fn registry_has_entries() {
    let reg = registry();
    assert!(reg.len() > 0, "registry should not be empty");
}

#[test]
fn registry_lookup_g_pn() {
    let reg = registry();
    let meta = reg.get("G\\PN").expect("G\\PN should be in registry");
    assert_eq!(meta.display_name, "Program Name");
    assert_eq!(meta.required_tag, RequiredTag::Required);
    assert_eq!(meta.max_field_size, Some(32));
}

#[test]
fn registry_lookup_r_group() {
    let reg = registry();
    let meta = reg.get("R-x\\PDP-n").expect("R-x\\PDP-n should be in registry");
    assert_eq!(meta.display_name, "Data Packing Option");
    assert!(meta.keywords.contains(&"UN"));
    assert!(meta.keywords.contains(&"TM"));
    assert!(meta.keywords.contains(&"PFS"));
}

#[test]
fn registry_group_filter() {
    let reg = registry();
    let g_attrs = reg.group(GroupPrefix::G);
    assert!(g_attrs.len() >= 5, "G-group should have at least 5 registered attributes");
    for meta in g_attrs {
        assert_eq!(meta.group, GroupPrefix::G);
    }
}

#[test]
fn registry_version_filter() {
    let reg = registry();

    // G\106 was introduced in 106-07
    let v04_attrs = reg.for_version(Irig106Version::V106_04);
    let v07_attrs = reg.for_version(Irig106Version::V106_07);

    // 106-07 should have at least as many attrs as 106-04
    assert!(v07_attrs.len() >= v04_attrs.len());

    // G\106 should not be in 106-04 set
    assert!(!v04_attrs.iter().any(|m| m.code_name_pattern == "G\\106"));
    // G\106 should be in 106-07 set
    assert!(v07_attrs.iter().any(|m| m.code_name_pattern == "G\\106"));
}

#[test]
fn registry_required_for_ch10() {
    let reg = registry();
    let ch10_req = reg.required_for_ch10(Irig106Version::V106_17);

    // Should include R-x\ID (RequiredCh10)
    assert!(ch10_req.iter().any(|m| m.code_name_pattern == "R-x\\ID"));
    // Should include G\PN (Required — applies to all contexts including Ch10)
    assert!(ch10_req.iter().any(|m| m.code_name_pattern == "G\\PN"));
}

// ═════════════════════════════════════════════════════════════════════════════
// AttrMeta Validation Methods
// ═════════════════════════════════════════════════════════════════════════════

#[test]
fn attr_meta_keyword_validation() {
    let reg = registry();
    let pdp = reg.get("R-x\\PDP-n").unwrap();

    assert!(pdp.validate_keyword("UN"));
    assert!(pdp.validate_keyword("un")); // case-insensitive
    assert!(pdp.validate_keyword("PFS"));
    assert!(pdp.validate_keyword("TM"));
    assert!(!pdp.validate_keyword("INVALID"));
    assert!(!pdp.validate_keyword(""));
}

#[test]
fn attr_meta_mfs_validation() {
    let reg = registry();
    let pn = reg.get("G\\PN").unwrap();

    assert!(pn.validate_mfs("SHORT"));
    assert!(pn.validate_mfs(&"A".repeat(32))); // exactly 32
    assert!(!pn.validate_mfs(&"A".repeat(33))); // exceeds 32
}

#[test]
fn attr_meta_version_check() {
    let reg = registry();
    let g106 = reg.get("G\\106").unwrap();

    // G\106 introduced in 106-07
    assert!(!g106.is_valid_for(Irig106Version::V106_04));
    assert!(!g106.is_valid_for(Irig106Version::V106_05));
    assert!(g106.is_valid_for(Irig106Version::V106_07));
    assert!(g106.is_valid_for(Irig106Version::V106_17));
}

// ═════════════════════════════════════════════════════════════════════════════
// Migration Diff (L3-VERSION-010, L2-VERSION-004)
// ═════════════════════════════════════════════════════════════════════════════

#[test]
fn migration_diff_04_to_07() {
    let reg = registry();
    let diff = reg.migration_diff(Irig106Version::V106_04, Irig106Version::V106_07);

    // G\106 was added in 106-07
    assert!(diff.added.iter().any(|m| m.code_name_pattern == "G\\106"),
        "G\\106 should appear as added in 04→07 migration");

    // R-x\CDT-n was added in 106-07
    assert!(diff.added.iter().any(|m| m.code_name_pattern == "R-x\\CDT-n"),
        "R-x\\CDT-n should appear as added in 04→07 migration");

    assert!(diff.has_changes());
}

#[test]
fn migration_diff_same_version() {
    let reg = registry();
    let diff = reg.migration_diff(Irig106Version::V106_17, Irig106Version::V106_17);
    assert!(diff.added.is_empty());
    assert!(diff.removed.is_empty());
}

// ═════════════════════════════════════════════════════════════════════════════
// Keyword Validation Rules (L2-VALID-003)
// ═════════════════════════════════════════════════════════════════════════════

#[test]
fn validate_invalid_data_source_type() {
    let input = b"G\\PN:TEST;G\\106:17;G\\DSI\\N:1;G\\DSI-1:SRC;G\\DST-1:BADTYPE;";
    let doc = parse(input).expect("parse failed");
    let report = validate(&doc);

    assert!(report.diagnostics.iter().any(|d| d.rule_id == "TMATS-V030"),
        "should flag invalid keyword for G\\DST-1");
}

#[test]
fn validate_valid_data_source_type() {
    let input = b"G\\PN:TEST;G\\106:17;G\\DSI\\N:1;G\\DSI-1:SRC;G\\DST-1:REC;";
    let doc = parse(input).expect("parse failed");
    let report = validate(&doc);

    assert!(!report.diagnostics.iter().any(|d| d.rule_id == "TMATS-V030"),
        "REC is a valid keyword, should not be flagged");
}

#[test]
fn validate_invalid_packing_option() {
    let input = b"G\\PN:TEST;G\\106:17;\
        R-1\\ID:REC;R-1\\N:1;R-1\\TK1-1:1;R-1\\PDP-1:BADPACK;R-1\\CDLN-1:L;\
        P-1\\DLN:L;P-1\\D1:1000000;P-1\\D2:NRZ-L;P-1\\F1:128;P-1\\F2:16;P-1\\F3:FE6B;";
    let doc = parse(input).expect("parse failed");
    let report = validate(&doc);

    assert!(report.diagnostics.iter().any(|d| d.rule_id == "TMATS-V031"),
        "should flag invalid packing keyword");
}

#[test]
fn validate_invalid_pcm_encoding() {
    let input = b"G\\PN:TEST;G\\106:17;\
        P-1\\DLN:PCM1;P-1\\D1:1000000;P-1\\D2:INVALID_ENC;P-1\\F1:128;P-1\\F2:16;P-1\\F3:FE;";
    let doc = parse(input).expect("parse failed");
    let report = validate(&doc);

    assert!(report.diagnostics.iter().any(|d| d.rule_id == "TMATS-V032"),
        "should flag invalid PCM encoding");
}

#[test]
fn validate_valid_pcm_encoding() {
    let input = b"G\\PN:TEST;G\\106:17;\
        P-1\\DLN:PCM1;P-1\\D1:1000000;P-1\\D2:NRZ-L;P-1\\F1:128;P-1\\F2:16;P-1\\F3:FE;";
    let doc = parse(input).expect("parse failed");
    let report = validate(&doc);

    assert!(!report.diagnostics.iter().any(|d| d.rule_id == "TMATS-V032"),
        "NRZ-L is valid");
}

#[test]
fn validate_invalid_bus_type() {
    let input = b"G\\PN:TEST;G\\106:17;B-1\\DLN:BUS1;B-1\\BT:CAN;";
    let doc = parse(input).expect("parse failed");
    let report = validate(&doc);

    assert!(report.diagnostics.iter().any(|d| d.rule_id == "TMATS-V033"),
        "CAN is not a valid B\\BT keyword (should be 1553 or A429)");
}

// ═════════════════════════════════════════════════════════════════════════════
// MFS Enforcement (L2-VALID-004)
// ═════════════════════════════════════════════════════════════════════════════

#[test]
fn validate_mfs_exceeded_program_name() {
    let long_name = "A".repeat(50);
    let input = format!("G\\PN:{};G\\106:17;", long_name);
    let doc = parse(input.as_bytes()).expect("parse failed");
    let report = validate(&doc);

    assert!(report.diagnostics.iter().any(|d| d.rule_id == "TMATS-V050"),
        "should flag program name exceeding 32 char MFS");
}

#[test]
fn validate_mfs_ok_program_name() {
    let input = b"G\\PN:REASONABLE_NAME;G\\106:17;";
    let doc = parse(input).expect("parse failed");
    let report = validate(&doc);

    assert!(!report.diagnostics.iter().any(|d| d.rule_id == "TMATS-V050"),
        "should not flag short program name");
}

// ═════════════════════════════════════════════════════════════════════════════
// Date Validation (L2-VALID-007)
// ═════════════════════════════════════════════════════════════════════════════

#[test]
fn validate_invalid_date_month() {
    let input = b"G\\PN:TEST;G\\106:17;G\\OD:13-01-2024;";
    let doc = parse(input).expect("parse failed");
    let report = validate(&doc);

    assert!(report.diagnostics.iter().any(|d| d.rule_id == "TMATS-V040"),
        "month 13 is invalid");
}

#[test]
fn validate_valid_date() {
    let input = b"G\\PN:TEST;G\\106:17;G\\OD:06-15-2024;";
    let doc = parse(input).expect("parse failed");
    let report = validate(&doc);

    assert!(!report.diagnostics.iter().any(|d| d.rule_id == "TMATS-V040"));
}

// ═════════════════════════════════════════════════════════════════════════════
// P-Group / B-Group Required Attrs (L2-VALID-001)
// ═════════════════════════════════════════════════════════════════════════════

#[test]
fn validate_p_group_missing_required() {
    // P-group with only DLN, missing D1, D2, F1, F2, F3
    let input = b"G\\PN:TEST;G\\106:17;P-1\\DLN:PCM1;";
    let doc = parse(input).expect("parse failed");
    let report = validate(&doc);

    let p_errors: Vec<_> = report.diagnostics.iter()
        .filter(|d| d.rule_id == "TMATS-V060")
        .collect();
    assert!(p_errors.len() >= 4,
        "should flag missing D1, D2, F1, F2, F3: found {} errors", p_errors.len());
}

#[test]
fn validate_p_group_complete() {
    let input = b"G\\PN:TEST;G\\106:17;\
        P-1\\DLN:PCM1;P-1\\D1:1000000;P-1\\D2:NRZ-L;P-1\\F1:128;P-1\\F2:16;P-1\\F3:FE6B;";
    let doc = parse(input).expect("parse failed");
    let report = validate(&doc);

    assert!(!report.diagnostics.iter().any(|d| d.rule_id == "TMATS-V060"),
        "complete P-group should pass");
}

#[test]
fn validate_b_group_missing_required() {
    let input = b"G\\PN:TEST;G\\106:17;B-1\\DLN:BUS1;";
    let doc = parse(input).expect("parse failed");
    let report = validate(&doc);

    assert!(report.diagnostics.iter().any(|d| d.rule_id == "TMATS-V061"),
        "should flag missing B\\BT");
}
