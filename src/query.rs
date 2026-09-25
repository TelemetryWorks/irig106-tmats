// irig106-tmats/src/query.rs
//
// # Attribute Query and Navigation
//
// Provides programmatic navigation of the TMATS group hierarchy and
// resolution of cross-group references.
//
// ## Traceability:
//   L1-QUERY → L2-QUERY-001..006 → L3-QUERY-001..007

use std::collections::HashMap;

use crate::model::*;

// ─── Channel Configuration Resolution (L3-QUERY-003) ────────────────────────

/// Resolved configuration chain for a single Ch10 channel.
///
/// **Requirement:** L2-QUERY-004, L3-QUERY-003
#[derive(Debug)]
pub struct ChannelConfig<'d, 'a: 'd> {
    /// The Chapter 10 channel ID.
    pub channel_id: u16,
    /// The R-group this channel belongs to.
    pub recorder: &'d RGroup<'a>,
    /// The R-group occurrence index.
    pub recorder_index: u32,
    /// The specific channel definition within the R-group.
    pub recorder_channel: &'d RChannel<'a>,
    /// The linked format group, if resolved.
    pub format: Option<FormatRef<'d, 'a>>,
    /// Linked D-group measurement descriptions.
    pub measurements: Vec<&'d DGroup<'a>>,
    /// Linked C-group data conversions.
    pub conversions: Vec<&'d CGroup<'a>>,
}

/// Reference to a format group (PCM, Bus, or Message).
///
/// **Requirement:** L3-QUERY-003
#[derive(Debug)]
pub enum FormatRef<'d, 'a: 'd> {
    Pcm(&'d PGroup<'a>),
    Bus(&'d BGroup<'a>),
    Message(&'d SGroup<'a>),
}

// ═════════════════════════════════════════════════════════════════════════════
// QUERY API
// ═════════════════════════════════════════════════════════════════════════════

/// Resolve a Chapter 10 channel ID to its complete configuration chain.
///
/// Walks: R-group channel → data link name → P/B/S group → D-group → C-group.
///
/// **Requirements:** L2-QUERY-004, L3-QUERY-003, L2-QUERY-003
pub fn resolve_channel<'d, 'a: 'd>(
    doc: &'d TmatsDocument<'a>,
    channel_id: u16,
) -> Option<ChannelConfig<'d, 'a>> {
    // Find the R-group and channel entry matching this channel ID
    for (&r_idx, r_group) in &doc.recorders {
        for (_, r_channel) in &r_group.channels {
            if r_channel.channel_id == Some(channel_id) {
                let data_link_name = r_channel.data_link_name.as_deref();

                // Resolve format group via data link name
                let format = data_link_name.and_then(|dln| {
                    // Try P-group first
                    for (_, p) in &doc.pcm_formats {
                        if p.data_link_name.as_deref() == Some(dln) {
                            return Some(FormatRef::Pcm(p));
                        }
                    }
                    // Try B-group
                    for (_, b) in &doc.bus_data {
                        if b.data_link_name.as_deref() == Some(dln) {
                            return Some(FormatRef::Bus(b));
                        }
                    }
                    // Try S-group
                    for (_, s) in &doc.message_data {
                        if s.data_link_name.as_deref() == Some(dln) {
                            return Some(FormatRef::Message(s));
                        }
                    }
                    None
                });

                // Resolve D-groups via data link name
                let measurements: Vec<&DGroup<'a>> = data_link_name
                    .map(|dln| {
                        doc.pcm_measurements
                            .values()
                            .filter(|d| d.data_link_name.as_deref() == Some(dln))
                            .collect()
                    })
                    .unwrap_or_default();

                // Resolve C-groups via measurement names from D-groups
                let conversions: Vec<&CGroup<'a>> = doc
                    .data_conversion
                    .values()
                    .filter(|c| {
                        // Link C-group if its measurement_name matches any D-group measurement
                        c.measurement_name.as_deref().is_some_and(|mn| {
                            measurements
                                .iter()
                                .any(|d| d.measurement_list_name.as_deref() == Some(mn))
                        })
                    })
                    .collect();

                return Some(ChannelConfig {
                    channel_id,
                    recorder: r_group,
                    recorder_index: r_idx,
                    recorder_channel: r_channel,
                    format,
                    measurements,
                    conversions,
                });
            }
        }
    }

    None
}

/// Enumerate all declared data sources from the G-group.
///
/// **Requirement:** L2-QUERY-005
pub fn enumerate_data_sources<'d, 'a: 'd>(
    doc: &'d TmatsDocument<'a>,
) -> Vec<(u32, &'d DataSourceDecl<'a>)> {
    doc.general
        .data_sources
        .iter()
        .map(|(&idx, ds)| (idx, ds))
        .collect()
}

/// Enumerate all channels across all R-groups.
///
/// Returns (recorder_index, channel_index, &RChannel) tuples.
pub fn enumerate_channels<'d, 'a: 'd>(
    doc: &'d TmatsDocument<'a>,
) -> Vec<(u32, u32, &'d RChannel<'a>)> {
    let mut result = Vec::new();
    for (&r_idx, r_group) in &doc.recorders {
        for (&ch_idx, ch) in &r_group.channels {
            result.push((r_idx, ch_idx, ch));
        }
    }
    result
}

/// Compute a structured diff between two TmatsDocument instances.
///
/// **Requirement:** L2-QUERY-006, L3-QUERY-006
#[derive(Debug, Default)]
pub struct TmatsDiff {
    /// Attributes present in `b` but not `a`.
    pub added: Vec<(String, String)>,
    /// Attributes present in `a` but not `b`.
    pub removed: Vec<(String, String)>,
    /// Attributes present in both but with different values.
    pub modified: Vec<(String, String, String)>,
}

/// Compare two documents and produce a diff.
///
/// **Requirement:** L2-QUERY-006, L3-QUERY-006
pub fn diff<'a>(
    a: &TmatsDocument<'a>,
    b: &TmatsDocument<'a>,
) -> Result<TmatsDiff, crate::error::TmatsError> {
    use crate::serial;

    let a_bytes = serial::serialize_to_vec(a)?;
    let b_bytes = serial::serialize_to_vec(b)?;

    let a_map = bytes_to_attr_map(&a_bytes);
    let b_map = bytes_to_attr_map(&b_bytes);

    let mut result = TmatsDiff::default();

    for (key, val) in &a_map {
        match b_map.get(key.as_str()) {
            Some(b_val) if b_val != val => {
                result
                    .modified
                    .push((key.clone(), val.clone(), b_val.clone()));
            }
            None => {
                result.removed.push((key.clone(), val.clone()));
            }
            _ => {}
        }
    }

    for (key, val) in &b_map {
        if !a_map.contains_key(key.as_str()) {
            result.added.push((key.clone(), val.clone()));
        }
    }

    Ok(result)
}

/// Parse serialized TMATS bytes into a simple key-value map for diffing.
fn bytes_to_attr_map(bytes: &[u8]) -> HashMap<String, String> {
    let mut map = HashMap::new();
    let text = String::from_utf8_lossy(bytes);

    for attr_str in text.split(';') {
        let attr_str = attr_str.trim();
        if attr_str.is_empty() {
            continue;
        }
        if let Some(colon) = attr_str.find(':') {
            let key = attr_str[..colon].trim().to_string();
            let val = attr_str[colon + 1..].to_string();
            map.insert(key, val);
        }
    }

    map
}
