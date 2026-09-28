# Test Data and Test Oracles

Where real TMATS and Chapter 10 data comes from, how it is used, and what
reference tools we compare against. This project does **not** host or manage
a data set: real files are downloaded to a developer's machine and used
locally; CI runs on synthesized examples and fuzzing only.

## Policy

- **Real recordings never enter this repository or CI.** They are fetched by
  a developer into a local directory outside the repository. Tests that need
  them are opt-in and skip cleanly when the directory is absent (the exact
  mechanism is decided with the test architecture in the design phase).
- **Spec fixtures**: the Chapter 9 Appendix 9-C format example and every
  expression and TMATS example in Appendix 9-E (§E.6.b, §E.9.a–d, both
  styles), taken verbatim from the archived standard with their citations.
- **CI uses synthesized fixtures**: small TMATS documents written for a
  specific behaviour, each citing the Chapter 9 clause it exercises, plus
  simulated variants of the conditions observed in real files (vendor quirks,
  non-conforming attributes, odd bytes), and fuzzing.
- **When a real file reveals a behaviour**, it is reproduced as a synthesized
  fixture in the same change, so CI keeps the lesson without the file.

## Real Chapter 10 recordings (local use only)

Order of work (owner decision, 2026-09-25): the irig106.org vendor sample
recordings first, because they cover the widest spread of vendor quirks,
then the owner's program files as they become available.

- **IRIG 106 sample data files** — laboratory recordings from recorder vendors,
  each with a TMATS setup record:
  <https://www.irig106.org/wiki/sample_data_files>.
  The site warns that they "provide examples of varying degrees of Chapter 10
  compliance", which makes them useful for non-conforming-input handling.
  Listed at the time of writing: Calculex (PCM at several rates, video and
  voice), Enertec (video and 1553), Heim (PCM/video/analog/UART, 16PP194 bus,
  CAN bus, Chapter 7 PCM), JDA Systems, NetAcquire, Smartronix (PCM with 1553,
  HD and SD video), Sypris, TTC (eight 1553 channels, ARINC 429), Wideband
  Systems (Ethernet, UART, video, PCM, 1553, ARINC 429, discrete), Wyle, and
  files synthesized with Data Bus Tools' FLIDAS.
- **IRIG 106 synthetic data file** — a fully synthesized flight recording
  (valid TMATS, time, video, and a 1553 navigation channel, about 150 MB) with
  an interface control document:
  <https://www.irig106.org/wiki/synthetic_data_files>.
- Some downloads require an irig106.org account.
- **Program recordings** supplied by the project owner, as they become
  available, under the same local-only rule.

## Test oracles (reference implementations)

- **`irig106lib`** — the long-standing C library for reading, writing, and
  parsing IRIG 106 data, including TMATS (BSD-3-Clause):
  <https://github.com/bbaggerman/irig106lib>.
- **`irig106utils`** — command-line utilities built on it, including
  `idmptmat` ("Read and print out the TMATS record in various formats"):
  <https://github.com/bbaggerman/irig106utils>.
- **irig106.org software downloads** (Windows builds, including
  `igDisplayTMATS`): <https://www.irig106.org/wiki/software_download>.

- **`idmptmat` in the standard itself** — the complete `idmptmat` source is
  published in the RCC 123 Chapter 10 Programmers' Handbook (Appendix C,
  "Example Program – Decode TMATS", in 123-16; Appendix D in 123-20), mirrored
  in `rcc-106-standards`.
- **`libirig106`** — a later C library whose README says it is "Forked from
  Bob Baggerman's `irig106lib`" (Chapter 10/11 parsing and generation; last
  pushed 2022-02): <https://github.com/atac/libirig106>. GitHub does not
  detect its license; confirm the license file before relying on it.
- **`igDisplayTMATS` 3.1** (Qt GUI, October 2022) — raw, summary, and tree
  views plus signature checks. Only Windows binaries are published; its
  source is not in `irig106lib`, `irig106utils`, or the retired SourceForge
  repository.

