# INT-035 owner sign-off — 2026-11-21

Owner: Joey. Under ADR-0033, the owner approved INT-035 as written in
[PR #13](https://github.com/TelemetryWorks/irig106-tmats/pull/13) at head
`e622311690fc5812f689c9d77a19e21447aab6fd`. The owner also relayed that the
independent AI check on that interpretation **CONCURS**; this PR/head and
the 2026-11-21 owner relay are the AI-check reference, not a claim of a
separately posted GitHub review.

## Additional owner decision

Empty or layout-only **terminated** regions produce one information-level,
located finding per maximal consecutive run, with one merged source span.
Thus `;;;;` produces one finding covering the four bytes, not four findings.
A nonempty attribute or non-layout malformed region ends the run. An empty
value such as `X:;` is not an empty region; `:v;` is an empty-code-name case.
Unterminated layout or padding at EOF stays silent per Chapter 9 section
9.4.1. Retain every source byte in all cases.

This settles the open finding-versus-silence question in the earlier draft.
It records approval, not implementation, complete analysis or passing tests.

## Reviewer notes to carry forward

- Appendix 9-E example text is not a conforming-file fixture: trailing
  annotations after a semicolon become the next region's code-name bytes.
  Use isolated, cited attributes or explicitly nonconforming excerpt tests;
  do not label an annotated example block a valid TMATS file.
- Plan a suggestion-only suspect-terminator finding for a line break inside
  a value followed by a complete code name and colon. It must not split the
  attribute or apply an edit. Derivation, precise matching and tests remain
  future work; no implementation or new interpretation approval is claimed
  for that planned diagnostic.

The earlier baseline/concerns draft remains historical. Its pending review
and open-question descriptions are superseded by this sign-off record.
Character/layout prerequisites, full analysis and executable tests still
remain outstanding; approval is not evidence that the library implements it.
