// irig106-tmats/src/version.rs
//
// # Version-Aware Attribute Registry
//
// Machine-readable encoding of the Ch9 §9.5 attribute tables across all
// supported IRIG 106 versions. Drives validation, keyword enforcement,
// MFS checking, and version migration guidance.
//
// ## Traceability:
//   L1-VERSION → L2-VERSION-001..005 → L3-VERSION-002..010

use crate::types_bridge::{GroupPrefix, Irig106Version};
use std::collections::HashMap;

// ─── Requirement Tag (L3-VERSION-003) ────────────────────────────────────────

/// Classifies how an attribute is required per the §9.5 tables.
///
/// **Requirement:** L3-VERSION-003
/// **Spec:** Ch9 §9.5 table column "R/O" markings
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum RequiredTag {
    /// Required in all contexts.
    Required,
    /// Required-Optional (context-dependent).
    RequiredOptional,
    /// Required for Chapter 10 setup records.
    RequiredCh10,
    /// Required-Optional for Chapter 10.
    RequiredOptionalCh10,
    /// Required when packing option is UN or PFS.
    RequiredPak,
    /// Optional.
    Optional,
}

impl core::str::FromStr for RequiredTag {
    type Err = core::convert::Infallible;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(match s.trim() {
            "R" => Self::Required,
            "RO" => Self::RequiredOptional,
            "R_CH10" => Self::RequiredCh10,
            "RO_CH10" => Self::RequiredOptionalCh10,
            "RO_PAK" => Self::RequiredPak,
            _ => Self::Optional,
        })
    }
}

impl RequiredTag {
    /// True if this attribute is mandatory in a Chapter 10 context.
    pub fn is_required_ch10(&self) -> bool {
        matches!(self, Self::Required | Self::RequiredCh10)
    }

    /// True if this attribute is mandatory in any context.
    pub fn is_required(&self) -> bool {
        matches!(self, Self::Required)
    }
}

// ─── Value Type (L3-VERSION-004) ─────────────────────────────────────────────

/// Expected type of an attribute value.
///
/// **Requirement:** L3-VERSION-004
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ValueType {
    Keyword,
    Text,
    Integer,
    Decimal,
    Scientific,
    Date,
    Boolean,
    FreeForm,
}

impl core::str::FromStr for ValueType {
    type Err = core::convert::Infallible;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(match s.trim() {
            "Keyword" => Self::Keyword,
            "Text" => Self::Text,
            "Integer" => Self::Integer,
            "Decimal" => Self::Decimal,
            "Scientific" => Self::Scientific,
            "Date" => Self::Date,
            "Boolean" => Self::Boolean,
            _ => Self::FreeForm,
        })
    }
}

impl ValueType {}

// ─── Attribute Metadata (L3-VERSION-002) ─────────────────────────────────────

/// Complete metadata record for one TMATS attribute, derived from §9.5 tables.
///
/// **Requirement:** L3-VERSION-002
#[derive(Debug, Clone)]
pub struct AttrMeta {
    /// Code-name pattern (e.g., "G\\PN", "R-x\\TK1-n").
    pub code_name_pattern: &'static str,
    /// Human-readable display name.
    pub display_name: &'static str,
    /// Group this attribute belongs to.
    pub group: GroupPrefix,
    /// Spec section reference.
    pub spec_section: &'static str,
    /// First IRIG 106 version containing this attribute.
    pub introduced_in: Irig106Version,
    /// Version where deprecated (still parseable with warning).
    pub deprecated_in: Option<Irig106Version>,
    /// Version where removed entirely.
    pub removed_in: Option<Irig106Version>,
    /// Requirement classification.
    pub required_tag: RequiredTag,
    /// Maximum field size in characters.
    pub max_field_size: Option<u16>,
    /// Expected value type.
    pub value_type: ValueType,
    /// Valid keyword values (empty if not a keyword type).
    pub keywords: &'static [&'static str],
    /// Brief description.
    pub description: &'static str,
}

impl AttrMeta {
    /// Check if this attribute is valid for a given version.
    pub fn is_valid_for(&self, version: Irig106Version) -> bool {
        if version < self.introduced_in {
            return false;
        }
        if let Some(removed) = self.removed_in {
            if version >= removed {
                return false;
            }
        }
        true
    }

    /// Check if this attribute is deprecated for a given version.
    pub fn is_deprecated_for(&self, version: Irig106Version) -> bool {
        if let Some(dep) = self.deprecated_in {
            version >= dep
        } else {
            false
        }
    }

