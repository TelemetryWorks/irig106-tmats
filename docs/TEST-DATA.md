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

Planned use: differential tests that compare this library's reading of each
sample file's TMATS with `idmptmat`'s output, recording the exact upstream
commit or version compared against. A disagreement is investigated against
Chapter 9, not assumed to be our bug or theirs.

Open question: `igDisplayTMATS` describes computing and checking an "IRIG 106
Ch 9 signature" for TMATS. The term does not appear in Chapter 9 of 106-09
through 106-23 or in RCC 123/124; its definition is to be traced in the
`irig106lib` / `irig106utils` source before it is treated as a requirement.

## Standards

The RCC 106 standards and handbooks are mirrored, with original URLs and
SHA-256, in <https://github.com/TelemetryWorks/rcc-106-standards>
(`python scripts/fetch.py --edition 106-24R1 --chapter 9`). Standards from
other bodies that IRIG 106 references — MIL-STD-1553B, ARINC 429, IEEE 1588,
IEEE 754 — are cited, not mirrored.
