---
status: accepted
date: 2026-09-26
decision-makers: Joey
---

# Setup records are assembled from packet fragments in the library; packets are sliced by the CLI's reader

## Context and Problem Statement

"A single setup record may span multiple consecutive packets. When spanning
multiple packets, the sequence counter shall increment in the order of
segmentation of the setup record, n+1" (Chapter 11 §11.2.7.2, 106-24R1). The
packet body of each Format 1 packet begins with the CSDW, so every fragment
carries its own CSDW. The design first said each setup packet was read
independently (UC-02), which would split a valid record, break attributes
that cross a packet boundary, and compute `G\SHA` over the wrong bytes. The
team design review (priority T3) asked for an explicit assembly contract and
precise payload slicing (Chapter 11 §11.2.1.1–11.2.1.4: header, optional
secondary header, Data Length excluding filler and data checksum).

The standard defines no end-of-record marker, so the boundary between one
multi-packet record and the next needs an interpretation.

## Decision Drivers

* Every valid recording can be extracted, validated, and checksummed
* Diagnostics can name the packet a problem came from
* The library stays free of I/O (ADR-0010); consumers reuse one assembler

## Considered Options

* **Assembler in the library (fragments with provenance in, complete records
  out); slicing in the CLI's reader until `irig106-core`**
* Assembler in the CLI first, moving to `irig106-core` (the team's suggestion)
* Read each packet independently (the original UC-02)

## Decision Outcome

Chosen option: **assembler in the library; slicing in the CLI's reader**
(owner decision, 2026-09-26, on the recommendation that followed the team's).

- **Slicing** (the CLI's reader, later `irig106-core`): the 24-byte header
  (sync `0xEB25`); a 12-byte secondary header when packet-flags bit 7 is 1;
  the body from there; the TMATS fragment is the body from offset 4 to Data
  Length; filler and the data checksum are never included. The header and
  secondary-header checksums are verified before any length field is
  trusted; the data checksum is verified and reported when present.
- **Assembly contract** (the library): fragments in file order, each with its
  bytes and provenance (file offset, channel ID, sequence number, relative
  time counter, CSDW fields, data-type version); out come complete setup
  records, each with the concatenated body, one CSDW summary, and a
  provenance map from every body offset to its packet and offset.
- **Boundary rule** (a reviewed interpretation under ADR-0022's two-person
  rule): a record is a run of consecutive data type `0x01` packets on one
  channel whose sequence numbers increase by one modulo 256 and whose CSDWs
  agree on FRMT and RCCVER; it ends at an intervening packet, a sequence gap,
  a CSDW change, or the end of input; anything ambiguous is reported.
- **Checksums:** `G\SHA` and the flex signature cover the assembled body.
- **Session rules** reported across records: ASCII and XML never mixed; SRCC
  = 1 preceded by a configuration-change event packet; setup records on
  channel `0x0000` from 106-17.
- **Edition codes:** RCCVER `0x07`–`0x0E` (106-07 to 106-22) with the rest
  reserved; `0x0E` reads as "106-22 or later"; `G\106` stays primary
  (ADR-0016). `irig106-time`'s mapping of `0x0F` to 106-23 contradicts
  106-24R1 and is to be corrected with the shared types in `irig106-types`
  (ADR-0009).
- **Acceptance test:** `multi_packet_setup_record_is_assembled`
  (`docs/TEST-DATA.md`).

### Consequences

* Good: multi-packet setup records are extracted, parsed, and checksummed
  correctly, and every diagnostic points to its packet.
* Good: one assembler serves the CLI, `irig106-ch10-reader`, `irig106-studio`,
  and `irig106-core`.
* Bad: the boundary rule is an interpretation, not a statement of the
  standard, and must be revisited if a future edition defines an end marker.
* Refines ADR-0019 (the CLI's reader now verifies header checksums).
  Diagram: `docs/diagrams/setup-record-assembly.svg`.
