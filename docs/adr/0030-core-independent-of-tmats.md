---
status: accepted
date: 2026-09-26
decision-makers: Joey
---

# `irig106-core` does not depend on `irig106-tmats`; they exchange plain data through `irig106-types`

## Context and Problem Statement

Reading a recording well needs a check of the packets against the setup
record that governs them: a packet on a channel the setup record does not
list, a data type that differs from the channel's `R-x\CDT-n`, data on a
channel marked disabled (`R-x\CHE-n`). `irig106-core` ("structural traversal
and packet parsing") walks the packets; `irig106-tmats` understands the
setup record. The question (ROADMAP follow-up F5) was where the check lives,
and so whether `irig106-core` must be built with `irig106-tmats` inside it.

## Decision Drivers

* The packet reader stays fast and usable when the TMATS is missing,
  damaged, or not wanted
* Each crate keeps one job
* No choice now that blocks a shared joining crate later

## Considered Options

* **A — core independent; the check is a function in `irig106-tmats` over
  plain data; each tool writes the joining loop**
* B — core depends on `irig106-tmats` and checks packets as it reads
* C — a third crate that joins core, the TMATS library, and later the
  decoder

## Decision Outcome

Chosen option: **A** (owner decision, 2026-09-26: "go with A and we may
switch to C in the future").

- `irig106-core` depends on `irig106-types`, not on `irig106-tmats`.
- The data that crosses between them is plain data defined in
  `irig106-types`: the **setup-record fragment with its provenance** (the
  assembler's input, ADR-0025) and the **packet summary** (channel ID, data
  type, offset, sequence number, relative time counter).
- The **check** is a function in `irig106-tmats` that takes packet summaries
  and the governing description and returns findings (L1-CH10-007).
- Each tool writes the **joining loop**: walk the recording with core, hand
  setup-record fragments to `irig106-tmats`, track which description governs
  the packets that follow, call the check, pass packets on to the decoder.
- `irig106-types` holds only plain data; anything with behaviour stays in
  the crate that owns it.

### Consequences

* Good: core and the TMATS library are independent, and each can be built,
  tested, and released alone.
* Good: switching to C later moves the joining loop into a crate; the check
  does not change.
* Bad: until then, tools repeat a short joining loop; if that repetition
  grows, C is revisited.
* `irig106-types` gains the two plain types (ROADMAP X2), and
  `irig106-core`'s design follows (ROADMAP X3).