    /// Validate a value against this attribute's keyword set.
    pub fn validate_keyword(&self, value: &str) -> bool {
        if self.keywords.is_empty() {
            return true; // No keyword constraint
        }
        let upper = value.trim().to_ascii_uppercase();
        self.keywords.iter().any(|k| k.eq_ignore_ascii_case(&upper))
    }

    /// Validate a value against MFS constraint.
    pub fn validate_mfs(&self, value: &str) -> bool {
        match self.max_field_size {
            Some(mfs) => value.len() <= mfs as usize,
            None => true,
        }
    }
}

// ─── Static Attribute Registry ───────────────────────────────────────────────
//
// This is the compiled form of data/attributes.toml.
//
// When `build.rs` runs successfully, it generates `version_generated.rs`
// in OUT_DIR with the same structure from the TOML source-of-truth.
// The hand-written array below serves as the fallback and must be kept
// in sync with data/attributes.toml.
//
// To use the generated version, uncomment the include! line below and
// remove the hand-written ATTR_REGISTRY.
//
// **Requirement:** L3-VERSION-005, L3-VERSION-006
//
// include!(concat!(env!("OUT_DIR"), "/version_generated.rs"));
// static ATTR_REGISTRY: &[AttrMeta] = GENERATED_REGISTRY;

