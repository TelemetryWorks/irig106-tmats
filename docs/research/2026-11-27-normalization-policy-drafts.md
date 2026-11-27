# Explicit normalization policy drafts: family 2 batch 41

Date: 2026-11-27. Docs-only contracts and proposed INT-064/065/066.
No interpretation is approved or implemented by this batch.

## Existing authority and source gap

L1-WRT-002 and UC-09 require normalized output only on explicit request,
including canonical order, consistent line endings and recomputed counters.
Accepted ADR-0002 makes original ordered bytes authoritative; ADR-0007
requires explicit apply, not silent repair. L2-WRT-004/L3-WRT-010/011 derive
the opt-in boundary but explicitly do not define concrete transform rules.
The coverage audit therefore cannot count the normalizer as fully derived.

Inspected 106-24R1 Chapter 9 sections 9.4.1/9.4.2 p. 9-2: order is free,
LF/CR help readability and field delimiters/blanks have separate rules. No
standard canonical comparator or newline representation is prescribed there.
Table 9-7 NUMBER OF FRAGMENTS p. 9-117 defines D-x\MNF\N-y-n-m as the
number of occupied word positions, range 1–8; D-x\WP-y-n-m-e supplies
members. It does not say to rewrite a counter to the highest observed index.
[Official PDF](https://www.trmc.osd.mil/wiki/download/attachments/335389015/Chapter9.pdf?api=v2),
source SHA-256:
`dfc5ebfb3e6f02f58389126863d43a45d89c7e2d0533a1c690d9537531c740cb`.

Approved INT-035/036/038/039/040/055 distinguish framing, raw/projection,
canonical identity, duplicates, membership uncertainty and numeric ranges.
None approves counter mutation or replacing raw controls with a view. This
batch records policy choices before implementation under ADR-0033 rather
than inventing defaults. Earlier research stays historical.

## Plain-English owner questions and recommendations

**INT-064 — what order does explicit normalization use?**
A (recommended) takes the caller's explicit complete-region order, verifies
that every region occurs once and replays unchanged chunks in that order.
B supplies a built-in stable projected-key tuple sort, with comments and
unknown/malformed content anchored as barriers; ties retain source order.
C waits for a fixed built-in profile before allowing order transformation.
A avoids pretending the standard mandates one canonical order, at the cost
of requiring the caller to identify the policy/plan. It does not silently
create a built-in canonical format. All mappings must preserve bytes and
establish prefix/suffix/layout ownership; unavailable mapping refuses.

**INT-065 — which line endings can change?**
A (recommended) converts only positively identified layout to caller-chosen
LF or CRLF, preserving CR/LF inside names/values and unknown content. B
converts all raw CR/LF, including fields; C requires every exact range from
the caller. A protects description/COMMENT/damaged field source, but cannot
claim a globally uniform raw newline style when field controls remain.
No platform default, BOM removal, padding deletion or missing terminator
repair is included. CRLF is one newline token, not two.

**INT-066 — when may counters be recomputed?**
A (recommended) requires a unique complete target, established applicable
scope and complete distinct contiguous members 1..k, then range-checks k and
writes canonical decimal. A deliberately refuses gaps/compatible damage,
missing/duplicate targets and unknown ranges; it never inserts/renumbers.
With those prerequisites, A would allow an explicitly requested recomputation
to replace a malformed old VALUE, retaining the old failure with its revision.
B writes distinct observed count despite gaps, preserving other findings.
C uses highest index, rejected in the draft because an index bound is not
member cardinality. A avoids hiding truncation but refuses more requests
than B. Invalid child values still contribute structural members; leading-zero
index spellings use INT-038 identity, not duplicate member counts.

These are PROPOSED, not accepted answers. Each needs owner sign-off and an
independent AI check, plus source freshness, written development analysis and
intensive executable tests. Policy-specific expected-byte fixtures are BLOCKED.
No decision is inferred from the word canonical or from a normalizer label.

## Common bounded contracts that can be reviewed now

L2-WRT-005/006 and L3-WRT-012–017 bind requests to source revision, explicit
transform kinds and selected policies. Preserve original source immutable,
retain every region/occurrence and exact untouched bytes, check mappings,
reject stale/conflicting or unresolved compound plans and avoid guessed
partial success. An explicit compound plan needs its sequencing/correspondence
specified; no automatic transform pipeline is invented.

New output has new item/span correspondence and can be reevaluated under
reviewed rules. Original findings remain attached to their original revision.
Normalization success alone is not field/record conformance or governing use;
it does not automatically repair cut/ambiguous source or change core evidence.
ADR-0037 raw-extract permission is not general normalization/semantic access;
proposed INT-031 and other qualification gates remain separate.

G\SHA bytes are not silently changed or their input verification copied as
output proof. Assess actual final bytes/domain. An explicitly requested stamp
follows normalization as the existing final verified transaction; no implicit
restamp or flex substitution. The library acquires no files and promises no
rollback of a caller's partially written external sink.

This is the library's bounded normalization obligation, not a general 0.5
set/delete/renumber/apply-all API or the later CLI clean-up command. The
2026-11-27 roadmap places CLI clean-up and its actual transformation rules
in 0.2+ while retaining L1-WRT-002 in 0.1. Owner review must reconcile the
concrete profile/capability with that wording; this PR does not move releases,
claim the obligation complete or silently reduce full 0.1 scope.

## Planned checks and practical limits

Nineteen planned cases cover common policy/revision/refusal and retained-source
contracts, plus explicitly blocked order/newline/counter Option-A examples.
Whole-byte output and untouched-region identity are independent oracles; typed
value equality or multiset equality alone misses loss/reordering. No normalizer,
source-patch API, renderer, schema or new Rust type is implemented here.

Local work is source inspection and light trace/ID/parent, case-link, source-
fingerprint, inventory and whitespace checks. No Rust builds/tests, installs,
reference/sample runs, packaging or release actions run locally; heavy checks
use GitHub CI. All new requirements remain Draft with no Evidence.

#59 remains held until the owner's std answer, then must rebase/regenerate/
recount. Existing strict-scope, report, Criterion, flex, V/X, pattern,
session/order/access/governance/resource holds stay unchanged. The newly
exposed normalization choices are not decided inside an unheld drafting
batch. Family 3/parser implementation has not started.
