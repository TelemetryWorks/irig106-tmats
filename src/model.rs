// irig106-tmats/src/model.rs
//
// # TMATS Domain Model
//
// Strongly-typed Rust representation of all TMATS attribute groups and their
// hierarchical relationships per Ch9 §9.5.1 (Figure 9-1).
//
// ## Design Decisions:
//   - All string fields use `Cow<'a, str>` for zero-copy borrowed or owned modes (L3-PERF-001)
//   - Repeated groups use `IndexMap<u32, T>` keyed by occurrence index (L3-MODEL-013)
//   - Every group has an `extra` field for unrecognized attributes (L3-MODEL-020)
//   - All fields are `Option<T>` because TMATS files may be partial/incomplete
//
// ## Traceability:
//   L1-MODEL → L2-MODEL-001..014 → L3-MODEL-001..020

use std::borrow::Cow;
use indexmap::IndexMap;

use crate::error::Diagnostic;
use crate::types_bridge::{GroupPrefix, Irig106Version};

// ─── Raw Attribute (L3-PARSE-014) ────────────────────────────────────────────

/// A parsed but un-structured TMATS attribute.
///
/// Produced by Phase 1 (tokenization) and consumed by Phase 2 (structuring).
/// Also used to preserve unrecognized attributes for lossless round-tripping.
///
/// **Requirement:** L3-PARSE-014, L2-MODEL-012
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct RawAttribute<'a> {
    /// Parsed code-name components.
    pub code_name: CodeName<'a>,
    /// Original unparsed code-name string.
    pub raw_code_name: Cow<'a, str>,
    /// Attribute value.
    pub value: Cow<'a, str>,
    /// Byte offset in source input.
    pub byte_offset: usize,
    /// 1-based line number.
    pub line: u32,
}

// ─── Code Name (L3-PARSE-005) ────────────────────────────────────────────────

/// Decomposed TMATS code-name structure.
///
/// Represents e.g. `R-1\TK1-3` as:
///   group = R, occurrence = Some(1), path = [PathSegment("TK1", Some(3))]
///
/// **Requirement:** L3-PARSE-005
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct CodeName<'a> {
    /// Group prefix letter (G, T, R, M, P, D, B, S, C).
    pub group: GroupPrefix,
    /// Top-level occurrence index (e.g., `R-1` → `Some(1)`).
    pub occurrence: Option<u32>,
    /// Hierarchical path segments after the group prefix.
    pub path: smallvec::SmallVec<[PathSegment<'a>; 4]>,
}

/// A single segment in a code-name path.
///
/// E.g., in `R-1\TK1-3`, the segment is `PathSegment { name: "TK1", index: Some(3) }`.
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct PathSegment<'a> {
    /// Attribute name within this segment.
    pub name: Cow<'a, str>,
    /// Optional occurrence index (the `-N` suffix).
    pub index: Option<u32>,
}

// ─── Attribute Value (L3-MODEL-002) ──────────────────────────────────────────

/// Typed representation of a TMATS attribute value.
///
/// **Requirement:** L3-MODEL-002
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum AttrValue<'a> {
    /// A keyword from a defined enumeration (e.g., "LO", "RO").
    Keyword(Cow<'a, str>),
    /// Free-form text.
    Text(Cow<'a, str>),
    /// Integer value.
    Integer(i64),
    /// Decimal value (non-scientific).
    Decimal(f64),
    /// Scientific notation value.
    Scientific(f64),
    /// Date in MM-DD-YYYY format.
    Date(TmatsDate),
    /// Boolean (TMATS "T"/"F").
    Boolean(bool),
    /// Empty/absent value.
    Empty,
}

// ─── TMATS Date (L3-MODEL-003) ───────────────────────────────────────────────

/// Date as represented in TMATS (MM-DD-YYYY).
///
/// **Requirement:** L3-MODEL-003
/// **Spec:** Ch9 §9.5.2 Table 9-1 (G\OD, G\RD, G\UD attributes)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct TmatsDate {
    pub month: u8,
    pub day: u8,
    pub year: u16,
}

impl TmatsDate {
    /// Parse from TMATS ASCII format "MM-DD-YYYY".
    pub fn from_tmats_str(s: &str) -> Option<Self> {
        let parts: Vec<&str> = s.split('-').collect();
        if parts.len() != 3 {
            return None;
        }
        Some(Self {
            month: parts[0].parse().ok()?,
            day: parts[1].parse().ok()?,
            year: parts[2].parse().ok()?,
        })
    }

    /// Parse from XML standard format "YYYY-MM-DD".
    pub fn from_xml_str(s: &str) -> Option<Self> {
        let parts: Vec<&str> = s.split('-').collect();
        if parts.len() != 3 {
            return None;
        }
        Some(Self {
            year: parts[0].parse().ok()?,
            month: parts[1].parse().ok()?,
            day: parts[2].parse().ok()?,
        })
    }
}

