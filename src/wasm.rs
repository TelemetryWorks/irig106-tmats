// irig106-tmats/src/wasm.rs
//
// # WASM Bindings
//
// Browser-compatible API for irig106-studio using wasm-bindgen.
// Exposes parse, validate, and channel resolution as JS-callable functions.
//
// ## Traceability:
//   L2-INTEROP-006: no_std core compatibility (model types)
//   L2-INTEROP-007: wasm-bindgen-compatible API
//   L3-INTEROP-008: WASM bindings: parse_tmats, validate_tmats, get_channel_config
//   L3-TEST-011: WASM smoke tests

use wasm_bindgen::prelude::*;

/// Parse TMATS ASCII data and return the document as JSON.
///
/// **Requirement:** L3-INTEROP-008
///
/// # Arguments
/// * `input` - Raw TMATS ASCII bytes
///
/// # Returns
/// JSON string representation of TmatsDocument, or error message.
#[wasm_bindgen]
pub fn parse_tmats(input: &[u8]) -> Result<JsValue, JsError> {
    let doc = crate::parse::parse_owned(input)
        .map_err(|e| JsError::new(&format!("parse failed: {e}")))?;

    serde_wasm_bindgen::to_value(&doc)
        .map_err(|e| JsError::new(&format!("serialization failed: {e}")))
}

/// Validate TMATS ASCII data and return the validation report as JSON.
///
/// **Requirement:** L3-INTEROP-008
///
/// # Arguments
/// * `input` - Raw TMATS ASCII bytes
///
/// # Returns
/// JSON string representation of ValidationReport.
#[wasm_bindgen]
pub fn validate_tmats(input: &[u8]) -> Result<JsValue, JsError> {
    let doc = crate::parse::parse_owned(input)
        .map_err(|e| JsError::new(&format!("parse failed: {e}")))?;

    let report = crate::validate::validate(&doc);

    serde_wasm_bindgen::to_value(&report)
        .map_err(|e| JsError::new(&format!("serialization failed: {e}")))
}

/// Resolve a channel ID and return its configuration chain as JSON.
///
/// **Requirement:** L3-INTEROP-008
///
/// # Arguments
/// * `input` - Raw TMATS ASCII bytes
/// * `channel_id` - The Chapter 10 channel ID to resolve
///
/// # Returns
/// JSON object with channel configuration, or null if not found.
#[wasm_bindgen]
pub fn get_channel_config(input: &[u8], channel_id: u16) -> Result<JsValue, JsError> {
    let doc = crate::parse::parse(input)
        .map_err(|e| JsError::new(&format!("parse failed: {e}")))?;

    match crate::query::resolve_channel(&doc, channel_id) {
        Some(config) => {
            // Build a serializable summary since ChannelConfig has references
            let summary = ChannelConfigSummary {
                channel_id: config.channel_id,
                recorder_id: config.recorder.recorder_id.as_deref()
                    .unwrap_or("unknown").to_string(),
                data_type: config.recorder_channel.data_type.as_deref()
                    .unwrap_or("unknown").to_string(),
                data_link_name: config.recorder_channel.data_link_name.as_deref()
                    .unwrap_or("none").to_string(),
                format_type: match &config.format {
                    Some(crate::query::FormatRef::Pcm(_)) => "PCM".to_string(),
                    Some(crate::query::FormatRef::Bus(_)) => "Bus".to_string(),
                    Some(crate::query::FormatRef::Message(_)) => "Message".to_string(),
                    None => "None".to_string(),
                },
                measurement_count: config.measurements.len(),
                conversion_count: config.conversions.len(),
            };

            serde_wasm_bindgen::to_value(&summary)
                .map_err(|e| JsError::new(&format!("serialization failed: {e}")))
        }
        None => Ok(JsValue::NULL),
    }
}

/// Serialize a TMATS document to ASCII format.
///
/// # Arguments
/// * `json_doc` - JSON representation of TmatsDocument (from parse_tmats)
///
/// # Returns
/// ASCII TMATS bytes.
#[wasm_bindgen]
pub fn serialize_tmats_ascii(input: &[u8]) -> Result<Vec<u8>, JsError> {
    let doc = crate::parse::parse(input)
        .map_err(|e| JsError::new(&format!("parse failed: {e}")))?;

    crate::serial::serialize_to_vec(&doc)
        .map_err(|e| JsError::new(&format!("serialize failed: {e}")))
}

/// Convert ASCII TMATS to XML format.
#[cfg(feature = "xml")]
#[wasm_bindgen]
pub fn convert_ascii_to_xml(input: &[u8]) -> Result<Vec<u8>, JsError> {
    crate::xml::ascii_to_xml(input)
        .map_err(|e| JsError::new(&format!("conversion failed: {e}")))
}

// ─── Internal types for WASM serialization ───────────────────────────────────

#[derive(serde::Serialize)]
struct ChannelConfigSummary {
    channel_id: u16,
    recorder_id: String,
    data_type: String,
    data_link_name: String,
    format_type: String,
    measurement_count: usize,
    conversion_count: usize,
}
