# INT-041 owner sign-off — 2026-11-23

Owner: Joey. Under ADR-0033, the owner approved INT-041 as written in
[PR #26](https://github.com/TelemetryWorks/irig106-tmats/pull/26) at head
`0fc0bc8917d4e80d1c0c3274bc8868083afb1a28`, merged at
`98f4d816f0f16394c48620566c5e09b072d2ea02`. The owner relayed that the
independent AI check **CONCURS**. This PR/head and the 2026-11-23 owner relay
are the AI-check reference, not a claim of a separately posted GitHub review.

## Approved interpretation

Unavailable reviewed interpretation or undecidable local validity reports
an unavailable/cannot-evaluate read with its reason and raw evidence. It is
not proof of Missing or Invalid data. Preserve established Ambiguous or
proved Invalid outcomes. For a supported field actually absent, lack of a
verified applicable default remains Missing with its precise reason. No
sixth semantic value state or concrete Rust result shape is selected here.

## Edge cases requested with sign-off

1. `R-1\TFMT-3:a;` is locally Explicit A, keeping raw `a`. 106-24R1 Chapter
   9 section 9.4.2, page 9-2, supplies case-insensitive meaning; this is not
   source normalization.
2. `R-1\TFMT-3: A;` is unavailable until a value-blank rule is reviewed,
   NOT automatically Invalid. Section 9.4.2 treats blanks as intentional.
   Do not silently trim the blank or default over the present field. This
   note approves no new trimming/rejection policy and does not alter the
   separate code-name-edge blank handling.
3. A present TFMT on a non-TIMEIN channel is Explicit locally with an
   applicability finding, never ignored as absent. Local type/range usability
   and condition validity remain separate observations. Baseline Table 9-4
   TIME FORMAT, page 9-72, has TIMEIN applicability; CHANNEL DATA TYPE,
   page 9-50, defines TIMEIN and PCMIN used by the planned fixture.
4. With two R-1\CDT-3 items and TFMT absent, the CDT operand is Ambiguous
   under INT-039, even if both say TIMEIN. TFMT is Missing with an unknown-
   condition reason and cannot-evaluate finding carrying both CDT candidates,
   not Defaulted A or a proven false condition.
5. Consumers must read the Missing reason. A reason of default applicability
   not established carries uncertainty; it must not be mistaken for a proved
   false condition or confirmation that a default is inapplicable. User-facing
   README/release-contract notes carry this distinction.
6. An Invalid verdict under a fallback edition retains that fallback label,
   applied edition/citation and basis reason. An invalidity finding does not
   turn a fallback into a declaration or erase the uncertainty of the basis.

The independent concurrence above applies to #26's exact head. These edge
notes were requested with owner sign-off; this record does not claim their
new wording or this new PR has already been independently reviewed.

## Remaining evidence

Approval records a semantic decision, not implementation, completed deep
analysis or passing tests. Requirements remain drafted; TEST-DATA cases are
planned. Full cross-edition/handbook/sample analysis and executable verification
remain development gates. The earlier dated effective-value draft is preserved
as history; its pending sign-off descriptions are superseded here. Remaining
family-2 batches still precede family 3's ADR-0035 restructure.

## Full blank sentence — owner-requested addition, 2026-11-23

106-24R1 Chapter 9 section 9.4.2, page 9-2, states in full:

> Leading, trailing, and embedded blanks are assumed to be intentional; they can be ignored in most cases but should not be used in code names, keywords, and data items used as links, such as measurement name.

This addition supplies the complete source for a future blank-rule review;
it approves no new value trimming, acceptance or rejection policy. The
existing INT-041 sign-off and its exact-head AI-check reference above stand.
