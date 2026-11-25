# Family 2 batch 16: bounded time-view metadata requirements

Date: 2026-11-25. Requirements and planned acceptance cases only. This is
not library implementation, a new interpretation or completed development
analysis. The owner-directed twelve-family plan still requires family 2
before the restructure and source-store implementation families.

## Why this slice is next

The requirements cover framing, characters, canonical names, scalar
multiplicity, effective values, payloads and assembly evidence. The bounded
TIMEIN/secondary-header view brought forward for irigtime still has no
field-specific L2/L3 decomposition. This slice supplies that contract using
those foundations and the 2026-11-21 owner direction in RELEASE-0.1. It does
not reopen the closed retransmission decisions or implement a consumer API.

It derives L1-VIEW-007's 0.1 subset and composes L1-VIEW-004 support already
specified by L2-VIEW-003/004. New obligations are L2-VIEW-005–007 and
L3-VIEW-012–022; all executable tests remain planned and coverage stays Draft.
The rest of the L1 parents is not claimed complete.

## Primary source inspection

Read the official [106-24R1 Chapter 9, January 2025](https://www.trmc.osd.mil/wiki/download/attachments/335389015/Chapter9.pdf?api=v2),
section 9.5.4, Table 9-4, on 2026-11-25. Download verification: 3,733,058
bytes, SHA-256 `dfc5ebfb3e6f02f58389126863d43a45d89c7e2d0533a1c690d9537531c740cb`,
matching the archive fingerprint already recorded in RELEASE-0.1. Source
inspection and fingerprint comparison are not runtime tests or registry coverage.

| Table 9-4 row | Printed page | Contract fact used |
| --- | --- | --- |
| TRACK NUMBER/CHANNEL ID, R-x\TK1-n | 9-48 | Actual ID, distinct from entry n; range 1–65535 |
| CHANNEL DATA TYPE, R-x\CDT-n | 9-50 | TIMEIN identifies the time-input category |
| SECONDARY HEADER TIME FORMAT, R-x\SHTF-n | 9-51 | Per-channel representation, not a TIMEIN-only field or a packet-presence bit |
| TIME DATA TYPE FORMAT, R-x\TTF-n | 9-71–72 | Separate from time representation; TIMEIN applicability |
| TIME FORMAT, R-x\TFMT-n | 9-72 | TIMEIN applicability and default A |
| TIME SOURCE, R-x\TSRC-n | 9-72 | Source category, not an associated channel ID |

The code-name rules use section 9.4.1–2 as interpreted by approved
INT-035–039; effective-field limitations use approved INT-041. Other
editions need their own supported, reviewed definitions. These rows do not
justify applying baseline defaults to unsupported editions.

## Composition rather than new choices

- Use document/revision and canonical recorder/entry identity; keep raw
  spelling, locations and every normalization/duplicate finding. Index n is
  never substituted for TK1, even when their numbers happen to agree.
- Confirm TIMEIN only from usable CDT in that scope. A damaged/duplicate
  CDT does not become usable because a TFMT or valid ID exists. Keep the
  unconfirmed entry evidence inspectable rather than returning empty success.
- Keep TTF, TFMT, TSRC and SHTF separate. Read SHTF on non-TIMEIN entries too.
  A source category is not a clock-selection link; field presence does not
  prove that any particular packet has a secondary header.
- Reuse the reviewed scalar states and default rules. TFMT default A needs
  a supported edition/basis and established TIMEIN condition. Unavailable
  blank handling remains unavailable under INT-041; no trim is introduced.
  Missing reasons and independent required-presence/cross-field findings stay
  visible. This batch does not derive a new complete validator or packet map.
- Views preserve bytes and provenance, with source-kind and eligibility
  limitations. Assembly/governing context is supplied, not chosen here.
  Same-document scalar duplicates also retain INT-048 assembly ambiguity;
  local diagnostic reads never make them usable. Retained-cut blockers stay.

No owner decision blocks these **requirements**: the existing unavailable
contract handles the blank-rule gap without selecting a value. Open general
INT-028/031/034 policies are not prerequisites for describing local metadata
with supplied context and explicit limitations. Their unresolved decisions
still block the corresponding future implementation, not this derivation.
No public Rust enum, symbol, layout or lifetime spelling is frozen here.

## Planned evidence and next boundaries

TEST-DATA names scoped identity, actual-ID mismatch, canonical-index,
duplicate, missing/invalid, default/blank, edition-label, SHTF/non-TIMEIN,
source-provenance and qualification cases. Test fragments illustrate the
field under test; complete source fixtures and independent expectations must
be constructed under the reviewed definitions before code. Do not use the
prototype extra-vector behavior as an oracle. Deliberately partial examples
are not asserted to be conforming full Chapter 10 setups.

Next bounded derivations remain: explicit NSB/source splitting and the
association-uncertainty/assumption contract, plus the remaining family-2
requirements. Measurement time-word views and full timelines stay in 0.2.
No format association, historical-edition behavior, correlation reset or
governing transition is silently added by this metadata contract.

Once the real time-view API exists, CI must build an irigtime-style CLI
consumer with `irig106-tmats = { version = "0.1", default-features = false,
features = ["std"] }` (RELEASE-0.1). The current prototype has no such API;
this documentation PR neither substitutes an all-features build for that
check nor invents a passing consumer test. No new manifest or workflow is needed.

Light local checks are requirement-ID/parent and fixture-link consistency,
trace drift and whitespace. GitHub CI runs the existing substantive checks;
its run IDs belong in the PR evidence. A green prototype suite is not
implementation verification of these new requirements.
