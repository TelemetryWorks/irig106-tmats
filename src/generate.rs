// irig106-tmats/src/generate.rs
//
// # TMATS Generation
//
// Generates minimal spec-compliant TMATS records from a channel inventory.
//
// ## Traceability:
//   L1-GEN → L2-GEN-001..007 → L3-GEN-001..008

use std::borrow::Cow;

use crate::error::TmatsError;
use crate::model::*;
use crate::types_bridge::{DataTypeCode, Irig106Version};

#[cfg(feature = "generate")]
pub use builder::*;

// ═════════════════════════════════════════════════════════════════════════════
// Channel Inventory (L3-GEN-001)
// ═════════════════════════════════════════════════════════════════════════════

/// Description of an observed channel for TMATS generation.
///
/// **Requirement:** L2-GEN-001, L3-GEN-001
#[derive(Debug, Clone)]
pub struct ChannelInventoryEntry {
    /// Channel ID (maps to R-x\TK1-n).
    pub channel_id: u16,
    /// Data type code observed in packet headers.
    pub data_type: DataTypeCode,
    /// Number of subchannels observed, if applicable.
    pub subchannel_count: Option<u16>,
    /// Observed bit rate in bps (for PCM channels).
    pub observed_bit_rate: Option<f64>,
    /// Observed sample rate (for analog channels).
    pub observed_sample_rate: Option<f64>,
    /// Total packets observed.
    pub packet_count: Option<u64>,
}

// ═════════════════════════════════════════════════════════════════════════════
// Builder API (L3-GEN-002, L2-GEN-006)
// ═════════════════════════════════════════════════════════════════════════════

#[cfg(feature = "generate")]
mod builder {
    use super::*;

    /// Builder for generating TMATS documents from channel inventory.
    ///
    /// **Requirement:** L2-GEN-006, L3-GEN-002
    ///
    /// # Example
    /// ```ignore
    /// let doc = TmatsBuilder::new(Irig106Version::V106_17)
    ///     .program_name("FLIGHT_TEST_001")
    ///     .add_channel(ChannelInventoryEntry { ... })
    ///     .build()?;
    /// ```
    #[derive(Debug)]
    pub struct TmatsBuilder {
        version: Irig106Version,
        program_name: Option<String>,
        test_number: Option<String>,
        channels: Vec<ChannelInventoryEntry>,
    }

    impl TmatsBuilder {
        pub fn new(version: Irig106Version) -> Self {
            Self {
                version,
                program_name: None,
                test_number: None,
                channels: Vec::new(),
            }
        }

        pub fn program_name(mut self, name: &str) -> Self {
            self.program_name = Some(name.to_string());
            self
        }

        pub fn test_number(mut self, tn: &str) -> Self {
            self.test_number = Some(tn.to_string());
            self
        }

        /// Add a channel from the inventory.
        ///
        /// **Requirement:** L2-GEN-001
        pub fn add_channel(mut self, entry: ChannelInventoryEntry) -> Self {
            self.channels.push(entry);
            self
        }

        /// Build and validate the generated document.
        ///
        /// **Requirements:** L2-GEN-002..007, L3-GEN-003..006, L3-INTEROP-005
        pub fn build(self) -> Result<OwnedTmatsDocument, TmatsError> {
            // L3-GEN-003: Validate builder state
            if self.channels.is_empty() {
                return Err(TmatsError::Parse(crate::error::ParseError {
                    kind: crate::error::ParseErrorKind::InvalidValue,
                    byte_offset: 0,
                    line: 0,
                    code_name: None,
                    message: "at least one channel is required for generation".into(),
                }));
            }

            let mut doc = TmatsDocument::empty();
            doc.source_version = Some(self.version);

            // L3-GEN-004: Generate G-group
            doc.general.program_name = Some(Cow::Owned(
                self.program_name.unwrap_or_else(|| "UNKNOWN".to_string()),
            ));
            doc.general.irig106_version = Some(Cow::Owned(self.version.as_g106_str().to_string()));
            if let Some(tn) = self.test_number {
                doc.general.test_number = Some(Cow::Owned(tn));
            }

            // Data source declarations
            doc.general.num_data_sources = Some(self.channels.len() as u32);
            for (i, ch) in self.channels.iter().enumerate() {
                let idx = (i + 1) as u32;
                doc.general.data_sources.insert(
                    idx,
                    DataSourceDecl {
                        data_source_id: Some(Cow::Owned(format!("CHAN_{}", ch.channel_id))),
                        data_source_type: Some(Cow::Owned("REC".to_string())),
                        classification: Some(Cow::Owned("U".to_string())),
                        extra: Vec::new(),
                    },
                );
            }

            // L3-GEN-005: Generate R-group
            let mut r_group = RGroup {
                recorder_id: Some(Cow::Owned("GENERATED".to_string())),
                num_channels: Some(self.channels.len() as u32),
                ..RGroup::default()
            };

            for (i, ch) in self.channels.iter().enumerate() {
                let ch_idx = (i + 1) as u32;
                let dln = format!("LINK_{}", ch.channel_id);

                r_group.channels.insert(
                    ch_idx,
                    RChannel {
                        channel_id: Some(ch.channel_id),
                        data_type: Some(Cow::Owned(format!("{:02X}", ch.data_type.to_u8()))),
                        data_source_id: Some(Cow::Owned(format!("CHAN_{}", ch.channel_id))),
                        data_link_name: Some(Cow::Owned(dln.clone())),
                        data_packing_option: Some(Cow::Owned("UN".to_string())),
                        channel_enabled: Some(true),
                        extra: Vec::new(),
                    },
                );

                // L3-GEN-006: Generate format group stubs
                match ch.data_type.tmats_group() {
                    Some(crate::types_bridge::GroupPrefix::P) => {
                        let p = PGroup {
                            data_link_name: Some(Cow::Owned(dln)),
                            bit_rate: ch.observed_bit_rate,
                            ..PGroup::default()
                        };
                        doc.pcm_formats.insert(ch_idx, p);
                    }
                    Some(crate::types_bridge::GroupPrefix::B) => {
                        let b = BGroup {
                            data_link_name: Some(Cow::Owned(dln)),
                            ..BGroup::default()
                        };
                        doc.bus_data.insert(ch_idx, b);
                    }
                    Some(crate::types_bridge::GroupPrefix::S) => {
                        let s = SGroup {
                            data_link_name: Some(Cow::Owned(dln)),
                            ..SGroup::default()
                        };
                        doc.message_data.insert(ch_idx, s);
                    }
                    _ => {}
                }
            }

            doc.recorders.insert(1, r_group);

            Ok(doc)
        }
    }
}
