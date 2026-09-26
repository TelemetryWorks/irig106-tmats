---
status: accepted
date: 2026-09-25
decision-makers: Joey
---

# The library performs no I/O

## Context and Problem Statement

TMATS arrives as a standalone file, as the setup-record payload of a Chapter
10 recording, and potentially over a network. Writing goes to files, to other
programs, and into new recordings. The question was whether one library call
should accept a whole Chapter 10 file.

The owner's answer: "The library should not, the simple test CLI should be
able to open the file enought to read the tmats packet or a tmats file."

## Decision Drivers

* The same calls must work in the CLI, in `irig106-studio`, in tests, and on
  data that never touches a disk
* Packet reading belongs to `irig106-core`; packet writing to `irig106-write`

## Considered Options

* **The library reads from bytes and writes to bytes or a caller-supplied
  writer; callers own files**
* The library also opens and writes files

## Decision Outcome

Chosen option: **no I/O in the library, in either direction**. The `tmats`
CLI opens recordings far enough to find setup-record packets (ADR-0019) and
writes output files. Writing TMATS text is in 0.1 (the library produces the
bytes); creating or modifying content is 0.5; putting TMATS into a recording
is a setup-record payload from the library plus packet writing in
`irig106-write`. See `docs/USE-CASES.md`, "The filesystem boundary".

### Consequences

* Good: the library is usable anywhere, and trivially testable.
* Good: memory-mapping and file-safety policy live in one place per tool.
* Bad: every consumer that starts from a file needs a small amount of glue
  (or uses the CLI).
