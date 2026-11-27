# Historical Format 1 CSDW layout source review — 2026-11-27

Maintainer-now item from #80, scoped to L1-CH10-001/002 and their existing
L2/L3 children. Source inspection only, not implementation, a new owner
policy, completed INT development analysis or independent ADR-0033 review.
Inspected tmats base 283ffd5 after merged #80. The arriving legacy checks
are recorded separately, not used as a substitute for this source inspection.

## Retrieval and method

Read the rcc-106-standards archive manifest and release asset URLs on
2026-11-27. Downloaded missing chapter PDFs to temporary storage and reused
existing temporary copies only after recomputing SHA-256 against the manifest.
Every fingerprint below matches the archived manifest. No archive checkout,
PDF, source text dump or real recording is added to this repository.
106-04 has a full-volume asset rather than a separate Chapter 10 PDF.
The archive has no Chapter 10 asset for 106-00/01; no layout is inferred there.

Inspected extracted PDF text and the numeric bit labels, field definitions
and edition-specific figure/page references for each source below. This is
not a raster/rendered-PDF comparison or an executable decoder check. Text
extraction is a locating aid; citations refer to printed pages. Not all
figure numbers remain constant across editions. No blanket current-table
application to historical words is justified by visual similarity.

## Primary source fingerprints

