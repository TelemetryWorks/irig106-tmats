# INT-036 and INT-037 owner sign-off — 2026-11-22

Owner: Joey. Under ADR-0033, the owner approved INT-036 and INT-037 as
written in [PR #16](https://github.com/TelemetryWorks/irig106-tmats/pull/16)
at head `891f77f137c9e90ecc15416a84b11d67b6bbde86`, merged at
`3197c685b0b419b2144f4a9dce936ddcfd2c63c5`. The owner relayed that the
independent AI recheck **CONCURRED** on both interpretations at that exact
head. This PR/head and the 2026-11-22 owner relay are the AI-check reference,
not a claim of a separately posted GitHub review.

## Owner choices confirmed against the alternatives

- **INT-036**: TAB and other controls are **discarded** from interpreted
  values per 106-24R1 Chapter 9 section 9.4.1, page 9-2. Never map them to
  spaces: `a\tb` becomes `ab`, not `a b`. Original bytes stay retained and
  human-display renderers use raw text, with explicitly labelled display
  decoding/escaping. Located information findings expose internal-name
  control omission and NUL in fields as specified in #16. Leading/trailing
  name layout remains silent except for the NUL-field rule; layout-only
  regions and EOF padding retain INT-035's approved behavior.
- **INT-037**: strict **7-bit ASCII** for semantic interpretation. A field
  with any high byte has no usable interpreted value: **Invalid**. It does
  not accept Latin-1; a `Temp °C` value containing Latin-1 byte B0 is Invalid.
  ADR-0003's explicitly labelled on-request `as_latin1` and lossy
  (`as_str_lossy`) display views remain available over retained bytes and do
  not validate the semantic value. The assembled-text marker rules, including
  UTF-32 handling, fragment-spanning recognition and markers after padding,
  are approved unchanged as written in #16.

These choices confirm the reviewed text; they add no new interpretation
behavior. The earlier CONCERNS on #15 were addressed by #16 and superseded
by this exact-head concurrence and owner sign-off.

## Approval and remaining work

This records interpretation approval, not implementation, completed deep
analysis or passing executable tests. L2/L3 requirements remain drafted;
TEST-DATA cases remain planned. The trace matrix continues to show no written
full analysis or executable tests for these interpretations. Cross-edition,
handbook/sample analysis and intensive tests remain development gates.

The earlier dated baseline and review-amendment record is preserved as
history. Its pending sign-off/recheck descriptions are superseded by this
record. Remaining family-2 requirements still precede family 3's virtual
workspace restructure under ADR-0035; this sign-off does not complete them.
