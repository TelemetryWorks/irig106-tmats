---
status: accepted; pipeline extended by ADR-0022
date: 2026-09-25
decision-makers: Joey
---

# Generate the registry into checked-in source with a script; no `build.rs`

## Context and Problem Statement

The prototype's `build.rs` generated a registry from `data/attributes.toml`
on every build, but the `include!` that would use it was commented out. Had it
been enabled, it would have silently disabled validation: it doubled
backslashes, so every `G\PN` lookup would miss and every registry-driven rule
would return no findings. The owner asked, "Do we need the build.rs in this
repo?"

## Decision Drivers

* Downstream crates should not compile and run a build script to use a
  library
* Generated code must be visible in code review and on docs.rs
* Drift between source data and generated code must fail CI

## Considered Options

* **A script under `scripts/` that writes a checked-in source file, with a
  `--check` mode in CI**
* `build.rs` generating into `OUT_DIR` (the prototype)
* Hand-written tables

## Decision Outcome

Chosen option: **a generator script with a checked-in output and a CI
`--check`**, the same pattern as the trace matrix. `build.rs` is deleted when
the prototype is removed. The generated tables are static (sorted arrays or a
perfect hash), so the registry has no runtime set-up cost.

### Consequences

* Good: no build script for dependents; generated code is reviewable.
* Good: a stale generated file fails CI rather than silently diverging.
* Bad: contributors must run the script after changing registry data (the CI
  check tells them).
