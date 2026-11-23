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


## Combined reconfirmation: INT-018/019/020 — 2026-11-23

Joey reconfirmed INT-018, INT-019 and INT-020 under ADR-0033, preserving
original owner acceptance dated 2026-09-26. Independent AI CONCUR on all
three is tied to the register review at main
`30e42a3eac791f2f794e5fe239c59590eb6682b8`, as relayed by the owner.
This supersedes their pending-AI descriptions above and removes only their
0.1 checksum interpretation-review blockers. F1/INT-021 and F7/INT-030
remain approved with their separate AI reference at #30 head 13173b0.
No approval of later follow-up wording or this amended PR is claimed.

INT-018 recognises the projected complete name but excludes raw bytes,
keeping leading layout in the hash. INT-019 malformed/truncated exclusions
also refuse authoritative COMPUTE. INT-020 permits case-insensitive digest
comparison with its warning, keeps omission information and reports the
zero placeholder unverified; blank handling remains unavailable. The
reviewer's notes and planned cases live in the register and TEST-DATA.
INT-043's proposed 02- handling is not included in this reconfirmation.

The [checksum derivation note](2026-11-23-checksum-followups-draft.md) records
sources, the proposed designator choice and independent hashlib preimages.
Freshness, completed analysis and executable tests remain implementation
gates. The earlier history is preserved; semantic approval is not completion
of family 2 or authority to skip development evidence.


## Follow-up AI review relay — 2026-11-23

The owner relayed AI CONCUR on INT-018/019/020 follow-up wording in PR #31
at `b44967341b12425d4380778cf5f7971b211fdb19`, except the COMPUTE eligibility
rule. That exception is corrected separately: refuse only for undefined or
ambiguous domain; a complete terminated malformed VALUE permits COMPUTE.
The placeholder follows that same rule. INT-043's label received CONCERNS
and remains proposed with the revised unsupported-algorithm wording. This
records the relay, not an approval of the newly amended head or completed
analysis/tests; the original owner/AI approval references remain unchanged.
