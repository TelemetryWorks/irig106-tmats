# Caller-choice owner answers for PR #52

Date: 2026-11-26. Owner: Joey. Owner-signed INT-056–060 under ADR-0033;
independent AI check of the drafted entries is pending. These supersede the
unsettled caller-choice policies in the original batch-20 proposal; earlier
research records remain history. No code, executable tests or implementation
approval is claimed.

## Owner answers (verbatim)

> 1. The chosen target has bad data (Invalid, duplicate or unavailable ID/format): THREE caller-selectable levels.
>    - Strict: refuse the choice, with a typed reason.
>    - Warn (DEFAULT): accept as a labelled assumption, with all bad-data findings attached.
>    - Quiet: accept as a labelled assumption with NO warning findings emitted.
>    Even in Quiet:
>    - the result is still labelled a caller assumption;
>    - the target's own states (Invalid / duplicate / unavailable) stay as they are and are never upgraded;
>    - the selected level is recorded in the result, so suppression is visible to anyone inspecting it.
>    One fixture per level, for each bad-data kind.

> 2. A: refuse a choice that is not a scoped TIMEIN candidate in the same recorder (a non-TIMEIN entry, another recorder, or a nonexistent ID). Typed refusal, with a fixture.

> 3. A: refuse a choice that contradicts evidence. The owner wants the contradiction rules made explicit. State that a contradiction exists ONLY when an owner-approved evidence rule (its own INT) establishes a different association for that data channel. No evidence rule exists in 0.1, since Chapter 9 has no data-to-time field, so this cannot trigger yet. Every future evidence source (e.g. a later edition's field, or a reviewed packet-level rule) needs its own owner-signed INT defining what it proves. Heuristics (order, only candidate, TSRC, DSI, NSB, observed time packets) are never evidence.

> 4. A: several unproven candidates means "unestablished"; "conflicting" is reserved for competing evidence.

> 5. A: no remembered or session-wide default; each query carries the choice explicitly.

## Interpretation map and boundaries

- INT-056: Strict/Warn/Quiet for bad target ID/format. Warn is the DEFAULT.
  Quiet emits no warning findings on the assumption result; assumption label,
  selected level and original field states remain visible. Original input
  diagnostics are retained; no source bytes/findings are deleted.
- INT-057: same-recorder scoped TIMEIN eligibility, typed refusal otherwise.
  Bad ID can be addressed through an existing entry's scoped identity; it
  never fabricates a usable ID or resolves a nonexistent ID-only request.
- INT-058: typed contradiction refusal only from a separate approved evidence
  INT establishing a different association. The policy itself proves nothing;
  no 0.1 evidence rule exists, and heuristic observations cannot trigger it.
- INT-059: unproven multiplicity is unestablished; conflicting needs competing
  evidence. A preclassified-conflict unit is not a production proof fixture.
- INT-060: explicit choice on every query, never remembered/session-wide.
  Default Warn is a level policy and cannot choose a target.

No level bypasses candidate eligibility, setup qualification, retained-cut
blocking or evidence gates. Acceptance of a labelled choice is separate from
validity/usability of the selected fields. Quiet changes emitted warnings,
not Invalid/Ambiguous/unavailable states or whether a setup governs.

## Source and planned verification

Baseline: official 106-24R1 Chapter 9 section 9.5.4 Table 9-4 TK1 p. 9-48,
CDT p. 9-50, TTF/TFMT/TSRC pp. 9-71–72; relevant metadata is already
inspected in the [batch note](2026-11-26-association-assumption-requirements.md).
The source SHA-256 is
`dfc5ebfb3e6f02f58389126863d43a45d89c7e2d0533a1c690d9537531c740cb`.
Those rows describe local identity/types/formats/source categories. Caller
levels, refusal conditions, evidence admission and stateless query policy
come from the owner's answers, not a newly discovered Chapter 9 field.
Each future evidence source needs its OWN owner-signed INT defining its
claim, plus independent review and development/source/test gates. Floating
hypothetical evidence in a unit test is not an approved source or proof path.

Plan nine named bad-data cases: Strict/Warn/Quiet crossed with Invalid,
duplicate and unavailable, each with both ID and format arms. Also test
same-recorder eligibility, no nonexistent-ID target, no heuristic contradiction,
future evidence-INT gating, unproven multiplicity, explicit next-query choice
and default Warn without an implicit target. Original field states and
assumption labels/levels must survive all accepted cases, including Quiet.

Local checks audit IDs/parents, planned case links, the full 3-by-3 fixture
matrix and verbatim owner text, regenerate/check trace and check whitespace.
Heavy work uses GitHub CI. No runtime acceptance test or full cross-edition
development analysis is executed/completed here; these remain required.
