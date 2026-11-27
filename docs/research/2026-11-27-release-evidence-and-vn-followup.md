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

L1-REL-006 and RELEASING.md already require binary SBOMs and verifiable build
provenance. Source-inspected RELEASING.md steps 5–7: the planned release.yml
builds Windows/Linux/macOS binaries, SHA-256 checksums, per-binary CycloneDX
SBOMs using cargo cyclonedx, and GitHub build-provenance attestations.
The documented user verification recipe uses gh attestation verify with
TelemetryWorks as owner. No external tool behavior or successful artifact
verification is claimed from quoting that recipe.

The current workflow file inventory contains CI, Scorecard and zizmor, not
implemented release.yml/trusted-publishing.yml. Naming future jobs in docs
does not establish their existence, completed results or crate registrations.
No registry/account state, secrets or other repositories are inspected.

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

## Binary artifact evidence boundary

L2-REL-006/L3-REL-012–016 require exact artifact identity and a verification
path. A binary and its checksum/SBOM/provenance must describe the same source,
version, target/features and actual bytes. Same filenames or another target's
SBOM do not prove that binding. The actual resolved build context matters,
not merely the manifest's semver requirements or a library-only inventory.

Retain applicable completed verification results, not just an attestation
step name or a green workflow badge. A missing file/evidence, changed digest,
wrong source or failed verification remains unsatisfied. No new CycloneDX
schema version, generator, signing/notarization policy, architecture list or
public TMATS report format is selected by this contract.

Binary distribution remains separate from crate publishing. All real crate
releases stay Trusted Publishing through trusted-publishing.yml with crates-io;
release.yml never acquires crate-publishing authority. The existing narrow
first-upload placeholder exception stays coordinator/owner-only, never CI or
this maintainer. This document changes/executes no registration, setting,
workflow, token flow, upload, packaging or publishing operation. Signed build
evidence is not a positive TMATS G\SHA result or semantic-conformance proof.

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
