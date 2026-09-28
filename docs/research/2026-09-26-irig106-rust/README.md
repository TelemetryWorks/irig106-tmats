# What `irig106-rust` held, kept before it is removed (2026-09-26)

The owner excluded `irig106-rust` from the ecosystem ("this repo will
probably get deleted. But if there is anything in the repo of note then we
should capture it before we dispose of it", owner-direction entry 29). This
folder keeps what it held of note, copied verbatim from its last commit
(`57b48d6`, 2026-01-23, `TelemetryWorks/irig106-rust`), with a review.
The material concerns the whole ecosystem, not only TMATS; `irig106-docs`
is its natural home if it moves.

## What the repository was

A crate named `irig106_timecodes` ("IRIG-106 timecode parsing/formatting
utilities with strong security posture"), edition 2021. Its code is a stub:
a `Timecode` type with four kinds (IRIG-B, IEEE 1588, day-of-year seconds,
time of day) and a parse-error enum — work that belongs to `irig106-time`.
Its value is in its documents and configuration.

## Kept here

| File | From | What it is | Worth keeping because |
|------|------|------------|-----------------------|
| `irig106_srs_l1.md` | `docs/l1_system_requirements_specifications/src/` | A Level 1 requirements specification for a complete IRIG 106 Chapter 10 library (draft, 2026-01-21) | The only ecosystem-wide requirement set; its non-functional requirements are not stated anywhere else (below) |
| `SECURITY.md` | root | Vulnerability reporting, supported versions, security controls, incident response | A ready security policy for every `irig106-*` repository; its contact is a placeholder (`security-contact@example.com`) |
| `CODE_ANALYSIS.md` | root | Per-change checks (fmt, clippy, coverage floor, cargo-deny, cargo-audit, semgrep, CodeQL, unsafe policy), continuous hardening (Miri, sanitizers, fuzzing), release integrity (SBOM, provenance, signatures) | A complete assurance checklist |
| `SUPPLYCHAIN.md` | root | Supply-chain practices | Short statement of the practices above |
| `SLSA1.md` | `notes/` | How to reach SLSA level 1 with GitHub Actions build provenance (`actions/attest-build-provenance`) and how to verify it | A worked recipe for release provenance |
| `ROADMAP.md` | root | A checklist of licensing, governance, reproducibility, scanning, CI hardening, release, and SLSA items | The same practices as a plan |
| `deny.toml` | root | cargo-deny policy: advisories, allowed licenses, bans, allowed sources | Directly reusable |
| `semgrep-rust.yml` | root | 14 semgrep rules: forbid `unsafe` and `transmute`; no panics, `unwrap`/`expect`, `dbg!`, or printing in library code; no blocking in async; no dead code in libraries | Directly reusable; matches this project's rules (no panics, no printing outside `main.rs`) |
| `cargo-config.toml` | `.cargo/config.toml` | Cargo aliases and `-C embed-bitcode=no` for smaller, cleaner release artifacts, with rationale | Reference for release builds |

Not kept: the timecode stub (`src/lib.rs`), an empty mdBook skeleton, two
one-line conformance READMEs, a commented-out CI file, a release script,
and license and editor files.

## Review of the requirements specification

**Worth carrying into the ecosystem** (none is stated in any `irig106-*`
repository today):

- **REQ-L1-100/101** — minimal allocation, streaming over large files,
  zero-copy where possible (consistent with `irig106-tmats`
  `docs/TMATS-IN-CHAPTER-10.md` section 4.13).
- **REQ-L1-102** — a throughput target: "minimum 100 Mbps sustained" for
  reading and writing.
- **REQ-L1-103** — Windows, Linux, and macOS.
- **REQ-L1-105/106** — safe Rust; no panics on malformed input.
- **REQ-L1-111** — "minimum 80% code coverage".
- **REQ-L1-114** — optional C foreign function interface for existing C/C++
  telemetry applications.
- **Future considerations** — Chapter 10 UDP streaming; advanced PCM
  decommutation; editions before 2013.

**Not to be carried over as written** — checked against 106-24R1:

- Its data-type list (REQ-L1-004) numbers types as formats ("PCM Data
  (Format 1)", "MIL-STD-1553 Data (Format 2)", "Time Data (Format 1, User
  Defined)"); Chapter 11 Table 11-4 gives data-type families, each with its
  own formats (for example PCM Data Format 1 is `0x09`, MIL-STD-1553 Data
  Format 1 is `0x19`).
- REQ-L1-014 calls the setup record "Computer Generated Data packets
  (Format 0)"; it is Format 1 (`0x01`), and Format 0 is user-defined data.
- Its section references (10.2, 10.3.x, 10.4, 10.5, 10.7) follow the older
  Chapter 10 layout; from 106-17 the packet formats are in Chapter 11.
- It cites 106-23 and Rust 1.70; the ecosystem baseline is 106-24R1 and
  edition 2024 (Rust 1.85).

## Proposed use

- Adopt the security and supply-chain practices across the ecosystem,
  starting with this repository: `SECURITY.md`, cargo-deny, cargo-audit,
  the semgrep rules, and release provenance (ROADMAP, "Security and supply
  chain").
- Carry the non-functional requirements above into the ecosystem's
  requirements, repository by repository, when each is designed.

## Where its useful parts went (2026-09-27)

The owner decided to delete the repository and to keep only what makes the
existing projects better ("Only copy the things that will make our existing
projects better"). Each item went where it is used:

| From `irig106-rust` | Now | Where |
|---------------------|-----|-------|
| `SECURITY.md` | the organisation's security policy, reporting through GitHub's private vulnerability reporting; its controls section rewritten to state what is in place and what is planned | `TelemetryWorks/.github` `SECURITY.md` (covers every repository without its own) |
| `deny.toml` | updated to the current cargo-deny schema and run in CI; its first run found three advisories (see L1-REL-005) | `deny.toml` and CI job `deny` in `irig106-tmats` and `irig106-time` |
| `semgrep-rust.yml` | its useful rules (no `unwrap`, `expect`, panics, `dbg!`, or printing in library code) as Clippy lints, which know types and already run in CI; its `unsafe` rules duplicate `#![forbid(unsafe_code)]` and its async rules do not apply | L1-ROB-002 and ADR-0020 here; `irig106-time` L1-API-012 and architecture section 9 |
| `notes/SLSA1.md`, the SBOM steps of `scripts/make_release.sh` | a software bill of materials and a build-provenance attestation with each release | `docs/RELEASING.md` step 7 and L1-REL-006 here; `irig106-time` L1-REL-006 |
| Draft L1: Windows, Linux, and macOS (REQ-L1-103) | requirement; `irig106-time`'s CI now tests on all three, as this repository's already did | L1-REL-004 in both repositories |
| Draft L1: 80% coverage (REQ-L1-111) | requirement | L1-ROB-003 here; `irig106-time` L1-TST-006 |
| Draft L1: 100 Mbps sustained (REQ-L1-102) | the figure the first performance measurement is compared with | L1-PERF-001 in both repositories |
| Draft L1: C interface; UDP streaming; PCM decommutation | ideas for later | `irig106-docs` `src/coverage.md`, "Ideas for later" |
| `.editorconfig` | adopted, with Markdown and YAML settings added | `.editorconfig` in `irig106-tmats` and `irig106-time` |

Not taken, because nothing would improve: the placeholder code, tests,
benchmark, and fuzz target; the empty conformance folders and documentation
book; the Makefile; `audit.toml` (cargo-deny covers advisories); the
commented-out CI file; the release script (its useful part is step 7 above);
the Rust 1.81 pin; `CODEOWNERS`; the licence file. The copies above in this
folder stay as the record.
