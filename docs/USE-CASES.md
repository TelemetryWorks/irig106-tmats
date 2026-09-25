# irig106-tmats — Use Cases and Concept of Operations

> **Status: draft for review.** This is the first document of the design phase
> (see `docs/ROADMAP.md`). It says *who* uses the library, *for what*, and
> *where the library's responsibility ends*. Architecture, ADRs, and the
> L1/L2/L3 requirements are derived from it and come after it.
>
> Use-case identifiers (`UC-NN`) are stable labels for discussion and for the
> L1 requirements' rationale. They are not requirement IDs.

All Chapter 9 references are to RCC IRIG 106-23 Chapter 9 (July 2023) unless
stated otherwise; 106-24 changed only the CRC parameter in Table 9-6, and
106-24R1 did not change Chapter 9. Both are to be re-confirmed against the
`TelemetryWorks/rcc-106-standards` archive.

## 1. What TMATS is, in one paragraph

TMATS is the text description of a test's telemetry: where the data comes from
(data sources, §9.5.2), how it is transmitted (T, §9.5.3) or recorded (R,
§9.5.4), how it is multiplexed (M, §9.5.5), how PCM frames (P, §9.5.6), PCM
measurements (D, §9.5.7), buses (B, §9.5.8), message streams (S, §9.5.9), and
message structures (Q, §9.5.10) are laid out, how raw values become
engineering units (C, §9.5.11), plus airborne hardware (H, §9.5.12), vendor
attributes (V, §9.5.13), and extensions (X, §9.5.14). Every item is written as
`CODE\NAME-i-j:value;` (§9.4.2). Groups link to each other *by value*: a data
source ID ties G to T/R, a data link name ties R, M, and P to P/B/S/Q/D, and a
measurement name ties R/M/D/B/S to C (§9.5.1 b, Figure 9-1). A Chapter 10
recording carries its TMATS as the setup record (Computer-Generated Data
Format 1, data type `0x01`) at the start of the file.

## 2. Actors

| Actor | Who | What they want from TMATS |
|-------|-----|---------------------------|
| **Recording reader** | Software that opens a Chapter 10 file: `irig106-ch10-reader`, `irig106-studio`, range processing pipelines | Channel labels and data types, data-source grouping, the IRIG 106 edition, and a clear report when TMATS is missing or malformed |
| **Data decoder** | `irig106-decode` and downstream analysis tools | For a channel: its format definition (PCM frame, bus, message), its measurements, and each measurement's conversion to engineering units |
| **Recorder setup author** | Engineers and tools preparing a recorder; `irig106-write` when it emits a setup record | Build or edit TMATS, check it against the Chapter 10 recorder rules, and produce the setup-record payload |
| **TMATS author / editor** | Instrumentation engineers and TMATS editing tools | Open any TMATS file, change it without disturbing anything they did not touch, and see precise diagnostics |
| **Validator / QA** | Range acceptance, data-quality checks, CI for instrumentation configurations | Validate against a named IRIG 106 edition, with a policy they control, and a stable, machine-readable report |
| **Extender** | Programs, vendors, and ranges whose TMATS uses vendor (V) or extension (X) attributes, private conventions, or deliberately non-conforming content | Teach the library their attributes and rules, relax or tighten checks, and never lose their content |
| **Archivist / analyst** | Data centers comparing configurations across revisions, flights, or mid-recording setup changes | Compare two TMATS documents meaningfully and trace which revision applies to which data |

## 3. Inputs and outputs at the boundary

**Inputs the library accepts**

- Code-name (ASCII) TMATS bytes: a standalone file, or the text body of a
  setup record. Real files may contain non-ASCII bytes (for example Latin-1 °),
  CR/LF or bare LF line breaks, blanks around delimiters, and comments
  (`COMMENT:` and `\COM` items, §9.4.2).
- A Chapter 10 setup-record **payload** (the bytes after the packet header):
  the channel-specific data word (CSDW) followed by the TMATS body. The CSDW
  carries the Chapter 10 version and a setup-record-configuration-change flag,
  and in later editions a flag saying whether the body is code-name or XML
  TMATS. (Exact bit positions to be confirmed from Chapter 10/11 in the
  archive and recorded in `irig106-types`.)

