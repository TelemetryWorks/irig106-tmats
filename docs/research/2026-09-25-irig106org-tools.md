# irig106.org tools: igDisplayTMATS, idmptmat, and irig106lib (2026-09-25)

Research record. It preserves, word for word, the material the project owner
supplied about the irig106.org tools, followed by what was verified against
the sources and what was found. Decisions drawn from it live in
`docs/USE-CASES.md` (UC-15, UC-16, UC-17), `docs/TEST-DATA.md` (reference
tools and defects D1–D7), `docs/ROADMAP.md`, `docs/RELEASING.md`, and
`docs/ARCHITECTURE.md`. Where this record and those documents differ, those
documents govern; this file is history, not specification.

## 1. The owner's question and pasted AI text (verbatim)

The owner wrote:

> Looking at this https://irig106.org/ and in particular this "igDisplayTMATS
> is a program to view TMATS from a Chatper 10 file or from a TMATS text file.
> TMATS is display in raw format, in as summary format, and in a hierachical
> tree. TMATS signatures can also be generated and checked." I think we should
> add these features in to our telementry works projects. Should this
> functionality be the goal for our projects test tool for the TMATS repo?
>
> AI Generated text when I asked about igDisplayTMATS (ignore the questions in
> the quoted text)
> "The program igDisplayTMATS is an open-source Qt-based GUI utility used to
> view, summarize, and verify Telemetry Attributes Transfer Standard (TMATS)
> data from IRIG 106 Chapter 10 telemetry files.Its open-source backend logic
> and primary source code are heavily associated with the main open-source
> IRIG 106 project repositories on GitHub:Core C Library Repository: The
> foundational code for parsing IRIG 106 records is available at the
> bbaggerman / irig106lib GitHub Repository.Alternative Telemetry
> Implementations: Related specialized parsing toolkits and extensions can be
> found on organizations like the Avionics Test and Analysis Corporation
> GitHub Organization.Features of igDisplayTMATSFormat Views: Displays TMATS
> setup data in raw text, summarized blocks, and interactive hierarchical
> trees.Signature Checking: Generates and validates cryptographic/hash
> signatures for TMATS data.Cross-Platform GUI: Built using the Qt framework
> to ensure high performance and seamless compatibility across Windows and
> Linux environments.Pre-compiled Windows executable bundles containing
> igDisplayTMATS alongside sister utilities (like igDmpHeaders and ig106Dub)
> are generally hosted on the official IRIG 106 Software Download
> Portal.Are you trying to compile the Qt source utilities yourself, or do you
> just need help parsing a specific Chapter 10 telemetry file?"
>
> I need the source code for the igDisplayTMATS so you think you can find it?

## 2. The owner's `CONVERSATION.md` (verbatim)

The file below was supplied as `CONVERSATION.md` in the repository root
(never committed) and is reproduced exactly, including its link-tracking
parameters and citation markers.

````markdown
# CONVERSATION

Here is a conversation I had with ChatGPT

irig106.org
Open Source Software Library and Utility Programs
An open source library and utility programs for writing, reading, and parsing IRIG106 format data files is under active development. The software library is written in portable C, and currently compiles into a DLL under various versions of Microsoft Visual Studio and as a library under GCC. A number of utility programs have also been written and more are on the way. Available software includes:
- i106lib - A software library for reading, writing, and parsing IRIG 106 data files
- i106stat - Generate a summary of data channels and message types
- i106trim - Trim a file based on start and stop time
- idmptmat - Extract and display TMATS in several formats
- idmp1553 - Extract and display 1553 messages
- idmpins - Extract, decode, and display 1553 INS messages. See the idmpins page for more details
- idmpuart - Extract and display UART messages
- igDmpHeaders - Browse and export Chapter 10 packet headers
- Display_TMATS - Extract, decode, and display TMATS from a recorded file
Compiled and ready to run versions of these utility programs are available from the Software Download page.
Source code is freely available for downloading. This projects are hosted on GitHub.
10/1/2018 - Once again the good folks at EMC have made available Version 3.9 of their Chapter 10 toolset including a validator and packet viewer for download.
I cant find the github repo

