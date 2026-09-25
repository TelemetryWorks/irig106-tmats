// irig106-tmats/src/parse.rs
//
// # TMATS ASCII Parser
//
// Two-phase parser: Phase 1 (tokenize) and Phase 2 (structure).
//
// ## Traceability:
//   L1-PARSE → L2-PARSE-001..009 → L3-PARSE-001..014

use std::borrow::Cow;

use indexmap::IndexMap;
use smallvec::SmallVec;

use crate::error::{ParseError, ParseErrorKind, TmatsErrors, TmatsError};
use crate::model::*;
use crate::types_bridge::{GroupPrefix, Irig106Version};

// ─── Parse Options (L3-PARSE-013) ────────────────────────────────────────────

/// Controls parser behavior.
///
/// **Requirement:** L3-PARSE-013
#[derive(Debug, Clone)]
pub struct ParseOptions {
    /// Strict mode aborts on first error; Lenient collects all errors.
    /// **Requirement:** L2-PARSE-009
    pub mode: ParseMode,
    /// Explicit version override (bypasses auto-detection).
    /// **Requirement:** L2-VERSION-005
    pub target_version: Option<Irig106Version>,
    /// Maximum number of attributes accepted (DoS protection).
    pub max_attributes: usize,
    /// Maximum value size in bytes (DoS protection).
    pub max_value_size: usize,
}

impl Default for ParseOptions {
    fn default() -> Self {
        Self {
            mode: ParseMode::Strict,
            target_version: None,
            max_attributes: 100_000,
            max_value_size: 1_048_576, // 1 MB
        }
    }
}

/// **Requirement:** L2-PARSE-009
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ParseMode {
    /// Abort on first malformed attribute.
    Strict,
    /// Collect all errors and recover as much data as possible.
    Lenient,
}

// ═════════════════════════════════════════════════════════════════════════════
// PHASE 1: TOKENIZATION (L3-PARSE-001, L3-PARSE-002)
// ═════════════════════════════════════════════════════════════════════════════

/// Phase 1 output: raw (code-name, value) pairs with source locations.
///
/// **Requirement:** L3-PARSE-001
#[derive(Debug)]
struct TokenizedAttribute<'a> {
    code_name_str: &'a str,
    value_str: &'a str,
    byte_offset: usize,
    line: u32,
}

