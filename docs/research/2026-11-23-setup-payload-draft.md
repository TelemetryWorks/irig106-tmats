# Format 1 payload boundary and CSDW source review — 2026-11-23

Family 2 batch 12 derives the bounded payload/field portion of L1-CH10-001,
002 and 006. It is source inspection and requirement derivation, not completed
development analysis or implementation verification. No new interpretation
is proposed; no existing interpretation status is changed.

## Source inspected

[106-24R1 Chapter 11](https://www.trmc.osd.mil/wiki/download/attachments/335389015/chapter11.pdf?api=v2),
January 2025 publication:

- Section 11.1, printed page 11-1, specifies least-significant byte first for
  the data structures, including CSDWs, and retains packet data's native
  order. The adapter reads the CSDW little endian; it does not byte-swap text.
- Section 11.2.7.2 a and Figure 11-34, page 11-50, place a 32-bit CSDW at
  the beginning of each Format 1 packet body. Bits 31–10 are reserved,
  FRMT is bit 9 (0 ASCII, 1 XML), SRCC bit 8, RCCVER bits 7–0.
- The RCCVER table on page 11-51 assigns 07,08,09,0A,0B,0C,0D,0E to RCC
  106-07,09,11,13,15,17,19,22 and reserves 00–06 and 0F–FF. The table's
  0E declaration is literally RCC 106-22, not a newer edition.

| Baseline raw CSDW bytes | Raw word | RCCVER | SRCC | FRMT |
| --- | --- | --- | --- | --- |
| `0E 00 00 00` | `0x0000000E` | `0x0E` | 0 | 0 (ASCII) |
| `0E 01 00 00` | `0x0000010E` | `0x0E` | 1 | 0 (ASCII) |
| `0E 02 00 00` | `0x0000020E` | `0x0E` | 0 | 1 (XML) |
| `0E 03 00 00` | `0x0000030E` | `0x0E` | 1 | 1 (XML) |
| `0E 03 00 80` | `0x8000030E` | `0x0E` | 1 | 1 (XML) |

The last fixture retains a reserved high bit, without choosing a severity or
rejection policy. These constants come from the cited bit positions, not
from production decoder output. A payload shorter than four bytes has no
complete baseline CSDW; exactly four bytes supplies an empty fragment body,
not a complete/valid setup record.

## Existing decisions and dependency boundary

ADR-0009 requires shared irig106-types definitions, not a local copy of wire
codes. The shared API/version and actual integration need verification before
implementation; no other repository was accessed or its current API asserted.
Adapter tests must use independently sourced bytes, not call the shared
decoder again to manufacture their expected answers.

ADR-0025 leaves packet/header/checksum/filler slicing to the CLI's reader,
then assembles bodies/provenance in the library. This adapter consumes a
bounded body payload and removes only its own CSDW. Per-fragment CSDWs and
body splits remain distinct until later assembly. A split BOM must reach
INT-037's assembled-text-offset-zero check intact; no fragment-level strip.
ADR-0015 reports XML unsupported, preserving raw bytes without scanning it
as code-name TMATS. None of those outcomes proves completeness or validity.

SRCC is local evidence only. FRMT/RCCVER joining checks do not prove SRCC
agreement (ADR-0025). The fragment-summary and session first-record/change/
event questions remain separate; approved F7 supplies no event identity.
Raw reserved bits and all unsupported/unavailable evidence remain accessible.

## INT-012 — plain-English review summary

The existing owner decision, revised 2026-09-27 in ADR-0032, reports a code
as its table says: 0E is RCC 106-22. A separately labelled archive observation
may say later archived editions kept that code, but this is neither a newer
declaration nor a guarantee about future editions. 0D similarly names RCC
106-19; archive reuse and edition fallback are separate facts/policy.

Recommendation: retain that existing literal-reading decision. A rejected
alternative would label 0E as 106-22 or later, implying more than the table
states. INT-012's owner acceptance remains recorded; its independent AI
review is still pending under ADR-0033. The owner may reconfirm that existing
choice while routing the independent check. This PR supplies no new approval,
completed cross-edition analysis or test evidence for it.

Historical routing and INT-016's older absent/CH10VER declaration semantics,
INT-014/015's G\106 reading and INT-017's fallback are later derivations with
their own review gates. This explicit baseline fixture set neither overrides
those entries nor silently applies a newest table to an unknown era.

## Remaining work

TEST-DATA holds planned length, field, reserved-code, XML, body preservation,
provenance, SRCC and declaration-separation cases. These must be implemented
and run through the shared adapter later. Full assembly/completeness,
historical layout routing, session checks, edition selection and bounded time
views remain family-2 work before their implementation families. Full deep
source/edition/handbook analysis and readiness evidence are still required.
