---
status: accepted
date: 2026-09-25
decision-makers: Joey
---

# Shared IRIG 106 types come from `irig106-types`

## Context and Problem Statement

The prototype defined its own `Irig106Version` (stopping at 106-17, with
`#[repr(u8)]` values that are neither CSDW codes nor year numbers),
`DataTypeCode`, and setup-record CSDW layout. `irig106-time` defines a
different, incompatible `Irig106Version`. The setup-record CSDW (version, the
setup-record-configuration-change flag, and the ASCII/XML format flag) is a
packet-level concern shared with `irig106-core`, `irig106-write`, and
`irig106-ch10-reader`.

## Decision Drivers

* One definition of each type across the ecosystem
* Versions through 106-24 and an `Unknown` value, so future editions are
  reported rather than rejected
* Owner direction: "Use irig106-types now"

## Considered Options

* **Define `Irig106Version`, the Chapter 10 data-type code, and the
  Computer-Generated Format 1 CSDW layout in `irig106-types` and use them
  here**
* Keep local copies and migrate later

## Decision Outcome

Chosen option: **use `irig106-types` now**. The types are `#[non_exhaustive]`
with an `Unknown` value where the wire can carry codes the crate does not
know. Bit positions are confirmed from the archived Chapter 10/11 text before
they are encoded.

### Consequences

* Good: `irig106-time` and this crate agree on versions.
* Bad: this crate depends on `irig106-types` being published before its own
  0.1 release; until then the dependency is a path dependency and CI needs the
  sibling repository or a published version.
