# Performance measurement: family 2 batch 29

Date: 2026-11-27. Requirements and planned evidence cases, not executed
benchmarks, measured speedups, a new budget or approval of proposed ADR-0021.

## Existing authority and source inspection

L1-PERF-001 requires measuring TMATS reading and finding every setup record
in a recording on synthesized/sample data, with idmptmat comparison on the
same files. It explicitly defers a numeric budget until first measurement.
The historical 100 Mbps figure in its rationale is comparison context, not
an accepted new release floor. ADR-0021 remains proposed as a whole.

ADR-0036 puts the single packet reader in core; TMATS consumes header facts,
separate integrity sideband and borrowed DATA. ADR-0025 and approved joining/
closure rules distinguish fragments, candidate runs and setup records.
Performance evidence must preserve those responsibilities and qualifications.

Source-inspected the prototype benches/parse_bench.rs and Cargo.toml:
there is a Criterion parse/serialization/validation benchmark and an existing
criterion dev dependency. Retaining that prototype Criterion dev dependency
for the redesign requires OWNER APPROVAL; presence in Cargo.toml does not
grant approval to keep it. The bench contains prototype fixtures and obsolete
traceability comments, not executable requirement markers for these new
requirements. Neither fixture semantics nor measured redesign performance
is established by its presence. Do not reuse its assertions as an oracle or
infer that its old comments verify a new similarly numbered requirement.

TEST-DATA's reference-tool section records idmptmat limitations: first
packet only, no standalone input, and nontermination in some summary/tree
cases. Real recording samples remain local-use data. No reference binary,
other repository, sample file or local computer is accessed in this slice.
No benchmark, Rust build, reference process or new tool install runs here.

## Pending owner item

PENDING OWNER: whether the tmats host must also do zero per-packet allocation
in steady state for non-setup packets, analogous to core L2-HOT-008. That
host allocation rule is not approved here. Allocation/copy metrics below are
reporting only, with no new numeric threshold; ADR-0036's existing borrowed-
span/no-extra-payload-copy assembly obligation still applies independently.
Retaining Criterion also needs owner approval before implementation chooses
that framework; no new or retained dev dependency is approved by this PR.

## Correct work before timing claims

L2-PERF-001 and L3-PERF-001–004 require separately scoped parser and recording
workloads. Recording traversal MUST use ADR-0036 borrowed-span assembly,
without coalescing or extra payload copies. A copying/coalescing run is not
accepted as recording-scan evidence, even if its output is correct or fast.
Any owned parse-source construction is a separately identified parser
workload, not an alternative recording-scan mode.
Before accepting times, compare retained bytes and the applicable
item/fragment/run/record inventory with independent fixture expectations.
A parser returning an early empty result, a scanner ignoring later setup
packets or an assembler counting each fragment as a record is not a faster
implementation of the required work.

Fixtures must use cited, reviewed source rules and explicit expected
quantities. Do not compute expectations by calling the same parser/assembler.
If completeness/access policy is unresolved for a case, retain its actual
qualification and missing evidence; timing cannot settle that interpretation.
A benchmark is not permission to use cut, ambiguous or unsupported data.

Text reading can measure an in-memory parser, while recording traversal can
include core acquisition and TMATS assembly. Record their actual boundaries:
source allocation/ownership, acquisition, indexing, checksum work, views and
rendering must not silently move in or out of the timer between runs.
Consume an observable result so optimization or unused work does not turn a
benchmark into a measurement of nothing.

Keep recording bytes and TMATS text bytes separate. Report units and actual
processed extents; a small setup's parsing rate is not a full recording's
scan rate. Cache and acquisition modes matter, so identify them rather than
invent a cold-cache policy or compare unlike modes without qualifications.
Allocation counts and payload-copy bytes must also be reported, measured
OUTSIDE the timer in a paired run with matching input/build/configuration
and workload scope. Use a test allocator like core's for allocation counts
and separate explicit instrumentation for payload-copy bytes; an allocator
alone does not count copies. Record counter definitions and scope. This is
reporting only, with no new numeric threshold; refusing coalescing enforces
ADR-0036 rather than inventing a metric budget. The reference to core's test
allocator is the supplied review direction, not a claim of cross-repository
source/API inspection or approval of an added dependency.
No output layout, file format or benchmark framework is chosen here.

## Same-file reference comparison with limitations visible

L2-PERF-002 and L3-PERF-005–008 require exact source/build/input identity,
versions/options, hardware/OS/architecture, compile-time target features
(separate from Cargo features) and run configuration. Use the
same input files for comparison and retain actual completion and consumed
extent for both tools. If idmptmat reads one packet while tmats finds later
records, the work differs: no unqualified whole-scan speedup ratio follows.

A timeout, rejected standalone input or semantic disagreement remains that
outcome, not a successful fast result or a fabricated matching record count.
Do not patch the reference or write another packet reader to manufacture
comparison equivalence. Correctness is decided against cited source/approved
rules, not whichever tool prints first or reports fewer records.

Initial evidence informs a later explicit performance-budget decision. A
numeric latency/throughput gate is not selected by this draft, nor is the
full single-pass/checksum-in-scanner architecture in ADR-0021 approved by
requiring measurement. Source preservation and correctness remain mandatory
independently of benchmark speed.

## Bounded compute and data handling

Future runs need an explicit finite input/time/memory/repetition plan,
owned-process cleanup and identifiable completed results. Synthesized heavy
work belongs in GitHub CI. Local checks stay light; this PR starts no
benchmark/reference process or background watcher.

Sample comparisons still require a data location and compute authorization
consistent with TEST-DATA's local-use restriction and the CI-heavy rule.
This documentation is not authorization to fetch/upload recordings or run
heavy local work. Missing sample/reference evidence remains missing until
that plan is available; synthetic results do not impersonate real samples.
No default timeout, file-size ceiling or fixed benchmark sample count is
invented. No new dependency is approved by selecting a measurement contract.

## Carried instructions after #61

For L3-ROB-009 compile-fail fixtures, use CI-scripted cargo clippy runs on
fixture crates. Do NOT add trybuild/ui_test or any dev dependency without
owner approval. The requirement and TEST-DATA note carry this instruction;
the ROADMAP places it under the guard-rail/coverage section, not performance.
This placement is called out in the PR description; no fixture framework or
dependency is installed or added by this PR.

Held #59 remains separate pending the owner's std decision. Before its
re-review it must rebase on then-current main and regenerate/recount trace
totals; the coordinator's expected counts were conditional on an unchanged
baseline. Newly merged requirement slices will change that baseline, so do
not reuse frozen totals or claim the old #59 head includes these additions.
No #59 branch update or std-policy resolution is made here.

## Verification gates

The new requirements are inspection-verifiable Draft entries with no
Evidence lines. TEST-DATA names planned measurement/acceptance cases. This
slice performs lightweight trace/whitespace/link checks only; no executable
performance claim follows from a passing documentation/prototype CI run.

Source freshness, meaningful executable fixtures and development analysis
remain required. Pattern scope/syntax, extraction, record access/governance,
report formats and the held std conflict remain unresolved. Lints, line
coverage and interpretation/requirements coverage stay separate from speed.
