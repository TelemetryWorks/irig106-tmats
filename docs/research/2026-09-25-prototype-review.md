# Prototype review (2026-09-25)

Research record of the review that led to the redesign. The prototype it
describes is preserved at the git tag `prototype-0`. Three reviewers read the
crate in parallel, each covering a separate part; their reports are
reproduced **word for word** in sections 2–4. Section 1 is the summary given
to the project owner; section 5 lists what was confirmed independently.
Decisions drawn from this review are in `docs/ROADMAP.md` ("Decisions to
record as ADRs") and `docs/ARCHITECTURE.md`.

The review was requested by the owner in these words: "I need to question
everything in this repo because you are smarter now and I want to make sure
we are on the correct path for a great project."

## 1. Summary given to the owner (verbatim)

**`build.rs`:** No, you don't need it. It generates a registry file that nothing uses (the `include!` is commented out). If it were switched on, it would silently disable validation, because it doubles backslashes. Its 44 entries have also drifted from the 30 written by hand. My recommendation is to delete it and generate a checked-in registry file with a script under `scripts/`. That script would get a `--check` mode for CI, the same pattern as the trace matrix.

**What the review found.** Three reviewers read the crate in parallel, and I spot-checked their key spec claims.
- **Parsing then serializing loses data.** Counters, some indexed values, some `R-x\EV`/`IDX` attributes, and any value that doesn't parse as a number are dropped. Lenient mode can return an empty document with no errors. Latin-1 bytes such as the degree sign `°` fail the whole parse.
- **Several code meanings are wrong.** P-group D1/D2/F1–F3 are mixed up (D1 is the PCM code, D2 the bit rate, frame sync is MF3–MF5). The keyword sets for `G\DST`, `C\DCT` and `R\CDT` look invented, and the generator writes hex into `CDT` instead of mnemonics like `PCMIN` or `1553IN`. Many tests assert the wrong behaviour, so they pass.
- **The design is the root cause.** There's one hand-written struct per group for hundreds of Chapter 9 attributes. Validation is hand-coded per field and ignores the registry.
- **Other problems:**
  - The CSDW ignores bit 9, the ASCII/XML format flag.
  - `Irig106Version` stops at 106-17 and duplicates the enum in irig106-time.
  - The XML support is an invented schema, not RCC's TMATS XSD.
  - `repair` changes data by default, including rewriting counters, which destroys the evidence that a file was truncated.
  - The `std` and `rich-errors` features do nothing.

## 2. Reviewer A — parse, model, serial, error (verbatim)

## TMATS crate review: parse, model, serial, error

I confirmed the findings below with a probe binary in the scratchpad that links the crate by path. I built into the target dir you gave me and did not touch the repo.

### Ranked findings

**1. Critical: parse followed by serialize silently loses data.** (model.rs routing, serial.rs `serialize_*_group`)
- **Counters are dropped.** Fields get parsed into slots that are never serialized, or only written back when a child map is non-empty:
  - Input `G\POC\N:1;G\DSI\N:3;R-1\N:4;B-1\NBS\N:2;D-1\MN\N:5;P-1\MF\N:4;` comes back with every counter gone.
  - `num_points_of_contact`, `num_minor_frames`, `num_buses` and `num_measurements` are never emitted.
  - `points_of_contact`, `drives`, `subcarriers`, `subframes`, `word_definitions`, `measurements` and `messages` are never filled by the parser.
- **Typed fields drop anything that doesn't parse.** `G\OD:2024-01-15`, `R-1\TK1-1:0x10` and `P-1\D1:NRZ-L` all vanish. `T-1\CF:2250.500` comes back as `2250.5`, and an `R-1\N:1` appears that was never in the input.
- **Arms that match a code but don't store it discard the attribute.** Examples: `R-1\EV\N:3`, `R-1\IDX\IT:TIME`, and `G\DST` or `R-1\TK1` with no index. Also, `route_r_attribute` matches `IDX`/`EV` with any second segment, so `R-1\IDX\IT:TIME` overwrites `index_enabled` with `None`. `R-1\EV\E:T;R-1\EV\N:3;R-1\IDX\E:T;R-1\IDX\IT:TIME;` serializes to just `R-1\ID:R;`.
- **Single slots take the last of several indexed values.** `D-1\MLN-1:A;D-1\MLN-2:B;` becomes `D-1\MLN:B` (the value A is lost and the index is renamed).
- **The docs overclaim.** ARCHITECTURE.md §"extra" says round-tripping is lossless. It isn't.
- **Fix:** keep every attribute in one ordered store. Typed fields become views over that store, never the only copy.

**2. Critical: several P/T/M/R code assignments don't match Chapter 9.** (`route_p_attribute`, `route_t_attribute`, `route_m_attribute`)
- **P group:** in the standard, D1 is the PCM code, D2 the bit rate, D4 polarity, D5 auto-polarity correction, F1 the common word length, F2 the MSB/LSB transfer order, F3 parity. Frame sync is MF3, MF4 and MF5. The model instead has D1 as bit rate, D2 as encoding, F1 as words per frame, F2 as bits per word, F3 as the sync pattern, and invents `F3\L`.
- **M group:** the model uses `BSG`, but real files use `M-x\BSG1`, so the typed field never matches.
- **T group:** `CF/MT/PW/AT/AP` don't look like Chapter 9 T-group codes to me, which are RF-style. `R-x\RI1` is the recorder manufacturer, not a description. `C-x\EU` and `B-x\BT` also need checking. All of these are from memory; confirm against the 106-22 tables.
- **The tests enshrine the errors.** `parse_p_group_pcm_format` and `round_trip_multi_group` assert `D1=1000000`, `D2=NRZ-L`, `F3=sync`.
- **Fix:** generate the code tables from the spec. Don't hand-write them.

**3. Critical: lenient mode can return an empty document with no diagnostics.** (`parse_with_options`)
- Any tokenizer error makes `tokenize` return `Err`, and lenient mode then returns `TmatsDocument::empty()`.
- Both `G\PN:TEST;G\TN:<0xFF>;G\106:17;` and `...;junk` produce `pn=None`, `diags=0`.
- **Fix:** have the tokenizer return tokens and errors together, and keep going past bad records.

**4. Major: tokenizer and code-name grammar problems.**
- **Non-UTF-8 bytes fail the whole parse in strict mode.** One Latin-1 byte (such as `0xB0`, a degree sign, common in real files) is enough. The error is labelled `InvalidAscii` but the check is actually for UTF-8. Values should be raw bytes, or decoded as Latin-1.
- **Spaces before the colon are rejected.** `G\PN :X;` fails. Trim instead.
- **`parse_group_prefix` only looks at the first character.** `RX-2\ID` goes to R; `R-A\ID` and `R\ID` both default to occurrence 1. `R-A\ID:X;RX-2\ID:Y;` merges into `R-1\ID:Y`, and X is lost with no duplicate error. `R-01` and `R-1` also collide silently. A bad index should be an error.
- **Nested indices are lost.** `parse_path_segment` uses `rfind('-')`, so `D-1\MN-1-1` becomes the name `MN-1` with index 1. `PathSegment` needs a list of indices.
- **Legitimate repeats count as duplicates.** Detection compares the raw uppercased string, so strict mode rejects repeated `G\COM` and the free-standing `COMMENT:` form. That contradicts `comments: Vec`.
- **`COMMENT:` is misfiled.** It is routed as C-group occurrence 1 and moved into C output.
- **`from_g106_str` doesn't know 19, 20, 22 or 23.**
- **Dead code:** `raw_upper` and the `parts.is_empty()` check in `parse_code_name`.

**5. Major: the serializer changes user data.**
- Counters are recomputed from map sizes. `serialize_fixes_incorrect_counter` explicitly checks that `DSI\N:5` becomes 1. That hides a validation error, and it's the job of `repair`, not the serializer.
- Output uses a fixed group order. Comments go to the end of their group, and `COM` placement changes. Original order and formatting can't be reproduced.
- **Fix:** default to faithful original-order output. Make canonical order and recomputed counters opt-in.

**6. Major: the `std` feature does nothing.**
- `lib.rs` has no `#![no_std]`, and `std::borrow`, `std::io::Write`, `std::vec::IntoIter` and `HashSet` are used without any guard.
- The `rich-errors` feature pulls in miette, which nothing uses.
- **Fix:** remove both features, or actually support no_std + alloc (`core::fmt::Write` sink, `alloc::`).

**7. Major: tests mostly check the code against itself.**
- `parse_lenient_recovers_valid_data` accepts both `Ok` and `Err`.
- `prop_serialized_counters_correct` can't fail, because the serializer derives the counters from the same maps it checks against.
- The property generators only produce R/G inputs that are known to parse. There is no "never panics on arbitrary bytes" property, no idempotence check (`ser(parse(ser(x))) == ser(x)`), and no multiset comparison of input vs. output attributes. That last one would have caught finding 1 immediately.
- There are no real-world or spec-sample fixtures.

**8. Minor: error design.**
- `parse` returns `TmatsErrors` wrapping `TmatsError`, even though only `Parse` can ever occur. It should return `Vec<ParseError>` or a dedicated `ParseErrors`.
- Error codes come from `kind as u8`, so they shift if the enum is reordered. `InvalidDelimiter` shows up as `P000`.
- Display prints `[ERROR] ...`. The Rust convention is a lowercase message with no severity.
- The enums have no `#[non_exhaustive]`.
- Lenient mode reports parse errors as validation `Diagnostic`s with a hard-coded "§9.4.2" reference.
- `attribute_count()` counts groups, not attributes.

**9. Minor: `unsafe from_utf8_unchecked` in `tokenize`.** It's sound, because every byte was checked as ASCII-graphic just before, but it isn't needed. The earlier `parse_group_prefix` unwrap and the `s[2..]` slice are also safe for the same reason. I found no panics in the files I reviewed.

**10. Minor: `Cow` + SmallVec isn't worth the cost.**
- Documents are KB to a few MB and get parsed once. Lifetimes then spread through every type, query.rs needed split lifetimes because SmallVec makes the model invariant over `'a`, `into_owned` is boilerplate on every struct, and borrowed serde deserialize doesn't work anyway.
- Parsing typed values already allocates (`to_ascii_uppercase` for every attribute, `format!` in the serializer).
- **Fix:** use an owned document by default: `String` or `Box<str>` (or `Arc<str>` / a single backing buffer with span offsets).

### Model design recommendation

Hand-writing one struct per group can't keep up with Chapter 9. It has hundreds of attributes per group, they drift between versions, and most of this crate's bugs come from that hand-mapping.

Recommended direction:
- **Core store:** an ordered list of `Attribute { code: CodeName, value, span }`, with an index map from normalized code to position.
- **Typed access:** zero-cost view types such as `doc.recorder(1)?.channel(3)?.data_type()`, generated from a version-aware registry (version.rs already exists).
- **Trade-off:** compile-time field names are weaker, but you get lossless round-trips, easy support for new standard versions, and far less code.

### Recommended direction
- Make the ordered attribute list the source of truth and generate typed accessors from a spec table. Retire the `extra` / hand-written struct design.
- Guarantee byte-faithful round-trips by default. Move counter recomputation and canonical ordering into `repair`/`normalize` options.
- Rewrite the tokenizer to accept raw or Latin-1 bytes, trim around `:`, and recover per record in lenient mode. Parse code names into a group, an occurrence, and segments that each hold several indices. Reject malformed occurrences.
- Replace the self-confirming tests with spec-sourced fixtures plus proptests for "never panics" and "the attribute multiset survives a round trip". Fix the P-group tests to use the spec's meanings.
- Remove the lifetimes (owned strings) and either implement no_std properly or remove the `std` and `rich-errors` features.

## 3. Reviewer B — validate, repair, query, generate, version, types_bridge, build.rs (verbatim)

## irig106-tmats review: validate, repair, query, generate, version, types_bridge, build.rs

I built and tested with `CARGO_TARGET_DIR=...\tmats-review-b` and edited nothing. The scoped tests pass (47 in total: validate/query 13, repair 10, registry 24). Several of them assert behaviour the spec doesn't support.

### Critical

**C1. The `generate` feature (and so `full`) does not compile.**
- In `generate.rs::TmatsBuilder::build`, `ch.data_type as u8` fails with E0605. `DataTypeCode` has an `Other(u8)` variant, so it can't be cast with `as`.
- Nothing in CI builds with `--features generate`.
- Fix: add a `DataTypeCode::to_u8()`. Also emit the CDT keyword (e.g. `PCMIN`), not hex `"09"`.

**C2. The generated registry is broken, so switching `include!` on would silently disable validation.**
- `build.rs::parse_kv` strips quotes but doesn't process TOML escapes. `escape_backslash` then doubles the backslashes again.
- The generated `version_generated.rs` (44 entries) contains `code_name_pattern: "G\\\\PN"`, which is `G\\PN` at runtime.
- Every `reg.get("G\\PN")` would miss. All registry-driven rules (V030–V033, V050) do `None => return vec![]`, so they'd go quiet with no error.
- The fail-silent `match reg.get(..)` pattern is dangerous on its own. A missing registry entry should panic in tests.

**C3. P-group code names look wrong, and parse/validate/registry all agree on the wrong meaning.**
- From memory of 106-17 Table 9-5: `P-d\D1` is the PCM code (NRZ-L…) and `D2` is the bit rate. `P-d\F1` is common word length, `F2` is word transfer order (MSB/LSB), `F3` is parity. The sync pattern is `MF5`, words per minor frame is `MF1`, sync length is `MF4`.
- The crate has D1 = bit rate, D2 = encoding, F1 = words/frame, F2 = bits/word, F3 = sync.
- Result: real files get "invalid encoding" errors on their bit rate, and `RequiredPGroupAttrs` (V060) demands the wrong fields.
- `parse.rs` maps the same way, so every layer is consistently wrong.

**C4. Several keyword sets look invented.** These are from my memory of the spec; please confirm against 106-22 before acting.
- `G\DST-n` = REC/TEL/MUL/PRE. Real files use `STO`, `RF`, `TAP`, `OTH`, so real Ch10 TMATS (`G\DST-1:STO;`) gets flagged as an error.
- `C\DCT` = PAIR/COEF/TABL/POLY/FORM. The spec uses 3-letter codes: PRS, COE, NPC, DER, DIS, OTH…
- `R\CDT` has `UARIN`, `A429IN`, `IMAIN`, `16PP`. I believe these should be `UARTIN`, `429IN`, `IMGIN`, and there are others.
- `P\D2` MFS is 5, but `RNRZ-L` is 6 characters. `FSKM`/`DBP-*` look dubious.
- `B-n\BT` is modelled per group; in the spec it's indexed per bus (`B-x\BT-i`).
- Recommendation: treat registry contents as unverified. Every entry should cite a table and row from a specific 106 edition.

### Major

**M1. Version model is incomplete and duplicated.**
- `types_bridge::Irig106Version` stops at 17. 106-19, 106-22 and 106-23 exist, and `from_g106_str("22")` returns `None` with no diagnostic.
- It's `#[repr(u8)]`, exhaustive, and has no `Unknown`.
- `../irig106-time/src/version.rs` already has a second, better `Irig106Version` (Pre07, V07…V23, `Unknown(u8)`), and its CSDW mapping reaches 0x0F. The two are incompatible.
- The `G\106` keyword list includes `"08"`, which the enum lacks.
- Recommendation: put one `Irig106Version` (non_exhaustive, with `Unknown`) plus `DataTypeCode` into `irig106-types` and delete both local copies. `GroupPrefix` and the registry types stay in tmats. `ChannelId` is unused (only re-exported in the prelude); move it to types or drop it.

**M2. Validation is hand-coded per field, not driven by the registry.**
- `RequiredTag`, `ValueType`, `is_valid_for`, `required_for_ch10` and `deprecated_in`/`removed_in` (never populated) aren't used by any rule.
- `ctx.source` (`TmatsSource`) is never read. `ValidationProfile::Ch10Required` is an empty match arm.
- `applicable_versions()` is `&[]` on every rule, and `severity()` is ignored because diagnostics hardcode severity.
- There's no public way to add rules; `default_rules()` is private and rebuilt on every call.
- `RequiredGVersion` errors on every version, but the registry says `G\106` was introduced in 07.
- Recommendation: keep a raw attribute list in the document and run one generic pass per attribute: match to registry pattern → version validity → keyword (per version) → MFS → value type → required-by-context. Keep hand-written rules only for cross-reference checks. Expose a public `Validator` with `with_rule`/severity overrides and `&'static str` metadata.

**M3. Important spec cross-checks are missing.**
- `G\DSI-n` ↔ `R-x\ID`/`T-x\ID` linkage.
- Channel ID (`TK1`) uniqueness, and channel ID 0 being reserved.
- `D-x\DLN` → P.
- `C\DCN` → a D/B measurement name.
- `CHE`-disabled channels.
- Required-for-Ch10 set (`CDT`, `TK1`, `ID`).
- Counter rules compare `len()` to N and don't check that indices 1..=N exist (`DSI-1, DSI-3` with N=2 passes).

**M4. The query resolution chain is partly wrong.**
- In `query.rs::resolve_channel`, C-groups are matched to `DGroup.measurement_list_name`. They should link through the D measurement names (`D-x\MN-y-n`) and B measurement names.
- Bus channels never resolve measurements.
- DLN matching is case-sensitive here but case-insensitive in `CrossGroupRefCdln`, so a document can pass validation and still fail to resolve.
- It returns the first `TK1` match across all recorders, and `FormatRef` has no group index.
- `diff()` goes through serialize → `split(';')` → `HashMap`. That drops repeated keys (`COM`) and gives nondeterministic output order.

**M5. The repair engine is risky as designed.**
- Defaults mutate: all `auto_fix_*` are true and `dry_run` is false.
- Setting `R-x\N`/`G\DSI\N` to match the content destroys the best evidence of truncation: the declared count may be the correct value.
- Bug: `CounterRepair*::apply` inserts a counter when it's absent (`old != Some(actual)` with `old = None`), and there's no matching finding.
- Rule dispatch uses `id.contains("R001")`. `auto_fix_missing_required`/R004 does nothing.
- `repair_dry_run` ignores its unused `clone`/`opts`.
- No provenance is recorded (no G\COM note or revision bump), and nothing prevents rewriting an embedded Ch10 setup record.
- Recommendation: rename it "fix suggestions". Default to dry-run, opt in per rule, and output a reviewable edit list (patch) against the source rather than mutating the typed model. Never auto-change counters; report them.

**M6. `TmatsBuilder` produces output that fails the crate's own validator.**
- Its doc says "Build and validate", but it never validates.
- B stubs have no `BT`; P stubs lack the required fields.
- `G\DST` gets the invented `REC`, `R-1\ID:"GENERATED"` doesn't match `G\DSI-n "CHAN_x"`, and there's no `G\OD`.
- Group indices are sparse (keyed by channel index), and `G\106` is emitted for 04/05.
- A builder failure returns a `ParseError`. The API takes `&str` rather than `impl Into<String>`.

### Minor
- `build.rs` has a hand-rolled TOML subset: no escapes, no arrays (keywords are comma strings), and it `panic!`s on unknown versions.
- `RequiredTag::from_str`/`ValueType::from_str` should implement `FromStr`.
- `registry().get` returns `Option<&&AttrMeta>`, and there's no lookup from a concrete name like `R-1\PDP-3` to its pattern.
- Date check accepts Feb 31.
- The `std` feature is fake: `HashMap` and `OnceLock` are used unconditionally.
- `lib.rs` docs use `//`, not `//!`, so the quick start is never doctested.

### Tests
- They encode the wrong spec: `registry_required_for_ch10` asserts G\PN is Ch10-required, and the P tests use D1 = bit rate.
- `repair_reduces_validation_errors` asserts `<=`, which passes even when nothing changed.
- No test compiles `generate` or exercises the generated registry.

### On build.rs (Q4)
Delete it. Generate the registry into a checked-in file (e.g. `src/registry_data.rs`) with an `xtask`, and add a test that regenerates it in memory and asserts it's byte-identical (`--check`). Why:
- Every downstream crate pays to compile and run a build script, which also blocks pipelining.
- Generated code is invisible in code review and on docs.rs.
- The current setup has already drifted (30 hand-written entries vs 44 in the TOML) and has the escaping bug from C2.
- The TOML needs per-version keyword/MFS rows and spec citations anyway.

### Recommended direction
- **Spec audit first.** Rebuild `attributes.toml` from 106-17/22 tables with a citation on every entry and per-version keywords/MFS. Fix the P-group D/F/MF mapping across parse, validate and the registry.
- **One registry, one generic validator.** Checked-in generated table plus a CI `--check`; no build.rs. Keep hand-written rules only for cross-reference checks, and make profiles/source/version actually do something.
- **Unify the ecosystem types.** Move `Irig106Version` (through V23, plus `Unknown`) and `DataTypeCode` into `irig106-types`, and remove the copies in tmats and irig106-time.
- **Make repair conservative.** Suggestions or patches, dry-run by default, never silently rewrite counters; fix the insert-without-finding bug.
- **Get generate compiling and self-consistent.** Output must pass `validate`, add a CI job for `--all-features`, and replace tests that assert invented behaviour.

## 4. Reviewer C — ch10, xml, wasm, lib, manifest, fuzz, benches, docs (verbatim)

## irig106-tmats review: ch10, xml, wasm, lib, manifest, fuzz, benches, docs

The biggest problems: several advertised features don't compile, the Chapter 10 header word (CSDW) is missing a bit, and the XML support is a made-up format. Build output is from `CARGO_TARGET_DIR=...tmats-review-c` on rustc 1.98. `cargo test --features xml,repair` passes (98 tests). `--no-default-features` builds.

### Critical
1. **`serde`, `wasm`, `generate` and `full` don't compile, and neither does the fuzz crate.** `cargo check --all-features` fails:
   - `SmallVec<[PathSegment;4]>: Serialize/Deserialize` is missing because smallvec isn't built with its `serde` feature.
   - `generate.rs` (TmatsBuilder data_type formatting) does `ch.data_type as u8`, which is illegal because `DataTypeCode` has an `Other(u8)` variant.
   - `fuzz/Cargo.toml` enables `generate`, so `cargo check --manifest-path fuzz/Cargo.toml` fails. The wasm32 check fails on the same serde errors.
   - **Fix:** add `smallvec/serde` to the `serde` feature. Add a `DataTypeCode::as_u8()` method. Add a CI feature matrix (cargo-hack `--each-feature`).
2. **The CSDW doesn't handle bit 9 (`SetupRecordCsdw` in ch10.rs).** In Ch10 106-09+ and Ch11 106-17+, bit 9 is the setup-record format (0 = ASCII, 1 = XML), and bits 10–31 are reserved. What is correct: bits 0–7 are the Ch10 version and bit 8 is SRCC (config change).
   - The code treats bits 9–31 as reserved. `encode` always writes 0 in bit 9, even for XML payloads.
   - `decode_setup_payload` ignores the bit and always parses as ASCII.
   - The test `csdw_reserved_bits_zero` locks the bug in, and `FOR_IRIG106_DOCS_REPO.md` publishes it as the contract with irig106-write.
   - `PayloadEncoding` is dead code: it is never used and isn't in the prelude.
3. **The version enum stops at 106-17 (`types_bridge::Irig106Version`).** For version codes 0x0D and up (106-19/22/23 in irig106-time's mapping), `from_csdw_version` returns `None`. `from_g106_str("22")` also returns `None`. So every current recording gets no version hint, the same as a pre-106-07 file.
   - This also duplicates `irig106-time/src/version.rs`, a different enum with the same name. That one has `Pre07`, `V19`, `V22`, `V23` and `Unknown(u8)`, so the two are incompatible.
   - The `#[repr(u8)]` values here (4, 5, 7, 9, 11…) are neither CSDW codes nor year numbers.
   - **Fix:** one enum in irig106-types, `#[non_exhaustive]`, with `Unknown(u8)`. Check the 0x0D–0x0F codes against the Ch11 table; I'm not sure whether 106-20 took a code.

### Major
4. **xml.rs is a made-up mapping, not the RCC TMATS XML schema.**
   - It writes an un-namespaced `<Tmats>` root with invented element names (`GeneralInformation`, `RecorderAttributes`, `PCMFormatAttributes`…) and no XSD validation. It does not check the root element.
   - Only about 30 typed fields are covered; raw and extra attributes are dropped. The parser ignores T, M, D and S groups and most P/R fields that the serializer writes (Polarity, WordsPerFrame, SyncPattern, Description, IndexEnabled, Enabled). XML round-trips lose data.
   - Parsing depends on element order: a `Channel` before `RecorderID` is silently dropped. A P or C element before `DataLinkName`/`MeasurementName` is dropped or attached to the previous group.
   - `KEYWORD_EXPANSIONS` is one global table for all fields, so values are expanded without context. It has no Top Secret, and single letters like T/F/M/S/N/R get expanded in the wrong fields.
   - Bad entity escapes are swallowed (`unescape().unwrap_or_default()`).
   - README/API_GUIDE claim "Tmats.xsd".
   - **Recommendation:** remove it or mark it experimental and unpublished until it is generated from the real XSD.
5. **Features that do nothing:**
   - `rich-errors`/miette is never referenced in src, and turning on `fancy` in a library is wrong anyway. Remove it.
   - `serde_json` is never used. Remove it; `wasm` then pulls in less.
   - `std` is fake. There is no `#![no_std]`, and `version.rs` uses `std::sync::OnceLock` and `HashMap`; xml uses `std::io`. Drop the feature or really gate it.
   - `generate` and `repair` are wired correctly: the modules and error variants are cfg-gated.
6. **A feature changes an enum's shape.** `PayloadEncoding::Xml` exists only under `cfg(feature="xml")`, and the enum isn't `#[non_exhaustive]`. A downstream exhaustive `match` breaks when some other crate turns on `xml`, so features aren't additive. There is no `#[non_exhaustive]` anywhere across 70 pub types. The CSDW struct has pub fields, so adding the format bit (item 2) is already a breaking change.
7. **WASM:**
   - Cargo.toml has no `crate-type = ["cdylib","rlib"]`, so wasm-pack can't produce a module from this crate.
   - `serialize_tmats_ascii`'s docstring says it takes JSON; it takes ASCII.
   - **Recommendation:** move this into a separate `irig106-tmats-wasm` crate. That keeps wasm-bindgen out of the core's dependency graph and semver, and gives the cdylib its own manifest.
8. **README code blocks are broken.** Escapes inside fenced blocks (`irig106\_tmats`, `\\\\PN`, `\&doc`, `\*`) render literally. PROJECT_STRUCTURE.md has the same problem in tables.

### Minor
9. **Docs don't match the code:**
   - Requirement counts: README says 62 L2 / 107 L3; REQUIREMENTS.md has about 87 L2 and 134 L3 unique IDs.
   - "Serde: JSON/YAML" — there is no YAML.
   - API_GUIDE lists XML and wasm as "(planned)" though both exist, and its `features = ["generate","repair","serde"]` example doesn't compile.
   - PROJECT_STRUCTURE test counts per file (10/16/8/14/24/10/11 + 6 proptests) are right, but line counts are stale (parse_tests is 327, not 313; repair_tests is 199, not 217).
   - The CSDW layout in FOR_IRIG106_DOCS_REPO.md is wrong (see item 2).
   - FOR_IRIG106_DOCS_REPO.md says it belongs in irig106-docs, which exists next door. Move it there and link to it.
10. **Placement of CSDW code.** The payload/packet split is reasonable. But the CSDW bit layout is a packet concern, and irig106-ch10-reader already detects 0x01 on its own. Put a `ComputerGenerated1Csdw` in irig106-core (or types) and have tmats consume it; keep only the payload↔document conversion here.
11. **Manifest:**
    - `[profile.release] lto/codegen-units` in a library has no effect on dependents and slows `cargo bench`. Remove it.
    - quick-xml 0.36 is behind: 0.37/0.38 changed the Text/unescape API, which xml.rs relies on.
    - criterion 0.5 is outdated (0.6+/0.7 moved `black_box` to `std::hint`).
    - Edition 2024 would make the MSRV 1.85. The code uses nothing past OnceLock (1.70): no let-chains, `is_none_or` or `LazyLock`. So after the move, `rust-version = "1.85"` is honest; today's floor is really about 1.70 (miette 7 / OnceLock), not a verified 1.75.
12. **Public API surface is too wide:**
    - Every module is `pub`, including `types_bridge`, version-registry internals, and `xml::expand_keyword`/`collapse_keyword`/`tmats_date_to_xml`.
    - The prelude re-exports about 60 items.
    - Make `types_bridge` a re-export shim, and make the registry and XML helpers `pub(crate)`.
13. **`detect_config_change`** compares serialized bytes. That's fine for "anything changed", but it isn't what SRCC means (a recorder configuration change), and it allocates twice. Document that, or compare semantically.
14. **Fuzz and benches:**
    - The three targets are reasonable panic probes, but there is no XML fuzz target, and the fuzz crate is currently broken (item 1).
    - The bench inputs use `R-1\CDT-n:09` (hex codes). Real `CDT` values are mnemonics (`PCMIN`, `1553IN`, …), and `generate.rs` also emits hex (`{:02X}`). That's a spec deviation that bleeds into ch10 and XML DataType.
15. **Panics:** none found in ch10.rs, wasm.rs or xml.rs. Slicing is length-checked, the xml `unwrap()`s follow an `is_none` guard, and there is no recursion.

### Recommended direction
- Fix the build first: repair smallvec/serde and the `DataTypeCode` cast, then add a cargo-hack feature-matrix CI plus the wasm32 and fuzz checks.
- Put one `Irig106Version` (non_exhaustive, with `Unknown`, covering 106-19 through 23) and the CSDW layout (including bit 9) in irig106-types/core, and delete both local copies.
- Remove miette, serde_json, the fake `std` feature and the release profile. Move WASM to `irig106-tmats-wasm`.
- Take xml.rs out of the published API (or behind an `unstable-xml` feature) until it's generated from the real RCC XSD.
- Mark pub enums and structs `#[non_exhaustive]` (or give them private fields plus constructors) before 0.1 is published. Then fix the README escapes and the doc claims, and move FOR_IRIG106_DOCS_REPO.md to irig106-docs.

## 5. Confirmed independently, and caveats

- **Confirmed against the archived 106-23 Chapter 9 P-group table** (text of `rcc-106-standards` asset `106-23/Chapter9.pdf`, extracted with `pdftotext -raw`): `P-d\D1` "PCM CODE … Define the data format code"; `P-d\D2` "BIT RATE … Data rate in bits per second"; `P-d\F1` "Number of bits in common word length"; `P-d\F2` "Define the default for the first bit transferred in normal …"; `P-d\F3` "PARITY … Normal word parity"; `P-d\MF1` "Specify the number of words in a minor frame"; `P-d\MF3` "SYNC TYPE … Define minor frame synchronization type"; `P-d\MF4` "Specify the minor frame synchronization pattern length"; `P-d\MF5` "Define minor frame synchronization pattern in bits". The prototype's P-group mapping was wrong on every one of D1, D2, F1, F2, F3. The reviewers' other claims marked "from memory" (T-group codes, keyword sets for `G\DST`, `C\DCT`, `R\CDT`) are **not yet** confirmed and must be checked against the archived tables before any is relied on.
- **Confirmed by build:** the `serde`, `wasm`, `generate`, and `full` features did not compile; after they were made to compile, `generate_validates_with_zero_errors` failed (generator output fails the crate's own validator) — now `#[ignore]`d with that reason at `prototype-0`.
- **Confirmed by reading:** `build.rs` output is never included (`include!` commented out in `version.rs`); `R-x\CDT-n` takes keywords (`TIMEIN`, `1553IN`, `PCMIN`, …), not hex codes.
- **CSDW bit positions** (bit 8 SRCC, bit 9 format) and the edition that introduced bit 9 are to be confirmed from Chapter 10/11 in the archive before they are recorded in `irig106-types`.