impl core::fmt::Display for TmatsDate {
    /// Emits TMATS ASCII format "MM-DD-YYYY".
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "{:02}-{:02}-{:04}", self.month, self.day, self.year)
    }
}

// ─── Data Link Name Newtype (L3-MODEL-014) ───────────────────────────────────

/// Typed cross-group reference key.
///
/// Used to link R-group channels to P/B/S/D/C format groups.
///
/// **Requirement:** L3-MODEL-014
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct DataLinkName<'a>(pub Cow<'a, str>);

impl<'a> DataLinkName<'a> {
    pub fn as_str(&self) -> &str {
        &self.0
    }

    pub fn to_owned_link(&self) -> DataLinkName<'static> {
        DataLinkName(Cow::Owned(self.0.to_string()))
    }
}

// ─── TmatsDocument Root (L3-MODEL-001) ───────────────────────────────────────

/// Root container for a parsed TMATS record.
///
/// **Requirement:** L3-MODEL-001
/// **Spec:** Ch9 §9.5.1 — all groups assembled per Figure 9-1
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct TmatsDocument<'a> {
    /// General Information group (G). Always exactly one.
    /// **Requirement:** L2-MODEL-001
    pub general: GGroup<'a>,

    /// Transmission Attributes groups (T), keyed by occurrence index.
    /// **Requirement:** L2-MODEL-002
    pub transmission: IndexMap<u32, TGroup<'a>>,

    /// Recorder-Reproducer Attributes groups (R), keyed by occurrence index.
    /// **Requirement:** L2-MODEL-003
    pub recorders: IndexMap<u32, RGroup<'a>>,

    /// Multiplex/Modulation Attributes groups (M), keyed by occurrence index.
    /// **Requirement:** L2-MODEL-004
    pub multiplex: IndexMap<u32, MGroup<'a>>,

    /// PCM Format Attributes groups (P), keyed by occurrence index.
    /// **Requirement:** L2-MODEL-005
    pub pcm_formats: IndexMap<u32, PGroup<'a>>,

    /// PCM Measurement Description groups (D), keyed by occurrence index.
    /// **Requirement:** L2-MODEL-006
    pub pcm_measurements: IndexMap<u32, DGroup<'a>>,

    /// Bus Data Attributes groups (B), keyed by occurrence index.
    /// **Requirement:** L2-MODEL-007
    pub bus_data: IndexMap<u32, BGroup<'a>>,

    /// Message Data Attributes groups (S), keyed by occurrence index.
    /// **Requirement:** L2-MODEL-008
    pub message_data: IndexMap<u32, SGroup<'a>>,

    /// Data Conversion Attributes groups (C), keyed by occurrence index.
    /// **Requirement:** L2-MODEL-009
    pub data_conversion: IndexMap<u32, CGroup<'a>>,

    /// Attributes that could not be mapped to any known group.
    /// **Requirement:** L2-MODEL-012
    pub unknown: Vec<RawAttribute<'a>>,

    /// Detected IRIG 106 version.
    /// **Requirement:** L2-VERSION-001
    pub source_version: Option<Irig106Version>,

    /// Diagnostics collected during parsing (lenient mode).
    pub parse_diagnostics: Vec<Diagnostic>,
}

impl<'a> TmatsDocument<'a> {
    /// Create an empty document.
    pub fn empty() -> Self {
        Self {
            general: GGroup::default(),
            transmission: IndexMap::new(),
            recorders: IndexMap::new(),
            multiplex: IndexMap::new(),
            pcm_formats: IndexMap::new(),
            pcm_measurements: IndexMap::new(),
            bus_data: IndexMap::new(),
            message_data: IndexMap::new(),
            data_conversion: IndexMap::new(),
            unknown: Vec::new(),
            source_version: None,
            parse_diagnostics: Vec::new(),
        }
    }

    /// Convert all borrowed data to owned, producing a `'static` document.
    ///
    /// **Requirement:** L3-MODEL-015, L3-MODEL-016, L3-PERF-002
    pub fn into_owned(self) -> TmatsDocument<'static> {
        TmatsDocument {
            general: self.general.into_owned(),
            transmission: self.transmission.into_iter()
                .map(|(k, v)| (k, v.into_owned())).collect(),
            recorders: self.recorders.into_iter()
                .map(|(k, v)| (k, v.into_owned())).collect(),
            multiplex: self.multiplex.into_iter()
                .map(|(k, v)| (k, v.into_owned())).collect(),
            pcm_formats: self.pcm_formats.into_iter()
                .map(|(k, v)| (k, v.into_owned())).collect(),
            pcm_measurements: self.pcm_measurements.into_iter()
                .map(|(k, v)| (k, v.into_owned())).collect(),
            bus_data: self.bus_data.into_iter()
                .map(|(k, v)| (k, v.into_owned())).collect(),
            message_data: self.message_data.into_iter()
                .map(|(k, v)| (k, v.into_owned())).collect(),
            data_conversion: self.data_conversion.into_iter()
                .map(|(k, v)| (k, v.into_owned())).collect(),
            unknown: self.unknown.into_iter()
                .map(|a| a.into_owned()).collect(),
            source_version: self.source_version,
            parse_diagnostics: self.parse_diagnostics,
        }
    }

    /// Total number of structured attributes across all groups.
    pub fn attribute_count(&self) -> usize {
        // Rough estimate; a full count would walk all fields
        self.recorders.len()
            + self.transmission.len()
            + self.pcm_formats.len()
            + self.pcm_measurements.len()
            + self.bus_data.len()
            + self.message_data.len()
            + self.data_conversion.len()
            + self.multiplex.len()
            + self.unknown.len()
            + 1 // G-group
    }
}