Planned use: differential tests that compare this library's reading of each
sample file's TMATS with `idmptmat`'s output, recording the exact upstream
commit or version compared against. Comparison is semantic (channels, types,
data sources, hierarchy), not textual. A disagreement is investigated against
Chapter 9, not assumed to be our bug or theirs.

## Known defects in the reference tools, and the tests that guard `tmats`

Every defect found in a reference tool becomes a named regression test in
this project, built from a synthesized fixture, so the same mistake cannot
reach `tmats` (UC-17). Found so far by reading the source (upstream
`master`, 2026-09-25):

| # | Tool | Defect | Guarding test for `tmats` |
|---|------|--------|---------------------------|
| D1 | `idmptmat` (`vDumpChannel`, `vDumpTree`) | The loop over G data sources advances with `psuFirstGDataSource->psuNext` instead of the current node's `psuNext`, so any TMATS with **two or more** `G\DSI-n` repeats the second data source forever (the program never ends). | Summary and tree output for documents with 1, 2, 3, and many data sources terminate and list each source exactly once, in order. |
| D2 | `idmptmat` | Reads only the **first packet** of a Chapter 10 file; later setup records (configuration changes) are never shown, and standalone TMATS files are not accepted. | A recording with several setup records reports each one; a plain TMATS text file is accepted. |
| D3 | `idmptmat` | Its signature option is compiled out with the comment `// SIGNATURE GENERATION ISN'T CORRECT. FIX LATER.` | `G\SHA` compute/verify tested against independently computed SHA-256 vectors (UC-15), not against another tool's output. |
| D4 | `idmptmat` tree view | Labels the channel type `(R-x\DST-n)` even when the value came from `R-x\CDT-n` (`irig106lib` stores both in one field). | Every displayed value is labelled with the code name it was actually read from. |
| D5 | `irig106lib` `enI106_Tmats_IRIG_Signature` | When `G\SHA` is the **last** item, it hashes `length − start` bytes from the beginning instead of the `start` bytes before `G\SHA`, producing a wrong digest. | `G\SHA` placed first, in the middle, last, and absent all give the digest defined by Chapter 6 §6.2.3.11 f. |
| D6 | `irig106lib` `enI106_Tmats_IRIG_Signature` | Finds `G\SHA` by case-insensitive substring search over the whole text, so the characters `G\SHA` inside another item's value (for example a comment) are treated as the checksum item. | Only a real `G\SHA` code name is excluded; the same text inside a value is hashed. |
| D7 | `irig106lib` flex signature | The R-group exclusion list names `MPOC4` twice and omits `DPOC4`, although `R-x\DPOC4` exists in 106-23 Chapter 9 — almost certainly a typo, so `DPOC4` changes the flex signature while `DPOC1`–`DPOC3` do not. | Our flex signature reproduces `irig106lib`'s behaviour exactly, because its only purpose is compatibility; the test pins the quirk with a `DPOC4` fixture and says so, rather than fixing it silently. |

Open items: `irig106lib` comments that `R-x\DST-n` existed only in 106-04 and
was replaced by `R-x\CDT-n`; confirm against the 106-04 and 106-05 Chapter 9
in the archive and record it in the edition deltas.

## Required acceptance tests

Tests that must exist, by name, before the requirement they verify can be
shown as Implemented in `docs/TRACE-MATRIX.md`. They are specified here
during the design phase and written with the code they test; no placeholder
or ignored test is added in the meantime, because a marker on it would make
the trace matrix report the requirement as verified.

### `multi_packet_setup_record_is_assembled`

