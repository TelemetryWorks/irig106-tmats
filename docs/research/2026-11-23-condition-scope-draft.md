# Condition scope and presence predicates — draft, 2026-11-23

## Status and boundary

Family 2 batch 8 derives L1-REG-004 and the condition-reporting portion of
L1-VAL-006. INT-042 is proposed: owner sign-off and independent AI review
under ADR-0033 are pending. This is a source review and alternatives record,
not the completed deep development analysis or evidence of executable tests.
Existing approved INT-035 through INT-041 remain unchanged.

General registry and validation keep their existing roadmap release scope.
The bounded 0.1 time view will consume only the reviewed subset it needs.
The restructure and source store remain later implementation families.

## Source inspection

Authority: [106-24R1 Chapter 9](https://www.trmc.osd.mil/wiki/download/attachments/335389015/Chapter9.pdf?api=v2),
January 2025 publication, inspected as source text for this draft.

- Section 9.5.1 a, printed page 9-6, distinguishes Allowed when, Required
  when, valid information when supplied, and defaults for absent attributes.
  Presence and validity are distinct facts; the text does not define an API
  recovery truth table for malformed/duplicate condition operands.
- Table 9-3 TRANSMIT ANTENNA TYPE T-x\AN1, printed page 9-18, states
  "Allowed when: T-x\ID is specified". Its explicit x reference avoids a
  new interpretation of an omitted occurrence index in the presence example.
- Table 9-4 TIME FORMAT R-x\TFMT-n, printed page 9-72, uses the TIMEIN
  comparison against R\CDT. This needs a usable value and a separately
  reviewed mapping of the source abbreviation to the same channel. A TIMEIN
  somewhere else in the document cannot satisfy that mapping.
- Section 9.4.2, printed page 9-2, supplies the full blank sentence now
  quoted in the INT-041 sign-off record, L3-VIEW-005 and RELEASE-0.1.
  This draft approves no new interpretation of blanks in values or links.
- ADR-0022 requires scope, link/default policy and cannot-evaluate; ADR-0023
  supplies the invoking four validation passes and effective values.
  ADR-0033 governs the review gate. INT-008/009 retain their own pending
  review obligations; a condition binding cannot bypass those obligations.

## Owner choice: INT-042

**Option A — recommended:** an explicitly reviewed is-specified predicate
asks whether actual matching items exist. Known absence is false; at least
one actual item is true, including empty, invalid and duplicate values.
Their value findings remain visible. Defaults alone are not physical presence.
A missing value in a VALUE comparison still gives cannot-evaluate.

In practice, a damaged source ID can activate an is-specified condition
without supplying a usable source ID. Reports therefore need to show both
presence and the separate invalidity/ambiguity. This does not certify that
supplying the dependent field or the entire configuration is conforming.

**Option B:** require a usable singular interpreted value even to answer
is-specified. Invalid/ambiguous values would make presence cannot-evaluate.
This avoids a true result beside invalid data, but conflates physical presence
with usability and obscures the fact that the writer supplied the field.

Recommendation is A because it preserves those facts separately. This is a
proposed recovery semantics choice, not a claim that the table explicitly
states it. Unknown scope, an unavailable semantic lookup or an unresolved
linked target remains cannot-evaluate under either option; neither may infer
an absent target or select a candidate. Owner sign-off is still required.

## Planned evidence and remaining derivations

TEST-DATA contains planned fixtures for known empty sets, present empty/bad/
duplicate values, wrong occurrences, unknown scopes, unavailable bindings,
missing-value inequality, explicit-only/default-enabled policies and evidence
retention. Synthetic supplied rule/link results test the evaluator contract;
they are not newly claimed Chapter 9 registry definitions or associations.
Every new L3 has planned acceptance coverage; none has executed code evidence.

Before implementation: complete deep analysis across supported editions,
handbook and representative data; obtain exact-head independent review and
owner sign-off; then write executable tests independent of evaluator logic.
Source changes must trigger interpretation re-review under L1-REG-003.

Still to derive: complete usage-condition grammar and compound semantics,
operator precedence, dependency-cycle handling and resource bounds, registry
extraction/freshness, link construction and full presence/profile rules.
Do not import Appendix 9-E expression semantics by analogy or silently use
Boolean short-circuiting to settle an unavailable required operand. No
compound-expression truth table is selected by this bounded batch.


## Review amendment — 2026-11-23

The owner relayed independent AI **CONCERNS** on PR #28 head `07a21a3`:
Option A is recommended with a narrower absence rule. This amendment
supersedes the baseline's unconditional zero-match/known-absence wording and
its general phrase damaged source ID: a bad VALUE with an exact name can
prove presence; a damaged NAME cannot. INT-042 remains proposed; owner
sign-off and independent recheck of the amended head are pending.

Textual basis for Option A: 106-24R1 Chapter 9 section 9.5.1 a, page 9-6,
under R/R Ch 10 Status, states:

> If the attribute is specified in the TMATS file, it must contain valid information.

The wording separates specification from its validity obligation. It supports
physically-present semantics, without explicitly prescribing damaged-input
recovery; the recovery guard below is still proposed interpretation.

Amended Option A: actual matches prove presence, even with unusable values.
Zero matches proves absence only without related damaged evidence in the
same declared literal family and compatible scope. An unterminated suffix
`T-1\ID:a` or damaged name `T-1\ID\xFF:a;` instead gives cannot-evaluate;
a dependent T-1\AN1 cannot be judged not allowed from that unknown answer.
Do not repair either input or promote it to a lookup item. INT-035 stays
unchanged. Related damage alongside a known exact match does not erase that
positive evidence.

INT-040's readable-parent compatibility guard applies to multiple-entry
children: unreadable parents do not exclude compatible scopes, and wrong
arity cannot be guessed into alignment. Retain the literal family and all
source evidence without searching values, accepting arbitrary printable
prefixes or enumerating hypothetical parents. Unrelated families and proved
incompatible parents do not block absence. The explicit trailing-high-byte
ID example is diagnostic family evidence, not a repaired ID key.

Matching uses projection/case folding and INT-038 canonical indices; raw
spellings and normalization findings remain. Negated presence complements
only true/false; unknown stays unknown. Separate documents/setup records do
not contribute even when their raw names match. INT-039 assembly scope stays
unchanged: one record spanning matching fragments remains one document.
Planned fixtures cover each edge; they are not executable tests.

Option B remains requiring a usable singular value even for presence, which
conflates physical presence with usability. Recommendation remains amended
A; its practical cost is more cannot-evaluate outcomes when damage prevents
proof of absence. No owner approval or amended-head concurrence is claimed.
