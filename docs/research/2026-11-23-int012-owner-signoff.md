# INT-012 owner reconfirmation — 2026-11-23

Joey reconfirmed INT-012 under ADR-0033, 2026-11-23, preserving its original
2026-09-26 owner acceptance and 2026-09-27 ADR-0032 revision. The independent
AI check **CONCURRED** on [PR #34](https://github.com/TelemetryWorks/irig106-tmats/pull/34)
at `e35232391326a4e56e478c5a3a33e10892038b04`, after reading RCCVER tables
of seven archived editions, as relayed by the owner. #34 merged at
`f62cfcc0a2a411ab82e414ddeca18b97a8d665c4`.

The seven-edition check is the reviewer activity reported by the owner, not
an assertion that the maintainer independently repeated that whole audit.
The exact PR/head is the AI-check reference; no separate posted review or
review of this new PR's clarifications is claimed.

## Scope of the approved reading

INT-012 reads the **Format 1** setup-record table, Figure 11-34: a literal
code declaration stays literal, with later archive reuse as a labelled note.
It is not a universal RCCVER/version decoder. Format 4 and packet-header
Data Type Version must keep their own namespaces and source tables.

In 106-24R1 Chapter 11, section 11.2.7.5/Figure 11-43 page 11-62 gives
Format 4's stale table: 0x0C = RCC-106-17, 0x0D–0xFF reserved. The reviewer
referred to Figure 11-41; in this edition that is an index header. The
source-inspected edition-correct Format 4 citation is Figure 11-43.
Section 11.2.1.1 e page 11-6 maps header Data Type Version 0x0A to RCC
106-22; Format 1 RCCVER 0x0A is RCC 106-13. L3-CH10-008 and planned
cross-code-space cases pin that separation.

The rule choices 0x0E -> 106-24R1 and 0x0D -> 106-20 are ADR-0032/INT-017
fallback POLICY, not INT-012's declaration reading. Keep tests/bindings
separate. This reconfirmation clears no INT-017 review or future Format 4
interpretation gate. It also supplies no general historical-layout policy.

## Remaining gates

Semantic approval is complete for INT-012; full development analysis,
source freshness and executable tests remain. Dated earlier source-review
records are preserved as history with their pending-review status superseded
by this record. New analysis notes/cases still need exact-head review.
