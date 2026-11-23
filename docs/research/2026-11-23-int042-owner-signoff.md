# INT-042 owner sign-off — 2026-11-23

Owner: Joey. Under ADR-0033, the owner approved **INT-042 Option A as
amended in [PR #28](https://github.com/TelemetryWorks/irig106-tmats/pull/28)**
at head `d69d275340d1773d9548cf365d57db8fc05822f1`, merged at
`9a3531cea5de17581f4739465c6b93efddb8f9f8`. The owner relayed that the
independent AI recheck **CONCURS with amended Option A**. The PR/head and
2026-11-23 owner relay are the AI-check reference; this is not a claim of
a separately posted GitHub review or review of this new sign-off-record PR.

## Approved interpretation

Actual exact matches establish physical presence even with empty, Invalid or
Ambiguous values; keep every item and independent finding. Zero matches
means false only without compatible related damaged evidence in the same
document/setup record. Related suffix/name damage gives cannot-evaluate,
never a manufactured match or false absence. A default is not supplied text.
Value comparisons still require usable operands. Negated presence flips
true/false only, preserving cannot-evaluate. Canonical name/index matching
retains raw bytes and normalization findings; evidence from another record
never contributes. The INT-040 compatible-scope guard applies; INT-035
framing and the other previously approved interpretations remain unchanged.

## Clarifications requested with sign-off

1. **A proper literal prefix at EOF blocks false.** With no exact T-1 ID
   item, `COMMENT:x;T-1\I` or `COMMENT:x;T-1\` is cannot-evaluate for
   predicates whose names that suffix could complete to, subject to readable
   parent compatibility. Do not judge T-1\AN1 not allowed from that unknown
   presence condition. No suffix becomes a lookup item. Readable conflicting
   literal/parent facts exclude incompatible predicates; check a requested
   predicate without enumerating hypothetical completions.
2. **Damaged names are not just a trailing-byte case.** Uninterpretable name
   bytes prevent an exact key while readable literal/parent facts remain
   compatible with the declared family. A high byte anywhere in the literal,
   such as `T-1\I\xFFD:a;`, blocks false just as `T-1\ID\xFF:a;` does.
   Preserve bytes and locations; do not remove high bytes to make a usable
   key. Controls are already handled by the approved INT-036 projection:
   `T-1\I\r\nD:x;` matches ID, with its omission information retained.

These are diagnostic compatibility guards, not arbitrary prefix matching of
complete printable names, value-text searches, source repair or new default
rules. Known exact presence remains true alongside additional damaged evidence.
The new planned cases record these clarifications; the prior-head concurrence
does not claim independent review of their wording in this new PR.

## Source-context caveat

[106-24R1 Chapter 9](https://www.trmc.osd.mil/wiki/download/attachments/335389015/Chapter9.pdf?api=v2),
section 9.5.1 a, printed page 9-6, states:

> If the attribute is specified in the TMATS file, it must contain valid information.

This sentence sits in the **R/R Ch 10 Status bullet**. It is strong textual
evidence separating specification from the validity obligation in that context,
not a general definition of specified or an explicit damaged-input truth table.
Table 9-3 TRANSMIT ANTENNA TYPE, page 9-18, supplies the cited presence
condition. Option A's recovery semantics come from the reviewed interpretation
and owner decision, not a claim that Chapter 9 states that recovery algorithm.

## Remaining evidence

Approval records the semantic decision, not implementation, completed deep
analysis or passing tests. L2/L3 requirements remain Draft; TEST-DATA cases
remain planned. Full analysis and executable verification remain development
gates. The earlier condition-scope draft is preserved as history; its pending
sign-off and Option A recommendation are superseded by this approval record.
Family 2's remaining requirements precede family 3's ADR-0035 restructure.
