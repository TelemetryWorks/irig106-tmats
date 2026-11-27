# Edition-independent reading: family 2 batch 33

Date: 2026-11-27. Documentation-first derivation, no implementation.

## Why this slice

L1-EDN-001 requires reading every edition. The release inventory includes it
in 0.1, but it has no L2 child before this batch. Existing L2-EDN-001/002
cover declarations and labelled basis, not the reader's independence from
those declarations. This slice supplies the structural boundary without
claiming that every historical attribute has a definition in the baseline.

L2-EDN-003/004 and L3-EDN-008–013 derive the common reader and separation
from semantic meaning/record qualification. No requirement IDs reserved by
held #59 are used. No new interpretation or owner choice is introduced.

## Sources inspected

- ADR-0016: the owner chose edition-independent parsing and one registry,
  with edition deltas delivered later. Its old precedence/conflict policy
  is superseded by ADR-0028, not reinstated here.
- ADR-0028: two declarations and a labelled rule basis; compatibility checking
  is not validation against the document's actual edition. Its outstanding
  interpretation reviews are not silently cleared by structural reading.
- L1-EDN-001, the full RELEASE-0.1 inventory, and existing reading/lookup/
  qualification requirements supply the boundary and scope.
- 106-24R1 Ch9 sections 9.4.1/9.4.2 p. 9-2 specify physical text and
  code-name/colon/data-item/semicolon framing. Source inspected from the
  archived PDF, SHA-256
  `dfc5ebfb3e6f02f58389126863d43a45d89c7e2d0533a1c690d9537531c740cb`.
  [Official PDF](https://www.trmc.osd.mil/wiki/download/attachments/335389015/Chapter9.pdf?api=v2).
  This source does not by itself prove every older edition's definitions.
- Approved INT-035–039 define framing, character handling, canonical index
  identity and reviewed scalar multiplicity. Those contracts retain their
  development-analysis, source-freshness and executable-test obligations.

## Acceptance boundary

Vary supplied metadata over the same bytes; source, raw spans, item order and
scanner findings remain equal. Changing the G\106 bytes instead changes
physical offsets, so that test uses independently listed offsets for each
input. Semantic findings may change with the chosen definitions: they are
not scanner invariants. Unknown code retention is not approval of the code.

Raw lookup/replay remains available even without a usable definition. Do not
invent a historical rename, cardinality, range or default from the baseline.
Known multiplicity uses reviewed metadata; unknown multiplicity stays unknown.
Separate supplied document/revision identities and record qualifications
remain intact; successful reading is not setup eligibility or governing use.
XML support remains outside 0.1, with the existing format adapter refusing it.
Supplied resource limits keep their typed failures, not empty success.

The cases are synthetic structural fixtures. COMMENT uses its existing
standalone comment handling; UNREGISTERED_TEST is deliberately not a claim
about any Chapter 9 vocabulary. No historical code meanings are added.
Future historical conformance tests require their edition/table/row citations
and review. Independent byte/span oracles must not use the production reader.

## Release follow-ups carried from #65

The owner's requested three optional review nits are folded into current
RELEASING.md and existing L3-REL-012/013, with planned checks:

1. The org standard governs where the summary differs.
2. Remote API verifies the annotated tag, peeled to github.sha, on main with
   successful intended CI for that exact commit.
3. Install an exact pinned cargo-cyclonedx with --locked; record build's Rust
   toolchain and reuse/check it in publish and publish-path-check.

These restate the inspected org TRUSTED-PUBLISHING.md gate/build contracts,
[revision 2adedc5](https://github.com/TelemetryWorks/.github/blob/2adedc5aaf27b7bafb54ad05a51bc17d89b65d3e/docs/TRUSTED-PUBLISHING.md).
They are not tool installs, workflow changes or publishing operations.
Earlier research records remain history, including batch 32's original note.

## Verification and owner gates

Planned internal-state assertions choose no findings rendering, assumption
labels, suppressed-marker formatting or public report schema. Those stay
owner-gated. No new question blocks the requirements in this slice; std,
historical interpretation reviews, INT-061/062/063, pattern scope/syntax,
extraction, allocation and record-access/governance holds remain unchanged.

Local verification is source inspection, trace regeneration/check and
lightweight identifier/span/link/whitespace checks. No Rust builds/tests,
registry packaging, dependency installation or artifact verifier runs here.
GitHub CI supplies heavy checks; its prototype passes cannot establish new
reader acceptance. Reviewed requirements precede meaningful executable tests
and source-store/scanner implementation. Family 3 remains the ADR-0035
crates/ + apps/ restructure with prototype retirement, not started here.
