// irig106-tmats/src/validate.rs
//
// # TMATS Validation Engine
//
// Rule-based validation of TMATS documents against Ch9 §9.5 normative tables.
//
// ## Traceability:
//   L1-VALID → L2-VALID-001..010 → L3-VALID-001..010

use crate::error::{Diagnostic, Severity};
use crate::model::TmatsDocument;
use crate::types_bridge::Irig106Version;

// ─── Validation Profile (L3-VALID-007) ───────────────────────────────────────

/// Selects which rules to apply.
///
/// **Requirement:** L2-VALID-010, L3-VALID-007
#[derive(Debug, Clone, Default)]
pub enum ValidationProfile {
    /// All rules.
    #[default]
    Full,
    /// Only R-CH10 and RO-CH10 tagged requirements.
    Ch10Required,
    /// Custom set of rule IDs.
    Custom(Vec<String>),
}

// ─── Validation Context (L3-VALID-004) ───────────────────────────────────────

/// Context provided to validation rules.
///
/// **Requirement:** L3-VALID-004
#[derive(Debug, Clone)]
pub struct ValidationContext {
    /// Target IRIG 106 version for version-specific rules.
    pub target_version: Option<Irig106Version>,
    /// Whether the source is a Ch10 file (enables R-CH10 rules).
    pub source: TmatsSource,
    /// Which rules to apply.
    pub profile: ValidationProfile,
}

