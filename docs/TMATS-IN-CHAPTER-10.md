# What TMATS gives Chapter 10 processing

> **Status: design draft, written one section at a time for the owner's
> review** (follow-up F4 in `docs/ROADMAP.md`; outline approved 2026-09-26).
> Every attribute, rule, and quotation is taken from the archived standard
> (`TelemetryWorks/rcc-106-standards`; baseline 106-24R1) and cited. What the
> other repositories expect today is recorded in
> `docs/research/2026-09-26-consumer-survey.md`.

## Contents

| Section | Status |
|---------|--------|
| 1. What `irig106-tmats` is for | **draft for review** |
| 2. Where TMATS sits in Chapter 10 processing | **draft for review** |
| 3. What the description answers, question by question | to be written |
| 4. Contracts per consumer | to be written |
| 5. Configuration over a recording | to be written |
| 6. When TMATS is missing, wrong, or disagrees with the data | to be written |
| 7. A worked example: Appendix 9-C from channel to engineering units | to be written |
| 8. What changes as a result | to be written |

---

## 1. What `irig106-tmats` is for

### 1.1 The problem: packets carry data, not meaning

A Chapter 10 recording is a sequence of packets. Each packet header gives
the channel ID, the data type, lengths, a sequence number, flags, and a
relative time; the body holds the data (Chapter 11 §11.2.1, 106-24R1). None
of that says **what** the data is. A packet on channel ID 2 with data type
`0x09` ("PCM Data, Format 1", Table 11-4) is a run of bits; nothing in it
says how those bits are framed, which of them hold which measurement, or
what a value means in engineering units.

That knowledge is in the **setup record**:

> "Format 1 defines a setup record that describes the hardware, software,
> and data channel configuration used to produce the other data packets in
> the file." — Chapter 11 §11.2.7.2

Its content is TMATS, Chapter 9:

> "Telemetry attributes are those parameters required by the
> receiving/processing system to acquire, process, and display the
> telemetry data received from the test item/source." — Chapter 9 §9.1

The setup record is a **required** packet type (Chapter 11 Table 11-3) of
data type `0x01` ("Computer-Generated Data, Format 1 — Setup Record",
Table 11-4), carried from 106-17 on channel ID `0x0000`, a channel that may
also carry Format 4 streaming configuration records (§11.2.1.1 b). One
record may be up to 134,217,728 bytes and may span several consecutive
packets (§11.2.1 c, §11.2.7.2); a recording may carry more than one
record when the recorder's configuration changes (§11.2.7.2, SRCC).

![What a packet says, and what the setup record adds](diagrams/packet-vs-setup-record.svg)

*The standard's own example* (106-24R1 Chapter 9, Appendix 9-C). A PCM
packet on channel ID 2 says only that it holds PCM bits. The setup record
says that channel 2 is a PCM input whose data link is named `PCM1`
(`R-1\TK1-1:2; R-1\CDT-1:PCMIN; R-1\CDLN-1:PCM1;`); that `PCM1` is a
2 Mbit/s NRZ-L stream of 10-bit words, 277 words per minor frame and 64
minor frames with a 30-bit sync pattern (`P-2`); that measurement `82AJ01`
is two fragments at words 113 and 121 of frame 5, repeating every 32 frames
(`D-3`); and that `82AJ01` is a two's-complement value multiplied by
0.03125, "LANTZ Norm acceleration" in m/s² (`C-7`). Without the setup
record the packet cannot be decoded; with it, every step is defined.
Section 7 follows this chain in full.

### 1.2 What `irig106-tmats` does

**`irig106-tmats` is the one place in the ecosystem where the setup record
is understood.** It turns the setup records of a recording — or a TMATS
file — into a verified, queryable **description of the recording**, and it
is the only component that reads, checks, changes, or produces TMATS.