/// **Requirement:** L3-MODEL-015
pub type OwnedTmatsDocument = TmatsDocument<'static>;

// ─── G-Group: General Information (L3-MODEL-004, L3-MODEL-005) ───────────────

/// General Information group (G).
///
/// **Requirement:** L2-MODEL-001, L3-MODEL-004
/// **Spec:** Ch9 §9.5.2, Table 9-1
#[derive(Debug, Clone, Default)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct GGroup<'a> {
    /// G\PN — Program Name
    pub program_name: Option<Cow<'a, str>>,
    /// G\106 — IRIG 106 Standard Version
    pub irig106_version: Option<Cow<'a, str>>,
    /// G\OD — Origination Date (MM-DD-YYYY)
    pub origination_date: Option<TmatsDate>,
    /// G\RN — Revision Number
    pub revision_number: Option<Cow<'a, str>>,
    /// G\RD — Revision Date
    pub revision_date: Option<TmatsDate>,
    /// G\UN — Update Number
    pub update_number: Option<Cow<'a, str>>,
    /// G\UD — Update Date
    pub update_date: Option<TmatsDate>,
    /// G\TN — Test Number
    pub test_number: Option<Cow<'a, str>>,
    /// G\DSI\N — Number of Data Sources (counter attribute, L2-MODEL-010)
    pub num_data_sources: Option<u32>,
    /// Data source declarations, keyed by occurrence index.
    /// **Requirement:** L3-MODEL-005
    pub data_sources: IndexMap<u32, DataSourceDecl<'a>>,
    /// G\POC\N — Number of Points of Contact
    pub num_points_of_contact: Option<u32>,
    /// Points of contact, keyed by occurrence index.
    pub points_of_contact: IndexMap<u32, PointOfContact<'a>>,
    /// G\COM — Comments
    pub comments: Vec<Cow<'a, str>>,
    /// Unrecognized G-group attributes (L3-MODEL-020)
    pub extra: Vec<RawAttribute<'a>>,
}

impl<'a> GGroup<'a> {
    pub fn into_owned(self) -> GGroup<'static> {
        GGroup {
            program_name: self.program_name.map(|c| Cow::Owned(c.into_owned())),
            irig106_version: self.irig106_version.map(|c| Cow::Owned(c.into_owned())),
            origination_date: self.origination_date,
            revision_number: self.revision_number.map(|c| Cow::Owned(c.into_owned())),
            revision_date: self.revision_date,
            update_number: self.update_number.map(|c| Cow::Owned(c.into_owned())),
            update_date: self.update_date,
            test_number: self.test_number.map(|c| Cow::Owned(c.into_owned())),
            num_data_sources: self.num_data_sources,
            data_sources: self.data_sources.into_iter()
                .map(|(k, v)| (k, v.into_owned())).collect(),
            num_points_of_contact: self.num_points_of_contact,
            points_of_contact: self.points_of_contact.into_iter()
                .map(|(k, v)| (k, v.into_owned())).collect(),
            comments: self.comments.into_iter()
                .map(|c| Cow::Owned(c.into_owned())).collect(),
            extra: self.extra.into_iter().map(|a| a.into_owned()).collect(),
        }
    }
}

/// Data source declaration from the G-group.
///
/// **Requirement:** L3-MODEL-005
/// **Spec:** Ch9 §9.5.2, G\DSI-n, G\DST-n, G\DSC-n
#[derive(Debug, Clone, Default)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct DataSourceDecl<'a> {
    /// G\DSI-n — Data Source Identifier
    pub data_source_id: Option<Cow<'a, str>>,
    /// G\DST-n — Data Source Type
    pub data_source_type: Option<Cow<'a, str>>,
    /// G\DSC-n — Data Source Classification
    pub classification: Option<Cow<'a, str>>,
    /// Unrecognized attributes in this data source scope.
    pub extra: Vec<RawAttribute<'a>>,
}

