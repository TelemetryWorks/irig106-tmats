# Legacy independent checks — batches 3/4 and INT-064 recheck

Recorded 2026-11-27 from reviewer thread thr_qsvwmtnn8f. Batch 3 reviewed
main c682e05babc5a9d7e6fe104e22bd278d68aba6af; batch 4/INT-064 recheck
reviewed main 283ffd5. Reviewer-supplied primary SHA-256 prefixes: 106-24R1
Chapter 9 dfc5ebfb, Chapter 11 0f63792d, Chapter 10 bdf5ab59 and Chapter 6
762d9634. This is received independent evidence, not this maintainer's
completed development analysis or independent review of later amended text.

## Verdicts as received

| INT | Verdict | Source and follow-up |
|---|---|---|
| 023 | NEEDS OWNER F3; reading rule CONCUR | Ch6 section 6.2.3.11 p. 6-22 uses G DSI N=18; reading keeps missing-delimiter/raw bytes. Draft suggestion-only guarded A versus no-suggestion B, owner still decides |
| 024 | CONCERNS, narrow; owner pending | Ch11 Table 11-4 analog rows p. 11-7 (table starts 11-5); PDTF 9-50, UTF 9-77, FCTF 9-82. Add baseline edition, unusable operand -> cannot-evaluate and validator ownership boundary |
| 025 | CONCUR, owner pending | TDTF is p. 9-81, corrected from review's 9-80; TSPIIN enum p. 9-50. Condition's TSPIN reads TSPIIN; literal document typo stays unknown |
| 026 | CONCUR, owner pending | RPS 9-51; MFF RPS 9-54, corrected from 9-53. Both P CDT references mean same-channel R CDT; seven neighboring conditions verified; doubled When retained as source defect |
| 027 | CONCUR, owner pending | C DCN p. 9-158 duplicates R AMN. Only R AMN/DMN carry Links-to C DCN; named other R measurement fields do not. Duplicate is a no-op, no inferred extra source |
| 028 | CONCERNS, substantive; owner/rewrite needed | Ch11 11.2.7.2 p. 11-50 and Ch10 10.5.1/Table 10-9 p. 10-2. Replace latest-Complete adoption shortcut with umbrella composing approved INT-045/046/048/049/051/052/053; explicit unresolved target/governance |
| 029 | CONCERNS, substantive; owner/rewrite needed | SRCC pp. 11-50–51 and Ch9 9.4.2 p. 9-2 do not authorize generic semantic equality. Propose findings-only raw-subtype table, no adoption inference; descriptive changes remain owner sub-question |
| 031 | CONCUR with recommendation, owner pending | Ch11 p. 11-50; ungoverned means no governing setup, distinct from unresolved comparison target. Any ambiguity-only opt-in is explicit per query; cut facets always block. Complete integrity findings use INT-051 |
| 032 | CONCUR, owner pending | Ch10 p. 10-2 and Ch11 11.2.3 p. 11-24; explicit per-query assumptions, never session default; XML unsupported in 0.1 |
| 033 | CONCUR, owner pending | WFP p. 9-117 plus Appendix 9-C/Table C-2 example: two absent positions default to 1 and trigger ambiguous-order finding; no index/prose inference |
| 034 | CONCERNS, narrow; framework/amendments CONCUR | Ch11 pp. 11-50 and 11-8, Ch9 p. 9-2. Clarify padding question, modulo-256 continuity and unavailable-source versus integrity-finding distinction; provisional/precedence owner items remain |
| 064 | Recheck CONCUR, owner pending | Ch9 section order exactly G/T/R/M/P/D/B/S/Q/C/H/V/X. A mechanism + named D or explicit L1 reinterpretation; no residual independent concern, no owner choice implied |

All page corrections in the reviewer's follow-up are applied above. The
reviewer reports the 27 outstanding legacy entries checked across batches
1–4. That means verdicts received, not 27 approvals: CONCERNS/open/proposed
entries and development-analysis/test gates still exist. The first two
batches are kept in a separate PR and evidence records.

