# Architecture Decision Records

MADR-format records of the decisions that shape `irig106-tmats` and the
`tmats` CLI, in the style of `mie-decoder`'s `docs/adr/`. Each record states
the problem, the options considered, the choice, and its consequences. A
decision is changed by a new record that supersedes the old one; records are
not rewritten.

**Status:** `accepted` — decided by the project owner; `proposed` — part of the
architecture proposal (`docs/ARCHITECTURE.md`) awaiting the owner's review.
`coordinator-decided` records coordinator direction, which the owner may override.
A record refined or partly superseded by a later one keeps its text; only its
status line points to the later record.

| ADR | Decision | Status |
|-----|----------|--------|
| [0001](0001-rebuild-rather-than-patch-the-prototype.md) | Rebuild the crate rather than patch the prototype | accepted |
| [0002](0002-lossless-ordered-attribute-store.md) | The ordered attribute list is the single source of truth | accepted |
| [0003](0003-owned-buffer-with-spans.md) | Owned byte buffer with spans instead of borrowed lifetimes | accepted |
| [0004](0004-layered-spec-cited-registry.md) | A layered, spec-cited, edition-tagged attribute registry | accepted; refined by 0022 |
| [0005](0005-generated-registry-without-build-rs.md) | Generate the registry into checked-in source with a script; no `build.rs` | accepted; extended by 0022 |
| [0006](0006-registry-driven-validation-with-policy.md) | Registry-driven validation with severity policy and user rules | accepted; mechanism superseded by 0023 |
| [0007](0007-suggested-edits-not-automatic-repair.md) | No automatic repair: suggested edits, applied only by the caller | accepted; refined by 0029 |
| [0008](0008-first-class-vendor-and-extension-groups.md) | Vendor (V) and extension (X) groups are first-class | accepted; extended to H by 0027 |
| [0009](0009-shared-types-from-irig106-types.md) | Shared IRIG 106 types come from `irig106-types` | accepted |
| [0010](0010-library-performs-no-io.md) | The library performs no I/O | accepted |
| [0011](0011-lockstep-library-and-cli-workspace.md) | One workspace; library and `tmats` CLI released in lockstep | accepted |
| [0012](0012-publishing-and-release-binaries.md) | Publish both crates together; `tmats` binaries via `cargo-dist` | accepted |
| [0013](0013-hand-rolled-cli-argument-parsing.md) | Hand-rolled argument parsing for the CLI | accepted |
| [0014](0014-tmats-checksums.md) | `G\SHA` over the original bytes; flex signature as a labelled compatibility option | accepted; refined by 0029 |
| [0015](0015-remove-xml-wasm-and-unused-features.md) | Remove XML, WASM, and the unused features until done properly | accepted |
| [0016](0016-edition-strategy.md) | Parse every edition; baseline 106-24R1; validate 106-04 … 106-24R1 | accepted; partly superseded by 0028 |
| [0017](0017-standards-archive-repository.md) | Mirror the RCC 106 standards in a separate repository, as release assets | accepted |
| [0018](0018-reference-tools-and-test-data.md) | Reference tools are oracles, not authorities; real data stays local | accepted |
| [0019](0019-cli-minimal-chapter-10-reader.md) | The CLI carries a minimal Chapter 10 packet reader until `irig106-core` | accepted; refined by 0025; reader responsibility superseded by 0036 |
| [0020](0020-correctness-guard-rails.md) | Correctness guard rails in the code and the tests | proposed |
| [0021](0021-single-pass-scanner.md) | A single-pass scanner that indexes, checksums, and reports as it reads | proposed; scanner rule added by 0024 |
| [0022](0022-registry-source-text-and-reviewed-interpretations.md) | The registry holds source text and reviewed, executable interpretations (review rule revised by 0033) | accepted; extended by 0026 and 0027; interpretation review superseded by 0033 |
| [0023](0023-validation-passes-and-effective-values.md) | Validate in four passes over effective values | accepted; refined by 0026 |
| [0024](0024-derived-parameters-parsed-not-evaluated.md) | Derived parameters parsed, validated, and described here; evaluated in `irig106-decode` | accepted; interpretation review superseded by 0033 |
| [0025](0025-setup-record-assembly-contract.md) | Setup records assembled from packet fragments in the library; packets sliced by the CLI's reader | accepted; record ending superseded by 0031; edition-code wording refined by 0032; interpretation review superseded by 0033; reader/checksum ownership superseded by 0036 |
| [0026](0026-counter-scopes-link-namespaces-and-comparison.md) | Counters declare their scope, links their namespace, selector, and cardinality; keys unique per attribute; case-insensitive comparison | accepted; extended by 0027 |
| [0027](0027-interpretation-register-and-three-source-relationships.md) | An interpretation register; relationships from Links to, Links from, and §9.5.1 b; the H group | accepted; interpretation review superseded by 0033 |
| [0028](0028-edition-declarations-and-validation-basis.md) | TMATS edition and recording-format version kept apart; validation basis labelled; fallback and compatibility checks named | accepted; RCCVER wording refined by 0032 |
| [0029](0029-transactional-edits-and-checksum-stamping.md) | Edits are transactions over defined targets; a stamp hashes the final bytes (F1 revised by 0034; F2/F3 open) | accepted; F1 superseded by 0034; F2–F3 open |
| [0030](0030-core-independent-of-tmats.md) | `irig106-core` does not depend on `irig106-tmats`; plain data through `irig106-types`; the packet check lives in `irig106-tmats` | accepted |
| [0031](0031-setup-record-completeness.md) | A setup record is complete, incomplete, or ambiguous — each with its basis | accepted; interpretation review superseded by 0033 |
| [0032](0032-rccver-read-literally.md) | RCCVER is read literally; later editions' reuse of a code is a labelled note; the fallback is labelled policy | accepted |
| [0033](0033-owner-signoff-and-independent-ai-interpretation-review.md) | One owner sign-off plus an independent AI check approves an interpretation | accepted (owner, 2026-11-21) |
| [0034](0034-coordinator-release-foundation-decisions.md) | W1-W3 and F1/F7 release-foundation decisions | coordinator-decided (2026-11-21); W1 superseded by owner decision 0035; others unchanged |
| [0035](0035-virtual-workspace-with-library-crate-and-cli-app.md) | Virtual root, library under crates/, CLI under apps/, exact shared dependency | accepted (owner, 2026-11-21); supersedes 0034 W1 only |
| [0036](0036-core-reader-and-borrowed-data-spans.md) | One core reader; unchanged core evidence; borrowed non-contiguous DATA input with zero extra payload copy | accepted owner architecture (2026-11-22); coordinator-decided integration (2026-11-26) |
| [0037](0037-extract-destination-and-findings-policy.md) | Extract overwrite/identity policy and byte-exact extraction under findings; --force/--strict | accepted (owner, 2026-11-27) |

New records take the next number and use the same front matter
(`status`, `date`, `decision-makers`).
