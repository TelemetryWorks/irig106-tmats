# 0.1 release scope and time-view contract

## Status and authority

Owner decision, 2026-11-21: deliver the full documented 0.1 scope, plus the
bounded time-view subset below, before `irig106-time` 0.8.0. The irigtime
consumer is CLI-only. This does not authorize releasing the prototype or
reducing the existing library, CLI, testing, or distribution obligations.

The release order and bounded scope are accepted owner direction. The API
contract and acceptance cases in this document are for independent review;
concrete Rust signatures, L2/L3 derivations, and interpretation reviews follow.
No implementation or passing-test claim follows from this document. The
[trace matrix](TRACE-MATRIX.md) remains the verification-status authority.

This document refines release scheduling in the [roadmap](ROADMAP.md) and
the consumer boundary in [TMATS-IN-CHAPTER-10.md](TMATS-IN-CHAPTER-10.md).
Existing [L1 requirements](L1-REQ.md) retain their statements and identifiers.
A partial time-view delivery does not mark an entire broader L1 implemented.

## Full 0.1 acceptance scope

The unchanged release obligations are grouped here for review. L2/L3 and
acceptance tests must precede implementation in each area; this inventory is
not a substitute for requirements or a new source of requirement IDs.

| Area | Existing L1 obligations |
| --- | --- |
| Ordered original bytes, all groups, complete indices, recovery and diagnostics, case folding, colon handling, suspect-semicolon warning | L1-READ-001 through L1-READ-007 |
| Complete setup records assembled from fragments, provenance and completeness outcomes, CSDW format/version/change fields and session checks | L1-CH10-001, 002, 004, 005, 006 |
| Edition-independent reading, separate raw declarations, labelled edition basis | L1-EDN-001, 002, 005 |
| Raw code-name/pattern lookup and locations | L1-VIEW-001 |
| Interpretation register and vendor/extension handling | L1-REG-006; L1-EXT-003 |
| Byte-faithful writing; normalization only by explicit request | L1-WRT-001, 002 |
| SHA compute, verify, final-byte verified stamping, labelled flex compatibility signature | L1-SUM-001 through L1-SUM-004 |
| No library I/O; bounded handling of arbitrary input, guard rails and coverage | L1-IO-001; L1-ROB-001, 002, 003 |
| Recording/text input, every complete record, show modes, extract, checksums, text/JSON, safe output, verified packet slicing | L1-CLI-001 through L1-CLI-009 |
| Measured parser and recording-scan performance | L1-PERF-001 |
| Lockstep versions, MSRV/platform/WASM checks, dependency policy, binary SBOM and provenance | L1-REL-001 through L1-REL-006 |

The `tmats` command scope remains `show` (raw, tree, channel summary),
`extract`, `checksum` (SHA and flex), `verify`, and `stamp`, with every
complete setup record addressable, plain text and versioned JSON output.
[CLI.md](CLI.md) governs command behavior. Help/version alone is insufficient.

Full registry generation and validation, general resolved format/measurement
views, time-word conversion views, configuration timelines, general edits,
and generation remain in their existing later releases. XML is reported as
unsupported; XML support and WebAssembly bindings are not added by this scope.
The plain library's WASM build remains a 0.1 obligation.

## Bounded time-view subset brought forward

The owner brings these parts forward from 0.2 for irigtime:

- L1-VIEW-007: enumerate TIMEIN channels with time data type format, time
  format and source category; expose secondary-header time format for each
  channel. PCM/network/1553 measurement time-word views remain in 0.2.
- L1-VIEW-004: effective-value states and their evidence for these fields.
  The general effective-value machinery remains in 0.2.
- L1-VIEW-005: split channel IDs where the recorder explicitly declares
  a valid number of multiplexer source bits; retain that declaration's source.
- L1-VIEW-006: unresolved and ambiguous results for time-view attribute
  lookups; no arbitrary winner among duplicates. Candidate clock associations
  are a separate L1-VIEW-007/008 contract with distinct states below.
- L1-VIEW-008: carry caller-assumption provenance through time-view answers.
  Other uses of labelled assumptions remain in 0.2.