The repository is still there; the IRIG106 site just does a poor job of linking to it.
The main repository behind the i106lib material you quoted is:
bbaggerman/irig106lib on GitHub
It is Bob Baggerman's IRIG 106 library, and its README identifies it as the open-source library from IRIG106.org for reading and writing IRIG 106 Chapter 10 files. GitHub
The source tree contains the pieces you've been working around, including:
- irig106ch10 — Chapter 10 file/header handling
- i106_decode_tmats — TMATS parsing
- i106_decode_1553f1 — MIL-STD-1553
- i106_decode_ethernet — Ethernet packets
- i106_decode_video — video packets
- i106_decode_uart — UART
- i106_decode_arinc429
- i106_decode_can
- i106_index
- i106_data_stream — Chapter 10 over UDP
That last one is particularly relevant to the work we've been doing with your Chapter 10 Ethernet/UDP/RTPS data. GitHub
The IRIG106 download page confirms that the project moved from SourceForge to GitHub on November 28, 2018, and says the GitHub version contains the current source and complete history. IRIG 106
There is also a more modern fork worth looking at:
atac/libirig106
That project explicitly says it is forked from Bob Baggerman's irig106lib and describes itself as a cross-platform C library for Chapter 10/11 parsing and generation. GitHub
For what we're doing, I would actually examine both. The original bbaggerman/irig106lib is useful as the closest thing to a reference implementation associated with irig106.org, while atac/libirig106 may contain portability/maintenance improvements.
There is also a newer Rust ecosystem under TelemetryWorks with separate repositories for Chapter 10 reading, TMATS, decoding, indexing, writing, and CLI tooling. GitHub
Given your existing Rust Chapter 10 work, that is probably worth a much deeper comparison. I can go through bbaggerman/irig106lib, atac/libirig106, and the TelemetryWorks Rust implementation and map their packet/decode architecture against the Chapter 10 parser we've been designing.

Is there a idmptmat or igDisplayTMATS you can locate?

Yes — I found both, and also found the GitHub repo I missed before.
idmptmat is in the separate IRIG 106 utilities repository, not the irig106lib repository:
bbaggerman/irig106utils on GitHub
The IRIG106.org download page explicitly links that repository as the source for the utility programs. IRIG 106
idmptmat
The current IRIG106.org download page still provides the compiled Windows executable:
Download idmptmat.exe directly
It lists it as Version 1.03 and describes it as:
“Read and print out the TMATS record in various formats”

The documented modes are:

```
idmptmat <infile> <outfile> <flags>

-c    Output channel summary format (default)
-t    Output tree view format
-r    Output raw TMATS
``` :chatgpt-content-reference{index="3"}


Even better for what we're doing: the **complete `idmptmat` C source is published in Appendix C of the official IRIG 106 Chapter 10 Programmer's Handbook**. So even if the old project gets difficult to build, we have a reference implementation to work from. :chatgpt-content-reference{index="4"}

---

### `igDisplayTMATS`

This one is considerably newer.

IRIG106.org lists:

**igDisplayTMATS 3.1 — released October 11, 2022**

It is a Qt/C++ GUI that can read either:

- a Chapter 10 file
- a standalone TMATS text file

and display TMATS as:

- raw text
- summary
- hierarchical tree

It can also calculate the IRIG 106 Chapter 9 TMATS signature and validate an embedded signature. :chatgpt-content-reference{index="5"}

The standalone ZIP is still directly hosted here:

