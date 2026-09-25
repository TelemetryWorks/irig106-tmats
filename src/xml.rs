// irig106-tmats/src/xml.rs
//
// # TMATS XML Format Support
//
// Parse and serialize TMATS in the XML schema format defined in Ch9 §9.4.3.
// Handles the documented divergences between XML and code-name formats.
//
// ## Divergences (Ch9 §9.4.3):
//   a. One C-group per data link (not single C-group)
//   b. No counter (\N) attributes
//   c. Keyword values expanded for readability
//   d. Date and time in XML standard format (YYYY-MM-DD)
//
// ## Traceability:
//   L1-XML → L2-XML-001..005 → L3-XML-001..008

use quick_xml::events::{Event, BytesStart, BytesEnd, BytesText};
use quick_xml::Reader;
use quick_xml::Writer;

use std::borrow::Cow;
use std::io::{BufRead, Write};

use crate::error::{TmatsError, XmlError};
use crate::model::*;
use crate::types_bridge::{GroupPrefix, Irig106Version};

// ─── Keyword Expansion Registry (L3-XML-005) ────────────────────────────────

/// Maps abbreviated keyword values to their expanded XML forms and vice versa.
///
/// **Requirement:** L3-XML-004, L3-XML-005
static KEYWORD_EXPANSIONS: &[(&str, &str)] = &[
    // Recorder media types
    ("LO", "Longitudinal"),
    ("RO", "Rotary"),
    // Data source types
    ("REC", "Recorder"),
    ("TEL", "Telemetry"),
    ("MUL", "Multiplex"),
    ("PRE", "Pre-Detect"),
    // Classification
    ("U", "Unclassified"),
    ("C", "Confidential"),
    ("S", "Secret"),
    // PCM encoding
    ("NRZ-L", "NRZ-L"),
    ("NRZ-M", "NRZ-M"),
    ("NRZ-S", "NRZ-S"),
    ("RNRZ-L", "RNRZ-L"),
    ("BIO-L", "BiPhase-Level"),
    ("BIO-M", "BiPhase-Mark"),
    ("BIO-S", "BiPhase-Space"),
    // Polarity
    ("N", "Normal"),
    ("R", "Reversed"),
    // Packing
    ("UN", "Unpacked"),
    ("PFS", "Packed"),
    ("TM", "Throughput"),
    // Booleans
    ("T", "True"),
    ("F", "False"),
    // Bus types
    ("1553", "MIL-STD-1553"),
    ("A429", "ARINC-429"),
    // Modulation
    ("FM", "FM"),
    ("PM", "PM"),
    ("AM", "AM"),
    // Transfer order
    ("M", "MSB"),
    ("L", "LSB"),
    ("D", "Default"),
    // Conversion types
    ("PAIR", "PairSets"),
    ("COEF", "Coefficients"),
    ("TABL", "Table"),
    ("POLY", "Polynomial"),
    ("FORM", "Formula"),
];

/// Expand an abbreviated keyword to its XML form.
///
/// **Requirement:** L3-XML-004
pub fn expand_keyword(abbrev: &str) -> &str {
    let upper = abbrev.trim();
    for &(short, long) in KEYWORD_EXPANSIONS {
        if short.eq_ignore_ascii_case(upper) {
            return long;
        }
    }
    // Return as-is if no expansion found
    abbrev
}

/// Collapse an expanded XML keyword to its abbreviated form.
pub fn collapse_keyword(expanded: &str) -> &str {
    let trimmed = expanded.trim();
    for &(short, long) in KEYWORD_EXPANSIONS {
        if long.eq_ignore_ascii_case(trimmed) {
            return short;
        }
    }
    expanded
}

// ─── Date Conversion (L3-XML-006) ───────────────────────────────────────────

/// Convert TMATS date (MM-DD-YYYY) to XML date (YYYY-MM-DD).
///
/// **Requirement:** L3-XML-006
pub fn tmats_date_to_xml(date: &TmatsDate) -> String {
    format!("{:04}-{:02}-{:02}", date.year, date.month, date.day)
}

/// Convert XML date (YYYY-MM-DD) to TmatsDate.
pub fn xml_date_to_tmats(xml_date: &str) -> Option<TmatsDate> {
    TmatsDate::from_xml_str(xml_date)
}

