# Family 2 batch 22: packet trust and exact setup slicing

Date: 2026-11-26. Follows merged #53 (head df9814a, merge f61f1d0).
Requirements/planned fixtures only; no packet reader implementation or report
format is approved. L1-CLI-008 derives into L2-CLI-003/004 and L3-CLI-004–013.
This continues the existing ADR-0019/0025 mechanics before CLI implementation.

## Verified primary source

Downloaded and inspected official [106-24R1 Chapter 11, January 2025](https://www.trmc.osd.mil/wiki/download/attachments/335389015/chapter11.pdf?api=v2).
SHA-256: `0f63792dcac39f6010df371ee0afc46a396017918f4155cba9ebab6c2d6d263e`.
This matches the baseline used in earlier setup/CSDW research. Source
inspection is not a passing runtime test or completed cross-edition analysis.

| Source | Printed pages | Mechanical fact |
| --- | --- | --- |
| 11.1 | 11-1 | Structural fields use little-endian byte ordering; data retains native order |
| 11.2.1 c / Table 11-3 | 11-4 | Setup Format 1 maximum 134,217,728 bytes; ordinary maximum 524,288 |
| 11.2.1.1 a–c | 11-5 | Fixed 24-byte primary header, sync 0xEB25, Packet Length includes all packet parts and is a multiple of four |
| 11.2.1.1 d / Table 11-4 | 11-6 | Data Length includes CSDW/data, excludes filler/checksum; setup type is 0x01 |
| 11.2.1.1 g | 11-8–9 | Bit 7 enables secondary header; bits 1–0 choose no/8/16/32-bit data checksum |
| 11.2.1.1 j | 11-10 | Header checksum is 16-bit arithmetic sum of 16-bit words, excluding its checksum word |
| 11.2.1.2 | 11-10 | Secondary header is 12 bytes; its checksum sums BYTES, excluding its checksum word |
| 11.2.1.3–4 | 11-11–12 | Data checksum covers body AND filler; optional filler can also keep channel packet sizes fixed |
| 11.2.7.2 / Figure 11-34 | 11-50 | Four-byte Format-1 CSDW precedes each setup packet's text fragment |

The secondary checksum's printed unit is explicitly different from the
primary unit. Do not borrow a familiar word-sum implementation from another
project: use the cited rule and a byte-distinguishing fixture. Likewise the
trailer prose explicitly includes filler in data checksum even though Data
Length excludes it. These mechanics follow the printed baseline, not a new
interpretation or a changed shared-types mapping. Other editions/profile
bindings and any source discrepancy require their own review before code.

## Trust ordering and intervals

ADR-0025 requires header checksums before trusting lengths. Read a bounded
fixed primary header first, establish its checksum, then use its bit-7 flag
to obtain fixed secondary bytes and verify them. No raw length may drive
allocation, source ranges or traversal before that prerequisite is met.
Bounds/alignment/profile checks still follow successful checksums.

Use O=packet source offset, H=24 or 36, P=Packet Length, D=Data Length,
C=0/1/2/4. After checked containment/alignment and P>=H+D+C:

- Body: [O+H,O+H+D).
- Format-1 raw CSDW: [O+H,O+H+4), requiring D>=4.
- Format-1 TMATS fragment: [O+H+4,O+H+D).
- Filler: [O+H+D,O+P-C).
- Data checksum: [O+P-C,O+P) if present.
- Data checksum domain: [O+H,O+P-C), which includes filler.

Do not assume filler is at most three bytes or scan the body for a new packet
sync. Do not consume secondary timestamps or fragment CSDWs as text. A zero
text span with D=4 is a structural case, not a complete-record/default input.
Decoded fragments feed the existing per-channel assembler; this slice never
joins packets itself or imports a different RCCVER/data-type-version mapping.

## Independent planned oracles

Construct small byte vectors with separately supplied region boundaries and
checksum values; compare exact output spans, not a hash or another parse.
A secondary header whose first ten bytes are 01 02 03 04 05 06 07 08 00 00
has byte-sum 0x0024 versus word-sum 0x1410, explicitly distinguishing units.
Its reserved bytes are zero. Do not infer a time scale from those bytes.

Use CSDW 00 00 00 00 plus text bytes 61 62 63 (D=7): for an 8-bit checksum
without extra filler, sum=0x26. With eight FF filler bytes, sum=0x1E. The
text fragment stays 61 62 63 in both; these snippets are payload mechanics,
not conforming complete TMATS fixtures. Header/secondary bytes are excluded
from the checksum domain. Also cover 16/32-bit widths and no checksum.

Trust/bounds mutations include incomplete fixed headers, checksum mismatch,
wrapped/oversized lengths, impossible H/D/C combinations, non-four-aligned
P, missing CSDW and source truncation. Standard limit tests use arithmetic
metadata, not allocations of 128 MiB. Expected checksum states/offsets are
not computed by the production reader under test. Future generated cases
need a published finite test budget; no exhaustive local scan is performed.

## Owner boundaries and remaining decisions

No owner decision blocks these already-decided byte/checksum mechanics.
They do not choose whether/how to adopt a payload with failed data checksum,
a damaged-file resynchronization policy, severity, process exit code or a
trusted-time fallback. Preserve the failed/internal evidence without calling
it verified; any downstream-use policy needs its own reviewed contract.

Per the owner's instruction after #53: STOP for owner direction before
report/rendered-output formats are derived, including findings, assumption
labels and suppressed markers. This slice makes no text/JSON schema, layout,
message wording or display-field decision and changes no report renderer.
That owner gate remains ahead of the future presentation slice.

Local audits verify source hash, arithmetic vectors, requirements/parents,
planned case links, trace regeneration/--check, whitespace and unchanged
render/schema/runtime scope. Heavy Rust/CI verification belongs to GitHub;
exact-head run IDs go in the PR. All new requirements remain Draft; source
freshness, full development analysis and executable tests precede code.
