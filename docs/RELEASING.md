# Releasing

> **Status: draft for review.** Written during the design phase so that
> publishing is understood before any code is released. The decisions here are
> recorded as ADRs; implementation must adopt the org standard before 0.1.

Owner direction (2026-11-22): all repositories maintain the org
[TRUSTED-PUBLISHING.md binary variant](https://github.com/TelemetryWorks/.github/blob/2adedc5aaf27b7bafb54ad05a51bc17d89b65d3e/docs/TRUSTED-PUBLISHING.md#repositories-that-also-ship-binaries).
This **supersedes** the 2026-09-25 separate release.yml/cargo-dist plan and
its later separate attestation step. They are historical, not parallel paths.
The org standard governs where this summary differs. The binary variant is
required before the first real release; this docs PR implements no workflow.

The 2026-11-21 owner decision keeps the full 0.1 release obligations and
adds a bounded time view before `irig106-time` 0.8.0. See the
[scope, API review and acceptance contract](RELEASE-0.1.md). This document
specifies future release mechanics; it does not claim the workflow exists.

## Published dependency prerequisites for tmats 0.1

Publish `irig106-types` 0.2 and `irig106-core` 0.2 BEFORE tmats 0.1.
Owner-reported 2026-11-28: core 0.2.0 is published, with
[release run 46162767638](https://github.com/TelemetryWorks/irig106-core/actions/runs/46162767638)
green; its publication prerequisite is met. Types 0.2 is still unpublished.
This workspace skeleton adds neither dependency; compatible API/release-graph
evidence remains required when integration is implemented.
The integration uses header-only PacketFacts from types, core's PacketInfo
integrity/trust sideband and borrowed DATA spans. Core 0.1.0 already being
published, or the core 0.2 reader being merged, does not satisfy publication
of these 0.2 dependencies. Verify the compatible published APIs before the
tmats release. The release graph must resolve both from crates.io without
path/git/[patch]/source overrides and exercise their compatible published
APIs; a CI-only source override is never release evidence. The internal exact
workspace library path is retained, with a matching published dependency in
the packaged CLI. Do not add fields to PacketFacts or ship a fallback reader.
Packet data-checksum verification is core-owned; pass its supplied evidence
unchanged, including NotChecked when reported. No publishing action is
authorised/performed by this documentation change.

## What gets released

| Crate | Kind | crates.io | Installs |
|-------|------|-----------|----------|
| `irig106-tmats` | library | yes | a dependency: `irig106-tmats = "0.1"` |
| `irig106-tmats-cli` | binary `tmats` | yes | `cargo install irig106-tmats-cli` → `tmats` |

The current owner-directed layout (ADR-0035) is a
virtual workspace: library in `crates/irig106-tmats/`, CLI in
`apps/irig106-tmats-cli/`. The mechanical skeleton remains at 0.1.0,
without a release/publish. Prebuilt `tmats` binaries for Windows, Linux, and
macOS are attached to each GitHub release for users without a Rust toolchain
(the way `idmptmat.exe` is distributed today).

Name notes (checked on crates.io, 2026-09-25): `irig106-tmats` is ours
(placeholders 0.0.1, whose `repository` link wrongly pointed at
`irig106-write`, and 0.0.2, which corrects it).
`irig106-tmats-cli` is free. `tmats` is also free; we do not claim it,
because the ecosystem uses the `irig106-` prefix and a bare `tmats` crate
would suggest an official or neutral package. Decided names: crate
`irig106-tmats-cli`, binary and command `tmats`.

**Placeholder fix (done 2026-09-25).** Published crate metadata cannot be
edited, so the wrong link was fixed by publishing a new placeholder,
**0.0.2**: no code, a README pointing at this repository, `repository` and
`homepage` set to `https://github.com/TelemetryWorks/irig106-tmats`,
`rust-version = "1.85"`. crates.io shows the latest version's metadata, so
0.0.1 was not yanked. The first real release is 0.1.0.

## Lockstep versioning

Real library and CLI releases always carry **the same version** and are
released together. The name-only CLI bootstrap below is the sole exception
and is not a functional CLI release:

- The version is declared once, in `[workspace.package] version`, and both
  crates use `version.workspace = true`.
- The virtual root declares the library in `[workspace.dependencies]` with
  `path = "crates/irig106-tmats"` and exact `version = "=X.Y.Z"`; the CLI
  inherits it with `irig106-tmats = { workspace = true }`. A `tmats`
  installed from crates.io is therefore always built against the matching
  library, and a mismatched pair cannot be published.
- One git tag (`vX.Y.Z`) and one `CHANGELOG.md` cover both; each entry says
  whether it affects the library, the CLI, or both.
- Consequence: a CLI-only fix also bumps the library's version (with no
  library change). That is accepted; the version tells users which pair
  they have.

Semantic versioning applies to the **library's public API** and to the
**CLI's documented interface** (commands, flags, exit codes, and the JSON
output schema). A breaking change to either is a breaking release for both.
Before 1.0, a breaking change bumps the minor version (0.1 → 0.2).

