// irig106-tmats/src/serial.rs
//
// # TMATS ASCII Serializer
//
// Walks TmatsDocument and emits spec-compliant code-name:value; format.
//
// ## Traceability:
//   L1-SERIAL → L2-SERIAL-001..005 → L3-SERIAL-001..007

use std::io::{self, Write};

use crate::error::SerializeError;
use crate::model::*;

// ─── Serialization Options (L3-SERIAL-006) ───────────────────────────────────

/// Controls serializer output format.
///
/// **Requirement:** L3-SERIAL-006
#[derive(Debug, Clone)]
pub struct SerializeOptions {
    /// Output format style.
    pub format: OutputFormat,
    /// Line ending style for pretty output.
    pub line_ending: LineEnding,
    /// Whether to emit comment attributes.
    pub emit_comments: bool,
    /// Whether to normalize keyword values to canonical case.
    /// **Requirement:** L3-SERIAL-005
    pub canonical_keyword_case: bool,
}

impl Default for SerializeOptions {
    fn default() -> Self {
        Self {
            format: OutputFormat::Pretty,
            line_ending: LineEnding::Lf,
            emit_comments: true,
            canonical_keyword_case: false,
        }
    }
}

/// **Requirement:** L2-SERIAL-004
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum OutputFormat {
    /// No whitespace between attributes.
    Compact,
    /// Line breaks between attributes for readability.
    Pretty,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LineEnding {
    Lf,
    CrLf,
}

impl LineEnding {
    fn as_str(&self) -> &str {
        match self {
            Self::Lf => "\n",
            Self::CrLf => "\r\n",
        }
    }
}

// ─── Emitter Trait (L3-SERIAL-002) ───────────────────────────────────────────

/// Low-level attribute emitter.
///
/// **Requirement:** L3-SERIAL-002
struct TmatsEmitter<'w, W: Write + ?Sized> {
    writer: &'w mut W,
    opts: SerializeOptions,
}

impl<'w, W: Write + ?Sized> TmatsEmitter<'w, W> {
    fn new(writer: &'w mut W, opts: SerializeOptions) -> Self {
        Self { writer, opts }
    }

    /// Emit a single attribute in code-name:value; format.
    /// **Requirement:** L2-SERIAL-001
    fn emit(&mut self, code_name: &str, value: &str) -> io::Result<()> {
        write!(self.writer, "{}:{};", code_name, value)?;
        if self.opts.format == OutputFormat::Pretty {
            write!(self.writer, "{}", self.opts.line_ending.as_str())?;
        }
        Ok(())
    }

    fn emit_comment(&mut self, prefix: &str, text: &str) -> io::Result<()> {
        if self.opts.emit_comments {
            self.emit(&format!("{}\\COM", prefix), text)?;
        }
        Ok(())
    }
}

// ═════════════════════════════════════════════════════════════════════════════
// SERIALIZATION VISITOR (L3-SERIAL-001)
// ═════════════════════════════════════════════════════════════════════════════

/// Serialize a TmatsDocument to a writer.
///
/// Visits groups in canonical order: G, T, R, M, P, D, B, S, C, unknown.
///
/// **Requirements:** L2-INTEROP-003, L3-INTEROP-002, L3-SERIAL-001
pub fn serialize(doc: &TmatsDocument<'_>, writer: &mut dyn Write) -> Result<(), SerializeError> {
    serialize_with_options(doc, writer, &SerializeOptions::default())
}

