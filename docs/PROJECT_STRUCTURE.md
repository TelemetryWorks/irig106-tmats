<!-- docs/PROJECT_STRUCTURE.md -->

# Project Structure

```text
irig106-tmats/
│
├── Cargo.toml                           ← crate manifest, 7 feature flags
├── build.rs                             ← TOML → Rust codegen for attribute registry
├── README.md                            ← overview, ecosystem diagram, quick start
│
├── data/
│   └── attributes.toml                  ← 23 attributes from Ch9 §9.5 tables
│
├── docs/
│   ├── REQUIREMENTS.md                  ← 182 requirements with traceability
│   ├── ARCHITECTURE.md                  ← module DAG, design decisions
│   ├── API_GUIDE.md                     ← 7 workflows with code examples
│   ├── PROJECT_STRUCTURE.md             ← full inventory, all metrics
│   └── FOR_IRIG106_DOCS_REPO.md         ← cross-crate contracts (→ irig106-docs)
│
├── src/                                 ── 14 modules, 6,204 lines ──
│   ├── lib.rs                           ← prelude, re-exports
│   ├── types_bridge.rs                  ← types → irig106-types
│   ├── error.rs                         ← TmatsError hierarchy
│   ├── model.rs                         ← 9 group structs + CodeName + AttrValue
│   ├── parse.rs                         ← two-phase parser + duplicate detection
│   ├── serial.rs                        ← visitor serializer + counter auto-compute
│   ├── ch10.rs                          ← CSDW codec, payload builder
│   ├── query.rs                         ← channel resolution, diff
│   ├── validate.rs                      ← 13 validation rules
│   ├── version.rs                       ← AttrMetaRegistry, migration diff
│   ├── generate.rs                      ← TmatsBuilder            (feature: generate)
│   ├── repair.rs                        ← 4 repair rules           (feature: repair)
│   ├── xml.rs                           ← XML parse/serialize      (feature: xml)
│   └── wasm.rs                          ← browser bindings         (feature: wasm)
│
├── tests/                               ── 8 files, 99 #[test] + 6 proptest ──
│   ├── parse_tests.rs                   ← 16 tests
│   ├── serial_tests.rs                  ← 8 tests
│   ├── ch10_tests.rs                    ← 10 tests
│   ├── validate_query_tests.rs          ← 14 tests
│   ├── version_registry_tests.rs        ← 24 tests
│   ├── repair_tests.rs                  ← 10 tests
│   ├── xml_tests.rs                     ← 11 tests
│   └── property_tests.rs               ← 6 proptest properties
│
├── benches/
│   └── parse_bench.rs                   ← 8 criterion benchmarks
│
└── fuzz/                                ── 3 targets, 2 seed files ──
    ├── Cargo.toml
    ├── corpus/fuzz_parse/
    │   ├── minimal.tmt
    │   └── multigroup.tmt
    └── fuzz_targets/
        ├── fuzz_parse.rs
        ├── fuzz_round_trip.rs
        └── fuzz_ch10_decode.rs
```

## Module Dependency DAG

```
                                ┌──────────┐
                                │ lib.rs   │ (prelude, re-exports)
                                └────┬─────┘
                                     │
         ┌───────────────────────────┼────────────────────────────┐
         │                           │                            │
    ┌────┴─────┐              ┌──────┴──────┐              ┌─────┴──────┐
    │ error.rs │◄─────────────│  model.rs   │              │types\_bridge│
    └────┬─────┘              └──────┬──────┘              └─────┬──────┘
         │                    ┌──────┼──────┬──────┐             │
         │               ┌───┴──┐ ┌─┴──┐ ┌─┴───┐ ┌┴────┐       │
         │               │parse │ │ser-│ │vali-│ │query│       │
         │               │ .rs  │ │ial │ │date │ │ .rs │       │
         │               └──┬───┘ │.rs │ │ .rs │ └─────┘       │
         │                  │     └─┬──┘ └──┬──┘               │
         │                  │       │    ┌──┴──────┐           │
         │               ┌──┴───────┴──┐ │version.rs│◄─────────┘
         │               │   ch10.rs   │ │(registry)│
         │               │(parse+serial│ └──────────┘
         │               └─────────────┘
         │
    ┌────┴──────────────────────────────────────┐
    │           Feature-gated modules           │
    ├───────────┬───────────┬──────────┬────────┤
    │generate.rs│ repair.rs │  xml.rs  │wasm.rs │
    │(model,    │(model,    │(model,   │(parse, │
    │ types)    │ validate) │ error)   │ valid, │
    │           │           │          │ query) │
    └───────────┴───────────┴──────────┴────────┘
```

## File Inventory

### Source Code (14 files, 5,804 lines)

