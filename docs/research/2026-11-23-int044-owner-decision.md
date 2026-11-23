# INT-044: owner chooses labelled fallback (Option B)

Date: 2026-11-23. Decision-maker: Joey. Context: #35 merged/approved at
799f42b; owner follows the independent reviewer's Option B recommendation.
Status: owner decision recorded, AI recheck of the amended wording pending.
This is not final ADR-0033 approval or implementation/analysis evidence.

## Owner decision and conditions

> OWNER DECISION 2026-11-23 (Joey) on INT-044: OPTION B (labelled fallback), following the independent reviewer's recommendation over the original A.

1. G\106 stays Ambiguous with every occurrence and duplicate finding; no
   candidate is selected.
2. The separate rules basis is labelled
   `fallback — reason: ambiguous G\106 (n candidates)`. This is a NEW
   explicit reason, never missing or unrecognised. Count all occurrences,
   including identical ones, rather than distinct years or edition readings.
3. Use the SAME INT-017 policy: Format 1 RCCVER 0x0E -> 106-24R1 rules,
   0x0D -> 106-20 rules, otherwise the existing policy/baseline, no new mapping.
   INT-017 already retains 0x07–0x0C's named editions; baseline applies with
   no usable eligible RCCVER. That existing behavior and review gate remain
   unchanged. Format 4/header codes do not supply the operand.
4. Defaults and findings under the basis carry the fallback label/reason.
5. An explicit valid override takes precedence; ambiguity evidence remains.

Identical duplicates still fall back, with agreement information only.
Separate records/documents do not form a duplicate set. A single high-byte
or blank G\106 follows INT-037/041 and the reviewed declaration binding.
CLI/JSON display the fallback basis at least as prominently as cannot-evaluate.
All bytes, candidate evidence and independent findings remain available.

## Rejected alternative and consequences

Option A would refuse automatic selection until a valid explicit override.
Its ADR-0028 reasoning: the existing fallback rule explicitly named missing
and unrecognised declarations, not Ambiguous ones. Blocking avoided silently
expanding that policy but required an override even for identical repeats.
Joey chose useful qualified checks instead. Option B explicitly expands the
reason, not the RCCVER mapping, and does not resolve the source declaration.
Consumers must retain the uncertainty when interpreting defaults/findings.

Sources remain 106-24R1 Chapter 9 section 9.4.2 page 9-2, Table 9-2 G\106
page 9-10, INT-039 and ADR-0028/0032. The ambiguity recovery is owner policy,
not a claim that Chapter 9 defines it. The original recommendation remains
in [the batch 13 draft](2026-11-23-edition-basis-draft.md) as dated history.
Planned fixtures and requirements are amended; no implementation or tests
have been executed for this behavior. AI recheck, analysis and tests remain.
