# What the other repositories expect from TMATS (2026-09-26)

A read-only survey of the sibling repositories in `telemetryworks/`, made to
answer the owner's question (owner-direction entry 22; ROADMAP follow-up
F4): what does processing the TMATS setup record give the rest of Chapter 10
processing? Nothing outside `irig106-tmats` was changed. Two findings were
re-checked by hand before this record was written: `irig106-studio`'s
planned interface (`docs/INTEGRATION.md`) and its TMATS extraction
(`crates/irig106-studio-core/src/summary.rs`); and `irig106-core` and
`irig106-decode` being 14-line stubs with no dependencies.

## Summary

- Only `irig106-studio` has real code that uses TMATS, plus
  `irig106-ch10-reader`'s presence check and `irig106-time`'s reading of
  the setup-record CSDW. No repository depends on `irig106-tmats` yet;
  studio has it only as a commented-out line.
- `irig106-core`, `irig106-decode`, `irig106-write`, `irig106-cli`, and
  `irig106-index` are placeholder crates with no TMATS design notes. What a
  decoder needs from TMATS is written down only in this repository
  (`docs/USE-CASES.md`) and in studio's `docs/INTEGRATION.md`.

## By repository

| Repository | State | Uses or plans to use from TMATS |
|------------|-------|---------------------------------|
| `irig106-studio` | Real code (Tauri, WASM, TypeScript) | Channel labels and data-source grouping instead of synthetic `"Ch {cid}"` and `"DS-1"`; the edition; raw TMATS text for display; TMATS as optional context to PCM, analog, and discrete decoders (`decode_payload(data_type, payload, tmats: Option<&TmatsMetadata>)`). Planned interface `parse_tmats(text) -> TmatsMetadata { recording, channels, standard_version }`. |
| `irig106-ch10-reader` | Real CLI (`ch10r`) | Presence only: a channel-0 packet of data type `0x01` means "TMATS present". Does not parse the text. |
| `irig106-time` | Real code | The setup-record CSDW only (`detect_version(tmats_csdw)`), not the text; its multi-time-source selection (L1-COR-003) does not yet use TMATS. |
| `irig106-types` | Real code | Nothing yet; ROADMAP X2 would move the edition enum, data-type codes, and Format 1 CSDW there. |
| `irig106-core`, `irig106-decode`, `irig106-write`, `irig106-cli`, `irig106-index` | Placeholders | Nothing stated. |
| `irig106-docs` | Mostly empty mdBook | One line: "`irig106-tmats`: Parsing and querying for TMATS setup records." |
| `irig106-rust` (timecodes) | Requirements only | REQ-L1-013 "parsing and generation of … (TMATS) records"; REQ-L1-014 calls the setup record "Format 0" (it is Format 1). |

## Problems found in consumers (not yet in ROADMAP X1–X7)

- Studio's TMATS extraction joins **every channel-0 payload whatever its
  data type** (events `0x02`, index `0x03`, and others mixed in), starts
  right after the 24-byte header so each **CSDW is included**, keeps any
  filler, and decodes with `from_utf8_lossy`.
- Studio's planned contract reads the edition "from R-1\ID or R-1\RI1" and a
  recorder field `R-1\NSS`; the edition is `G\106` (ADR-0028), and USE-CASES
  §5 already records that no `NSS` code name exists.
- Studio's TMATS syntax highlighter names groups wrongly ("D = Data",
  "B = Bus", "M = Multiplexer", "S = Spacecraft"; Chapter 9 Tables 9-7, 9-8, 9-5, and 9-9 name
  them PCM Measurement Description, Bus Data, Multiplex/Modulation, and
  Message Data).
- `irig106-time`'s ecosystem diagram shows `irig106-tmats` feeding only the
  reader and the CLI, not decode, studio, or write.
- `irig106-rust` REQ-L1-014: "Format 0" for the setup record.

## Needs no repository states yet (gaps)

1. PCM frame synchronisation (P group): bit rate, word and frame lengths,
   sync pattern, minor and major frames, subframe ID counter, bit order,
   packing.
2. Measurement placement (D group): word and frame positions, masks,
   fragments, supercommutation.
3. Bus and message layouts (B, S, Q groups): 1553 and ARINC 429 message to
   measurement mapping.
4. Per-channel recorder settings (R group) that govern how packets are
   packed (PCM packing mode, analog, discrete, video, Ethernet formats).
5. Time: which channel is the time source and its format.
6. Configuration over time: which setup record governs which packets.
7. Engineering-unit conversion (C group) as a typed contract to
   `irig106-decode`, and searches by measurement name (`irig106-index`,
   `irig106-cli`).
8. Channels present in the data but absent from TMATS, and the reverse;
   disabled channels.
9. `irig106-write`'s needs (payload, `G\SHA`, CSDW, splitting a record across
   packets), stated only in this repository.

These gaps become the working list of the document planned under F4.
