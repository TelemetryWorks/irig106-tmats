// irig106-tmats/src/types_bridge.rs
//
// # Types Bridge — Types That Belong in `irig106-types`
//
// These types are defined here temporarily. Once `irig106-types` exports them,
// this module should be replaced with re-exports.
//
// ## Types to migrate to `irig106-types`:
//   - `Irig106Version`     → used across all crates for version-aware processing
//   - `DataTypeCode`       → Ch10 data type codes (0x00–0xFF), used by ch10-reader, write, decode
//   - `ChannelId`          → Newtype for u16 channel IDs
//   - `GroupPrefix`        → TMATS group letter identifiers (G, T, R, M, P, D, B, S, C)
//
// ## Traceability:
//   L3-VERSION-001, L3-INTEROP-010, L3-PARSE-006

use core::fmt;

/// IRIG 106 standard version identifiers.
///
/// Maps to the `G\106` TMATS attribute and the `iCh10Ver` CSDW field.
///
/// **Requirement:** L3-VERSION-001
/// **Spec:** Ch9 §9.5.2 Table 9-1 (G\106 attribute)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[repr(u8)]
pub enum Irig106Version {
    /// IRIG 106-04 (no CSDW structure)
    V106_04 = 4,
    /// IRIG 106-05
    V106_05 = 5,
    /// IRIG 106-07 (introduced CSDW for setup records)
    V106_07 = 7,
    /// IRIG 106-09
    V106_09 = 9,
    /// IRIG 106-11
    V106_11 = 11,
    /// IRIG 106-13 (removed PAM attributes)
    V106_13 = 13,
    /// IRIG 106-15
    V106_15 = 15,
    /// IRIG 106-17 (Chapter 10/11 split)
    V106_17 = 17,
}

impl Irig106Version {
    /// Parse from the `G\106` attribute string value.
    ///
    /// Accepts formats like "07", "106-07", "IRIG 106-07", etc.
    pub fn from_g106_str(s: &str) -> Option<Self> {
        let s = s.trim();
        // Try to extract the two-digit version number
        let digits: String = s.chars().rev().take_while(|c| c.is_ascii_digit()).collect::<String>()
            .chars().rev().collect();
        match digits.as_str() {
            "04" => Some(Self::V106_04),
            "05" => Some(Self::V106_05),
            "07" => Some(Self::V106_07),
            "09" => Some(Self::V106_09),
            "11" => Some(Self::V106_11),
            "13" => Some(Self::V106_13),
            "15" => Some(Self::V106_15),
            "17" => Some(Self::V106_17),
            _ => None,
        }
    }

    /// Map from the CSDW `iCh10Ver` field value.
    ///
    /// **Requirement:** L3-VERSION-009
    pub fn from_csdw_version(ver: u8) -> Option<Self> {
        match ver {
            0 => None, // Pre-106-07, field was undefined
            7 => Some(Self::V106_07),
            8 => Some(Self::V106_09),
            9 => Some(Self::V106_11),
            10 => Some(Self::V106_13),
            11 => Some(Self::V106_15),
            12 => Some(Self::V106_17),
            _ => None,
        }
    }

    /// Convert to the CSDW `iCh10Ver` encoding.
    pub fn to_csdw_version(self) -> u8 {
        match self {
            Self::V106_04 | Self::V106_05 => 0,
            Self::V106_07 => 7,
            Self::V106_09 => 8,
            Self::V106_11 => 9,
            Self::V106_13 => 10,
            Self::V106_15 => 11,
            Self::V106_17 => 12,
        }
    }

    /// Return the `G\106` attribute string representation.
    pub fn as_g106_str(&self) -> &'static str {
        match self {
            Self::V106_04 => "04",
            Self::V106_05 => "05",
            Self::V106_07 => "07",
            Self::V106_09 => "09",
            Self::V106_11 => "11",
            Self::V106_13 => "13",
            Self::V106_15 => "15",
            Self::V106_17 => "17",
        }
    }
}

impl fmt::Display for Irig106Version {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "IRIG 106-{}", self.as_g106_str())
    }
}