**Outputs**

- A document that holds every attribute exactly as read, in order, with its
  source location.
- Structured views over it (groups, channels, links) for callers who do not
  want to handle code names.
- Diagnostics: parse problems, validation findings, and suggested edits.
- Serialized TMATS: byte-faithful by default, normalized on request.
- A setup-record payload for writing into a Chapter 10 file.

**Outside the library's responsibility**

- Finding packets in a Chapter 10 file, reading packet headers, checksums,
  and multi-packet reassembly (`irig106-core`, `irig106-ch10-reader`).
- Decoding telemetry data itself (`irig106-decode`), time correlation
  (`irig106-time`), and writing packets (`irig106-write`).
- XML TMATS, DDML, and IHAL (§9.4.3, §9.6, §9.7) — deferred; the library
  recognizes an XML setup record and reports it as unsupported.
- Rendering diagnostics for a terminal or UI.

## 4. Use cases

Each use case lists the release that delivers it (see `docs/ROADMAP.md`).

### UC-01 Read TMATS without losing anything — *0.1*

**Actor:** every actor. **Trigger:** a caller has TMATS bytes.

1. The caller passes the bytes (or a setup-record payload, UC-02).
2. The library splits them into attributes, parses each code name into group,
   occurrence index, and path segments with all their indices (for example
   `D-1\MN-1-2`), and records each attribute's byte span.
3. Problems — a missing `:` or `;`, an unparseable code name, a duplicate
   single-entry attribute, non-ASCII bytes — become diagnostics attached to
   the affected attribute. Reading continues with the next attribute.

**Outcome:** a document holding every attribute in input order, including
unknown, vendor, extension, duplicate, and malformed ones, plus diagnostics.
Nothing is dropped, and an empty document is never returned in place of
diagnostics.

**Notes:** TMATS is case-insensitive and attributes may appear in any order
(§9.4.2); the document answers lookups case-insensitively but keeps the
original spelling.

### UC-02 Read TMATS from a Chapter 10 setup record — *0.1*

**Actor:** recording reader. **Trigger:** `irig106-core` or a reader has
extracted the payload of a data type `0x01` packet.

1. The caller passes the payload.
2. The library decodes the CSDW (using the layout from `irig106-types`),
   reports the Chapter 10 version, the configuration-change flag, and the
   body format.
3. If the body is code-name TMATS, it is read as in UC-01. If it is XML, the
   library returns the CSDW information and an "XML not supported" result.

**Outcome:** CSDW facts plus the document. A recording can contain more than
one setup record (a configuration change during recording); each is read
independently, and UC-11 compares them.

### UC-03 Find attributes by code name — *0.1*

**Actor:** every actor. **Trigger:** the caller knows the code name it wants.

The caller asks for `G\106`, `R-1\TK1-3`, or a pattern such as "every
`R-1\TK1-n`". Lookup is case-insensitive and does not depend on attribute
order. The answer is the original value text and its location; typed
interpretation is UC-05.

### UC-04 Determine the IRIG 106 edition — *0.1*

**Actor:** validator, recording reader. **Trigger:** any document.

