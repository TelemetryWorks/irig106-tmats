# INT-021/F1 and INT-030/F7 owner sign-off — 2026-11-23

Owner: Joey. Under ADR-0033, Joey approved **F1/INT-021** and **F7/INT-030**
on 2026-11-23. The independent AI check **CONCURRED on both** at
[PR #30](https://github.com/TelemetryWorks/irig106-tmats/pull/30) head
`13173b04faac989fa581b8e9ebbd0df6c80d6b98`, as relayed by the owner.
That is the exact-head AI-check reference, not a claim of independent review
of later amended wording or a separately posted GitHub review.

## Approved policies

F1: duplicate G\SHA items make verification indeterminate. Preserve every
byte, list every occurrence and never select a winner or verify positively.
Refuse stamping until the ambiguity is fixed. The explicit ADR-0029 edit
removing one exact revision-bound item remains allowed; rebuild/re-read the
result and stamp only afterwards, potentially as the transaction's last step.
The recorded compatible-damaged-name uncertainty guard remains; this approval
does not turn damaged evidence into a usable checksum key.

F7: require verified identification of the configuration-change event and
report indeterminate meanwhile. A nearby event is only a candidate, never
proof of compliance. Approval supplies no packet identity and does not
complete L1-CH10-005's event-presence check. First-record SRCC reporting,
fragment SRCC disagreement and the meaning of prior remain separate analysis
notes; no fragment aggregation or ordering interpretation is chosen here.

## What this sign-off changes

The owner approved the existing coordinator recommendations, including the
explicit exact-item repair and candidate-only clarifications. Their design
and Review statuses are now approved, with coordinator origin retained.
This record supersedes pending-owner descriptions in the earlier gate note,
which is preserved as history. It does not change ADR-0033's review rule or
claim completed development analysis, source freshness or executable tests.

## Remaining checksum and implementation gates

INT-018, INT-019 and INT-020 still have their historical owner acceptance but
need independent AI checks before implementation; the owner will route them.
Neither F1/F7 approval nor their AI concurrence clears those separate checks.
Requirements and acceptance cases remain Draft/planned. Source-hash freshness,
completed deep analysis and required executable verification remain additional
implementation gates for both approved policies. No code or tests are delivered
by this sign-off record; the amended PR still needs exact-head independent review.
