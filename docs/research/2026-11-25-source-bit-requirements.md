# Family 2 batch 17: explicit source-bit partition requirements

Date: 2026-11-25. Requirements and PLANNED tests only, not implementation or
completed development analysis. Batch 16 identified explicit NSB splitting
as the next bounded derivation; this slice derives L1-VIEW-005 with existing
L1-VIEW-004 effective-field support. It adds L2-VIEW-008/009 and
L3-VIEW-023–030 without changing the interpretation register.

## Primary sources and scope

- [106-24R1 Chapter 9](https://www.trmc.osd.mil/wiki/download/attachments/335389015/Chapter9.pdf?api=v2),
  section 9.5.4 Table 9-4 NUMBER OF SOURCE BITS R-x\NSB, printed p. 9-33:
  recorder-level count of high channel-ID bits, domain 0–13, with explicit
  zero for one source. Its applicability is based on specified R\ID.
- [106-24R1 Chapter 11](https://www.trmc.osd.mil/wiki/download/attachments/335389015/chapter11.pdf?api=v2),
  section 11.2.1.1 b and b(1), printed p. 11-5: two-byte channel-ID value,
  high bits for multiplexer source ID and remaining low bits for the local
  channel. No packet parser or historical conformance cutoff is derived here.
- Chapter 9 Table 9-4 TRACK NUMBER/CHANNEL ID R-x\TK1-n, p. 9-48, distinguishes
  the actual 1–65535 ID from entry index n. Batch 16 keeps this identity.
- RELEASE-0.1 and the 2026-11-21 owner direction permit a split only with an
  explicit valid declaration, never a presumed/default bit count.

The official Chapter 11 download inspected on 2026-11-25 is 1,847,593 bytes,
SHA-256 `0f63792dcac39f6010df371ee0afc46a396017918f4155cba9ebab6c2d6d263e`.
Chapter 9 retains the fingerprint verified in batch 16:
`dfc5ebfb3e6f02f58389126863d43a45d89c7e2d0533a1c690d9537531c740cb`.
Reading/fingerprinting standards is source inspection, not executable coverage.
Other editions need their own supported reviewed definition/layout evidence;
no baseline bit range or conformance rule is silently extended to them.

## No new owner decision or interpretation

Approved INT-035–039 retain framing, projection, canonical recorder identity
and scalar multiplicity. Approved INT-041 retains unavailable prerequisites;
INT-042 handles the existing specified-attribute condition if evaluated.
The five supported-field states stay unchanged. Invalid/duplicate/unknown
metadata never becomes an explicit count by truncation, trimming or a first
match. An absent NSB does not mean an explicit zero or a defaulted single source.
No new blank-handling rule, decoder, public Rust type or governing policy is
selected. The operation takes supplied recorder/entry and full-ID evidence;
it does not decide channel enumeration or clock association.

## Arithmetic specification and independent planned oracle

Let C be the usable full 16-bit channel ID, N the explicit count and W=16-N.
High N bits form the source ID; low W bits form the local ID. The numerical
meaning is source=floor(C/2^W), local=C mod 2^W. Preserve C and both widths.
At N=0 there is no high-bit substring: source=0 and local=C. Implementations
must avoid a shift by their native type width and other wrapping arithmetic.
A derived zero local component is not the reserved full channel ID zero.

The fixed examples below were checked locally by splitting a zero-padded
16-character binary string, rather than invoking a library split routine:

| N | Full C (numeric fixture notation) | Source | Local | Source / local bit strings |
| ---: | --- | ---: | ---: | --- |
| 0 | 0x002A (42) | 0 | 42 | empty / 0000000000101010 |
| 4 | 0xA02A (41002) | 10 | 42 | 1010 / 000000101010 |
| 4 | 0xB02A (45098) | 11 | 42 | 1011 / 000000101010 |
| 13 | 0xFFFF (65535) | 8191 | 7 | 1111111111111 / 111 |
| 13 | 0xA000 (40960) | 5120 | 0 | 1010000000000 / 000 |
| 1 | 0x8000 (32768) | 1 | 0 | 1 / 000000000000000 |

Hex is fixture notation, not a new TMATS numeric lexical rule. Full TK1 zero
remains outside its declared range, whereas zero-valued derived components
are not independently reinterpreted as TK1. Two full IDs can share a local
ID without becoming one channel or proving a shared time reference.

Planned executable verification streams all 917,490 baseline C/N pairs
(C=1–65535, N=0–13) against a bit-string oracle, then asserts reconstruction,
width/range and lossless source evidence. Focused parsed-input tests separately
cover declaration validity, source scope, provenance and qualifiers. The full
runtime arithmetic product was NOT executed locally; it belongs in bounded
CI verification once the reviewed implementation exists. Six vector checks
are planning evidence only, not requirement coverage or deep analysis.

## #48 follow-ups and remaining derivations

**DSI is explicitly queued:** R-x\DSI-n (DATA SOURCE ID, Table 9-4 p. 9-49)
needs a later scoped-context derivation. It is source context, not a clock
reference. This batch neither reads a typed DSI nor treats a numeric source
component as that string, TSRC or a time-channel association. The ROADMAP
remaining-derivations list records the same boundary.

**CHE handling is pending an owner decision.** The follow-up to merged #48
recommended to the owner that a disabled TIMEIN entry still be enumerated,
with CHANNEL ENABLE R-x\CHE-n retained as context rather than silently
filtered. The instruction explicitly said not to specify this until owner
confirmation. The recommendation is NOT a decision or requirement here;
no CHE-dependent enumeration/filter rule or acceptance expectation is added.
The field's source is Table 9-4 CHANNEL ENABLE, p. 9-49.

Remaining time-view derivations include DSI context, CHE after owner choice,
and association-uncertainty/caller-assumption provenance. General measurement
views and full timelines remain 0.2. The pending CHE choice does not block
this numerical partition contract for an already supplied channel identity.

No Rust, manifest, workflow or interpretation changes are needed. Light
local checks cover source/vector consistency, requirement IDs/parents,
fixture links, trace drift and whitespace. Substantive checks run through
GitHub CI; run IDs and outcomes will be recorded in the PR. A green prototype
suite does not verify these new requirements or the future consumer build.