/// Tokenize TMATS ASCII input into raw attribute pairs.
///
/// Implements a single-pass byte-level state machine per L3-PARSE-002.
///
/// **Requirements:** L2-PARSE-001, L2-PARSE-002, L3-PARSE-002, L3-PARSE-004
fn tokenize<'a>(
    input: &'a [u8],
    opts: &ParseOptions,
) -> Result<Vec<TokenizedAttribute<'a>>, Vec<ParseError>> {
    let mut tokens = Vec::new();
    let mut errors = Vec::new();
    let mut pos = 0;
    let mut line: u32 = 1;

    while pos < input.len() {
        // Skip whitespace and non-printable characters (L2-PARSE-002)
        // Per Ch9 §9.4.1: discard non-printable except CR/LF
        while pos < input.len() {
            let b = input[pos];
            if b == b'\n' {
                line += 1;
                pos += 1;
            } else if b == b'\r' || b <= 0x20 {
                pos += 1;
            } else {
                break;
            }
        }

        if pos >= input.len() {
            break;
        }

        // Check DoS limit
        if tokens.len() >= opts.max_attributes {
            errors.push(ParseError {
                kind: ParseErrorKind::MaxAttributesExceeded,
                byte_offset: pos,
                line,
                code_name: None,
                message: format!("exceeded maximum attribute count of {}", opts.max_attributes),
            });
            break;
        }

        let attr_start = pos;
        let attr_line = line;

        // Scan for colon delimiter (code-name terminator) — L2-PARSE-001
        let colon_pos = match find_byte(input, pos, b':') {
            Some(p) => p,
            None => {
                errors.push(ParseError {
                    kind: ParseErrorKind::InvalidDelimiter,
                    byte_offset: pos,
                    line,
                    code_name: None,
                    message: "expected ':' delimiter, found end of input".into(),
                });
                break;
            }
        };

        // Validate code-name is ASCII (L3-PARSE-004)
        let cn_bytes = &input[attr_start..colon_pos];
        if !cn_bytes.iter().all(|b| b.is_ascii_graphic()) {
            errors.push(ParseError {
                kind: ParseErrorKind::InvalidAscii,
                byte_offset: attr_start,
                line: attr_line,
                code_name: None,
                message: "code-name contains non-ASCII-graphic bytes".into(),
            });
            pos = colon_pos + 1;
            // Skip to next semicolon to recover
            if let Some(semi) = find_byte(input, pos, b';') {
                pos = semi + 1;
            } else {
                break;
            }
            continue;
        }

        let code_name_str = unsafe { std::str::from_utf8_unchecked(cn_bytes) };

        // Scan for semicolon delimiter (value terminator) — L2-PARSE-001
        let value_start = colon_pos + 1;
        let semi_pos = match find_byte(input, value_start, b';') {
            Some(p) => p,
            None => {
                errors.push(ParseError {
                    kind: ParseErrorKind::InvalidDelimiter,
                    byte_offset: value_start,
                    line: attr_line,
                    code_name: Some(code_name_str.to_string()),
                    message: "expected ';' delimiter after value".into(),
                });
                break;
            }
        };

        // Check value size limit
        let value_len = semi_pos - value_start;
        if value_len > opts.max_value_size {
            errors.push(ParseError {
                kind: ParseErrorKind::MaxValueSizeExceeded,
                byte_offset: value_start,
                line: attr_line,
                code_name: Some(code_name_str.to_string()),
                message: format!("value size {value_len} exceeds maximum {}", opts.max_value_size),
            });
            pos = semi_pos + 1;
            continue;
        }

        let value_bytes = &input[value_start..semi_pos];
        // Count newlines in value for line tracking
        for &b in value_bytes {
            if b == b'\n' {
                line += 1;
            }
        }

        let value_str = match std::str::from_utf8(value_bytes) {
            Ok(s) => s,
            Err(_) => {
                errors.push(ParseError {
                    kind: ParseErrorKind::InvalidAscii,
                    byte_offset: value_start,
                    line: attr_line,
                    code_name: Some(code_name_str.to_string()),
                    message: "value contains invalid UTF-8".into(),
                });
                pos = semi_pos + 1;
                continue;
            }
        };

        tokens.push(TokenizedAttribute {
            code_name_str,
            value_str,
            byte_offset: attr_start,
            line: attr_line,
        });

        pos = semi_pos + 1;
    }

    if errors.is_empty() {
        Ok(tokens)
    } else {
        Err(errors)
    }
}

fn find_byte(input: &[u8], start: usize, target: u8) -> Option<usize> {
    input[start..].iter().position(|&b| b == target).map(|p| start + p)
}

// ═════════════════════════════════════════════════════════════════════════════
// CODE-NAME PARSING (L3-PARSE-003, L3-PARSE-005)
// ═════════════════════════════════════════════════════════════════════════════

/// Parse a code-name string into its structural components.
///
/// Examples:
///   "G\\PN"       → G, None, [PN]
///   "R-1\\TK1-3"  → R, Some(1), [TK1(3)]
///   "R-1\\ASR-11" → R, Some(1), [ASR(11)]
///
/// **Requirements:** L2-PARSE-003, L3-PARSE-005, L2-PARSE-004
fn parse_code_name<'a>(raw: &'a str) -> Result<CodeName<'a>, ParseError> {
    let raw_upper = raw; // We preserve original case (L3-PARSE-008)

    // Split by backslash to get group-prefix and path segments
    let parts: Vec<&'a str> = raw.split('\\').collect();
    if parts.is_empty() {
        return Err(ParseError {
            kind: ParseErrorKind::MalformedCodeName,
            byte_offset: 0,
            line: 0,
            code_name: Some(raw.to_string()),
            message: "empty code-name".into(),
        });
    }

    // First part: GROUP[-occurrence]
    let (group, occurrence) = parse_group_prefix(parts[0])?;

    // Remaining parts: attribute path segments
    let mut path = SmallVec::new();
    for &part in &parts[1..] {
        path.push(parse_path_segment(part));
    }

    Ok(CodeName { group, occurrence, path })
}