/// Serialize with explicit options.
///
/// **Requirements:** L3-INTEROP-002
pub fn serialize_with_options(
    doc: &TmatsDocument<'_>,
    writer: &mut dyn Write,
    opts: &SerializeOptions,
) -> Result<(), SerializeError> {
    let mut emitter = TmatsEmitter::new(writer, opts.clone());

    serialize_g_group(&mut emitter, &doc.general).map_err(|e| SerializeError {
        message: e.to_string(),
        attribute_path: None,
    })?;

    for (&idx, t) in &doc.transmission {
        serialize_t_group(&mut emitter, idx, t).map_err(|e| SerializeError {
            message: e.to_string(),
            attribute_path: None,
        })?;
    }

    for (&idx, r) in &doc.recorders {
        serialize_r_group(&mut emitter, idx, r).map_err(|e| SerializeError {
            message: e.to_string(),
            attribute_path: None,
        })?;
    }

    for (&idx, m) in &doc.multiplex {
        serialize_m_group(&mut emitter, idx, m).map_err(|e| SerializeError {
            message: e.to_string(),
            attribute_path: None,
        })?;
    }

    for (&idx, p) in &doc.pcm_formats {
        serialize_p_group(&mut emitter, idx, p).map_err(|e| SerializeError {
            message: e.to_string(),
            attribute_path: None,
        })?;
    }

    for (&idx, d) in &doc.pcm_measurements {
        serialize_d_group(&mut emitter, idx, d).map_err(|e| SerializeError {
            message: e.to_string(),
            attribute_path: None,
        })?;
    }

    for (&idx, b) in &doc.bus_data {
        serialize_b_group(&mut emitter, idx, b).map_err(|e| SerializeError {
            message: e.to_string(),
            attribute_path: None,
        })?;
    }

    for (&idx, s) in &doc.message_data {
        serialize_s_group(&mut emitter, idx, s).map_err(|e| SerializeError {
            message: e.to_string(),
            attribute_path: None,
        })?;
    }

    for (&idx, c) in &doc.data_conversion {
        serialize_c_group(&mut emitter, idx, c).map_err(|e| SerializeError {
            message: e.to_string(),
            attribute_path: None,
        })?;
    }

    // Emit unknown/extra attributes (L2-MODEL-012 round-trip)
    for attr in &doc.unknown {
        emitter
            .emit(&attr.raw_code_name, &attr.value)
            .map_err(|e| SerializeError {
                message: e.to_string(),
                attribute_path: None,
            })?;
    }

    Ok(())
}

/// Serialize to an in-memory buffer.
///
/// **Requirement:** L3-SERIAL-007, L3-INTEROP-002
pub fn serialize_to_vec(doc: &TmatsDocument<'_>) -> Result<Vec<u8>, SerializeError> {
    let estimated_size = doc.attribute_count() * 40;
    let mut buf = Vec::with_capacity(estimated_size);
    serialize(doc, &mut buf)?;
    Ok(buf)
}

// ─── Group-specific serializers ──────────────────────────────────────────────

fn serialize_g_group<W: Write + ?Sized>(
    e: &mut TmatsEmitter<'_, W>,
    g: &GGroup<'_>,
) -> io::Result<()> {
    if let Some(ref v) = g.program_name {
        e.emit("G\\PN", v)?;
    }
    if let Some(ref v) = g.irig106_version {
        e.emit("G\\106", v)?;
    }
    if let Some(ref v) = g.origination_date {
        e.emit("G\\OD", &v.to_string())?;
    }
    if let Some(ref v) = g.revision_number {
        e.emit("G\\RN", v)?;
    }
    if let Some(ref v) = g.revision_date {
        e.emit("G\\RD", &v.to_string())?;
    }
    if let Some(ref v) = g.update_number {
        e.emit("G\\UN", v)?;
    }
    if let Some(ref v) = g.update_date {
        e.emit("G\\UD", &v.to_string())?;
    }
    if let Some(ref v) = g.test_number {
        e.emit("G\\TN", v)?;
    }

    // Counter attribute auto-computation (L2-SERIAL-002, L3-SERIAL-003)
    if !g.data_sources.is_empty() {
        e.emit("G\\DSI\\N", &g.data_sources.len().to_string())?;
        for (&idx, ds) in &g.data_sources {
            if let Some(ref v) = ds.data_source_id {
                e.emit(&format!("G\\DSI-{idx}"), v)?;
            }
            if let Some(ref v) = ds.data_source_type {
                e.emit(&format!("G\\DST-{idx}"), v)?;
            }
            if let Some(ref v) = ds.classification {
                e.emit(&format!("G\\DSC-{idx}"), v)?;
            }
            for attr in &ds.extra {
                e.emit(&attr.raw_code_name, &attr.value)?;
            }
        }
    }

    if !g.points_of_contact.is_empty() {
        e.emit("G\\POC\\N", &g.points_of_contact.len().to_string())?;
    }

    for comment in &g.comments {
        e.emit_comment("G", comment)?;
    }
    for attr in &g.extra {
        e.emit(&attr.raw_code_name, &attr.value)?;
    }
    Ok(())
}

