# irig106-tmats — Architecture

> **Status: draft for review (design phase).** This document replaces the
> prototype's architecture notes (preserved at the git tag `prototype-0`).
> Section 2 records the architecture proposal **word for word** as it was
> made in the design discussion on 2026-09-25; later sections will refine it
> into modules, data flow, and interfaces. The decisions are recorded in
> `docs/adr/` (ADR-0020 and ADR-0021 capture this proposal, pending review). Use cases are in `docs/USE-CASES.md`.

## 1. Direction from the project owner

> "I dont necessarily want to follow the legacy tools Pipeline sketch. Lets
> make our architecture better, faster, less error prone. What can we do
> better here?"

The "legacy tools pipeline sketch" is the one proposed in the owner's
ChatGPT conversation (recorded verbatim, with verification notes, in
`docs/research/2026-09-25-irig106org-tools.md`):

```text
Chapter 10 packet
      │
      ▼
TMATS packet 0x01
      │
      ▼
raw TMATS text
      │
      ▼
TMATS lexer/parser
      │
      ├── G records
      ├── R records
      ├── M records
      ├── P records
      ├── B records
      ├── C records
      ├── D records
      └── ...
      │
      ▼
structured TMATS model
      │
      ├── raw view
      ├── tree view
      ├── channel mapping
      └── semantic lookup
```

Its overall flow (packet → text → parse → model → views) is sound; its "one
record type per group" model is the design that failed in the prototype, and
is not adopted.

## 2. The proposal (verbatim, 2026-09-25)

### A better architecture than the legacy pipeline

Here's what `idmptmat` and `irig106lib` actually do, from their source:
1. Copy the packet's text.
2. Split it into lines with `strtok`, into fixed 2 KB buffers.
3. Make an upper-cased copy of every line.
4. Match each code name against hand-written, per-group decoder files (`i106_decode_tmats_r.c` and so on) into linked lists of C structs that hold only the fields someone thought to decode.
5. Walk those lists to print.

Every stage loses information or invites bugs. Unknown fields are dropped, the linked-list walk hangs (D1), the signature re-reads the text on its own path with its own bugs (D5, D6), only the first packet is read (D2), and the first error aborts.

Here's what I'd do instead, organised around one principle: **read the bytes once, never copy them, and let the spec's own tables drive everything else.**

```text
bytes ──► 1 scan ──► Document (immutable, one owned buffer)
                      ├─ items: [ {code span, value span, parsed key} ]   flat, in order
                      ├─ keys:  interned, case-folded structured code names
                      ├─ index: key → item ids   (built during the scan)
                      ├─ digest: G\SHA SHA-256   (computed during the same scan)
                      └─ diagnostics: [ {span, kind} ]   (never aborts)
                                │
     spec-cited registry ───────┼──► link graph (from the spec's "Links to/from")
     (static tables +           │
      user overlay)             ├──► typed views: Recorder, Channel, PcmFormat…
                                │     (borrow the document, parse values lazily)
                                ├──► validator (rules = registry data + policy)
                                └──► edits = patch list over the original bytes
                                      └──► writer: original bytes + patches
```

**Faster**
1. **One pass, no copies.** A byte scanner (`memchr` for `:` and `;`, vectorised) records spans into one owned buffer. There are no per-line allocations and no upper-cased copy of the file. Case-insensitivity is handled once, when each code name is folded into a compact key (group, occurrence, interned path, indices).
2. **Index while scanning.** Every lookup is a hash probe; pattern lookups like "every `R-1\TK1-n`" hit a secondary index. There are no linked lists to walk, which rules out the whole class of bug behind D1.
3. **Checksum during the same pass.** `G\SHA`'s excluded span is known at scan time, so the SHA-256 is computed on the fly without re-reading the file (fixing D5 and D6 by construction).
4. **Parse values only when asked.** Views hand back spans and decode numbers, keywords and dates only when someone reads them. Showing the raw text or a tree never parses a value.
5. **A static registry with no start-up cost.** It's generated as sorted, compile-time tables (binary search or a perfect hash), unlike the prototype's `OnceLock<HashMap>` built at runtime. User definitions sit in a small runtime overlay that's checked first.
6. **The CLI reads large recordings efficiently.** It memory-maps the file and skips from packet header to packet header, reading only `0x01` payloads, so finding every setup record in a multi-gigabyte file costs very little.

**Less error-prone**
7. **One source of truth, generated from the spec.** Views, links and validation all come from the spec-cited registry, whose "Links to / Links from / Allowed when / Range" fields are Chapter 9's own. The hand-written per-group decoders, where both the legacy code and the prototype went wrong, disappear.
8. **Values are bytes, not `str`.** A Latin-1 `°` can't fail a parse. Decoding is explicit (`as_str_lossy`, `as_latin1`) and happens at the view.
9. **Errors are data, with locations.** Reading never fails partway through; it returns the document plus diagnostics. "Strict" becomes a policy you apply to the diagnostics, not a separate parser mode, so there's no second code path to drift.
10. **Byte-faithful by construction.** Edits are a patch list over the original bytes, so output is the original with the patches applied. Undo, diffing and "did the checksum change?" all fall out of the patch list.
11. **Guard rails in the code:**
    - `#![forbid(unsafe_code)]`;
    - newtype IDs (`ItemId`, `Span`) so indices can't be mixed up;
    - no panicking indexing;
    - `#[non_exhaustive]` on public enums;
    - an immutable, `Send + Sync` document that's safe to share across threads.