/// Chapter 10 data type codes.
///
/// Used to identify the type of data in a Ch10 packet and to map
/// channels to TMATS format groups during generation.
///
/// **Requirement:** L3-GEN-008
/// **Spec:** Ch10/Ch11 data type field definitions
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[repr(u8)]
pub enum DataTypeCode {
    ComputerGeneratedFormat0 = 0x00,
    ComputerGeneratedFormat1 = 0x01, // TMATS setup record
    ComputerGeneratedFormat2 = 0x02, // Recording events
    ComputerGeneratedFormat3 = 0x03, // Recording index
    Pcm = 0x09,
    TimeDataFormat1 = 0x11,
    Mil1553Format1 = 0x19,
    Mil1553Format2 = 0x1A,
    AnalogFormat1 = 0x21,
    DiscreteFormat1 = 0x29,
    MessageData = 0x30,
    Arinc429Format0 = 0x38,
    VideoFormat0 = 0x40,
    VideoFormat1 = 0x41,
    VideoFormat2 = 0x42,
    ImageFormat0 = 0x48,
    ImageFormat1 = 0x49,
    UartFormat0 = 0x50,
    Ieee1394Format0 = 0x58,
    ParallelFormat0 = 0x60,
    EthernetFormat0 = 0x68,
    CanBusFormat0 = 0x78,
    FibreChannelFormat0 = 0x79,
    /// Any unrecognized type code
    Other(u8),
}

impl DataTypeCode {
    pub fn from_u8(v: u8) -> Self {
        match v {
            0x00 => Self::ComputerGeneratedFormat0,
            0x01 => Self::ComputerGeneratedFormat1,
            0x02 => Self::ComputerGeneratedFormat2,
            0x03 => Self::ComputerGeneratedFormat3,
            0x09 => Self::Pcm,
            0x11 => Self::TimeDataFormat1,
            0x19 => Self::Mil1553Format1,
            0x1A => Self::Mil1553Format2,
            0x21 => Self::AnalogFormat1,
            0x29 => Self::DiscreteFormat1,
            0x30 => Self::MessageData,
            0x38 => Self::Arinc429Format0,
            0x40 => Self::VideoFormat0,
            0x41 => Self::VideoFormat1,
            0x42 => Self::VideoFormat2,
            0x48 => Self::ImageFormat0,
            0x49 => Self::ImageFormat1,
            0x50 => Self::UartFormat0,
            0x58 => Self::Ieee1394Format0,
            0x60 => Self::ParallelFormat0,
            0x68 => Self::EthernetFormat0,
            0x78 => Self::CanBusFormat0,
            0x79 => Self::FibreChannelFormat0,
            other => Self::Other(other),
        }
    }

    /// Map data type to the TMATS format group it belongs to.
    ///
    /// **Requirement:** L3-GEN-008
    pub fn tmats_group(&self) -> Option<GroupPrefix> {
        match self {
            Self::Pcm => Some(GroupPrefix::P),
            Self::Mil1553Format1 | Self::Mil1553Format2 | Self::Arinc429Format0 => {
                Some(GroupPrefix::B)
            }
            Self::MessageData | Self::UartFormat0 | Self::EthernetFormat0 => {
                Some(GroupPrefix::S)
            }
            _ => None,
        }
    }
}

/// Newtype wrapper for Chapter 10 channel IDs.
///
/// Channel IDs are u16 values matching the `R-x\TK1-n` TMATS track numbers.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct ChannelId(pub u16);

impl fmt::Display for ChannelId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Ch{}", self.0)
    }
}

/// TMATS attribute group prefix identifiers.
///
/// **Requirement:** L3-PARSE-006
/// **Spec:** Ch9 §9.5.1 Figure 9-1 (Group Relationships)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum GroupPrefix {
    /// General Information (§9.5.2)
    G,
    /// Transmission Attributes (§9.5.3)
    T,
    /// Recorder-Reproducer Attributes (§9.5.4)
    R,
    /// Multiplex/Modulation Attributes (§9.5.5)
    M,
    /// PCM Format Attributes (§9.5.6.1)
    P,
    /// PCM Measurement Description (§9.5.6.2)
    D,
    /// Bus Data Attributes (§9.5.6.3)
    B,
    /// Message Data Attributes (§9.5.7)
    S,
    /// Data Conversion Attributes (§9.5.8)
    C,
    /// Unknown/forward-compatible group
    Unknown(char),
}

impl GroupPrefix {
    /// Parse a group prefix from the first character of a code-name.
    pub fn from_char(c: char) -> Self {
        match c.to_ascii_uppercase() {
            'G' => Self::G,
            'T' => Self::T,
            'R' => Self::R,
            'M' => Self::M,
            'P' => Self::P,
            'D' => Self::D,
            'B' => Self::B,
            'S' => Self::S,
            'C' => Self::C,
            other => Self::Unknown(other),
        }
    }

    /// Convert to the canonical character representation.
    pub fn as_char(&self) -> char {
        match self {
            Self::G => 'G',
            Self::T => 'T',
            Self::R => 'R',
            Self::M => 'M',
            Self::P => 'P',
            Self::D => 'D',
            Self::B => 'B',
            Self::S => 'S',
            Self::C => 'C',
            Self::Unknown(c) => *c,
        }
    }
}

impl fmt::Display for GroupPrefix {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.as_char())
    }
}
