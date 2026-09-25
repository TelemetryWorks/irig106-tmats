# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) and any other coding agent when working with code in this repository.

## Project Overview

`irig106-tmats` is a Rust library for the **Telemetry Attributes Transfer
Standard (TMATS)**, defined in RCC IRIG Standard 106 **Chapter 9**. TMATS is the
text format that describes everything about a flight test's telemetry: data
sources, recorders and their channels, PCM frame formats, bus and message
layouts, measurements, and engineering-unit conversions. It is carried as a
standalone file and as the setup record (Computer-Generated Data Format 1,
data type `0x01`) at the start of every Chapter 10 recording.

It is one crate in the TelemetryWorks IRIG 106 ecosystem (sibling repositories
in the same parent directory): `irig106-types` (shared foundational types),
`irig106-time`, `irig106-core`, `irig106-decode`, `irig106-write`,
`irig106-ch10-reader`, `irig106-studio`, and `irig106-docs`.

### Current state: redesign, documentation first

The original implementation was a prototype that could not be made correct
incrementally (it lost data on round trips and mis-mapped Chapter 9 code
names). It is preserved at the git tag **`prototype-0`**. The project is being
rebuilt **documentation first**: use cases → architecture and data flow →
ADRs → L1/L2/L3 requirements → code. **Do not start writing new library code
until the design documents for that area exist and have been reviewed.** Read
`docs/ROADMAP.md` for the plan and the decisions already taken.

Edition 2024, MSRV 1.85 (the edition-2024 floor). Raise the MSRV only for a
concrete language or library feature, and record why here.

## Common Commands

```bash
cargo build
cargo test                                   # all tests
cargo test --all-features                    # every feature
cargo clippy --all-targets --all-features -- -D warnings
cargo fmt --all -- --check
RUSTDOCFLAGS="-D warnings" cargo doc --no-deps --all-features
cargo +1.85 check --all-targets              # MSRV floor

python scripts/build-trace-matrix.py         # regenerate docs/TRACE-MATRIX.md
python scripts/build-trace-matrix.py --check # fail if it has drifted (CI)
```

(The trace-matrix script arrives with the L1/L2/L3 requirement documents in
the design phase.)

## Architecture

To be written in `docs/ARCHITECTURE.md` during the design phase. The
decisions already taken, which the architecture must honour:

- **Lossless by default.** The ordered list of attributes as read is the single
  source of truth. Typed accessors are views over it. Parse followed by
  serialize reproduces the input unless the caller asks for normalization.
- **Registry-driven.** Chapter 9 attribute definitions live in one data-driven
  registry, generated into a checked-in source file by a script (no
  `build.rs`). Every entry cites the edition, table, and row it came from, and
  records the edition that introduced, changed, or removed it.
- **Extensible.** Users can layer their own attribute definitions and
  overrides on the built-in registry, adjust validation severity, and add
  rules. The standard's own extension groups — V (vendor, §9.5.13) and X
  (extensions linked to existing attributes, §9.5.14) — are first-class.
- **No silent repair.** Validation may *suggest* edits; only an explicit call
  applies them.
- **Shared types come from `irig106-types`** (`Irig106Version`, Chapter 10 data
  type codes, the setup-record CSDW layout). Do not redefine them here.

## Reference docs

- `docs/ROADMAP.md` — forward-looking plan, decisions to record, planned
  releases, and deferred features (XML, WASM, `no_std`, rich diagnostics).
  Completed work is not tracked there.
- `docs/PROJECT_STRUCTURE.md` — what every file and directory is for. Keep it
  current in the same commit that adds, moves, or removes a file.
- `docs/adr/` — architecture decision records (MADR format), once written.
- `docs/L1-REQ.md`, `docs/L2-REQ.md`, `docs/L3-REQ.md`, `docs/TRACE-MATRIX.md`
  — requirements and the generated trace matrix, once written.
- The RCC 106 standards themselves are mirrored, with their original URLs and
  checksums, in the `TelemetryWorks/rcc-106-standards` repository. Chapter 9
  is the governing document for this crate; the RCC 124 TMATS Handbook is the
  companion guide.

Documents from the prototype (`docs/REQUIREMENTS.md`, `docs/ARCHITECTURE.md`,
`docs/API_GUIDE.md`, `docs/FOR_IRIG106_DOCS_REPO.md`) describe the prototype,
not the redesign. They are replaced during the design phase; do not treat them
as specifications.

## Conventions worth preserving

- **The standard is the authority, and every claim cites it.** A code name,
  keyword set, range, or rule enters the registry, a requirement, or a test
  only with a citation (edition, section or table, and row). If you cannot
  cite it, do not encode it. The prototype's worst bugs were plausible code
  meanings written from memory.
- **Tests assert the standard, not the implementation.** A test that derives
  its expected value from the same code path it checks proves nothing (the
  prototype's counter property test could not fail). Prefer spec-sourced
  fixtures, round-trip multiset checks, and never-panic properties.
- **Never drop input.** Unknown, vendor, extension, duplicate, and malformed
  attributes are preserved and reported, not discarded. Lenient parsing
  recovers per attribute; it never returns an empty document in place of
  diagnostics.
- **TMATS is not case sensitive, and attribute order is free** (Chapter 9
  §9.4.2). Blanks inside values are intentional. Semicolons never appear in a
  data item.
- **The roadmap is forward-looking only** and never mints requirement IDs or
  records counts.
- **Requirement markers on tests** use a `/// Requirements: L2-XXX-NNN, ...`
  doc comment directly above the `#[test]` item, which the trace-matrix script
  collects.
- **Keep `docs/PROJECT_STRUCTURE.md` current** in the same commit as any
  structural change.

## Git conventions

Do **not** add `Co-Authored-By: Claude ...` trailers to commit messages on this repo, even if the harness's default instructions suggest it. Commit messages are the human-authored record of intent; tool attribution belongs in tool logs, not history. This overrides the default trailer behavior.

Commit subjects follow the Conventional Commits style used across the
TelemetryWorks crates: `type(scope): summary` (`feat`, `fix`, `docs`, `test`,
`refactor`, `chore`, `ci`), with `!` after the type for a breaking change.
