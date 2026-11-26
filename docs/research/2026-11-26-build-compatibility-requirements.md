# Build compatibility: family 2 batch 27

Date: 2026-11-26. Requirements and planned inspection cases, not execution
of redesigned builds or acceptance of a new dependency/target policy.

## Authority and scope

L1-REL-002 requires Rust 1.85; L1-REL-003 requires the plain library to build
for wasm32-unknown-unknown with a check on every change from the first new
library code; L1-REL-004 requires library/CLI builds and tests on Windows,
Linux and macOS. These are existing full-0.1 obligations. CLAUDE.md records
the owner-requested edition-2024 minimum; ADR-0035 supplies the future
virtual workspace and crates/ + apps/ package locations.

The owner-required consumer contract in RELEASE-0.1 is exact:

```toml
irig106-tmats = { version = "0.1", default-features = false, features = ["std"] }
```

Once the time view exists, CI must compile that consumer and exercise the
public parse/time-view API. An all-features workspace build is not that
check. This is for the irigtime CLI only, not the time library; the local
consumer fixture does not require accessing or editing another repository.

ADR-0015 removes prototype WASM bindings/fake features and defers an
alloc-only no_std build until properly supported. Plain-library WASM build
support remains in 0.1. A std-enabled dependency with defaults disabled is
not a no_std promise. Build checks do not authorize a bindings crate.

## Source inspection today, not executed checks

Inspected current root Cargo.toml and .github/workflows/ci.yml:

- The prototype still has a root package and sibling CLI; ADR-0035's virtual
  layout has not been implemented.
- Workspace package metadata declares edition 2024 and rust-version 1.85.
- The native test matrix lists Ubuntu, Windows and macOS and executes
  workspace --all-features and --no-default-features tests.
- The msrv job selects 1.85 and runs cargo check --workspace --all-features;
  its comment distinguishes production packages from newer dev tooling.
- The package dry run uses stable; RELEASING.md separately documents the
  publishing toolchain. That is not a replacement for the consumer MSRV.

These are source observations, not newly executed builds. No Rust/toolchain
installation, dependency resolution, WASM build, platform test, package
operation or publishing action is performed in this docs PR. New L2/L3
requirements receive no Evidence lines from inspecting prototype config.
The two old L1 MSRV/platform configuration references are retained under an
explicit prototype-configuration label rather than executable acceptance
Evidence, so they cannot mark the new obligations implemented by inheritance.

## Required evidence after implementation

L2-REL-002/003/004 and L3-REL-002–008 derive these acceptance boundaries.
Future inspection must identify the reviewed head, actual toolchain,
target/features, commands and completed run/job results. A matrix entry,
skip condition or overall green run does not prove each required job ran.
A cancelled/missing job stays unsatisfied, not accepted as another OS's pass.

Manifest inspection resolves real workspace inheritance for both packages.
It must not match a comment, a development fixture or only the virtual root.
Rust 1.85 must actually check the redesigned packages and their production
dependencies; stable-only success and declared rust-version are insufficient.
A dependency MSRV failure is a real compatibility gap, not permission to
silently omit the CLI, drop a supported feature or raise the minimum.

Native tests must exercise implemented library and reusable CLI behavior,
not only help/version scaffolding. Platform-specific file/link capability
limitations remain explicit coverage gaps; this does not waive input safety
or decide the pending identity policy for unusual output destinations.
This slice adds no new CPU-architecture or cross-compilation matrix.

Plain WASM evidence names the library target and features. It must not be
a native build or the old wasm-bindgen prototype feature. It does not
require compiling the CLI/core reader to WASM, prove no_std support or add
WebAssembly bindings. Once first redesigned library code exists, it must
not remain a permanently skipped or optional check.

## Isolated consumer and dependency source

The consumer's dependency entry stays exactly as specified above. Compile
outside workspace feature unification so an unrelated all-features member
cannot enable a missing library feature. Exercise actual reviewed public
parse and time-view calls once implemented; help output or an empty main
that never uses those APIs does not meet the consumer obligation.

Before 0.1 publication, a separately recorded CI source override can bind
that unchanged dependency entry to the exact reviewed library checkout.
Verify the resolved package source/version/configuration, not just the text
of Cargo.toml. Do not claim a placeholder or unpublished package provides
the new API. No fixture is built and no override added in this docs PR.

The declared std consumer configuration is owner authority; concrete public
API calls still follow reviewed implementation rather than prototype names.
Published types/core 0.2 and actual compatible published APIs remain release
prerequisites. No cross-repository API inspection or compatibility claim is
made here. A local source fixture before publication does not satisfy those
publication prerequisites or authorize a release.

## Remaining gates

This slice selects no new owner policy, rendering/schema, dependency
exception, target feature or API signature. Pattern scope and public syntax,
extraction overwrite/identity/eligibility and report-format decisions remain
pending. Compile success cannot authorize qualified-record use, approve an
interpretation or count as semantic parser coverage.

CI implementation belongs with the corresponding new code and reviewed
checks. Publishing/attestation changes belong to the later release family;
all real releases stay Trusted Publishing through trusted-publishing.yml and
the crates-io environment, with the already documented coordinator/owner
first-upload exception unchanged. No workflow, manifest, source, credential
or settings change is included in this requirements batch.
