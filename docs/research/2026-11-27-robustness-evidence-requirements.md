# Library guard rails and coverage: family 2 batch 28

Date: 2026-11-27. Requirements/acceptance-inspection draft, not executable
coverage, adoption of a proposed ADR or implementation of a new CI job.
Batch 27 remains in held #59 pending the owner's std decision; this slice
starts from main and does not incorporate that branch's changes.

## Existing authority, bounded derivation

L1-ROB-002 requires the library outside tests to avoid unwrap/expect,
explicit panic/todo/unimplemented/unreachable/debug operations and printing,
with build failure on violation. Its rationale names the nine Clippy lints
and plans a workspace deny table plus scoped test/CLI-main exceptions.
L1-IO-001 supplies the library process-I/O boundary.

L1-ROB-003 already sets the minimum to 80% of library lines measured in CI,
from the first new library code. It plans cargo llvm-cov with
--fail-under-lines 80. This batch derives that existing threshold; it does
not propose a new number or substitute line coverage for the trace matrix.

ADR-0020 remains proposed. Its additional architecture options, including
unsafe-code policy, public enum policy, fuzz infrastructure and the complete
guard-rail bundle, are not approved by deriving these explicit L1 clauses.
Existing source-span/framing/resource requirements remain separate. No
numeric memory limit, recovery policy or std feature choice is made here.

## Source inspection, not checks executed

Inspected the current root Cargo.toml and .github/workflows/ci.yml on main:

- The prototype still occupies the root package and sibling CLI; the
  ADR-0035 virtual layout remains future work.
- CI invokes cargo clippy --workspace --all-targets --all-features with
  -D warnings. That invocation alone does not prove each restricted lint is
  enabled and effective on the replacement library.
- The root manifest does not yet contain the planned workspace lint table;
  no complete source-level lint audit or redesigned compile check is claimed.
- The inspected CI file has no llvm-cov measurement/80% enforcement job.

No Rust build, Clippy run, coverage measurement, tool installation or
compile-fail fixture is executed locally in this docs PR. Actual workflow
configuration and results must be inspected after implementation; current
prototype CI cannot provide Evidence for unimplemented checks.

## Lint acceptance

L2-ROB-004 and L3-ROB-007–009 require effective compiler-aware enforcement.
The exact existing list is unwrap_used, expect_used, panic, todo,
unimplemented, unreachable, dbg_macro, print_stdout and print_stderr.
Comments and strings mentioning those operations are not violations.
Conversely, a non-test production library operation must not escape merely
because it sits in a submodule or generated included source.

Inspect workspace/package inheritance after restructure: a table only in a
virtual root is not proof that a package applies it. Test-only allowances
or CLI-main exceptions cannot disable the separately compiled production
library check. A broad production allow or invocation override is not an
approved new exception. Concrete Rust lint wiring is implementation work.

Future isolated compile-fail fixtures must contain syntactically valid typed
operations and fail because of the intended lint. A syntax error, unrelated
dependency failure or nonzero command with no matching lint is not evidence
that enforcement worked. Record command, compiler/Clippy version, source
head and actual result. Keep the tests separate from production source;
this PR introduces no prohibited operations or compile-fail crates.

These lints are guard rails, not proof of panic freedom. Checked span access,
non-progress bounds, malformed-input testing, fuzzing and resource accounting
still need their own meaningful implementation/verification. No runtime
panic-catching wrapper is authorized as a substitute for correct behavior.

## Line coverage acceptance

L2-ROB-005 and L3-ROB-010–013 require actual CI measurement of the redesigned
library and failure below 80%. The metric is the coverage tool's library
line metric, not raw physical line count, CLI application lines, branch
coverage, a percentage of requirements or a claim that the standard is met.

Record the library package/source scope, measured features/configuration,
command/tool version, exact source head and result. Include actual omissions
and exclusions rather than silently deleting untested production files from
the denominator. Generated/conditional library code must remain accounted
for in the scope description; no exclusion list or new feature-matrix policy
is approved by this batch. Unmeasured configurations remain unmeasured.

Applicable project tests that execute library code may contribute to its
metric; CLI application coverage cannot inflate the library score. Evidence
from another source head, prototype report or successful unrelated job is
not a current library measurement. Missing/failed measurement stays missing/
failed, even if the threshold flag appears in workflow source.

Demonstrate the gate's actual failure boundary with a controlled below-floor
case and a matching-head completed passing measurement. Do not rely on a
rounded display value or assume the command's failure propagated. No fake
coverage artifact or threshold mutation is produced in this docs-only PR.

## Gates and carried documentation correction

New requirements are inspection-verifiable drafts with no Evidence lines.
Their planned cases are in TEST-DATA. Implementation must add checks with
the first new library code and use bounded GitHub CI; local work stays light.
No workflow, manifest, dependency, runtime code, credential or policy/settings
change is part of this slice.

The carried #60 nit updates TMATS-IN-CHAPTER-10 section 8.5: recording/setup
cross-check tooling belongs to irig106-validator under the 2026-11-27 roadmap
direction; tmats validate concerns TMATS text rules. It does not add a new
command or prescribe a report format.

No owner decision is needed to derive these existing L1 clauses. Broader
ADR-0020 remains proposed; #59's std conflict and the pattern, extraction,
record-access/governance and report-format holds remain open. Passing lints
or a line floor cannot sign off an interpretation or permit qualified-record
use. Source freshness, deep development analysis and executable semantic
fixtures remain distinct gates before implementation/acceptance.
