---
status: accepted
date: 2026-09-25
decision-makers: Joey
---

# Owned byte buffer with spans instead of borrowed lifetimes

## Context and Problem Statement

The prototype borrowed from the caller's input (`Cow<'a, str>`) and stored
path segments in `SmallVec`. Lifetimes spread through every public type,
`SmallVec` made the model invariant over `'a` (the query API needed split
lifetimes to compile), every struct needed `into_owned` boilerplate, borrowed
serde deserialization did not work, and the parser allocated anyway
(upper-cased copies, `format!`). TMATS documents are kilobytes to a few
megabytes and are read once per recording.

## Decision Drivers

* Simple public types without lifetime parameters
* Zero-copy access to values without re-parsing
* Values stay bytes: real files contain non-UTF-8 bytes (for example a
  Latin-1 degree sign)

## Considered Options

* **One owned buffer plus `(start, len)` spans** held by the document
* **Borrowed `Cow<'a, str>`** (the prototype)
* **Owned `String` per value**

## Decision Outcome

Chosen option: **the document owns one byte buffer, and items refer to it by
spans**. Values are bytes; decoding (`as_str_lossy`, `as_latin1`, typed
parsing) happens at the view, on request.

### Consequences

* Good: no lifetime parameters on public types; the document is `Send + Sync`
  and can be shared.
* Good: no per-value allocation, and no UTF-8 failure mode.
* Bad: the caller's bytes are copied once into the document (negligible at
  TMATS sizes).
* Span arithmetic is guarded by newtypes (`ItemId`, `Span`) and non-panicking
  access (ADR-0020).