/// Parse "R-1" → (R, Some(1)) or "G" → (G, None)
fn parse_group_prefix(s: &str) -> Result<(GroupPrefix, Option<u32>), ParseError> {
    if s.is_empty() {
        return Err(ParseError {
            kind: ParseErrorKind::MalformedCodeName,
            byte_offset: 0,
            line: 0,
            code_name: None,
            message: "empty group prefix".into(),
        });
    }

    let first_char = s.chars().next().unwrap();
    let group = GroupPrefix::from_char(first_char);

    let occurrence = if s.len() > 1 && s.as_bytes()[1] == b'-' {
        s[2..].parse::<u32>().ok()
    } else {
        None
    };

    Ok((group, occurrence))
}

/// Parse "TK1-3" → PathSegment { name: "TK1", index: Some(3) }
fn parse_path_segment(s: &str) -> PathSegment<'_> {
    // Find the last hyphen that separates name from index
    if let Some(dash_pos) = s.rfind('-') {
        let after_dash = &s[dash_pos + 1..];
        if let Ok(idx) = after_dash.parse::<u32>() {
            return PathSegment {
                name: Cow::Borrowed(&s[..dash_pos]),
                index: Some(idx),
            };
        }
    }
    PathSegment {
        name: Cow::Borrowed(s),
        index: None,
    }
}

// ═════════════════════════════════════════════════════════════════════════════
// PHASE 2: STRUCTURING (L3-PARSE-001, L3-PARSE-009)
// ═════════════════════════════════════════════════════════════════════════════

