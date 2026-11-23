# INT-011: owner reconfirmation and independent AI concurrence

Date: 2026-11-23. Owner: Joey. Interpretation: INT-011.

> INT-011 is now RECONFIRMED by the owner (decision 2026-11-23, Joey) with the AI concurrence on #38 at c623b15.

This completes ADR-0033's interpretation-review gate for independent
per-channel joining. The previous AI CONCERNS on #37 at fabbc06 were
addressed by the amendment; the amended wording received CONCURS on #38
at c623b15. Prior owner acceptance and the 2026-11-23 clarification remain
recorded; source freshness, deep development analysis and intensive executable
tests are still required. No implementation is claimed.

The owner relays three review consequences for subsequent requirements:

1. INT-034's "followed by another packet" means SAME-CHANNEL. A run ending
   at EOF in `;` is complete with its label, not downgraded solely for EOF.
2. Expose a PROVISIONAL classification for an open run, e.g. "text ends at
   ;, run still open", so streaming consumers such as irig106-time's CLI
   can use setup data before EOF. This is never a joining stop. The exact
   consumer-use/governance contract remains a review item, not silently
   authorized by INT-011 approval.
3. Per-channel open runs need an explicit memory budget under L1-ROB and a
   typed limit error, never silent run loss.

The new requirements and planned cases carry these consequences. INT-034's
precise rules, INT-028/031 governance and provisional-use details remain
unapproved. An ordinary START label is retained evidence, not known damage.
Earlier decision and batch-14 research records remain unchanged history.