The edition is taken from, in order: a caller override; `G\106` ("last 2
digits of the year", Table 9-2); the Chapter 10 version in the setup-record
CSDW. The answer states which source was used and whether the sources
disagree. Unknown or future editions are reported as such, never guessed.

### UC-05 See channels and their definitions — *0.2*

**Actor:** recording reader, data decoder. **Trigger:** the caller needs to
interpret channel `N` of a recording.

1. The library finds the R-group channel whose channel ID (`R-x\TK1-n`)
   equals `N`, with its data type (`R-x\CDT-n`), data source, enabled flag,
   and data link name (`R-x\CDLN-n`).
2. It follows the links defined in §9.5.1 b and the attributes' "Links to /
   Links from" fields: from R to the P, B, S, or Q group with that data link
   name, from P to embedded P, D, and B groups, and from measurement names to
   C groups.
3. Links that cannot be resolved are reported, not silently skipped.

**Outcome:** a structured channel view — label, data type, data source, the
linked format definition, its measurements, and their conversions — which is
what `irig106-studio` needs for labels and data-source grouping and what
`irig106-decode` needs to decode PCM, bus, and message data.

### UC-06 Validate against an edition — *0.3*

**Actor:** validator/QA, recorder setup author. **Trigger:** a document and
an edition (UC-04 or explicit).

The library checks every attribute against the registry entry for that
edition, using the fields Chapter 9 defines for each attribute (§9.5.1 a):
*R/R Ch 10 Status*, *Allowed when*, *Required when*, *Links to*, *Links from*
(values that are keys must be unique), *Range* (enumerations, numeric
ranges, lengths, hexadecimal, binary patterns, IP addresses, dates), and
*Default*. It also checks that multiple-entry indices run from 1 to their
`\N` count "with no missing values" (§9.5.1 a) and that every link resolves.
A "Chapter 10 recorder" profile applies the R/R Ch 10 Status rules on top.

**Outcome:** a report of findings, each with a stable rule identifier, a
severity, the attribute and its location, and the Chapter 9 citation.

### UC-07 Extend or relax the rules — *0.3*

**Actor:** extender. **Trigger:** a program's TMATS uses attributes or
conventions the standard does not define, or deliberately breaks a rule.

1. The extender supplies additional attribute definitions (new V-group
   vendor attributes, X-group extensions, or private codes) and overrides of
   built-in ones, from a file or in code.
2. The extender sets a policy: change a rule's severity, disable it, or add
   custom rules.
3. Validation (UC-06) runs with the layered registry and policy.

**Outcome:** the extender's TMATS validates on their terms, and the report
says which findings were changed by policy. Content the library does not
understand is still preserved (UC-01).

### UC-08 Keep vendor and extension attributes attached — *0.1 read, 0.5 edit*

**Actor:** extender, TMATS author. **Trigger:** a document with V or X groups.

V attributes (`V-x\acr\attribute-string`, §9.5.13) are grouped by data source
and vendor acronym. X attributes (`X-x\ORGANIZATION\ORIGCODE\EXTENSION_CODE-i-j-m-n`,
§9.5.14) are linked to the attribute they extend by matching group, code, and
leading indices. When an edit renumbers the extended attribute, its
extensions are renumbered with it, as §9.5.14 asks of editors.

### UC-09 Write TMATS back out — *0.1*

**Actor:** TMATS author, recorder setup author. **Trigger:** a document to
serialize.

By default the output reproduces the input bytes exactly, including order,
spelling, line breaks, and comments, with the caller's edits (UC-10) applied
in place. On request, a normalized form is produced (canonical order,
consistent line endings, recomputed `\N` counters); normalization is always
explicit.

### UC-10 Edit a document — *0.5*

**Actor:** TMATS author, recorder setup author. **Trigger:** the caller wants
to change TMATS.

The caller sets, inserts, removes, or renumbers attributes. The document keeps
everything else untouched, keeps X-group links intact (UC-08), and can
re-validate the result.

### UC-11 Compare two documents — *0.5*

**Actor:** archivist/analyst, recording reader. **Trigger:** two revisions of
a configuration, or two setup records from one recording.

The comparison is by attribute (case-insensitive code names, order ignored),
reports added, removed, and changed attributes, and handles repeated items
such as comments. For setup-record changes it says whether anything a decoder
depends on (channels, formats, measurements, conversions) changed.

### UC-12 Get fix suggestions and apply the chosen ones — *0.5*

**Actor:** TMATS author, validator. **Trigger:** a validation report.

Where a fix is unambiguous, a finding carries a suggested edit (for example
"`R-1\N` says 8 but channels 1–6 are defined; possible fix: set `R-1\N` to 6").
The library never applies a suggestion by itself; the caller picks
suggestions and applies them through UC-10. Counter mismatches are always
reported, because the declared count may be the true value (truncated input).

### UC-13 Generate TMATS from a channel inventory — *0.5*

**Actor:** recorder setup author, `irig106-write`. **Trigger:** a list of
channels (IDs, data types, and what is known about each).

The library produces a minimal document with G and R groups and format-group
stubs whose content satisfies the Chapter 10 recorder rules for the chosen
edition, using the correct keywords (for example `R-x\CDT-n` data type
mnemonics, not numeric codes). The output passes UC-06, and anything the
caller must still supply is reported as a finding rather than invented.

### UC-14 Handle non-conforming TMATS gracefully — *0.1 onward*

**Actor:** every actor. **Trigger:** real-world TMATS that breaks the standard
(wrong keywords, missing required attributes, bad counters, odd bytes).

Reading always succeeds as far as the bytes allow (UC-01). Validation reports
problems without stopping. Views (UC-05) return what can be resolved and
say what cannot. The caller decides what is fatal.

### UC-15 Compute and verify the TMATS checksum (`G\SHA`) — *0.1*

**Actor:** recorder setup author, validator/QA, recording reader.
**Trigger:** the caller wants to know whether TMATS is intact, or to stamp it.

Chapter 9 Table 9-2 defines `G\SHA`, "Message Digest / Checksum" (present from
106-15 onward): an algorithm designator, a hyphen, and hex digits, with SHA2-256
written as `2-` followed by 64 hex characters. Chapter 6 §6.2.3.11 f (the
recorder `.TMATS CHECKSUM` command) fixes the calculation: SHA-256 per FIPS
180-4 over the entire TMATS, discarding only the text from `G\SHA` through the
following semicolon, reported as `2-` plus 64 lower-case hex characters.

1. **Compute:** the library hashes the original bytes (UC-01 keeps them
   exactly), excluding the `G\SHA` item if present.
2. **Verify:** it compares the result with the embedded `G\SHA` value and
   reports match, mismatch, absent, unknown algorithm designator, or
   malformed value.
3. **Stamp:** on request it produces a *suggested edit* that inserts or updates
   `G\SHA` (UC-10/UC-12); it never rewrites the checksum on its own.

**Why the lossless design matters here:** the digest covers raw bytes, so any
normalization (reordering, case changes, line endings) invalidates it. After
an edit (UC-10), the report says the stored checksum no longer matches and
offers the stamp edit.

These are integrity checks, not cryptographic signatures: there is no key, so
they detect accidental change, not tampering.

### UC-16 Compute the irig106.org "flex signature" — *0.1, opt-in*

**Actor:** analysts comparing configurations; users of irig106.org tools.
**Trigger:** the caller needs a signature compatible with `igDisplayTMATS` /
`irig106lib`.

This is **not** part of IRIG 106. `irig106lib` (`enI106_Tmats_Signature`,
BSD-3-Clause) defines it: a Fletcher-32 checksum of each upper-cased
`code:value;` line, **summed** over lines (so attribute order does not matter),
excluding by default comments, V-group attributes, G-group attributes, and a
fixed list of R-group recorder-identity and point-of-contact attributes; flags
include each class, and the result is written `OO-SSSSSSSS` (opcode = flags and
algorithm version, then the signature). Its purpose is a stable fingerprint of
the *data-describing* content that ignores comments, contacts, and recorder
identity. The library reproduces it exactly, including its exclusion list, and
labels it non-standard everywhere it appears.

### UC-17 Inspect TMATS from the command line (`tmats`) — *0.1 onward*

**Actor:** every human actor; CI scripts. **Trigger:** someone has a Chapter 10
file or a TMATS text file and wants to look at it.

A command-line tool, `tmats`, built on this library, replaces and extends
irig106.org's `idmptmat` (whose source is published in the RCC 123 Chapter 10
Programmers' Handbook) and the viewing parts of `igDisplayTMATS`:

| Command | Does | Delivered |
|---------|------|-----------|
| `tmats show FILE` | Channel summary (default): channel ID, data type, enabled, data source — `idmptmat -c` | 0.1 from R-group items; 0.2 with resolved links (UC-05) |
| `tmats show --tree FILE` | Hierarchy: data sources → recorders → channels, then every group by occurrence and code path — `idmptmat -t` | 0.1 |
| `tmats show --raw FILE` | The TMATS text exactly as stored — `idmptmat -r` | 0.1 |
| `tmats extract FILE.ch10 -o OUT` | Write the setup-record TMATS bytes to a file | 0.1 |
| `tmats checksum FILE` | Print the `G\SHA` value (UC-15) | 0.1 |
| `tmats verify FILE` | Check the embedded `G\SHA`; exit status reflects the result | 0.1 |
| `tmats stamp FILE -o OUT` | Write a copy with `G\SHA` inserted or updated | 0.1 |
| `tmats checksum --flex [--include ...] FILE` | irig106.org flex signature (UC-16) | 0.1 |
| `tmats validate FILE` | Validation report (UC-06/07) | 0.3 |
| `tmats diff A B` | Compare two documents (UC-11) | 0.5 |

Input is detected by content (a Chapter 10 file starts with the packet sync
pattern; anything else is treated as TMATS text). Unlike `idmptmat`, which
reads only the first packet, `tmats` reports every setup record in a
recording. Output is plain text by default and machine-readable (JSON) on
request, with documented exit codes so scripts can rely on it.

The tool ships from this repository as a second crate, `irig106-tmats-cli`
(binary `tmats`), at **the same version as the library, always**. It is a
focused TMATS tool: the complete, ecosystem-wide command-line tool is
`irig106-cli`, which uses this library (and may mount `tmats`' commands as a
subcommand). A GUI view belongs in `irig106-studio`, using the same library
calls.

## 5. Consumer notes

- **`irig106-ch10-reader`** reports only TMATS presence and payload size
  today. With UC-02/04/05 it can report the edition, channel labels, and
  TMATS findings in a detail mode.
- **`irig106-studio`** (`docs/INTEGRATION.md` in that repo) wants channel
  labels, data-source grouping, and the standard version. Its sketched
  contract takes the version from `R-1\ID` and uses an `R-1\NSS` attribute;
  the edition belongs to `G\106` (UC-04), and no `NSS` code name exists
  anywhere in 106-23 Chapter 9. The studio contract should be revised against
  UC-04 and UC-05 when this library's API exists.
- **`irig106-decode`** needs UC-05's full chain (format, measurements,
  conversions) for PCM, analog, and discrete data.
