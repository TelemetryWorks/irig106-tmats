# irig106-tmats Architecture

## Document Placement Rationale

**Crate-specific documents (this repo):**
- `docs/REQUIREMENTS.md` — L1/L2/L3 requirements specific to TMATS parsing, serialization, validation, generation
- `docs/ARCHITECTURE.md` — Module structure, internal design decisions, dependency DAG
- `docs/API_GUIDE.md` — Usage examples, migration guides, workflow walkthroughs

**Cross-ecosystem documents (irig106-docs repo):**
- Ecosystem-wide crate dependency graph and integration contracts
- Cross-crate interface specifications (e.g., irig106-write ↔ irig106-tmats payload contract)
- Standards traceability matrix covering all chapters/crates
- Contributor guides and coding standards

## Module Dependency DAG

```
L3-ARCH-003

error:: ← model::  ← parse::
                    ← serial::
                    ← validate:: ← version:: (embedded registry)
                    ← query::
                    ← generate:: (feature: "generate")
                    ← repair::   (feature: "repair") ← validate::
         ch10::    ← parse:: + serial::
         xml::     (feature: "xml") ← model:: + error::
```

## Feature Flags (L3-ARCH-002)

| Flag | Effect | Dependencies Added |
|------|--------|--------------------|
| `std` (default) | Enables std::error::Error impls | — |
| `xml` | XML parse/serialize | `quick-xml` |
| `serde` | Serde derives on model types | `serde`, `serde_json`, `indexmap/serde` |
| `wasm` | WASM-bindgen exports | `wasm-bindgen`, `serde-wasm-bindgen` |
| `rich-errors` | Terminal diagnostic rendering | `miette` |
| `generate` | TMATS generation from channel inventory | — |
| `repair` | Auto-fix broken TMATS | — |
| `full` | All of the above except `wasm` | all optional deps |

## Key Design Decisions

### 1. Two-Phase Parser (L3-PARSE-001)

Phase 1 (tokenize): raw bytes → `Vec<RawAttribute>`. Single-pass byte-level state machine, no regex, no backtracking. Handles non-printable filtering inline.

Phase 2 (structure): `Vec<RawAttribute>` → `TmatsDocument`. Routes attributes to group-specific builders by `GroupPrefix`. Each builder is independent, enabling future parallelization.

**Rationale:** TMATS attributes can appear in any order (L2-PARSE-007), so we must collect all attributes before structuring. The two-phase split cleanly separates physical format concerns from semantic structuring.

### 2. Cow<'a, str> Everywhere (L3-PERF-001)

All string fields use `Cow<'a, str>`, enabling:
- **Zero-copy mode:** Parser borrows directly from input buffer — zero allocations for string data
- **Owned mode:** `doc.into_owned()` produces `TmatsDocument<'static>` for storage/transfer across threads

### 3. IndexMap<u32, T> for Repeated Groups (L3-MODEL-013)

TMATS occurrence indices are explicitly numbered and **not necessarily contiguous** (e.g., channels 1 and 2 may use indices 2 and 1 respectively). `IndexMap` preserves insertion order while allowing non-contiguous integer keys.

### 4. `extra` Fields on Every Group (L3-MODEL-020)

Every group struct includes `extra: Vec<RawAttribute<'a>>` capturing all parsed-but-unmodeled attributes. This ensures lossless round-tripping even when the model doesn't cover every attribute in the spec.

### 5. Payload, Not Packet (L1-CH10)

This crate produces `(CSDW + serialized bytes)` payloads. `irig106-write` is responsible for packet framing (headers, trailers, checksums, multi-packet splitting). The interface contract is documented in L3-INTEROP-009.

### 6. Validation Rule Registry (L3-VALID-001)

Rules implement the `ValidationRule` trait and are stored as `Vec<Box<dyn ValidationRule>>`. This enables:
- Version-filtered execution (skip rules not applicable to the detected version)
- Profile-based selection (Full, Ch10Required, Custom)
- Future extensibility (users can add custom rules)

## Workflow Summary

| # | Workflow | Input → Output | Crates Involved |
|---|----------|---------------|-----------------|
| 1 | READ | Ch10 file → TmatsDocument | ch10-reader → **tmats** |
| 2 | WRITE | TmatsDocument → Ch10 file | **tmats** → write |
| 3 | VALIDATE | TmatsDocument → ValidationReport | **tmats** |
| 4 | GENERATE | Channel inventory → TmatsDocument | ch10-reader → **tmats** |
| 5 | REPAIR | Broken TMATS → Fixed TMATS | **tmats** |
| 6 | CONVERT | ASCII ↔ XML | **tmats** |
| 7 | QUERY | TmatsDocument → ChannelConfig | **tmats** |

## Types Bridge → irig106-types

`src/types_bridge.rs` defines types that should live in `irig106-types`:

| Type | Reason |
|------|--------|
| `Irig106Version` | Used by every crate for version-aware processing |
| `DataTypeCode` | Ch10 data type codes used by reader, writer, decoder |
| `ChannelId` | Shared newtype for channel IDs |
| `GroupPrefix` | TMATS group identifiers (useful for validator, CLI) |
