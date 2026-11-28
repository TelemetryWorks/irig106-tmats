# INT-034 Q25 owner sign-off — 2026-11-28

Owner: Joey. Process: ADR-0033. Owner direction, verbatim:

25 (INT-034 precedence) A:
- A padding-only record (no substantive attributes) is INCOMPLETE, with the reason 'no substantive attributes', and no truncation claim.
- Proven missing leading text gives a BLOCKING cut facet that always wins, even when duplicates make the aggregate Ambiguous. No ambiguity opt-in can make it usable.
- A final ';' never erases proven leading loss.
- Merely being the start of the supplied input is NOT proof of loss.

This completes the previously open owner choice, not the independent check
of this new wording. Earlier signed parts retain their #87 e0a0cce FINAL
CONCUR; Q25's independent AI check is pending. Written development analysis,
analysis review, source freshness and intensive executable tests still gate
implementation. No final analysis approval or passing parser test is claimed.

The signed rule composes existing INT-035/036 layout/source framing with
INT-048's retained-cut facet and INT-031's ambiguity-only per-query access.
No new padding byte, missing-byte inference, joining rule, implicit split,
severity, Rust symbol or governing transition is added. Apply the padding-only
case to established absence of substantive attributes, not to a document made
empty by filtering invalid/unknown fields or empty values. Padding-only alone
never proves loss; independently proven loss still retains its own evidence.
Raw replay/extraction and scanner finding rules remain separate.

Current requirements/fixtures specify zero-length/padding/empty-region cases,
known leading loss with/without duplicates and final semicolons, no opt-in
bypass, and ordinary supplied-input starts that prove no loss. They remain
PLANNED. Q26 report formatting is a separate decision/PR, not chosen here.