fn serialize_t_group<W: Write + ?Sized>(
    e: &mut TmatsEmitter<'_, W>,
    idx: u32,
    t: &TGroup<'_>,
) -> io::Result<()> {
    let pfx = format!("T-{idx}");
    if let Some(ref v) = t.transmitter_id {
        e.emit(&format!("{pfx}\\ID"), v)?;
    }
    if let Some(v) = t.carrier_frequency_mhz {
        e.emit(&format!("{pfx}\\CF"), &v.to_string())?;
    }
    if let Some(ref v) = t.modulation_type {
        e.emit(&format!("{pfx}\\MT"), v)?;
    }
    if let Some(v) = t.power_watts {
        e.emit(&format!("{pfx}\\PW"), &v.to_string())?;
    }
    if let Some(ref v) = t.antenna_type {
        e.emit(&format!("{pfx}\\AT"), v)?;
    }
    if let Some(ref v) = t.antenna_polarization {
        e.emit(&format!("{pfx}\\AP"), v)?;
    }
    for c in &t.comments {
        e.emit_comment(&pfx, c)?;
    }
    for attr in &t.extra {
        e.emit(&attr.raw_code_name, &attr.value)?;
    }
    Ok(())
}

fn serialize_r_group<W: Write + ?Sized>(
    e: &mut TmatsEmitter<'_, W>,
    idx: u32,
    r: &RGroup<'_>,
) -> io::Result<()> {
    let pfx = format!("R-{idx}");
    if let Some(ref v) = r.recorder_id {
        e.emit(&format!("{pfx}\\ID"), v)?;
    }
    if let Some(ref v) = r.recorder_description {
        e.emit(&format!("{pfx}\\RI1"), v)?;
    }
    if let Some(ref v) = r.media_type {
        e.emit(&format!("{pfx}\\RMT"), v)?;
    }

    // Auto-compute N counter (L3-SERIAL-003)
    if !r.channels.is_empty() {
        e.emit(&format!("{pfx}\\N"), &r.channels.len().to_string())?;
    }

    for (&ch_idx, ch) in &r.channels {
        if let Some(v) = ch.channel_id {
            e.emit(&format!("{pfx}\\TK1-{ch_idx}"), &v.to_string())?;
        }
        if let Some(ref v) = ch.data_type {
            e.emit(&format!("{pfx}\\CDT-{ch_idx}"), v)?;
        }
        if let Some(ref v) = ch.data_source_id {
            e.emit(&format!("{pfx}\\DSI-{ch_idx}"), v)?;
        }
        if let Some(ref v) = ch.data_link_name {
            e.emit(&format!("{pfx}\\CDLN-{ch_idx}"), v)?;
        }
        if let Some(ref v) = ch.data_packing_option {
            e.emit(&format!("{pfx}\\PDP-{ch_idx}"), v)?;
        }
        if let Some(v) = ch.channel_enabled {
            e.emit(&format!("{pfx}\\CHE-{ch_idx}"), if v { "T" } else { "F" })?;
        }
        for attr in &ch.extra {
            e.emit(&attr.raw_code_name, &attr.value)?;
        }
    }

    if let Some(v) = r.index_enabled {
        e.emit(&format!("{pfx}\\IDX\\E"), if v { "T" } else { "F" })?;
    }
    if let Some(v) = r.events_enabled {
        e.emit(&format!("{pfx}\\EV\\E"), if v { "T" } else { "F" })?;
    }
    for c in &r.comments {
        e.emit_comment(&pfx, c)?;
    }
    for attr in &r.extra {
        e.emit(&attr.raw_code_name, &attr.value)?;
    }
    Ok(())
}

fn serialize_m_group<W: Write + ?Sized>(
    e: &mut TmatsEmitter<'_, W>,
    idx: u32,
    m: &MGroup<'_>,
) -> io::Result<()> {
    let pfx = format!("M-{idx}");
    if let Some(ref v) = m.baseband_signal_type {
        e.emit(&format!("{pfx}\\BSG"), v)?;
    }
    if let Some(ref v) = m.modulation_sense {
        e.emit(&format!("{pfx}\\MS"), v)?;
    }
    for c in &m.comments {
        e.emit_comment(&pfx, c)?;
    }
    for attr in &m.extra {
        e.emit(&attr.raw_code_name, &attr.value)?;
    }
    Ok(())
}

