# irig106-tmats Roadmap

> Spec-compliant TMATS parser, validator, and generator for IRIG 106 Chapter 9 — built in Rust.

This roadmap outlines the phased development plan for `irig106-tmats`. Each milestone maps to L1/L2/L3 requirements documented in `docs/REQUIREMENTS.md` and aligns with the broader TelemetryWorks ecosystem release cadence.

---

## Current Status

**v0.1.0-dev** — All 13 L1 capabilities implemented. Core parse/serialize/validate/query/ch10 workflows operational with 105 tests, 13 validation rules, 4 repair rules, and 23-entry attribute registry.

```text
═══════════════════════════════════════════════════════════════════
  38 files │ 10,571 lines │ 105 tests │ 3 fuzz │ 8 benchmarks
  13/13 L1 │ 57/62 L2 │ 96/107 L3 │ 13 validation │ 4 repair rules
═══════════════════════════════════════════════════════════════════
```
---

## Phase 1: Foundation (v0.1.0) ✅

*Goal: Core parse → model → serialize loop with Ch10 payload integration.*

- [x] Two-phase ASCII parser (tokenize + structure)
- [x] Strongly-typed domain model for all 9 TMATS groups (G/T/R/M/P/D/B/S/C)
- [x] ASCII serializer with counter auto-computation
- [x] Ch10 Type 0x01 CSDW decode/encode
- [x] Setup record payload builder for `irig106-write`
- [x] Zero-copy borrowed parsing (`Cow<'a, str>`)
- [x] Owned parsing mode (`TmatsDocument<'static>`)
- [x] Error hierarchy with byte-offset/line location
- [x] Lenient parse mode for malformed TMATS recovery
- [x] Prelude and public API surface

**Delivered:** 12 source files, 39 tests

---

## Phase 2: Validation & Query (v0.2.0) ✅

*Goal: Spec-driven validation engine and channel resolution API.*

- [x] `ValidationRule` trait with version-filtered execution
- [x] Required attribute rules (G\PN, G\106, P-group, B-group)
- [x] Counter consistency rules (G\DSI\N, R-x\N)
- [x] Cross-group reference integrity (R-x\CDLN-n → P/B/S)
- [x] Channel configuration resolution (R → P/B/S → D → C)
- [x] Data source and channel enumeration
- [x] Document diff API
- [x] Validation profiles (Full, Ch10Required, Custom)
- [x] TMATS generation from channel inventory (`TmatsBuilder`)
- [x] TMATS repair engine with counter recomputation
- [x] Dry-run repair mode

**Delivered:** Generate, repair, query modules; 51 tests

---

## Phase 3: Registry & Rigor (v0.3.0) ✅

*Goal: Machine-readable attribute registry driving validation, plus property-based testing.*

- [x] `AttrMeta` struct and `AttrMetaRegistry` with version/group indexing
- [x] 23-entry registry from Ch9 §9.5 tables
- [x] `data/attributes.toml` source-of-truth
- [x] `build.rs` codegen (TOML → static Rust array)
- [x] Registry-driven keyword validation (G\DST, R\PDP, P\D2, B\BT)
- [x] MFS enforcement (G\PN ≤ 32 chars)
- [x] Date format validation (MM-DD-YYYY range checks)
- [x] Version migration diff algorithm
- [x] Property-based tests (proptest: round-trip, counters, Ch10, into_owned)
- [x] Criterion benchmark suite (8 benchmarks)

**Delivered:** Version registry, 13 validation rules, 87 tests, benchmarks

---

## Phase 4: Full Coverage (v0.4.0) ✅

*Goal: XML module, WASM bindings, fuzz testing, repair expansion.*

- [x] XML parse/serialize with `quick-xml`
- [x] XML divergence handling (per-link C-groups, no counters, expanded keywords, XML dates)
- [x] ASCII ↔ XML bidirectional conversion
- [x] Keyword expansion registry (abbreviated ↔ full form)
- [x] WASM bindings (`parse_tmats`, `validate_tmats`, `get_channel_config`)
- [x] Fuzz targets (parse, round-trip, Ch10 decode) with seed corpus
- [x] Case normalization repair rule
- [x] Orphan detection repair rule
- [x] Duplicate attribute detection in parser (strict reject / lenient keep-last)

**Delivered:** XML, WASM, fuzz, 4 repair rules, 105 tests total

---

## Phase 5: Attribute Registry Expansion (v0.5.0) 🔜

*Goal: Complete the attribute registry to cover all ~200+ attributes across §9.5 tables.*

- [ ] Expand `data/attributes.toml` to full G-group (Table 9-1, ~30 attrs)
- [ ] Expand R-group attributes (Table 9-3/9-4, ~60 attrs including drive/volume/event config)
- [ ] Expand P-group attributes (Table 9-5, ~40 attrs including subframe/word definitions)
- [ ] Expand D-group measurement attributes
- [ ] Expand B-group attributes (1553 message definitions, ARINC-429 specifics)
- [ ] Expand M-group, S-group, C-group attributes
- [ ] T-group transmission attributes (antenna, receiver, subcarrier config)
- [ ] Enable `build.rs` → `include!()` path (replace hand-written static array)
- [ ] Conditional "Required when" rule evaluation (L2-VALID-002)
- [ ] R-CH10/RO-CH10 mandatory attribute enforcement in Ch10 context (L2-VALID-008)
- [ ] Registry-driven MFS enforcement for all attributes (not just G\PN)
- [ ] Per-attribute keyword validation auto-generated from registry

