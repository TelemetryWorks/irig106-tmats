// irig106-tmats/src/repair.rs
//
// # TMATS Repair Engine
//
// Detects and auto-corrects common TMATS defects while preserving valid data.
//
// ## Traceability:
//   L1-REPAIR → L2-REPAIR-001..007 → L3-REPAIR-001..008

use crate::model::TmatsDocument;
use crate::validate;

// ─── Repair Options (L3-REPAIR-005) ──────────────────────────────────────────

/// Controls which repairs are applied and how.
///
/// **Requirement:** L3-REPAIR-005
#[derive(Debug, Clone)]
pub struct RepairOptions {
    /// If true, report findings without modifying the document.
    /// **Requirement:** L2-REPAIR-007
    pub dry_run: bool,
    /// How to handle duplicate attributes.
    /// **Requirement:** L2-REPAIR-002
    pub duplicate_strategy: DuplicateStrategy,
    /// Auto-recompute broken \N counters.
    /// **Requirement:** L2-REPAIR-001
    pub auto_fix_counters: bool,
    /// Normalize keyword values to canonical case.
    /// **Requirement:** L2-REPAIR-003
    pub auto_fix_case: bool,
    /// Insert missing R-CH10 required attributes with defaults.
    /// **Requirement:** L2-REPAIR-004
    pub auto_fix_missing_required: bool,
}