fn serialize_p_group<W: Write + ?Sized>(
    e: &mut TmatsEmitter<'_, W>,
    idx: u32,
    p: &PGroup<'_>,
) -> io::Result<()> {
    let pfx = format!("P-{idx}");
    if let Some(ref v) = p.data_link_name {
        e.emit(&format!("{pfx}\\DLN"), v)?;
    }
    if let Some(v) = p.bit_rate {
        e.emit(&format!("{pfx}\\D1"), &v.to_string())?;
    }
    if let Some(ref v) = p.encoding {
        e.emit(&format!("{pfx}\\D2"), v)?;
    }
    if let Some(ref v) = p.polarity {
        e.emit(&format!("{pfx}\\D3"), v)?;
    }
    if let Some(v) = p.auto_polarity_correction {
        e.emit(&format!("{pfx}\\D4"), if v { "T" } else { "F" })?;
    }
    if let Some(v) = p.num_words_per_frame {
        e.emit(&format!("{pfx}\\F1"), &v.to_string())?;
    }
    if let Some(v) = p.num_bits_per_word {
        e.emit(&format!("{pfx}\\F2"), &v.to_string())?;
    }
    if let Some(ref v) = p.sync_pattern {
        e.emit(&format!("{pfx}\\F3"), v)?;
    }
    if let Some(v) = p.sync_pattern_length {
        e.emit(&format!("{pfx}\\F3\\L"), &v.to_string())?;
    }
    for c in &p.comments {
        e.emit_comment(&pfx, c)?;
    }
    for attr in &p.extra {
        e.emit(&attr.raw_code_name, &attr.value)?;
    }
    Ok(())
}

fn serialize_d_group<W: Write + ?Sized>(
    e: &mut TmatsEmitter<'_, W>,
    idx: u32,
    d: &DGroup<'_>,
) -> io::Result<()> {
    let pfx = format!("D-{idx}");
    if let Some(ref v) = d.measurement_list_name {
        e.emit(&format!("{pfx}\\MLN"), v)?;
    }
    if let Some(ref v) = d.data_link_name {
        e.emit(&format!("{pfx}\\DLN"), v)?;
    }
    if !d.measurements.is_empty() {
        e.emit(&format!("{pfx}\\MN\\N"), &d.measurements.len().to_string())?;
    }
    for c in &d.comments {
        e.emit_comment(&pfx, c)?;
    }
    for attr in &d.extra {
        e.emit(&attr.raw_code_name, &attr.value)?;
    }
    Ok(())
}

fn serialize_b_group<W: Write + ?Sized>(
    e: &mut TmatsEmitter<'_, W>,
    idx: u32,
    b: &BGroup<'_>,
) -> io::Result<()> {
    let pfx = format!("B-{idx}");
    if let Some(ref v) = b.data_link_name {
        e.emit(&format!("{pfx}\\DLN"), v)?;
    }
    if let Some(ref v) = b.bus_type {
        e.emit(&format!("{pfx}\\BT"), v)?;
    }
    for c in &b.comments {
        e.emit_comment(&pfx, c)?;
    }
    for attr in &b.extra {
        e.emit(&attr.raw_code_name, &attr.value)?;
    }
    Ok(())
}

fn serialize_s_group<W: Write + ?Sized>(
    e: &mut TmatsEmitter<'_, W>,
    idx: u32,
    s: &SGroup<'_>,
) -> io::Result<()> {
    let pfx = format!("S-{idx}");
    if let Some(ref v) = s.data_link_name {
        e.emit(&format!("{pfx}\\DLN"), v)?;
    }
    for c in &s.comments {
        e.emit_comment(&pfx, c)?;
    }
    for attr in &s.extra {
        e.emit(&attr.raw_code_name, &attr.value)?;
    }
    Ok(())
}

fn serialize_c_group<W: Write + ?Sized>(
    e: &mut TmatsEmitter<'_, W>,
    idx: u32,
    c: &CGroup<'_>,
) -> io::Result<()> {
    let pfx = format!("C-{idx}");
    if let Some(ref v) = c.measurement_name {
        e.emit(&format!("{pfx}\\DCN"), v)?;
    }
    if let Some(ref v) = c.conversion_type {
        e.emit(&format!("{pfx}\\DCT"), v)?;
    }
    if let Some(ref v) = c.eu_units {
        e.emit(&format!("{pfx}\\EU"), v)?;
    }
    for comment in &c.comments {
        e.emit_comment(&pfx, comment)?;
    }
    for attr in &c.extra {
        e.emit(&attr.raw_code_name, &attr.value)?;
    }
    Ok(())
}
