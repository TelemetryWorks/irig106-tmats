# Changelog

All notable changes to the `irig106-tmats` library and the `tmats` CLI
(`irig106-tmats-cli`) are recorded here. Both crates are released together at
the same version (see `docs/RELEASING.md`); each entry says whether it
affects the library, the CLI, or both.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

Nothing has been released from this repository yet beyond the crates.io name
placeholders `irig106-tmats` 0.0.1 and 0.0.2 (0.0.2, published 2026-09-25,
corrects the placeholder's repository link). The original implementation was a
prototype, preserved at the git tag `prototype-0`; the project is being
rebuilt documentation first (`docs/ROADMAP.md`).

### Changed

- Both: virtual workspace layout, with the library in `crates/irig106-tmats`
  and the CLI in `apps/irig106-tmats-cli`, still version 0.1.0 in lockstep.
- CLI: existing help/version scaffold moved behind reusable `run()` with
  caller-owned arguments/output sinks and a thin executable.

### Removed

- Library: incorrect prototype implementation, tests, generated registry,
  benchmark and fuzz harness; history remains at `prototype-0`.
- Both: Criterion and all other third-party dependencies; the workspace now
  contains only the two crate skeletons.

### Added

- Library: checked `Span` construction and borrowed raw-byte access, with
  integer-boundary, shorter-source and bounded interval tests.

- Both: a Cargo workspace releasing the library and the CLI in lockstep —
  one version in `[workspace.package]`, the CLI pinned to the library with an
  exact `=X.Y.Z` requirement, and tests that fail if either drifts.
- CLI: the `irig106-tmats-cli` crate with the `tmats` binary as a scaffold:
  `--version` reports both the CLI and library versions, `--help` points at
  the planned commands; no commands are implemented yet.
- Library: `VERSION`, the crate version as a constant.
- Continuous integration: tests on Linux, Windows, and macOS, the MSRV check,
  clippy and rustdoc with warnings denied, formatting, and a workspace publish
  dry run.