12. **Proven, not asserted:**
    - fuzzing from the first commit;
    - property tests: the scan never panics, a round trip is byte-identical, and the index agrees with the item list;
    - the D1–D7 regression tests;
    - semantic comparison against `idmptmat` on real files, run locally.

**Better for users**
13. **Every setup record, as a sequence:** `SetupRecords` yields each one in turn, each with its CSDW facts.
14. **Stable machine output:** the CLI's JSON is a documented schema, so scripts and `irig106-cli` can rely on it.
15. **Performance budget:** a benchmark suite on the sample recordings from day one, reporting throughput next to `idmptmat` on the same files. I'd set the numeric target after the first measurement, not guess it now.

Each of these becomes an ADR and part of `docs/ARCHITECTURE.md`, which is the next design document.

## 3. The architecture in pictures

Hand-authored SVGs in `docs/diagrams/`; they follow GitHub's light or dark
theme on their own. Each shows one mechanism.

![irig106-tmats in the TelemetryWorks ecosystem](diagrams/system-context.svg)

*Context.* The `tmats` CLI is the only part that opens files: it
memory-maps a Chapter 10 recording and hands each setup-record payload (or a
standalone TMATS file's bytes) to the library. CLI and library are one
workspace released in lockstep; the sibling crates, including the complete
`irig106-cli`, depend on the library; shared types come from
`irig106-types`; `irig106-core` will later replace the CLI's packet reader.

![Legacy pipeline compared with irig106-tmats](diagrams/legacy-vs-new.svg)

*What changes.* The legacy path copies and upper-cases the text, decodes only
the fields someone hand-wrote, walks linked lists, and re-reads the text for
the checksum; each red box is where it loses data or has a defect (D1–D7).
The new path scans once into a lossless document and derives everything
else from it and from the spec registry.

![Data flow through irig106-tmats](diagrams/data-flow.svg)

*Data flow* (updated for T1, section 4). The Document and the registry meet
in the effective-value resolver and the condition evaluator; views and the
validator read everything through them. The registry holds source text and
reviewed interpretations, with the user's overlay checked first. The link
graph is built before validation, and validation runs four passes. The only
paths that change anything are amber: the user's overlay and the caller's
choice of suggested edits. The writer is the original bytes plus the chosen
patches.

![One buffer, spans, keys, and an index](diagrams/document-model.svg)

*Document model.* The bytes are kept once; each item is a pair of spans plus a
parsed, case-folded key; an index maps keys and patterns to items; the
`G\SHA` digest covers every byte except the `G\SHA` item (Chapter 6
§6.2.3.11 f) and is computed in the same pass.

![How TMATS groups link by value](diagrams/link-graph.svg)

*Link graph.* The ties between groups that Chapter 9 §9.5.1 b defines, each
labelled with the attribute value that carries it. The link graph and the
channel views (UC-05) are built from these, as recorded per attribute in the
registry's "Links to / Links from" fields; X attributes attach to the
attribute they extend (§9.5.14). The H (airborne hardware) group's ties are
not listed in §9.5.1 b and are added when its tables are transcribed.

![Edits are patches; the checksum is suggested](diagrams/edits-and-checksum.svg)

*Edits and the checksum.* A chosen edit becomes a patch over the original
bytes, and the writer copies every other byte unchanged. The checksum of the
result no longer matches the stored `G\SHA`, so the library reports it stale
and suggests a stamp edit; `G\SHA` changes only if the caller applies that
suggestion (`tmats stamp` does this on request).

## 4. Refinement T1: executable registry, validation passes, effective values

From the team design review, priority T1 (`docs/ROADMAP.md`, "Team design
review"; text in `docs/research/2026-09-26-team-design-review.md`). Applied
2026-09-26 at the owner's direction; recorded in ADR-0022 and ADR-0023. The
shape of the architecture is unchanged — bytes → single scan → lossless
Document → views and validator ← registry. Two components are deepened and
one is added. This section **amends points 4, 5, 7, and 9 of section 2**,
which stays as the verbatim record of the original proposal.

### 4.1 Registry: source text plus reviewed interpretations (amends point 5)

Each registry entry has two layers:

- **Source layer** — the Chapter 9 table row exactly as printed: parameter
  name, code-name pattern, the usage-attribute text, and the definition prose,
  with its citation (edition, table, page). Produced by the table extractor;
  never edited by hand. Prose-only facts survive here, such as `C-d\DPNO`'s
  "Default is 1." (Table 9-11), which has no `Default:` field.
- **Interpretation layer** — what the code executes: Allowed-when and
  Required-when as expressions in a small, defined **condition language**;
  the default and its origin (a `Default:` field or the prose); typed ranges;
  links. Each interpretation records its **author and an independent
  reviewer** (two people; tooling or AI drafting counts as neither), the
  review date, a note wherever it is not literal (for example "`C\DCT` in
  `C-d\DPNO`'s condition means `C-d\DCT` of the same occurrence"), and a
  **hash of the source text** it interprets.

Both layers are generated into the static tables of point 5; conditions are
compiled, so the registry still has no runtime set-up cost. The user overlay
supplies entries of the same shape.

![Building the registry](diagrams/registry-pipeline.svg)

*Building the registry.* The extractor produces the source layer from the
archived edition; people author interpretations and a second person reviews
each; the generator compiles both into checked-in tables. Four CI checks
guard it: every table code name and prose default is in the registry
(completeness); every source row is interpreted or explicitly marked "not
yet interpreted"; an interpretation whose source text hash changed is
flagged for re-review; and every interpretation names two different people.

### 4.2 Condition semantics

Defined once, not per rule:

- **Occurrence scope.** Each condition declares whether its operands refer to
  the same occurrence, a linked occurrence, or the whole document.
- **Links.** A condition may follow a registry link (for example from a D
  group to the P group it describes); the link graph is therefore built
  before conditions are evaluated.
- **Missing or invalid dependencies.** A condition whose operand is missing
  or invalid evaluates to **cannot evaluate**, reported as its own finding —
  never silently true or false.
- **Defaults.** Each condition declares whether it sees defaulted values or
  only explicit ones.

The **condition evaluator** implements these rules and returns true, false,
or cannot-evaluate, with the operands it used.

### 4.3 Effective values (amends point 4)

Views and the validator read values through an **effective-value resolver**
between the Document and the registry. Every read returns one of five
states, and the stored bytes are never changed (ADR-0002):

| State | Carries |
|-------|---------|
| Explicit | the value and its location in the bytes |
| Defaulted | the default and the citation it came from |
| Missing | nothing present and no default |
| Invalid | the raw text and why it failed |
| Ambiguous | the conflicting candidates (for example disagreeing duplicates) |

Values are still parsed only when asked for.

### 4.4 Validation in four passes (amends point 7)

Validation runs four passes, in order, each over the Document, the link
graph, and the registry through the condition evaluator and the resolver:

1. **Present attributes** — each is known, allowed where it appears, and
   within its range and type.
2. **Presence** — every Required-when and R/R Ch 10 Status rule is evaluated
   for each occurrence it applies to. This pass reports attributes that are
   absent, such as a missing `G\106` ("Required when: Always", Table 9-2);
   a pass over present attributes alone cannot.
3. **Counters and indices** — each `\N` counter agrees with its entries, and
   indices run "with no missing values" (§9.5.1 a).
4. **Relationships** — links resolve and key values are unique.

Every finding names its pass. Severity policy, user rules, and the Chapter 10
recorder profile apply as before (ADR-0006). Diagnostics gain one kind,
**cannot evaluate** (amends point 9).

### 4.5 Cost

The presence pass iterates registry rules per group occurrence rather than
only the attributes present. The rules are compiled and static and TMATS
documents are small, so the cost is expected to be negligible; the benchmarks
of point 15 measure it.

## 5. Decisions this architecture must honour

Recorded as ADRs in `docs/adr/` (0001–0019, 0022–0023 accepted): the lossless ordered attribute store as the
single source of truth; owned storage instead of borrowed lifetimes; the
layered, spec-cited registry generated by a script (no `build.rs`);
registry-driven validation with severity policy and user rules; no automatic
repair (suggested edits, explicit apply); first-class V and X groups; shared
ecosystem types from `irig106-types`; the two-crate lockstep workspace
(`irig106-tmats` and `irig106-tmats-cli`, binary `tmats`); a minimal Chapter
10 packet reader inside the CLI until `irig106-core` provides one; `G\SHA`
over the original bytes and the flex signature as a labelled compatibility
option; no I/O in the library; the edition strategy; the two-layer registry
with reviewed interpretations and defined condition semantics (ADR-0022);
validation in four passes over effective values (ADR-0023). The original
proposal is captured in ADR-0020 and ADR-0021 (proposed).

## 6. To be written

- Module structure and public API sketch (document, scanner, keys and
  index, registry and overlay, link graph, condition evaluator,
  effective-value resolver, views, validator, edits and writer, checksums,
  Chapter 10 setup-record adapter, CLI).
- The condition language's grammar and the interpretation file format.
- Data flow per use case (UC-01 … UC-17).
- Error and diagnostic model; JSON output schema for the CLI.
- Performance and memory budget, set after the first benchmark.
