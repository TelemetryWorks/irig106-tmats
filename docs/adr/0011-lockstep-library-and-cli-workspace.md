---
status: accepted
date: 2026-09-25
decision-makers: Joey
---

# One workspace; the library and the `tmats` CLI are released in lockstep

## Context and Problem Statement

The project needs a command-line tool — a Rust successor to irig106.org's
`idmptmat`, with its three output formats and TMATS checksums. The owner
asked whether it would be a new crate in this repository and said, "we should
align the CLI version with the Lib version at all times." Names were decided
as crate `irig106-tmats-cli`, binary and command `tmats`. The ecosystem-wide
tool, `irig106-cli`, is separate: "irig106-cli repo and crates.io is a more
robust and complete CLI tool instead of a simple TMATS only CLI tool."

## Decision Drivers

* A user of `tmats` always knows which library produced the output
* A mismatched library/CLI pair can never be built or published
* The CLI is thin; its features track the library's

## Considered Options

* **A Cargo workspace in this repository: one version in
  `[workspace.package]`, the CLI pinned to the library with `=X.Y.Z`, one tag,
  one changelog**
* A separate repository for the CLI with its own version (the way
  `irig106-time-cli` sits beside `irig106-time`, unversioned against it)
* A binary target inside the library crate

## Decision Outcome

Chosen option: **the lockstep workspace**. `tests/lockstep.rs` in the CLI
crate fails if the versions or the exact pin drift apart, and
`tmats --version` prints both versions. A CLI-only fix also bumps the
library's version; that is accepted.

### Consequences

* Good: drift is impossible to publish; CI proves it with
  `cargo publish --workspace --dry-run`.
* Good: the library does not carry CLI dependencies, and the CLI can be
  installed on its own.
* Bad: version bumps with no change to one of the two crates.
* Relation to `irig106-cli`: it uses the library like any consumer; whether
  it mounts the `tmats` commands directly is still open (`docs/ROADMAP.md`).