impl<'a> DataSourceDecl<'a> {
    pub fn into_owned(self) -> DataSourceDecl<'static> {
        DataSourceDecl {
            data_source_id: self.data_source_id.map(|c| Cow::Owned(c.into_owned())),
            data_source_type: self.data_source_type.map(|c| Cow::Owned(c.into_owned())),
            classification: self.classification.map(|c| Cow::Owned(c.into_owned())),
            extra: self.extra.into_iter().map(|a| a.into_owned()).collect(),
        }
    }
}

/// Point of contact from the G-group.
#[derive(Debug, Clone, Default)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct PointOfContact<'a> {
    pub name: Option<Cow<'a, str>>,
    pub agency: Option<Cow<'a, str>>,
    pub address: Option<Cow<'a, str>>,
    pub telephone: Option<Cow<'a, str>>,
    pub extra: Vec<RawAttribute<'a>>,
}

impl<'a> PointOfContact<'a> {
    pub fn into_owned(self) -> PointOfContact<'static> {
        PointOfContact {
            name: self.name.map(|c| Cow::Owned(c.into_owned())),
            agency: self.agency.map(|c| Cow::Owned(c.into_owned())),
            address: self.address.map(|c| Cow::Owned(c.into_owned())),
            telephone: self.telephone.map(|c| Cow::Owned(c.into_owned())),
            extra: self.extra.into_iter().map(|a| a.into_owned()).collect(),
        }
    }
}

// ─── T-Group: Transmission Attributes (L3-MODEL-017) ─────────────────────────

/// Transmission Attributes group (T).
///
/// **Requirement:** L2-MODEL-002, L3-MODEL-017
/// **Spec:** Ch9 §9.5.3
#[derive(Debug, Clone, Default)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct TGroup<'a> {
    /// T-n\ID — Transmitter Identification
    pub transmitter_id: Option<Cow<'a, str>>,
    /// T-n\CF — Carrier Frequency (MHz)
    pub carrier_frequency_mhz: Option<f64>,
    /// T-n\MT — Modulation Type
    pub modulation_type: Option<Cow<'a, str>>,
    /// T-n\PW — Total Transmitted Power (Watts)
    pub power_watts: Option<f64>,
    /// T-n\AT — Transmit Antenna Type
    pub antenna_type: Option<Cow<'a, str>>,
    /// T-n\AP — Transmit Antenna Polarization
    pub antenna_polarization: Option<Cow<'a, str>>,
    /// T-n\COM — Comments
    pub comments: Vec<Cow<'a, str>>,
    pub extra: Vec<RawAttribute<'a>>,
}

impl<'a> TGroup<'a> {
    pub fn into_owned(self) -> TGroup<'static> {
        TGroup {
            transmitter_id: self.transmitter_id.map(|c| Cow::Owned(c.into_owned())),
            carrier_frequency_mhz: self.carrier_frequency_mhz,
            modulation_type: self.modulation_type.map(|c| Cow::Owned(c.into_owned())),
            power_watts: self.power_watts,
            antenna_type: self.antenna_type.map(|c| Cow::Owned(c.into_owned())),
            antenna_polarization: self.antenna_polarization.map(|c| Cow::Owned(c.into_owned())),
            comments: self.comments.into_iter().map(|c| Cow::Owned(c.into_owned())).collect(),
            extra: self.extra.into_iter().map(|a| a.into_owned()).collect(),
        }
    }
}

// ─── R-Group: Recorder-Reproducer Attributes (L3-MODEL-006, L3-MODEL-007) ───

/// Recorder-Reproducer Attributes group (R).
///
/// **Requirement:** L2-MODEL-003, L3-MODEL-006
/// **Spec:** Ch9 §9.5.4
#[derive(Debug, Clone, Default)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct RGroup<'a> {
    /// R-x\ID — Recorder Identification
    pub recorder_id: Option<Cow<'a, str>>,
    /// R-x\RI1 — Recorder Description
    pub recorder_description: Option<Cow<'a, str>>,
    /// R-x\RMT — Recorder Media Type
    pub media_type: Option<Cow<'a, str>>,
    /// R-x\N — Number of channels/tracks
    pub num_channels: Option<u32>,
    /// Channel definitions, keyed by track index.
    /// **Requirement:** L3-MODEL-007
    pub channels: IndexMap<u32, RChannel<'a>>,
    /// Drive definitions.
    pub drives: IndexMap<u32, RDrive<'a>>,
    /// R-x\IDX\E — Index enabled ("T"/"F")
    pub index_enabled: Option<bool>,
    /// R-x\EV\E — Events enabled
    pub events_enabled: Option<bool>,
    /// R-x\COM — Comments
    pub comments: Vec<Cow<'a, str>>,
    pub extra: Vec<RawAttribute<'a>>,
}