/// Phase 2: Route raw attributes into typed group builders.
///
/// **Requirements:** L3-PARSE-009, L2-PARSE-007
fn structure_document<'a>(
    tokens: Vec<TokenizedAttribute<'a>>,
    opts: &ParseOptions,
) -> Result<TmatsDocument<'a>, Vec<ParseError>> {
    let mut doc = TmatsDocument::empty();
    let mut errors = Vec::new();

    // L3-PARSE-012: Track seen code-names for duplicate detection
    let mut seen_code_names: std::collections::HashSet<String> = std::collections::HashSet::new();

    // L2-PARSE-007: Order independence — we process all tokens regardless of order
    for token in tokens {
        let code_name = match parse_code_name(token.code_name_str) {
            Ok(cn) => cn,
            Err(mut e) => {
                e.byte_offset = token.byte_offset;
                e.line = token.line;
                match opts.mode {
                    ParseMode::Strict => return Err(vec![e]),
                    ParseMode::Lenient => {
                        errors.push(e);
                        continue;
                    }
                }
            }
        };

        // L3-PARSE-012: Duplicate detection
        let normalized_cn = token.code_name_str.to_ascii_uppercase();
        if !seen_code_names.insert(normalized_cn) {
            let dup_err = ParseError {
                kind: ParseErrorKind::DuplicateAttribute,
                byte_offset: token.byte_offset,
                line: token.line,
                code_name: Some(token.code_name_str.to_string()),
                message: format!("duplicate attribute '{}' (keeping last)", token.code_name_str),
            };
            match opts.mode {
                ParseMode::Strict => return Err(vec![dup_err]),
                ParseMode::Lenient => {
                    // Record warning but continue (keep-last semantics)
                    doc.parse_diagnostics.push(crate::error::Diagnostic {
                        severity: crate::error::Severity::Warning,
                        rule_id: "TMATS-P004".to_string(),
                        message: dup_err.message.clone(),
                        spec_reference: "Ch9 §9.4.2".into(),
                        attribute_path: dup_err.code_name.clone(),
                        expected: None,
                        actual: None,
                        suggested_fix: Some("remove duplicate attribute".into()),
                    });
                }
            }
        }

        let raw_attr = RawAttribute {
            code_name: code_name.clone(),
            raw_code_name: Cow::Borrowed(token.code_name_str),
            value: Cow::Borrowed(token.value_str),
            byte_offset: token.byte_offset,
            line: token.line,
        };

        // Route by group prefix (L3-PARSE-009)
        match code_name.group {
            GroupPrefix::G => route_g_attribute(&mut doc.general, &raw_attr),
            GroupPrefix::T => route_indexed_attribute(
                &mut doc.transmission, &raw_attr, TGroup::default,
                route_t_attribute,
            ),
            GroupPrefix::R => route_indexed_attribute(
                &mut doc.recorders, &raw_attr, RGroup::default,
                route_r_attribute,
            ),
            GroupPrefix::M => route_indexed_attribute(
                &mut doc.multiplex, &raw_attr, MGroup::default,
                route_m_attribute,
            ),
            GroupPrefix::P => route_indexed_attribute(
                &mut doc.pcm_formats, &raw_attr, PGroup::default,
                route_p_attribute,
            ),
            GroupPrefix::D => route_indexed_attribute(
                &mut doc.pcm_measurements, &raw_attr, DGroup::default,
                route_d_attribute,
            ),
            GroupPrefix::B => route_indexed_attribute(
                &mut doc.bus_data, &raw_attr, BGroup::default,
                route_b_attribute,
            ),
            GroupPrefix::S => route_indexed_attribute(
                &mut doc.message_data, &raw_attr, SGroup::default,
                route_s_attribute,
            ),
            GroupPrefix::C => route_indexed_attribute(
                &mut doc.data_conversion, &raw_attr, CGroup::default,
                route_c_attribute,
            ),
            GroupPrefix::Unknown(_) => {
                doc.unknown.push(raw_attr);
            }
        }
    }

    // Version detection (L2-VERSION-001, L3-VERSION-008)
    if let Some(ref override_ver) = opts.target_version {
        doc.source_version = Some(*override_ver);
    } else if let Some(ref ver_str) = doc.general.irig106_version {
        doc.source_version = Irig106Version::from_g106_str(ver_str);
    }

    if errors.is_empty() {
        Ok(doc)
    } else {
        // In lenient mode, attach diagnostics and return the partial doc
        for e in &errors {
            doc.parse_diagnostics.push(crate::error::Diagnostic {
                severity: crate::error::Severity::Error,
                rule_id: format!("TMATS-P{:03}", e.kind as u8),
                message: e.message.clone(),
                spec_reference: "Ch9 §9.4.2".into(),
                attribute_path: e.code_name.clone(),
                expected: None,
                actual: None,
                suggested_fix: None,
            });
        }
        Ok(doc) // Lenient: return partial doc with diagnostics
    }
}

// ─── Group-specific attribute routing ────────────────────────────────────────

fn route_indexed_attribute<'a, T: Default>(
    map: &mut IndexMap<u32, T>,
    attr: &RawAttribute<'a>,
    default_fn: fn() -> T,
    router: fn(&mut T, &RawAttribute<'a>),
) {
    let idx = attr.code_name.occurrence.unwrap_or(1);
    let group = map.entry(idx).or_insert_with(default_fn);
    router(group, attr);
}