|File|Lines|L1|Key Types/Functions|
|-|-|-|-|
|`lib.rs`|159|ARCH|`prelude`, re-exports|
|`types\_bridge.rs`|284|VERSION|`Irig106Version`, `DataTypeCode`, `ChannelId`, `GroupPrefix`|
|`error.rs`|364|ERR|`TmatsError`, `ParseError`, `Diagnostic`, `Severity`|
|`model.rs`|946|MODEL|`TmatsDocument`, `GGroup`, `TGroup`, `RGroup`, `MGroup`, `PGroup`, `DGroup`, `BGroup`, `SGroup`, `CGroup`, `RChannel`, `CodeName`, `AttrValue`, `TmatsDate`|
|`parse.rs`|714|PARSE|`parse()`, `parse\_owned()`, `parse\_with\_options()`, `ParseOptions`, `ParseMode`|
|`serial.rs`|351|SERIAL|`serialize()`, `serialize\_to\_vec()`, `SerializeOptions`, `OutputFormat`|
|`ch10.rs`|202|CH10|`SetupRecordCsdw`, `SetupRecordPayload`, `decode\_setup\_payload()`, `encode\_setup\_payload()`|
|`query.rs`|221|QUERY|`resolve\_channel()`, `enumerate\_channels()`, `enumerate\_data\_sources()`, `diff()`, `ChannelConfig`, `FormatRef`|
|`validate.rs`|736|VALID|`validate()`, `validate\_with\_options()`, `ValidationRule` trait, 13 built-in rules, `ValidationReport`, `ValidationProfile`|
|`version.rs`|635|VERSION|`AttrMeta`, `AttrMetaRegistry`, `RequiredTag`, `ValueType`, `MigrationDiff`, `registry()`|
|`generate.rs`|187|GEN|`TmatsBuilder`, `ChannelInventoryEntry`|
|`repair.rs`|298|REPAIR|`repair()`, `repair\_dry\_run()`, `RepairRule` trait, `RepairOptions`, `RepairReport`|
|`xml.rs`|575|XML|`parse\_xml()`, `serialize\_xml()`, `ascii\_to\_xml()`, `xml\_to\_ascii()`, keyword expansion registry|
|`wasm.rs`|132|INTEROP|`parse\_tmats()`, `validate\_tmats()`, `get\_channel\_config()`, `serialize\_tmats\_ascii()`|

### Tests (8 files, 1,915 lines, 99 #\[test] + 6 proptest)

|File|Lines|Tests|Coverage Area|
|-|-|-|-|
|`parse\_tests.rs`|313|16|Tokenization, per-group parsing, order independence, case, comments, lenient, duplicates|
|`serial\_tests.rs`|155|8|Round-trip fidelity, counter auto-computation, compact/pretty, extra preservation|
|`ch10\_tests.rs`|140|10|CSDW encode/decode, version mapping, payload round-trip, config change|
|`validate\_query\_tests.rs`|207|14|Required attrs, counters, cross-refs, channel resolution, generation validation|
|`version\_registry\_tests.rs`|302|24|Registry lookup, group/version filter, keyword/MFS/date validation, migration diff|
|`repair\_tests.rs`|217|10|Counter recomputation, idempotency, dry-run, error reduction, case normalization, orphans|
|`xml\_tests.rs`|272|11|XML parse/serialize, divergence handling, ASCII↔XML conversion|
|`property\_tests.rs`|259|6|Round-trip (simple/channels), counter correctness, Ch10 payload, `into\_owned()`|

### Benchmarks (1 file, 163 lines, 8 benchmarks)

|Benchmark|What It Measures|
|-|-|
|`parse/small`|Parse \~150 byte TMATS|
|`parse/medium`|Parse \~20KB TMATS (64 channels)|
|`parse\_owned/medium`|Parse + into\_owned|
|`serialize/medium`|Serialize 64-channel doc|
|`round\_trip/medium`|Parse + serialize|
|`validate/medium`|Run all 13 rules|
|`encode\_payload/medium`|Ch10 payload construction|
|`decode\_payload/medium`|Ch10 payload extraction|

### Fuzz Targets (3 targets, 2 seed files)

|Target|Strategy|
|-|-|
|`fuzz\_parse`|Arbitrary bytes → strict + lenient parser|
|`fuzz\_round\_trip`|Parse → serialize → reparse (checks no panic)|
|`fuzz\_ch10\_decode`|Arbitrary bytes → Ch10 payload decoder|

### Data (1 file, 532 lines)

|File|Description|
|-|-|
|`data/attributes.toml`|23 attribute metadata entries from Ch9 §9.5 tables across 106-04 to 106-17|

### Documentation (4 files, 1,188 lines)

