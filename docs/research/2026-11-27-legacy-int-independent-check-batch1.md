# Legacy interpretation independent check — batch 1

Date recorded: 2026-11-27. Reviewer message from thread
thr_qsvwmtnn8f, ADR-0033 independent check at main
`c682e05babc5a9d7e6fe104e22bd278d68aba6af`. The reviewer verified the
106-24R1 Chapter 9 PDF with SHA-256
`dfc5ebfb3e6f02f58389126863d43a45d89c7e2d0533a1c690d9537531c740cb`.
This records the received check, not a new owner decision, completed
maintainer development analysis or an independent review of this later PR.

## Received verdicts and source locations

| Entry | Reviewer verdict | Verified source and rationale |
|---|---|---|
| INT-001 | CONCUR | R CDLN p. 9-50 and EV DLN p. 9-46 omit Q; Q DLN Table 9-10 p. 9-146 and section 9.5.1 b(h) p. 9-7 supply the R-to-Q tie; two sources preserve an otherwise lost valid relationship |
| INT-002 | CONCUR | R ANM/UCNM/MCNM/ENAM/CBM pp. 9-56–82 omit Links-to; prose section 9.5.1 b(g)/(h) p. 9-7 supplies ties. B BT p. 9-124 is 1553/A429; S p. 9-137 and Q p. 9-146 establish their allowed channel types; CANIN has no B/S/Q target |
| INT-003 | CONCUR | Section 9.5.1 b p. 9-7 says all paths appear in Links fields, contradicted by INT-001/002. Three-source generation and rejecting unregistered one-sided relationships is a project process rule, not a claimed standard requirement |
| INT-004 | CONCUR with note | P DLN p. 9-95 references B-d; B table p. 9-124 defines B-x. Placeholder comparison requires equal index-position arity and applies only to definition references, never document index values |
| INT-005 | CONCERNS, narrow | Target mapping is sound; selection with unusable same-channel CDT must be cannot-evaluate, retaining all candidate groups. The Q target depends on INT-007's FBCIN reading |
| INT-006 | CONCERNS, narrow | Section 9.5.12 p. 9-171 supplies H-to-G test-item tie; G TA p. 9-10 is the right target. Specify ASCII case comparison, INT-041 blank handling and ambiguous duplicates under ADR-0026/INT-039 |
| INT-007 | CONCUR | Q DLN p. 9-146 says FBCIN; CDT p. 9-50 defines FBCHIN, as does S p. 9-137. Reviewer also verified one FBCIN occurrence in each archived 106-22/23/24/24R1; a document's FBCIN is still unknown |
| INT-008 | CONCUR | C DPNO p. 9-167 says Allowed when C DCT is DER and Default is 1 in Definition prose; DCT p. 9-165 is occurrence-local. Optional prose-origin labelling applied |

The reviewer's historical Chapter 9 fingerprints for INT-007 were supplied
as prefixes: 106-22 773bb32f, 106-23 32b74561, 106-24 7e9253a9.
They are review evidence, not invented full hashes or a claim that this PR
independently repeated that archived-edition investigation.

## Applied amendments, awaiting recheck where concerned

INT-004 now explicitly distinguishes definition-placeholder identity from
actual document indices and requires equal arity. Two planned fixtures pin
those boundaries. The original CONCUR reference remains at the reviewed
snapshot; later wording still receives exact-head PR review.

INT-005 now requires cannot-evaluate for Missing, Invalid, duplicate/Ambiguous
or unavailable CDT, preserving all CDT evidence and every target group
candidate. A usable non-target type is different from inability to select.
The Q/FBCHIN path cites INT-007; FBCIN in a document is not repaired.

INT-006 now specifies ASCII comparison, raw preservation, leading blank
unavailable/no trimming under INT-041, and ambiguity on multiple G/H TA
occurrences even when equal. Unreviewed other blanks are not silently
normalized. G TA is not replaced by another group's TA. Qualifications and
semantic-access gates are not upgraded by link evaluation.

INT-008's prose default gets explicit Definition-prose provenance, not an
invented printed Default usage field. All eight follow-up fixtures are
planned; no general link-resolution implementation or 0.1 scope expansion.

No new owner decision was identified in these narrow amendments. INT-005
and INT-006 retain CONCERNS with amended wording pending independent recheck;
the six received CONCURs are recorded only for their actual reviewed scope.
Existing owner acceptance dates are preserved. Development Analysis fields
remain not yet written; source freshness, intensive tests and other
implementation gates remain in force. This record is review metadata, not
the future maintainer development-analysis task.

## Checks and continuing work

Regenerate/check TRACE-MATRIX after register changes; check only the received
entry set and planned fixture references locally. No Rust tests or source
qualification runs are claimed. Heavy checks belong in GitHub CI.

The historical CSDW-layout maintainer task remains a separate docs PR; these
arriving review decisions are not folded into its source analysis. Independent
legacy batches and the INT-064 recheck will be recorded only as received.
