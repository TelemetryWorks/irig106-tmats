---
status: accepted
date: 2026-09-25
decision-makers: Joey
---

# The CLI carries a minimal Chapter 10 packet reader until `irig106-core` provides one

## Context and Problem Statement

`tmats` must open a Chapter 10 recording "enough to read the tmats packet"
(owner), and must report every setup record, not only the first (unlike
`idmptmat`, defect D2). The library performs no I/O (ADR-0010).
`irig106-core`, which should own packet reading, is still a placeholder.

## Decision Drivers

* The CLI is needed before `irig106-core` exists
* Large recordings must not be read in full just to find setup records
* Packet-level knowledge should end up in one crate

## Considered Options

* **A small reader inside the CLI: memory-map the file, check the sync
  pattern, walk from packet header to packet header by packet length, and
  return data type `0x01` payloads with their offsets**
* Wait for `irig106-core`
* Put packet reading in the library

## Decision Outcome

Chosen option: **a minimal reader inside the CLI**, replaced by
`irig106-core` when it is available. Input is detected by content: a file
starting with the Chapter 10 sync pattern is a recording; anything else is
TMATS text. Header checksums and damaged-file recovery stay out of scope
until `irig106-core`.

### Consequences

* Good: `tmats` is useful on real recordings from 0.1, and fast on large
  files.
* Bad: a small amount of packet code exists in two places until
  `irig106-core` lands; it is kept deliberately minimal to make the
  replacement easy.