/// All known TMATS attributes across all supported versions.
static ATTR_REGISTRY: &[AttrMeta] = &[
    // ── G-Group ──────────────────────────────────────────────────────────
    AttrMeta {
        code_name_pattern: "G\\PN",
        display_name: "Program Name",
        group: GroupPrefix::G,
        spec_section: "§9.5.2 Table 9-1",
        introduced_in: Irig106Version::V106_04,
        deprecated_in: None,
        removed_in: None,
        required_tag: RequiredTag::Required,
        max_field_size: Some(32),
        value_type: ValueType::Text,
        keywords: &[],
        description: "Name of the test program",
    },
    AttrMeta {
        code_name_pattern: "G\\106",
        display_name: "IRIG 106 Version",
        group: GroupPrefix::G,
        spec_section: "§9.5.2 Table 9-1",
        introduced_in: Irig106Version::V106_07,
        deprecated_in: None,
        removed_in: None,
        required_tag: RequiredTag::Required,
        max_field_size: Some(2),
        value_type: ValueType::Keyword,
        keywords: &["04", "05", "07", "08", "09", "11", "13", "15", "17"],
        description: "Version of IRIG 106 standard",
    },
    AttrMeta {
        code_name_pattern: "G\\OD",
        display_name: "Origination Date",
        group: GroupPrefix::G,
        spec_section: "§9.5.2 Table 9-1",
        introduced_in: Irig106Version::V106_04,
        deprecated_in: None,
        removed_in: None,
        required_tag: RequiredTag::Required,
        max_field_size: Some(10),
        value_type: ValueType::Date,
        keywords: &[],
        description: "Date of origination (MM-DD-YYYY)",
    },
    AttrMeta {
        code_name_pattern: "G\\DSI\\N",
        display_name: "Number of Data Sources",
        group: GroupPrefix::G,
        spec_section: "§9.5.2 Table 9-1",
        introduced_in: Irig106Version::V106_04,
        deprecated_in: None,
        removed_in: None,
        required_tag: RequiredTag::Required,
        max_field_size: Some(4),
        value_type: ValueType::Integer,
        keywords: &[],
        description: "Number of data sources",
    },
    AttrMeta {
        code_name_pattern: "G\\DSI-n",
        display_name: "Data Source Identifier",
        group: GroupPrefix::G,
        spec_section: "§9.5.2 Table 9-1",
        introduced_in: Irig106Version::V106_04,
        deprecated_in: None,
        removed_in: None,
        required_tag: RequiredTag::Required,
        max_field_size: Some(32),
        value_type: ValueType::Text,
        keywords: &[],
        description: "Descriptive name for this source",
    },
    AttrMeta {
        code_name_pattern: "G\\DST-n",
        display_name: "Data Source Type",
        group: GroupPrefix::G,
        spec_section: "§9.5.2 Table 9-1",
        introduced_in: Irig106Version::V106_04,
        deprecated_in: None,
        removed_in: None,
        required_tag: RequiredTag::Required,
        max_field_size: Some(3),
        value_type: ValueType::Keyword,
        keywords: &["REC", "TEL", "MUL", "PRE"],
        description: "Type of source",
    },
    AttrMeta {
        code_name_pattern: "G\\DSC-n",
        display_name: "Data Source Classification",
        group: GroupPrefix::G,
        spec_section: "§9.5.2 Table 9-1",
        introduced_in: Irig106Version::V106_04,
        deprecated_in: None,
        removed_in: None,
        required_tag: RequiredTag::Optional,
        max_field_size: Some(1),
        value_type: ValueType::Keyword,
        keywords: &["U", "C", "S"],
        description: "Classification of the data",
    },
    // ── R-Group ──────────────────────────────────────────────────────────
    AttrMeta {
        code_name_pattern: "R-x\\ID",
        display_name: "Recorder Identification",
        group: GroupPrefix::R,
        spec_section: "§9.5.4",
        introduced_in: Irig106Version::V106_04,
        deprecated_in: None,
        removed_in: None,
        required_tag: RequiredTag::RequiredCh10,
        max_field_size: Some(32),
        value_type: ValueType::Text,
        keywords: &[],
        description: "Recorder identification",
    },
    AttrMeta {
        code_name_pattern: "R-x\\N",
        display_name: "Number of Channels",
        group: GroupPrefix::R,
        spec_section: "§9.5.4",
        introduced_in: Irig106Version::V106_04,
        deprecated_in: None,
        removed_in: None,
        required_tag: RequiredTag::RequiredCh10,
        max_field_size: Some(4),
        value_type: ValueType::Integer,
        keywords: &[],
        description: "Number of channels/tracks",
    },
    AttrMeta {
        code_name_pattern: "R-x\\TK1-n",
        display_name: "Track/Channel Number",
        group: GroupPrefix::R,
        spec_section: "§9.5.4",
        introduced_in: Irig106Version::V106_04,
        deprecated_in: None,
        removed_in: None,
        required_tag: RequiredTag::RequiredCh10,
        max_field_size: Some(5),
        value_type: ValueType::Integer,
        keywords: &[],
        description: "Track number or Channel ID",
    },
    AttrMeta {
        code_name_pattern: "R-x\\CDT-n",
        display_name: "Channel Data Type",
        group: GroupPrefix::R,
        spec_section: "§9.5.4",
        introduced_in: Irig106Version::V106_07,
        deprecated_in: None,
        removed_in: None,
        required_tag: RequiredTag::RequiredCh10,
        max_field_size: Some(7),
        value_type: ValueType::Keyword,
        keywords: &[
            "PCMIN", "TIMEIN", "VIDIN", "ANAIN", "IMAIN", "UARIN", "1553IN", "16PP", "A429IN",
            "MSGIN", "DISIN", "CANIN", "FC", "ETHIN",
        ],
        description: "Channel data type",
    },
    AttrMeta {
        code_name_pattern: "R-x\\PDP-n",
        display_name: "Data Packing Option",
        group: GroupPrefix::R,
        spec_section: "§9.5.4",
        introduced_in: Irig106Version::V106_07,
        deprecated_in: None,
        removed_in: None,
        required_tag: RequiredTag::RequiredCh10,
        max_field_size: Some(3),
        value_type: ValueType::Keyword,
        keywords: &["UN", "PFS", "TM"],
        description: "Data packing: Unpacked, Packed, Throughput",
    },
    // ── P-Group ──────────────────────────────────────────────────────────
    AttrMeta {
        code_name_pattern: "P-n\\DLN",
        display_name: "Data Link Name",
        group: GroupPrefix::P,
        spec_section: "§9.5.6.1",
        introduced_in: Irig106Version::V106_04,
        deprecated_in: None,
        removed_in: None,
        required_tag: RequiredTag::Required,
        max_field_size: Some(32),
        value_type: ValueType::Text,
        keywords: &[],
        description: "PCM data link name",
    },
    AttrMeta {
        code_name_pattern: "P-n\\D1",
        display_name: "Bit Rate",
        group: GroupPrefix::P,
        spec_section: "§9.5.6.1",
        introduced_in: Irig106Version::V106_04,
        deprecated_in: None,
        removed_in: None,
        required_tag: RequiredTag::Required,
        max_field_size: Some(20),
        value_type: ValueType::Decimal,
        keywords: &[],
        description: "Data rate in bits per second",
    },
    AttrMeta {
        code_name_pattern: "P-n\\D2",
        display_name: "Encoding",
        group: GroupPrefix::P,
        spec_section: "§9.5.6.1",
        introduced_in: Irig106Version::V106_04,
        deprecated_in: None,
        removed_in: None,
        required_tag: RequiredTag::Required,
        max_field_size: Some(5),
        value_type: ValueType::Keyword,
        keywords: &[
            "NRZ-L", "NRZ-M", "NRZ-S", "RNRZ-L", "BIO-L", "BIO-M", "BIO-S", "DBP-M", "DBP-S",
            "DBP-L", "FSKM",
        ],
        description: "PCM encoding type",
    },
    AttrMeta {
        code_name_pattern: "P-n\\F1",
        display_name: "Words per Minor Frame",
        group: GroupPrefix::P,
        spec_section: "§9.5.6.1",
        introduced_in: Irig106Version::V106_04,
        deprecated_in: None,
        removed_in: None,
        required_tag: RequiredTag::Required,
        max_field_size: Some(5),
        value_type: ValueType::Integer,
        keywords: &[],
        description: "Number of words per minor frame",
    },
    AttrMeta {
        code_name_pattern: "P-n\\F2",
        display_name: "Bits per Word",
        group: GroupPrefix::P,
        spec_section: "§9.5.6.1",
        introduced_in: Irig106Version::V106_04,
        deprecated_in: None,
        removed_in: None,
        required_tag: RequiredTag::Required,
        max_field_size: Some(2),
        value_type: ValueType::Integer,
        keywords: &[],
        description: "Common word length in bits",
    },
    AttrMeta {
        code_name_pattern: "P-n\\F3",
        display_name: "Sync Pattern",
        group: GroupPrefix::P,
        spec_section: "§9.5.6.1",
        introduced_in: Irig106Version::V106_04,
        deprecated_in: None,
        removed_in: None,
        required_tag: RequiredTag::Required,
        max_field_size: Some(64),
        value_type: ValueType::Text,
        keywords: &[],
        description: "Frame sync pattern (hex)",
    },
    // ── B-Group ──────────────────────────────────────────────────────────
    AttrMeta {
        code_name_pattern: "B-n\\DLN",
        display_name: "Data Link Name",
        group: GroupPrefix::B,
        spec_section: "§9.5.6.3",
        introduced_in: Irig106Version::V106_04,
        deprecated_in: None,
        removed_in: None,
        required_tag: RequiredTag::Required,
        max_field_size: Some(32),
        value_type: ValueType::Text,
        keywords: &[],
        description: "Bus data link name",
    },
    AttrMeta {
        code_name_pattern: "B-n\\BT",
        display_name: "Bus Type",
        group: GroupPrefix::B,
        spec_section: "§9.5.6.3",
        introduced_in: Irig106Version::V106_04,
        deprecated_in: None,
        removed_in: None,
        required_tag: RequiredTag::Required,
        max_field_size: Some(4),
        value_type: ValueType::Keyword,
        keywords: &["1553", "A429"],
        description: "Bus type",
    },
    // ── C-Group ──────────────────────────────────────────────────────────
    AttrMeta {
        code_name_pattern: "C-n\\DCN",
        display_name: "Data Conversion Name",
        group: GroupPrefix::C,
        spec_section: "§9.5.8",
        introduced_in: Irig106Version::V106_04,
        deprecated_in: None,
        removed_in: None,
        required_tag: RequiredTag::Required,
        max_field_size: Some(32),
        value_type: ValueType::Text,
        keywords: &[],
        description: "Measurement name for conversion",
    },
    AttrMeta {
        code_name_pattern: "C-n\\DCT",
        display_name: "Data Conversion Type",
        group: GroupPrefix::C,
        spec_section: "§9.5.8",
        introduced_in: Irig106Version::V106_04,
        deprecated_in: None,
        removed_in: None,
        required_tag: RequiredTag::Required,
        max_field_size: Some(4),
        value_type: ValueType::Keyword,
        keywords: &["PAIR", "COEF", "TABL", "POLY", "FORM"],
        description: "Conversion type",
    },
];