// ═════════════════════════════════════════════════════════════════════════════
// XML SERIALIZER (L2-XML-002, L3-XML-003, L3-XML-004, L3-XML-007)
// ═════════════════════════════════════════════════════════════════════════════

/// Serialize a TmatsDocument to TMATS XML format.
///
/// **Requirements:** L2-XML-002, L3-XML-003, L3-XML-004, L3-XML-006, L3-XML-007
pub fn serialize_xml(doc: &TmatsDocument<'_>, writer: &mut dyn Write) -> Result<(), TmatsError> {
    let mut xml_writer = Writer::new_with_indent(writer, b' ', 2);

    // XML declaration
    xml_writer.write_event(Event::Decl(
        quick_xml::events::BytesDecl::new("1.0", Some("UTF-8"), None)
    )).map_err(|e| TmatsError::Xml(XmlError { message: e.to_string() }))?;

    // Root element
    let root_start = BytesStart::new("Tmats");
    xml_writer.write_event(Event::Start(root_start.clone()))
        .map_err(|e| TmatsError::Xml(XmlError { message: e.to_string() }))?;

    // G-Group
    write_xml_g_group(&mut xml_writer, &doc.general)?;

    // T-Groups
    for (_, t) in &doc.transmission {
        write_xml_t_group(&mut xml_writer, t)?;
    }

    // R-Groups
    for (_, r) in &doc.recorders {
        write_xml_r_group(&mut xml_writer, r)?;
    }

    // M-Groups
    for (_, m) in &doc.multiplex {
        write_xml_m_group(&mut xml_writer, m)?;
    }

    // P-Groups
    for (_, p) in &doc.pcm_formats {
        write_xml_p_group(&mut xml_writer, p)?;
    }

    // D-Groups
    for (_, d) in &doc.pcm_measurements {
        write_xml_d_group(&mut xml_writer, d)?;
    }

    // B-Groups
    for (_, b) in &doc.bus_data {
        write_xml_b_group(&mut xml_writer, b)?;
    }

    // S-Groups
    for (_, s) in &doc.message_data {
        write_xml_s_group(&mut xml_writer, s)?;
    }

    // C-Groups — one per data link (L3-XML-007)
    for (_, c) in &doc.data_conversion {
        write_xml_c_group(&mut xml_writer, c)?;
    }

    // Close root
    xml_writer.write_event(Event::End(BytesEnd::new("Tmats")))
        .map_err(|e| TmatsError::Xml(XmlError { message: e.to_string() }))?;

    Ok(())
}

/// Serialize to an in-memory XML buffer.
pub fn serialize_xml_to_vec(doc: &TmatsDocument<'_>) -> Result<Vec<u8>, TmatsError> {
    let mut buf = Vec::with_capacity(4096);
    serialize_xml(doc, &mut buf)?;
    Ok(buf)
}

// ─── XML element helpers ─────────────────────────────────────────────────────

fn write_xml_element<W: Write>(
    w: &mut Writer<W>,
    name: &str,
    value: &str,
) -> Result<(), TmatsError> {
    w.write_event(Event::Start(BytesStart::new(name)))
        .map_err(xml_err)?;
    w.write_event(Event::Text(BytesText::new(value)))
        .map_err(xml_err)?;
    w.write_event(Event::End(BytesEnd::new(name)))
        .map_err(xml_err)?;
    Ok(())
}

fn write_xml_element_expanded<W: Write>(
    w: &mut Writer<W>,
    name: &str,
    value: &str,
) -> Result<(), TmatsError> {
    // L3-XML-004: Expand keyword values
    write_xml_element(w, name, expand_keyword(value))
}

fn xml_err(e: quick_xml::Error) -> TmatsError {
    TmatsError::Xml(XmlError { message: e.to_string() })
}

// ─── Group-specific XML writers ──────────────────────────────────────────────
// Note: \N counter attributes are NOT emitted per L3-XML-003

