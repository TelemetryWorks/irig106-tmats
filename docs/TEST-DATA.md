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
- **CI uses synthesized fixtures**: small TMATS documents written for a
  specific behaviour, each citing the Chapter 9 clause it exercises, plus
  simulated variants of the conditions observed in real files (vendor quirks,
  non-conforming attributes, odd bytes), and fuzzing.
- **When a real file reveals a behaviour**, it is reproduced as a synthesized
  fixture in the same change, so CI keeps the lesson without the file.

## Real Chapter 10 recordings (local use only)

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

## Standards

The RCC 106 standards and handbooks are mirrored, with original URLs and
SHA-256, in <https://github.com/TelemetryWorks/rcc-106-standards>
(`python scripts/fetch.py --edition 106-24R1 --chapter 9`). Standards from
other bodies that IRIG 106 references — MIL-STD-1553B, ARINC 429, IEEE 1588,
IEEE 754 — are cited, not mirrored.