| Edition / archive asset | SHA-256 |
|---|---|
| [106-04 106-04.pdf](https://github.com/TelemetryWorks/rcc-106-standards/releases/download/106-04/106-04.pdf) | `a75ef16a77ecb4838c0c9678b7d483049e84c94d7a49a3edb7f7199ce98914c8` |
| [106-05 chapter10.pdf](https://github.com/TelemetryWorks/rcc-106-standards/releases/download/106-05/chapter10.pdf) | `7aa84ef996df5ecd59da4159b82415ef2ba791a81dd20266c7d96a29adbf6bae` |
| [106-07 chapter10.pdf](https://github.com/TelemetryWorks/rcc-106-standards/releases/download/106-07/chapter10.pdf) | `50d125bc744001d803dcb8cadcb5c2038940ca980098afeb0ebcd25601d8e582` |
| [106-09 chapter10.pdf](https://github.com/TelemetryWorks/rcc-106-standards/releases/download/106-09/chapter10.pdf) | `1f93423591c43277009762c31c816358aead90bea9b570a4afc4c68e84ee0ec6` |
| [106-11 chapter10.pdf](https://github.com/TelemetryWorks/rcc-106-standards/releases/download/106-11/chapter10.pdf) | `8cef16928df5d3f93dfff6f95806e9923b41927413090b0844bf28313065c7a7` |
| [106-13 chapter10.pdf](https://github.com/TelemetryWorks/rcc-106-standards/releases/download/106-13/chapter10.pdf) | `e3740c7ac3099236f4e97cbe5b84c169340e6d4a4222769619e83ea0f8176fde` |
| [106-15 chapter10.pdf](https://github.com/TelemetryWorks/rcc-106-standards/releases/download/106-15/chapter10.pdf) | `cace968a4a796a56895eff36073840ae2db3a9c57dd33794d0c348e95fe0c204` |
| [106-17 chapter11.pdf](https://github.com/TelemetryWorks/rcc-106-standards/releases/download/106-17/chapter11.pdf) | `7297c782e0368e72ac783a72edccaa04eed28a8ea0e8ab7c61ea39379f516750` |
| [106-19 chapter11.pdf](https://github.com/TelemetryWorks/rcc-106-standards/releases/download/106-19/chapter11.pdf) | `ac06990ab1b98594cf70fd82815ae2d1efb6d7fb58bf329cf2dcc71a8b64ad9d` |
| [106-20 chapter11.pdf](https://github.com/TelemetryWorks/rcc-106-standards/releases/download/106-20/chapter11.pdf) | `e66370ce21b4ed44ed45f32e4785667398cf401e358f03e6f07494f06615748d` |
| [106-22 chapter11.pdf](https://github.com/TelemetryWorks/rcc-106-standards/releases/download/106-22/chapter11.pdf) | `e3e85ed09ddec95d49e9e1851ac4234b5df35b6340d9d91f355b1c598543d0a6` |
| [106-23 chapter11.pdf](https://github.com/TelemetryWorks/rcc-106-standards/releases/download/106-23/chapter11.pdf) | `ed30d02fe27cd89d9172ada824251fd6cd748bc87cb63c15f371152fa3274781` |
| [106-24 chapter11.pdf](https://github.com/TelemetryWorks/rcc-106-standards/releases/download/106-24/chapter11.pdf) | `d26e7926f4b2e7977df5e6210044cfc0d4c1191d45a66b71da2d530bf68de6a8` |
| [106-24R1 chapter11.pdf](https://github.com/TelemetryWorks/rcc-106-standards/releases/download/106-24R1/chapter11.pdf) | `0f63792dcac39f6010df371ee0afc46a396017918f4155cba9ebab6c2d6d263e` |

These are original-source documents mirrored by the project archive, not
reference-tool outputs. The archive manifest retains their original URLs.

## Per-edition Format 1 layout evidence

Every listed word is 32 bits (four bytes); text begins after that word, not
after a version-dependent guessed offset. Bit numbers below are wire-word
positions, not host-native struct layouts. The retained source word is never
rewritten. Reserved does not mean a known false flag or a declared version.

| Edition | Section / figure | Printed pages | Fields actually defined |
|---|---|---|---|
| 106-04 | 10.6.7.1 / 10.6.7.2.2, Fig. 10-34 | 10-48 | All 31–0 reserved; shared computer-generated CSDW, no version/SRCC/FRMT |
| 106-05 | 10.6.7.2, Fig. 10-34b | 10-55 | All 31–0 reserved; no version/SRCC/FRMT |
| 106-07 | 10.6.7.2 a, Fig. 10-50 | 10-71–72 | CH10VER 7–0; SRCC 8; 31–9 reserved; no FRMT |
| 106-09 | 10.6.7.2 a, Fig. 10-50 | 10-67–68 | CH10VER 7–0; SRCC 8; FRMT 9; 31–10 reserved |
| 106-11 | 10.6.7.2 a, Fig. 10-54 | 10-67–68 | CH10VER 7–0; SRCC 8; FRMT 9; 31–10 reserved |
| 106-13 | 10.6.7.2 a, Fig. 10-36 | 10-66–67 | CH10VER 7–0; SRCC 8; FRMT 9; 31–10 reserved |
| 106-15 | 10.6.7.2 a, Fig. 10-36 | 10-63–64 | CH10VER 7–0; SRCC 8; FRMT 9; 31–10 reserved |
| 106-17 | 11.2.7.2 a, Fig. 11-31 | 11-44–45 | RCCVER 7–0; SRCC 8; FRMT 9; 31–10 reserved |
| 106-19 | 11.2.7.2 a, Fig. 11-32 | 11-46–47 | RCCVER 7–0; SRCC 8; FRMT 9; 31–10 reserved |
| 106-20 | 11.2.7.2 a, Fig. 11-32 | 11-46–47 | RCCVER 7–0; SRCC 8; FRMT 9; 31–10 reserved |
| 106-22 | 11.2.7.2 a, Fig. 11-34 | 11-50–51 | RCCVER 7–0; SRCC 8; FRMT 9; 31–10 reserved |
| 106-23 | 11.2.7.2 a, Fig. 11-34 | 11-50–51 | RCCVER 7–0; SRCC 8; FRMT 9; 31–10 reserved |
| 106-24 | 11.2.7.2 a, Fig. 11-34 | 11-50–51 | RCCVER 7–0; SRCC 8; FRMT 9; 31–10 reserved |
| 106-24R1 | 11.2.7.2 a, Fig. 11-34 | 11-50–51 | RCCVER 7–0; SRCC 8; FRMT 9; 31–10 reserved |

The 106-04 layout is a common computer-generated word (section 10.6.7.1),
with Format 1 setup defined separately in 10.6.7.2.2. Do not cite the modern
Format 1 subsection numbering for it. In 106-05, Figure 10-34b gives a
specific Format 1 all-reserved word. SRCC first appears in the inspected
106-07 source; FRMT appears in 106-09, not only in 106-17. For every inspected
FRMT-bearing layout, bit 9 distinguishes Chapter 9 ASCII (0) and XML (1).
XML remains unsupported by tmats 0.1 under ADR-0015; unknown historical flag
availability cannot be converted into a successful empty ASCII document.

## Version table versus layout versus fallback

| Source edition | Defined low-byte entries; surrounding values reserved |
|---|---|
| 106-04/05 | No declared version byte; all-reserved word |
| 106-07 | 07 -> 106-07 only |
| 106-09 | 07 -> 106-07, 08 -> 106-09 |
| 106-11 | Adds 09 -> 106-11 |
| 106-13 | Adds 0A -> 106-13 |
| 106-15 | Adds 0B -> 106-15 |
| 106-17 | Adds 0C -> 106-17 |
| 106-19/20 | Adds 0D -> 106-19; 20 keeps that table |
| 106-22/23/24/24R1 | Adds 0E -> 106-22; later listed editions keep it |

From 106-07 onward the table reserves 00–06, plus codes above its last
listed entry. The table describes recording-format compliance, not the
TMATS-generating edition or when the bytes were physically written.
INT-012's newest literal-table reading and INT-016's reserved/not-declared
handling remain separate from a historical table's actual printed entries.
INT-017's 0D/0E newer-rules fallback is labelled owner POLICY, not an added
literal code or a reason to select a CSDW layout by circular inference.
INT-014/015 govern the separate G\106 declaration, not the wire layout.

The modern note saying CH10VER was the name in 106-04 through 106-15 is
over-broad: the actual 106-04/05 words define no version byte. Preserve this
source discrepancy with INT-016's existing historical reading; do not invent
a field because a later retrospective sentence names one. No new reading
or owner override is selected here.

Format 4 has a different CSDW/code space. Packet-header Data Type Version
is another code space and belongs to core; neither supplies a substitute
Format 1 version table. This note is not a complete Format 4 review.

## Consequences and remaining implementation gates

An explicitly identified supported layout is necessary under L2-CH10-001.
Retain its provenance and absent-versus-present field distinction; do not
pretend that an old reserved bit is FRMT/SRCC just because modern shifts
produce a value. A zero low byte does not uniquely identify 106-04/05, since
later layouts reserve 00 too. Nor can G\106 alone prove the packet layout:
it describes text generation, and is inside the payload being interpreted.

This source review does not select a caller API, automatic layout-selection
heuristic, unavailable-context fallback or historical reserved-bit severity.
Those require reviewed supported-layout binding and actual shared types/core
API evidence before implementation. If supplied layout evidence is missing
or contradictory, expose the gap; do not silently decode as modern/default.
No new policy is approved by this source review or by passing bit fixtures.

Existing core PacketFacts plus PacketInfo and borrowed DATA spans remain the
only packet boundary. Logical four-byte reads may cross spans; there is no
new reader, header reparse, packet checksum, mandatory coalescing or local
wire type. Retain core trust/integrity evidence unchanged. Field extraction
never proves record completeness, governance or session conformance.

## Planned evidence and limits of this review

TEST-DATA gains edition-specific source-word fixtures plus absent-field and
reserved-bit adversaries. Use hand-specified words under explicit layout
premises; compare bytes/locations and field availability, not expected values
computed by the decoder being checked. Shared decoder/API compatibility and
actual executable tests remain outstanding.

Local execution: SHA-256 verification of 14 archived PDF assets, bounded PDF
text extraction/source inspection, and documentation/trace checks. No Rust
build/test, assembly run, real-recording check or rendering comparison ran.
No tool/dependency installation or persistent/background job is needed.
Independent exact-head PR review remains required. This is one source-review
item, not completion of L1-CH10-001/002 or the separate approved-INT analysis
and Appendix 9-C inventory tasks; those retain their own PRs.
