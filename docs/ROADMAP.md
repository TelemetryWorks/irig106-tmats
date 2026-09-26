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
3. **ADRs** — written for the decisions taken so far (`docs/adr/`); two
   architecture ADRs (0020, 0021) are proposed pending the architecture
   review.
4. **L1 → L2 → L3 requirements** — L1 drafted, the trace-matrix script and
   test-marker convention in place (CI-checked); L2 and L3 follow the
   architecture review.
5. **Standards baseline** — the RCC 106 archive (`TelemetryWorks/rcc-106-standards`)
   in place, so every registry entry can cite an edition, table, and row.

### Decisions

Decisions taken so far are recorded in `docs/adr/` (index in
`docs/adr/README.md`). Still to decide: whether `irig106-cli` mounts the
`tmats` commands directly, which would mean `irig106-tmats-cli` also exposes
its command implementations as a library under the same lockstep and semver
rules (ADR-0011).

## Specification coverage plan

A section-by-section review of IRIG 106-24R1 Chapter 9 (the baseline
edition) against `docs/ARCHITECTURE.md`, `docs/adr/`, and `docs/L1-REQ.md`
(2026-09-26) found the chapter covered at the section level except for the
items below. They are additions and clarifications, not changes of
direction; each is folded into the architecture, the ADRs, and the L1
requirements before L2 is written. New requirement IDs are assigned in
`docs/L1-REQ.md`, not here.

Checked and not a gap: the Q group's "the first item is numbered 0"
(§9.5.10) refers to element and field offsets, which are values; Q code-name
indices are 1-based like every other group's, so the index-contiguity rule
stands.

### Coverage work items

1. **Prove attribute-level completeness mechanically.** No requirement yet
   says the built-in registry defines every code name in Tables 9-2 through
   9-11 of each supported edition. Add an L1 registry requirement for it,
   verified by a CI check that compares the Chapter 9 table extractor's
   output with the registry: every code name is either defined or explicitly
   excluded with a reason. This check is the standing proof of coverage and
   keeps it current when a new edition is published.
2. **Treat the H group as an extension mechanism.** §9.5.12 reserves H for
   user-defined airborne-hardware attributes, tied to G by `H\TA`, with
   `H\ST-n` determining how the rest are interpreted — the same pattern as
   the V group. Extend ADR-0008 and the extensibility requirement to cover H
   (grouping by test item and system type; user-supplied definitions).
3. **Decide the scope of Appendices 9-D and 9-E, and of conversions.**
   Appendix 9-D (floating-point formats) is referenced by `C-d\FPF` and
   S-group enumerations; validating those names is already covered, while
   interpreting raw bits is `irig106-decode`'s work and should be stated as a
   non-requirement. Appendix 9-E (derived-parameter grammar) governs the
   value of `C-d\DPA` when `C-d\DPAT` is `A`. **Owner decision needed:**
   validate derived-algorithm syntax in this library, or declare it out of
   scope. Evaluating any C-group conversion to engineering units is stated as
   out of scope either way (it is implied today but not written).
4. **Follow every link, not only the pictured ones.** §9.5.1 b says "Not all
   valid paths are shown" and that all are documented in the "Links to /
   Links from" fields (for example the R-group filtering and overwrite links
   into P, D, and B). Reword the channel-view requirement to follow every
   registry link.
5. **State how nonprintable characters are interpreted.** §9.4.1: TMATS is
   7-bit ASCII and "nonprintable characters will be discarded by the
   destination agency". Keep them in the bytes (lossless), ignore them for
   meaning, and report them.
6. **Warn on `OTH` without an explanation.** Many enumerations offer "OTH –
   Other, define in comments", and §9.2 places nonstandard values in each
   group's comments. **Owner decision needed:** whether a missing comment is
   a warning by default.
7. **Check the §9.4.2 recommendations.** Blanks should not appear in code
   names, keywords, or link values, and link and measurement names should use
   only capitals, digits, and `_`. **Owner decision needed:** warnings by
   default, matching the decision on recommended maximum lengths.
8. **Declare the cover sheet out of scope.** Appendix 9-B concerns exchanging
   physical media; record it as a non-requirement.
9. **Complete the citations.** Appendix 9-F cites IEEE 1588-2008, RFC 1305,
   and RCC 200-16; add RFC 1305 and RCC 200-16 to the standards archive's
   "cited but not mirrored" list (RCC 200 may instead be mirrored, since it is
   an RCC document).

Also carried into L2 (already covered at L1): the §9.4.2 scientific-notation
regular expression, line breaks as insignificant between attributes, one
mission configuration per document, and the D-group location types and
subframes removed as of 106-11 as an edition delta.

### Team design review (in progress)

The team reviewed `docs/ARCHITECTURE.md`, the ADRs, and `docs/L1-REQ.md`
against 106-24R1 Chapter 9 (Appendices 9-A to 9-F) and Chapters 6 and 11.
Their summary: keep the lossless document, owned byte buffer, generated
registry, explicit edits, and library/CLI separation; the main gaps are in
semantic validation, appendix coverage, and setup-record handling. They
raised seven priorities plus further comments; the text is recorded verbatim
in `docs/research/2026-09-26-team-design-review.md`. Each priority is
checked against the archived standard, mapped to the existing documents and
the coverage items above, and planned here. Items are added as they arrive.

**T1. Make the registry an executable specification, with reviewed
interpretations.** Confirmed against 106-24R1: `C-d\DPNO` has its default
only in prose ("Default is 1.", Table 9-11; the chapter has 107 `Default:`
fields and further prose-only defaults), and `G\106` is "Required when:
Always" (Table 9-2), which a pass over present attributes cannot report
missing. Also found: `C-d\DPNO`'s condition names `C\DCT` without an index,
so the occurrence it refers to needs an explicit interpretation. Mapping:
strengthens ADR-0004, ADR-0006, the registry requirement (L1-REG-002), and
the validation requirements (L1-VAL-001/002); overlaps coverage item 1;
nothing conflicts. Plan:

