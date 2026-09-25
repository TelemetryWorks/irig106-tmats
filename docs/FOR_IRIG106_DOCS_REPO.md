# irig106-tmats ↔ Ecosystem Integration Specification

> **This document belongs in the `irig106-docs` repository.**
> It defines the cross-crate contracts that other crates depend on.

## irig106-tmats ↔ irig106-write

### Payload Contract

`irig106-tmats::ch10::encode_setup_payload()` produces a `SetupRecordPayload` whose `to_bytes()` method returns bytes formatted as:

```
[Offset 0..3]  CSDW — 4 bytes, little-endian u32
                 Bits 0-7:   iCh10Ver
                 Bit 8:      bConfigChange
                 Bits 9-31:  Reserved (zero)

[Offset 4..]   TMATS ASCII payload — 7-bit ASCII, code-name:value; format
```

`irig106-write` is responsible for:
- Wrapping these bytes in a Type 0x01 Ch10 packet (header + secondary header + data + trailer)
- Setting packet header fields (sync, channel ID = 0, data type = 0x01, packet length)
- Computing and appending the packet checksum
- If payload exceeds maximum packet size, splitting into multiple consecutive Type 0x01 packets

`irig106-tmats` is NOT responsible for:
- Packet headers, trailers, or checksums
- Multi-packet splitting (packet-level concern)
- File-level positioning (first packet requirement)

### Type Dependencies

`irig106-write` should import from `irig106-tmats`:
- `SetupRecordPayload` — the payload container
- `SetupRecordCsdw` — for CSDW construction

## irig106-tmats ↔ irig106-ch10-reader

### Read Workflow

`irig106-ch10-reader` extracts the raw data payload from Type 0x01 packets:
1. Reader finds packet(s) with data_type = 0x01
2. Reader handles multi-packet reassembly (concatenating payloads from consecutive 0x01 packets)
3. Reader passes the reassembled payload bytes to `irig106-tmats::ch10::decode_setup_payload()`

`irig106-tmats` receives a contiguous byte buffer and handles CSDW + TMATS parsing.

### Type Dependencies

`irig106-ch10-reader` should import from `irig106-tmats`:
- `decode_setup_payload()` — the entry point
- `SetupRecordCsdw` — returned alongside the document
- `TmatsDocument` — the parsed result

## irig106-tmats ↔ irig106-validator

### Validation Integration

`irig106-validator` performs file-level validation and delegates TMATS-specific checks to this crate:

```rust
let (csdw, doc) = irig106_tmats::decode_setup_payload(payload)?;

let ctx = ValidationContext {
    target_version: csdw.irig_version(),
    source: TmatsSource::Ch10,
    profile: ValidationProfile::Full,
};

let tmats_report = irig106_tmats::validate_with_options(&doc, &ctx);

// Merge into file-level validation report
for diag in tmats_report.diagnostics {
    file_report.add_finding(diag);
}
```

## irig106-tmats ↔ irig106-cli

### CLI Commands

The CLI should expose TMATS operations as subcommands:

- `irig106 tmats dump <file.ch10>` — parse and display TMATS
- `irig106 tmats validate <file.ch10>` — validate TMATS
- `irig106 tmats repair <file.ch10> -o <output.ch10>` — repair and rewrite
- `irig106 tmats generate <file.ch10> -o <output.tmt>` — generate TMATS from packet scan
- `irig106 tmats diff <a.ch10> <b.ch10>` — compare TMATS records
- `irig106 tmats channels <file.ch10>` — list channels with resolved config

## Shared Types (irig106-types)

The following types are currently defined in `irig106-tmats::types_bridge` and should migrate to `irig106-types`:

| Type | Used By |
|------|---------|
| `Irig106Version` | tmats, ch10-reader, write, validator, cli, studio |
| `DataTypeCode` | tmats, ch10-reader, write, decode, cli |
| `ChannelId` | tmats, ch10-reader, write, index, cli |
| `GroupPrefix` | tmats, validator, cli |

Once `irig106-types` exports these, `irig106-tmats` should:
1. Remove `src/types_bridge.rs`
2. Add `irig106-types` as a dependency
3. Re-export the types in the prelude