**Target:** ~200+ registry entries, full Ch10 validation coverage

---

## Phase 6: Performance & Streaming (v0.6.0)

*Goal: Production-grade performance for high-throughput telemetry pipelines.*

- [ ] `TmatsInput` trait for `&[u8]`, `Read`, memory-mapped files (L3-PARSE-003)
- [ ] Incremental/streaming parse for large multi-packet TMATS (L2-PARSE-008)
- [ ] Lazy/deferred group construction — only materialize groups the caller uses (L3-PERF-006)
- [ ] Pre-sized collections from \N counter hints (L3-PERF-005)
- [ ] O(1) code-name lookup via lazy `HashMap` index with `OnceLock` (L3-QUERY-001)
- [ ] Generic group iterator via `TmatsGroup` trait (L3-QUERY-004)
- [ ] Parallel validation rule execution via `rayon` (L3-VALID-009)
- [ ] Allocation count tracking in benchmarks (L3-PERF-003)
- [ ] Benchmark regression CI gate — fail build if throughput regresses >10% (L3-TEST-012)
- [ ] Multi-packet TMATS assembly support for very large configs (L2-PERF-004)

**Target:** <1μs parse for small TMATS, <100μs for 64-channel configs

---

## Phase 7: Advanced Features (v0.7.0)

*Goal: DDML/IHAL support, advanced repair, and operational tooling.*

- [ ] DDML (Data Description Markup Language) representation (L2-MODEL-013, Ch9 §9.6)
- [ ] IHAL (Instrumentation Hardware Abstraction Language) representation (L2-MODEL-014, Ch9 §9.7)
- [ ] Mid-stream TMATS detection for network streaming scenarios (L2-CH10-005)
- [ ] Generation metadata: annotate inferred vs. provided vs. missing attributes (L2-GEN-005)
- [ ] Duplicate attribute resolution repair rule (L2-REPAIR-002)
- [ ] Missing R-CH10 required attribute insertion repair rule (L2-REPAIR-004)
- [ ] `GroupBuilder` trait extraction for extensible parser architecture (L3-PARSE-010)
- [ ] Custom user-defined validation rules via the `ValidationRule` trait
- [ ] TMATS template library — pre-built configs for common recorder types

---

## Phase 8: Ecosystem Integration (v1.0.0)

*Goal: Stable release with full ecosystem wiring.*

- [ ] Migrate `types_bridge.rs` types to `irig106-types` crate
- [ ] Verify `no_std + alloc` compatibility for WASM core (L2-INTEROP-006)
- [ ] WASM smoke tests via `wasm-pack test` in CI (L3-TEST-011)
- [ ] `irig106-cli` subcommand integration (`tmats dump/validate/repair/generate/diff/channels`)
- [ ] `irig106-studio` WASM integration — browser-based TMATS viewer/editor
- [ ] `irig106-validator` delegation — file-level validator consumes TMATS validation API
- [ ] `irig106-write` integration test — full pipeline from `TmatsBuilder` → Ch10 file
- [ ] `irig106-ch10-reader` integration test — real Ch10 files → `decode_setup_payload`
- [ ] Publish to crates.io
- [ ] Semantic versioning commitment and MSRV policy
- [ ] Cross-crate integration spec finalized in `irig106-docs`

---

## Backlog (Unscheduled)

Items that may be pulled into a phase based on ecosystem needs:

| Item | Driver |
|------|--------|
| TMATS XML Schema (Tmats.xsd) strict validation | User demand for XML-first workflows |
| `miette` rich error rendering integration | `irig106-cli` UX improvement |
| TMATS comparison reports (HTML/PDF output) | Flight test report generation |
| Real-time TMATS monitoring (watch for config changes) | Network streaming use case |
| Python bindings via PyO3 | Data science / Jupyter integration |
| C FFI bindings | Legacy system integration |
| TMATS editor GUI component | `irig106-studio` feature |
| Attribute coverage report (% of §9.5 tables modeled) | Quality gate metric |
| PAM attributes (removed in 106-13, needed for legacy files) | Historical data processing |
| STANAG 4575 directory metadata integration | NATO interoperability |

---

## How to Contribute

1. Check the phase you're interested in above
2. Find the corresponding L2/L3 requirements in `docs/REQUIREMENTS.md`
3. Items marked `(planned)` in REQUIREMENTS.md are available for contribution
4. Open an issue referencing the requirement ID (e.g., "Implement L2-PARSE-008: Streaming parse")
5. PRs should include tests that trace to the requirement

---

## Version History

| Version | Date | Milestone |
|---------|------|-----------|
| 0.1.0 | — | Phase 1: Foundation |
| 0.2.0 | — | Phase 2: Validation & Query |
| 0.3.0 | — | Phase 3: Registry & Rigor |
| 0.4.0-dev | 2026-03-23 | Phase 4: Full Coverage (current) |