Verifies that one setup record spanning several consecutive packets is
assembled into one complete record ("A single setup record may span multiple
consecutive packets", Chapter 11 §11.2.7.2, 106-24R1). Requirements:
L1-CH10-001, L1-CH10-004, L1-CH10-006, L1-CLI-003, L1-CLI-008, L1-SUM-001.
Pictured in
`docs/diagrams/setup-record-assembly.svg`.

*Input* — a synthesized recording, in file order:

1. A PCM packet on channel 3.
2. Data type `0x01`, channel `0x0000`, sequence number `0xFE`, no secondary
   header, CSDW with FRMT = 0 (ASCII), SRCC = 0, RCCVER = `0x0E`; TMATS text
   fragment 1 is the first part of a document that includes a correct `G\SHA`
   for the whole document, split in the middle of an attribute.
3. Data type `0x01`, channel `0x0000`, sequence number `0xFF`, **with** a
   secondary header (packet-flags bit 7 = 1), the same CSDW, fragment 2, 3
   bytes of `0x00` filler, and a 16-bit data checksum (flags bits 1–0 = 10).
4. Data type `0x01`, channel `0x0000`, sequence number `0x00` (rollover),
   the same CSDW, fragment 3, `0xFF` filler, and an 8-bit data checksum.
5. A PCM packet on channel 3.

Every header and secondary-header checksum is correct.

*Expected*:

- Exactly one complete setup record, whose TMATS body is byte-for-byte
  fragment 1 + fragment 2 + fragment 3 — no header, secondary header, CSDW,
  filler, or checksum bytes.
- One CSDW summary: ASCII, no configuration change, RCCVER `0x0E` read as
  "RCC 106-22" with the note "unchanged through 106-24R1" (ADR-0032).
- The provenance map sends a body offset inside fragment 2 to packet 3 and
  the right offset within it, and an attribute split across fragments 1 and 2
  to both packets.
- The document parses without diagnostics; the attribute split across the
  packet boundary is one attribute.
- `G\SHA` verifies as a match over the assembled body.
- `tmats` reports one setup record, not three.

*Companion cases* (same fixture builder): the same record with a sequence gap
(`0xFE`, `0x00`) is reported as two incomplete records with a diagnostic;
fragment 2 with a different RCCVER ends the record and is reported; a PCM
packet between fragments 1 and 2 ends the record; a corrupted header
checksum on fragment 2 is reported and the record is not assembled from
untrusted lengths.

### `appendix_9c_example_reports_suspected_semicolons`

Verifies that the standard's own code-name example (106-24R1 Appendix 9-C) is
read without loss and that its 18 colon-for-semicolon errata are each
reported. Requirements: L1-READ-001, L1-READ-006, L1-READ-007. **Suspect**
(owner, 2026-09-26): the finding and the diagnostic are held in doubt until
checked against real recordings (`docs/ROADMAP.md`, S1).

*Input* — the Appendix 9-C code-name example transcribed verbatim from the
archived 106-24R1 Chapter 9 (Distribution A), including page C-8.

*Expected*:

- Exactly 18 "possible `;` typed as `:`" warnings, one at each of these
  attributes, each with a suggested edit replacing the colon after the value
  by `;`:
  `D-1\MML\N-1-1`, `D-1\MNF\N-1-1-1`, `D-1\MNF\N-1-1-2`, `D-1\MML\N-1-2`,
  `D-1\MNF\N-1-2-1`, `D-1\MML\N-1-3`, `D-1\MNF\N-1-3-1` to
  `D-1\MNF\N-1-3-6`, `D-1\MML\N-1-4`, `D-1\MNF\N-1-4-1`, `D-2\MML\N-1-1`,
  `D-2\MNF\N-1-1-1`, `D-2\MML\N-1-2`, `D-2\MNF\N-1-2-1`.
- No other reading diagnostic; D-3 and D-4 (correctly delimited) produce
  none.
- Writing the document back reproduces the input bytes exactly.
- Applying all 18 suggestions yields a document in which each of those
  counters is a separate attribute with a numeric value.

*Companion cases*: `C-1\DPA:A?B:C;` and the Appendix 9-E expressions
produce no such warning; a value ending `: G\COM:x;` is reported (the G group
has no occurrence index).

### `appendix_9c_channel_2_trace`

Verifies the path of `docs/TMATS-IN-CHAPTER-10.md` section 7: from channel
ID 2 of the standard's own example to measurement 82AJ01 in engineering
units, with every state. Requirements: L1-VIEW-002, L1-VIEW-004,
L1-VIEW-006, L1-VAL-005, L1-EDN-002, L1-SUM-002.

*Input* — the Appendix 9-C code-name example transcribed verbatim from the
archived 106-24R1 Chapter 9 (the same fixture as
`appendix_9c_example_reports_suspected_semicolons`).

*Expected*:

- The channel view for channel ID 2 gives `R-1\CDT-1` PCMIN, `R-1\DSI-1`,
  and `R-1\CDLN-1` `PCM1` as explicit; `R-1\CHE-1`, `R-1\PDTF-1`, and
  `R-1\PDP-1` as missing, each with a required-attribute finding.
- The format link resolves to `P-2` only; the view gives exactly the values
  of section 7.1 (bit rate 2,000,000, 10-bit words with words 121 and 122 of
  6 and 4 bits, 64 × 277-word minor frames, the 30-bit pattern, the ID
  counter).
- The measurement link resolves to `D-3`; 82AJ01 has one location with two
  fragments at words 113 and 121, frame 5, frame interval 32, full words;
  both transfer orders defaulted to msb first; both positions defaulted to
  1, with a finding and an ambiguous fragment order (INT-033).
- The conversion link resolves to `C-7`, with every value explicit.
- `G\106` missing is reported; the validation basis is a fallback to
  106-24R1, labelled; `G\SHA` is absent (not an error).
- No channel has `R-x\CDT-n` TIMEIN.

### `setup_record_completeness_outcomes`

Verifies the three outcomes of ADR-0031 and INT-034, one case each, all on
channel `0x0000` with an ASCII CSDW. Requirements: L1-CH10-004,
L1-CH10-008.

| Case | Input | Expected |
|------|-------|----------|
| a | a record in two fragments ending with `;`, then a time packet | complete, "followed by another packet" |
| b | the same record as the last packets of the input | complete, "end of input" (labelled) |
| c | the same record with its last fragment cut inside an attribute, at the end of input | incomplete; no description |
| d | three fragments with the middle one missing (sequence jumps by 2) and an attribute split across the gap | incomplete; no description |
| e | two complete records back to back, sequence continuous, same CSDW, the second starting again with `G\106` | ambiguous; the split point and both readings reported; neither governs by default |
| f | a sequence gap where the text before it ends with `;` | ambiguous |

### `srcc_compares_configuration_not_bytes`

Verifies INT-029's comparison. Each case is a governing record followed by
a second complete record. Requirements: L1-CH10-008, L1-READ-004.

| Case | Second record | Expected |
|------|---------------|----------|
| a | the same attributes in another order, SRCC 0 | same configuration; information only |
| b | the same attributes with keywords in lower case, SRCC 0 | same configuration; information only |
| c | one value changed, SRCC 0 | changed configuration; unannounced-change finding |
| d | identical bytes, SRCC 1 | identical; change-bit-with-nothing-changed finding |
| e | reordered only, SRCC 1 | same configuration; change-bit-with-nothing-changed finding |
| f | one malformed attribute added, SRCC 0 | not comparable; textual differences; no SRCC finding |

## Known defects in the prototype, and the tests that guard the rebuild

The team's spec alignment review (`docs/research/2026-09-27-spec-alignment-review.md`)
reproduced five defects in the prototype (tag `prototype-0`) against
106-24R1. Each is known from the 2026-09-25 prototype review and becomes a
regression test for the rebuilt library, as D1–D7 do for the reference
tools. They describe the prototype, not the design.

| # | Prototype defect | Standard | Regression test |
|---|------------------|----------|-----------------|
| P1 | `P-d\D1` read as bit rate, `D2` as encoding, `F1`–`F3` misread; D1 and F2 dropped when serialized | Table 9-6: D1 PCM code, D2 bit rate, F1 common word length, F2 word transfer order, F3 parity | `pcm_d_and_f_attributes_keep_their_table_9_6_meanings` (L1-READ-001, L1-REG-001) |
| P2 | `G\DST-1:STO` rejected as an invalid keyword | Table 9-2, `G\DST-n`: "STO Storage" | `g_dst_accepts_every_table_9_2_keyword` (L1-REG-001, L1-VAL-001) |
| P3 | CSDW bit 9 treated as reserved; `[12, 2, 0, 0]` re-encoded as `[12, 0, 0, 0]` | Figure 11-34: "FRMT. Bit 9 is the setup record format"; bits 31–10 reserved | `setup_record_format_bit_survives_decode_and_encode` (L1-CH10-002, L1-CH10-003) |
| P4 | RCCVER `0x0B` with `G\106:17` reported as source version 106-15 | ADR-0028: two declarations, neither derived from the other | `rccver_does_not_replace_g106` (L1-EDN-005) |
| P5 | Lenient reading of `G\PN:TEST;G\106:17;BROKEN` returned an empty document with no diagnostics | L1-READ-001, L1-READ-003 (preservation) | `lenient_reading_keeps_attributes_before_a_malformed_tail` (L1-READ-001, L1-READ-003) |

## Errata in the standard's own examples

Found while checking the design against the archived standard. Each is kept
verbatim in fixtures and handled by a diagnostic, never corrected silently.
Each also has an entry in the interpretation register
(`docs/INTERPRETATIONS.md`), which holds every interpretation, not only
errata.

| # | Where | Erratum | Handling |
|---|-------|---------|----------|
| E1 (INT-013) | Chapter 9 Appendix 9-C, page C-8 (106-24R1); the same 18 in 106-17, 106-19, 106-20, 106-22, 106-23, 106-24 | 18 attributes after D-group counters end with `:` instead of `;`, for example `D-1\MML\N-1-1:2: D-1\MNF\N-1-1-1:1: D-1\WP-1-1-1-1:14;` | L1-READ-007; `appendix_9c_example_reports_suspected_semicolons`. **Suspect** (ROADMAP S1). |
| E3 (INT-023) | Chapter 6 §6.2.3.11, `.TMATS WRITE` and `.TMATS READ` examples (106-24R1) | `G\DSI\N=18;` — `=` where §9.4.2 requires `:` | Read as a missing delimiter and kept exactly; whether to suggest `:` is open (follow-up F3). |
| E4 | Chapter 6 §6.2.3.11, example setup file (106-24R1) | `G\SHA:0;` — not "integer followed by "-" followed by hex characters" (Table 9-2) | Verification reports malformed (L1-SUM-002). |
| E2 (INT-010) | Chapter 9 Appendix 9-E, Table E-3 | The equality operator printed `= =`; the grammar gives `==` | ARCHITECTURE section 5.2 (T2): `==` is the operator, `= =` accepted with a warning. |

## Checks to run against real data

The owner's doubts to settle when the local sample recordings are read
(`docs/ROADMAP.md`, "Suspect findings to confirm against real data"):

- **S1** — count every "possible `;` typed as `:`" warning and every value
  containing `: ` followed by a code name across all samples; review each
  one; record whether real TMATS writers produce the pattern, whether any
  legitimate value triggers it, and whether a warning is the right default.
  Record the result in `docs/research/` and update the ROADMAP entry.

## Standards

The RCC 106 standards and handbooks are mirrored, with original URLs and
SHA-256, in <https://github.com/TelemetryWorks/rcc-106-standards>
(`python scripts/fetch.py --edition 106-24R1 --chapter 9`). Standards from
other bodies that IRIG 106 references — MIL-STD-1553B, ARINC 429, IEEE 1588,
IEEE 754 — are cited, not mirrored.
