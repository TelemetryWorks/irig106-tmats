---
status: accepted
date: 2026-09-25
decision-makers: Joey
---

# Mirror the RCC 106 standards in a separate repository, as release assets

## Context and Problem Statement

Every registry entry, rule, and test must cite an edition, table, and row
(ADR-0004). The source documents live on sites that move: the WSMR RCC site
is already retired, and irig106.org serves some broken links. The owner asked
to "capture all the RCC 106 Telemetry Standards ... But not in the TMATS repo
... Make sure we preserve the original links as well."

## Decision Drivers

* Citations must keep working
* Provenance: where every file came from
* Clones and CI must stay light

## Considered Options

* **A dedicated public repository whose git tree holds a manifest (all
  original URLs, SHA-256, sizes, dates, distribution marking) and a fetch
  script, with the files as GitHub Release assets, one release per edition**
* The same repository with files in Git LFS
* Files inside this repository

## Decision Outcome

Chosen option: **`TelemetryWorks/rcc-106-standards`, public, release assets**
(owner: "New repo + Release assets", "TelemetryWorks/rcc-106-standards,
public", "All editions, full + chapters", then the extension with RCC
118/119, schemas, pink sheets, and TmNS companions, excluding the 106-96
installer). The documents are marked "DISTRIBUTION A: APPROVED FOR PUBLIC
RELEASE" where a marking exists. Non-RCC standards (MIL-STD-1553B, ARINC 429,
IEEE 1588, IEEE 754, FIPS 180-4) are cited, not mirrored.

### Consequences

* Good: 735 documents across 47 releases, each traceable to its original URLs
  and verifiable by checksum (`scripts/fetch.py`).
* Good: no LFS quotas; this repository stays small.
* Bad: the archive is maintained by hand when RCC publishes a new edition
  (procedure in its README).
