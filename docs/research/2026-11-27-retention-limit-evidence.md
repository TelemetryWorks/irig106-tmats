# Retained-resource and typed-limit evidence: family 2 batch 42

Date: 2026-11-27. Documentation-only decomposition; no implementation,
new interpretation, accounting policy or owner approval is claimed.

## Existing authority

L1-ROB-001 requires bounded resource handling. L2-ROB-003 and
L3-ROB-004/005/006 already require an explicit finite aggregate budget,
checked charging, typed failure and no silent eviction or manufactured
joining/finality event. They leave charging details and recovery under review.
[ADR-0036](../adr/0036-core-reader-and-borrowed-data-spans.md) requires direct
borrowed non-contiguous DATA spans, retained backing ownership and accounting
for retained block owners and metadata. Zero payload copies do not imply
zero retained memory. Core remains the sole reader and integrity authority.

The [batch 15 draft](2026-11-23-completeness-contracts-draft.md) recommends
pre-admission failure with unchanged state, but does not approve failure
atomicity or a resume API. This batch does not turn that recommendation into
a requirement. No Chapter 9 code name, range, default or interpretation is
introduced: these are product robustness and architecture obligations.

## Inventory before charging

The new L2-ROB-006 / L3-ROB-014..016 separate retention facts from charging.
Identify the declared ownership domain, channels/runs, fragments/spans,
backing owners and live references, owned text where applicable, provenance,
assembler-retained snapshots and error-path storage. Distinguish used bytes
from retained capacity and explicitly unavailable information.

A small slice can retain a larger backing allocation; an empty text fragment
still retains metadata. Two channels may refer to one owner. Releasing one
reference cannot prove that owner is freed while another remains. Handoff
outside the assembler budget domain is not deallocation or a process-wide
memory reduction. A completed record held by a caller may outlive its run.

This inventory is internal verification evidence, not a new public report,
Rust signature, owner-ID field on PacketFacts/PacketInfo or renderer/schema.
Actual ownership/capacity APIs or caller descriptors need verification before
implementation; header lengths cannot invent those facts. Which shared or
caller-owned bytes to charge, allocator overhead, and conservative bounds
remain separate review work. An unknown required quantity is not zero.

A production hard-bound claim needs a reviewed accounting model covering its
whole stated domain, including metadata-only streams and error/snapshot
state. Synthetic inventories and supplied charges test contract mechanics;
they cannot qualify actual allocation behavior or prove a process heap/RSS
bound. The host's non-setup steady-state allocation target remains owner-held.

## Arithmetic and operational failure

L2-ROB-007 / L3-ROB-017..019 refine existing typed failure evidence without
selecting rollback or partial acceptance. A proposed full operation over the
remaining limit or arithmetic domain cannot be reported fully admitted within
budget. Equality with a byte limit is not excess of that limit; other declared
constraints may still reject it. Test small synthetic domains, not enormous
source allocations. Native implementation checks later need appropriate CI
profiles so debug/release overflow behavior cannot hide a defect.

Record the selected model/limit, operation/run/channel, input provenance and
actual accepted/retained/returned/rejected source extents and ownership.
Injected fixture dispositions can test the common ledger for whole rejection
or an explicit accepted prefix. That does not approve either production API,
safe retry, resumability or continuing semantic use after failure.

Error-path storage belongs in the inventory if retained by the assembler.
An uncharged full-payload error copy cannot evade the budget. There is no
spill-to-disk or forced-eviction permission. A resource failure is not EOF,
a malformed-data finding, a joining stop, checksum success or governing
setup evidence. Preserve record qualifications and accepted source provenance.

The raw extract direction does not invent unavailable source extents or make
an unavailable operational result successful. This adds no always-refuse
bad-data category and does not decide the pending extract strict predicate.

## Verification and remaining gates

The 18 planned TEST-DATA cases specify independent category/reference
inventories, supplied charges, exact arithmetic and source dispositions.
They include sharing across channels, empty fragments, retained snapshots,
unknown capacity, error storage and owner handoff. Production code must verify
its real ownership/accounting, beyond these fixture-model checks.

No executable resource test is claimed here. Local validation is limited to
requirement/fixture links, generated trace consistency and whitespace. GitHub
CI runs the existing checks; no dependencies, tools, local Rust build or
background job are needed for this documentation change.

Numeric budgets/defaults, shared/caller/allocator charging, failure atomicity
and recovery remain review work. Provisional semantic use and incomplete/
ambiguous access remain gated (including proposed INT-031); normalization
INT-064/065/066 await the owner. Existing report-format, std/#59, Criterion,
flex reference, X/V, pattern, session/mixture, ordering and host-allocation
holds are unchanged. No family 3 restructure or parser code starts here.
