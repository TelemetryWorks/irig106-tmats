---
status: accepted
date: 2026-09-25
decision-makers: Joey
---

# A layered, spec-cited, edition-tagged attribute registry

## Context and Problem Statement

Chapter 9 defines each attribute in a table with seven "Usage Attributes"
fields: R/R Ch 10 Status, Allowed when, Required when, Links to, Links from
(which makes a value a key that must be unique), Range, and Default
(§9.5.1 a). Definitions change between editions (106-22 added the Q group;
106-24 changed the CRC parameter in Table 9-6). Users need definitions the
standard lacks — vendor and extension attributes, program conventions, and
deliberate violations.

The prototype hand-wrote a partial registry (about 30 entries) that
disagreed with its own TOML source (44 entries) and with the standard, and
its validator ignored it.

## Decision Drivers

* Every definition traceable to an edition, table, and row
* One definition set drives views, links, and validation
* Multiple editions without separate copies
* Users can add and override definitions

## Considered Options

* **Built-in registry transcribed from the Chapter 9 tables, each entry citing
  its source and tagged with the editions that introduced, changed, or removed
  it; a user overlay layered on top and consulted first**
* Hand-written rules per attribute
* One registry per edition

## Decision Outcome

Chosen option: **the layered, spec-cited, edition-tagged registry**. Each
entry records the seven usage-attribute fields for its code-name pattern,
with a citation. The baseline edition is 106-24R1; entries carry edition tags
back to 106-04. The user overlay (loaded from a file or built in code) adds
or overrides entries.

### Consequences

* Good: views, the link graph, and validation share one definition set
  (ADR-0006).
* Good: a new edition is a data change plus its citations.
* Good: extension is first-class rather than a patch.
* Bad: transcribing the tables is substantial, careful work; an extraction
  script (`pdftotext -raw` yields one table row per line) reduces it but
  every row is still checked against the PDF.
* Generation mechanics: ADR-0005. Edition selection: ADR-0016.
