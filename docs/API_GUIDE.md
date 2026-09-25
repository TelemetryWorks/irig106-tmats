# irig106-tmats API Guide

## Table of Contents

1. [Workflow Overview](#workflow-overview)
2. [Parsing TMATS](#parsing-tmats)
3. [Querying the Model](#querying-the-model)
4. [Serializing TMATS](#serializing-tmats)
5. [Chapter 10 Integration](#chapter-10-integration)
6. [Validation](#validation)
7. [Generating TMATS](#generating-tmats)
8. [Repairing TMATS](#repairing-tmats)
9. [Integration with Other Crates](#integration-with-other-crates)
10. [Feature Flags](#feature-flags)

---

## Workflow Overview

There are seven primary workflows. Each maps to a real operational scenario in flight test data processing:

| Workflow | Scenario | Entry Point |
|----------|----------|-------------|
| **READ** | Open a Ch10 file, extract TMATS | `ch10::decode_setup_payload()` |
| **WRITE** | Embed TMATS into a Ch10 file | `ch10::encode_setup_payload()` → `irig106-write` |
| **VALIDATE** | Check TMATS conformance | `validate()` |
| **GENERATE** | Ch10 has no TMATS, synthesize from packets | `TmatsBuilder::new()` |
| **REPAIR** | Fix broken counters, missing attributes | `repair()` |
| **CONVERT** | ASCII ↔ XML conversion | (planned, `xml` feature) |
| **QUERY** | "What's on channel 5?" | `resolve_channel()` |

```text
WORKFLOW 1: READ (Ch10 → TMATS Model)
──────────────────────────────────────
  irig106-ch10-reader        irig106-tmats
  ┌──────────────────┐       ┌───────────────────┐
  │ Open Ch10 file   │       │                   │
  │ Find Type 0x01   │──────>│ Decode CSDW       │
  │ Extract payload  │       │ Parse ASCII/XML   │
  │ (multi-packet    │       │ Build model       │
  │  assembly if     │       │ Return            │
  │  needed)         │       │  TmatsDocument    │
  └──────────────────┘       └───────────────────┘

  Notes:
  - irig106-ch10-reader handles packet-level concerns (headers,
    checksums, multi-packet reassembly)
  - irig106-tmats receives the raw payload bytes after CSDW
  - Question: does multi-packet assembly live in ch10-reader or tmats?
    → ch10-reader, because it's packet-framing logic, not TMATS logic.
    tmats just sees a contiguous byte buffer.
```

```text
WORKFLOW 2: WRITE (TMATS Model → Ch10)
──────────────────────────────────────
  irig106-tmats               irig106-write
  ┌───────────────────┐       ┌──────────────────┐
  │ TmatsDocument     │       │                  │
  │ Serialize to      │──────>│ Build CSDW       │
  │   ASCII bytes     │       │ Construct Type   │
  │ (or XML bytes)    │       │   0x01 packet    │
  │                   │       │   header/trailer │
  │ Encode CSDW       │──────>│ Write to Ch10    │
  │   fields          │       │   stream         │
  └───────────────────┘       └──────────────────┘

  Notes:
  - irig106-tmats produces: (SetupRecordCsdw, Vec<u8> payload)
  - irig106-write consumes that tuple and handles packet framing
  - If payload exceeds max packet size, irig106-write splits into
    multiple Type 0x01 packets (packet-level concern)
  - irig106-tmats exposes a PayloadBuilder API for this
```

```text
WORKFLOW 3: VALIDATE (TMATS → Diagnostics)
──────────────────────────────────────────
  irig106-tmats
  ┌─────────────────────────────────────────┐
  │ TmatsDocument                           │
  │   │                                     │
  │   ├─> Structural validation (counters,  │
  │   │   required attrs, MFS, keywords)    │
  │   │                                     │
  │   ├─> Cross-group reference integrity   │
  │   │                                     │
  │   ├─> Version-specific rule filtering   │
  │   │                                     │
  │   └─> Emit Vec<Diagnostic>              │
  └─────────────────────────────────────────┘

  Consumed by: irig106-validator, irig106-cli, irig106-studio
```

```text
WORKFLOW 4: GENERATE (Ch10 Packet Stream → Synthetic TMATS)
───────────────────────────────────────────────────────────
  irig106-ch10-reader        irig106-tmats
  ┌──────────────────┐       ┌───────────────────────┐
  │ Scan all packets │       │                       │
  │ Build channel    │──────>│ TmatsGenerator        │
  │   inventory:     │       │   .add_channel(id,    │
  │   - Channel IDs  │       │     data_type, ...)   │
  │   - Data types   │       │   .set_version(v)     │
  │   - Observed     │       │   .set_program(name)  │
  │     bit rates    │       │   .build()            │
  │   - Packet count │       │     → TmatsDocument   │
  │   - Time range   │       │                       │
  └──────────────────┘       └───────────────────────┘

  Notes:
  - This is the "missing TMATS" recovery workflow
  - irig106-ch10-reader provides a ChannelInventory summary
  - irig106-tmats has a builder API that constructs a minimal
    but spec-compliant TMATS from that inventory
  - The generated TMATS will have R-group channel definitions
    matching observed channels, but will lack detail that can
    only come from external knowledge (measurement names,
    calibration data, etc.)
  - The builder should mark generated-vs-known attributes
```

```text
WORKFLOW 5: REPAIR (Broken TMATS → Fixed TMATS)
───────────────────────────────────────────────
  irig106-tmats
  ┌─────────────────────────────────────────────┐
  │ Parse (lenient mode) → partial model        │
  │   │                                         │
  │   ├─> Validate → collect diagnostics        │
  │   │                                         │
  │   ├─> Auto-fix engine:                      │
  │   │   - Recompute broken \N counters        │
  │   │   - Add missing R-CH10 required attrs   │
  │   │   - Normalize keyword case              │
  │   │   - Remove duplicate attributes         │
  │   │                                         │
  │   ├─> Re-validate → confirm fixes           │
  │   │                                         │
  │   └─> Serialize → corrected TMATS bytes     │
  └─────────────────────────────────────────────┘

  Consumed by: irig106-cli (tmats repair command),
               irig106-validator (with --fix flag)
```

```text
WORKFLOW 6: CONVERT (ASCII ↔ XML)
─────────────────────────────────
  irig106-tmats
  ┌──────────────────────────────────────┐
  │ Parse ASCII → TmatsDocument          │
  │   │                                  │
  │   ├─> Apply XML divergence rules:    │
  │   │   - Expand C-groups per link     │
  │   │   - Drop \N counters             │
  │   │   - Expand keyword values        │
  │   │   - Convert date formats         │
  │   │                                  │
  │   └─> Serialize XML                  │
  │        (and vice versa)              │
  └──────────────────────────────────────┘
```

```text
WORKFLOW 7: QUERY (Interactive Navigation)
──────────────────────────────────────────
  irig106-studio / irig106-cli       irig106-tmats
  ┌────────────────────────┐         ┌──────────────────┐
  │ User: "show me what's  │         │ query API:       │
  │  on channel 5"         │────────>│  resolve_channel │
  │                        │         │  (5)             │
  │ User: "list all PCM    │         │                  │
  │  data sources"         │────────>│  iter_groups::<  │
  │                        │         │    PGroup>()     │
  │ User: "what version    │         │                  │
  │  is this file?"        │────────>│  doc.version()   │
  └────────────────────────┘         └──────────────────┘
```

---

## Parsing TMATS

### Basic Parse (Zero-Copy)

```rust
use irig106_tmats::prelude::*;

let input = b"G\\PN:FLIGHT_TEST;G\\106:17;R-1\\ID:MDR;R-1\\N:1;R-1\\TK1-1:5;";

// Borrowed parse — references input buffer, no string allocations
let doc = parse(input)?;

assert_eq!(doc.general.program_name.as_deref(), Some("FLIGHT_TEST"));
assert_eq!(doc.source_version, Some(Irig106Version::V106_17));
```

The returned `TmatsDocument<'_>` borrows from the input buffer. This is the fastest mode but the document's lifetime is tied to the input.

### Owned Parse

```rust
// Produces TmatsDocument<'static> — can be stored, sent across threads
let doc = parse_owned(input)?;
```

### Parse with Options

```rust
let opts = ParseOptions {
    mode: ParseMode::Lenient,        // Continue past errors
    target_version: Some(Irig106Version::V106_17),
    max_attributes: 50_000,          // DoS protection
    max_value_size: 512_000,
};

let doc = parse_with_options(input, &opts)?;

// In lenient mode, check for parse diagnostics
for diag in &doc.parse_diagnostics {
    eprintln!("{}", diag);
}
```

---

## Querying the Model

### Resolve a Channel's Full Configuration

This walks the hierarchy: R-group → data link name → P/B/S group → D-group → C-group.

```rust
if let Some(config) = resolve_channel(&doc, 5) {
    println!("Channel {} on recorder '{}'",
        config.channel_id,
        config.recorder.recorder_id.as_deref().unwrap_or("unknown"));

    match &config.format {
        Some(FormatRef::Pcm(p)) => println!("  PCM @ {} bps", p.bit_rate.unwrap_or(0.0)),
        Some(FormatRef::Bus(b)) => println!("  Bus type: {}", b.bus_type.as_deref().unwrap_or("?")),
        Some(FormatRef::Message(s)) => println!("  Message stream"),
        None => println!("  No format group linked"),
    }
}
```

### Enumerate All Channels

```rust
for (r_idx, ch_idx, channel) in enumerate_channels(&doc) {
    println!("R-{}\\TK1-{}: channel_id={:?}, type={:?}",
        r_idx, ch_idx, channel.channel_id, channel.data_type);
}
```

### Enumerate Data Sources

```rust
for (idx, ds) in enumerate_data_sources(&doc) {
    println!("G\\DSI-{}: {} ({})",
        idx,
        ds.data_source_id.as_deref().unwrap_or("?"),
        ds.classification.as_deref().unwrap_or("U"));
}
```

### Direct Group Access

```rust
// Iterate all R-groups
for (&idx, r_group) in &doc.recorders {
    println!("R-{}: {} channels", idx, r_group.channels.len());
}

// Access a specific P-group
if let Some(p) = doc.pcm_formats.get(&1) {
    println!("P-1 bit rate: {:?}", p.bit_rate);
}
```

---

## Serializing TMATS

### Basic Serialize

```rust
// To Vec<u8> (in-memory)
let bytes = serialize_to_vec(&doc)?;

// To any Write implementor
let mut file = std::fs::File::create("output.tmt")?;
serialize(&doc, &mut file)?;
```

### With Options

```rust
use irig106_tmats::serial::{SerializeOptions, OutputFormat, LineEnding};

let opts = SerializeOptions {
    format: OutputFormat::Pretty,   // Line breaks between attrs
    line_ending: LineEnding::CrLf,  // Windows-style
    emit_comments: true,
    canonical_keyword_case: false,
};

let mut buf = Vec::new();
serialize_with_options(&doc, &mut buf, &opts)?;
```

### Counter Auto-Computation

The serializer automatically computes \N counter values from actual group sizes. If the parsed document had `G\DSI\N:99` but only 2 data sources, the serializer emits `G\DSI\N:2;`.

---

## Chapter 10 Integration

### Reading TMATS from a Ch10 Packet

```rust
// `payload_bytes` is the data portion of a Type 0x01 packet
// (after the packet header, before the trailer)
let (csdw, doc) = decode_setup_payload(payload_bytes)?;

println!("IRIG version: {:?}", csdw.irig_version());
println!("Config changed: {}", csdw.config_change);
println!("Program: {:?}", doc.general.program_name);
```

### Writing TMATS for a Ch10 Packet

```rust
let csdw = SetupRecordCsdw::from_version(Irig106Version::V106_17, false);
let payload = encode_setup_payload(&doc, &csdw)?;

// payload.to_bytes() → [CSDW (4 bytes)] + [TMATS ASCII]
// Pass this to irig106-write for packet framing
let raw_bytes = payload.to_bytes();
```

### Detecting Configuration Changes

```rust
let changed = detect_config_change(&prev_doc, &curr_doc)?;
if changed {
    let csdw = SetupRecordCsdw::from_version(Irig106Version::V106_17, true);
    // Emit a new setup record with config_change = true
}
```

---

## Validation

### Basic Validation

```rust
let report = validate(&doc);

println!("{} errors, {} warnings, {} info",
    report.errors, report.warnings, report.info);

for diag in &report.diagnostics {
    println!("{}", diag);
    // Output: [ERROR] TMATS-V001: missing required attribute G\PN → Ch9 §9.5.2
}
```

### With Context

```rust
use irig106_tmats::validate::*;

let ctx = ValidationContext {
    target_version: Some(Irig106Version::V106_17),
    source: TmatsSource::Ch10,
    profile: ValidationProfile::Ch10Required,
};

let report = validate_with_options(&doc, &ctx);
```

### Built-in Rules

| Rule ID | Checks | Severity |
|---------|--------|----------|
| TMATS-V001 | G\PN required | Error |
| TMATS-V002 | G\106 required | Error |
| TMATS-V010 | G\DSI\N counter matches data sources | Error |
| TMATS-V011 | R-x\N counter matches channels | Error |
| TMATS-V020 | R-x\CDLN-n references valid P/B/S link | Warning |

---

## Generating TMATS

When a Ch10 file has missing or corrupt TMATS, generate one from observed channel data:

```rust
use irig106_tmats::prelude::*;

let doc = TmatsBuilder::new(Irig106Version::V106_17)
    .program_name("RECOVERY_GEN")
    .test_number("FT-2024-001")
    .add_channel(ChannelInventoryEntry {
        channel_id: 1,
        data_type: DataTypeCode::Pcm,
        subchannel_count: None,
        observed_bit_rate: Some(5_000_000.0),
        observed_sample_rate: None,
        packet_count: Some(10_000),
    })
    .add_channel(ChannelInventoryEntry {
        channel_id: 3,
        data_type: DataTypeCode::Mil1553Format1,
        subchannel_count: None,
        observed_bit_rate: None,
        observed_sample_rate: None,
        packet_count: Some(5_000),
    })
    .build()?;

// Generated TMATS validates cleanly
let report = validate(&doc);
assert_eq!(report.errors, 0);

// Serialize and embed in a Ch10 file via irig106-write
let payload = encode_setup_payload(
    &doc,
    &SetupRecordCsdw::from_version(Irig106Version::V106_17, false),
)?;
```

The generator automatically creates: G-group (program, version, data source declarations), R-group (channel definitions), and stub format groups (P for PCM, B for 1553/ARINC, S for message data).

---

## Repairing TMATS

### Auto-Repair

```rust
use irig106_tmats::repair::*;

let mut doc = parse(broken_tmats_bytes)?;

let report = repair(&mut doc, &RepairOptions::default());

println!("Pre-repair errors: {}", report.pre_repair_errors);
println!("Post-repair errors: {}", report.post_repair_errors);
for action in &report.actions_taken {
    println!("  {:?} {} : {:?} → {:?}",
        action.action_type, action.attribute_path,
        action.old_value, action.new_value);
}
```

### Dry-Run

```rust
let report = repair_dry_run(&doc);
// No modifications — just findings
for finding in &report.findings {
    println!("{}: {} (fixable: {})",
        finding.attribute_path.as_deref().unwrap_or("?"),
        finding.description,
        finding.auto_fixable);
}
```

### Selective Repairs

```rust
let opts = RepairOptions {
    dry_run: false,
    auto_fix_counters: true,
    auto_fix_case: false,             // Skip case normalization
    auto_fix_missing_required: false, // Don't insert placeholders
    duplicate_strategy: DuplicateStrategy::KeepLast,
};

let report = repair(&mut doc, &opts);
```

---

## Integration with Other Crates

### With irig106-ch10-reader (READ workflow)

```rust
// Pseudocode — ch10-reader provides the raw packet payload
let payload = ch10_reader.read_setup_record()?;
let (csdw, doc) = irig106_tmats::decode_setup_payload(&payload)?;
```

### With irig106-write (WRITE workflow)

```rust
// This crate produces the payload; irig106-write frames it
let payload = irig106_tmats::encode_setup_payload(&doc, &csdw)?;
let packet_bytes = irig106_write::build_type_0x01_packet(payload.to_bytes());
```

### With irig106-validator

```rust
// Validator consumes the validation report
let report = irig106_tmats::validate(&doc);
if report.has_errors() {
    for diag in &report.diagnostics {
        validator.record_finding(diag);
    }
}
```

### With irig106-studio (WASM)

Enable the `wasm` feature and use the WASM bindings module (planned).

---

## Feature Flags

Enable features in your `Cargo.toml`:

```toml
[dependencies]
irig106-tmats = { version = "0.1", features = ["generate", "repair", "serde"] }

# Or everything:
irig106-tmats = { version = "0.1", features = ["full"] }
```

| Flag | Adds | Use When |
|------|------|----------|
| `generate` | `TmatsBuilder`, `ChannelInventoryEntry` | Synthesizing TMATS from channel inventory |
| `repair` | `repair()`, `RepairOptions`, `RepairReport` | Auto-fixing broken TMATS files |
| `serde` | JSON/YAML serialize/deserialize on model types | Exporting to dashboards, APIs |
| `xml` | XML parse/serialize (Tmats.xsd) | Working with XML-format TMATS |
| `wasm` | Browser-compatible API | irig106-studio integration |
| `rich-errors` | miette terminal diagnostics | CLI tools with pretty error rendering |
| `full` | All above except `wasm` | Development convenience |