fn write_xml_g_group<W: Write>(w: &mut Writer<W>, g: &GGroup<'_>) -> Result<(), TmatsError> {
    w.write_event(Event::Start(BytesStart::new("GeneralInformation"))).map_err(xml_err)?;

    if let Some(ref v) = g.program_name { write_xml_element(w, "ProgramName", v)?; }
    if let Some(ref v) = g.irig106_version { write_xml_element(w, "Irig106Version", v)?; }
    if let Some(d) = &g.origination_date {
        write_xml_element(w, "OriginationDate", &tmats_date_to_xml(d))?;
    }
    if let Some(ref v) = g.revision_number { write_xml_element(w, "RevisionNumber", v)?; }
    if let Some(d) = &g.revision_date {
        write_xml_element(w, "RevisionDate", &tmats_date_to_xml(d))?;
    }
    if let Some(ref v) = g.test_number { write_xml_element(w, "TestNumber", v)?; }

    // Data sources — no \N counter (L3-XML-003)
    for (_, ds) in &g.data_sources {
        w.write_event(Event::Start(BytesStart::new("DataSource"))).map_err(xml_err)?;
        if let Some(ref v) = ds.data_source_id { write_xml_element(w, "DataSourceID", v)?; }
        if let Some(ref v) = ds.data_source_type {
            write_xml_element_expanded(w, "DataSourceType", v)?;
        }
        if let Some(ref v) = ds.classification {
            write_xml_element_expanded(w, "Classification", v)?;
        }
        w.write_event(Event::End(BytesEnd::new("DataSource"))).map_err(xml_err)?;
    }

    for comment in &g.comments {
        write_xml_element(w, "Comment", comment)?;
    }

    w.write_event(Event::End(BytesEnd::new("GeneralInformation"))).map_err(xml_err)?;
    Ok(())
}

fn write_xml_t_group<W: Write>(w: &mut Writer<W>, t: &TGroup<'_>) -> Result<(), TmatsError> {
    w.write_event(Event::Start(BytesStart::new("TransmissionAttributes"))).map_err(xml_err)?;
    if let Some(ref v) = t.transmitter_id { write_xml_element(w, "TransmitterID", v)?; }
    if let Some(v) = t.carrier_frequency_mhz { write_xml_element(w, "CarrierFrequency", &v.to_string())?; }
    if let Some(ref v) = t.modulation_type { write_xml_element_expanded(w, "ModulationType", v)?; }
    w.write_event(Event::End(BytesEnd::new("TransmissionAttributes"))).map_err(xml_err)?;
    Ok(())
}

fn write_xml_r_group<W: Write>(w: &mut Writer<W>, r: &RGroup<'_>) -> Result<(), TmatsError> {
    w.write_event(Event::Start(BytesStart::new("RecorderAttributes"))).map_err(xml_err)?;
    if let Some(ref v) = r.recorder_id { write_xml_element(w, "RecorderID", v)?; }
    if let Some(ref v) = r.recorder_description { write_xml_element(w, "Description", v)?; }

    for (_, ch) in &r.channels {
        w.write_event(Event::Start(BytesStart::new("Channel"))).map_err(xml_err)?;
        if let Some(v) = ch.channel_id { write_xml_element(w, "ChannelID", &v.to_string())?; }
        if let Some(ref v) = ch.data_type { write_xml_element(w, "DataType", v)?; }
        if let Some(ref v) = ch.data_link_name { write_xml_element(w, "DataLinkName", v)?; }
        if let Some(ref v) = ch.data_packing_option {
            write_xml_element_expanded(w, "DataPacking", v)?;
        }
        if let Some(v) = ch.channel_enabled {
            write_xml_element(w, "Enabled", if v { "True" } else { "False" })?;
        }
        w.write_event(Event::End(BytesEnd::new("Channel"))).map_err(xml_err)?;
    }

    if let Some(v) = r.index_enabled {
        write_xml_element(w, "IndexEnabled", if v { "True" } else { "False" })?;
    }

    w.write_event(Event::End(BytesEnd::new("RecorderAttributes"))).map_err(xml_err)?;
    Ok(())
}

fn write_xml_m_group<W: Write>(w: &mut Writer<W>, m: &MGroup<'_>) -> Result<(), TmatsError> {
    w.write_event(Event::Start(BytesStart::new("MultiplexAttributes"))).map_err(xml_err)?;
    if let Some(ref v) = m.baseband_signal_type { write_xml_element(w, "BasebandSignalType", v)?; }
    w.write_event(Event::End(BytesEnd::new("MultiplexAttributes"))).map_err(xml_err)?;
    Ok(())
}

