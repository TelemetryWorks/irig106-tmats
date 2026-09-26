---
status: accepted
date: 2026-09-25
decision-makers: Joey
---

# Hand-rolled argument parsing for the `tmats` CLI

## Context and Problem Statement

`tmats` has a small, stable command set (`show`, `extract`, `checksum`,
`verify`, `stamp`, later `validate` and `diff`). The owner's decision: "Go
with hand-rolled argument parsing like mie-decoder" — the sibling project that
hand-rolls argument parsing, CSV, TOML, logging, and errors to keep its
dependency footprint minimal.

## Decision Drivers

* No dependency, and so no MSRV or semver churn from one
* Full control of every help and error message (needed for ASCII-only output)
* A small binary

## Considered Options

* **Hand-rolled, table-driven parser**
* `clap` (derive or builder)
* `lexopt` or a similar minimal crate

## Decision Outcome

Chosen option: **hand-rolled**. No argument-parsing crate is added. The
parser is table-driven, and every flag, value, and usage error has a test.

### Consequences

* Good: the CLI's only dependency is the library.
* Bad: no generated shell completions; more parsing code to own and test.