/// Route a G-group attribute.
/// **Requirement:** L2-MODEL-001, L2-PARSE-005
fn route_g_attribute<'a>(g: &mut GGroup<'a>, attr: &RawAttribute<'a>) {
    let path_key = first_path_name_upper(&attr.code_name);

    match path_key.as_deref() {
        Some("PN") => g.program_name = Some(attr.value.clone()),
        Some("106") => g.irig106_version = Some(attr.value.clone()),
        Some("OD") => g.origination_date = TmatsDate::from_tmats_str(&attr.value),
        Some("RN") => g.revision_number = Some(attr.value.clone()),
        Some("RD") => g.revision_date = TmatsDate::from_tmats_str(&attr.value),
        Some("UN") => g.update_number = Some(attr.value.clone()),
        Some("UD") => g.update_date = TmatsDate::from_tmats_str(&attr.value),
        Some("TN") => g.test_number = Some(attr.value.clone()),
        Some("DSI") => {
            if is_counter_attr(&attr.code_name) {
                g.num_data_sources = attr.value.trim().parse().ok();
            } else if let Some(idx) = first_path_index(&attr.code_name) {
                let ds = g.data_sources.entry(idx).or_default();
                ds.data_source_id = Some(attr.value.clone());
            }
        }
        Some("DST") => {
            if let Some(idx) = first_path_index(&attr.code_name) {
                let ds = g.data_sources.entry(idx).or_default();
                ds.data_source_type = Some(attr.value.clone());
            }
        }
        Some("DSC") => {
            if let Some(idx) = first_path_index(&attr.code_name) {
                let ds = g.data_sources.entry(idx).or_default();
                ds.classification = Some(attr.value.clone());
            }
        }
        Some("POC") => {
            if is_counter_attr(&attr.code_name) {
                g.num_points_of_contact = attr.value.trim().parse().ok();
            }
        }
        Some("COM") => g.comments.push(attr.value.clone()),
        _ => g.extra.push(attr.clone()),
    }
}

fn route_t_attribute<'a>(t: &mut TGroup<'a>, attr: &RawAttribute<'a>) {
    let path_key = first_path_name_upper(&attr.code_name);
    match path_key.as_deref() {
        Some("ID") => t.transmitter_id = Some(attr.value.clone()),
        Some("CF") => t.carrier_frequency_mhz = attr.value.trim().parse().ok(),
        Some("MT") => t.modulation_type = Some(attr.value.clone()),
        Some("PW") => t.power_watts = attr.value.trim().parse().ok(),
        Some("AT") => t.antenna_type = Some(attr.value.clone()),
        Some("AP") => t.antenna_polarization = Some(attr.value.clone()),
        Some("COM") => t.comments.push(attr.value.clone()),
        _ => t.extra.push(attr.clone()),
    }
}

fn route_r_attribute<'a>(r: &mut RGroup<'a>, attr: &RawAttribute<'a>) {
    let path_key = first_path_name_upper(&attr.code_name);
    match path_key.as_deref() {
        Some("ID") => r.recorder_id = Some(attr.value.clone()),
        Some("RI1") => r.recorder_description = Some(attr.value.clone()),
        Some("RMT") => r.media_type = Some(attr.value.clone()),
        Some("N") => r.num_channels = attr.value.trim().parse().ok(),
        Some("TK1") => {
            if let Some(idx) = first_path_index(&attr.code_name) {
                let ch = r.channels.entry(idx).or_default();
                ch.channel_id = attr.value.trim().parse().ok();
            }
        }
        Some("CDT") => {
            if let Some(idx) = first_path_index(&attr.code_name) {
                let ch = r.channels.entry(idx).or_default();
                ch.data_type = Some(attr.value.clone());
            }
        }
        Some("DSI") => {
            if let Some(idx) = first_path_index(&attr.code_name) {
                let ch = r.channels.entry(idx).or_default();
                ch.data_source_id = Some(attr.value.clone());
            }
        }
        Some("CDLN") => {
            if let Some(idx) = first_path_index(&attr.code_name) {
                let ch = r.channels.entry(idx).or_default();
                ch.data_link_name = Some(attr.value.clone());
            }
        }
        Some("PDP") => {
            if let Some(idx) = first_path_index(&attr.code_name) {
                let ch = r.channels.entry(idx).or_default();
                ch.data_packing_option = Some(attr.value.clone());
            }
        }
        Some("CHE") => {
            if let Some(idx) = first_path_index(&attr.code_name) {
                let ch = r.channels.entry(idx).or_default();
                ch.channel_enabled = parse_bool(&attr.value);
            }
        }
        Some("IDX") => {
            // R-x\IDX\E
            if attr.code_name.path.len() >= 2 {
                r.index_enabled = parse_bool(&attr.value);
            }
        }
        Some("EV") => {
            if attr.code_name.path.len() >= 2 {
                r.events_enabled = parse_bool(&attr.value);
            }
        }
        Some("COM") => r.comments.push(attr.value.clone()),
        _ => r.extra.push(attr.clone()),
    }
}

