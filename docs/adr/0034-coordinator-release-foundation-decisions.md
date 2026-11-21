---
status: coordinator-decided; owner may override
date: 2026-11-21
decision-makers: coordinator
---

# Coordinator decisions for the 0.1 release foundation

Coordinator decisions, 2026-11-21; the owner may override each:

- W1: keep the library at the repository root and `irig106-tmats-cli/`
  alongside it, matching irig106-time. No move to `crates/`.
- W2: remove the prototype in the restructure change; `prototype-0` keeps
  its history. If the PR is too large to review, split it or return to the
  owner before proceeding.
- W3: offer both a host-callable `run(args, stdout, stderr) -> exit class`
  that never exits the process and individual command/render APIs, with a
  thin `main`, matching irig106-time.
- F1: duplicate SHA items are indeterminate. List every occurrence,
  preserve every byte, refuse stamping until the input is fixed, and never
  claim positive verification. This supersedes ADR-0029's open F1 policy.
- F7: require verified identification of the configuration-change event and
  report indeterminate meanwhile. Never guess a packet type. Its concrete
  identification remains unverified; this decision does not supply one.