fn write_xml_p_group<W: Write>(w: &mut Writer<W>, p: &PGroup<'_>) -> Result<(), TmatsError> {
    w.write_event(Event::Start(BytesStart::new("PCMFormatAttributes"))).map_err(xml_err)?;
    if let Some(ref v) = p.data_link_name { write_xml_element(w, "DataLinkName", v)?; }
    if let Some(v) = p.bit_rate { write_xml_element(w, "BitRate", &v.to_string())?; }
    if let Some(ref v) = p.encoding { write_xml_element_expanded(w, "Encoding", v)?; }
    if let Some(ref v) = p.polarity { write_xml_element_expanded(w, "Polarity", v)?; }
    if let Some(v) = p.num_words_per_frame { write_xml_element(w, "WordsPerFrame", &v.to_string())?; }
    if let Some(v) = p.num_bits_per_word { write_xml_element(w, "BitsPerWord", &v.to_string())?; }
    if let Some(ref v) = p.sync_pattern { write_xml_element(w, "SyncPattern", v)?; }
    if let Some(v) = p.sync_pattern_length { write_xml_element(w, "SyncPatternLength", &v.to_string())?; }
    w.write_event(Event::End(BytesEnd::new("PCMFormatAttributes"))).map_err(xml_err)?;
    Ok(())
}

fn write_xml_d_group<W: Write>(w: &mut Writer<W>, d: &DGroup<'_>) -> Result<(), TmatsError> {
    w.write_event(Event::Start(BytesStart::new("PCMMeasurementDescription"))).map_err(xml_err)?;
    if let Some(ref v) = d.measurement_list_name { write_xml_element(w, "MeasurementListName", v)?; }
    if let Some(ref v) = d.data_link_name { write_xml_element(w, "DataLinkName", v)?; }
    w.write_event(Event::End(BytesEnd::new("PCMMeasurementDescription"))).map_err(xml_err)?;
    Ok(())
}

fn write_xml_b_group<W: Write>(w: &mut Writer<W>, b: &BGroup<'_>) -> Result<(), TmatsError> {
    w.write_event(Event::Start(BytesStart::new("BusDataAttributes"))).map_err(xml_err)?;
    if let Some(ref v) = b.data_link_name { write_xml_element(w, "DataLinkName", v)?; }
    if let Some(ref v) = b.bus_type { write_xml_element_expanded(w, "BusType", v)?; }
    w.write_event(Event::End(BytesEnd::new("BusDataAttributes"))).map_err(xml_err)?;
    Ok(())
}

fn write_xml_s_group<W: Write>(w: &mut Writer<W>, s: &SGroup<'_>) -> Result<(), TmatsError> {
    w.write_event(Event::Start(BytesStart::new("MessageDataAttributes"))).map_err(xml_err)?;
    if let Some(ref v) = s.data_link_name { write_xml_element(w, "DataLinkName", v)?; }
    w.write_event(Event::End(BytesEnd::new("MessageDataAttributes"))).map_err(xml_err)?;
    Ok(())
}

fn write_xml_c_group<W: Write>(w: &mut Writer<W>, c: &CGroup<'_>) -> Result<(), TmatsError> {
    w.write_event(Event::Start(BytesStart::new("DataConversionAttributes"))).map_err(xml_err)?;
    if let Some(ref v) = c.measurement_name { write_xml_element(w, "MeasurementName", v)?; }
    if let Some(ref v) = c.conversion_type { write_xml_element_expanded(w, "ConversionType", v)?; }
    if let Some(ref v) = c.eu_units { write_xml_element(w, "EngineeringUnits", v)?; }
    w.write_event(Event::End(BytesEnd::new("DataConversionAttributes"))).map_err(xml_err)?;
    Ok(())
}

// ═════════════════════════════════════════════════════════════════════════════
// XML PARSER (L2-XML-001, L3-XML-001, L3-XML-002, L3-XML-008)
// ═════════════════════════════════════════════════════════════════════════════

