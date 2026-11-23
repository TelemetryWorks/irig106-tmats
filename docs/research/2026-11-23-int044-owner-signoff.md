# INT-044 Option B: approval and review follow-ups

Date: 2026-11-23. Owner: Joey. Interpretation: INT-044 Option B.

> #36 is MERGED, APPROVED at 6f6e080; the AI recheck of INT-044 Option B CONCURS.

Under ADR-0033 this completes the owner/independent-AI interpretation-review
gate. The owner chose Option B on 2026-11-23; independent AI concurrence is
tied to PR #36 at head 6f6e080. Source freshness, written development analysis
and intensive executable tests remain required. Nothing is implemented by
recording the approval. Earlier decision/draft records remain dated history.

The owner also requests three current-specification notes:

1. ADR-0028 rejected "validate nothing edition-specific without an override"
   for missing or unrecognised G\106. Option A would have validated worse
   inputs but refused duplicated good ones. B gives useful qualified checks
   without selecting a declaration candidate or concealing ambiguity.
2. Mixed duplicates `G\106:zz;G\106:24;` remain Ambiguous with all occurrences
   and duplicate findings; n=2, reason ambiguous G\106. Individual value
   recognition must not change that precedence to unrecognised or declared.
3. JSON has stable machine code `ambiguous_g106` with numeric `candidates`,
   separate from the display label. Count occurrences, including identical
   repeats; the label is not the machine-readable representation.

Current requirements, planned fixtures and CLI contract carry these notes.
All five Option B conditions and the unchanged INT-017 policy still apply;
INT-017's separate independent review is not cleared by INT-044 concurrence.
The owner/AI approval decides fallback for an already established document;
it does not prove a candidate fragment run is a complete record (INT-034).
