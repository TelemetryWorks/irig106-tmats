# Releasing

> **Status: draft for review.** Written during the design phase so that
> publishing is understood before any code is released. The decisions here are
> recorded as ADRs; the mechanics are finalized with the first release (0.1).

## What gets released

| Crate | Kind | crates.io | Installs |
|-------|------|-----------|----------|
| `irig106-tmats` | library | yes | a dependency: `irig106-tmats = "0.1"` |
| `irig106-tmats-cli` | binary `tmats` | yes | `cargo install irig106-tmats-cli` → `tmats` |

Both live in this repository as one Cargo workspace (library at the root, CLI
in `irig106-tmats-cli/`). Prebuilt `tmats` binaries for Windows, Linux, and
macOS are attached to each GitHub release for users without a Rust toolchain
(the way `idmptmat.exe` is distributed today).

Name notes (checked on crates.io, 2026-09-25): `irig106-tmats` is ours (a
0.0.1 placeholder whose `repository` link wrongly points at `irig106-write`;
the next publish corrects it). `irig106-tmats-cli` is free. `tmats` is also
free; we do not claim it, because the ecosystem uses the `irig106-` prefix
and a bare `tmats` crate would suggest an official or neutral package.

## Lockstep versioning

The library and the CLI always carry **the same version** and are always
released together:

- The version is declared once, in `[workspace.package] version`, and both
  crates use `version.workspace = true`.
- The CLI depends on the library with an **exact** requirement next to the
  path: `irig106-tmats = { path = "..", version = "=X.Y.Z" }`. A `tmats`
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

1. `[Unreleased]` in `CHANGELOG.md` becomes the new dated version section.
2. Bump `[workspace.package] version` and the CLI's `=X.Y.Z` requirement in
   one commit (`chore(release): prepare X.Y.Z`).
3. CI must be green, including: tests on every feature set, clippy and
   rustdoc with `-D warnings`, the MSRV check, the trace-matrix and registry
   `--check` scripts, a public-API break check on the library
   (`cargo semver-checks`) once a previous version exists, and
   `cargo publish --workspace --dry-run`.
4. Tag `vX.Y.Z` and push the tag.
5. The release workflow publishes both crates with
   `cargo publish --workspace` (stable since Rust 1.90; it publishes in
   dependency order, library first, verifying the set together). Publishing
   is **not atomic**: if the CLI upload fails after the library succeeded,
   re-run for the CLI only.
6. The workflow builds `tmats` for each target, attaches the binaries and
   their SHA-256 checksums to the GitHub release, and copies the changelog
   section into the release notes.

## Credentials

Publishing runs only in GitHub Actions using **crates.io trusted publishing**
(OIDC via `rust-lang/crates-io-auth-action`): the workflow exchanges its
identity for a short-lived token, so no long-lived crates.io API token is
stored in the repository. Trusted publishing is configured once per crate on
crates.io, restricted to this repository, the release workflow, and a
protected `release` environment. Local `cargo publish` is not part of the
normal process.

The release toolchain may be newer than the MSRV (1.85); only building and
testing are held to the MSRV.

## Mistakes

A published version cannot be overwritten. A broken release is **yanked**
(both crates) and followed by a fixed patch release; yanking prevents new
dependency resolution but does not delete anything.

## Open questions

- Which tool builds and uploads the release binaries (a hand-written
  workflow or a release tool such as `cargo-dist`)? Decided in an ADR before
  0.1.
- Should `tmats` binaries be signed, beyond the published SHA-256
  checksums?
- `irig106-cli` is the complete ecosystem tool and is released from its own
  repository on its own schedule; it depends on the `irig106-tmats` library
  like any consumer. If `irig106-tmats-cli` exposes its commands as a library
  for `irig106-cli` to mount, that library surface falls under the same
  lockstep version and semver rules as the binary.

Sources: [Rust 1.90 workspace publishing](https://www.infoworld.com/article/4060262/rust-1-90-brings-workspace-publishing-support-to-cargo.html),
[crates.io trusted publishing](https://crates.io/docs/trusted-publishing),
[crates-io-auth-action](https://github.com/rust-lang/crates-io-auth-action).
