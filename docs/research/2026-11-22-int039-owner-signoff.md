# INT-039 owner sign-off — 2026-11-22

Owner: Joey. Under ADR-0033, the owner approved INT-039 as written in
[PR #22](https://github.com/TelemetryWorks/irig106-tmats/pull/22) at head
`b22d45c4fb49c7b1a43ab6a4fd0a9d142c667f28`, merged at
`b8a3ea71a800415aab9a68af55b60adb3df31687`. The owner relayed that the
independent AI check **CONCURS** on INT-039. This PR/head and the 2026-11-22
owner relay are the AI-check reference, not a claim of a separately posted
GitHub review.

## Owner choices confirmed

Repeated single-entry fields are Ambiguous, with a duplicate finding listing
every occurrence, including identical repeats. Never select the first or last,
coalesce equal values, filter an invalid candidate to keep a valid one, or
default over multiplicity. Raw lookup retains all matching items and lines.
Unknown cardinality and candidate-specific findings keep their documented
meaning; normalization information and F1's integrity rule remain unchanged.

Standalone COMMENT is a retained ordered collection. Group `\COM` fields
are different: `G\COM`, `R-x\COM`, etc. are single-entry per the standard,
so repeated instances of the same slot are duplicates, not a comment
collection. Baseline sources are 106-24R1 Chapter 9 section 9.4.2, pages
9-2/3, Table 9-2 COMMENTS `G\COM`, page 9-13, and Table 9-4 COMMENTS
`R-x\COM`, page 9-84. Raw lookup still returns every line.

The owner knowingly chose the standard's single-entry treatment despite some
writers splitting long remarks across repeated fields. The alternative was
treating those repeated group comments like standalone COMMENT. That was not
chosen: it would hide duplicate declarations and introduce an uncited joining
or collection convention. Preserve the remarks and their locations without
joining, discarding or choosing a line; the user may explicitly fix the input.
One attribute whose value merely contains a line break is not several fields;
INT-035/036 framing and projection remain unchanged.

## Assembly-scope clarification requested with sign-off

One Format 1 setup record spanning several packets is ONE document after
ADR-0025 assembly. Each packet's CSDW/framing is excluded and the text fragments
are joined with provenance before duplicate grouping. G\PN in fragment 1 and
again in fragment 2 is therefore a scalar duplicate, with both assembled
locations and packet provenance retained. A packet boundary does not reset
candidate grouping. Separate successive setup records remain separate
records/documents; counting duplicates does not decide their completeness or
which record governs a packet.

The independent concurrence above applies to #22's exact head. The group-\COM
confirmation and assembly clarification were requested with owner sign-off;
this record does not claim that their new wording or this PR has already
received independent review.

## Remaining evidence

Approval records the interpretation decision, not implementation, completed
analysis or passing tests. L2/L3 requirements remain drafted and TEST-DATA
cases remain planned. Cross-edition/handbook/sample analysis and intensive
executable tests remain development gates. The earlier dated scalar-duplicate
draft is preserved as history; its pending sign-off descriptions are
superseded by this record. Remaining family-2 requirements still precede
family 3's ADR-0035 restructure.