// ─── AttrMetaRegistry (L3-VERSION-005) ───────────────────────────────────────

/// Indexed registry for fast attribute metadata lookup.
///
/// **Requirement:** L3-VERSION-005
pub struct AttrMetaRegistry {
    /// All attribute entries.
    entries: &'static [AttrMeta],
    /// Index by normalized code-name pattern.
    by_pattern: HashMap<&'static str, &'static AttrMeta>,
    /// Index by group.
    by_group: HashMap<GroupPrefix, Vec<&'static AttrMeta>>,
}

impl Default for AttrMetaRegistry {
    fn default() -> Self {
        Self::new()
    }
}

impl AttrMetaRegistry {
    /// Build the registry from the static data.
    pub fn new() -> Self {
        let mut by_pattern = HashMap::new();
        let mut by_group: HashMap<GroupPrefix, Vec<&'static AttrMeta>> = HashMap::new();

        for meta in ATTR_REGISTRY {
            by_pattern.insert(meta.code_name_pattern, meta);
            by_group.entry(meta.group).or_default().push(meta);
        }

        Self {
            entries: ATTR_REGISTRY,
            by_pattern,
            by_group,
        }
    }

    /// Look up attribute metadata by exact code-name pattern.
    pub fn get(&self, pattern: &str) -> Option<&&'static AttrMeta> {
        self.by_pattern.get(pattern)
    }

    /// Get all attributes for a given group.
    pub fn group(&self, prefix: GroupPrefix) -> &[&'static AttrMeta] {
        self.by_group
            .get(&prefix)
            .map(|v| v.as_slice())
            .unwrap_or(&[])
    }

    /// Get all attributes valid for a given version.
    pub fn for_version(&self, version: Irig106Version) -> Vec<&'static AttrMeta> {
        self.entries
            .iter()
            .filter(|m| m.is_valid_for(version))
            .collect()
    }

    /// Get all required attributes for a given version.
    pub fn required_for_version(&self, version: Irig106Version) -> Vec<&'static AttrMeta> {
        self.entries
            .iter()
            .filter(|m| m.is_valid_for(version) && m.required_tag.is_required())
            .collect()
    }

    /// Get all required attributes for Ch10 context.
    pub fn required_for_ch10(&self, version: Irig106Version) -> Vec<&'static AttrMeta> {
        self.entries
            .iter()
            .filter(|m| m.is_valid_for(version) && m.required_tag.is_required_ch10())
            .collect()
    }

    /// Compute migration diff between two versions.
    ///
    /// **Requirement:** L2-VERSION-004, L3-VERSION-010
    pub fn migration_diff(&self, from: Irig106Version, to: Irig106Version) -> MigrationDiff {
        let from_set: Vec<&str> = self
            .for_version(from)
            .iter()
            .map(|m| m.code_name_pattern)
            .collect();
        let to_set: Vec<&str> = self
            .for_version(to)
            .iter()
            .map(|m| m.code_name_pattern)
            .collect();

        let added: Vec<&'static AttrMeta> = self
            .entries
            .iter()
            .filter(|m| {
                !from_set.contains(&m.code_name_pattern) && to_set.contains(&m.code_name_pattern)
            })
            .collect();

        let removed: Vec<&'static AttrMeta> = self
            .entries
            .iter()
            .filter(|m| {
                from_set.contains(&m.code_name_pattern) && !to_set.contains(&m.code_name_pattern)
            })
            .collect();

        let deprecated: Vec<&'static AttrMeta> = self
            .entries
            .iter()
            .filter(|m| m.is_valid_for(to) && m.is_deprecated_for(to) && !m.is_deprecated_for(from))
            .collect();

        MigrationDiff {
            from,
            to,
            added,
            removed,
            deprecated,
        }
    }

    /// All registered entries.
    pub fn all(&self) -> &[AttrMeta] {
        self.entries
    }

    /// Total number of registered attributes.
    pub fn len(&self) -> usize {
        self.entries.len()
    }

    /// Whether the registry has no entries.
    pub fn is_empty(&self) -> bool {
        self.entries.is_empty()
    }
}

/// Results of comparing attribute sets between two IRIG 106 versions.
///
/// **Requirement:** L3-VERSION-010
#[derive(Debug)]
pub struct MigrationDiff {
    pub from: Irig106Version,
    pub to: Irig106Version,
    pub added: Vec<&'static AttrMeta>,
    pub removed: Vec<&'static AttrMeta>,
    pub deprecated: Vec<&'static AttrMeta>,
}

impl MigrationDiff {
    pub fn has_changes(&self) -> bool {
        !self.added.is_empty() || !self.removed.is_empty() || !self.deprecated.is_empty()
    }
}

// ─── Global registry accessor ────────────────────────────────────────────────

use std::sync::OnceLock;

static REGISTRY: OnceLock<AttrMetaRegistry> = OnceLock::new();

/// Get the global attribute metadata registry.
pub fn registry() -> &'static AttrMetaRegistry {
    REGISTRY.get_or_init(AttrMetaRegistry::new)
}