## Release checklist

Before tagging the first real release, complete the CLI name-placeholder
bootstrap below and verify both crates' Trusted Publisher registrations.

1. `[Unreleased]` in `CHANGELOG.md` becomes the new dated version section.
2. Bump `[workspace.package] version` and the CLI's `=X.Y.Z` requirement in
   one commit (`chore(release): prepare X.Y.Z`).
3. CI must be green, including: tests on every feature set, clippy and
   rustdoc with `-D warnings`, the MSRV check, the trace-matrix and registry
   `--check` scripts, a public-API break check on the library
   (`cargo semver-checks`) once a previous version exists, and
   `cargo publish --workspace --dry-run`.
4. Commit `docs/releases/vX.Y.Z.md` with the exact release body and title
   (first heading), then tag `vX.Y.Z` and push the tag after review.
   The gate verifies the **annotated** tag through the remote GitHub API,
   peels it to `github.sha`, verifies that commit is on `main` and requires
   successful intended CI on that exact commit; a local tag or unrelated
   green run is insufficient.
5. `.github/workflows/trusted-publishing.yml` (name **Trusted publishing**)
   uses job IDs `gate`, `build`, `publish`, `github-release`, `verify`, plus
   read-only `publish-path-check` for PR/manual dry runs. There is **no
   top-level attest job**. `build` needs `gate` and calls reusable
   `./.github/workflows/build-binaries.yml`, which uses read-only
   `build-binaries-core.yml` to build/qualify targets, package crates and
   Windows/Linux/macOS binaries, and generate per-binary CycloneDX SBOMs
   (an exact pinned `cargo-cyclonedx`, installed with `--locked`) and one
   `SHA256SUMS`. Record the exact Rust toolchain as build output `rust-version`;
   `publish` and `publish-path-check` install and check that recorded version,
   never newly resolved stable after an approval delay. The builder checks out
   `github.sha` and self-attests its own verified output in a nested
   **Checksums and attestations** job only on `v*` tag push, independently
   guarded. The caller passes the gate decision, never signing inputs.
   PR/manual runs build but never sign. Building/signing precedes approval.
6. `publish` needs `[gate, build]`, uses the protected `crates-io` environment
   and **compiles nothing**. Revalidate the downloaded checksums; reproduce
   each `.crate` with `cargo package --locked --no-verify` and `cmp` it with
   the attested package. Publish in dependency waves, library before CLI,
   using fresh short-lived OIDC credentials per wave and
   `cargo publish --locked --no-verify`. Never store a token. Publication is
   not atomic: retry only missing uploads, but compare **every** served
   `.crate`, even already-published versions, byte for byte with the attested
   file and against the registry checksum. A mismatch stops the run.
7. `github-release` needs `[gate, publish]`. Create or resume a draft through
   the releases list, verify tag/title/exact committed body/draft state,
   upload all assets and check each name, size and digest. Publish and check
   again. Never replace assets or modify an already published release.
   The single release contains both `.crate` files, all binary archives,
   SBOMs, **one `SHA256SUMS`** and **one
   `irig106-tmats-<version>.intoto.jsonl`** bundle for all signed files.
8. `verify` needs `[gate, github-release]` and checks the actual published
   files. Verify served crate bytes/checksums and provenance for the intended
   repository/tag/reusable-builder signer. The documented user path includes
   `gh attestation verify <file> --owner TelemetryWorks`, constrained also to
   this repository, `--source-ref refs/tags/vX.Y.Z` and the expected builder
   workflow. Validate the checksum manifest and bundle as evidence, without
   claiming they necessarily have individual attestations. Require successful
   docs.rs builds within a bounded wait (pending at deadline fails), and a
   clean exact-version `cargo install --locked` plus executable version check.

