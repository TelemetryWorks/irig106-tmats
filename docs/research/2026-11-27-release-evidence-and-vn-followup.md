# Dependency/artifact acceptance and usable VN: family 2 batch 32

Date: 2026-11-27. Documentation-first requirements and an INT-063 definition
amendment. No implementation, policy edit, credential access or release.
Held #59 reserves L2-REL-002–004/L3-REL-002–008; this independent slice uses
L2-REL-005/006 and L3-REL-009–016 rather than reusing those identifiers.

## Existing authority and source-only inspection

L1-REL-005 requires the repository dependency policy with documented reasons
for exceptions. Inspected current deny.toml and .github/workflows/ci.yml:

- Graph configuration enables all features; the source rules deny unknown
  registries/git and list crates.io as allowed. The internal workspace path
  dependency is already accepted; it is not a blanket third-party allowance.
- The policy lists permitted licences, warns on duplicate versions and denies
  wildcard versions with the documented workspace-path handling. This batch
  does not change severities, allowed licences, sources or tool options.
- Two listed advisory ignores are justified only by the prototype XML code;
  their comments explicitly say to remove them with that code. This is a
  source observation, not a fresh vulnerability/advisory-status lookup.
- The CI deny job exists, but a source reference to prototype configuration
  cannot prove acceptance of the future redesigned dependency graph.

The old L1-REL-005 workflow reference is retained as prototype configuration,
not Evidence that these new graph obligations are implemented. Tool behavior
and actual matching-head results still need verification; comments do not
prove the intended rule was enforced. No cargo-deny command/database fetch,
lockfile resolution or dependency installation runs locally in this PR.

L1-REL-006 requires binary SBOMs and verifiable build provenance. Owner
direction of 2026-11-22 requires all repos to maintain the org standard.
Source-inspected [TRUSTED-PUBLISHING.md](https://github.com/TelemetryWorks/.github/blob/2adedc5aaf27b7bafb54ad05a51bc17d89b65d3e/docs/TRUSTED-PUBLISHING.md),
particularly “Repositories that also ship binaries” and base publication/
verification sections. It requires the binary variant before tmats' first
real release. This supersedes RELEASING.md's 2026-09-25 separate
release.yml/cargo-dist plan; RELEASING.md is reconciled in this batch.

The current workflow inventory contains CI, Scorecard and zizmor, not an
implemented release pipeline. Naming future jobs proves neither their
existence nor completed results or registrations. Only the authorized org
standard was read externally; no registry/account state or secrets inspected.

## Dependency evidence boundary

L2-REL-005/L3-REL-009–011 require the actual resolved package graph, checked
configuration, source head, policy revision, tool/command and advisory-source
snapshot/time. A direct manifest list can miss transitive dependencies or
actual selected sources; another feature graph cannot silently substitute.
No new database freshness interval or target/feature matrix is chosen here.

Keep failures/limitations and applicable documented exceptions explicit.
A warning is not silently promoted into a newly denied condition, nor may a
denial become a pass because the wrapper ignores a command failure. Review
prototype-only XML ignores when that code is retired; do not transfer their
rationale to replacement dependencies or approve a new exception in this PR.
Criterion retention and new dev dependencies still require owner approval.

## Unified release-artifact evidence boundary

L2-REL-006 keeps L1-REL-006 as primary parent and broadens the child to release
artifacts, motivated also by L1-REL-001/005 and the org mandate. No new L1 or
requirement IDs: crate-package evidence accompanies binary evidence.
L3-REL-012–016 derive the binary variant:

- trusted-publishing.yml has gate/build/publish/github-release/verify plus
  read-only publish-path-check, no top-level attest job. build invokes
  build-binaries.yml and read-only build-binaries-core.yml; its nested
  Checksums and attestations signs its own verified output only on v* tag
  push. The caller cannot choose signing inputs; PR/manual builds never sign.
- One release contains .crate files, binary archives and per-binary CycloneDX
  SBOMs, one SHA256SUMS and one irig106-tmats-<version>.intoto.jsonl bundle.
  Match source, version, target/features and bytes, not filenames alone.
- publish needs [gate, build], compiles nothing, uses crates-io and fresh
  ephemeral OIDC per wave. Repackage with cargo package --locked --no-verify,
  cmp against the attested package, then compare every served .crate byte
  for byte and against the registry checksum, including on retries.
- github-release creates/resumes a draft, verifies tag/title/exact committed
  docs/releases/vX.Y.Z.md body/draft state, checks each asset name/size/digest
  before and after publication and never modifies a published release.
- verify checks actual published files. User attestation verification must
  constrain repository, tag and reusable-builder signer; validate the manifest
  and bundle without assuming each has an individual attestation.

A missing/mismatched artifact or failed verification remains unsatisfied.
No new SBOM schema, generator, architecture list, signing/notarization policy
or TMATS output format is selected, and provenance alone claims no SLSA level.
Real releases remain TP-only; the existing narrowly scoped first-upload
exception stays coordinator/owner-only, never CI or this maintainer.
No workflow, registration, settings, token, packaging or upload action runs
here. Signed build evidence proves no TMATS G\SHA or semantic conformance.

The release dependency graph must use compatible published types 0.2/core 0.2
APIs from crates.io without path/git/[patch]/source overrides. A CI-only
source override is never release evidence. Internal exact-version workspace
library paths remain allowed; the packaged CLI must resolve its matching
published library. Publication of both external dependencies precedes tmats.

## INT-063 usable VN amendment requested after #64

A usable VN is one present, non-Ambiguous candidate with available character/
blank interpretation: INT-036 projection, INT-041 blank/availability guards,
INT-037 character handling and exactly three projected characters under
106-24R1 section 9.5.13 p. 9-171. Compare ASCII case-insensitively under
section 9.4.2 p. 9-2, never locale/Unicode/high-byte folding. Source PDF
[official archive](https://www.trmc.osd.mil/wiki/download/attachments/335389015/Chapter9.pdf?api=v2),
SHA-256 `dfc5ebfb3e6f02f58389126863d43a45d89c7e2d0533a1c690d9537531c740cb`.

Available projected wrong length is Invalid and leaves confirmed attribution
unconfirmed under proposed A. Do not demote that known failure to generic
unavailable. Unreviewed blank handling remains unavailable under INT-041;
do not trim a leading blank to fabricate usable VN or import INT-055's
numeric syntax. Projection never changes source bytes, and equal duplicate
header values do not cure Ambiguous. No additional acronym alphabet rule
is introduced. Other proposed attribution options cannot make Invalid VN
valid even if they later allow labelled path-derived attribution.

INT-063 remains PROPOSED, with owner sign-off and amended-wording independent
AI review pending. The added definition/Option-A fixtures are BLOCKED, not
current conformance coverage. Earlier dated V research stays historical.

## Verification and remaining gates

TEST-DATA names planned graph/artifact acceptance cases and VN examples.
Negative fixtures require the intended failure reason, not unrelated network/
syntax errors. Artifact verification/executable checks belong to later
reviewed work, not this docs-only PR. New inspection requirements have no
Evidence lines; current prototype CI cannot fill them by inheritance.

Local work is source inspection plus trace/identifier/link/whitespace checks.
No Rust builds/tests, cargo-deny, SBOM generator, attestation verifier, sample
access or install runs here; heavy checks stay in GitHub CI. Pending std,
INT-061/062/063, pattern, extraction, allocation, record-access/governance and
report-format gates remain unchanged; #59 must rebase/recount before re-review.
