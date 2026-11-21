---
status: accepted
date: 2026-11-21
decision-makers: Joey
---

# Virtual workspace with a library crate and CLI app

Owner decision, 2026-11-21: use a virtual workspace modelled on
[Praevis-Quant/praevis-thetadata](https://github.com/Praevis-Quant/praevis-thetadata).
This supersedes only W1 in ADR-0034 and matches the owner's 2026-09-26
[section 25 direction](../research/2026-09-25-owner-direction.md#25-the-cli-as-a-reusable-library-the-workspace-layout-f5-explained-2026-09-26).

- `crates/irig106-tmats/` holds the library.
- `apps/irig106-tmats-cli/` holds the CLI, intended for testing and simple
  examples; the full documented 0.1 scope and W3 remain unchanged.
- The root Cargo.toml is virtual: workspace configuration only, no root
  package, with members, a shared lockstep `[workspace.package]` version and
  `[workspace.dependencies]` declaring the exact-version library path dependency.

W2 (prototype removal in the restructure change) and W3 (both host runner
and individual command/render APIs with a thin main) are unchanged.