|File|Audience|Content|
|-|-|-|
|`REQUIREMENTS.md`|Engineers, reviewers|182 requirements (13 L1 + 62 L2 + 107 L3) with spec refs and test cross-refs|
|`ARCHITECTURE.md`|Developers|Module DAG, design decisions, feature flags, types bridge migration plan|
|`API\_GUIDE.md`|Users|7 workflow walkthroughs with code examples|
|`FOR\_IRIG106\_DOCS\_REPO.md`|Ecosystem team|Cross-crate contracts (→ move to irig106-docs)|

### Build/Config (3 files, 326 lines)

|File|Purpose|
|-|-|
|`Cargo.toml`|Dependencies, features, bench config|
|`build.rs`|TOML→Rust codegen for attribute registry|
|`fuzz/Cargo.toml`|Fuzz workspace manifest|

## Feature Flag Matrix

```
Feature        │ Modules Enabled        │ Dependencies Added        │ Use Case
───────────────┼────────────────────────┼───────────────────────────┼──────────────────────
std (default)  │ std::error impls       │ —                         │ Normal usage
xml            │ xml.rs                 │ quick-xml                 │ XML TMATS format
serde          │ derives on model types │ serde, serde\_json         │ JSON/YAML export
wasm           │ wasm.rs                │ wasm-bindgen, serde-wasm  │ irig106-studio
rich-errors    │ miette impls           │ miette                    │ CLI pretty errors
generate       │ generate.rs            │ —                         │ TMATS from inventory
repair         │ repair.rs              │ —                         │ Auto-fix broken TMATS
full           │ all above except wasm  │ all optional              │ Development
```

## Validation Rule Inventory (13 rules)

```
Rule ID    │ L2 Req    │ Category        │ What It Checks
───────────┼───────────┼─────────────────┼────────────────────────────────────
TMATS-V001 │ VALID-001 │ Required Attr   │ G\\PN present
TMATS-V002 │ VALID-001 │ Required Attr   │ G\\106 present
TMATS-V010 │ VALID-005 │ Counter         │ G\\DSI\\N matches data source count
TMATS-V011 │ VALID-005 │ Counter         │ R-x\\N matches channel count
TMATS-V020 │ VALID-006 │ Cross-Ref       │ R-x\\CDLN-n → P/B/S data link name
TMATS-V030 │ VALID-003 │ Keyword         │ G\\DST-n value in {REC,TEL,MUL,PRE}
TMATS-V031 │ VALID-003 │ Keyword         │ R-x\\PDP-n value in {UN,PFS,TM}
TMATS-V032 │ VALID-003 │ Keyword         │ P-n\\D2 PCM encoding
TMATS-V033 │ VALID-003 │ Keyword         │ B-n\\BT bus type in {1553,A429}
TMATS-V040 │ VALID-007 │ Date Format     │ MM-DD-YYYY month/day range
TMATS-V050 │ VALID-004 │ MFS             │ G\\PN ≤ 32 characters
TMATS-V060 │ VALID-001 │ Required Attr   │ P-group: DLN, D1, D2, F1, F2, F3
TMATS-V061 │ VALID-001 │ Required Attr   │ B-group: DLN, BT
```

## Repair Rule Inventory (4 rules)

```
Rule ID    │ L2 Req      │ Auto-Fix │ What It Does
───────────┼─────────────┼──────────┼──────────────────────────────────────
TMATS-R001 │ REPAIR-001  │ Yes      │ Recompute G\\DSI\\N from data source count
TMATS-R002 │ REPAIR-001  │ Yes      │ Recompute R-x\\N from channel count
TMATS-R003 │ REPAIR-003  │ Yes      │ Normalize keywords to uppercase (PDP, D2, DST)
TMATS-R005 │ REPAIR-005  │ No       │ Detect orphan CDLN references (informational)
```

## Aggregate Metrics

|Metric|Value|
|-|-|
|Total files|38|
|Total lines (code + docs + config)|10,571|
|Source modules|14|
|Source lines|6,204|
|Test files|8|
|Test lines|1,915|
|`#\[test]` functions|99|
|Proptest properties|6|
|Fuzz targets|3|
|Criterion benchmarks|8|
|Validation rules|13|
|Repair rules|4|
|Registry attribute entries|23|
|L1 requirements|13 (all implemented)|
|L2 requirements|62 (57 implemented)|
|L3 requirements|107 (96 implemented)|
|Total requirements|182|

## Commands

```bash
# Build (default features)
cargo build

# Build with all features
cargo build --features full

# Run tests
cargo test
cargo test --features full

# Run benchmarks
cargo bench

# Run fuzz targets (requires nightly)
cargo +nightly fuzz run fuzz\_parse
cargo +nightly fuzz run fuzz\_round\_trip
cargo +nightly fuzz run fuzz\_ch10\_decode

# Build for WASM
cargo build --target wasm32-unknown-unknown --features wasm
```