This is metadata extraction, not a time decoder or a general validation
engine. Time conversion, packet timestamp interpretation, clock quality,
correlation and selection policy remain the time consumer's responsibilities.

## Standards evidence for the subset

The baseline here is IRIG 106-24R1, Chapter 9, January 2025, section 9.5.4,
Table 9-4. Each row below names its exact source row and printed page.
Other editions require their own verified definitions; this contract does not
silently apply a baseline default to every historical edition.

| Code | Table 9-4 row | Page | Use in the view |
| --- | --- | --- | --- |
| `R-x\TK1-n` | TRACK NUMBER/CHANNEL ID | 9-48 | Actual channel ID; distinguish it from occurrence index `n` |
| `R-x\DSI-n` | DATA SOURCE ID | 9-49 | Preserve source identity as context, not a clock reference |
| `R-x\CDT-n` | CHANNEL DATA TYPE | 9-50 | Identify TIMEIN channels |
| `R-x\SHTF-n` | SECONDARY HEADER TIME FORMAT | 9-51 | Per-channel secondary-header format |
| `R-x\TTF-n` | TIME DATA TYPE FORMAT | 9-71 through 9-72 | Time packet format, distinct from TFMT |
| `R-x\TFMT-n` | TIME FORMAT | 9-72 | Time representation; baseline default A where applicable |
| `R-x\TSRC-n` | TIME SOURCE | 9-72 | Source category, not a time-channel ID |
| `R-x\NSB` | NUMBER OF SOURCE BITS | 9-33 | Explicit multiplexer source-bit declaration |