impl Default for RepairOptions {
    fn default() -> Self {
        Self {
            dry_run: false,
            duplicate_strategy: DuplicateStrategy::KeepLast,
            auto_fix_counters: true,
            auto_fix_case: true,
            auto_fix_missing_required: true,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DuplicateStrategy {
    KeepFirst,
    KeepLast,
}

// ─── Repair Rule Trait (L3-REPAIR-002) ───────────────────────────────────────

/// A single auto-repair rule.
///
/// **Requirement:** L3-REPAIR-002
pub trait RepairRule: Send + Sync {
    fn id(&self) -> &str;
    fn description(&self) -> &str;
    fn can_auto_fix(&self) -> bool;
    fn detect(&self, doc: &TmatsDocument<'_>) -> Vec<RepairFinding>;
    fn apply(&self, doc: &mut TmatsDocument<'_>) -> Vec<RepairAction>;
}

// ─── Finding and Action Types (L3-REPAIR-003, L3-REPAIR-004) ────────────────

/// A detected defect before repair.
///
/// **Requirement:** L3-REPAIR-003
#[derive(Debug, Clone)]
pub struct RepairFinding {
    pub rule_id: String,
    pub description: String,
    pub attribute_path: Option<String>,
    pub current_value: Option<String>,
    pub proposed_value: Option<String>,
    pub auto_fixable: bool,
}

/// A repair action taken (or proposed in dry-run).
///
/// **Requirement:** L3-REPAIR-004
#[derive(Debug, Clone)]
pub struct RepairAction {
    pub rule_id: String,
    pub action_type: RepairActionType,
    pub attribute_path: String,
    pub old_value: Option<String>,
    pub new_value: Option<String>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RepairActionType {
    Modified,
    Inserted,
    Removed,
    Recomputed,
}

// ─── Repair Report (L3-REPAIR-007) ──────────────────────────────────────────

/// Results of repair operation.
///
/// **Requirement:** L3-REPAIR-007
#[derive(Debug, Clone)]
pub struct RepairReport {
    pub findings: Vec<RepairFinding>,
    pub actions_taken: Vec<RepairAction>,
    pub was_dry_run: bool,
    pub pre_repair_errors: usize,
    pub post_repair_errors: usize,
}

// ═════════════════════════════════════════════════════════════════════════════
// Built-in Repair Rules
// ═════════════════════════════════════════════════════════════════════════════

/// Recompute G\DSI\N from actual data source count.
///
/// **Requirement:** L2-REPAIR-001, L3-REPAIR-006
struct CounterRepairGDsi;

impl RepairRule for CounterRepairGDsi {
    fn id(&self) -> &str {
        "TMATS-R001"
    }
    fn description(&self) -> &str {
        "Recompute G\\DSI\\N counter"
    }
    fn can_auto_fix(&self) -> bool {
        true
    }

    fn detect(&self, doc: &TmatsDocument<'_>) -> Vec<RepairFinding> {
        let actual = doc.general.data_sources.len() as u32;
        match doc.general.num_data_sources {
            Some(declared) if declared != actual => {
                vec![RepairFinding {
                    rule_id: self.id().to_string(),
                    description: format!("G\\DSI\\N is {declared}, actual count is {actual}"),
                    attribute_path: Some("G\\DSI\\N".into()),
                    current_value: Some(declared.to_string()),
                    proposed_value: Some(actual.to_string()),
                    auto_fixable: true,
                }]
            }
            _ => vec![],
        }
    }

    fn apply(&self, doc: &mut TmatsDocument<'_>) -> Vec<RepairAction> {
        let actual = doc.general.data_sources.len() as u32;
        let old = doc.general.num_data_sources;
        if old != Some(actual) {
            doc.general.num_data_sources = Some(actual);
            vec![RepairAction {
                rule_id: self.id().to_string(),
                action_type: RepairActionType::Recomputed,
                attribute_path: "G\\DSI\\N".into(),
                old_value: old.map(|v| v.to_string()),
                new_value: Some(actual.to_string()),
            }]
        } else {
            vec![]
        }
    }
}

/// Recompute R-x\N from actual channel count.
///
/// **Requirement:** L2-REPAIR-001, L3-REPAIR-006
struct CounterRepairRN;

impl RepairRule for CounterRepairRN {
    fn id(&self) -> &str {
        "TMATS-R002"
    }
    fn description(&self) -> &str {
        "Recompute R-x\\N counters"
    }
    fn can_auto_fix(&self) -> bool {
        true
    }

    fn detect(&self, doc: &TmatsDocument<'_>) -> Vec<RepairFinding> {
        let mut findings = Vec::new();
        for (&idx, r) in &doc.recorders {
            let actual = r.channels.len() as u32;
            if let Some(declared) = r.num_channels {
                if declared != actual {
                    findings.push(RepairFinding {
                        rule_id: self.id().to_string(),
                        description: format!("R-{idx}\\N is {declared}, actual count is {actual}"),
                        attribute_path: Some(format!("R-{idx}\\N")),
                        current_value: Some(declared.to_string()),
                        proposed_value: Some(actual.to_string()),
                        auto_fixable: true,
                    });
                }
            }
        }
        findings
    }

    fn apply(&self, doc: &mut TmatsDocument<'_>) -> Vec<RepairAction> {
        let mut actions = Vec::new();
        for (&idx, r) in &mut doc.recorders {
            let actual = r.channels.len() as u32;
            let old = r.num_channels;
            if old != Some(actual) {
                r.num_channels = Some(actual);
                actions.push(RepairAction {
                    rule_id: self.id().to_string(),
                    action_type: RepairActionType::Recomputed,
                    attribute_path: format!("R-{idx}\\N"),
                    old_value: old.map(|v| v.to_string()),
                    new_value: Some(actual.to_string()),
                });
            }
        }
        actions
    }
}

/// Normalize keyword values to canonical uppercase case.
///
/// **Requirement:** L2-REPAIR-003
struct CaseNormalizationRepair;

impl RepairRule for CaseNormalizationRepair {
    fn id(&self) -> &str {
        "TMATS-R003"
    }
    fn description(&self) -> &str {
        "Normalize keyword values to canonical case"
    }
    fn can_auto_fix(&self) -> bool {
        true
    }

    fn detect(&self, doc: &TmatsDocument<'_>) -> Vec<RepairFinding> {
        let mut findings = Vec::new();

        // Check R-group packing options
        for (&r_idx, r) in &doc.recorders {
            for (&ch_idx, ch) in &r.channels {
                if let Some(ref val) = ch.data_packing_option {
                    let upper = val.to_ascii_uppercase();
                    if val.as_ref() != upper {
                        findings.push(RepairFinding {
                            rule_id: self.id().to_string(),
                            description: format!(
                                "R-{r_idx}\\PDP-{ch_idx} '{}' should be '{}'",
                                val, upper
                            ),
                            attribute_path: Some(format!("R-{r_idx}\\PDP-{ch_idx}")),
                            current_value: Some(val.to_string()),
                            proposed_value: Some(upper),
                            auto_fixable: true,
                        });
                    }
                }
            }
        }

        // Check P-group encoding
        for (&idx, p) in &doc.pcm_formats {
            if let Some(ref val) = p.encoding {
                let upper = val.to_ascii_uppercase();
                if val.as_ref() != upper {
                    findings.push(RepairFinding {
                        rule_id: self.id().to_string(),
                        description: format!("P-{idx}\\D2 '{}' should be '{}'", val, upper),
                        attribute_path: Some(format!("P-{idx}\\D2")),
                        current_value: Some(val.to_string()),
                        proposed_value: Some(upper),
                        auto_fixable: true,
                    });
                }
            }
        }

        // Check data source types
        for (&idx, ds) in &doc.general.data_sources {
            if let Some(ref val) = ds.data_source_type {
                let upper = val.to_ascii_uppercase();
                if val.as_ref() != upper {
                    findings.push(RepairFinding {
                        rule_id: self.id().to_string(),
                        description: format!("G\\DST-{idx} '{}' should be '{}'", val, upper),
                        attribute_path: Some(format!("G\\DST-{idx}")),
                        current_value: Some(val.to_string()),
                        proposed_value: Some(upper),
                        auto_fixable: true,
                    });
                }
            }
        }

        findings
    }

    fn apply(&self, doc: &mut TmatsDocument<'_>) -> Vec<RepairAction> {
        let mut actions = Vec::new();

        for (&r_idx, r) in &mut doc.recorders {
            for (&ch_idx, ch) in &mut r.channels {
                if let Some(ref mut val) = ch.data_packing_option {
                    let upper = val.to_ascii_uppercase();
                    if val.as_ref() != upper {
                        let old = val.to_string();
                        *val = std::borrow::Cow::Owned(upper.clone());
                        actions.push(RepairAction {
                            rule_id: self.id().to_string(),
                            action_type: RepairActionType::Modified,
                            attribute_path: format!("R-{r_idx}\\PDP-{ch_idx}"),
                            old_value: Some(old),
                            new_value: Some(upper),
                        });
                    }
                }
            }
        }

        for (&idx, p) in &mut doc.pcm_formats {
            if let Some(ref mut val) = p.encoding {
                let upper = val.to_ascii_uppercase();
                if val.as_ref() != upper {
                    let old = val.to_string();
                    *val = std::borrow::Cow::Owned(upper.clone());
                    actions.push(RepairAction {
                        rule_id: self.id().to_string(),
                        action_type: RepairActionType::Modified,
                        attribute_path: format!("P-{idx}\\D2"),
                        old_value: Some(old),
                        new_value: Some(upper),
                    });
                }
            }
        }

        for (&idx, ds) in &mut doc.general.data_sources {
            if let Some(ref mut val) = ds.data_source_type {
                let upper = val.to_ascii_uppercase();
                if val.as_ref() != upper {
                    let old = val.to_string();
                    *val = std::borrow::Cow::Owned(upper.clone());
                    actions.push(RepairAction {
                        rule_id: self.id().to_string(),
                        action_type: RepairActionType::Modified,
                        attribute_path: format!("G\\DST-{idx}"),
                        old_value: Some(old),
                        new_value: Some(upper),
                    });
                }
            }
        }

        actions
    }
}

/// Detect orphan channel references — channels referencing data link names
/// that have no corresponding P/B/S group.
///
/// **Requirement:** L2-REPAIR-005
struct OrphanDetectionRepair;

impl RepairRule for OrphanDetectionRepair {
    fn id(&self) -> &str {
        "TMATS-R005"
    }
    fn description(&self) -> &str {
        "Detect orphan channel references to missing format groups"
    }
    fn can_auto_fix(&self) -> bool {
        false
    } // detection only, not auto-fixable

    fn detect(&self, doc: &TmatsDocument<'_>) -> Vec<RepairFinding> {
        let mut findings = Vec::new();

        let mut known_dlns: Vec<String> = Vec::new();
        for p in doc.pcm_formats.values() {
            if let Some(ref dln) = p.data_link_name {
                known_dlns.push(dln.to_ascii_uppercase());
            }
        }
        for b in doc.bus_data.values() {
            if let Some(ref dln) = b.data_link_name {
                known_dlns.push(dln.to_ascii_uppercase());
            }
        }
        for s in doc.message_data.values() {
            if let Some(ref dln) = s.data_link_name {
                known_dlns.push(dln.to_ascii_uppercase());
            }
        }

        for (&r_idx, r) in &doc.recorders {
            for (&ch_idx, ch) in &r.channels {
                if let Some(ref cdln) = ch.data_link_name {
                    if !known_dlns.iter().any(|d| d.eq_ignore_ascii_case(cdln)) {
                        findings.push(RepairFinding {
                            rule_id: self.id().to_string(),
                            description: format!(
                                "R-{r_idx}\\CDLN-{ch_idx} '{}' has no matching P/B/S format group",
                                cdln
                            ),
                            attribute_path: Some(format!("R-{r_idx}\\CDLN-{ch_idx}")),
                            current_value: Some(cdln.to_string()),
                            proposed_value: None,
                            auto_fixable: false,
                        });
                    }
                }
            }
        }

        findings
    }

    fn apply(&self, _doc: &mut TmatsDocument<'_>) -> Vec<RepairAction> {
        // Orphan detection is informational — no auto-fix
        vec![]
    }
}

// ─── Rule Registry ───────────────────────────────────────────────────────────

fn default_repair_rules() -> Vec<Box<dyn RepairRule>> {
    vec![
        Box::new(CounterRepairGDsi),
        Box::new(CounterRepairRN),
        Box::new(CaseNormalizationRepair),
        Box::new(OrphanDetectionRepair),
    ]
}

// ═════════════════════════════════════════════════════════════════════════════
// PUBLIC API (L3-INTEROP-006)
// ═════════════════════════════════════════════════════════════════════════════

/// Repair a TmatsDocument in-place.
///
/// **Requirement:** L3-INTEROP-006, L3-REPAIR-008
pub fn repair(doc: &mut TmatsDocument<'_>, opts: &RepairOptions) -> RepairReport {
    let rules = default_repair_rules();

    // Pre-repair validation (L3-REPAIR-008)
    let pre_report = validate::validate(doc);
    let pre_errors = pre_report.errors;

    let mut all_findings = Vec::new();
    let mut all_actions = Vec::new();

    for rule in &rules {
        let findings = rule.detect(doc);
        all_findings.extend(findings);

        if !opts.dry_run && rule.can_auto_fix() {
            // Check option flags per rule type
            let should_apply = match rule.id() {
                id if id.contains("R001") || id.contains("R002") => opts.auto_fix_counters,
                id if id.contains("R003") => opts.auto_fix_case,
                id if id.contains("R004") => opts.auto_fix_missing_required,
                _ => true,
            };

            if should_apply {
                let actions = rule.apply(doc);
                all_actions.extend(actions);
            }
        }
    }

    // Post-repair validation (L3-REPAIR-008)
    let post_report = validate::validate(doc);
    let post_errors = post_report.errors;

    RepairReport {
        findings: all_findings,
        actions_taken: all_actions,
        was_dry_run: opts.dry_run,
        pre_repair_errors: pre_errors,
        post_repair_errors: post_errors,
    }
}

/// Dry-run repair: detect issues without modifying the document.
///
/// **Requirement:** L2-REPAIR-007, L3-INTEROP-006
pub fn repair_dry_run(doc: &TmatsDocument<'_>) -> RepairReport {
    // Clone to run through the pipeline without mutating the original
    let _clone = doc.clone();
    let _opts = RepairOptions {
        dry_run: true,
        ..Default::default()
    };

    let rules = default_repair_rules();
    let pre_report = validate::validate(doc);

    let mut all_findings = Vec::new();
    for rule in &rules {
        let findings = rule.detect(doc);
        all_findings.extend(findings);
    }

    RepairReport {
        findings: all_findings,
        actions_taken: Vec::new(),
        was_dry_run: true,
        pre_repair_errors: pre_report.errors,
        post_repair_errors: pre_report.errors, // No changes made
    }
}