impl<'a> RGroup<'a> {
    pub fn into_owned(self) -> RGroup<'static> {
        RGroup {
            recorder_id: self.recorder_id.map(|c| Cow::Owned(c.into_owned())),
            recorder_description: self.recorder_description.map(|c| Cow::Owned(c.into_owned())),
            media_type: self.media_type.map(|c| Cow::Owned(c.into_owned())),
            num_channels: self.num_channels,
            channels: self.channels.into_iter()
                .map(|(k, v)| (k, v.into_owned())).collect(),
            drives: self.drives.into_iter()
                .map(|(k, v)| (k, v.into_owned())).collect(),
            index_enabled: self.index_enabled,
            events_enabled: self.events_enabled,
            comments: self.comments.into_iter().map(|c| Cow::Owned(c.into_owned())).collect(),
            extra: self.extra.into_iter().map(|a| a.into_owned()).collect(),
        }
    }
}

/// A single channel/track within an R-group.
///
/// **Requirement:** L3-MODEL-007
#[derive(Debug, Clone, Default)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct RChannel<'a> {
    /// R-x\TK1-n — Track/Channel Number
    pub channel_id: Option<u16>,
    /// R-x\CDT-n — Channel Data Type
    pub data_type: Option<Cow<'a, str>>,
    /// R-x\DSI-n — Data Source ID (link to G-group data source)
    pub data_source_id: Option<Cow<'a, str>>,
    /// R-x\CDLN-n — Channel Data Link Name (cross-group link to P/B/S)
    pub data_link_name: Option<Cow<'a, str>>,
    /// R-x\PDP-n — Data Packing Option (UN/PFS/TM)
    pub data_packing_option: Option<Cow<'a, str>>,
    /// R-x\CHE-n — Channel Enabled
    pub channel_enabled: Option<bool>,
    pub extra: Vec<RawAttribute<'a>>,
}

impl<'a> RChannel<'a> {
    pub fn into_owned(self) -> RChannel<'static> {
        RChannel {
            channel_id: self.channel_id,
            data_type: self.data_type.map(|c| Cow::Owned(c.into_owned())),
            data_source_id: self.data_source_id.map(|c| Cow::Owned(c.into_owned())),
            data_link_name: self.data_link_name.map(|c| Cow::Owned(c.into_owned())),
            data_packing_option: self.data_packing_option.map(|c| Cow::Owned(c.into_owned())),
            channel_enabled: self.channel_enabled,
            extra: self.extra.into_iter().map(|a| a.into_owned()).collect(),
        }
    }
}

/// Drive definition within an R-group.
#[derive(Debug, Clone, Default)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct RDrive<'a> {
    /// R-x\DRN-n — Drive Number
    pub drive_number: Option<u32>,
    /// R-x\DRN-n drive name
    pub drive_name: Option<Cow<'a, str>>,
    pub extra: Vec<RawAttribute<'a>>,
}

impl<'a> RDrive<'a> {
    pub fn into_owned(self) -> RDrive<'static> {
        RDrive {
            drive_number: self.drive_number,
            drive_name: self.drive_name.map(|c| Cow::Owned(c.into_owned())),
            extra: self.extra.into_iter().map(|a| a.into_owned()).collect(),
        }
    }
}

// ─── M-Group: Multiplex/Modulation Attributes (L3-MODEL-018) ─────────────────

/// Multiplex/Modulation Attributes group (M).
///
/// **Requirement:** L2-MODEL-004, L3-MODEL-018
/// **Spec:** Ch9 §9.5.5
#[derive(Debug, Clone, Default)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct MGroup<'a> {
    /// M-n\BSG — Baseband Signal Type
    pub baseband_signal_type: Option<Cow<'a, str>>,
    /// M-n\MS — Modulation Sense
    pub modulation_sense: Option<Cow<'a, str>>,
    /// Subcarrier definitions.
    pub subcarriers: IndexMap<u32, MSubcarrier<'a>>,
    pub comments: Vec<Cow<'a, str>>,
    pub extra: Vec<RawAttribute<'a>>,
}

impl<'a> MGroup<'a> {
    pub fn into_owned(self) -> MGroup<'static> {
        MGroup {
            baseband_signal_type: self.baseband_signal_type.map(|c| Cow::Owned(c.into_owned())),
            modulation_sense: self.modulation_sense.map(|c| Cow::Owned(c.into_owned())),
            subcarriers: self.subcarriers.into_iter()
                .map(|(k, v)| (k, v.into_owned())).collect(),
            comments: self.comments.into_iter().map(|c| Cow::Owned(c.into_owned())).collect(),
            extra: self.extra.into_iter().map(|a| a.into_owned()).collect(),
        }
    }
}

#[derive(Debug, Clone, Default)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct MSubcarrier<'a> {
    pub frequency_hz: Option<f64>,
    pub extra: Vec<RawAttribute<'a>>,
}

impl<'a> MSubcarrier<'a> {
    pub fn into_owned(self) -> MSubcarrier<'static> {
        MSubcarrier {
            frequency_hz: self.frequency_hz,
            extra: self.extra.into_iter().map(|a| a.into_owned()).collect(),
        }
    }
}

// ─── P-Group: PCM Format Attributes (L3-MODEL-008) ──────────────────────────

