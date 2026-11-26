# Family 2 batch 20: association uncertainty and caller assumptions

Date: 2026-11-26. Follows merged #51 (reviewed head 91edc62, merge f5ab351).
Requirements and PLANNED tests only, not a new time-view implementation,
public API or completed intensive development analysis.

## Authority and scope

The owner's 2026-11-21 full 0.1 plus bounded time-view direction, recorded
in RELEASE-0.1, requires association uncertainty and labelled caller choices.
It explicitly says no cited TMATS field establishes data channel -> time
channel. Candidate associations derive L1-VIEW-007/008 only, not the general
L1-VIEW-006 link resolver. L1-VIEW-007 measurement time words and other
L1-VIEW-008 assumption uses remain 0.2. The work is the next remaining
bounded derivation in ROADMAP after metadata, NSB, DSI and CHE/numeric syntax.

L2-VIEW-013 derives scoped association uncertainty under L1-VIEW-007;
L2-VIEW-014 derives caller-assumption provenance under L1-VIEW-008.
L3-VIEW-043–047 specify scope, state separation, forbidden inference,
future-proof admission and retained candidate qualifications.
L3-VIEW-048–051 specify explicit choice/basis, propagation, no authority
upgrade and scope/byte preservation. They leave concrete Rust types for
later API review. No requirement is satisfied by merely naming a planned test.

## Source inspection

Re-read the official 106-24R1 Chapter 9 baseline PDF previously downloaded
from <https://www.trmc.osd.mil/wiki/download/attachments/335389015/Chapter9.pdf?api=v2>.
SHA-256 verified locally on 2026-11-26:
`dfc5ebfb3e6f02f58389126863d43a45d89c7e2d0533a1c690d9537531c740cb`.
Inspected section 9.5.4 Table 9-4 rows:

| Code | Row / printed page | Role retained by this slice |
| --- | --- | --- |
| R-x\NSB | NUMBER OF SOURCE BITS, 9-33 | Explicit multiplexer partition declaration, not a clock reference |
| R-x\TK1-n | TRACK NUMBER/CHANNEL ID, 9-48 | Actual identity distinct from entry index |
| R-x\DSI-n | DATA SOURCE ID, 9-49 | Source context with its own links, not a time-channel reference |
| R-x\CHE-n | CHANNEL ENABLE, 9-49 | Enable context, not proof or a silent enumeration filter |
| R-x\CDT-n | CHANNEL DATA TYPE, 9-50 | TIMEIN classification in its own entry |
| R-x\SHTF-n | SECONDARY HEADER TIME FORMAT, 9-51 | Per-entry format declaration |
| R-x\TTF-n | TIME DATA TYPE FORMAT, 9-71–72 | Time data-type format, not a target-channel key |
| R-x\TFMT-n | TIME FORMAT, 9-72 | Explicit/defaulted format evidence, not an association |
| R-x\TSRC-n | TIME SOURCE, 9-72 | Source category, not the ID of another entry |

The inspected local metadata rows do not supply the reference the consumer
needs. The no-inference contract is the OWNER's library policy; this is not
an exhaustive proof that no other standard/extension can ever express one.
A future association rule needs its own actual source, reviewed semantics,
applicability and test oracles. Unknown vendor/X fields remain losslessly
inspectable; a suggestive clock-like name is not an executable proof rule.
No other repository, consumer source tree or local recording was accessed.

## No default clock and no fabricated positive proof

Zero/one/many possible TIMEIN entries without adequate association evidence
remain association_unestablished. Preserve every relevant candidate and its
scope; never infer from order, lowest/nearest ID, CHE enabled/disabled,
matching DSI, shared NSB component, TSRC or defaulted/explicit TFMT.
Conflicting is the distinct state for supported competing associations;
a candidate-count heuristic does not establish that classification.
This slice does not invent evidence weights, a conflict threshold, provider
API, extension code name, or standards-defined positive-association fixture.

Evidence admission remains conditional on a future reviewed rule. Tests here
exercise rejection/unavailability and composition. The conflict composition
unit supplies a preclassified conflicting result and asserts it survives a
caller choice; it tests no actual proof parser or classification algorithm.
All eventual positive-proof fixtures must bring actual source evidence as
RELEASE-0.1 requires. Format decoding and time correlation stay with consumers.

## Caller choices and setup eligibility

An explicit choice without proof is labelled assumed, with caller basis,
source/target scope and setup/revision on every answer depending on it.
The underlying association remains unestablished/conflicting. A choice
cannot make a duplicate/invalid/unknown target usable or choose a governing
setup, erase retained-cut blocking or override a packet's own evidence.
Source scope is not guessed from a shared numeric ID. A new record/revision
requires newly applicable evidence or a freshly scoped labelled choice.
Earlier answers retain their historical provenance rather than being relabelled.

This composes approved INT-039/041/048/054/055 and batch-16 qualification;
it does NOT approve proposed INT-028/031/032/033. In particular no earlier
permission is inferred for arbitrary fragment-order assumptions, unknown
packet descriptions, ambiguous split acceptance or pre-EOF semantic use.
Those decisions do not block deriving this narrower already-owner-directed
contract. Full governing timelines stay 0.2; the caller supplies context.

## Planned checks and implementation gates

Named PLANNED cases cover missing/one/many candidates, scoped identities,
metadata-only non-proof, unknown extensions, stale evidence, caller selection,
independent/conflicting results, target ambiguity, retained-cut state,
provenance and raw/revision preservation. Future generated tests should vary
candidate order/IDs/DSI/enable/format fields independently and verify no
association-evidenced result arises from those mutations alone.

Local work checks requirement IDs/parents, case links and source/contract
boundaries, regenerates the trace matrix, and runs its --check and whitespace
checks. No Rust/parser/runtime builds/tests or heavy local enumeration run.
GitHub CI supplies the heavy checks; exact-head run IDs are in the PR.
Existing prototype CI is not evidence that these new requirements are verified.
Source freshness, full development analysis and meaningful executable
verification remain required before code; no new INT approval is requested.
