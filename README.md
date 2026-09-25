# irig106-tmats

**TMATS parser, validator, and generator for the IRIG 106 ecosystem.**

Part of the [TelemetryWorks](https://github.com/TelemetryWorks) organization — open, high-performance telemetry tooling for flight test and range instrumentation, built in Rust.  

## What is TMATS?

The Telemetry Attributes Transfer Standard (TMATS) is defined in IRIG 106 Chapter 9. It provides a standardized way to describe the configuration of telemetry data — channel definitions, PCM formats, bus configurations, measurement descriptions, and calibration data. TMATS records are embedded as the first packet (Type 0x01) in every IRIG 106 Chapter 10 recording file.  

## Features

|Capability|Description|Feature Flag|
|-|-|-|
|**Parse**|Tokenize and structure TMATS ASCII data|always|
|**Model**|Strongly-typed Rust domain model for all 9 attribute groups|always|
|**Serialize**|Emit spec-compliant `code-name:value;` format|always|
|**Ch10 Integration**|Decode/encode Type 0x01 setup record payloads|always|
|**Validate**|Rule-based validation against Ch9 §9.5 normative tables|always|
|**Query**|Channel resolution, hierarchy traversal, diff|always|
|**XML**|Parse/serialize TMATS XML (Tmats.xsd)|`xml`|
|**Generate**|Synthesize TMATS from channel inventory|`generate`|
|**Repair**|Auto-fix broken counters, missing attributes|`repair`|
|**Serde**|JSON/YAML interchange on model types|`serde`|
|**WASM**|Browser-compatible API for irig106-studio|`wasm`|

## Quick Start

```rust
use irig106\_tmats::prelude::\*;

// Parse TMATS from raw bytes
let input = b"G\\\\PN:FLIGHT\_TEST;G\\\\106:17;R-1\\\\ID:MDR;R-1\\\\N:1;R-1\\\\TK1-1:1;";
let doc = parse(input)?;

assert\_eq!(doc.general.program\_name.as\_deref(), Some("FLIGHT\_TEST"));
assert\_eq!(doc.source\_version, Some(Irig106Version::V106\_17));

// Validate against spec
let report = validate(\&doc);
println!("{} errors, {} warnings", report.errors, report.warnings);

// Resolve a channel's full configuration chain
if let Some(config) = resolve\_channel(\&doc, 1) {
    println!("Channel 1 recorder: {:?}", config.recorder.recorder\_id);
}

// Serialize back to ASCII
let output = serialize\_to\_vec(\&doc)?;
```

## Ecosystem Integration

```
┌─────────────────────┐     ┌──────────────┐     ┌──────────────┐
│ irig106-ch10-reader │────>│ irig106-tmats│<────│ irig106-write│
│ (reads Ch10 files)  │     │ (this crate) │     │ (writes Ch10)│
└─────────────────────┘     └──────┬───────┘     └──────────────┘
                                   │
                    ┌──────────────┼──────────────┐
                    │              │              │
              ┌─────┴─────┐ ┌─────┴─────┐ ┌─────┴──────┐
              │irig106-cli│ │irig106-   │ │irig106-    │
              │           │ │validator  │ │studio      │
              └───────────┘ └───────────┘ └────────────┘
```

**Important boundary:** This crate produces *payloads*, not *packets*. The `ch10::encode\_setup\_payload()` function returns `(CSDW + TMATS bytes)` which `irig106-write` wraps in a Ch10 packet header and trailer.

## Types That Belong in `irig106-types`

The following types are defined in `src/types\_bridge.rs` temporarily and should migrate to the shared `irig106-types` crate:

* `Irig106Version` — version enum used across all crates
* `DataTypeCode` — Ch10 data type codes
* `ChannelId` — u16 channel ID newtype
* `GroupPrefix` — TMATS group letter identifiers

## Requirements Traceability

All public APIs trace to requirements documented in `docs/REQUIREMENTS.md`:

* **L1** (13 capabilities): PARSE, MODEL, SERIAL, XML, VALID, VERSION, CH10, GEN, REPAIR, QUERY, ERR, PERF, INTEROP
* **L2** (62 functional requirements)
* **L3** (107 design requirements)

See `docs/ARCHITECTURE.md` for module structure and design decisions.

## Standards References

* **IRIG 106 Chapter 9** — Telemetry Attributes Transfer Standard (primary)
* **IRIG 106 Chapter 10** — Type 0x01 Setup Record format
* **RCC 123-20** — Chapter 10 Programmer's Handbook
* **RCC 124-13** — TMATS Handbook

## License

Apache-2.0