| Step | What the library does | Decided in |
|------|-----------------------|------------|
| Assemble | Joins setup-record fragments, supplied by the caller with their provenance, into complete setup records, or accepts TMATS text | ADR-0025; L1-CH10-001, 004 |
| Read | Reads every attribute exactly as written — known, unknown, vendor, extension, duplicate, or malformed — and reports every problem without stopping | ADR-0002, 0003, 0021; L1-READ |
| Interpret | Gives each attribute its meaning from a registry built from the cited Chapter 9 tables, with reviewed interpretations where the standard is unclear | ADR-0022, 0026, 0027; `docs/INTERPRETATIONS.md` |
| Answer | Presents channels, formats, measurements, conversions, and derived parameters, following the links between groups | ADR-0024, 0026; L1-VIEW, L1-DER |
| Check | Validates against an edition, stating which edition's rules were applied and why; verifies `G\SHA` | ADR-0023, 0028, 0014; L1-VAL, L1-EDN, L1-SUM |
| Change and produce | Edits as verified transactions, stamps `G\SHA`, generates TMATS from a channel inventory, builds the setup-record payload | ADR-0007, 0029; L1-WRT, L1-CH10-003 |

The **description** that consumers receive is: the document (every
attribute, as read, with its location); the answers built over it (channel,
format, measurement, conversion, and derived-parameter views); the
declarations it carries (the TMATS edition, each setup record's
recording-format version, the checksum); and every diagnostic found along
the way.

### 1.3 What it deliberately does not do

| Not done here | Done by | Why |
|---------------|---------|-----|
| Open files; read packets other than setup-record fragments | the caller — today the `tmats` CLI's minimal reader, later `irig106-core` | The library performs no I/O (ADR-0010, ADR-0019; ROADMAP X3) |
| Decode packet data; extract measurement values; apply conversions; evaluate derived parameters; interpret floating-point formats | `irig106-decode` | It needs data over time and numeric policy that a description does not have (ADR-0024; NR-007; ROADMAP X6) |
| Handle time (time packets, time formats, correlating relative time) | `irig106-time` | Separate concern; TMATS only says which channels and formats are involved |
| Write packets or files | `irig106-write` | The library builds the setup-record payload only (L1-CH10-003) |
| XML TMATS, DDML, IHAL | deferred | ADR-0015, NR-001; Chapter 9 §9.6–9.7 |
| Decide what is fatal | the caller | Reading and validation report; the caller decides (UC-14) |
| Repair anything by itself | nobody | Fixes are suggested edits the caller chooses (ADR-0007) |

### 1.4 Who relies on it

| Consumer | What it needs from the description | Today (survey) |
|----------|-----------------------------------|----------------|
| `irig106-decode` | For each channel: the format its data follow, where each measurement is, how each converts, how derived parameters are defined | Placeholder; needs stated nowhere yet |
| `irig106-core` | What each channel ID is (data type, enabled) to check packets against the configuration; setup-record fragments to hand over | Placeholder |
| `irig106-time` | Which channels carry time, in what format; the recording-format version | Reads the CSDW version byte only |
| `irig106-studio` | Channel labels, data-source grouping, the edition, the raw text, findings | Real code; parses nothing yet (its TMATS changes: studio `docs/TMATS-ISSUES.md`) |
| `irig106-ch10-reader` | Presence, size, configuration changes, a summary | Presence check only |
| `irig106-index`, `irig106-cli` | Channel and measurement catalogues per setup record | Placeholders |
| `irig106-write` | Generated or edited TMATS, stamped, as a setup-record payload | Placeholder |

Section 4 turns each row into a contract.

### 1.5 Promises every consumer can rely on

1. **Nothing is lost.** Every byte and attribute of the setup record is
   kept, in order; output is byte-identical unless the caller asks
   otherwise (ADR-0002).
2. **Nothing is silent.** Every value is explicit, defaulted (with the
   citation of the default), missing, invalid, or ambiguous; every link is
   resolved, unresolved, or ambiguous with all candidates listed; every
   problem is a diagnostic with a location (ADR-0023, ADR-0026).
3. **Nothing is guessed.** Where the standard is unclear, the behaviour is a
   reviewed, tested entry in the interpretation register (ADR-0027).
4. **Everything is cited.** Each rule names the edition, table, and row it
   comes from (L1-REG-001).
5. **Editions are stated, not assumed.** The TMATS edition and the
   recording-format version are reported as declared, and every check names
   the edition applied and why (ADR-0028).
6. **Nothing changes without a decision.** The library never alters a
   document except through an edit the caller applies, as a verified
   transaction (ADR-0007, ADR-0029).
