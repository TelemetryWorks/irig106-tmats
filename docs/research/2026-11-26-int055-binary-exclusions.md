# INT-055: binary and other non-decimal exclusions

Date: 2026-11-26. Review of PR #51 at b43a469 returned CHANGES REQUESTED:
INT-054 final CONCUR; INT-055's only remaining concern was binary exclusion.
This makes the owner's 2026-11-25 “decimal-integer fields ... only” boundary
explicit; it does not add an owner choice. Final INT-054/055 concurrence
at ebd5c98 is now recorded in the [final review record](2026-11-26-int054-int055-final-concurrence.md). Earlier research records remain history.

## Dispatch boundary

INT-055 requires a reviewed definition identifying a decimal-integer value
or a numeric code in an EXPLICIT enumeration table. Binary patterns, masks,
hex, floating-point, character strings, dates, times, IP addresses and any
other non-decimal representation are outside it, including all-digit spellings.
Do not choose a grammar from the characters alone. A mixed representation's
non-decimal arms retain their own rule; unresolved dispatch stays unavailable.
An enumeration row is not blanket authorization: identify its reviewed code
arm first. A binary pattern is never a numeric-looking enumeration code.

The [earlier inventory](2026-11-25-int055-owner-answers.md) lists the 43
floating-point and 10 hex-bearing Range rows and extra Q-group hex arms.
The lists below complete the binary/mask inventory and record the full Range
scan. Separate representation rules need review; existing G\SHA rules remain.

## Binary-labelled Range rows (27)

Sources: 106-24R1 Chapter 9, named section/table, row and printed page.
Wrapped code-name suffixes are joined here without changing their identity.

| Section / table | Code name(s) | Page |
| --- | --- | --- |
| 9.5.4 / Table 9-4 | R-x\MFD\TRA-n-m | 9-57 |
| 9.5.4 / Table 9-4 | R-x\MFD\STA-n-m, R-x\MFD\DWC-n-m | 9-58 |
| 9.5.4 / Table 9-4 | R-x\MFD\RTRA-n-m, R-x\MFD\RSTA-n-m, R-x\MFD\RDWC-n-m | 9-59 |
| 9.5.4 / Table 9-4 | R-x\AMSK-n-m | 9-61 |
| 9.5.4 / Table 9-4 | R-x\DMSK-n-m | 9-65 |
| 9.5.4 / Table 9-4 | R-x\UCSV-n-m | 9-79 |
| 9.5.6 / Table 9-6 | P-d\MF5 (name on 9-100, Range on 9-101) | 9-100–101 |
| 9.5.8 / Table 9-8 | B-x\BID-i | 9-124 |
| 9.5.8 / Table 9-8 | B-x\U1M-i | 9-125 |
| 9.5.8 / Table 9-8 | B-x\U2M-i | 9-126 |
| 9.5.8 / Table 9-8 | B-x\U3M-i | 9-127 |
| 9.5.8 / Table 9-8 | B-x\TRA-i-n, B-x\STA-i-n | 9-129 |
| 9.5.8 / Table 9-8 | B-x\DWC-i-n, B-x\LBL-i-n | 9-130 |
| 9.5.8 / Table 9-8 | B-x\RTRA-i-n, B-x\RSTA-i-n, B-x\RDWC-i-n | 9-131 |
| 9.5.8 / Table 9-8 | B-x\MBM-i-n-p-e | 9-133 |
| 9.5.9 / Table 9-9 | S-d\MBM-i-n-p-e (name on 9-141, Range on 9-142) | 9-141–142 |
| 9.5.10 / Table 9-10 | Q-d\MHIM-i-n-m | 9-148 |
| 9.5.10 / Table 9-10 | Q-d\MBM-i-n-m-o-p | 9-150 |
| 9.5.10 / Table 9-10 | Q-d\SMHIM-i-n-m-o | 9-152 |
| 9.5.10 / Table 9-10 | Q-d\SMBM-i-n-m-o-p-e | 9-154 |

“Binary” includes fixed-width strings/patterns, don't-care X and FW arms;
none is decimal input for INT-055. In particular preserve 00101 as five
bits, not decimal 101; the decimal warning/ten-digit cap never applies.

## Masks/patterns without a Binary label in Range (11)

