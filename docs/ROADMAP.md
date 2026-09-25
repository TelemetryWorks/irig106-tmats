# irig106-tmats Roadmap

> **This roadmap is forward-looking only.** Completed work is not tracked here —
> it lives in `CHANGELOG.md` (release history), `docs/L1-REQ.md` /
> `docs/L2-REQ.md` / `docs/L3-REQ.md` (the normative requirements), and
> `docs/TRACE-MATRIX.md` (verification status), all backed by git history.
>
> **Do not mint requirement IDs (`L2-*`, `L3-*`) in this file.** An ID is born
> only when its requirement is written in `docs/L2-REQ.md` / `docs/L3-REQ.md`;
> describe intended work in prose and let the requirement process own the IDs.
> Provisional IDs minted in a roadmap collide with real assignments once the
> requirement documents catch up.
>
> Likewise, do not record test counts, line counts, or requirement counts here.
> They drift the moment they are written; the trace matrix is the source of
> truth for coverage.

## Where the project stands

The original implementation was a prototype. A review against IRIG 106
Chapter 9 found that its core design could not be made correct incrementally:
it lost data on a parse/serialize round trip, mapped several P-group code names
to the wrong meanings, and hand-coded validation that disagreed with its own
attribute registry. It is preserved at the git tag `prototype-0` and will be
removed from `main` when the new implementation's first code lands.

The project is being rebuilt **documentation first**: nothing below is
implemented until the design phase has produced reviewed use cases,
architecture, ADRs, and L1/L2/L3 requirements.

## Queued for the next release (`[Unreleased]`)

`[Unreleased]` is emptied at each release cut; whatever sits above the most
recent dated section in `CHANGELOG.md` is the live queue.

## Design phase (before any new code)

Produced in this order, each reviewed before the next begins:

1. **Use cases and concept of operations** — who reads, writes, validates,
   and extends TMATS, from which sources (Chapter 10 setup records, standalone
   `.tmt` files, telemetry network metadata), and what they need back.
2. **Architecture and data flow** — the lossless ordered attribute store, the
   code-name grammar, the layered attribute registry, the validator, the
   edit/suggestion model, and the Chapter 10 setup-record boundary.
3. **ADRs** — one per decision already taken in review (listed under
   "Decisions to record" below) and any the architecture work raises.
4. **L1 → L2 → L3 requirements**, the trace-matrix script, and test-marker
   conventions.
5. **Standards baseline** — the RCC 106 archive (`TelemetryWorks/rcc-106-standards`)
   in place, so every registry entry can cite an edition, table, and row.

### Decisions to record as ADRs

- Rebuild rather than patch the prototype.
- The ordered attribute list is the single source of truth; typed accessors
  are views over it, and a parse/serialize round trip is byte-faithful by
  default.
- Owned storage (one backing buffer plus spans) instead of `Cow<'a, str>`
  lifetimes on public types.
- A layered attribute registry: built-in, spec-cited entries tagged by the
  edition that introduced, changed, or removed them, with user-supplied
  definitions and overrides layered on top.
- Validation is registry-driven, with severity policy and user rules; the
  document's edition is taken from a caller override, then `G\106`, then the
  Chapter 10 setup-record CSDW.
- No automatic repair: diagnostics may carry suggested edits, and the caller
  applies the ones it chooses.
- V (vendor) and X (extension) groups are first-class, with X attributes
  linked to the attribute they extend (Chapter 9 §9.5.13–9.5.14).
- The registry is generated into a checked-in source file by a script with a
  `--check` mode in CI; no `build.rs`.
- `Irig106Version`, the Chapter 10 data-type code, and the Computer-Generated
  Format 1 CSDW layout live in `irig106-types`, shared across the ecosystem.
- XML, WASM bindings, and the unused `std` / `rich-errors` features are
  removed until they can be done properly (see "Deferred features").

## Planned releases

| Version | Theme | Scope |
|---------|-------|-------|
| 0.1 | Read | Lossless parser for code-name (ASCII) TMATS and the Chapter 10 setup-record payload, including the CSDW format bit (ASCII vs XML). Lookup by code name. V and X groups preserved and linked. Byte-faithful writer. Edition detection. An `examples/` dump tool for trying real files. |
| 0.2 | Registry | Spec-derived registry for the baseline edition (106-24R1) covering every group in Chapter 9 (G, T, R, M, P, D, B, S, Q, C, H, V, X). Typed accessors generated from it. Channel resolution from R through P/B/S/Q to D and C. |
| 0.3 | Validate | Registry-driven validation: required/allowed-when rules, keywords, ranges, value types, counters and index consistency, cross-group references. User registries, severity policy, custom rules. |
| 0.4 | Editions | Registry deltas for every edition from 106-04 to 106-24R1, a generated `VERSION-DELTAS.md`, and edition-aware validation. |
| 0.5 | Generate and edit | Edit API (set/insert/remove), validation diagnostics carrying suggested edits with explicit apply, and a builder whose output passes validation. |

Testing strategy for every release: spec-sourced fixtures (starting with the
Chapter 9 Appendix 9-C example), property tests (never panics on arbitrary
bytes; the attribute multiset survives a round trip), and real-world TMATS
files as they become available.

## Deferred features

Removed from the prototype or not yet started. Each is recorded so it is not
forgotten; none has a committed version.

- **TMATS XML.** Chapter 9 §9.4.3 defines an XML form of TMATS as an XSD
  schema set published by RCC (the edition that introduced it is to be
  confirmed from the archive during the design phase). The prototype had an
  invented `<Tmats>` mapping that was not that schema and lost data on a round
  trip. To bring it back: obtain the official XSDs (to be mirrored in
  `rcc-106-standards`), generate the mapping from them, handle the documented
  differences from the code-name form (one C group per data link, no `\N`
  counters, expanded keywords, XML date formats, semicolons allowed in text),
  and prove round trips against the code-name form. The Chapter 10 setup
  record already signals XML payloads through the CSDW format bit, so 0.1
  detects them and reports them as unsupported rather than misparsing them.
- **WebAssembly bindings.** The prototype exposed parse/validate/serialize to
  JavaScript through `wasm-bindgen` inside this crate, intended for
  `irig106-studio`'s in-browser TMATS view. To bring it back: a separate
  `irig106-tmats-wasm` crate (`crate-type = ["cdylib", "rlib"]`) so the core
  crate's dependency graph and semver are unaffected.
- **`no_std` support.** The prototype declared a `std` feature but used `std`
  unconditionally. To bring it back: a real `no_std` + `alloc` build with an
  output sink abstraction, verified in CI on a `no_std` target. Only worth it
  if an embedded consumer appears.
- **Rich diagnostics.** The prototype's `rich-errors` feature pulled in
  `miette` with its `fancy` renderer but used nothing from it. To bring it
  back: source-span diagnostics rendered by a *binary* (CLI or example), not
  by the library, so library users do not inherit a terminal renderer.
- **Serde support.** Serialize/deserialize for the document and diagnostics,
  once the new model's public shape is stable.
- **CLI.** A `tmats` command-line tool (dump, validate, diff, apply fixes),
  built on the library after 0.3.
- **Data Display Markup Language and IHAL.** Chapter 9 §9.6–9.7 define two
  further XML schemas (data displays and instrumentation hardware). Out of
  scope unless a consumer needs them.
