---
status: accepted
date: 2026-09-25
decision-makers: Joey
---

# Remove XML, WASM bindings, and the unused `std` / `rich-errors` features until they can be done properly

## Context and Problem Statement

The prototype advertised features that did not deliver:

- **XML:** an invented `<Tmats>` mapping, not RCC's TMATS XML schema; it lost
  data on a round trip.
- **WASM:** `wasm-bindgen` bindings inside the core crate, without the
  `cdylib` crate type needed to build a module.
- **`std`:** declared, but `std` was used unconditionally.
- **`rich-errors`:** pulled in `miette` with its `fancy` renderer, used by
  nothing.

The owner chose: "Drop XML until real XSD, Move WASM out, Remove fake
features", and asked that they be captured "so I do not forget about them and
what they were for".

## Decision Drivers

* Publish only what works and is faithful to the standard
* Keep the core crate's dependency graph small

## Considered Options

* **Remove them now; record each in the ROADMAP's "Deferred features" with its
  purpose and what bringing it back requires**
* Keep them behind "unstable" feature flags

## Decision Outcome

Chosen option: **remove and record**. XML returns when generated from the
official XSDs (archived in `rcc-106-standards` from 106-07 onward); WASM
returns as a separate `irig106-tmats-wasm` crate; `no_std` returns only with a
real `alloc`-only build verified in CI; rich diagnostics return in a binary,
not the library. Until XML is supported, a setup record whose CSDW marks an
XML body is recognised and reported as unsupported rather than misparsed.

### Consequences

* Good: every published feature works.
* Good: nothing is forgotten; the ROADMAP says what each was for.
* Bad: users who need TMATS XML today are not served.
