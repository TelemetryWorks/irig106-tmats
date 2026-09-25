// irig106-tmats — TMATS parser, validator, and generator
//
// # Overview
//
// This crate provides complete support for the Telemetry Attributes Transfer
// Standard (TMATS) as defined in IRIG 106 Chapter 9. It is part of the
// [TelemetryWorks](https://github.com/TelemetryWorks) ecosystem of open,
// high-performance telemetry tooling built in Rust.
//
// # Capabilities
//
// | Capability | Module | Feature |
// |------------|--------|---------|
// | Parse ASCII TMATS | `parse` | always |
// | Domain model | `model` | always |
// | Serialize ASCII | `serial` | always |
// | Ch10 payload integration | `ch10` | always |
// | Query & navigation | `query` | always |
// | Validation | `validate` | always |
// | XML format | `xml` | `xml` |
// | TMATS generation | `generate` | `generate` |
// | TMATS repair | `repair` | `repair` |
// | Serde support | on model types | `serde` |
// | WASM bindings | `wasm` | `wasm` |
// | Rich error rendering | on error types | `rich-errors` |
//
// # Quick Start
//
// ```rust
// use irig106_tmats::prelude::*;
//
// // Parse TMATS from raw bytes
// let tmats_bytes = b"G\\PN:TEST_PROGRAM;G\\106:17;";
// let doc = parse(tmats_bytes).expect("parse failed");
//
// assert_eq!(doc.general.program_name.as_deref(), Some("TEST_PROGRAM"));
//
// // Validate
// let report = validate(&doc);
// println!("{} errors, {} warnings", report.errors, report.warnings);
//
// // Serialize back to ASCII
// let output = serialize_to_vec(&doc).expect("serialize failed");
// ```
//
// # Crate Boundary
//
// This crate produces **payloads**, not packets. For writing TMATS into a
// Chapter 10 file, use `irig106-write` which consumes the payload produced
// by `ch10::encode_setup_payload()`.
//
// # Traceability
//
// All public functions and types trace to requirements documented in
// `docs/REQUIREMENTS.md`. The traceability chain is L1 (capability) →
// L2 (functional) → L3 (design) → implementation.
//
// # Module Layout (L3-ARCH-001)
//
// ```text
// error::    ← model::  ← parse::
//                        ← serial::
//                        ← validate:: ← version registry
//                        ← query::
//                        ← generate::
//                        ← repair::    ← validate::
//            ch10::      ← parse:: + serial::
// ```

// ─── Module declarations (L3-ARCH-001) ───────────────────────────────────────

pub mod error;
pub mod model;
pub mod parse;
pub mod serial;
pub mod ch10;
pub mod query;
pub mod validate;
pub mod version;
pub mod types_bridge;

#[cfg(feature = "generate")]
pub mod generate;

#[cfg(feature = "repair")]
pub mod repair;

#[cfg(feature = "xml")]
pub mod xml;

#[cfg(feature = "wasm")]
pub mod wasm;

// ─── Prelude (L3-ARCH-005) ──────────────────────────────────────────────────

/// Ergonomic imports for common usage.
///
/// **Requirement:** L3-ARCH-005
///
/// ```rust
/// use irig106_tmats::prelude::*;
/// ```
pub mod prelude {
    // Core types
    pub use crate::model::{
        TmatsDocument, OwnedTmatsDocument,
        GGroup, TGroup, RGroup, MGroup, PGroup, DGroup, BGroup, SGroup, CGroup,
        RChannel, DataSourceDecl, TmatsDate, DataLinkName,
        RawAttribute, CodeName, AttrValue, TmatsGroup,
    };

    // Type bridge (→ irig106-types)
    pub use crate::types_bridge::{
        Irig106Version, DataTypeCode, ChannelId, GroupPrefix,
    };

    // Parse API (L3-INTEROP-001)
    pub use crate::parse::{parse, parse_owned, parse_with_options, ParseOptions, ParseMode};

    // Serialize API (L3-INTEROP-002)
    pub use crate::serial::{serialize, serialize_to_vec, serialize_with_options, SerializeOptions};

    // Ch10 API (L3-INTEROP-003)
    pub use crate::ch10::{
        SetupRecordCsdw, SetupRecordPayload,
        decode_setup_payload, encode_setup_payload, detect_config_change,
    };

    // Query API
    pub use crate::query::{resolve_channel, enumerate_data_sources, enumerate_channels, ChannelConfig};

    // Validation API (L3-INTEROP-004)
    pub use crate::validate::{validate, validate_with_options, ValidationReport, ValidationContext};

    // Error types
    pub use crate::error::{TmatsError, TmatsErrors, ParseError, Diagnostic, Severity};

    // Version registry (L3-VERSION-005)
    pub use crate::version::{registry, AttrMetaRegistry, AttrMeta, RequiredTag, ValueType, MigrationDiff};

    // Generate API (L3-INTEROP-005)
    #[cfg(feature = "generate")]
    pub use crate::generate::{ChannelInventoryEntry, TmatsBuilder};

    // Repair API (L3-INTEROP-006)
    #[cfg(feature = "repair")]
    pub use crate::repair::{repair, repair_dry_run, RepairOptions, RepairReport};

    // XML API (L2-XML-001..004)
    #[cfg(feature = "xml")]
    pub use crate::xml::{parse_xml, serialize_xml, serialize_xml_to_vec, ascii_to_xml, xml_to_ascii};
}

// ─── Re-exports for top-level access (L3-ARCH-004) ──────────────────────────

pub use parse::{parse, parse_owned};
pub use serial::{serialize, serialize_to_vec};
pub use validate::validate;
pub use ch10::{decode_setup_payload, encode_setup_payload};