/// Parse TMATS XML into a TmatsDocument.
///
/// **Requirements:** L2-XML-001, L3-XML-001, L3-XML-002, L3-XML-006, L3-XML-008
pub fn parse_xml(input: &[u8]) -> Result<OwnedTmatsDocument, TmatsError> {
    let mut reader = Reader::from_reader(input);
    reader.config_mut().trim_text(true);

    let mut doc = TmatsDocument::empty();
    let mut buf = Vec::new();
    let mut path: Vec<String> = Vec::new();
    let mut text_buf = String::new();

    // Mutable state for current group being parsed
    let mut current_ds: Option<DataSourceDecl<'static>> = None;
    let mut current_ds_idx: u32 = 0;
    let mut current_ch: Option<RChannel<'static>> = None;
    let mut current_ch_idx: u32 = 0;
    let mut r_group: Option<RGroup<'static>> = None;

    loop {
        match reader.read_event_into(&mut buf) {
            Ok(Event::Start(ref e)) => {
                let name = String::from_utf8_lossy(e.name().as_ref()).to_string();
                path.push(name);
                text_buf.clear();
            }
            Ok(Event::End(ref e)) => {
                let name = String::from_utf8_lossy(e.name().as_ref()).to_string();
                let text = text_buf.trim().to_string();

                // Route based on current path context
                let parent = path.get(path.len().saturating_sub(2)).map(|s| s.as_str());

                match (parent, name.as_str()) {
                    // G-Group
                    (Some("GeneralInformation"), "ProgramName") => {
                        doc.general.program_name = Some(Cow::Owned(text));
                    }
                    (Some("GeneralInformation"), "Irig106Version") => {
                        doc.general.irig106_version = Some(Cow::Owned(text.clone()));
                        doc.source_version = Irig106Version::from_g106_str(&text);
                    }
                    (Some("GeneralInformation"), "OriginationDate") => {
                        doc.general.origination_date = xml_date_to_tmats(&text);
                    }
                    (Some("GeneralInformation"), "RevisionNumber") => {
                        doc.general.revision_number = Some(Cow::Owned(text));
                    }
                    (Some("GeneralInformation"), "RevisionDate") => {
                        doc.general.revision_date = xml_date_to_tmats(&text);
                    }
                    (Some("GeneralInformation"), "TestNumber") => {
                        doc.general.test_number = Some(Cow::Owned(text));
                    }
                    (Some("GeneralInformation"), "Comment") => {
                        doc.general.comments.push(Cow::Owned(text));
                    }

                    // Data source elements
                    (Some("DataSource"), "DataSourceID") => {
                        if current_ds.is_none() { current_ds = Some(DataSourceDecl::default()); }
                        current_ds.as_mut().unwrap().data_source_id = Some(Cow::Owned(text));
                    }
                    (Some("DataSource"), "DataSourceType") => {
                        if current_ds.is_none() { current_ds = Some(DataSourceDecl::default()); }
                        current_ds.as_mut().unwrap().data_source_type =
                            Some(Cow::Owned(collapse_keyword(&text).to_string()));
                    }
                    (Some("DataSource"), "Classification") => {
                        if current_ds.is_none() { current_ds = Some(DataSourceDecl::default()); }
                        current_ds.as_mut().unwrap().classification =
                            Some(Cow::Owned(collapse_keyword(&text).to_string()));
                    }
                    (_, "DataSource") => {
                        if let Some(ds) = current_ds.take() {
                            current_ds_idx += 1;
                            doc.general.data_sources.insert(current_ds_idx, ds);
                        }
                    }

                    // R-Group / Channel
                    (Some("RecorderAttributes"), "RecorderID") => {
                        if r_group.is_none() { r_group = Some(RGroup::default()); }
                        r_group.as_mut().unwrap().recorder_id = Some(Cow::Owned(text));
                    }
                    (Some("Channel"), "ChannelID") => {
                        if current_ch.is_none() { current_ch = Some(RChannel::default()); }
                        current_ch.as_mut().unwrap().channel_id = text.parse().ok();
                    }
                    (Some("Channel"), "DataType") => {
                        if current_ch.is_none() { current_ch = Some(RChannel::default()); }
                        current_ch.as_mut().unwrap().data_type = Some(Cow::Owned(text));
                    }
                    (Some("Channel"), "DataLinkName") => {
                        if current_ch.is_none() { current_ch = Some(RChannel::default()); }
                        current_ch.as_mut().unwrap().data_link_name = Some(Cow::Owned(text));
                    }
                    (Some("Channel"), "DataPacking") => {
                        if current_ch.is_none() { current_ch = Some(RChannel::default()); }
                        current_ch.as_mut().unwrap().data_packing_option =
                            Some(Cow::Owned(collapse_keyword(&text).to_string()));
                    }
                    (_, "Channel") => {
                        if let Some(ch) = current_ch.take() {
                            current_ch_idx += 1;
                            if let Some(ref mut rg) = r_group {
                                rg.channels.insert(current_ch_idx, ch);
                            }
                        }
                    }
                    (_, "RecorderAttributes") => {
                        if let Some(mut rg) = r_group.take() {
                            rg.num_channels = Some(rg.channels.len() as u32);
                            let idx = (doc.recorders.len() + 1) as u32;
                            doc.recorders.insert(idx, rg);
                            current_ch_idx = 0;
                        }
                    }

                    // P-Group
                    (Some("PCMFormatAttributes"), "DataLinkName") => {
                        let idx = (doc.pcm_formats.len() + 1) as u32;
                        let p = doc.pcm_formats.entry(idx).or_default();
                        p.data_link_name = Some(Cow::Owned(text));
                    }
                    (Some("PCMFormatAttributes"), "BitRate") => {
                        let idx = doc.pcm_formats.len() as u32;
                        if let Some(p) = doc.pcm_formats.get_mut(&idx) {
                            p.bit_rate = text.parse().ok();
                        }
                    }
                    (Some("PCMFormatAttributes"), "Encoding") => {
                        let idx = doc.pcm_formats.len() as u32;
                        if let Some(p) = doc.pcm_formats.get_mut(&idx) {
                            p.encoding = Some(Cow::Owned(collapse_keyword(&text).to_string()));
                        }
                    }

                    // B-Group
                    (Some("BusDataAttributes"), "DataLinkName") => {
                        let idx = (doc.bus_data.len() + 1) as u32;
                        let b = doc.bus_data.entry(idx).or_default();
                        b.data_link_name = Some(Cow::Owned(text));
                    }
                    (Some("BusDataAttributes"), "BusType") => {
                        let idx = doc.bus_data.len() as u32;
                        if let Some(b) = doc.bus_data.get_mut(&idx) {
                            b.bus_type = Some(Cow::Owned(collapse_keyword(&text).to_string()));
                        }
                    }

                    // C-Group
                    (Some("DataConversionAttributes"), "MeasurementName") => {
                        let idx = (doc.data_conversion.len() + 1) as u32;
                        let c = doc.data_conversion.entry(idx).or_default();
                        c.measurement_name = Some(Cow::Owned(text));
                    }
                    (Some("DataConversionAttributes"), "ConversionType") => {
                        let idx = doc.data_conversion.len() as u32;
                        if let Some(c) = doc.data_conversion.get_mut(&idx) {
                            c.conversion_type = Some(Cow::Owned(collapse_keyword(&text).to_string()));
                        }
                    }
                    (Some("DataConversionAttributes"), "EngineeringUnits") => {
                        let idx = doc.data_conversion.len() as u32;
                        if let Some(c) = doc.data_conversion.get_mut(&idx) {
                            c.eu_units = Some(Cow::Owned(text));
                        }
                    }

                    _ => {}
                }

                path.pop();
                text_buf.clear();
            }
            Ok(Event::Text(ref e)) => {
                text_buf.push_str(&e.unescape().unwrap_or_default());
            }
            Ok(Event::Eof) => break,
            Err(e) => {
                return Err(TmatsError::Xml(XmlError {
                    message: format!("XML parse error: {e}"),
                }));
            }
            _ => {}
        }
        buf.clear();
    }

    // Auto-compute counters
    doc.general.num_data_sources = Some(doc.general.data_sources.len() as u32);

    Ok(doc)
}

// ═════════════════════════════════════════════════════════════════════════════
// BIDIRECTIONAL CONVERSION (L2-XML-004)
// ═════════════════════════════════════════════════════════════════════════════

/// Convert an ASCII-parsed document to XML bytes.
///
/// **Requirement:** L2-XML-004
pub fn ascii_to_xml(ascii_input: &[u8]) -> Result<Vec<u8>, TmatsError> {
    let doc = crate::parse::parse(ascii_input).map_err(|e| {
        TmatsError::Xml(XmlError { message: format!("ASCII parse failed: {e}") })
    })?;
    serialize_xml_to_vec(&doc)
}

/// Convert an XML document to ASCII bytes.
///
/// **Requirement:** L2-XML-004
pub fn xml_to_ascii(xml_input: &[u8]) -> Result<Vec<u8>, TmatsError> {
    let doc = parse_xml(xml_input)?;
    crate::serial::serialize_to_vec(&doc).map_err(|e| TmatsError::from(e))
}