Sources: [official Chapter 9](https://www.trmc.osd.mil/wiki/download/attachments/335389015/Chapter9.pdf?api=v2),
[archived Chapter 9](https://github.com/TelemetryWorks/rcc-106-standards/releases/download/106-24R1/Chapter9.pdf).
The archive manifest records SHA-256
`dfc5ebfb3e6f02f58389126863d43a45d89c7e2d0533a1c690d9537531c740cb`.

Code-name, keyword and link comparison follows Chapter 9 section 9.4.2:
ASCII case-insensitive interpretation while preserving the input spelling.

## Public API review boundary

The following are semantic contracts, not declarations of stable Rust
symbols. Family 2 derives the actual public interfaces and their tests.
The library's owned ordered source store remains the source of truth;
channel and time views refer back to that store and its document revision.

| Current prototype surface | Required release outcome |
| --- | --- |
| `parse::parse_with_options` | Preserve input and return located findings with recoverable data; tokenizer failure must not silently return an empty successful document |
| `TmatsDocument` | Reviewed lossless representation and ownership/lifetime contract; current public group fields are not frozen by this review |
| `RChannel.extra` | Do not make it the consumer contract: the prototype never populates it and routes these time attributes to `RGroup.extra` |
| `ch10::decode_setup_payload` | Reviewed boundary for complete records, correct format handling and separate declarations; no implicit parsing of an arbitrary fragment |

A consumer must not need to reconstruct channel semantics by searching
prototype `extra` vectors. Preserve raw lookup for all attributes alongside
an explicit time view with located values, findings and provenance.
A malformed or incomplete record cannot produce an unqualified usable view.

The view must identify the recorder occurrence, entry index, actual channel
ID and governing complete setup record. Duplicate IDs or attributes must
remain observable. A source location includes the document byte span and,
for assembled records, the contributing packet/fragment provenance.

Parsed results must have a documented ownership/lifetime contract. Returned
views cannot outlive their source or silently survive a document revision;
no consumer may depend on dangling input buffers or stale item identities.

The release must preserve the exact consumer dependency configuration:

```toml
irig106-tmats = { version = "0.1", default-features = false, features = ["std"] }
```

This is a dependency of the irigtime CLI only. The time library must not gain
a TMATS dependency through this contract. Once the time view exists, add a
GitHub CI consumer build using this exact declaration and exercising the
public time-view API. A workspace all-features test alone is not that check.
No CI or manifest change is made in this documentation family.

## Effective values and default evidence

Each time-view field reports one of these semantic states:

| State | Required evidence |
| --- | --- |
| Explicit | Valid interpreted value, original raw value and location |
| Defaulted | Interpreted value, exact edition/table/row citation, applicability and edition-selection basis; no fabricated input location |
| Missing | Attribute absent and no verified applicable default available |
| Invalid | Raw value/location, reason interpretation cannot be used, and applied edition/basis including any fallback label |
| Ambiguous | Every conflicting candidate and its location; no selected winner |

For example, an explicit `R-1\TFMT-3:A;` and an absent TFMT with an applicable
106-24R1 Table 9-4 TIME FORMAT default both yield A as the interpreted value,
but they have different states and evidence. Defaulting requires the TIMEIN
applicability condition and an identified edition basis. If either cannot be
established, report the missing/indeterminate reason instead of inventing A.

Consumers must read the Missing reason. In particular, "default applicability
not established" means the default condition is unknown, not proven false;
an Ambiguous CDT operand can cause that result even though TFMT is absent.
The reason and condition finding must remain visible when deciding how to
proceed; Missing alone is not proof that a default is inapplicable.

For a present TFMT, lowercase `a` means Explicit A while raw `a` stays intact
(section 9.4.2, page 9-2). A leading value blank (` A`) stays unavailable
until a blank-handling rule is reviewed; intentional blanks are neither
silently trimmed nor automatically Invalid. A present locally valid TFMT on
a non-TIMEIN channel stays Explicit with an applicability finding. Any Invalid
verdict reached under a fallback edition keeps its fallback label and basis.
These are planned-view contracts; their acceptance cases live in TEST-DATA.md.

The full source for the future value-blank review is 106-24R1 Chapter 9 section 9.4.2, page 9-2:

> Leading, trailing, and embedded blanks are assumed to be intentional; they can be ignored in most cases but should not be used in code names, keywords, and data items used as links, such as measurement name.

No general blank-handling policy is approved by quoting this sentence;
the unavailable-value contract above remains in force until that review.

The TFMT default does not supply a time source, prove an association, or
change any bytes. Do not default to IRIG-B, UTC, a channel number, or a
multiplexer source-bit count. Invalid or conflicting explicit attributes do
not become valid by replacing them with defaults. Do not overwrite or insert
attributes while building a view (L1-VIEW-004; ADR-0002 and ADR-0023).

A labelled fallback edition under L1-EDN-002 remains a fallback, not a
claimed declaration by the recorder. That basis must accompany any default
whose interpretation relies on it. RCCVER and `G\106` remain separate;
`G\106` is the TMATS version declaration (106-24R1 Chapter 9 section 9.5.2,
Table 9-2, RCC IRIG 106 REVISION LEVEL row, page 9-10).

## Data-channel to time-channel association uncertainty

Owner direction: there is no cited TMATS field establishing this association.
The baseline rows above expose identity, formats, source category and
multiplexer grouping. They do not establish a clock-selection reference.
In particular, TSRC is not such a reference and TFMT is not a source ID.

This association is not a standard-defined TMATS link: a possible candidate
is not proof that a link resolved. Given a data channel and its governing
setup record, the contract returns an association state, all relevant
candidates, provenance and a reason:

| State | Meaning |
| --- | --- |
| `association_unestablished` | No adequate evidence identifies a time channel; may include one or more candidates that are only possible matches |
| `association_conflicting` | Multiple conflicting or equally supported associations remain; return all candidates and their evidence |
| `association_evidenced` | One association is actually established by evidence valid for this record; return that evidence, not just a selected ID |

These semantic state names are distinct from L1-VIEW-006 link/lookup states
and do not freeze Rust symbols. Parent this association contract to
L1-VIEW-007 and L1-VIEW-008 only.

A single TIMEIN channel is still `association_unestablished` without evidence.
The first, lowest-numbered, nearest, enabled, or only time channel must never
be selected by default. Equal DSI values, recorder occurrence or multiplexer
source grouping alone are not proof of association. A defaulted TFMT does
not change an unestablished association into an evidenced one.

Evidence for a future `association_evidenced` case must identify its origin, applicability
to the recording/setup record and channel identities, and how it establishes
the relationship. A cited extension with reviewed semantics may supply such
evidence; this family defines no extension, inference rule or resolver API.
Unknown vendor/extension attributes are preserved, never treated as proof
merely because their names appear to suggest a clock relationship.

An explicit caller choice without supporting evidence is an assumption,
not a verified TMATS association. A consumer may use such an override only
with the choice and assumption basis attached to every derived answer
(L1-VIEW-008). The underlying association remains
`association_unestablished` or `association_conflicting`; the override must
not relabel it as `association_evidenced`.

Do not carry associations from another recording or setup revision without
new evidence. Full configuration-timeline construction stays in 0.2; the
0.1 caller must identify the complete setup record used for each query.

## Setup-record boundary

Use 106-24R1 Chapter 11 section 11.2.7.2 and Figure 11-34 for format/SRCC/
RCCVER handling, and section 11.2.1.1 b for multiplexer channel-ID layout.
[Official Chapter 11](https://www.trmc.osd.mil/wiki/download/attachments/335389015/chapter11.pdf?api=v2).
These are Chapter 11 facts, not invented Chapter 9 defaults.

Only complete assembled records are parsed into usable views. Preserve the
assembly's complete/incomplete/ambiguous outcome and evidence. Reject XML
as unsupported for this release. Do not use RCCVER to overwrite the TMATS
edition declaration, or confuse packet data-type version with either one.
The packet reader and time decoder remain outside the TMATS library.

INT-011's per-channel joining is reconfirmed by Joey (decision 2026-11-23),
with independent AI CONCURS on #38 head `c623b15`. Review completion does not
complete analysis or tests. See the [reconfirmation record](research/2026-11-23-int011-owner-signoff.md).
INT-034's "followed by another packet" means SAME-CHANNEL; an otherwise
complete run ending in `;` at EOF stays complete, labelled "end of input".
An open run may expose PROVISIONAL boundary evidence, never a joining stop.
Pre-EOF semantic use by irig106-time's CLI, snapshot invalidation and its
interaction with INT-028/031 remain explicit owner-review items. Recommended
option: opt-in labelled snapshots; alternative: final-only semantic use. No
option is selected here or silently relaxes the complete-only contract above.
Per-channel open-run state needs an explicit finite aggregate memory budget
and typed limit error, never silent dropping; numeric policy, full accounting
and recovery/resume details need review before implementation. See the
[batch 15 review draft](research/2026-11-23-completeness-contracts-draft.md).

## Acceptance cases to derive before code

These are test obligations for the next requirements family, not tests already
implemented. Give each derived L2/L3 obligation a parent and mark the tests;
regenerate the trace matrix without manually upgrading status.

| Case | Required result | Parent L1 requirements |
| --- | --- | --- |
| Entry index 3 has channel ID 42 | Associate fields by entry index, return actual ID 42 | L1-VIEW-007; L1-READ-002 |
| Lower-case codes/keywords and reordered fields | Same interpretation, unchanged bytes and locations | L1-READ-001, 004 |
| Explicit A versus applicable missing-TFMT default A | Different states and evidence, identical input bytes on emission | L1-VIEW-004; L1-WRT-001 |
| Invalid, duplicate/conflicting, or unsupported-edition format | Invalid/ambiguous or missing reason; no guessed default | L1-VIEW-004, 006 |
| One TIMEIN, no association evidence | `association_unestablished`, even with a valid/defaulted TFMT | L1-VIEW-007, 008 |
| Several possible time sources | `association_unestablished` or `association_conflicting` with candidates; never first-match | L1-VIEW-007, 008 |
| Caller selects a time channel without proof | Assumed answer; underlying association not upgraded | L1-VIEW-007, 008 |
| Evidence from another record/revision | No `association_evidenced` result for the current record | L1-VIEW-007, 008 |
| TIMEIN, TTF=2, TFMT absent at the baseline | TFMT is defaulted A with its edition/citation plus a finding about the network-time inconsistency; never override packet evidence | L1-VIEW-004, 007 |
| TK1=0; NSB=0; NSB outside 0-13 | TK1 invalid (range 1-65535); explicit NSB=0 is valid (source ID 0, full channel ID retained) and proves no association; invalid NSB is reported without splitting/defaulting | L1-VIEW-004, 005, 007 |
| Explicit valid NSB versus absent/invalid NSB | Split only with valid declaration; otherwise retain uncertainty | L1-VIEW-005 |
| Multi-packet, truncated, XML or ambiguous setup record | Preserve assembly outcome; no false usable ASCII view | L1-CH10-001, 002, 004 |
| Malformed attribute among valid time metadata | Preserve input with diagnostics rather than empty success | L1-READ-001, 003 |
| Exact CLI consumer dependency feature set | Public time-view use builds in hosted CI | L1-VIEW-007; L1-REL-002, 004 |

All eventual association-proof fixtures must carry the actual source evidence
and its review; a fixture invented merely to make resolution succeed is not
proof. Standards-based positive and negative cases must be independent of
the implementation, including the known prototype regressions.

## Decisions recorded 2026-11-21

The owner may override every coordinator decision. Policy authority and
interpretation review completion are separate: coordinator direction below
does not by itself sign off an executable interpretation.

| Decision | Authority | Recorded outcome |
| --- | --- | --- |
| Interpretation reviews | Owner-decided, 2026-11-21; ADR-0033 | Agents draft with citations. One human owner sign-off plus the independent AI reviewer as a second, non-human check approves an interpretation |
| W1: workspace | Owner-decided, 2026-11-21; ADR-0035 supersedes ADR-0034 W1 | Virtual root; library `crates/irig106-tmats/`, CLI `apps/irig106-tmats-cli/`; shared lockstep workspace version and exact-version path dependency in workspace.dependencies |
| W2: prototype retirement | Coordinator-decided, 2026-11-21; owner may override | Remove the prototype in the restructure change; `prototype-0` retains history. Split an oversized PR or return to the owner |
| W3: host CLI integration | Coordinator-decided, 2026-11-21; owner may override | Both a host-callable `run(args, stdout, stderr) -> exit class` that never exits the process and individual command/render APIs; thin `main`, matching irig106-time |
| F1: duplicate SHA items | Owner-approved 2026-11-23 under ADR-0033 (INT-021); AI CONCUR at #30 head 13173b0; originally coordinator-decided 2026-11-21 | Reject as indeterminate, list every occurrence, preserve every byte, refuse stamping until fixed, and never claim positive verification |
| F7: configuration-change event | Owner-approved 2026-11-23 under ADR-0033 (INT-030); AI CONCUR at #30 head 13173b0; originally coordinator-decided 2026-11-21 | Require verified identification and report indeterminate meanwhile; never guess a packet type |

[ADR-0033](adr/0033-owner-signoff-and-independent-ai-interpretation-review.md)
supersedes the two-human rule. Record the draft author, owner sign-off and
independent AI check; preserve source-hash re-review, citations, analysis and
test obligations. Earlier accepted design labels do not establish that an
interpretation's executable rule has completed these checks.

[ADR-0034](adr/0034-coordinator-release-foundation-decisions.md) records the
coordinator decisions. Its W1 alone is superseded by
[ADR-0035](adr/0035-virtual-workspace-with-library-crate-and-cli-app.md), the
owner's virtual crates/apps layout. W2 prototype removal and W3 reusable CLI
library plus thin binary are unchanged; the full 0.1 scope remains binding.

The readiness gates apply to reader/validator behaviour tied to register
entries, including hand-written behaviour, not only generated registry rules.
Joey approved F1 (INT-021) and F7 (INT-030), 2026-11-23, under ADR-0033;
the independent AI check CONCURRED on both at PR #30 head `13173b0`.
Their semantic approval/review gates are complete; freshness, development
analysis and executable verification remain. F7's event identity is still
unverified, and prior-ordering/fragment-SRCC questions remain analysis work.
F1 remains in the full 0.1 checksum/stamping scope. See the
[owner approval record](research/2026-11-23-int021-int030-owner-signoff.md) and the historical
[review-gate note](research/2026-11-23-f1-f7-review-gates.md).

For pre-ADR-0033 entries INT-001 through INT-034, existing owner acceptance
with Review: pending is not completed review: the independent AI check is
still required before implementation. Preserve that owner acceptance; do not
grandfather the missing check or treat other open/coordinator entries as
owner-approved. Joey reconfirmed **INT-018** (projected G\SHA name, never
inside a value), **INT-019** (unterminated SHA is malformed) and **INT-020**
(upper-case digest digits), 2026-11-23, under ADR-0033. Independent AI CONCUR
on all three is tied to the register review at main `30e42a3`. Together with
F1/INT-021, these clear their interpretation-review blockers only; analysis,
source freshness and tests remain. See the [combined approval record](research/2026-11-23-int021-int030-owner-signoff.md).
INT-043 Option B is owner-decided (Joey, 2026-11-23): ASCII-digit designators
whose canonical digit string is 2 are algorithm 2, with a warning for every
noncanonical spelling and canonical 2- stamping. See the
[owner decision](research/2026-11-23-int043-owner-decision.md) and
[approval record](research/2026-11-23-int043-owner-signoff.md). The independent
AI recheck CONCURS on #32 head 8941ca1; INT-043 is approved under ADR-0033.
The per-occurrence noncanonical warning still appears when another reason
blocks verification; it supplies no authority to guess a key/exclusion.
Development analysis, freshness and executable tests remain outstanding.

INT-044 Option B is approved (Joey, owner decision, 2026-11-23; independent
AI CONCURS on #36 head `6f6e080`). Duplicate G\106 stays Ambiguous while
a separate, prominently labelled fallback uses unchanged INT-017 policy;
mixed recognised/unrecognised occurrences still use the ambiguity reason.
JSON carries machine code `ambiguous_g106` and numeric `candidates` separately
from its display label. ADR-0028 already allowed useful checks for missing
or unrecognised declarations; blocking duplicated good input would have
been less useful than handling those worse inputs. See the
[approval record](research/2026-11-23-int044-owner-signoff.md). Analysis,
source freshness and executable tests remain; INT-017's own review gate and
INT-034's assembly-scope decision are not cleared by this approval.

F1 deliberately leaves damaged recordings readable but without a positive
integrity verdict. Fixing duplicates requires an explicit action before
stamping; verification must not silently choose or consolidate occurrences.
A damaged name compatible with G\SHA also makes verification indeterminate.
ADR-0029 still permits an explicit repair removing a duplicate by its exact,
revision-bound item ID. Rebuild/re-read the edited input; stamp only after
that repair has resolved the ambiguity, potentially as the transaction's
last step. Refusal to stamp ambiguous input is not refusal to allow its repair.

F7 decides the uncertainty behavior, not the actual event identity. The latter
still needs verified evidence before the event-presence check is implementable.
These approvals do not mark requirements implemented or development evidence complete.
F2/F3 and other unreviewed interpretations remain open where already recorded.

## Delivery and release gates

Follow the twelve-family sequence: scope/API contracts; derived requirements;
workspace decisions/layout; source store/scanner/recovery; lookup/writer/
extensions; CSDW/editions; assembly/session checks; bounded time view;
checksums; CLI; robustness/evidence/prototype retirement; release pipeline.
Each family may need multiple PRs, normally 300-600 changed lines including
requirements, tests and documentation. Keep tests and requirements ahead of
implementation; keep the full 0.1 gates visible throughout.

Use GitHub CI for builds, substantive tests, coverage and bounded fuzz or
benchmark evidence. Local checks stay lightweight; stop background jobs.
No release is justified by source inspection or a green prototype suite alone.

All real releases use Trusted Publishing through `trusted-publishing.yml`
and the protected `crates-io` environment. The sole first-upload exception is
the coordinator/owner CLI name-placeholder bootstrap in RELEASING.md under
the existing 2026-11-15 policy; CI never performs it or stores its token.
[RELEASING.md](RELEASING.md) governs the library/CLI lockstep,
package order, binaries, checksums, SBOMs and provenance. Workflow creation,
publisher-registration verification and release execution are later work.

Every PR is handed to the independent reviewer with its exact commit head
before the owner merges. Changed heads require fresh review; no admin bypass,
force push or tool-attribution trailers.