7. **No I/O, no global state.** The library works on bytes the caller
   supplies (ADR-0010).

### 1.6 Questions for the next sections

- **Does `irig106-core` depend on `irig106-tmats`?** Checking packets against
  the channel table needs the description; parsing packets does not. Either
  core depends on the library, or a layer above joins them (section 4).
- **Two version code lists.** The packet header's *data type version* uses
  one list ("0x09 = RCC 106-19", "0x0A = RCC 106-22", §11.2.1.1 e) and the
  setup record's RCCVER another ("0x0E = RCC 106-22", §11.2.7.2): the same
  number means different editions in the two fields. Consumers must not
  mix them (sections 3 and 4).
- **Channel `0x0000` is not only TMATS.** From 106-17 it carries setup
  records or Format 4 streaming configuration records, so setup records are
  found by data type `0x01`, never by channel ID alone (sections 4 and 6).
- **One description per setup record.** When a recording's configuration
  changes, consumers need to know which description governs which packets
  (section 5).
- **`irig106-studio` runs as WebAssembly.** ADR-0015 moved the WASM bindings
  out of the library ("WASM returns as a separate `irig106-tmats-wasm`
  crate"); the library's lack of I/O keeps that possible, and section 4 must
  say what studio can rely on until then.

---

## 2. Where TMATS sits in Chapter 10 processing

![Where TMATS sits in Chapter 10 processing](diagrams/tmats-in-the-pipeline.svg)

*Two directions.* Reading a recording (top): a packet reader walks the file,
hands setup-record fragments to `irig106-tmats` and every other packet to
the components that decode them; `irig106-tmats` gives them, and the tools
that present results, the description they need. Producing a recording
(bottom): a caller's channel inventory or edits become a stamped
setup-record payload that `irig106-write` packs into packets. Letters name
what crosses each boundary; section 2.3 defines each.

### 2.1 Reading a recording

1. **A packet reader walks the file** (A). It reads each packet header,
   checks the header checksums before trusting any length, and slices each
   packet body by its Data Length (Chapter 11 §11.2.1; ADR-0025). Today this
   is the `tmats` CLI's minimal reader (ADR-0019); it moves to
   `irig106-core` when that exists (ROADMAP X3).
2. **Setup-record fragments go to `irig106-tmats`** (B). A packet of data type
   `0x01` carries a fragment of a setup record (Table 11-4). The reader hands
   over each fragment's bytes — after the CSDW, without filler or checksum —
   with its provenance: file offset, channel ID, sequence number, relative
   time counter, the CSDW fields, and the data-type version (ADR-0025).
   Finding setup records by data type, never by channel ID alone, matters:
   channel `0x0000` also carries Format 4 streaming configuration records
   from 106-17 (§11.2.1.1 b).
3. **`irig106-tmats` builds the description.** It assembles complete setup
   records ("A single setup record may span multiple consecutive packets",
   §11.2.7.2), reads each one without loss, interprets it through the
   registry, and makes the answers, declarations, checksum status, and
   findings available (section 1.2). One description exists per setup
   record.
4. **Every other packet goes to the component that decodes it** (C), by
   channel ID: data packets to `irig106-decode`, time packets to
   `irig106-time`.
5. **Those components ask the description how to interpret each channel**
   (D, E). `irig106-decode` needs the channel's data type and format
   definition, where each measurement lies, how each converts, and how
   derived parameters are defined; `irig106-time` needs which channels carry
   time, in what format, and the recording-format version (section 3 lists
   the attributes).
6. **Tools present the results** (F, G): `irig106-studio`, `irig106-index`,
   `irig106-ch10-reader`, and `irig106-cli` show labels, groupings,
   declarations, findings, and the raw TMATS from the description, and
   engineering-unit values from `irig106-decode`.

**Order matters.** When a recorder's configuration changes, "the new setup
record packet will be committed to the stream prior to any new or changed
data packets", preceded by "a setup record configuration change event
packet" (§11.2.7.2). A packet is therefore interpreted with the description
of the setup record that precedes it; section 5 defines how a consumer
learns which one that is.

### 2.2 Producing a recording

1. **The caller supplies a channel inventory, or edits** to an existing
   document.
2. **`irig106-tmats` generates or edits the TMATS** as a verified
   transaction: with sufficient input the result validates for the selected
   edition, otherwise it is an incomplete draft with missing-input findings
   (L1-WRT-006, L1-WRT-007); the `G\SHA` stamp is the transaction's last step
   (L1-SUM-004).
3. **It returns the setup-record payload** (H): the CSDW and the TMATS body
   (L1-CH10-003).
4. **`irig106-write` packs the payload into packets and writes the file.**
   A packet may hold at most 524,288 bytes, while a setup record may be up to
   134,217,728 bytes (§11.2.1 c, Table 11-3), so a large record is split
   across consecutive packets whose "sequence counter shall increment in the
   order of segmentation of the setup record, n+1" (§11.2.7.2). For a
   configuration change, `irig106-write` sets the SRCC bit and inserts the
   configuration-change event packet first (§11.2.7.2).

### 2.3 What crosses each boundary

| | From → to | What crosses | Defined in |
|---|-----------|--------------|------------|
| A | recording → packet reader | the file's bytes (memory-mapped) | ADR-0010, ADR-0019 |
| B | packet reader → `irig106-tmats` | setup-record fragments: bytes after the CSDW, and provenance (file offset, channel ID, sequence number, relative time counter, CSDW fields, data-type version) | ADR-0025; L1-CH10-001, 004 |
| C | packet reader → `irig106-decode`, `irig106-time` | every other packet, by channel ID | `irig106-core` (to be designed) |
| D | `irig106-tmats` → `irig106-decode` | for each channel: data type and packing, format definition (PCM frame, bus messages, message data), measurement locations, conversions, derived-parameter descriptions — each with its state | sections 3 and 4 |
| E | `irig106-tmats` → `irig106-time` | the channels that carry time and their formats; the recording-format version declared | sections 3 and 4 |
| F | `irig106-tmats` → presentation tools | channel labels and data-source grouping, the TMATS edition and recording-format version declared, `G\SHA` status, findings, the raw TMATS bytes | sections 3 and 4 |
| G | `irig106-decode` → presentation tools | values in engineering units | `irig106-decode` |
| H | `irig106-tmats` → `irig106-write` | the stamped setup-record payload (CSDW and body) | L1-CH10-003, L1-SUM-004 |

Every boundary carries bytes and plain data, never files or handles: the
library performs no I/O (ADR-0010). Shared types — the edition, the
data-type codes, the setup-record CSDW layout — come from `irig106-types`
(ADR-0009).

### 2.4 Which crate depends on which (proposal)

Section 1.6 asked whether `irig106-core` depends on `irig106-tmats`. The
proposal, for the owner's decision (ROADMAP follow-up F5):

| Crate | Depends on | Does not depend on |
|-------|-----------|--------------------|
| `irig106-types` | — | — |
| `irig106-tmats` | `irig106-types` | any packet reader or decoder |
| `irig106-core` | `irig106-types` | `irig106-tmats` |
| `irig106-decode` | `irig106-types`, `irig106-tmats` (the description's types) | the packet reader's internals |
| `irig106-time` | `irig106-types` | — (receives what it needs from TMATS as plain data) |
| tools (`studio`, `ch10-reader`, `cli`, `index`) | `irig106-core`, `irig106-tmats`, `irig106-decode`, `irig106-time` | — |

Why `irig106-core` should not depend on `irig106-tmats`:

- Its job is structural — walking packets fast, even when the TMATS is
  missing or damaged; a dependency on the setup record would couple the two
  for no gain.
- It still serves the library: it produces the fragments (B) the assembler
  needs, as plain data.
- Checking packets against the configuration — a channel in the data but
  not in TMATS, a data type that differs from `R-x\CDT-n`, a disabled channel
  that carries data — needs both the packets and the description, so it
  belongs to the layer that holds both: the tools, or a small checking
  function in `irig106-tmats` that takes a packet summary as plain data
  (section 6).

The alternative — `irig106-core` depends on `irig106-tmats` and checks every
packet against the channel table as it reads — puts the check in one place,
at the cost of making the structural reader depend on the setup record.
