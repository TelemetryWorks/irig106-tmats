# Scanner framing and conservative recovery — 2026-11-21

Draft analysis for INT-035 and the family-2 framing requirements. This is
not an owner sign-off, independent review result or executable-test report.

## Source and scope

Inspected 106-24R1 Chapter 9 section 9.4.2, printed page 9-2, with COMMENT
on page 9-3. The official PDF defines colon-separated code/value pairs,
semicolon termination, no semicolons inside data items, case-insensitive
TMATS and freely ordered attributes. Chapter 9 section 9.4.1, page 9-2,
separately discusses nonprintable characters and CR/LF for readability.

[Official Chapter 9](https://www.trmc.osd.mil/wiki/download/attachments/335389015/Chapter9.pdf?api=v2)
([archive](https://github.com/TelemetryWorks/rcc-106-standards/releases/download/106-24R1/Chapter9.pdf)).
Baseline archive SHA-256:
`dfc5ebfb3e6f02f58389126863d43a45d89c7e2d0533a1c690d9537531c740cb`.

The valid-input grammar does not define a malformed-input recovery algorithm.
Lossless recovery is the product policy in L1-READ-001/003 and ADR-0002.
INT-035 therefore labels semicolon-based recovery as a proposal, not as a
normative standard instruction to repair damaged configuration data.

## Alternatives and failure modes

- Aborting on the first malformed region hides later useful data; silently
  returning empty success repeats the prototype defect.
- Searching past a semicolon for a colon borrows a separator from another
  region and can convert missing punctuation into an apparently valid item.
- Splitting on code-looking text inside values invents a missing semicolon;
  colon-bearing values are allowed by the accepted L1-READ-006 contract.
- Semicolon-bounded recovery preserves all bytes and gives a predictable
  next boundary, but does not reconstruct a writer's intended attributes.
  A missing semicolon can leave several intended items in one raw value.

Propose the last approach, with located findings and no silent repairs.
An unterminated non-layout suffix remains raw material with a finding;
classifying permitted layout is a separate character-rule prerequisite.
Do not define quote/escape syntax to hide semicolons from the byte scanner.

## Planned edge coverage and remaining work

TEST-DATA.md's batch-2 matrix covers missing colon, missing terminator,
extra colons, code-like value text, nonconforming quoted semicolons, empty
input, delimiter-only runs and arbitrary bytes. Compare retained source to
an independent original, not a reconstructed typed model. No tests were run.

Only the baseline text was checked here. Cross-edition confirmation and
relevant RCC 123/124 handbook/sample investigation remain outstanding under
the interpretation register's development obligations; no historical-edition
or real-recording evidence is claimed. Nonprintable-character meaning,
full code-name validity, duplicate rules and L1-READ-007's suspect findings
need later batches. Owner sign-off and independent AI review are pending;
this analysis does not authorize implementing INT-035 yet.

## Owner sign-off request and implementation gate

In plain English: keep malformed bytes and explain what is wrong; move on
only when an actual semicolon gives the next boundary. Never guess that
code-looking text inside a value begins a new attribute, and never invent a
missing terminator at EOF. This is conservative recovery, not automatic
repair or a guarantee that later metadata describes the recording correctly.

The requested sign-off concerns this proposed product behavior, not a claim
that every edition's grammar or every malformed-file case has been verified.
The independent AI check and owner sign-off must be recorded under ADR-0033.
Full analysis, character/layout prerequisites and the specified tests remain
implementation gates. Keep **Analysis** at `not yet written` in the register
until the complete required analysis is available; this file is a draft.
