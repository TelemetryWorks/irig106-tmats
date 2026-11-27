# Complete-record enumeration: family 2 batch 34

Date: 2026-11-27. Requirements and planned fixtures, no implementation.

## Existing obligation and chosen scope

L1-CLI-003 requires every complete setup record in file order. The current
prototype's first-packet limitation is not an accepted implementation.
Existing routing, mounting and extraction children do not fully derive this
all-record obligation. This batch adds L2-CLI-010/011 and L3-CLI-038–044.
It consumes evaluated assembly outcomes rather than deciding their rules.

No new interpretation or owner answer is required to state these internal
contracts. There is no new packet reader, report schema, extraction mode,
command default, integrity policy or governing-setup resolver here.

## Source inspection and authority

- L1-CLI-003 and CLI.md's every-record contract require enumeration rather
  than just one packet. Defect D2 in TEST-DATA records the reference tool's
  first-packet-only limitation; the tool is not a standards authority.
- 106-24R1 Chapter 11 section 11.2.7.2 p. 11-50 says a setup record may
  span multiple consecutive packets, with sequence increment n+1.
  [Official PDF](https://www.trmc.osd.mil/wiki/download/attachments/335389015/chapter11.pdf?api=v2),
  inspected archive SHA-256:
  `0f63792dcac39f6010df371ee0afc46a396017918f4155cba9ebab6c2d6d263e`.
  This supports the packet/record distinction, not an observable terminator.
- ADR-0025 requires library assembly and provenance, with its reader/checksum
  ownership superseded by ADR-0036. Core supplies header-only PacketFacts,
  PacketInfo evidence alongside it and borrowed DATA spans; tmats never
  acquires a second reader or verifies core-owned checksums.
- INT-011 supplies independent per-channel runs; INT-039 preserves separate
  documents; INT-045/048 retain time-seal revisions, ambiguity and blocking
  cut evidence; INT-053 keeps previous-copy identity informational. None
  permits record deduplication to erase source occurrences.
- INT-034 and the access/governance decisions remain pending where marked.
  Tests may receive evaluated final states without claiming to test those
  pending classifications or bypassing their implementation gates.

## Ordering and identity

File order means each record's first contributing fragment position in the
supplied recording order. This is a derivation of the project contract,
not an additional Chapter 11 interpretation. Completion order differs when
independent channel runs overlap; ordering by timestamps or channel would
also lose source order. Preserve positions supplied by core, not guessed
physical lengths. The synthetic two-channel case receives already evaluated
runs and does not assert any edition permits arbitrary setup channels.

A fragmented record has one identity with all of its fragments. Two bounded
records with identical bytes remain two identities; checksum agreement is
not deduplication authority. UC-19's later timeline may share descriptions,
but that does not remove occurrences from 0.1's record inventory. Records
are not concatenated into one document for duplicate or checksum evaluation.

This slice does not choose a streaming publication strategy. A final
inventory uses supplied final outcomes; an earlier Complete time seal may
be revised to Ambiguous and its findings/history must survive. No pending
provisional-use rule is approved by exposing internal record computations.

## Qualification, failures and selection

Complete, Ambiguous, Incomplete, retained cut, provisional and unsupported
remain distinct. Complete is not a positive G\SHA result, valid metadata,
permission to extract, or a governing setup. A list of Complete entries must
not erase the existence/provenance/findings of non-Complete candidates.
No specific user-facing heading, label, JSON field or exit mapping is set.

A failure before the first result is not successful zero records. A failure
later leaves an incomplete inventory with extent/context; earlier observed
records cannot prove absence of later ones. Keep typed resource failures;
no silent run dropping, alternate-route retry or unbounded staging. Choosing
whether/how to publish partial results remains outside this requirement.

Consumers identify a scoped candidate/revision explicitly. Enumeration does
not select first/latest/matching-checksum records as a default, number a
public selector, or decide how multiple outputs should be framed. Those
choices and existing extraction/eligibility holds remain for owner review.

## Follow-up carried from #66

L3-EDN-012 now explicitly distinguishes edition-independent TEXT reading
from edition-dependent packet routing. The owner-requested examples are
channel-0-only setup from 106-11, core's legacy_setup_any_channel, 106-04
time CSDW EXT bits and Time F2. These remain core / L3-CH10 responsibilities.
This note adds no packet-level mapping or local routing predicate.

## Planned verification and remaining work

The planned internal fixtures list identities, source positions, fragment
membership and outcomes independently. Classification tests remain in the
assembler suite. A second reader or the production enumerator cannot serve
as the oracle. Three complete records, fragmented plus single records,
identical copies, out-of-order finalization, non-Complete states, revised
seals and early/late failures cover the main enumeration boundaries.

Light local checks cover trace regeneration/drift, unique IDs, fixture links,
source fingerprint and whitespace. No Rust builds/tests, packaging, installs,
recording access or publishing commands run locally; heavy checks use CI.
New requirements stay Draft with no Evidence. No rendered output formats
are introduced; when their derivation is reached, stop for owner direction.
Family 3's crates/ + apps/ restructure and parser work have not started.
