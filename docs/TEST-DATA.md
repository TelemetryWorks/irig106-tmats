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
  "106-22 or later".
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