These are future acceptance mechanics, not completed release evidence.
The reusable builder builds binaries, SBOMs and attestations; it never
publishes to crates.io. Only `publish` in `trusted-publishing.yml` with the
`crates-io` environment uploads crates. No release workflow is implemented
by this documentation change; build provenance alone claims no SLSA level.

## Credentials

Except for the first-upload bootstrap below, publishing runs only in GitHub
Actions using **crates.io trusted publishing**
(OIDC via `rust-lang/crates-io-auth-action`): the workflow exchanges its
identity for a short-lived token, so no long-lived crates.io API token is
stored in the repository. Trusted publishing is configured once per crate on
crates.io, restricted to this repository, `trusted-publishing.yml`, and the
protected `crates-io` environment (owner decision, 2026-11-21). Both crates
need matching publisher registrations. Publishing is through that workflow
only; stored registry tokens and local publishing are prohibited outside
the coordinator/owner bootstrap exception below.

The release toolchain may be newer than the MSRV (1.85); only building and
testing are held to the MSRV.

### First-upload bootstrap exception (existing owner policy, 2026-11-15)

Trusted Publishing cannot create a brand-new crate with its first upload.
`irig106-tmats-cli` has never been published, so the coordinator and owner
must bootstrap its name before the real lockstep 0.1 release, exactly as for
`irig106-time-cli` 0.0.1. This records existing policy, not a new permission
for a maintainer or an automated publisher.

1. Only for that brand-new crate's first upload, the coordinator and owner
   use a one-time, crate-scoped `publish-new` token that expires in **1 day**.
2. Upload only an `irig106-tmats-cli` **0.0.1 name placeholder**, using the
   same name-placeholder bootstrap procedure as `irig106-time-cli` 0.0.1.
   This reserves the name; it does not release prototype functionality or
   alter the workspace's planned 0.1.0 version or real-release lockstep rule.
3. Revoke the token **immediately** after the upload. Register the crate's
   Trusted Publisher for this repository, `trusted-publishing.yml` and the
   `crates-io` environment, and verify that registration before 0.1.
   Before any later upload, enable crates.io's per-crate
   Trusted-Publishing-only setting for `irig106-tmats-cli` and ensure it is
   also enabled for the existing `irig106-tmats` library crate, as required by
   [the security program policy](https://github.com/TelemetryWorks/.github/blob/main/SECURITY-PROGRAM.md#L37-L42).
4. Every later release is Trusted Publishing only. This is not a fallback
   for a failed TP run or permission problem, nor authorization to bootstrap
   an already published crate.

The coordinator and owner perform the bootstrap, **never CI**. Never store
its token as a repository/environment secret, file in this repository, or
workflow input. The maintainer does not create, request, handle or use it.
This documentation change performs no bootstrap, publication or settings
operation. The existing library placeholders do not need another bootstrap.

## Mistakes

A published version cannot be overwritten. An owner yanks a broken release
(both crates) through the **crates.io web UI**, then a fixed patch release
follows the Trusted Publishing path. Yanking prevents new dependency
resolution but does not delete anything.

## Open questions

- ~~Should `tmats` binaries be signed, beyond the published SHA-256
  checksums?~~ Answered by the reusable-builder protocol above: signed files carry a
  build-provenance attestation (2026-09-27). Signing the binaries themselves
  (Authenticode on Windows, notarisation on macOS) stays open.
- `irig106-cli` is the complete ecosystem tool and is released from its own
  repository on its own schedule; it depends on the `irig106-tmats` library
  like any consumer. If `irig106-tmats-cli` exposes its commands as a library
  for `irig106-cli` to mount, that library surface falls under the same
  lockstep version and semver rules as the binary.

Sources: [Rust 1.90 workspace publishing](https://www.infoworld.com/article/4060262/rust-1-90-brings-workspace-publishing-support-to-cargo.html),
[crates.io trusted publishing](https://crates.io/docs/trusted-publishing),
[crates-io-auth-action](https://github.com/rust-lang/crates-io-auth-action).