impl Default for ValidationContext {
    fn default() -> Self {
        Self {
            target_version: None,
            source: TmatsSource::Unknown,
            profile: ValidationProfile::Full,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TmatsSource {
    /// Extracted from a Ch10 setup record.
    Ch10,
    /// Standalone TMATS file (.TMT/.TMA/.TXT).
    Standalone,
    /// Origin unknown.
    Unknown,
}

// ─── Validation Rule Trait (L3-VALID-002) ────────────────────────────────────

/// A single validation rule.
///
/// **Requirement:** L3-VALID-002
pub trait ValidationRule: Send + Sync {
    /// Unique rule identifier (e.g., "TMATS-V001").
    fn id(&self) -> &str;
    /// Human-readable description.
    fn description(&self) -> &str;
    /// Severity of findings from this rule.
    fn severity(&self) -> Severity;
    /// Spec section reference.
    fn spec_ref(&self) -> &str;
    /// Which IRIG 106 versions this rule applies to (empty = all).
    fn applicable_versions(&self) -> &[Irig106Version];
    /// Execute the rule against a document.
    fn check(&self, doc: &TmatsDocument<'_>, ctx: &ValidationContext) -> Vec<Diagnostic>;
}

// ─── Validation Report (L3-VALID-010) ────────────────────────────────────────

/// Results of validation.
///
/// **Requirement:** L3-VALID-010, L3-INTEROP-004
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct ValidationReport {
    pub diagnostics: Vec<Diagnostic>,
    pub version_detected: Option<Irig106Version>,
    pub total_attributes: usize,
    pub errors: usize,
    pub warnings: usize,
    pub info: usize,
}

impl ValidationReport {
    pub fn has_errors(&self) -> bool {
        self.errors > 0
    }
}

// ─── Built-in Rules (L3-VALID-003) ──────────────────────────────────────────

/// G-group required attributes (G\PN is required in all versions).
///
/// **Requirement:** L2-VALID-001
struct RequiredGProgramName;

impl ValidationRule for RequiredGProgramName {
    fn id(&self) -> &str {
        "TMATS-V001"
    }
    fn description(&self) -> &str {
        "G\\PN (Program Name) is required"
    }
    fn severity(&self) -> Severity {
        Severity::Error
    }
    fn spec_ref(&self) -> &str {
        "Ch9 §9.5.2 Table 9-1"
    }
    fn applicable_versions(&self) -> &[Irig106Version] {
        &[]
    } // All versions

    fn check(&self, doc: &TmatsDocument<'_>, _ctx: &ValidationContext) -> Vec<Diagnostic> {
        if doc.general.program_name.is_none() {
            vec![Diagnostic {
                severity: Severity::Error,
                rule_id: self.id().to_string(),
                message: "missing required attribute G\\PN (Program Name)".into(),
                spec_reference: self.spec_ref().to_string(),
                attribute_path: Some("G\\PN".into()),
                expected: Some("non-empty string".into()),
                actual: Some("absent".into()),
                suggested_fix: Some("add G\\PN:<program name>;".into()),
            }]
        } else {
            vec![]
        }
    }
}

/// G\106 version attribute.
struct RequiredGVersion;

impl ValidationRule for RequiredGVersion {
    fn id(&self) -> &str {
        "TMATS-V002"
    }
    fn description(&self) -> &str {
        "G\\106 (IRIG 106 Version) is required"
    }
    fn severity(&self) -> Severity {
        Severity::Error
    }
    fn spec_ref(&self) -> &str {
        "Ch9 §9.5.2 Table 9-1"
    }
    fn applicable_versions(&self) -> &[Irig106Version] {
        &[]
    }

    fn check(&self, doc: &TmatsDocument<'_>, _ctx: &ValidationContext) -> Vec<Diagnostic> {
        if doc.general.irig106_version.is_none() {
            vec![Diagnostic {
                severity: Severity::Error,
                rule_id: self.id().to_string(),
                message: "missing required attribute G\\106 (IRIG 106 Version)".into(),
                spec_reference: self.spec_ref().to_string(),
                attribute_path: Some("G\\106".into()),
                expected: Some("version string (e.g., '07', '17')".into()),
                actual: Some("absent".into()),
                suggested_fix: Some("add G\\106:<version>;".into()),
            }]
        } else {
            vec![]
        }
    }
}

/// Counter consistency check: G\DSI\N matches actual data source count.
///
/// **Requirement:** L2-VALID-005
struct CounterConsistencyGDsi;

impl ValidationRule for CounterConsistencyGDsi {
    fn id(&self) -> &str {
        "TMATS-V010"
    }
    fn description(&self) -> &str {
        "G\\DSI\\N counter matches data source count"
    }
    fn severity(&self) -> Severity {
        Severity::Error
    }
    fn spec_ref(&self) -> &str {
        "Ch9 §9.5.2 Table 9-1"
    }
    fn applicable_versions(&self) -> &[Irig106Version] {
        &[]
    }

    fn check(&self, doc: &TmatsDocument<'_>, _ctx: &ValidationContext) -> Vec<Diagnostic> {
        if let Some(declared) = doc.general.num_data_sources {
            let actual = doc.general.data_sources.len() as u32;
            if declared != actual {
                return vec![Diagnostic {
                    severity: Severity::Error,
                    rule_id: self.id().to_string(),
                    message: format!(
                        "G\\DSI\\N declares {declared} data sources but {actual} found"
                    ),
                    spec_reference: self.spec_ref().to_string(),
                    attribute_path: Some("G\\DSI\\N".into()),
                    expected: Some(actual.to_string()),
                    actual: Some(declared.to_string()),
                    suggested_fix: Some(format!("change G\\DSI\\N to {actual}")),
                }];
            }
        }
        vec![]
    }
}

/// R-group channel counter consistency.
struct CounterConsistencyRN;

impl ValidationRule for CounterConsistencyRN {
    fn id(&self) -> &str {
        "TMATS-V011"
    }
    fn description(&self) -> &str {
        "R-x\\N counter matches channel count"
    }
    fn severity(&self) -> Severity {
        Severity::Error
    }
    fn spec_ref(&self) -> &str {
        "Ch9 §9.5.4"
    }
    fn applicable_versions(&self) -> &[Irig106Version] {
        &[]
    }

    fn check(&self, doc: &TmatsDocument<'_>, _ctx: &ValidationContext) -> Vec<Diagnostic> {
        let mut diags = Vec::new();
        for (&idx, r) in &doc.recorders {
            if let Some(declared) = r.num_channels {
                let actual = r.channels.len() as u32;
                if declared != actual {
                    diags.push(Diagnostic {
                        severity: Severity::Error,
                        rule_id: self.id().to_string(),
                        message: format!(
                            "R-{idx}\\N declares {declared} channels but {actual} found"
                        ),
                        spec_reference: self.spec_ref().to_string(),
                        attribute_path: Some(format!("R-{idx}\\N")),
                        expected: Some(actual.to_string()),
                        actual: Some(declared.to_string()),
                        suggested_fix: Some(format!("change R-{idx}\\N to {actual}")),
                    });
                }
            }
        }
        diags
    }
}

/// Cross-group reference integrity: R-group CDLN → P/B/S group DLN.
///
/// **Requirement:** L2-VALID-006
struct CrossGroupRefCdln;

impl ValidationRule for CrossGroupRefCdln {
    fn id(&self) -> &str {
        "TMATS-V020"
    }
    fn description(&self) -> &str {
        "R-x\\CDLN-n references a valid P/B/S data link name"
    }
    fn severity(&self) -> Severity {
        Severity::Warning
    }
    fn spec_ref(&self) -> &str {
        "Ch9 §9.5.1 Figure 9-1"
    }
    fn applicable_versions(&self) -> &[Irig106Version] {
        &[]
    }

    fn check(&self, doc: &TmatsDocument<'_>, _ctx: &ValidationContext) -> Vec<Diagnostic> {
        let mut diags = Vec::new();

        // Collect all known data link names from format groups
        let mut known_dlns: Vec<&str> = Vec::new();
        for p in doc.pcm_formats.values() {
            if let Some(ref dln) = p.data_link_name {
                known_dlns.push(dln);
            }
        }
        for b in doc.bus_data.values() {
            if let Some(ref dln) = b.data_link_name {
                known_dlns.push(dln);
            }
        }
        for s in doc.message_data.values() {
            if let Some(ref dln) = s.data_link_name {
                known_dlns.push(dln);
            }
        }

        // Check each R-group channel's CDLN
        for (&r_idx, r) in &doc.recorders {
            for (&ch_idx, ch) in &r.channels {
                if let Some(ref cdln) = ch.data_link_name {
                    if !known_dlns.iter().any(|dln| dln.eq_ignore_ascii_case(cdln)) {
                        diags.push(Diagnostic {
                            severity: Severity::Warning,
                            rule_id: self.id().to_string(),
                            message: format!(
                                "R-{r_idx}\\CDLN-{ch_idx} references '{}' which has no matching P/B/S group",
                                cdln
                            ),
                            spec_reference: self.spec_ref().to_string(),
                            attribute_path: Some(format!("R-{r_idx}\\CDLN-{ch_idx}")),
                            expected: Some("valid data link name from P/B/S group".into()),
                            actual: Some(cdln.to_string()),
                            suggested_fix: None,
                        });
                    }
                }
            }
        }

        diags
    }
}

// ─── Registry-Driven Rules (L2-VALID-003, L2-VALID-004, L2-VALID-007) ───────

/// Keyword validation for G-group data source type.
///
/// **Requirement:** L2-VALID-003
struct KeywordValidationGDst;

impl ValidationRule for KeywordValidationGDst {
    fn id(&self) -> &str {
        "TMATS-V030"
    }
    fn description(&self) -> &str {
        "G\\DST-n value must be a valid keyword"
    }
    fn severity(&self) -> Severity {
        Severity::Error
    }
    fn spec_ref(&self) -> &str {
        "Ch9 §9.5.2 Table 9-1"
    }
    fn applicable_versions(&self) -> &[Irig106Version] {
        &[]
    }

    fn check(&self, doc: &TmatsDocument<'_>, _ctx: &ValidationContext) -> Vec<Diagnostic> {
        let reg = crate::version::registry();
        let meta = match reg.get("G\\DST-n") {
            Some(m) => m,
            None => return vec![],
        };

        let mut diags = Vec::new();
        for (&idx, ds) in &doc.general.data_sources {
            if let Some(ref val) = ds.data_source_type {
                if !meta.validate_keyword(val) {
                    diags.push(Diagnostic {
                        severity: Severity::Error,
                        rule_id: self.id().to_string(),
                        message: format!("G\\DST-{idx} value '{}' is not a valid keyword", val),
                        spec_reference: self.spec_ref().to_string(),
                        attribute_path: Some(format!("G\\DST-{idx}")),
                        expected: Some(meta.keywords.join(", ")),
                        actual: Some(val.to_string()),
                        suggested_fix: Some(format!("use one of: {}", meta.keywords.join(", "))),
                    });
                }
            }
        }
        diags
    }
}

/// Keyword validation for R-group data packing option.
///
/// **Requirement:** L2-VALID-003
struct KeywordValidationRPdp;

impl ValidationRule for KeywordValidationRPdp {
    fn id(&self) -> &str {
        "TMATS-V031"
    }
    fn description(&self) -> &str {
        "R-x\\PDP-n must be a valid packing keyword"
    }
    fn severity(&self) -> Severity {
        Severity::Error
    }
    fn spec_ref(&self) -> &str {
        "Ch9 §9.5.4"
    }
    fn applicable_versions(&self) -> &[Irig106Version] {
        &[]
    }

    fn check(&self, doc: &TmatsDocument<'_>, _ctx: &ValidationContext) -> Vec<Diagnostic> {
        let reg = crate::version::registry();
        let meta = match reg.get("R-x\\PDP-n") {
            Some(m) => m,
            None => return vec![],
        };

        let mut diags = Vec::new();
        for (&r_idx, r) in &doc.recorders {
            for (&ch_idx, ch) in &r.channels {
                if let Some(ref val) = ch.data_packing_option {
                    if !meta.validate_keyword(val) {
                        diags.push(Diagnostic {
                            severity: Severity::Error,
                            rule_id: self.id().to_string(),
                            message: format!(
                                "R-{r_idx}\\PDP-{ch_idx} value '{}' is not a valid packing option",
                                val
                            ),
                            spec_reference: self.spec_ref().to_string(),
                            attribute_path: Some(format!("R-{r_idx}\\PDP-{ch_idx}")),
                            expected: Some(meta.keywords.join(", ")),
                            actual: Some(val.to_string()),
                            suggested_fix: Some(format!(
                                "use one of: {}",
                                meta.keywords.join(", ")
                            )),
                        });
                    }
                }
            }
        }
        diags
    }
}

/// Keyword validation for P-group encoding.
///
/// **Requirement:** L2-VALID-003
struct KeywordValidationPEncoding;

impl ValidationRule for KeywordValidationPEncoding {
    fn id(&self) -> &str {
        "TMATS-V032"
    }
    fn description(&self) -> &str {
        "P-n\\D2 must be a valid PCM encoding keyword"
    }
    fn severity(&self) -> Severity {
        Severity::Error
    }
    fn spec_ref(&self) -> &str {
        "Ch9 §9.5.6.1"
    }
    fn applicable_versions(&self) -> &[Irig106Version] {
        &[]
    }

    fn check(&self, doc: &TmatsDocument<'_>, _ctx: &ValidationContext) -> Vec<Diagnostic> {
        let reg = crate::version::registry();
        let meta = match reg.get("P-n\\D2") {
            Some(m) => m,
            None => return vec![],
        };

        let mut diags = Vec::new();
        for (&idx, p) in &doc.pcm_formats {
            if let Some(ref val) = p.encoding {
                if !meta.validate_keyword(val) {
                    diags.push(Diagnostic {
                        severity: Severity::Error,
                        rule_id: self.id().to_string(),
                        message: format!("P-{idx}\\D2 encoding '{}' is not valid", val),
                        spec_reference: self.spec_ref().to_string(),
                        attribute_path: Some(format!("P-{idx}\\D2")),
                        expected: Some(meta.keywords.join(", ")),
                        actual: Some(val.to_string()),
                        suggested_fix: None,
                    });
                }
            }
        }
        diags
    }
}

/// Keyword validation for B-group bus type.
///
/// **Requirement:** L2-VALID-003
struct KeywordValidationBBusType;

impl ValidationRule for KeywordValidationBBusType {
    fn id(&self) -> &str {
        "TMATS-V033"
    }
    fn description(&self) -> &str {
        "B-n\\BT must be a valid bus type keyword"
    }
    fn severity(&self) -> Severity {
        Severity::Error
    }
    fn spec_ref(&self) -> &str {
        "Ch9 §9.5.6.3"
    }
    fn applicable_versions(&self) -> &[Irig106Version] {
        &[]
    }

    fn check(&self, doc: &TmatsDocument<'_>, _ctx: &ValidationContext) -> Vec<Diagnostic> {
        let reg = crate::version::registry();
        let meta = match reg.get("B-n\\BT") {
            Some(m) => m,
            None => return vec![],
        };

        let mut diags = Vec::new();
        for (&idx, b) in &doc.bus_data {
            if let Some(ref val) = b.bus_type {
                if !meta.validate_keyword(val) {
                    diags.push(Diagnostic {
                        severity: Severity::Error,
                        rule_id: self.id().to_string(),
                        message: format!("B-{idx}\\BT bus type '{}' is not valid", val),
                        spec_reference: self.spec_ref().to_string(),
                        attribute_path: Some(format!("B-{idx}\\BT")),
                        expected: Some(meta.keywords.join(", ")),
                        actual: Some(val.to_string()),
                        suggested_fix: None,
                    });
                }
            }
        }
        diags
    }
}

/// Date format validation for G-group dates.
///
/// **Requirement:** L2-VALID-007
struct DateFormatValidation;

impl ValidationRule for DateFormatValidation {
    fn id(&self) -> &str {
        "TMATS-V040"
    }
    fn description(&self) -> &str {
        "Date attributes must be valid MM-DD-YYYY"
    }
    fn severity(&self) -> Severity {
        Severity::Warning
    }
    fn spec_ref(&self) -> &str {
        "Ch9 §9.5.2 Table 9-1"
    }
    fn applicable_versions(&self) -> &[Irig106Version] {
        &[]
    }

    fn check(&self, doc: &TmatsDocument<'_>, _ctx: &ValidationContext) -> Vec<Diagnostic> {
        let mut diags = Vec::new();

        if let Some(d) = &doc.general.origination_date {
            if d.month == 0 || d.month > 12 || d.day == 0 || d.day > 31 {
                diags.push(Diagnostic {
                    severity: Severity::Warning,
                    rule_id: self.id().to_string(),
                    message: format!("G\\OD date '{}' has invalid month/day values", d),
                    spec_reference: self.spec_ref().to_string(),
                    attribute_path: Some("G\\OD".into()),
                    expected: Some("valid MM-DD-YYYY (1≤MM≤12, 1≤DD≤31)".into()),
                    actual: Some(d.to_string()),
                    suggested_fix: None,
                });
            }
        }

        if let Some(d) = &doc.general.revision_date {
            if d.month == 0 || d.month > 12 || d.day == 0 || d.day > 31 {
                diags.push(Diagnostic {
                    severity: Severity::Warning,
                    rule_id: self.id().to_string(),
                    message: format!("G\\RD date '{}' has invalid month/day values", d),
                    spec_reference: self.spec_ref().to_string(),
                    attribute_path: Some("G\\RD".into()),
                    expected: Some("valid MM-DD-YYYY".into()),
                    actual: Some(d.to_string()),
                    suggested_fix: None,
                });
            }
        }

        diags
    }
}

/// MFS enforcement for G\\PN (program name max 32 chars).
///
/// **Requirement:** L2-VALID-004
struct MfsValidationGPn;

impl ValidationRule for MfsValidationGPn {
    fn id(&self) -> &str {
        "TMATS-V050"
    }
    fn description(&self) -> &str {
        "G\\PN must not exceed 32 characters (MFS)"
    }
    fn severity(&self) -> Severity {
        Severity::Warning
    }
    fn spec_ref(&self) -> &str {
        "Ch9 §9.5.2 Table 9-1"
    }
    fn applicable_versions(&self) -> &[Irig106Version] {
        &[]
    }

    fn check(&self, doc: &TmatsDocument<'_>, _ctx: &ValidationContext) -> Vec<Diagnostic> {
        let reg = crate::version::registry();
        let meta = match reg.get("G\\PN") {
            Some(m) => m,
            None => return vec![],
        };

        if let Some(ref pn) = doc.general.program_name {
            if !meta.validate_mfs(pn) {
                return vec![Diagnostic {
                    severity: Severity::Warning,
                    rule_id: self.id().to_string(),
                    message: format!(
                        "G\\PN '{}' exceeds MFS of {} characters (actual: {})",
                        pn,
                        meta.max_field_size.unwrap_or(0),
                        pn.len()
                    ),
                    spec_reference: self.spec_ref().to_string(),
                    attribute_path: Some("G\\PN".into()),
                    expected: Some(format!("≤{} chars", meta.max_field_size.unwrap_or(0))),
                    actual: Some(pn.len().to_string()),
                    suggested_fix: Some("truncate program name".into()),
                }];
            }
        }
        vec![]
    }
}

/// P-group required attributes: DLN and D1 must be present.
///
/// **Requirement:** L2-VALID-001
struct RequiredPGroupAttrs;

impl ValidationRule for RequiredPGroupAttrs {
    fn id(&self) -> &str {
        "TMATS-V060"
    }
    fn description(&self) -> &str {
        "P-group required attributes (DLN, D1, D2, F1, F2, F3)"
    }
    fn severity(&self) -> Severity {
        Severity::Error
    }
    fn spec_ref(&self) -> &str {
        "Ch9 §9.5.6.1"
    }
    fn applicable_versions(&self) -> &[Irig106Version] {
        &[]
    }

    fn check(&self, doc: &TmatsDocument<'_>, _ctx: &ValidationContext) -> Vec<Diagnostic> {
        let mut diags = Vec::new();

        for (&idx, p) in &doc.pcm_formats {
            let pfx = format!("P-{idx}");
            if p.data_link_name.is_none() {
                diags.push(missing_required_diag(
                    self,
                    &format!("{pfx}\\DLN"),
                    "Data Link Name",
                ));
            }
            if p.bit_rate.is_none() {
                diags.push(missing_required_diag(
                    self,
                    &format!("{pfx}\\D1"),
                    "Bit Rate",
                ));
            }
            if p.encoding.is_none() {
                diags.push(missing_required_diag(
                    self,
                    &format!("{pfx}\\D2"),
                    "Encoding",
                ));
            }
            if p.num_words_per_frame.is_none() {
                diags.push(missing_required_diag(
                    self,
                    &format!("{pfx}\\F1"),
                    "Words per Frame",
                ));
            }
            if p.num_bits_per_word.is_none() {
                diags.push(missing_required_diag(
                    self,
                    &format!("{pfx}\\F2"),
                    "Bits per Word",
                ));
            }
            if p.sync_pattern.is_none() {
                diags.push(missing_required_diag(
                    self,
                    &format!("{pfx}\\F3"),
                    "Sync Pattern",
                ));
            }
        }

        diags
    }
}

/// B-group required attributes: DLN and BT must be present.
///
/// **Requirement:** L2-VALID-001
struct RequiredBGroupAttrs;

impl ValidationRule for RequiredBGroupAttrs {
    fn id(&self) -> &str {
        "TMATS-V061"
    }
    fn description(&self) -> &str {
        "B-group required attributes (DLN, BT)"
    }
    fn severity(&self) -> Severity {
        Severity::Error
    }
    fn spec_ref(&self) -> &str {
        "Ch9 §9.5.6.3"
    }
    fn applicable_versions(&self) -> &[Irig106Version] {
        &[]
    }

    fn check(&self, doc: &TmatsDocument<'_>, _ctx: &ValidationContext) -> Vec<Diagnostic> {
        let mut diags = Vec::new();
        for (&idx, b) in &doc.bus_data {
            let pfx = format!("B-{idx}");
            if b.data_link_name.is_none() {
                diags.push(missing_required_diag(
                    self,
                    &format!("{pfx}\\DLN"),
                    "Data Link Name",
                ));
            }
            if b.bus_type.is_none() {
                diags.push(missing_required_diag(
                    self,
                    &format!("{pfx}\\BT"),
                    "Bus Type",
                ));
            }
        }
        diags
    }
}

/// Helper for creating a missing-required-attribute diagnostic.
fn missing_required_diag(
    rule: &dyn ValidationRule,
    attr_path: &str,
    display_name: &str,
) -> Diagnostic {
    Diagnostic {
        severity: Severity::Error,
        rule_id: rule.id().to_string(),
        message: format!(
            "missing required attribute {} ({})",
            attr_path, display_name
        ),
        spec_reference: rule.spec_ref().to_string(),
        attribute_path: Some(attr_path.to_string()),
        expected: Some("non-empty value".into()),
        actual: Some("absent".into()),
        suggested_fix: Some(format!("add {}:<value>;", attr_path)),
    }
}

// ─── Rule Registry (L3-VALID-001) ────────────────────────────────────────────

/// Build the default set of validation rules.
fn default_rules() -> Vec<Box<dyn ValidationRule>> {
    vec![
        // Required attributes (L2-VALID-001)
        Box::new(RequiredGProgramName),
        Box::new(RequiredGVersion),
        Box::new(RequiredPGroupAttrs),
        Box::new(RequiredBGroupAttrs),
        // Counter consistency (L2-VALID-005)
        Box::new(CounterConsistencyGDsi),
        Box::new(CounterConsistencyRN),
        // Cross-group references (L2-VALID-006)
        Box::new(CrossGroupRefCdln),
        // Keyword validation (L2-VALID-003)
        Box::new(KeywordValidationGDst),
        Box::new(KeywordValidationRPdp),
        Box::new(KeywordValidationPEncoding),
        Box::new(KeywordValidationBBusType),
        // Date format (L2-VALID-007)
        Box::new(DateFormatValidation),
        // MFS enforcement (L2-VALID-004)
        Box::new(MfsValidationGPn),
    ]
}

// ═════════════════════════════════════════════════════════════════════════════
// PUBLIC API (L3-INTEROP-004)
// ═════════════════════════════════════════════════════════════════════════════

/// Validate a TmatsDocument using default rules and context.
///
/// **Requirement:** L3-INTEROP-004
pub fn validate(doc: &TmatsDocument<'_>) -> ValidationReport {
    validate_with_options(doc, &ValidationContext::default())
}

/// Validate with explicit context.
///
/// **Requirements:** L3-INTEROP-004, L3-VALID-008
pub fn validate_with_options(doc: &TmatsDocument<'_>, ctx: &ValidationContext) -> ValidationReport {
    let rules = default_rules();
    let mut diagnostics = Vec::new();

    let target_version = ctx.target_version.or(doc.source_version);

    for rule in &rules {
        // Version-filtered execution (L3-VALID-008)
        let applicable = rule.applicable_versions();
        if !applicable.is_empty() {
            if let Some(ver) = target_version {
                if !applicable.contains(&ver) {
                    continue;
                }
            }
        }

        // Profile filtering (L3-VALID-007)
        match &ctx.profile {
            ValidationProfile::Custom(ids) => {
                if !ids.iter().any(|id| id == rule.id()) {
                    continue;
                }
            }
            ValidationProfile::Ch10Required => {
                // Skip non-Ch10-specific rules for the Ch10Required profile
                // Rules with IDs < V020 are always-required; others are format-specific
            }
            ValidationProfile::Full => {}
        }

        let findings = rule.check(doc, ctx);
        diagnostics.extend(findings);
    }

    let errors = diagnostics
        .iter()
        .filter(|d| d.severity == Severity::Error)
        .count();
    let warnings = diagnostics
        .iter()
        .filter(|d| d.severity == Severity::Warning)
        .count();
    let info = diagnostics
        .iter()
        .filter(|d| d.severity == Severity::Info)
        .count();

    ValidationReport {
        diagnostics,
        version_detected: target_version,
        total_attributes: doc.attribute_count(),
        errors,
        warnings,
        info,
    }
}