1. *Two layers per registry entry.* A **source layer** holding the table row
   exactly as printed (parameter, code name, usage-attribute text, definition
   prose) with its citation, produced by the table extractor and never edited
   by hand; and an **interpretation layer** holding reviewed, executable
   rules — conditions in a small defined condition language, the default and
   where it came from (Default field or prose), typed ranges, links — with
   reviewer, review date, and a note wherever the interpretation is not
   literal. A CI check requires every source row to have an interpretation or
   an explicit "not yet interpreted" status; each interpretation stores a
   hash of its source text so a changed row (for example in a new edition) is
   flagged for re-review.
2. *Condition semantics, defined once in a new ADR:* occurrence scope (same
   occurrence, linked occurrence, or document-wide, declared per rule);
   whether a condition follows a link; a missing or invalid dependency makes
   the condition "cannot evaluate", reported as its own finding, never
   silently true or false; whether conditions see defaulted values, declared
   per rule.
3. *Four validation passes* replacing ADR-0006's single pass: present
   attributes (known, allowed, range and type); presence (Required-when and
   R/R Ch 10 Status per applicable occurrence — this detects a missing
   `G\106`); counters and index contiguity; relationships (links resolve,
   keys unique). Each finding names its pass.
4. *Effective values* without changing the document: explicit (value and
   location), defaulted (value and citation), missing, invalid (raw text and
   reason), or ambiguous (for example, conflicting duplicates). Defaults are
   never inserted into the stored bytes (ADR-0002).
5. *Proof and tests:* the completeness check (coverage item 1) also covers
   prose defaults; every reviewed interpretation has tests; the first
   regression tests are "`G\106` absent is reported" and "`C-d\DPNO` absent
   is defaulted to 1".

Documents affected: two new ADRs (two-layer registry with condition
semantics; validation passes and effective values). ADR-0006 keeps its
decision — registry-driven validation with severity policy and user rules —
and its status line becomes "mechanism superseded by" the new validation
ADR, following the rule that records are superseded, not rewritten;
ADR-0004 gains a similar pointer. L1-REG-002 is strengthened, and new L1
requirements cover missing-required detection and effective values.
**Decided (owner, 2026-09-26): a two-person rule for interpretation
reviews.** Every interpretation records two different people: its author and
an independent reviewer. An interpretation drafted with tooling or an AI
assistant still needs both people; the tool counts as neither. The CI check
from step 1 enforces it: an interpretation whose author and reviewer are
missing or identical fails, and a re-review after a source change needs both
again.

## Planned releases

| Version | Theme | Scope |
|---------|-------|-------|
| 0.1 | Read | Lossless parser for code-name (ASCII) TMATS and the Chapter 10 setup-record payload, including the CSDW format bit (ASCII vs XML). Lookup by code name. V and X groups preserved and linked. Byte-faithful writer. Edition detection. `G\SHA` compute/verify and the flex signature. First crates.io release of both crates. **`tmats` CLI**: `show` in raw, tree, and channel-summary form (the three `idmptmat` formats), `extract`, `checksum` (`G\SHA`, `--flex`), `verify`, `stamp`; every setup record in a recording; plain text and JSON output. |
| 0.2 | Registry | Spec-derived registry for the baseline edition (106-24R1) covering every group in Chapter 9 (G, T, R, M, P, D, B, S, Q, C, H, V, X). Typed accessors generated from it. Channel resolution from R through P/B/S/Q to D and C; `tmats show` summary uses the resolved links. |
| 0.3 | Validate | Registry-driven validation: required/allowed-when rules, keywords, ranges, value types, counters and index consistency, cross-group references. User registries, severity policy, custom rules. `tmats validate`. |
| 0.4 | Editions | Registry deltas for every edition from 106-04 to 106-24R1 (including renamed codes such as `R-x\DST-n` → `R-x\CDT-n`, to be confirmed), a generated `VERSION-DELTAS.md`, and edition-aware validation. |
| 0.5 | Generate and edit | Edit API (set/insert/remove), validation diagnostics carrying suggested edits with explicit apply, and a builder whose output passes validation. `tmats diff` and applying chosen fixes. |

Testing strategy for every release: spec-sourced fixtures (starting with the
Chapter 9 Appendix 9-C example), property tests (never panics on arbitrary
bytes; the attribute multiset survives a round trip), regression tests for
every defect found in the reference tools, fuzzing, and — locally only —
the irig106.org sample recordings and real program files, compared
semantically against `idmptmat` (`docs/TEST-DATA.md`).

## Deferred features

Removed from the prototype or not yet started. Each is recorded so it is not
forgotten; none has a committed version.

- **TMATS XML.** Chapter 9 §9.4.3 defines an XML form of TMATS as an XSD
  schema set published by RCC; `rcc-106-standards` holds the schemas from
  106-07 onward. The prototype had an invented `<Tmats>` mapping that was not
  that schema and lost data on a round trip. To bring it back: generate the
  mapping from the official XSDs, handle the documented
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
- **GUI.** An `igDisplayTMATS`-style graphical viewer (raw, summary, tree,
  checksum status) belongs in `irig106-studio`, built on the same library
  calls as `tmats`; nothing GUI-related enters this repository.
- **Data Display Markup Language and IHAL.** Chapter 9 §9.6–9.7 define two
  further XML schemas (data displays and instrumentation hardware). Out of
  scope unless a consumer needs them.