## Owner summaries and recommendations

**INT-023/F3:** A suggests ':' only when a complete recognized name before
'=' and no colon establish the guard; never repair automatically. B gives no
suggestion. Recommend A; raw reading/missing-delimiter finding is unchanged.

**INT-024:** approve a reviewed baseline type/format table with uncertainty
instead of arithmetic code guessing; historical unavailable mappings and
unusable operands cannot become mismatches. Owner still decides the proposed
mapping. tmats owns the mapping; irig106-validator consumes the check.

**INT-025/026/027:** approve the proposed obvious errata readings (TSPIIN,
same-channel R CDT, duplicate link reference a no-op). The independently
concurred proposals remain unapproved until owner sign-off.

**INT-028 (substantive rewrite):** recommend A, an umbrella over the already
approved guards, not “latest Complete wins”. A candidate may fail to take
control; later comparison can be unresolved, with retained evidence. B defers
governing application until all takeover/boundary decisions exist. Both keep
multiple simultaneous files out of scope. File-order first-fragment versus
completion and exact takeover boundaries still need owner choices.

**INT-029 (substantive rewrite):** recommend A, a raw comparison/SRCC findings
table only. Identity, SHA-only, layout-only and other raw differences cannot
prove semantic equivalence or a clock reset. Independent reviewed semantics
may establish a semantic class; governance defers to INT-053/028. B defers
this table until semantic equivalence is defined. Ask separately whether
G COM/G OD descriptive changes count as configuration change. No option is
approved; duplicate ambiguity still blocks adoption regardless of equality.

**INT-031/032/033:** retain their independently concurred conservative
recommendations, explicit per-query labelled choices, no default decoding or
inferred measurement order. They still need owner decisions. Integrity on
otherwise eligible Complete records must not be folded into incomplete-input
refusal; distinguish no governing setup from unresolved comparison target.

**INT-034:** modulo-256 continuity is explicit source fact, not an owner
choice. Final-padding precedence remains explicitly open under (b), using
INT-035/036 classes rather than a hidden literal-space-only test. Retain
source availability separately from core data/secondary integrity findings;
Mismatch alone on available well-framed text cannot silently quarantine under
INT-051. Actual core evidence bindings need verification, not invented fields.
Provisional use and precedence (a)/(b), including any stronger quarantine,
remain owner items; the amended details still require independent recheck.

**INT-064:** amended A+D recommendation now has independent recheck CONCUR;
owner must still choose A+D (recommended), another built-in profile or an
explicit L1 reinterpretation. Default replay never normalizes. This record
clears only the independent-recheck pending note, not the owner gate.

No new owner decision was made. The rewrites are proposals with no enabled
implementation. Analysis fields remain not yet written. Planned cases below
are not executable evidence; exact-head PR review and substantive rewritten
interpretation checks remain necessary.

## Later exact-head independent review and remaining narrow fix

Reviewer message received 2026-11-27: #81 at
`699029b9e739f7bef7f27e7f67d8cac23c20d90d` APPROVED; the INT-005/006/010/015
amendments all rechecked CONCUR. Their register fields now cite that exact
head; old batch records retain their dated initial CONCERNS. Development
Analysis fields and executable-test gates are unchanged.

The reviewer APPROVED #82 at ef9f7ea and requested one narrow change in #83
at `ada532841ea9cddef10994f91739faa1ad09b5b8`: enumerate INT-028 A's takeover
questions. The owner brief's alternatives/recommendations were otherwise
judged fair and complete, including INT-022 options (CONCUR as options,
owner pending), and the approved-rule alignment was checked. This later
review is not a new owner sign-off or an independent check of the newly
expanded question wording. The five sub-questions now identify effective
boundary, unresolved-target takeover, F7 effect, distributed-source scope
and file order; B's deferral option is unchanged. All five remain unanswered.