fn route_m_attribute<'a>(m: &mut MGroup<'a>, attr: &RawAttribute<'a>) {
    let path_key = first_path_name_upper(&attr.code_name);
    match path_key.as_deref() {
        Some("BSG") => m.baseband_signal_type = Some(attr.value.clone()),
        Some("MS") => m.modulation_sense = Some(attr.value.clone()),
        Some("COM") => m.comments.push(attr.value.clone()),
        _ => m.extra.push(attr.clone()),
    }
}

fn route_p_attribute<'a>(p: &mut PGroup<'a>, attr: &RawAttribute<'a>) {
    let path_key = first_path_name_upper(&attr.code_name);
    match path_key.as_deref() {
        Some("DLN") => p.data_link_name = Some(attr.value.clone()),
        Some("D1") => p.bit_rate = attr.value.trim().parse().ok(),
        Some("D2") => p.encoding = Some(attr.value.clone()),
        Some("D3") => p.polarity = Some(attr.value.clone()),
        Some("D4") => p.auto_polarity_correction = parse_bool(&attr.value),
        Some("MF") => {
            if is_counter_attr(&attr.code_name) {
                p.num_minor_frames = attr.value.trim().parse().ok();
            }
        }
        Some("F1") => p.num_words_per_frame = attr.value.trim().parse().ok(),
        Some("F2") => p.num_bits_per_word = attr.value.trim().parse().ok(),
        Some("F3") => {
            // F3 is sync pattern, F3\L is sync pattern length
            if attr.code_name.path.len() >= 2 {
                p.sync_pattern_length = attr.value.trim().parse().ok();
            } else {
                p.sync_pattern = Some(attr.value.clone());
            }
        }
        Some("COM") => p.comments.push(attr.value.clone()),
        _ => p.extra.push(attr.clone()),
    }
}

fn route_d_attribute<'a>(d: &mut DGroup<'a>, attr: &RawAttribute<'a>) {
    let path_key = first_path_name_upper(&attr.code_name);
    match path_key.as_deref() {
        Some("MLN") => d.measurement_list_name = Some(attr.value.clone()),
        Some("DLN") => d.data_link_name = Some(attr.value.clone()),
        Some("MN") => {
            if is_counter_attr(&attr.code_name) {
                d.num_measurements = attr.value.trim().parse().ok();
            }
        }
        Some("COM") => d.comments.push(attr.value.clone()),
        _ => d.extra.push(attr.clone()),
    }
}

fn route_b_attribute<'a>(b: &mut BGroup<'a>, attr: &RawAttribute<'a>) {
    let path_key = first_path_name_upper(&attr.code_name);
    match path_key.as_deref() {
        Some("DLN") => b.data_link_name = Some(attr.value.clone()),
        Some("BT") => b.bus_type = Some(attr.value.clone()),
        Some("NBS") => {
            if is_counter_attr(&attr.code_name) {
                b.num_buses = attr.value.trim().parse().ok();
            }
        }
        Some("COM") => b.comments.push(attr.value.clone()),
        _ => b.extra.push(attr.clone()),
    }
}

