# INT-055: owner-signed clarification answers and excluded fields

Owner: Joey. Decision date: 2026-11-25; recorded 2026-11-26 in PR #51.
All three answers are owner-signed under ADR-0033. Independent AI recheck
of the amended wording is pending. Earlier owner/reviewer records remain
history; this record supersedes their unresolved choices and recommendations.

## Owner answers (verbatim)

> 1. A: a leading blank makes EVERY numeric field value and numeric-looking code "unavailable" (INT-041, applied uniformly). A trailing or embedded blank is Invalid.
> 2. A (overrides the projection-first recommendation): the digit check runs on the RAW value bytes. Any control byte (CR, LF or other) inside a numeric value makes it Invalid. INT-036 projection is not used to repair numbers. Update numeric_projection_keeps_character_findings: NSB 0\r\n4 is now Invalid.
> 3. A: INT-055 covers decimal-integer fields and numeric-looking enumeration codes only. Floating-point fields (24 ranges, e.g. ITH p.9-52, record speed p.9-32) and hexadecimal fields (10 ranges, e.g. MRC\SPM/SRM p.9-48, CMD/RCMD) are explicitly out of scope, with their own reviewed rules later. List them.

## Source inspection and counting convention

Re-read the [official 106-24R1 Chapter 9 PDF](https://www.trmc.osd.mil/wiki/download/attachments/335389015/Chapter9.pdf?api=v2)
on 2026-11-26. Downloaded SHA-256:
`dfc5ebfb3e6f02f58389126863d43a45d89c7e2d0533a1c690d9537531c740cb`.
The lists below identify printed table rows, not instantiated occurrences.
The 24 floating-point ranges are those whose Range starts with “Floating
point” (case-insensitive). Another 19 Range rows say “positive floating
point”; those are ALSO floating-point fields and excluded, not decimal
integers merely because their sign is constrained. All ten hex-bearing
Range rows include structured/mixed forms; they are listed separately below.
No new floating/hex grammar is approved here; existing separately reviewed
rules (notably G\SHA) retain their authority. This is source inspection and
an exclusion inventory, not completed cross-edition development analysis.

## Floating-point exclusions: 24 Range rows starting “Floating point”

All references: 106-24R1 Chapter 9, section/table and printed page below.
Each listed code-name row is excluded even when its given value is digits only.

| Section / table | Code name(s) | Printed page |
| --- | --- | --- |
| 9.5.3 / Table 9-3 | T-x\RF7 | 9-17 |
| 9.5.4 / Table 9-4 | R-x\TC4 | 9-31 |
| 9.5.4 / Table 9-4 | R-x\TC6 | 9-32 |
| 9.5.4 / Table 9-4 | R-x\ITH-n | 9-52 |
| 9.5.4 / Table 9-4 | R-x\SGV-n | 9-74 |
| 9.5.11 / Table 9-11 | C-d\BWTV-n (Floating Point or S), C-d\MC3-n | 9-161 |
| 9.5.11 / Table 9-11 | C-d\MA3-n, C-d\FDL | 9-162 |
| 9.5.11 / Table 9-11 | C-d\MOT1, C-d\MOT2, C-d\MOT3, C-d\MOT4, C-d\MOT5, C-d\MOT6, C-d\MOT7 | 9-163 |
| 9.5.11 / Table 9-11 | C-d\PS3-n, C-d\PS4-n | 9-165 |
| 9.5.11 / Table 9-11 | C-d\CO, C-d\CO-n, C-d\NPC, C-d\NPC-n | 9-166 |
| 9.5.11 / Table 9-11 | C-d\DPC-n (code on 9-167, Range on 9-168), C-d\DICC-n | 9-167–168 |

## Additional floating-point exclusions: 19 positive floating-point rows

| Section / table | Code name(s) | Printed page |
| --- | --- | --- |
| 9.5.4 / Table 9-4 | R-x\ASR-n | 9-60 |
| 9.5.4 / Table 9-4 | R-x\ASBW-n-m, R-x\AII-n-m, R-x\AGI-n-m, R-x\AFSI-n-m | 9-62 |
| 9.5.4 / Table 9-4 | R-x\AOVI-n-m | 9-63 |
| 9.5.4 / Table 9-4 | R-x\DSR-n | 9-65 |
| 9.5.4 / Table 9-4 | R-x\VAC-n | 9-68 |
| 9.5.4 / Table 9-4 | R-x\CBR-n, R-x\VBR-n, R-x\VED-n | 9-69 |
| 9.5.4 / Table 9-4 | R-x\IFR-n, R-x\PTG-n, R-x\TOTF-n, R-x\EXP-n | 9-74 |
| 9.5.4 / Table 9-4 | R-x\UCR-n-m | 9-78 |
| 9.5.4 / Table 9-4 | R-x\UCBR-n-m | 9-79 |
| 9.5.4 / Table 9-4 | R-x\CBBS-n-m | 9-82 |
| 9.5.6 / Table 9-6 | P-d\D2 | 9-95 |

## Hexadecimal/hex-bearing exclusions: 10 Range rows

| Section / table | Code name(s) | Printed page / representation |
| --- | --- | --- |
| 9.5.2 / Table 9-2 | G\SHA | 9-13; integer designator, hyphen, hex; existing INT-018–021/043 rules retained |
| 9.5.4 / Table 9-4 | R-x\MRC\SPM, R-x\MRC\SRM | 9-48; hexadecimal 0000–FFFF |
| 9.5.4 / Table 9-4 | R-x\CMD-n-m | 9-57; hexadecimal 0000–FFFF |
| 9.5.4 / Table 9-4 | R-x\RCMD-n-m | 9-58; hexadecimal 0000–FFFF |
| 9.5.8 / Table 9-8 | B-x\CMD-i-n | 9-129; hexadecimal |
| 9.5.8 / Table 9-8 | B-x\RCMD-i-n | 9-131; hexadecimal |
| 9.5.9 / Table 9-9 | S-d\MDEL-i, S-d\FDEL-i-n | 9-138; Hex or Enums |
| 9.5.9 / Table 9-9 | S-d\MID-i-n | 9-139; ASCII or Hex |

This ten-row count is not a claim that every hex-capable field says Hex
in its Range: Table 9-10 Q-d\MHIV-i-n-m p. 9-148, Q-d\SMHIV-i-n-m-o
p. 9-152 and Q-d\SNOMM\N-i-n-m p. 9-152 describe 0x-prefixed hex in
the definition. Those representation arms are also outside INT-055 and
need their own reviewed rules; decimal-only syntax must not reject their
hex arms by accident. Chapter 9 section 9.5.1 a's representation definitions
(p. 9-6) remain source authority, not a parser implemented by this inventory.

## Consequences for current planned cases

The existing `numeric_projection_keeps_character_findings` name remains for
trace continuity, but actual CR/LF in NSB 0\r\n4 now yields Invalid from raw
bytes, never Explicit 4. Check every control byte in numeric values and keep
raw located findings. Do not change INT-036's code-name or general text
projection. Uniform leading-blank cases explicitly cover NSB/TK1/R\N and
TTF/SHTF/TFMT; trailing and embedded blanks remain Invalid. An invalid or
unavailable R\N cannot prove applicability. Out-of-scope representation tests
must assert no INT-055 verdict, without inventing a separate usable float/hex
value before its own rule is reviewed. Development/source freshness and
executable tests still gate implementation after independent recheck.