/// PCM Format Attributes group (P).
///
/// **Requirement:** L2-MODEL-005, L3-MODEL-008
/// **Spec:** Ch9 §9.5.6.1
#[derive(Debug, Clone, Default)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct PGroup<'a> {
    /// P-n\DLN — Data Link Name
    pub data_link_name: Option<Cow<'a, str>>,
    /// P-n\D1 — Bit Rate (bps)
    pub bit_rate: Option<f64>,
    /// P-n\D2 — Encoding
    pub encoding: Option<Cow<'a, str>>,
    /// P-n\D3 — Polarity
    pub polarity: Option<Cow<'a, str>>,
    /// P-n\D4 — Auto-Polarity Correction
    pub auto_polarity_correction: Option<bool>,
    /// P-n\MF\N — Number of Minor Frames (per Major Frame)
    pub num_minor_frames: Option<u32>,
    /// P-n\F1 — Words per Minor Frame
    pub num_words_per_frame: Option<u32>,
    /// P-n\F2 — Bits per Word (common word length)
    pub num_bits_per_word: Option<u32>,
    /// P-n\F3 — Sync Pattern
    pub sync_pattern: Option<Cow<'a, str>>,
    /// P-n\F3\L — Sync Pattern Length (bits)
    pub sync_pattern_length: Option<u32>,
    /// Subframe definitions.
    pub subframes: IndexMap<u32, PSubframe<'a>>,
    /// Word position definitions.
    pub word_definitions: IndexMap<u32, PWordDef<'a>>,
    /// Embedded format definitions.
    pub embedded_formats: IndexMap<u32, PEmbeddedFormat<'a>>,
    pub comments: Vec<Cow<'a, str>>,
    pub extra: Vec<RawAttribute<'a>>,
}

impl<'a> PGroup<'a> {
    pub fn into_owned(self) -> PGroup<'static> {
        PGroup {
            data_link_name: self.data_link_name.map(|c| Cow::Owned(c.into_owned())),
            bit_rate: self.bit_rate,
            encoding: self.encoding.map(|c| Cow::Owned(c.into_owned())),
            polarity: self.polarity.map(|c| Cow::Owned(c.into_owned())),
            auto_polarity_correction: self.auto_polarity_correction,
            num_minor_frames: self.num_minor_frames,
            num_words_per_frame: self.num_words_per_frame,
            num_bits_per_word: self.num_bits_per_word,
            sync_pattern: self.sync_pattern.map(|c| Cow::Owned(c.into_owned())),
            sync_pattern_length: self.sync_pattern_length,
            subframes: self.subframes.into_iter()
                .map(|(k, v)| (k, v.into_owned())).collect(),
            word_definitions: self.word_definitions.into_iter()
                .map(|(k, v)| (k, v.into_owned())).collect(),
            embedded_formats: self.embedded_formats.into_iter()
                .map(|(k, v)| (k, v.into_owned())).collect(),
            comments: self.comments.into_iter().map(|c| Cow::Owned(c.into_owned())).collect(),
            extra: self.extra.into_iter().map(|a| a.into_owned()).collect(),
        }
    }
}

#[derive(Debug, Clone, Default)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct PSubframe<'a> { pub extra: Vec<RawAttribute<'a>> }

impl<'a> PSubframe<'a> {
    pub fn into_owned(self) -> PSubframe<'static> {
        PSubframe { extra: self.extra.into_iter().map(|a| a.into_owned()).collect() }
    }
}

#[derive(Debug, Clone, Default)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct PWordDef<'a> { pub extra: Vec<RawAttribute<'a>> }

impl<'a> PWordDef<'a> {
    pub fn into_owned(self) -> PWordDef<'static> {
        PWordDef { extra: self.extra.into_iter().map(|a| a.into_owned()).collect() }
    }
}

#[derive(Debug, Clone, Default)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct PEmbeddedFormat<'a> { pub extra: Vec<RawAttribute<'a>> }

impl<'a> PEmbeddedFormat<'a> {
    pub fn into_owned(self) -> PEmbeddedFormat<'static> {
        PEmbeddedFormat { extra: self.extra.into_iter().map(|a| a.into_owned()).collect() }
    }
}

// ─── D-Group: PCM Measurement Description (L3-MODEL-009) ────────────────────

/// PCM Measurement Description group (D).
///
/// **Requirement:** L2-MODEL-006, L3-MODEL-009
/// **Spec:** Ch9 §9.5.6.2
#[derive(Debug, Clone, Default)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct DGroup<'a> {
    /// D-n\MLN — Measurement List Name (link to P-group)
    pub measurement_list_name: Option<Cow<'a, str>>,
    /// D-n\DLN — Data Link Name
    pub data_link_name: Option<Cow<'a, str>>,
    /// D-n\MN\N — Number of Measurements
    pub num_measurements: Option<u32>,
    /// Individual measurement definitions.
    pub measurements: IndexMap<u32, DMeasurement<'a>>,
    pub comments: Vec<Cow<'a, str>>,
    pub extra: Vec<RawAttribute<'a>>,
}

