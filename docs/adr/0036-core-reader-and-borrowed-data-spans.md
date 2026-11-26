---
status: accepted architecture; coordinator-decided integration
date: 2026-11-26
decision-makers: Joey (2026-11-22 architecture); coordinator (integration decisions)
---

# One core reader; TMATS consumes borrowed DATA spans

The owner's 2026-11-22 architecture puts the ONE Chapter 10 reader in
`irig106-core`. The coordinator confirms core 0.1.0 is published and the
core 0.2 whole reader is merged. ADR-0019's replacement condition has been
met: the tmats CLI depends on core for packet finding, trust and primary/
secondary-header checksum evidence. It must not grow a second reader.
This supersedes the temporary-reader responsibility in ADR-0019 and the
packet-reading/checksum ownership in ADR-0025, not its TMATS assembly rules.

Coordinator decisions, 2026-11-26 (owner may override):

- tmats does NOT verify packet data checksums. It consumes core's integrity
  evidence as-is, including NotChecked until core provides verification.
  Data-checksum verification is a core feature request; the owner decides
  when core implements it. There is no tmats fallback implementation.
- TMATS accepts non-contiguous DATA spans directly, with zero extra payload
  copy and no required coalescing. The adapter consumes core PacketFacts and
  borrowed DATA spans; tmats keeps only logical CSDW/fragment slicing.
  Core's evidence, including RDG-008 byte/word secondary-checksum readings,
  passes through unchanged; tmats chooses neither algorithm nor verdict.

A four-byte CSDW and text may cross block boundaries. Per-channel assembly
retains ordered borrowed spans and provenance, not a mandatory joined byte
allocation. Input ownership/lifetimes keep blocks alive until borrows end;
metadata/scalar reads are distinct from copying/coalescing payload bytes.
Standalone owned-document construction remains a separate path; it must not
be a required intermediate to span-based assembly. Retained block owners/span metadata must be accounted for in the existing
resource-budget design; zero extra copy is not zero retained memory.
No new global span-count or record-size cap is chosen by the 256-span maximum-packet fixture.

Keep Chapter 11 facts as reference, not a second implementation. These are
architecture/integration decisions, not new TMATS interpretation approvals.
No report/rendered-output format is chosen. Concrete dependency version/API
and lifetime signatures must be verified before implementation; no core
source/API inspection or cross-repository issue filing is implied here.
