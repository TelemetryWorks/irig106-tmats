# Legacy interpretation independent check — batch 2

Recorded 2026-11-27 from reviewer thread thr_qsvwmtnn8f at main
`c682e05babc5a9d7e6fe104e22bd278d68aba6af`. Primary source: 106-24R1
Chapter 9 SHA-256 dfc5ebfb3e6f02f58389126863d43a45d89c7e2d0533a1c690d9537531c740cb.
Historical facts below are reviewer-verified archive evidence; abbreviated
hashes are retained as supplied, not expanded into invented fingerprints.

| INT | Received independent verdict | Source/check and action |
|---|---|---|
| 009 | CONCUR | D WFT/WFP, Table 9-7 p. 9-117: unindexed MNF count is same parent scope; compatible with INT-040 |
| 010 | CONCERNS, citation only | E-3 p. E-2 has == Equal To A==B; E-6 p. E-3 has = = !=; grammar p. E-5 region has ==. Correct E-6 citation/title, retain operator/warning behavior; amended citation awaits recheck |
| 013 | CONCUR | Appendix 9-C p. C-8 colon-ended example and section 9.4.2 framing confirmed; recurring pattern in 106-17/19/20/22/23/24/24R1, absent 13/15. Exact historical 18 count is extraction-sensitive, not independently reproduced; preserve S1 suspect status and suggest-only behavior |
| 014 | CONCUR | G revision row p. 9-10: two-digit year/range; 106-24 and 24R1 text equal except spacing. 24R1 publication in January 2025 does not make 25 a recognized year |
| 015 | CONCERNS, source fact only | Year wording already in 106-15 (f8c6ec06), absent in 106-13 (33028a32); size 2 already in 106-05 (ccdc77cc) / 07 (ef8783a5). Correct onset/size wording, preserve recognition/fallback behavior; amended wording awaits recheck |
| 016 | CONCUR | 106-05 Ch10 (7aa84ef9) all-reserved word; 106-07 (50d125bc) CH10VER plus SRCC bit 8; modern RCCVER describes recording compliance, not TMATS edition. Preserve no-declaration/reserved handling |
| 017 | CONCUR with note | Existing owner policy maps intended basis, not literal edition-table meaning. Before edition deltas exist, applied registry is 24R1 baseline: distinguish intended basis and actual vocabulary/availability, not a claim of older-edition validation |
| 022 | NEEDS OWNER, no proposal | Section 9.5.14 p. 9-172 covers renumbering, not removal. Open F2 remains open; this is 0.5 editing, not a 0.1 blocker; register amendment/options belong to PR B |

## Narrow amendments and remaining decisions

INT-010 no longer claims E-3 prints spaced equality. No new rendered-PDF
claim or operator change is introduced. INT-015 and current ROADMAP source
prose now state 106-15 onset and earlier field-size 2. Source search found
the obsolete onset also in ADR-0028's dated context; the accepted ADR remains
historical, its recognition/fallback decision unchanged. The corrected source
fact in INT-015 governs current claims. No current normative/test branch was
found depending on a first-106-17 onset; a planned source-metadata case pins
that correction. Historical research is not rewritten.

INT-017's intended-edition policy is retained, with baseline vocabulary and
availability limitation explicit. This does not approve INT-062's proposed
X vocabulary policy or introduce rendered report fields. The review's CONCUR
is tied to the policy at the reviewed snapshot; added wording receives normal
exact-head PR review. It is not completed development analysis.

**Owner item to route: INT-022/F2**, later 0.5 scope. Reviewer suggestions:
A refuse removal while extensions attach, unless explicitly overridden;
B remove the original but keep/report every now-orphaned X; C cascade-remove
only on explicit request. All need reviewed INT-061/062 attachment evidence,
never guessed targets. These are alternatives for a future owner proposal,
not a selected recommendation or approved operation. No 0.1 blocker is added.

Three planned follow-up cases accompany the citation/basis clarifications.
Analysis fields remain not yet written; source freshness, development and
executable verification gates are not waived. No Rust test or local source
qualification run is claimed. The historical CSDW-layout source review is a
separate maintainer task and PR, informed but not replaced by this check.

## Additional recheck: INT-064

Reviewer batch 4, main 283ffd5, received 2026-11-27: **CONCUR** with amended
INT-064's A mechanism plus named built-in D, or explicit owner L1
reinterpretation. The section order G/T/R/M/P/D/B/S/Q/C/H/V/X is a project
profile, not a standard mandate; anchors, stable within-group order and
unchanged default replay are preserved. No residual independent concern.
The owner choice remains pending. This clears only the amended-wording
independent check; it is not a sign-off of a normalization policy or completed
development analysis. Recorded in PR A per the coordinator's requested split.
