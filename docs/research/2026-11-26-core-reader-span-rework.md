# PR #54 correction: one core reader and borrowed DATA spans

Date: 2026-11-26. Supersedes the reader/checksum ownership proposed at #54
head 7b892ce; the [earlier source note](2026-11-26-packet-slicing-requirements.md)
remains historical reference, including its Chapter 11 facts/example values.
ADR-0036 records the architecture/integration decisions. No code or core API
inspection is claimed; the coordinator supplied core's availability/status.

## Authority and decisions

Owner architecture, 2026-11-22, as relayed in review: ONE packet reader, in
core; no new reader elsewhere. ADR-0019 already said the temporary CLI reader
would be replaced when core became available. The coordinator reports core
0.1.0 published and core 0.2's whole reader merged. Thus the CLI depends on
core for packet finding, trust and primary/secondary-header checksums.

Coordinator decisions following that architecture/performance direction
(owner may override), 2026-11-26:

> Decision 2: tmats does NOT verify data checksums. It consumes core's integrity evidence as-is (NotChecked until core provides it). Record data-checksum verification as a core feature request, not a second implementation; the owner decides when core builds it.
>
> Decision 3: the TMATS assembler accepts NON-CONTIGUOUS DATA spans directly (no coalescing required; zero extra copy). Add fixtures where a CSDW, and separately a text fragment, straddles a block boundary, plus a 256-span max-setup case.

Re-point L2-CLI-003 and L3-CLI-004–007 to consumption of core facts rather
than local reading/checks. L1-CLI-008 now names that dependency boundary.
L2-CLI-004/L3-CLI-008–013 define only DATA prefix/text spans, logical assembly,
provenance, lifetime retention and unchanged evidence. No requirement ID is
reused for an unrelated purpose. No new secondary-checksum interpretation is
chosen here: core owns RDG-008 and its byte/word readings/verdicts.

The CLI adapter consumes PacketFacts and borrowed DATA spans from core;
the TMATS assembler accepts those non-contiguous spans directly with the
required provenance/evidence. This does not move packet finding/checksums
into the TMATS library, invent a second PacketFacts type, or make core depend
on TMATS. Exact compatible core version/API and lifetime binding must be
verified before implementation. In particular, published 0.1.0 is not proof
that a merged 0.2 API is already in that published version. No dependency
version or release schedule is silently chosen here.

## Core feature request: packet data-checksum verification

Requested capability belongs to core: provide data-integrity evidence for
optional 8/16/32-bit packet checksums, with the Ch11 body-plus-filler domain.
Owner decides when core implements it; until then pass NotChecked exactly.
No local fallback, retry checksum, inferred absence/pass/failure or use of
G\SHA to fill that gap. Preserve whatever evidence core provides later too.
This is a LOCAL cross-repository dependency request for coordinator/owner
routing; no issue was filed and no other repository was accessed/changed.
It is not a blocker requiring tmats to build a substitute reader/checksum.

## Logical spans and lifetime/performance contract

Core DATA already excludes headers, filler and checksum. Let D be its logical
length. For Format 1, read CSDW [0,4) across any necessary spans; text is
[4,D). Remove the prefix ONCE per packet, not once per span/block. D<4 cannot
fabricate bytes, while D=4 is an empty text region without a completeness
claim. Checked span/range arithmetic remains local; raw packet/header/profile
validation does not. Do not reconstruct DATA by adding a guessed 24/36.

CSDW/text slicing and per-channel assembly retain borrowed span references
and core source origins; there is no mandatory contiguous intermediate or
payload-byte copy. Fixed-width scalar reads/span descriptors/provenance
metadata are not payload coalescing. Keep the input owners alive for all
borrows and prevent block recycling while references are used. Logical source
positions map through every block/span and packet-fragment boundary.

Owned standalone document construction is a separate mode: an explicit
materialization may be chosen by its caller, but cannot be required merely
to slice or assemble this span-backed input. This slice does not freeze a
public Rust lifetime/signature or claim the current prototype supports it.

## Kept Chapter 11 facts and reference values

Official [106-24R1 Chapter 11](https://www.trmc.osd.mil/wiki/download/attachments/335389015/chapter11.pdf?api=v2),
SHA-256 `0f63792dcac39f6010df371ee0afc46a396017918f4155cba9ebab6c2d6d263e`:
sections 11.1/11.2.1–11.2.1.4 pp. 11-1–12 and 11.2.7.2/Figure 11-34
p. 11-50 remain the source. Header sizes 24/12, sync 0xEB25, four-byte CSDW,
Data Length excluding trailer, body-plus-filler checksum domain and source
limits 134,217,728/524,288 remain facts; core implements the reader checks.

Keep the original numeric examples as reference, NOT tmats checksum tests:
secondary bytes 01 02 03 04 05 06 07 08 00 00 yield byte/word readings
0x0024/0x1410. TMATS forwards BOTH RDG-008 readings and core's outcome
unchanged; neither local source-text preference nor a new calculation may
select or strengthen a reading. The original 8-bit body/filler examples
0x26/0x1E and the H/D/P/C intervals remain references for the core feature
request; no TMATS code/tests duplicate their verification algorithm.

## Planned borrowed-span fixtures

- Split the four CSDW bytes 0E 01 00 00 at each internal boundary; expected
  logical scalar/raw fields are unchanged, with text skipped only once.
- Separately split text (including within a code/value/terminator fixture)
  across blocks; return the same logical bytes/provenance without requiring
  contiguous text or treating a block edge as a record/packet edge.
- Maximum baseline Format-1 packet: P=134,217,728, H=24, no data checksum.
  Core DATA D=134,217,704 and text length=134,217,700. Use 256 borrowed
  spans: first 524,264 DATA bytes, then 255 spans of 524,288. The input owner
  supplies backing in bounded GitHub CI; local checks are descriptor math
  only. Assert original span owners/offsets, no extra payload copy/coalescing,
  exact logical text length and unchanged NotChecked evidence. This is one
  maximum-PACKET fixture, not a new max record size/span count (257 is not
  implicitly rejected).

Test core integration with supplied/core-certified facts, not a duplicate
reader/checksum oracle. Instrument payload-copy/coalescing to remain zero;
metadata allocations do not hide a joined payload. Keep RDG-008 and data
NotChecked unchanged through each path. Actual API compatibility, borrow
safety and performance need meaningful executable verification later.

No rendered report/findings/assumption/suppressed-marker format is selected;
the owner gate remains. Local audits check requirement/ADR links, span and
maximum-case arithmetic, source hash, stale reader responsibility, trace
regeneration/--check and whitespace. No Rust build, large allocation or
runtime reader test runs locally. Heavy work uses GitHub CI; source freshness,
full analysis and executable checks remain implementation gates.
