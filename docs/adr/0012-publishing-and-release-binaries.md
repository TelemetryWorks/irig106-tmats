---
status: accepted
date: 2026-09-25
decision-makers: Joey
---

# Publish both crates to crates.io together; ship `tmats` binaries with `cargo-dist`

## Context and Problem Statement

The owner asked to "make sure we understand how publishing to crates.io will
happen or if that will happen for the cli tool." Users of today's
`idmptmat.exe` do not have a Rust toolchain. The crates.io name
`irig106-tmats` is held by placeholders (0.0.1 with a wrong repository link,
fixed by 0.0.2 on 2026-09-25).

## Decision Drivers

* Library and CLI published as one set (ADR-0011)
* No long-lived publishing secrets in the repository
* Prebuilt binaries for Windows, Linux, and macOS

## Considered Options

* **`cargo publish --workspace` from GitHub Actions with crates.io trusted
  publishing; `cargo-dist` for binaries**
* Manual publishing from a developer machine
* A hand-written binary release workflow

## Decision Outcome

Chosen option: **both crates published together by `cargo publish
--workspace` (stable since Rust 1.90; library first; not atomic, so a failed
CLI upload is re-run), authenticated by trusted publishing (OIDC via
`rust-lang/crates-io-auth-action`), and binaries plus SHA-256 checksums built
by `cargo-dist` from the release tag.** A broken release is yanked and
followed by a patch release. Details: `docs/RELEASING.md`.

### Consequences

* Good: no stored crates.io token; every release comes from a tagged CI run.
* Good: users without Rust get `tmats` from the GitHub release.
* Bad: trusted publishing and `cargo-dist` must be configured before 0.1.