| Section / table | Code name | Page / printed Range |
| --- | --- | --- |
| 9.5.6 / Table 9-6 | P-d\AEF6-n-w | 9-106; 1 to size of word ... of 0,1 |
| 9.5.6 / Table 9-6 | P-d\FFI2 | 9-106; 1 to size of word ... of 0,1 |
| 9.5.6 / Table 9-6 | P-d\MLC1-n | 9-107; 1 to the size of the word ... of 0,1 |
| 9.5.6 / Table 9-6 | P-d\ADM\OHM-n | 9-109; 1 to the size of word ... of 0,1 |
| 9.5.6 / Table 9-6 | P-d\ADM\FDP-n | 9-109; 1 to the size of word ... of 0,1 |
| 9.5.6 / Table 9-6 | P-d\ADM\DOP-n | 9-109; 1 to the size of word ... of 0,1 |
| 9.5.6 / Table 9-6 | P-d\ADM\SDP-n | 9-109; 1 to the size of word ... of 0,1 |
| 9.5.6 / Table 9-6 | P-d\ADM\UDP-n | 9-109; 1 to the size of word ... of 0,1 |
| 9.5.7 / Table 9-7 | D-x\WFM-y-n-m-e | 9-117; 1-64 of 0,1 or FW |
| 9.5.7 / Table 9-7 | D-x\TD3-y-n-m | 9-119; 1 to 64 of 0,1 or FW |
| 9.5.7 / Table 9-7 | D-x\REL2-y-n-m | 9-120; 1-64 of 0,1 or FW |

The P-group definitions call these binary masks, patterns of 1s and 0s,
or patterns in binary; the three D rows define binary strings. This is why a
representation is determined from the complete row, not just Range digits.

## Every-Range-row inspection

Rechecked the previously downloaded [official 106-24R1 Chapter 9 PDF](https://www.trmc.osd.mil/wiki/download/attachments/335389015/Chapter9.pdf?api=v2):
SHA-256 `dfc5ebfb3e6f02f58389126863d43a45d89c7e2d0533a1c690d9537531c740cb`.
Used layout extraction, separating Usage Range text from the Definition
column, and inspected the distinct Range forms and associated definitions.
Correction after final review at ebd5c98: eight wrapped P-group mask Range
continuations were initially left in the 239-row review bucket. Inspecting
their full rows moves them into the unlabelled mask bucket (3 to 11), leaving
231 review rows; total 729 and all other buckets stay unchanged. This corrects
the inventory, not the already-concurred decimal-only interpretation.
A full-text search for “binary” incorrectly includes decimal word-position
rows whose Definition merely mentions binary data; those are not masks.
Wrapped Range and code-name continuations are read with their preceding page.

The scan finds 731 Range labels/forms: 726 with colon and 5 using equals or
no punctuation. Two are explanatory prose on 9-6, leaving 729 table Range
rows. The 5 alternate forms are R-x\EPL\LSSN-n (9-42), D-x\SON-y-n (9-118),
Q-d\SMHO-i-n (9-150), C-d\BWT\N and C-d\BWTB-n (9-161).

| Inspection bucket | Rows | INT-055 disposition |
| --- | ---: | --- |
| Floating point, including positive floating point | 43 | Excluded; prior inventory lists each |
| Hex/structured/mixed hex | 10 | Excluded; prior inventory lists each |
| Binary-labelled representations | 27 | Excluded; listed above |
| Binary masks/patterns labelled 0,1 (with or without FW) | 11 | Excluded; listed above |
| Character-length rows, including T-x\COM/P-d\COM printed only as Range 3200 | 197 | Strings, not decimal by all-digit spelling |
| Date/time and IP representations | 10 | Excluded; own representation rules |
| Enumeration rows | 200 | Only reviewed numeric table-code arms can use INT-055 |
| Remaining integer/symbolic/mixed forms | 231 | Review full definition; never infer decimal from Range spelling |
| Total table Range rows | 729 | Every scanned row accounted for |

The last bucket includes decimal domains, symbolic bounds, integer-or-N/NO/BF
arms and bits-per-word choices. It is not 231 automatically executable
integer rules: a literal/structured arm stays out, unsupported definitions
stay unavailable, and the extra Q-group hex-capable definitions remain
excluded on their hex arms. The representation vocabulary in section 9.5.1 a
p. 9-6 distinguishes dates, addresses, decimal/hex/binary and character values.
This is baseline source inspection, not registry completion, a runtime test
or the required deep cross-edition development analysis.

## Planned regression

Extend numeric_exclusions_dispatch_by_definition with MFD TRA=00101,
32-bit DMSK, 64-bit AMSK, 81-bit UCSV, FW and X-pattern arms, plus D-group
masks and P-d\ADM\FDP-n=0011 whose Range lacks “Binary”. Assert no INT-055 zero stripping, decimal
warning/range comparison or ten-digit rejection; raw width/bytes stay intact.
A reviewed binary rule may give its own result; if absent, remain unavailable,
not fabricated Explicit. Strings/dates/IP containing digits likewise never
enter decimal dispatch. Final interpretation concurrence is recorded separately;
source freshness, development analysis and executable verification remain.