impl<'a> DGroup<'a> {
    pub fn into_owned(self) -> DGroup<'static> {
        DGroup {
            measurement_list_name: self.measurement_list_name.map(|c| Cow::Owned(c.into_owned())),
            data_link_name: self.data_link_name.map(|c| Cow::Owned(c.into_owned())),
            num_measurements: self.num_measurements,
            measurements: self.measurements.into_iter()
                .map(|(k, v)| (k, v.into_owned())).collect(),
            comments: self.comments.into_iter().map(|c| Cow::Owned(c.into_owned())).collect(),
            extra: self.extra.into_iter().map(|a| a.into_owned()).collect(),
        }
    }
}

#[derive(Debug, Clone, Default)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct DMeasurement<'a> {
    /// D-n\MN-m — Measurement Name
    pub measurement_name: Option<Cow<'a, str>>,
    /// D-n\MN-m\WP — Word Position
    pub word_position: Option<u32>,
    /// D-n\MN-m\WL — Word Length (bits)
    pub word_length: Option<u32>,
    /// D-n\MN-m\BM — Bit Mask
    pub bit_mask: Option<Cow<'a, str>>,
    /// D-n\MN-m\TO — Transfer Order (M/L/D)
    pub transfer_order: Option<Cow<'a, str>>,
    pub extra: Vec<RawAttribute<'a>>,
}

impl<'a> DMeasurement<'a> {
    pub fn into_owned(self) -> DMeasurement<'static> {
        DMeasurement {
            measurement_name: self.measurement_name.map(|c| Cow::Owned(c.into_owned())),
            word_position: self.word_position,
            word_length: self.word_length,
            bit_mask: self.bit_mask.map(|c| Cow::Owned(c.into_owned())),
            transfer_order: self.transfer_order.map(|c| Cow::Owned(c.into_owned())),
            extra: self.extra.into_iter().map(|a| a.into_owned()).collect(),
        }
    }
}

// ─── B-Group: Bus Data Attributes (L3-MODEL-010) ────────────────────────────

/// Bus Data Attributes group (B).
///
/// **Requirement:** L2-MODEL-007, L3-MODEL-010
/// **Spec:** Ch9 §9.5.6.3
#[derive(Debug, Clone, Default)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct BGroup<'a> {
    /// B-n\DLN — Data Link Name
    pub data_link_name: Option<Cow<'a, str>>,
    /// B-n\BT — Bus Type (1553, ARINC429, etc.)
    pub bus_type: Option<Cow<'a, str>>,
    /// B-n\NBS\N — Number of Buses
    pub num_buses: Option<u32>,
    /// Message definitions.
    pub messages: IndexMap<u32, BMessage<'a>>,
    pub comments: Vec<Cow<'a, str>>,
    pub extra: Vec<RawAttribute<'a>>,
}

impl<'a> BGroup<'a> {
    pub fn into_owned(self) -> BGroup<'static> {
        BGroup {
            data_link_name: self.data_link_name.map(|c| Cow::Owned(c.into_owned())),
            bus_type: self.bus_type.map(|c| Cow::Owned(c.into_owned())),
            num_buses: self.num_buses,
            messages: self.messages.into_iter()
                .map(|(k, v)| (k, v.into_owned())).collect(),
            comments: self.comments.into_iter().map(|c| Cow::Owned(c.into_owned())).collect(),
            extra: self.extra.into_iter().map(|a| a.into_owned()).collect(),
        }
    }
}

#[derive(Debug, Clone, Default)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct BMessage<'a> {
    pub message_name: Option<Cow<'a, str>>,
    pub extra: Vec<RawAttribute<'a>>,
}

impl<'a> BMessage<'a> {
    pub fn into_owned(self) -> BMessage<'static> {
        BMessage {
            message_name: self.message_name.map(|c| Cow::Owned(c.into_owned())),
            extra: self.extra.into_iter().map(|a| a.into_owned()).collect(),
        }
    }
}

// ─── S-Group: Message Data Attributes (L3-MODEL-019) ─────────────────────────

/// Message Data Attributes group (S).
///
/// **Requirement:** L2-MODEL-008, L3-MODEL-019
/// **Spec:** Ch9 §9.5.7
#[derive(Debug, Clone, Default)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct SGroup<'a> {
    /// S-n\DLN — Data Link Name
    pub data_link_name: Option<Cow<'a, str>>,
    /// Message definitions.
    pub message_definitions: IndexMap<u32, SMessageDef<'a>>,
    pub comments: Vec<Cow<'a, str>>,
    pub extra: Vec<RawAttribute<'a>>,
}

