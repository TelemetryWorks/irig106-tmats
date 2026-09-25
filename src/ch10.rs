// irig106-tmats/src/ch10.rs
//
// # Chapter 10 Payload Integration
//
// Decode/encode TMATS from/to Ch10 Type 0x01 setup record payloads.
// This crate produces PAYLOADS, not packets. Packet framing (headers,
// trailers, checksums) is the responsibility of `irig106-write`.
//
// ## Traceability:
//   L1-CH10 → L2-CH10-001..007 → L3-CH10-001..008

use crate::error::{Ch10Error, TmatsError, TmatsErrors};
use crate::model::{TmatsDocument, OwnedTmatsDocument};
use crate::parse::{self, ParseOptions};
use crate::serial;
use crate::types_bridge::Irig106Version;

/// Size of the CSDW in bytes.
const CSDW_SIZE: usize = 4;

// ─── CSDW Struct (L3-CH10-001) ───────────────────────────────────────────────

/// Type 0x01 Computer-Generated Data Format 1 (Setup Record) CSDW.
///
/// **Requirement:** L3-CH10-001
/// **Spec:** RCC 123-20 §5.5.2
///
/// Layout (32 bits, little-endian):
///   Bits  0-7:  iCh10Ver — IRIG 106 Chapter 10 version
///   Bit   8:    bConfigChange — recorder config changed since last TMATS
///   Bits  9-31: Reserved (must be zero)
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct SetupRecordCsdw {
    /// IRIG 106 Chapter 10 version encoding.
    pub ch10_version: u8,
    /// Whether recorder configuration has changed.
    pub config_change: bool,
}

impl SetupRecordCsdw {
    /// Decode CSDW from the first 4 bytes of a payload buffer.
    ///
    /// **Requirement:** L2-CH10-001, L3-CH10-002
    pub fn decode(bytes: &[u8]) -> Result<Self, Ch10Error> {
        if bytes.len() < CSDW_SIZE {
            return Err(Ch10Error {
                message: format!(
                    "CSDW requires {} bytes, got {}",
                    CSDW_SIZE,
                    bytes.len()
                ),
            });
        }

        let word = u32::from_le_bytes([bytes[0], bytes[1], bytes[2], bytes[3]]);

        Ok(Self {
            ch10_version: (word & 0xFF) as u8,
            config_change: (word >> 8) & 1 == 1,
        })
    }

    /// Encode CSDW to a 4-byte little-endian buffer.
    ///
    /// **Requirement:** L2-CH10-002, L3-CH10-003
    pub fn encode(&self) -> [u8; CSDW_SIZE] {
        let mut word: u32 = self.ch10_version as u32;
        if self.config_change {
            word |= 1 << 8;
        }
        // Bits 9-31 reserved = 0
        word.to_le_bytes()
    }

    /// Map the CSDW version field to an `Irig106Version`.
    ///
    /// Returns `None` for pre-106-07 files where the field is undefined (zero).
    ///
    /// **Requirement:** L3-VERSION-009, L2-CH10-006
    pub fn irig_version(&self) -> Option<Irig106Version> {
        Irig106Version::from_csdw_version(self.ch10_version)
    }

    /// Create a CSDW from an `Irig106Version`.
    pub fn from_version(version: Irig106Version, config_change: bool) -> Self {
        Self {
            ch10_version: version.to_csdw_version(),
            config_change,
        }
    }
}

// ─── Setup Record Payload (L3-CH10-005) ──────────────────────────────────────

/// Complete setup record payload ready for consumption by `irig106-write`.
///
/// **Requirement:** L3-CH10-005, L2-INTEROP-004
///
/// ## Contract (L3-INTEROP-009):
/// The bytes are formatted as: `[CSDW (4 bytes LE)] + [TMATS payload]`.
/// Callers (irig106-write) are responsible for wrapping this in a Type 0x01
/// Ch10 packet with appropriate header, secondary header, and trailer.
#[derive(Debug, Clone)]
pub struct SetupRecordPayload {
    /// The CSDW for this setup record.
    pub csdw: SetupRecordCsdw,
    /// Serialized TMATS bytes (ASCII or XML).
    pub tmats_bytes: Vec<u8>,
}

impl SetupRecordPayload {
    /// Total payload size (CSDW + TMATS data).
    pub fn total_len(&self) -> usize {
        CSDW_SIZE + self.tmats_bytes.len()
    }

    /// Produce the complete payload byte vector.
    pub fn to_bytes(&self) -> Vec<u8> {
        let mut out = Vec::with_capacity(self.total_len());
        out.extend_from_slice(&self.csdw.encode());
        out.extend_from_slice(&self.tmats_bytes);
        out
    }
}

/// Payload encoding format.
///
/// **Requirement:** L3-CH10-008
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PayloadEncoding {
    Ascii,
    #[cfg(feature = "xml")]
    Xml,
}

// ═════════════════════════════════════════════════════════════════════════════
// PUBLIC API (L3-INTEROP-003)
// ═════════════════════════════════════════════════════════════════════════════

/// Decode a Ch10 Type 0x01 setup record payload into a CSDW + TmatsDocument.
///
/// The `payload` parameter is the raw data bytes of the Ch10 data packet
/// (everything after the packet header/secondary header, before the trailer).
///
/// **Requirements:** L2-CH10-001, L2-CH10-003, L3-INTEROP-003
/// **Spec:** RCC 123-20 §5.5.2
pub fn decode_setup_payload(payload: &[u8]) -> Result<(SetupRecordCsdw, TmatsDocument<'_>), TmatsError> {
    // Decode CSDW from first 4 bytes
    let csdw = SetupRecordCsdw::decode(payload)?;

    // Extract TMATS payload after CSDW (L2-CH10-003, L3-CH10-004)
    if payload.len() <= CSDW_SIZE {
        return Err(TmatsError::Ch10(Ch10Error {
            message: "setup record payload contains only CSDW, no TMATS data".into(),
        }));
    }

    let tmats_bytes = &payload[CSDW_SIZE..];

    // Build parse options with version hint from CSDW
    let mut opts = ParseOptions::default();
    if let Some(ver) = csdw.irig_version() {
        opts.target_version = Some(ver);
    }

    let doc = parse::parse_with_options(tmats_bytes, &opts).map_err(|errs| {
        // Take the first error for the top-level TmatsError
        errs.errors.into_iter().next().unwrap_or(
            TmatsError::Ch10(Ch10Error { message: "TMATS parse failed".into() })
        )
    })?;

    Ok((csdw, doc))
}

/// Encode a TmatsDocument into a setup record payload for irig106-write.
///
/// **Requirements:** L2-CH10-002, L2-CH10-004, L3-INTEROP-003
pub fn encode_setup_payload(
    doc: &TmatsDocument<'_>,
    csdw: &SetupRecordCsdw,
) -> Result<SetupRecordPayload, TmatsError> {
    let tmats_bytes = serial::serialize_to_vec(doc)?;

    Ok(SetupRecordPayload {
        csdw: *csdw,
        tmats_bytes,
    })
}

/// Compare two documents and determine if config has changed.
///
/// **Requirement:** L2-CH10-007, L3-CH10-006
pub fn detect_config_change(
    prev: &TmatsDocument<'_>,
    curr: &TmatsDocument<'_>,
) -> Result<bool, TmatsError> {
    let prev_bytes = serial::serialize_to_vec(prev)?;
    let curr_bytes = serial::serialize_to_vec(curr)?;
    Ok(prev_bytes != curr_bytes)
}
