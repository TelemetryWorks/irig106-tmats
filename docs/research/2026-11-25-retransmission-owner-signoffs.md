# Retransmission matrix owner sign-offs and stricter QD precedence

Date: 2026-11-25. Owner: Joey. Source: owner decisions relayed in this
repository thread after merged #44 at `81b630499b0acabb5f7b8e9b4d7bb68de579b999`
(main merge `43ddcdf3a504d5c31e9fddd6f7a5208af0dccfbb`).

## Owner wording (verbatim)

> Q0 ACCEPTED as recommended: inconsistent observations are rejected or held, with named consistency/rejection tests; they stay listed and counted but never count as parser coverage.
>
> QD STRICTER RULE, NOW (owner overrode the recommendation): a duplicate G\SHA (or other field ambiguity) forces assembly Ambiguous, effective immediately, not deferred. The qd_proposal column becomes the approved behaviour.
> - Byte identity, equality or a digest must NEVER upgrade that Ambiguous to Complete. This interacts with the QI identity exception; make the interaction explicit and regenerate the matrix.
>
> QI ACCEPTED: the identity exception applies only to bounded, otherwise-Complete candidates (time seal, same channel, sequence +1, same FRMT/RCCVER), independent of following SRCC 0/1/U. The INT-045 premature-seal finding stays visible, including on the Complete outcome and in duplicate-SHA cases.
>
> QB/QE ACCEPTED: no automatic extension; raw identity is kept as information only.
>
> QC/QP ACCEPTED: report the raw change subtype, with no speculative semantic reset; an incomplete comparison stays unresolved.
>
> QH/QG ACCEPTED: prominent integrity-finding mode by default; strict quarantine is opt-in, only after its own review. Do not waive F1, framing, trust or ambiguous scope.
>
> QS ACCEPTED: keep equality as evidence; withhold automatic eligibility or governing inference; do not infer the bit.
>
> QM: NOT final yet. Owner direction pending; I'll send it separately. Leave QM as out of scope for now.

## Register mapping and review gate

| Question | Owner-signed interpretation | Consequence |
| --- | --- | --- |
| QI | INT-046 | Bounded identity exception; keep the INT-045 finding; QD dominates |
| Q0 | INT-047 | Reject/hold contradictions without false parser coverage |
| QD | INT-048 | Immediate assembly Ambiguous for field ambiguity, not a deferred proposal |
| QB/QE | INT-049 | No automatic boundary/profile extension |
| QC/QP | INT-050 | Explicit raw changes; incomplete comparison unresolved |
| QH/QG | INT-051 | Prominent findings default after eligibility checks; quarantine needs its own review |
| QS | INT-052 | Equality evidence does not infer unreadable SRCC or governing eligibility |
| QM | No new decision | Multiple-copy comparison/composition stays outside this pairwise matrix |

These record the owner's human sign-off under ADR-0033. The independent AI
check of this amended wording is PENDING; no concurrence is claimed and no
implementation is authorised by owner sign-off alone. Source freshness,
written development analysis and executable whole-matrix tests still apply.
The documentation generator is not parser/runtime verification.

The former `qd_proposal` is promoted into the current matrix baseline; the
less-strict baseline in #44 is retained in Git history, not as an active option.
Strict QD is candidate-local and applies before identity/digest exceptions.
Cut evidence remains visible if a known duplicate makes aggregate Ambiguous.
Other established field ambiguity is represented by input state A; simultaneous
cut/other-field cases require named byte fixtures, not an inferred cardinality.
No new dimension or multiple-copy rule is silently introduced.

## Documentation artifact delta from merged #44

The product is unchanged at 139,968 members. Full-output classes change from
39,078 to 39,072 (six fewer); IDs hash outputs, so class identities are revised.

| Sealed outcome change | Members | Exact coherent input class |
| --- | ---: | --- |
| Complete → Ambiguous | 5,040 | supplied T, sealed SHA D, without the qualifying time/C/SS follower guard |
| Incomplete → Ambiguous | 3,456 | supplied X, sealed SHA D; QD now dominates while the cut facet remains visible |
| Other sealed outcome changes | 0 | Existing A stays A; all 30 QI identity outcomes stay Complete; R0 holds unchanged |

The 8,496 changed sealed classifications plus 24 QI baseline promotions make
8,520 changed baseline verdicts. Those 24 already had a Complete proposed
outcome in #44; QI now makes that the owner-signed baseline. A new conditional
following-assembly column explicitly marks all 13,824 coherent following-D
members Ambiguous. No following raw-comparison subtype changes. All 59,184
R0 members retain their classification and nominal test name, never parser
coverage. Findings are retained, including INT-045's finding on identity rows.

These are documentation comparisons and coverage audits, not executed parser
or CLI tests, completed development analysis or independent AI concurrence.