impl<'a> SGroup<'a> {
    pub fn into_owned(self) -> SGroup<'static> {
        SGroup {
            data_link_name: self.data_link_name.map(|c| Cow::Owned(c.into_owned())),
            message_definitions: self.message_definitions.into_iter()
                .map(|(k, v)| (k, v.into_owned())).collect(),
            comments: self.comments.into_iter().map(|c| Cow::Owned(c.into_owned())).collect(),
            extra: self.extra.into_iter().map(|a| a.into_owned()).collect(),
        }
    }
}

#[derive(Debug, Clone, Default)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct SMessageDef<'a> {
    pub extra: Vec<RawAttribute<'a>>,
}

impl<'a> SMessageDef<'a> {
    pub fn into_owned(self) -> SMessageDef<'static> {
        SMessageDef { extra: self.extra.into_iter().map(|a| a.into_owned()).collect() }
    }
}

// ─── C-Group: Data Conversion Attributes (L3-MODEL-011, L3-MODEL-012) ───────

/// Data Conversion Attributes group (C).
///
/// **Requirement:** L2-MODEL-009, L3-MODEL-011
/// **Spec:** Ch9 §9.5.8
#[derive(Debug, Clone, Default)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct CGroup<'a> {
    /// C-n\DCN — Data Conversion Name / Measurement Name
    pub measurement_name: Option<Cow<'a, str>>,
    /// C-n\DCT — Data Conversion Type
    pub conversion_type: Option<Cow<'a, str>>,
    /// C-n\EU — Engineering Units label
    pub eu_units: Option<Cow<'a, str>>,
    /// Conversion pair sets (raw → EU).
    /// **Requirement:** L3-MODEL-012
    pub pair_sets: Vec<ConversionPairSet>,
    pub comments: Vec<Cow<'a, str>>,
    pub extra: Vec<RawAttribute<'a>>,
}

impl<'a> CGroup<'a> {
    pub fn into_owned(self) -> CGroup<'static> {
        CGroup {
            measurement_name: self.measurement_name.map(|c| Cow::Owned(c.into_owned())),
            conversion_type: self.conversion_type.map(|c| Cow::Owned(c.into_owned())),
            eu_units: self.eu_units.map(|c| Cow::Owned(c.into_owned())),
            pair_sets: self.pair_sets,
            comments: self.comments.into_iter().map(|c| Cow::Owned(c.into_owned())).collect(),
            extra: self.extra.into_iter().map(|a| a.into_owned()).collect(),
        }
    }
}

/// A single raw-to-EU conversion pair.
///
/// **Requirement:** L3-MODEL-012
#[derive(Debug, Clone, Copy, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct ConversionPairSet {
    pub raw_value: f64,
    pub eu_value: f64,
}

// ─── TmatsGroup Trait (L3-QUERY-005) ─────────────────────────────────────────

/// Trait for TMATS group types, enabling generic iteration.
///
/// **Requirement:** L3-QUERY-005
pub trait TmatsGroup {
    fn group_prefix() -> GroupPrefix;
}

impl<'a> TmatsGroup for GGroup<'a> { fn group_prefix() -> GroupPrefix { GroupPrefix::G } }
impl<'a> TmatsGroup for TGroup<'a> { fn group_prefix() -> GroupPrefix { GroupPrefix::T } }
impl<'a> TmatsGroup for RGroup<'a> { fn group_prefix() -> GroupPrefix { GroupPrefix::R } }
impl<'a> TmatsGroup for MGroup<'a> { fn group_prefix() -> GroupPrefix { GroupPrefix::M } }
impl<'a> TmatsGroup for PGroup<'a> { fn group_prefix() -> GroupPrefix { GroupPrefix::P } }
impl<'a> TmatsGroup for DGroup<'a> { fn group_prefix() -> GroupPrefix { GroupPrefix::D } }
impl<'a> TmatsGroup for BGroup<'a> { fn group_prefix() -> GroupPrefix { GroupPrefix::B } }
impl<'a> TmatsGroup for SGroup<'a> { fn group_prefix() -> GroupPrefix { GroupPrefix::S } }
impl<'a> TmatsGroup for CGroup<'a> { fn group_prefix() -> GroupPrefix { GroupPrefix::C } }

// ─── RawAttribute into_owned ─────────────────────────────────────────────────

impl<'a> RawAttribute<'a> {
    pub fn into_owned(self) -> RawAttribute<'static> {
        RawAttribute {
            code_name: self.code_name.into_owned_cn(),
            raw_code_name: Cow::Owned(self.raw_code_name.into_owned()),
            value: Cow::Owned(self.value.into_owned()),
            byte_offset: self.byte_offset,
            line: self.line,
        }
    }
}

impl<'a> CodeName<'a> {
    pub fn into_owned_cn(self) -> CodeName<'static> {
        CodeName {
            group: self.group,
            occurrence: self.occurrence,
            path: self.path.into_iter().map(|s| PathSegment {
                name: Cow::Owned(s.name.into_owned()),
                index: s.index,
            }).collect(),
        }
    }
}