[Download igDisplayTMATS 3.1 ZIP](https://www.irig106.org/downloads/igDisplayTMATS-3.1.393798.zip?utm_source=chatgpt.com)

There is also an installer and password-protected ZIP on the official download page:

[IRIG106 Software Download page](https://www.irig106.org/wiki/software_download?utm_source=chatgpt.com)

The password-protected archives use `irig106` as the password. :chatgpt-content-reference{index="8"}

### The important distinction

There are really three related things:

| Component | Type | Purpose |
|---|---|---|
| `i106_decode_tmats` | C library code | Actual TMATS parser/data structures |
| `idmptmat` | CLI utility | Extract/dump TMATS from Chapter 10 |
| `igDisplayTMATS` | Qt GUI | Browse TMATS raw/summary/tree + signatures |

The `irig106lib` README confirms that its TMATS decoder consists of **four modules that decode a TMATS packet into a tree structure**. :chatgpt-content-reference{index="9"}

That is especially interesting for our work. Rather than treating `idmptmat` as just a utility to run, I think we should inspect its implementation alongside `i106_decode_tmats`. It gives us a fairly authoritative model for turning the TMATS text in your Chapter 10 files into structured objects.

For your parser, I would model a Rust equivalent roughly as:

```text
Chapter 10 packet
      │
      ▼
TMATS packet 0x01
      │
      ▼
raw TMATS text
      │
      ▼
TMATS lexer/parser
      │
      ├── G records
      ├── R records
      ├── M records
      ├── P records
      ├── B records
      ├── C records
      ├── D records
      └── ...
      │
      ▼
structured TMATS model
      │
      ├── raw view
      ├── tree view
      ├── channel mapping
      └── semantic lookup
```

And idmptmat is probably the best reference program to validate our Rust output against: run the same .ch10 file through both and compare the parsed channel hierarchy.
That could be particularly useful with the TMATS you've already shown me — CG01 All, the 24 channels, EI port definitions, etc.

````

## 3. What was verified (2026-09-25)

| Claim | Finding |
|---|---|
| `idmptmat` usage `idmptmat <infile> <outfile> <flags>` with `-c` channel summary (default), `-t` tree, `-r` raw | **Correct** (`irig106utils/src/idmptmat.c`, version 01.03). A fourth option, `-s` (signature), is compiled out under `#if 0` with the comment `// SIGNATURE GENERATION ISN'T CORRECT. FIX LATER.` |
| The complete `idmptmat` source is in "Appendix C" of the RCC Chapter 10 Programmer's Handbook | **Partly.** It is Appendix C ("Example Program – Decode TMATS") in RCC 123-16 and Appendix D in RCC 123-20; the 123-20 listing is headed "idmptmat - Read and dump a TMATS record from an IRIG 106 Ch 10 data file". RCC 123-20 Appendix A is a "Summary of Ch 9 TMATS Code Values Supported". |
| `idmptmat` is in `bbaggerman/irig106utils`, not `irig106lib` | **Correct.** |
| `irig106lib` has a TMATS decoder of "four modules that decode a TMATS packet into a tree structure" | **Correct** per its `readme.txt` ("i106_decode_tmats - Four modules for decoding a TMATS data packet into a tree structure for easy interpretation"); the source today has per-group files for B, C, D, G, M, P, and R. |
| `i106_data_stream` supports Chapter 10 over UDP | **Correct** (`readme.txt`: "Support for receiving Chapter 10 standard UDP data packets"). Relevant to `irig106-core`, not to this crate. |
| The project moved from SourceForge to GitHub on November 28, 2018 | **Consistent** with the irig106.org download page ("11/28/2018 - The source has been moved from SourceForge to GitHub"). The retired SourceForge SVN at `svn.code.sf.net/p/irig106/code` is still readable. |
| `atac/libirig106` is a fork of `irig106lib` | **Correct** per its README ("Forked from Bob Baggerman's `irig106lib`"); GitHub does not mark it as a fork and does not detect its license; last pushed 2022-02-15. |
| `igDisplayTMATS` 3.1, released October 11, 2022; Qt/C++; reads a Chapter 10 file or a standalone TMATS file; raw, summary, tree; calculates and validates the Chapter 9 signature | **Consistent** with the download page and the executable in `igDisplayTMATS-3.1.393798.zip` (dated 2022-10-11). |
| Password-protected archives use `irig106` | Stated on the irig106.org download page. |
| The GUI's source is open and on GitHub (pasted AI text) | **Not found.** `bbaggerman` publishes only `irig106lib` (BSD-3-Clause) and `irig106utils`; neither contains the Qt GUIs; GitHub code search finds nothing; the SourceForge SVN has only `irig106lib` and `irig106utils`; the zip holds only `igDisplayTMATS.exe`, Qt DLLs, and plugins. The only route to the source is to ask its author (contact details on irig106.org). |
| "Cross-platform ... Windows and Linux environments" (pasted AI text) | Only Windows builds are published. |
| "cryptographic/hash signatures" (pasted AI text) | **Wrong.** Neither signature uses a key. See section 4. |
| Other utilities listed on irig106.org | `i106lib`, `i106stat` (channel and message summary), `i106trim` (trim by time), `idmptmat`, `idmp1553`, `idmpins` (1553 INS messages), `idmpuart`, `igDmpHeaders` (browse and export packet headers), `Display_TMATS`. irig106.org also announced (2018-10-01) EMC's Chapter 10 toolset 3.9 with a validator and packet viewer — another possible reference tool. |

The conversation's suggestion to compare `bbaggerman/irig106lib`,
`atac/libirig106`, and the TelemetryWorks crates' packet and decode
architecture is recorded as a possible ecosystem study; for this crate the
comparison that matters is the TMATS one in `docs/TEST-DATA.md`.

The conversation's pipeline sketch is discussed in `docs/ARCHITECTURE.md`
section 1: its overall flow is sound, its one-record-type-per-group model is
not adopted.

## 4. What "TMATS signatures" are

Strings in `igDisplayTMATS.exe` name two signatures: "Calculated IRIG 106
Chapter 9 Standard Signature" and "Calculated Irig106.org Flex Signature"
(with "No signature found in TMATS", "Calculated signature matches signature
in TMATS", and "Calculated signature differs from signature in TMATS").

1. **The standard one is `G\SHA`.** Chapter 9 Table 9-2 (106-23), parameter
   "MESSAGE DIGEST/CHECKSUM", code name `G\SHA`, "TMATS Checksum": "Provide a
   message digest / checksum of the TMATS. The entire contents of the TMATS
   file except the characters from "G\SHA:" to the following ";" (inclusive)
   shall be used to calculate the checksum. The value integer is an algorithm
   designator and the hex digits are the checksum. SHA2-256 shall be
   represented as "2-" followed by 64 hex characters. See Subsection 6.2.3.11.f
   for more information." Range: "integer followed by "-" followed by hex
   characters". Present in Chapter 9 of 106-15, 17, 19, 20, 22, 23, and 24R1;
   absent from 106-09, 11, and 13.

   Chapter 6 §6.2.3.11 f (106-23), the recorder `.TMATS CHECKSUM [n]`
   command: "returns a message digest of the entire specified or default (0)
   TMATS record excluding only the G\SHA code name, if present. The message
   digest shall be calculated in accordance with (IAW) Federal Information
   Processing Standards Publication 180-41, algorithm "SHA-256." The message
   digest is a string of 64 lower-case hexadecimal characters, prefixed with
   the constant string "2-" to designate the algorithm. If the TMATS includes
   a G\SHA code name, all text between the "G\SHA" and the following
   semicolon, inclusive, shall be discarded for the purposes of digest
   calculation." (The text reads "180-41"; FIPS PUB 180-4 is the Secure Hash
   Standard, so the trailing "1" is most likely a footnote marker run into the
   number by text extraction.)

   Searches for the word "signature" in Chapter 9 (106-09 … 106-23) and in
   RCC 123/124 find nothing, because the standard calls it a checksum.
2. **The "flex signature" is irig106.org's own, not IRIG 106.**
   `irig106lib`'s `enI106_Tmats_Signature` computes a Fletcher-32 checksum of
   each upper-cased `code:value;` line and **sums** them (so line order does
   not matter). By default it skips `COMMENT` lines and `\COM` items, the whole
   G group, the whole V group, and these R-group items: `RI1`–`RI10`, `DPOC1`,
   `DPOC2`, `DPOC3`, `MPOC1`–`MPOC4` (with `MPOC4` listed twice and no
   `DPOC4`), `RIM\N`, `RIMI-`, `RIMS-`, `RIMF-`, `RMM\N`, `RMMID-`, `RMMS-`,
   `RMMF-`. Flags (`TMATS_SIGFLAG_INC_ALL` 0x1, `_INC_COMMENT` 0x2,
   `_INC_VENDOR` 0x4, `_INC_G` 0x8) include those classes; the opcode is
   `((flags & 0xF) << 4) | (version & 0xF)` with version 1, printed by
   `idmptmat` as `%2.2X-%8.8X`. The executable's help text describes the
   format as "'V' identifies the TMATS signature algorithm version", "'FV' is
   OpCode in hexidecimal. OpCode defines how the signature was calculated.",
   and "'SSSSSSSS' is signature value in hexidecimal."

Neither is a cryptographic signature: there is no key, so both detect
accidental change, not tampering.

## 5. Defects found while reading the source

Recorded, with the regression test that guards `tmats` against each, as D1–D7
in `docs/TEST-DATA.md`. In brief: `idmptmat` loops forever on two or more G
data sources (D1), reads only the first packet (D2), has signature output
compiled out as incorrect (D3), and mislabels `CDT` values as `DST` in its
tree view (D4); `irig106lib`'s SHA-256 routine hashes the wrong bytes when
`G\SHA` is the last item (D5) and matches the text `G\SHA` inside values
(D6); its flex exclusion list has the `MPOC4`/`DPOC4` slip (D7).

`irig106lib` also comments that `R-x\DST-n` ("Data Source Type") existed only
in 106-04 and was replaced by `R-x\CDT-n` ("Channel Data Type"); to be
confirmed against the archived 106-04/05 Chapter 9 (ROADMAP 0.4).

## 6. Program data mentioned in the conversation

The conversation refers to the owner's own program TMATS ("CG01 All, the 24
channels, EI port definitions, etc."). Such files are used locally only
(`docs/TEST-DATA.md`); anything they reveal is reproduced as a synthesized
fixture.