- **`irig106-write`** needs UC-09/UC-13 and the setup-record payload builder.
- **`irig106-ch10-reader`**'s Windows guide currently tells users to extract
  TMATS bytes with other tools; `tmats extract` (UC-17) replaces that advice.
- **irig106.org tools** (`idmptmat`, `igDisplayTMATS`, `irig106lib`) are
  reference points, not authorities: see `docs/TEST-DATA.md` for the defects
  found in them.

## 6. Quality attributes

- **Faithful:** round trips are byte-identical by default; nothing is dropped.
- **Standard-aligned:** every definition and rule cites edition, table, and
  row; behaviour the standard does not define is either configurable or
  reported, never invented.
- **Fast enough to be invisible:** TMATS is kilobytes to a few megabytes and
  read once per recording. Reading is a single pass with one owned buffer,
  lookups are constant-time, and typed values are parsed only when asked for.
  Benchmarks on real files guard this.
- **Extensible:** layered definitions, policies, and rules (UC-07).
- **Robust:** never panics on any input; fuzzed.
- **Small dependency footprint:** no terminal renderers or bindings in the
  core crate.

## 7. Open questions for review

1. Should one library call accept a whole Chapter 10 file and find the setup
   record itself, or should that stay in `irig106-core` / readers? Current
   position: the library stays payload-level; the `tmats` CLI (UC-17) needs to
   open Chapter 10 files, so it carries a minimal packet reader until
   `irig106-core` provides one.
2. Chapter 9 marks some ranges as a *recommended* maximum length. Should
   exceeding them be a warning by default?
3. For UC-11, is a mid-recording setup-record change something `irig106-ch10-reader`
   should surface by default?
4. Which real-world TMATS sources should the test corpus include first
   (recorder vendors, ranges, synthetic files from editors)?