fn route_s_attribute<'a>(s: &mut SGroup<'a>, attr: &RawAttribute<'a>) {
    let path_key = first_path_name_upper(&attr.code_name);
    match path_key.as_deref() {
        Some("DLN") => s.data_link_name = Some(attr.value.clone()),
        Some("COM") => s.comments.push(attr.value.clone()),
        _ => s.extra.push(attr.clone()),
    }
}

fn route_c_attribute<'a>(c: &mut CGroup<'a>, attr: &RawAttribute<'a>) {
    let path_key = first_path_name_upper(&attr.code_name);
    match path_key.as_deref() {
        Some("DCN") => c.measurement_name = Some(attr.value.clone()),
        Some("DCT") => c.conversion_type = Some(attr.value.clone()),
        Some("EU") => c.eu_units = Some(attr.value.clone()),
        Some("COM") => c.comments.push(attr.value.clone()),
        _ => c.extra.push(attr.clone()),
    }
}

// ─── Helper functions ────────────────────────────────────────────────────────

fn first_path_name_upper(cn: &CodeName<'_>) -> Option<String> {
    cn.path.first().map(|seg| seg.name.to_ascii_uppercase())
}

fn first_path_index(cn: &CodeName<'_>) -> Option<u32> {
    cn.path.first().and_then(|seg| seg.index)
}

fn is_counter_attr(cn: &CodeName<'_>) -> bool {
    cn.path.last().map_or(false, |seg| seg.name.eq_ignore_ascii_case("N"))
}

fn parse_bool(s: &str) -> Option<bool> {
    match s.trim().to_ascii_uppercase().as_str() {
        "T" | "TRUE" | "YES" | "1" => Some(true),
        "F" | "FALSE" | "NO" | "0" => Some(false),
        _ => None,
    }
}

// ═════════════════════════════════════════════════════════════════════════════
// PUBLIC API (L3-INTEROP-001)
// ═════════════════════════════════════════════════════════════════════════════

/// Parse TMATS ASCII data from a byte slice (zero-copy borrowed mode).
///
/// Returns a `TmatsDocument` with borrowed references into the input buffer.
///
/// **Requirements:** L2-INTEROP-002, L3-INTEROP-001, L2-PERF-001
pub fn parse(input: &[u8]) -> Result<TmatsDocument<'_>, TmatsErrors> {
    parse_with_options(input, &ParseOptions::default())
}

/// Parse TMATS ASCII data with explicit options.
///
/// **Requirements:** L3-INTEROP-001
pub fn parse_with_options<'a>(
    input: &'a [u8],
    opts: &ParseOptions,
) -> Result<TmatsDocument<'a>, TmatsErrors> {
    // Phase 1: Tokenize
    let tokens = match tokenize(input, opts) {
        Ok(t) => t,
        Err(parse_errors) => {
            if opts.mode == ParseMode::Strict {
                let mut errs = TmatsErrors::new();
                for e in parse_errors {
                    errs.push(TmatsError::Parse(e));
                }
                return Err(errs);
            }
            // In lenient mode with total tokenization failure, return empty doc
            return Ok(TmatsDocument::empty());
        }
    };

    // Phase 2: Structure
    match structure_document(tokens, opts) {
        Ok(doc) => Ok(doc),
        Err(parse_errors) => {
            let mut errs = TmatsErrors::new();
            for e in parse_errors {
                errs.push(TmatsError::Parse(e));
            }
            Err(errs)
        }
    }
}

/// Parse and produce an owned (`'static`) document.
///
/// **Requirements:** L2-PERF-002, L3-INTEROP-001
pub fn parse_owned(input: &[u8]) -> Result<OwnedTmatsDocument, TmatsErrors> {
    parse(input).map(|doc| doc.into_owned())
}
