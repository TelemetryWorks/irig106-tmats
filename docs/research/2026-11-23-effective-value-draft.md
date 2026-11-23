# Effective-value evidence draft — 2026-11-23

Baseline analysis for family 2 batch 7 and proposed INT-041. Not completed
cross-edition analysis, owner sign-off, independent review or executable-test
evidence. Earlier approved interpretation entries are unchanged.

## Sources inspected

Baseline: RCC 106-24R1 Chapter 9, January 2025.
[Official PDF](https://www.trmc.osd.mil/wiki/download/attachments/335389015/Chapter9.pdf?api=v2)
([archive](https://github.com/TelemetryWorks/rcc-106-standards/releases/download/106-24R1/Chapter9.pdf)).

- Section 9.5.1 a, page 9-6: Default describes processing an absent
  attribute. Its recorder-status note still requires physical inclusion for
  applicable R-status fields even when a default exists.
- Table 9-4 TIME FORMAT R-x\TFMT-n, page 9-72: TIMEIN applicability,
  enumerated values and Default A. An explicit A, a defaulted A, an explicit
  zero enumeration and an empty projected value must not become the same
  observation. No other edition inherits that default without verified rules.
- Table 9-11 NUMBER OF OCCURRENCES C-d\DPNO, page 9-167: the default 1 is
  in the definition prose, not a Default usage field. This is a source-origin
  example, not evidence that its complete condition interpretation is ready.
- Table 9-2 PROGRAM NAME G\PN, page 9-10: a simple field with no default
  in that row's usage/prose, used with an explicitly reviewed fixture binding.

ADR-0022 separates source from interpretation; ADR-0023 supplies five states
and a read-only effective-value layer. ADR-0028 carries labelled edition
basis without conflating G\106 and RCCVER. RELEASE-0.1.md already prohibits
fallback over invalid/conflicting explicit time fields and requires a reason
when the default's applicability/basis cannot be established.

## Derived behavior already decided

With reviewed scalar cardinality, multiplicity remains Ambiguous under
INT-039 before choosing by value validity. One locally usable actual value
is Explicit with raw provenance. A demonstrated character/type/range failure
is Invalid with evidence, not an invitation to default. Present empty or
control-only projected content is not absent; its usability follows the
field's own reviewed rules. INT-036 information does not invalidate every
text field, and INT-037 high-byte Invalid remains in force.

For an absent supported field, Defaulted requires its verified applicable
rule and identified edition basis, with origin/citation and no fictitious
input span. Otherwise Missing carries the reason no applicable default was
verified. This includes an unknown default condition/basis, which must not
be relabelled false. It does not mean an entirely unavailable field binding
was successfully resolved as Missing. An identified fallback stays labelled
fallback; this resolver does not choose or repair the edition declaration.

Physical required-field checks and cross-field/packet findings remain
independent. At baseline TIMEIN/TTF=2 with missing TFMT, the reviewed release
contract still says Defaulted A plus a network-time inconsistency finding,
not a substituted format or overwritten packet evidence. A read never adds
that A to the ordered source, nor proves a data-channel/time-source link.

## INT-041 choice and alternatives

Recommend an unavailable/cannot-evaluate read when the required reviewed
field definition, supported interpretation basis or evidence needed to decide
local validity is unavailable. Keep raw candidates and the precise reason.
Do not label unsupported knowledge Missing (which claims absence) or Invalid
(which claims a proved failure of the data). Preserve an already established
Ambiguous or demonstrated Invalid outcome; lack of another rule must not
suppress known evidence. Marker blocking likewise is not an absent key.

Alternative: force every failed request into one of five value states. That
simplifies a result shape but disguises unsupported coverage as missing/bad
input and may encourage invented fallback. The cost of the recommended
choice is that callers must handle read availability as well as the five
semantic states of supported fields. No sixth field-state, Rust wrapper,
public error enum or exhaustive API shape is decided here. This semantic
choice needs owner sign-off and independent AI review under ADR-0033.

## Planned checks and remaining work

TEST-DATA batch 7 supplies fixtures with explicit reviewed bindings and basis;
these are not a production registry. Cases cover Explicit/Defaulted A,
Invalid versus absence, empty/control-only input, no default, unknown/false
default conditions, unsupported metadata/basis, unavailable validity operands,
prior ambiguity/failure, source origin, scopes and immutable emission.
Expected states and original spans must be independently specified.

Full source-hash/freshness enforcement, cross-edition/handbook/sample analysis,
condition language and dependency rules, dynamic types/ranges, presence/profile
validation, time APIs and other family-2 areas still need their own requirements
and executable evidence before family 3. No prototype test result is used as
redesign verification, and no release-scope expansion is implied.
