# Family 2 batch 15: completeness, streaming observations and memory limits

Date: 2026-11-23. Status: requirements/acceptance draft, not implementation
or completed development analysis. INT-011 reconfirmation is separately
recorded with AI concurrence on #38 at c623b15.

## Source and scope

106-24R1 Chapter 11 section 11.2.7.2 page 11-50 supplies fragmentation but
no end-of-record marker. Section 11.2.1.1 f makes counters per-channel.
Chapter 9 section 9.4.2 page 9-2 frames attributes with semicolons and says:

> Semicolons are not allowed in any data item (including comment items).

ADR-0031 accepts three final outcomes in principle, with precise INT-034
rules still proposed. Same-channel joining in reconfirmed INT-011 does not
approve those rules, INT-028 governance or INT-031 invalidation. This slice
adds L2-CH10-006/007, L2-ROB-003, L3-CH10-017–021 and L3-ROB-004–006.
It does not derive a configuration timeline or reduce the full 0.1 scope.

## Closed-run evidence

The reviewer consequence is explicit: "followed by another packet" refers
to SAME-CHANNEL traffic, not interleaved channels. An otherwise complete
trusted ASCII run ending exactly at `;` at EOF is Complete labelled "end of
input". EOF alone is not a downgrade; normal "start of supplied input"
evidence does not invent leading truncation. Independent known damage or
split evidence still needs a precedence rule. No source bytes are repaired.

INT-034's current wording also has unresolved edges: known leading loss,
empty/padding-only bodies, trailing controls/padding under approved
INT-035/036, and duplicate-versus-split evidence under INT-039/044. This
batch retains those as open questions rather than approving an algorithm.
An XML body remains unsupported/unknown completeness despite a final `;`.

## Streaming-use options for the owner

**Option A, recommended: explicit opt-in provisional semantic snapshots.**
Expose an observed extent/revision with PROVISIONAL and its reason, allow a
consumer explicitly to use the labelled snapshot before EOF, and retain any
assumptions and later invalidation. The still-open run continues joining.
This avoids waiting for EOF in irig106-time's CLI but requires a reviewed
contract for typed reads, authority, later appended duplicates/damage and
outputs already emitted. It must reconcile the complete-only parse/checksum
boundary and INT-028/031; it must not silently promote a snapshot to the
complete governing record. Details and owner sign-off remain outstanding.

**Option B: final-only semantic use.** Expose raw PROVISIONAL observations
but wait for actual closure/completeness before semantic adoption. This
preserves the current boundary directly but can indefinitely delay streaming
consumers while other-channel data continues. A later input pause must not
be forged into EOF or another-channel packet into a joining stop to avoid
that delay. Neither option is selected in this draft.

Provisional is a lifecycle observation, not a fourth final completeness
outcome. Requirements specify its label/evidence and no false authority;
consumer permission, snapshot API and invalidation remain an open INT-034/
governance item for owner review. No new interpretation is approved.

## Bounded retention

Independent channels can hold several runs open concurrently. The budget
must be explicit, finite and aggregate; a typed operational limit error is
separate from malformed data or record finality. No hidden eviction, merge,
truncation, fabricated EOF or local spill I/O is allowed. Include text,
metadata, provenance and assembler-retained snapshots in documented accounting.

Review before implementation: numeric defaults if any; allocator overhead
and shared/caller ownership; failure atomicity and resume behavior. Recommend
caller-configurable finite aggregate bytes and failure before accepting an
over-budget fragment, with accepted state unchanged and rejected ownership
explicit. This is a recommendation, not a silently chosen API or numeric
budget. No error-path buffer may evade the bound. Planned fixtures use an
independently supplied accounting model, not claimed production allocation
measurements. A metadata-only stream must not bypass retention accounting.

## Evidence and remaining gates

Planned cases cover clean EOF, same-channel stopping, open provisional
extension, XML, unresolved damage/ambiguity, aggregate budgets, exact limit
boundaries and overflow. All requirements remain Draft. Review/analysis and
executable tests are still needed; light document consistency and synthetic
arithmetic checks do not verify implementation, performance or hard memory
bounds. No builds, background jobs, release or publishing actions belong here.
