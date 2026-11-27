# Family-2 0.1 coverage audit — 2026-11-27

## Verdict and inspected snapshots

**Family 2 is not complete.** Requirement children and planned fixtures are
not proof of complete decomposition or implementation. Do not begin family 3
or claim parser readiness merely from the number of merged docs batches.
The twelve-family sequence and full 0.1 scope remain unchanged.

Final inspected main: `f2437176937b4c8cb5a855745e07edd480319c18`, including
merged #72 at `6d21d4a` and merged #73 at
`8cc35f3291fca7f716abd1d9d079d6bb16a16167`. The initial inspection used
`afd0fa0e530d9ddcac50d0943a90dc205908dc70`; #73 merged while the audit was
being written, so the final totals/descendant check below were rerun on the
updated main. This is a dated source-inspection snapshot, not a live status
generator. Follow the regenerated TRACE-MATRIX for later totals.
Also inspected held [#59](https://github.com/TelemetryWorks/irig106-tmats/pull/59)
at `5748ba016f37cdcd4ac791d685345ca0df497c93`. Extraction's owner-policy
record [#73](https://github.com/TelemetryWorks/irig106-tmats/pull/73) is now
merged documentation, not implementation. The three #57 policies are decided; the later
clarification of --strict's refusal scope remains PENDING OWNER.

Main has L2 92, L3 306 and 33 L1 leaves: 431 countable requirements. Held
#59's requirements are not counted as merged. Scope is the 49 L1 parents
named by RELEASE-0.1's inventory and bounded time-view subset, plus
L1-WRT-007's minimum stamp prerequisites: 50 rows below. Full later-release
portions of shared parents are excluded from this audit, not silently
promoted into 0.1.

The trace shows zero redesigned Rust test-marker coverage. Its two evidenced
L1 MSRV/platform inspections refer to existing prototype CI configuration;
they do not qualify replacement crates, the exact irigtime consumer or these
new contracts. Source inspection and green docs/prototype CI do not execute
any planned parser, assembly, extraction, compatibility or release checks.
Source documents: [release scope](../RELEASE-0.1.md), [L1](../L1-REQ.md),
[L2](../L2-REQ.md), [L3](../L3-REQ.md), [trace matrix](../TRACE-MATRIX.md),
[planned fixtures](../TEST-DATA.md) and [interpretation register](../INTERPRETATIONS.md).

## Per-parent decomposition inventory

D = relevant structural children drafted, **not** a completeness/implementation
claim. P = explicitly partial derivation or missing source/policy detail.
M = no direct L2 child on main. H = missing from main, with held #59's draft.
Owner, source and development gates are shown separately in the last column.

Inventory categories: D 35, P 9, M 3, H 3.

| Scoped L1 | Primary children on inspected main | Class | Remaining scope/evidence boundary |
|---|---|---|---|
| [L1-READ-001](../L1-REQ.md#l1-read-001) | [L2-READ-001](../L2-REQ.md#l2-read-001), [L2-READ-002](../L2-REQ.md#l2-read-002), [L2-READ-007](../L2-REQ.md#l2-read-007) | D | Owned original source, ordered items and retention; replacement-store tests still absent. |
| [L1-READ-002](../L1-REQ.md#l1-read-002) | [L2-READ-010](../L2-REQ.md#l2-read-010) | D | Complete indices/canonical-string identity; independent source/index fixtures still to execute. |
| [L1-READ-003](../L1-REQ.md#l1-read-003) | [L2-READ-004](../L2-REQ.md#l2-read-004), [L2-READ-006](../L2-REQ.md#l2-read-006), [L2-READ-009](../L2-REQ.md#l2-read-009), [L2-READ-012](../L2-REQ.md#l2-read-012), [L2-READ-013](../L2-REQ.md#l2-read-013) | D | Recovery/character/name/duplicate branches; approved interpretations still need development analysis/tests. |
| [L1-READ-004](../L1-REQ.md#l1-read-004) | [L2-READ-005](../L2-REQ.md#l2-read-005) | D | ASCII comparison and original bytes; no Unicode/locale substitution. |
| [L1-READ-005](../L1-REQ.md#l1-read-005) | [L2-READ-011](../L2-REQ.md#l2-read-011) | D | All baseline groups and COMMENT recognition; private meaning is not implied. |
| [L1-READ-006](../L1-REQ.md#l1-read-006) | [L2-READ-003](../L2-REQ.md#l2-read-003), [L2-READ-008](../L2-REQ.md#l2-read-008) | D | First colon/semicolon framing and name blanks; value policy remains field-specific. |
| [L1-READ-007](../L1-REQ.md#l1-read-007) | [L2-READ-014](../L2-REQ.md#l2-read-014), [L2-READ-015](../L2-REQ.md#l2-read-015) | D | Suspect warning only; mandatory Appendix vocabulary inventory and S1 real-data confirmation remain. |
| [L1-CH10-001](../L1-REQ.md#l1-ch10-001) | [L2-CH10-001](../L2-REQ.md#l2-ch10-001) | P | Borrowed baseline slicing exists on paper; historical layout/version bindings remain gated. |
| [L1-CH10-002](../L1-REQ.md#l1-ch10-002) | [L2-CH10-002](../L2-REQ.md#l2-ch10-002) | P | Baseline format/SRCC/RCCVER and unsupported XML; historical layout interpretations need review. |
| [L1-CH10-004](../L1-REQ.md#l1-ch10-004) | [L2-CH10-004](../L2-REQ.md#l2-ch10-004), [L2-CH10-005](../L2-REQ.md#l2-ch10-005), [L2-CH10-006](../L2-REQ.md#l2-ch10-006), [L2-CH10-007](../L2-REQ.md#l2-ch10-007) | P | Joining/provenance and matrix contracts drafted; precise completeness, provisional use and resource policy remain open. |
| [L1-CH10-005](../L1-REQ.md#l1-ch10-005) | [L2-CH10-008](../L2-REQ.md#l2-ch10-008), [L2-CH10-009](../L2-REQ.md#l2-ch10-009) | P | Mixture scope/L1 wording are recommended A, pending owner; F7 identity/prior scope not established. |
| [L1-CH10-006](../L1-REQ.md#l1-ch10-006) | [L2-CH10-003](../L2-REQ.md#l2-ch10-003) | D | INT-012 literal Format-1 table reconfirmed; fallback policy/other code spaces stay separate. |
| [L1-EDN-001](../L1-REQ.md#l1-edn-001) | [L2-EDN-003](../L2-REQ.md#l2-edn-003), [L2-EDN-004](../L2-REQ.md#l2-edn-004) | D | Common text reader; no claim of historical semantic coverage or edition-independent packet routing. |
| [L1-EDN-002](../L1-REQ.md#l1-edn-002) | [L2-EDN-002](../L2-REQ.md#l2-edn-002) | D | Declaration/override/fallback requirements exist; relevant legacy interpretation reviews remain gates. |
| [L1-EDN-005](../L1-REQ.md#l1-edn-005) | [L2-EDN-001](../L2-REQ.md#l2-edn-001) | D | Raw G\106 and CSDW declarations kept separate; historical mappings remain source/review gated. |
| [L1-VIEW-001](../L1-REQ.md#l1-view-001) | [L2-VIEW-001](../L2-REQ.md#l2-view-001), [L2-VIEW-015](../L2-REQ.md#l2-view-015) | D | Exact/pattern candidate retrieval; public pattern scope/syntax remain pending owner. |
| [L1-VIEW-004](../L1-REQ.md#l1-view-004) | [L2-VIEW-002](../L2-REQ.md#l2-view-002), [L2-VIEW-003](../L2-REQ.md#l2-view-003), [L2-VIEW-004](../L2-REQ.md#l2-view-004), [L2-VIEW-011](../L2-REQ.md#l2-view-011) | D | Bounded time-field state/default/provenance subset only; general effective values remain later. |
| [L1-VIEW-005](../L1-REQ.md#l1-view-005) | [L2-VIEW-008](../L2-REQ.md#l2-view-008), [L2-VIEW-009](../L2-REQ.md#l2-view-009) | D | Explicit valid NSB split and preserved absence/invalidity; numeric syntax approval is recorded. |
| [L1-VIEW-006](../L1-REQ.md#l1-view-006) | None on main | M | No direct L2 for the 0.1 bounded lookup-state slice; READ duplicate and VIEW state children under other parents are related coverage, not this missing trace. |
| [L1-VIEW-007](../L1-REQ.md#l1-view-007) | [L2-VIEW-005](../L2-REQ.md#l2-view-005), [L2-VIEW-006](../L2-REQ.md#l2-view-006), [L2-VIEW-007](../L2-REQ.md#l2-view-007), [L2-VIEW-010](../L2-REQ.md#l2-view-010), [L2-VIEW-012](../L2-REQ.md#l2-view-012), [L2-VIEW-013](../L2-REQ.md#l2-view-013) | D | Bounded TIMEIN/formats/source context/association uncertainty; no positive data-to-clock evidence rule exists in 0.1. |
| [L1-VIEW-008](../L1-REQ.md#l1-view-008) | [L2-VIEW-014](../L2-REQ.md#l2-view-014) | D | Bounded explicit caller assumptions, levels and suppression evidence; rendered format remains held. |
| [L1-REG-006](../L1-REQ.md#l1-reg-006) | [L2-REG-006](../L2-REQ.md#l2-reg-006), [L2-REG-007](../L2-REQ.md#l2-reg-007) | D | Readiness/register gates drafted; approved entries are not implemented without freshness/analysis/executable tests. |
| [L1-EXT-003](../L1-REQ.md#l1-ext-003) | [L2-EXT-001](../L2-REQ.md#l2-ext-001), [L2-EXT-002](../L2-REQ.md#l2-ext-002), [L2-EXT-003](../L2-REQ.md#l2-ext-003), [L2-EXT-004](../L2-REQ.md#l2-ext-004) | P | V/X structures drafted; proposed INT-061/062/063 block the affected mapping/vocabulary/vendor choices. |
| [L1-WRT-001](../L1-REQ.md#l1-wrt-001) | [L2-WRT-001](../L2-REQ.md#l2-wrt-001), [L2-WRT-003](../L2-REQ.md#l2-wrt-003) | D | Exact byte replay and fallible writer delivery; replacement-code verification still required. |
| [L1-WRT-002](../L1-REQ.md#l1-wrt-002) | [L2-WRT-004](../L2-REQ.md#l2-wrt-004) | P | Explicit-request boundary only; actual order/line-ending/counter transformations are not fully derived. Resolve scope/rules without dropping 0.1 obligation. |
| [L1-SUM-001](../L1-REQ.md#l1-sum-001) | [L2-SUM-001](../L2-REQ.md#l2-sum-001) | D | Raw authoritative exclusion/domain rules drafted; source/completeness and independent digest gates remain. |
| [L1-SUM-002](../L1-REQ.md#l1-sum-002) | [L2-SUM-002](../L2-REQ.md#l2-sum-002) | D | Operation-specific verdicts and warnings; no core-integrity or whole-document upgrade. |
| [L1-SUM-003](../L1-REQ.md#l1-sum-003) | [L2-SUM-004](../L2-REQ.md#l2-sum-004), [L2-SUM-005](../L2-REQ.md#l2-sum-005) | P | Reference choice/revision pending owner; qualified source/arithmetic and fixed independent vectors unfinished. |
| [L1-SUM-004](../L1-REQ.md#l1-sum-004) | [L2-SUM-003](../L2-REQ.md#l2-sum-003) | D | Minimal final-byte/re-read stamp transaction; general editing stays outside the 0.1 slice. |
| [L1-IO-001](../L1-REQ.md#l1-io-001) | [L2-IO-001](../L2-REQ.md#l2-io-001) | D | Library-owned I/O prohibition; actual redesigned API/reachability inspection still needed. |
| [L1-ROB-001](../L1-REQ.md#l1-rob-001) | [L2-ROB-001](../L2-REQ.md#l2-rob-001), [L2-ROB-002](../L2-REQ.md#l2-rob-002), [L2-ROB-003](../L2-REQ.md#l2-rob-003) | P | Checked spans/termination/open-run budget contracts exist; concrete accounting/limits/recovery and bounded executable campaigns unfinished. |
| [L1-ROB-002](../L1-REQ.md#l1-rob-002) | [L2-ROB-004](../L2-REQ.md#l2-rob-004) | D | Guard rails and CI-scripted Clippy fixtures; no trybuild/ui_test/new dev dependency approved. |
| [L1-ROB-003](../L1-REQ.md#l1-rob-003) | [L2-ROB-005](../L2-REQ.md#l2-rob-005) | D | Existing 80% library line floor and separate requirements/semantic evidence; no inherited prototype acceptance. |
| [L1-CLI-001](../L1-REQ.md#l1-cli-001) | [L2-CLI-005](../L2-REQ.md#l2-cli-005), [L2-CLI-006](../L2-REQ.md#l2-cli-006) | D | Content routing/acquisition boundary, core reader only; parser success is not valid input. |
| [L1-CLI-002](../L1-REQ.md#l1-cli-002) | None on main | M | Raw/tree/channel-summary report derivation absent; stop for owner report-format decision, not a help-only substitute. |
| [L1-CLI-003](../L1-REQ.md#l1-cli-003) | [L2-CLI-010](../L2-REQ.md#l2-cli-010), [L2-CLI-011](../L2-REQ.md#l2-cli-011) | D | Internal all-record identity/order; first-fragment meaning and public/live selection policy remain owner-gated. |
| [L1-CLI-004](../L1-REQ.md#l1-cli-004) | [L2-CLI-008](../L2-REQ.md#l2-cli-008) | D | Byte/destination contracts and accepted 2026-11-27 owner policy are in merged #73; strict listed-only versus any-finding scope is pending owner; implementation/platform/core-access evidence remains. |
| [L1-CLI-005](../L1-REQ.md#l1-cli-005) | [L2-CLI-012](../L2-REQ.md#l2-cli-012), [L2-CLI-013](../L2-REQ.md#l2-cli-013) | P | Library command propagation drafted; flex detail and actual command grammar/report integration remain incomplete. |
| [L1-CLI-006](../L1-REQ.md#l1-cli-006) | None on main | M | Versioned JSON/plain reports and public exit mappings not derived; explicit owner report-format hold. |
| [L1-CLI-007](../L1-REQ.md#l1-cli-007) | [L2-CLI-009](../L2-REQ.md#l2-cli-009) | D | Tool-opened input-alias/delivery safety; #73 records force proof exception while known aliases always refuse. |
| [L1-CLI-008](../L1-REQ.md#l1-cli-008) | [L2-CLI-003](../L2-REQ.md#l2-cli-003), [L2-CLI-004](../L2-REQ.md#l2-cli-004) | D | Core evidence and borrowed DATA slicing; published types/core 0.2 and real API compatibility still need evidence. |
| [L1-CLI-009](../L1-REQ.md#l1-cli-009) | [L2-CLI-001](../L2-REQ.md#l2-cli-001), [L2-CLI-002](../L2-REQ.md#l2-cli-002), [L2-CLI-007](../L2-REQ.md#l2-cli-007), [L2-CLI-014](../L2-REQ.md#l2-cli-014), [L2-CLI-015](../L2-REQ.md#l2-cli-015) | D | Shared runner/individual APIs/parser mechanism; every actual reviewed row still needs integration tests. |
| [L1-PERF-001](../L1-REQ.md#l1-perf-001) | [L2-PERF-001](../L2-REQ.md#l2-perf-001), [L2-PERF-002](../L2-REQ.md#l2-perf-002) | D | Independent-work parser/borrowed-span traversal evidence, untimed allocations/copy counts; numeric budgets await measurement. Retaining prototype Criterion is pending owner (#62). |
| [L1-REL-001](../L1-REQ.md#l1-rel-001) | [L2-REL-001](../L2-REQ.md#l2-rel-001) | D | ADR-0035 virtual crates/apps layout and lockstep exact path/version; actual restructure not done. |
| [L1-REL-002](../L1-REQ.md#l1-rel-002) | None on main | H | No direct child on main: L2/L3 slice held in #59 on owner std decision; rebase/recount before re-review. |
| [L1-REL-003](../L1-REQ.md#l1-rel-003) | None on main | H | Plain-library WASM obligation; held #59 is not main evidence or WASM-bindings approval. |
| [L1-REL-004](../L1-REQ.md#l1-rel-004) | None on main | H | Native target/MSRV/consumer acceptance in held #59; published dependency compatibility unverified. |
| [L1-REL-005](../L1-REQ.md#l1-rel-005) | [L2-REL-005](../L2-REQ.md#l2-rel-005) | D | Actual graph/deny policy and published types/core-only release graph; no new dependency/exception approval. |
| [L1-REL-006](../L1-REQ.md#l1-rel-006) | [L2-REL-006](../L2-REQ.md#l2-rel-006) | D | Org binary variant artifact acceptance drafted; no release pipeline or publication evidence. |
| [L1-WRT-007](../L1-REQ.md#l1-wrt-007) | [L2-WRT-002](../L2-REQ.md#l2-wrt-002) | D | Auxiliary 0.1 minimum stamp transaction only; full general edits remain 0.5. |

## L3 and planned-case linkage check

The primary chains above reach 84 L2 and 284 L3 descendants on the inspected
snapshot (260 Test and 24 Inspection methods). These include shared-parent
detail; a count does not promote later
parts into 0.1. The check parses actual L3 Parent fields, then searches named
TEST-DATA table rows for their requirement references, not Rust test results.

283 descendants have at least one such table-row reference. The exception is
L3-IO-001, an Inspection requirement: absence of a Rust fixture is not a test
gap, but redesigned API/reachable-I/O inspection and its named Evidence are
still absent. Table-row presence for the others proves only documentary
linkage, not a sufficient independent oracle, complete branches, executed
check or correct verification method. Non-Test requirements still require
their actual inspection/analysis artifact; counted planned references cannot
turn them into passing Evidence. No trace status is manually upgraded.

## Findings that require further work

1. **Bounded L1-VIEW-006 lacks its own child.** Existing READ multiplicity
   and VIEW field-state tests are related, but the direct trace is missing.
   Next unheld requirement slice: derive the bounded time-view lookup states
   and no-winner/candidate retention contract from already approved rules,
   with primary parenting and independent cases. Do not add general 0.2
   link resolution or invent a clock-association proof source.
2. **Show/report formats remain owner-held.** L1-CLI-002/006 have no direct
   children; raw/tree/summary, findings, assumptions/suppression markers,
   JSON/versioning and exit presentation cannot be settled through parser
   tables or stubs. STOP before deriving those formats and obtain the owner
   decision. This audit selects no option or rendering.
3. **Build acceptance is held, not absent from the plan.** #59 reserves
   L2-REL-002–004/L3-REL-002–008 pending the std-feature decision. Rebase and
   regenerate/recount on then-current main before re-review; the old expected
   totals are stale. Exact irigtime features=[std] consumer CI starts once the
   time view exists. A CI-only source override never qualifies the release
   graph; types/core 0.2 must be published and compatible from crates.io.
4. **Normalization is partial.** L2-WRT-004 explicitly derives only opt-in,
   not actual transformation rules. L1-WRT-002 remains in 0.1; later CLI
   clean-up placement does not erase it. Clarify/derive the required library
   capability and source-cited transformations before claiming coverage;
   neither implement repairs silently nor silently cut the release scope.
5. **Flex arithmetic/source qualification is unfinished.** Owner reference/
   revision choice is pending. Later reference runs are bounded GitHub CI,
   synthetic only, no new project dependency; retain values/revision/hashes,
   never paste reference source. Existing prose and wrapper mocks are not
   independently fixed vectors or a complete byte/arithmetic specification.
6. **Assembly/session/resource detail remains gated.** INT-034 precise
   completeness, provisional use/invalidation, general INT-028/031/032/033
   access/governance, aggregate accounting/defaults/recovery, and first-fragment
   file-order confirmation remain open where recorded. The host's possible
   zero-allocation steady-state rule for non-setup packets is still pending
   owner; borrowed DATA's zero extra payload copy does not decide it.
   Mixture scope and L1
   channel wording are recommended A, PENDING OWNER. F7 approval establishes
   indeterminate-until-identified, not event identity or what prior means.
   Do not claim a determinate missing-event check or completed full-parent
   acceptance without source evidence/owner reconciliation.
7. **V/X and reviewed metadata are not automatically ready.** Proposed
   INT-061/062/063, public pattern scope/syntax and missing historical
   definition/character policies retain their gates. The 0.1 minimal
   recognizer must inventory Appendix 9-C names/index patterns; a missing
   name is visible coverage failure, never a clean missed warning. Full
   baseline registry generation/validation and general views remain later.
8. **Approved behavior still needs development evidence.** Complete the
   written source-freshness/development analyses and meaningful executable
   fixtures, including the full retransmission product and QM sequences,
   independent SHA/flex results, byte replay, qualification, core evidence
   independence and budgets. Legacy INT-001–034 pending AI reviews are not
   grandfathered. Do not replace this with a requirement-ID count.

## Owner items and the work they block

This table separates outstanding owner choices from already approved policies
whose source/interpretation evidence is unfinished. Criterion retention from
#62 is explicitly owner-held; an existing prototype dependency is not approval
to retain it in the redesign or add another benchmarking dependency.

| Owner item | Status/boundary | What it blocks |
|---|---|---|
| Report format | PENDING OWNER; no renderer/schema chosen here | L1-CLI-002/006, raw/tree/summary show presentation, findings/assumption/suppression rendering, JSON and public exit presentation |
| std feature | PENDING OWNER; #59 held | #59, L1-REL-002–004 derivations/acceptance and the exact irigtime consumer feature check |
| Flex reference/revision | PENDING OWNER, bbaggerman versus atac source | Qualified flex vectors, exact arithmetic/source derivation and flex implementation; reference runs remain bounded synthetic-only CI, no new project dependency or pasted source |
| INT-061/062/063 | Proposed, owner sign-off/review pending | Affected X attachment/unindexed mapping/vocabulary and V attribution; not raw source retention |
| INT-031 | Proposed; no general access exception approved | Semantic/governing use of incomplete/ambiguous records; ADR-0037 raw extract is separate and cut facets remain blocking for semantic use |
| Pattern scope/syntax | PENDING OWNER | UC-03 CLI querying, public pattern language/escaping and approved 0.1 query scope; not already defined internal raw candidate retention |
| Host allocation | PENDING OWNER | Whether the tmats host must enforce zero steady-state per-packet allocation for non-setup traffic; borrowed-DATA zero extra copy does not decide it |
| Mixture scope | Recommended Complete-only A, PENDING OWNER | Formal ASCII/XML mixture-finding eligibility, especially Complete ASCII plus Incomplete XML; qualified FRMT retention remains defined |
| L1-CH10-005 wording | Recommended A, PENDING OWNER after source re-verification | Wording/acceptance alignment for setup-on-0 from 106-11 versus 106-13/17 exclusivity changes; core still owns routing/profile evidence |
| File order | First-fragment meaning recommended, owner confirmation pending | Final public record ordering/selection interpretation for interleaved runs and its dependent CLI contract; no completion-order default inferred |
| F7 event identity/prior scope | Indeterminate policy approved; identity still unverified, prior meaning needs its own reviewed interpretation | Determinate configuration-change-event presence/absence check; a generic Format 2 candidate is not proof; do not reopen the approved uncertainty policy |
| --strict scope | PENDING OWNER: listed bad-data kinds versus any finding | Strict behavior for unlisted findings, including the information-only fixture; listed-kind/no-finding cases and default raw extraction are common to both options |
| Criterion retention (#62) | PENDING OWNER | Keeping the prototype Criterion dev dependency and accepting a harness that relies on it; no automatic new benchmark dependency approval |
| Completeness/provisional/resource policy | Open where recorded under INT-034 and related governance work | Precise unresolved classification, provisional semantic use/invalidation and aggregate default/accounting/recovery policy; unheld drafting can cover already decided portions only |

## Extract policy is decided, not still an owner blocker

The owner decisions of 2026-11-27 remove #57's overwrite, identity and
finding-eligibility holds. Merged #73 records them verbatim and explicitly makes
--force/--strict owner-approved spellings, with remaining grammar held.
Force can authorize unproven regular identity, NEVER a known input alias.
Default raw extract reports every finding and preserves bytes. Strict refuses
the listed bad-data kinds; whether it also refuses every other finding is
PENDING OWNER. The earlier any-finding shorthand is not a decided predicate.
No finding is default always-refuse. This does not make cut/
ambiguous/invalid input semantically usable or governing, nor invent a core
raw extent. Do not repeat #59's historical extraction-hold prose as current
owner status. The final inspected main includes #73's removal of those stale
holds; do not ask for the same decisions again. This audit PR also marks
--strict scope pending in current L2/L3/CLI/ROADMAP/fixtures without changing
ADR-0037's verbatim text or older research. Listed-kind and no-finding cases
are common to both options; the information-only other-finding strict arm is
explicitly blocked. No new owner answer or recommendation is invented.

## Family-3 readiness and recommended order

The previous one-audit/very-few-batches estimate was too low: counting absent
parent links missed substantial partial-contract work. Revised planning
estimate is **about 4–6 unheld batches** plus **about 5–8 owner-gated batches**,
roughly **9–14 before family 3**, subject to review size and owner/source
answers. This is not a promise of a completion date or permission to bypass
gates. The unheld estimate includes the companion trace relabel requested
with this review; it is not another owner decision.

Unheld work comprises bounded VIEW-006's direct decomposition, explicit
normalization transformation definitions, already-decided assembly/resource
detail, show-free CLI integration contracts/grammar-independent computation,
and the
separate prototype-evidence trace relabel. These may split or combine to
stay reviewable. Unresolved classification/resource choices remain in the
owner-gated column; do not decide them inside an otherwise unheld batch.

Owner-gated batches cover the table above, notably report/show formats, std,
flex qualification and the pending V/X, access, pattern, session/order/event,
allocation, Criterion and strict-scope items. They can be grouped only when
the actual answers and source evidence support doing so. Route existing
questions instead of inventing defaults or silently reducing 0.1 scope.
Finish the unheld VIEW-006 slice next and reassess after each merged answer.

A **separate small PR** relabels L1-REL-002/004's current prototype Evidence
as prototype configuration and regenerates TRACE-MATRIX, so its Verified
rollup no longer counts those two configuration references as readiness.
The inspected snapshot above describes the earlier state; relabelling is
not execution of replacement-code MSRV/platform checks or resolution of #59.

Once applicable family-2 gates are satisfied, the already decided family 3
is ADR-0035's virtual root, crates/irig106-tmats and apps/irig106-tmats-cli,
shared exact-version lockstep dependency and prototype removal under W2.
That decision needs no new layout permission, but this audit does not start
code, remove the prototype or authorize unapproved dependencies/features.

Verify path/trace/fuzz/CI/package references after that move. All substantive
Rust/MSRV/target/consumer/coverage/fuzz/performance checks use bounded GitHub
CI; no current prototype pass is replacement-code acceptance. Stop own
background jobs, preserve arbitrary source, keep no trailers and independent
exact-head PR review. Release execution remains later TP-only work with the
existing coordinator/owner bootstrap exception; no release action occurs here.

## Reproduction and practical limits

The local audit parses primary Parent links from L2/L3, compares the explicit
50-row scope with RELEASE-0.1's inventory, verifies all links/IDs and runs
scripts/build-trace-matrix.py --check. Six parents have no direct L2 on this
snapshot: VIEW-006, CLI-002/006 and REL-002/003/004. This is a source/trace
finding, not proof that every other parent is fully decomposed. The partial
class comes from inspecting the actual statements and their stated gaps.

No Rust test/build, dependency install, sample/reference execution, packaging
or external repository inspection is performed by this audit. No new test,
requirement ID, interpretation decision, policy exception or Evidence line
is added. Heavy CI for the audit head only checks the current checkout; it
cannot certify the unimplemented planned checks listed here. Earlier dated
research remains historical, and both pending A choices remain pending.
