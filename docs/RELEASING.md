# Releasing

> **Status: draft for review.** Written during the design phase so that
> publishing is understood before any code is released. The decisions here are
> recorded as ADRs; the mechanics are finalized with the first release (0.1).

The 2026-11-21 owner decision keeps the full 0.1 release obligations and
adds a bounded time view before `irig106-time` 0.8.0. See the
[scope, API review and acceptance contract](RELEASE-0.1.md). This document
specifies future release mechanics; it does not claim the workflow exists.

## Published dependency prerequisites for tmats 0.1

Publish `irig106-types` 0.2 and `irig106-core` 0.2 BEFORE tmats 0.1.
The integration uses header-only PacketFacts from types, core's PacketInfo
integrity/trust sideband and borrowed DATA spans. Core 0.1.0 already being
published, or the core 0.2 reader being merged, does not satisfy publication
of these 0.2 dependencies. Verify the compatible published APIs before the
tmats release; do not add fields to PacketFacts or ship a fallback reader.
Packet data-checksum verification remains a separately owner-scheduled core
feature; pass NotChecked until core provides it. No publishing action is
authorised/performed by this documentation change.

## What gets released

| Crate | Kind | crates.io | Installs |
|-------|------|-----------|----------|
| `irig106-tmats` | library | yes | a dependency: `irig106-tmats = "0.1"` |
| `irig106-tmats-cli` | binary `tmats` | yes | `cargo install irig106-tmats-cli` → `tmats` |

The release target after the owner-directed restructure (ADR-0035) is a
virtual workspace: library in `crates/irig106-tmats/`, CLI in
`apps/irig106-tmats-cli/`. The current files have not moved in this docs PR. Prebuilt `tmats` binaries for Windows, Linux, and
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
4. Tag `vX.Y.Z` and push the tag.
5. `.github/workflows/trusted-publishing.yml`, gated by the protected
   `crates-io` environment, publishes both crates using Trusted Publishing with
   `cargo publish --workspace` (stable since Rust 1.90; it publishes in
   dependency order, library first, verifying the set together). Publishing
   is **not atomic**: if the CLI upload fails after the library succeeded,
   re-run for the CLI only.
6. The planned cargo-dist workflow `.github/workflows/release.yml` builds
   `tmats` for Windows, Linux, and macOS, attaches the
   binaries and their SHA-256 checksums to the GitHub release, and copies the
   changelog section into the release notes (decided 2026-09-25; configured
   before 0.1).
7. That same `release.yml` binary-distribution workflow attaches a
   **software bill of materials** for each binary (CycloneDX, `cargo cyclonedx`) and creates a
   **build-provenance attestation** for every attached file with
   `actions/attest-build-provenance` (SLSA build level 1, signed through
   GitHub's Sigstore instance; the workflow needs `id-token: write` and
   `attestations: write`). A user verifies a download with
   `gh attestation verify <file> --owner TelemetryWorks` (L1-REL-006;
   recipe from the retired `irig106-rust`'s `notes/SLSA1.md`, kept in
   `docs/research/2026-09-26-irig106-rust/`).

The binary-distribution `release.yml` workflow never publishes to crates.io
and receives no crates.io publishing credential. Package publishing belongs
only to `trusted-publishing.yml`; these workflows are still to be implemented.

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
  checksums?~~ Answered by step 7: each file carries a signed
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
