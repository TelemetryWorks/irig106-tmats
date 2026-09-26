---
status: accepted
date: 2026-09-25
decision-makers: Joey
---

# The ordered attribute list is the single source of truth

## Context and Problem Statement

TMATS is a list of `code:value;` items that "may appear in any order" and is
"not case sensitive" (Chapter 9 §9.4.2). Real files carry vendor (V) and
extension (X) attributes, deliberately non-standard attributes, comments,
duplicates, and malformed items. The standard's `G\SHA` checksum covers the
raw bytes (Chapter 6 §6.2.3.11 f), so any change to spelling, order, or line
endings is visible.

The prototype stored parsed values in typed fields and dropped whatever did
not fit; the legacy tools (`irig106lib`) do the same.

## Decision Drivers

* Never lose input (owner: users will "extend the tmats definitions or even
  flat out violate the standards")
* Byte-faithful round trips, so `G\SHA` can be verified and preserved
* Support for new editions without code changes to storage

## Considered Options

* **Ordered list of every attribute as read**, with typed views over it
* **Typed struct per group** plus an "extra" bucket for the rest (the
  prototype's design)
* **Generic tree** keyed by code-name path

## Decision Outcome

Chosen option: **the ordered list of every attribute as read is the single
source of truth**. Each item keeps its code-name span, value span, and a
parsed, case-folded key; typed accessors are views over the list and never
the only copy. Serialization reproduces the input bytes exactly unless the
caller asks for normalization.

### Consequences

* Good: nothing is dropped — unknown, vendor, extension, duplicate, and
  malformed items are preserved and reported.
* Good: `G\SHA` verification and byte-faithful editing (ADR-0007) are
  possible.
* Good: supporting a new edition changes the registry (ADR-0004), not storage.
* Bad: typed access is through generated views rather than plain struct
  fields; compile-time field names are weaker.
* Diagram: `docs/diagrams/document-model.svg`.
