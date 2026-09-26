---
status: accepted; stamping contract refined by 0029
date: 2026-09-25
decision-makers: Joey
---

# `G\SHA` over the original bytes; the irig106.org flex signature as a labelled compatibility option

## Context and Problem Statement

`igDisplayTMATS` "can also calculate the IRIG 106 Chapter 9 TMATS signature
and validate an embedded signature", and the owner wants "the different
signature functionality" in `tmats`. Investigation
(`docs/research/2026-09-25-irig106org-tools.md`) found two different things:

1. **`G\SHA`**, Chapter 9 Table 9-2 (from 106-15): "The entire contents of the
   TMATS file except the characters from "G\SHA:" to the following ";"
   (inclusive) shall be used to calculate the checksum ... SHA2-256 shall be
   represented as "2-" followed by 64 hex characters." Chapter 6 §6.2.3.11 f
   specifies SHA-256 and "64 lower-case hexadecimal characters".
2. **The "flex signature"**, defined only by `irig106lib`: a sum of per-line
   Fletcher-32 checksums over upper-cased lines, excluding comments, the G
   and V groups, and a fixed list of R-group identity attributes by default,
   written `OO-SSSSSSSS`.

`irig106lib`'s SHA-256 routine hashes the wrong range when `G\SHA` is last and
matches `G\SHA` text inside values (D5, D6); `idmptmat` compiled its signature
output out as incorrect (D3).

## Decision Drivers

* Follow the standard exactly for the standard checksum
* Compatibility with irig106.org tools where users already rely on them
* Never change a stored checksum silently

## Considered Options

* **Compute `G\SHA` over the original bytes during the scan; support the flex
  signature exactly as `irig106lib` defines it, labelled non-standard**
* Standard checksum only
* Reuse `irig106lib`'s behaviour as the reference

## Decision Outcome

Chosen option: **`G\SHA` computed during the single scan over the original
bytes, excluding exactly the `G\SHA` item; the flex signature reproduced
bit-for-bit (including its `MPOC4`/`DPOC4` quirk, D7) and labelled
non-standard wherever it appears.** Verification reports match, mismatch,
absent, unknown algorithm, or malformed. After an edit, a stale `G\SHA` is
reported with a suggested stamp edit (ADR-0007); `tmats stamp` applies it on
request. Neither is a cryptographic signature: there is no key.

### Consequences

* Good: D5 and D6 cannot occur by construction; the checksum is free.
* Good: users of `igDisplayTMATS` can compare flex signatures directly.
* Bad: the flex signature carries `irig106lib`'s quirks forever, by design.
