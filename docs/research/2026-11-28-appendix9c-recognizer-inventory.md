# Appendix 9-C whole-name recognizer inventory — 2026-11-28

Maintainer-now vocabulary item from #80, for L3-READ-034's minimum Appendix
coverage. Source inspection and documentary inventory only; no recognizer,
registry implementation, code generation or positive runtime coverage exists.
No owner-held pattern query syntax or V/X decomposition is chosen.

Source: 106-24R1 Chapter 9, [archived PDF](https://github.com/TelemetryWorks/rcc-106-standards/releases/download/106-24R1/Chapter9.pdf),
SHA-256 dfc5ebfb3e6f02f58389126863d43a45d89c7e2d0533a1c690d9537531c740cb.
Recomputed against the fresh archive manifest on 2026-11-28. Inspected
Appendix 9-C pp. C-1–C-32 (PDF pp. 201–232), including code-name listings,
full-name references in XML examples, and relative XML annotations. Printed
page footers govern; synthetic extraction header page labels do not.

## Extraction scope and reproducibility

The [literal inventory](2026-11-28-appendix9c-literal-names.csv) records every
full-name token immediately followed by optional whitespace and ':' in the
inspected Appendix text: 374 occurrences, 357 distinct literal names and
118 definition patterns below. Includes malformed colon-ended examples;
these are observed name spellings, not 374 well-framed attributes or one
conforming document. Repeated example references/XML comments are not scalar
duplicates within a parsed setup. Counts describe this extraction, not all
registry attributes, all possible names or a required warning count.

Bounded source method: per-page text, printed C-page footer, then a full-name
token with group, optional decimal occurrence and backslash components,
optional decimal index suffixes, immediately before ':'. Lowercase path
letters are allowed by the inventory search; no missing separators/indices
are repaired. Pattern grouping replaces only hyphenated decimal tokens,
not embedded keyword digits (MF1, IDC10, PS3). CSV keeps every literal name,
all printed pages and occurrence count; no values/source paragraphs copied.
A local second counter checks the distinct literal set/counts independently.

For definition evidence, locate actual Code Name table cells, not a Links-to
reference or a group diagram alone. 113 cells are unwrapped; five wrapped
cells were reconstructed from their printed code-name column continuations:
M BB DLN p. 9-88, D MML N/MNF N/WP/WFM p. 9-117. This is source-column
reading, not repair of an observed TMATS token. The table below preserves the
printed index placeholders and exact arity. Each source citation is its row
in the named 106-24R1 Chapter 9 table.

## Required minimum definition patterns

Arity counts ALL index positions, including the occurrence. It is not a
range check or integer-width limit; INT-038 canonical-string identity applies.
Literal ASCII case matching follows 9.4.2 p. 9-2; INT-036/037 source/projection
and INT-038 name/index findings remain, with original bytes/locations retained.
A match recognizes a complete name shape, not a valid value, correct count,
link, default, record qualification or permission to make an edit.

| Definition row / complete shape | Index arity | Appendix pages | Table / printed row page | Observed occurrences |
|---|---:|---|---|---:|
| `G\PN` | 0 | C-2 | 9-2 / 9-10 | 1 |
| `G\TA` | 0 | C-2 | 9-2 / 9-10 | 1 |
| `G\OD` | 0 | C-2 | 9-2 / 9-10 | 1 |
| `G\RN` | 0 | C-2 | 9-2 / 9-10 | 1 |
| `G\TN` | 0 | C-2 | 9-2 / 9-10 | 1 |
| `G\POC1-n` | 1 | C-2, C-3 | 9-2 / 9-10 | 2 |
| `G\POC2-n` | 1 | C-2, C-3 | 9-2 / 9-11 | 2 |
| `G\POC3-n` | 1 | C-2, C-3 | 9-2 / 9-11 | 2 |
| `G\POC4-n` | 1 | C-2, C-3 | 9-2 / 9-11 | 2 |
| `G\DSI-n` | 1 | C-3 | 9-2 / 9-11 | 2 |
| `G\DST-n` | 1 | C-3 | 9-2 / 9-11 | 2 |
| `G\COM` | 0 | C-3 | 9-2 / 9-13 | 1 |
| `T-x\ID` | 1 | C-3 | 9-3 / 9-16 | 1 |
| `T-x\RF1` | 1 | C-3 | 9-3 / 9-16 | 1 |
| `T-x\RF2` | 1 | C-3 | 9-3 / 9-16 | 1 |
| `T-x\RF3` | 1 | C-3 | 9-3 / 9-16 | 1 |
| `T-x\RF4` | 1 | C-3 | 9-3 / 9-16 | 1 |
| `T-x\RF5` | 1 | C-3 | 9-3 / 9-17 | 1 |
| `T-x\SCO\N` | 1 | C-3 | 9-3 / 9-17 | 1 |
| `T-x\AN2` | 1 | C-3 | 9-3 / 9-18 | 1 |
| `T-x\AP\POC1` | 1 | C-3 | 9-3 / 9-18 | 1 |
| `T-x\AP\POC2` | 1 | C-3 | 9-3 / 9-18 | 1 |
| `T-x\AP\POC3` | 1 | C-3 | 9-3 / 9-18 | 1 |
| `T-x\AP\POC4` | 1 | C-3 | 9-3 / 9-18 | 1 |
| `R-x\ID` | 1 | C-3, C-27 | 9-4 / 9-30 | 2 |
| `R-x\R1` | 1 | C-3 | 9-4 / 9-30 | 1 |
| `R-x\TC1` | 1 | C-3 | 9-4 / 9-30 | 1 |
| `R-x\RI1` | 1 | C-3 | 9-4 / 9-33 | 1 |
| `R-x\RI2` | 1 | C-3 | 9-4 / 9-33 | 1 |
| `R-x\N` | 1 | C-3 | 9-4 / 9-32 | 1 |
| `R-x\RI3` | 1 | C-3 | 9-4 / 9-33 | 1 |
| `R-x\RI4` | 1 | C-3 | 9-4 / 9-33 | 1 |
| `R-x\POC1` | 1 | C-3 | 9-4 / 9-33 | 1 |
| `R-x\POC2` | 1 | C-3 | 9-4 / 9-33 | 1 |
| `R-x\POC3` | 1 | C-3 | 9-4 / 9-34 | 1 |
| `R-x\POC4` | 1 | C-3 | 9-4 / 9-34 | 1 |
| `R-x\TK1-n` | 2 | C-3 | 9-4 / 9-48 | 2 |
| `R-x\DSI-n` | 2 | C-3 | 9-4 / 9-49 | 2 |
| `R-x\CDT-n` | 2 | C-3 | 9-4 / 9-50 | 2 |
| `R-x\CDLN-n` | 2 | C-3 | 9-4 / 9-50 | 2 |
| `M-x\ID` | 1 | C-3 | 9-5 / 9-87 | 1 |
| `M-x\BB1` | 1 | C-3 | 9-5 / 9-87 | 1 |
| `M-x\BB2` | 1 | C-3 | 9-5 / 9-87 | 1 |
| `M-x\BSG1` | 1 | C-3 | 9-5 / 9-87 | 1 |
| `M-x\BSF2` | 1 | C-3 | 9-5 / 9-88 | 1 |
| `M-x\BB\DLN` | 1 | C-3 | 9-5 / 9-88 | 1 |
| `P-d\DLN` | 1 | C-6, C-7, C-8, C-18, C-28, C-31 | 9-6 / 9-95 | 7 |
| `P-d\D1` | 1 | C-6, C-7 | 9-6 / 9-95 | 2 |
| `P-d\D2` | 1 | C-6, C-7 | 9-6 / 9-95 | 2 |
| `P-d\D3` | 1 | C-6, C-7 | 9-6 / 9-95 | 2 |
| `P-d\D4` | 1 | C-6, C-7 | 9-6 / 9-96 | 2 |
| `P-d\D6` | 1 | C-6 | 9-6 / 9-96 | 1 |
| `P-d\D7` | 1 | C-6 | 9-6 / 9-96 | 1 |
| `P-d\TF` | 1 | C-6, C-7, C-8 | 9-6 / 9-97 | 3 |
| `P-d\F1` | 1 | C-6, C-7, C-8 | 9-6 / 9-97 | 3 |
| `P-d\F2` | 1 | C-6, C-7, C-8 | 9-6 / 9-98 | 3 |
| `P-d\F3` | 1 | C-6, C-7, C-8 | 9-6 / 9-98 | 3 |
| `P-d\MF\N` | 1 | C-6, C-7, C-8 | 9-6 / 9-99 | 3 |
| `P-d\MF1` | 1 | C-6, C-7, C-8 | 9-6 / 9-100 | 3 |
| `P-d\MF2` | 1 | C-6, C-8 | 9-6 / 9-100 | 2 |
| `P-d\MF3` | 1 | C-6, C-8 | 9-6 / 9-100 | 2 |
| `P-d\MF4` | 1 | C-6, C-7, C-8 | 9-6 / 9-100 | 3 |
| `P-d\MF5` | 1 | C-6, C-7, C-8 | 9-6 / 9-100 | 3 |
| `P-d\SYNC1` | 1 | C-6, C-7, C-8 | 9-6 / 9-101 | 3 |
| `P-d\SYNC2` | 1 | C-6 | 9-6 / 9-101 | 1 |
| `P-d\SYNC3` | 1 | C-6 | 9-6 / 9-101 | 1 |
| `P-d\SYNC4` | 1 | C-6 | 9-6 / 9-101 | 1 |
| `P-d\MFW1-n` | 2 | C-6, C-7 | 9-6 / 9-102 | 4 |
| `P-d\MFW2-n` | 2 | C-6, C-7 | 9-6 / 9-102 | 4 |
| `P-d\ISF\N` | 1 | C-6, C-7, C-8 | 9-6 / 9-102 | 3 |
| `P-d\ISF1-n` | 2 | C-6, C-7, C-8 | 9-6 / 9-102 | 3 |
| `P-d\ISF2-n` | 2 | C-6, C-7, C-8 | 9-6 / 9-102 | 3 |
| `P-d\IDC1-n` | 2 | C-6, C-7, C-8 | 9-6 / 9-103 | 3 |
| `P-d\IDC3-n` | 2 | C-7, C-8 | 9-6 / 9-103 | 3 |
| `P-d\IDC4-n` | 2 | C-7, C-8 | 9-6 / 9-103 | 3 |
| `P-d\IDC5-n` | 2 | C-7, C-8 | 9-6 / 9-103 | 3 |
| `P-d\IDC6-n` | 2 | C-7, C-8 | 9-6 / 9-103 | 3 |
| `P-d\IDC7-n` | 2 | C-7, C-8 | 9-6 / 9-104 | 3 |
| `P-d\IDC8-n` | 2 | C-7, C-8 | 9-6 / 9-104 | 3 |
| `P-d\IDC9-n` | 2 | C-7, C-8 | 9-6 / 9-104 | 3 |
| `P-d\IDC10-n` | 2 | C-7, C-8 | 9-6 / 9-104 | 3 |
| `P-d\AEF\N` | 1 | C-7 | 9-6 / 9-104 | 1 |
| `P-d\AEF\DLN-n` | 2 | C-7 | 9-6 / 9-104 | 1 |
| `P-d\AEF1-n` | 2 | C-7 | 9-6 / 9-105 | 1 |
| `P-d\AEF2-n` | 2 | C-7 | 9-6 / 9-105 | 1 |
| `P-d\AEF3-n-w` | 3 | C-7 | 9-6 / 9-105 | 1 |
| `D-x\DLN` | 1 | C-8, C-9, C-10, C-15, C-19, C-29, C-31 | 9-7 / 9-115 | 8 |
| `D-x\ML\N` | 1 | C-8, C-9, C-15, C-19 | 9-7 / 9-115 | 4 |
| `D-x\MLN-y` | 2 | C-8, C-9, C-10 | 9-7 / 9-115 | 3 |
| `D-x\MN\N-y` | 2 | C-8, C-9, C-10 | 9-7 / 9-115 | 3 |
| `D-x\MN-y-n` | 3 | C-8, C-9, C-10 | 9-7 / 9-115 | 7 |
| `D-x\MN3-y-n` | 3 | C-8, C-9 | 9-7 / 9-116 | 6 |
| `D-x\LT-y-n` | 3 | C-8, C-9, C-10 | 9-7 / 9-116 | 7 |
| `D-x\MML\N-y-n` | 3 | C-8, C-9, C-10 | 9-7 / 9-117 | 7 |
| `D-x\MNF\N-y-n-m` | 4 | C-8, C-9, C-10 | 9-7 / 9-117 | 13 |
| `D-x\WP-y-n-m-e` | 5 | C-8, C-9, C-10 | 9-7 / 9-117 | 14 |
| `D-x\WI-y-n-m-e` | 5 | C-8, C-9, C-10 | 9-7 / 9-117 | 14 |
| `D-x\FP-y-n-m-e` | 5 | C-8, C-9, C-10 | 9-7 / 9-117 | 14 |
| `D-x\FI-y-n-m-e` | 5 | C-8, C-9, C-10 | 9-7 / 9-117 | 14 |
| `D-x\WFM-y-n-m-e` | 5 | C-8, C-9, C-10 | 9-7 / 9-117 | 14 |
| `D-x\MN1-y-n` | 3 | C-8, C-9 | 9-7 / 9-115 | 5 |
| `D-x\MN2-y-n` | 3 | C-8, C-9 | 9-7 / 9-116 | 5 |
| `C-d\DCN` | 1 | C-10, C-11, C-16, C-17, C-24, C-25, C-26, C-30 | 9-11 / 9-158 | 14 |
| `C-d\MN1` | 1 | C-10, C-11 | 9-11 / 9-159 | 7 |
| `C-d\MN3` | 1 | C-10, C-11 | 9-11 / 9-159 | 4 |
| `C-d\BFM` | 1 | C-10, C-11 | 9-11 / 9-159 | 7 |
| `C-d\MOT1` | 1 | C-10, C-11 | 9-11 / 9-163 | 4 |
| `C-d\MOT2` | 1 | C-10, C-11 | 9-11 / 9-163 | 4 |
| `C-d\DCT` | 1 | C-10, C-11 | 9-11 / 9-164 | 7 |
| `C-d\PS\N` | 1 | C-10, C-11 | 9-11 / 9-164 | 4 |
| `C-d\PS1` | 1 | C-10, C-11 | 9-11 / 9-165 | 4 |
| `C-d\PS2` | 1 | C-10, C-11 | 9-11 / 9-165 | 4 |
| `C-d\PS3-n` | 2 | C-10, C-11 | 9-11 / 9-165 | 8 |
| `C-d\PS4-n` | 2 | C-10, C-11 | 9-11 / 9-165 | 8 |
| `C-d\CO\N` | 1 | C-11 | 9-11 / 9-165 | 1 |
| `C-d\CO1` | 1 | C-11 | 9-11 / 9-165 | 1 |
| `C-d\CO` | 1 | C-11 | 9-11 / 9-166 | 1 |
| `C-d\CO-n` | 2 | C-11 | 9-11 / 9-166 | 1 |

D-x\MML\N-y-n has three total indices; D-x\MNF\N-y-n-m has FOUR;
D-x\WP-y-n-m-e has FIVE. Do not repeat the prototype's or older drafts'
shortened fragment-counter arity. C CO (offset) and C CO-n are distinct
complete shapes; embedded keyword numerals and missing suffixes are not
interchangeable. Every future recognizer coverage check must name omitted
rows as unready coverage, not silently miss their suspect-terminator warnings.

## XML annotations and explicit inventory limitations

Appendix C-12–C-31 includes XML, not additional code-name-format records.
A separate annotation scan finds 334 closed XML comments and 97 normalized
pre-colon stems. Relative stems are context-dependent, not independently
valid full names; do not feed XML to the ASCII scanner. Most share suffixes
with the full-name rows above. Explicit anomalies/coverage boundaries:

- C-13 has M1\ID in an implicit annotation, not valid M-1\ID. Keep the
  source spelling visible; it does not add an accepted M1 group grammar.
- Relative ISF1, ISF2 and IDC1/3/4/5/6/7/8/9/10 annotations omit the indexed
  full-name suffix/context. Their source text cannot justify accepting bare
  P-d\ISF1 or P-d\IDC1 instead of reviewed -n shapes. Full indexed versions
  are in the minimum table. No contextual XML name reconstruction is claimed.
- Other XML annotations without ':' (for example MFW1-1/MFW2-1) or implicit
  values are not colon-lookahead candidates. Existing full shapes remain
  required from the code-name examples. Annotation counts are not runtime
  oracles or proof of conforming attributes.

There is no V/X/private vocabulary in this full-name set, nor every standard
group or the entire 0.1 metadata vocabulary. This is the required Appendix
minimum, not a complete 0.1 recognizer or registry. Standalone COMMENT and
other release-required names still need their separate reviewed definitions;
public pattern scope/grammar and INT-061/062/063 stay owner-gated.

## Independent acceptance plan and remaining gaps

Coverage remains UNIMPLEMENTED for all rows: no runtime matcher was tested.
When it exists, use this source-fixed CSV/pattern table as an independent
coverage oracle, not a list generated by the matcher itself. Enumerate every
literal spelling and its required full shape, plus cases with missing/extra
indices, trailing path material, ASCII mixed case, projected internal controls,
canonical-index variants and high-byte look-alikes. No prefix-only success.

Use the actual C-8/C-9 colon-ended sequences as NONCONFORMING raw snippets:
INT-035 partitions only at semicolons, so the later names may be inside a
value; recognition can support a located suggestion but cannot create lookup
items or apply the edit. S1 remains suspect pending authorized real-data
confirmation. The earlier extraction-sensitive count of 18 is not adopted
as a fixed expected number of warnings.

Positive named/negative near-miss fixtures and exact source correspondence
remain necessary. Skipping unterminated suffixes, respecting cut qualification,
bounded recognition work and preserving raw replay remain existing rules.
No actual matcher/source freshness/interpretation/semantic gate is waived.

Executed locally: source hash check, bounded text inventory and table-cell
inspection, CSV/pattern arity/count consistency and docs/trace checks. No
Rust build/test, XML parser, real recording, reference tool, dependency
installation or code-generation step. Source text/PDFs stay temporary; only
name metadata/citations are committed. This source inventory needs independent
exact-head review and is not permission to start parser implementation.
