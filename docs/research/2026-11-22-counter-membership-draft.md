# Counter scope and membership draft — 2026-11-22

Baseline analysis for family 2 batch 6 and proposed INT-040. Not completed
cross-edition analysis, owner sign-off, independent AI review or executable
test evidence. Earlier approved interpretations remain unchanged.

## Sources inspected

Baseline: RCC 106-24R1 Chapter 9, January 2025.
[Official PDF](https://www.trmc.osd.mil/wiki/download/attachments/335389015/Chapter9.pdf?api=v2)
([archive](https://github.com/TelemetryWorks/rcc-106-standards/releases/download/106-24R1/Chapter9.pdf)).

- Section 9.5.1 a after Table 9-1, page 9-5, declares the general 1-to-N
  rule and contiguity, not a universal textual suffix-to-counter algorithm.
- Table 9-7 NUMBER OF MEASUREMENT LOCATIONS, page 9-117:
  D-x\MML\N-y-n counts locations of one measurement. NUMBER OF FRAGMENTS,
  same page: D-x\MNF\N-y-n-m counts fragments of one location. WORD
  POSITION D-x\WP-y-n-m-e carries the additional fragment index e. Their
  distinct parent scopes and governed positions must not be flattened.
- Section 9.5.14, page 9-172, permits noncontiguous X occurrence indices
  that match original objects. This does not authorize an invented X count
  or exempt every nested extension-only index from its own cited rules.

ADR-0026 already requires declared counter patterns, governed positions and
parent scope. Approved INT-038 already settles DISTINCT canonical numeric
identities, source spelling preservation and normalization findings. INT-039
keeps repeated scalar counters Ambiguous. ADR-0023's separate validity and
counter passes motivate preserving entry evidence when a payload is bad.
No archived edition, handbook or real-recording audit is claimed complete.

## INT-040 choice and alternatives

Recommend structural membership: an entry with an unambiguously identified
scope/index counts even if its payload is Invalid or that scalar slot has
several declarations. Count its canonical identity once, retain every witness
and keep the value/duplicate findings. Several properties of one entry are
also one identity when the reviewed member rule declares them witnesses.

Alternative: count only individually valid values. A malformed value would
then make a visibly declared entry disappear, mixing validity with cardinality.
Alternative: count lines. Duplicates and several child properties would inflate
N. Structural membership exposes both defects but has an important cost:
matching cardinality does not establish valid values or overall conformance.
Reports and tests must keep that distinction explicit for the owner.

Zero and out-of-domain identities remain observed evidence. Do not delete
them to make a passing set; check domain and gaps separately. If a required
member's identity/scope cannot be established, cannot-evaluate names that
limitation rather than certifying a partial set. An unrelated malformed item
must not be assigned to a scope by guess. Membership rules must declare all
relevant patterns before a complete membership check can be claimed.

An absent, Invalid or Ambiguous effective counter also blocks evaluation.
Do not infer N from the maximum index or number of source lines. A real
cited default is usable only through an applicable reviewed effective-value
rule; no lexical count parser or new default is chosen by this draft.
The scope of an unreviewed rule remains unavailable, not guessed from \N.

## Scope, domain and resource boundaries

For a contiguous rule, independently test cardinality and domain/coverage.
N=2 with {1,3} fails despite the right size; N=3 with {1,3} fails despite
the right maximum. Canonical 01 and 1 do not create two members. Parent
occurrence and every fixed nested index separate evaluations. X's cited
occurrence exception does not disable unrelated ordinary counter checks.

Counter comparisons may order canonical decimal strings by length then
lexicographically; equality/hash keep INT-038 semantics. A 40-digit member
index must not trigger array allocation or iteration to its magnitude.
Represent missing intervals compactly and report evaluation limits explicitly,
not as silent success or a fabricated exhaustive list. Detailed budgets are
later work; no machine integer width or new dependency is mandated here.

## Cross-fragment fixture follow-up

The owner requested explicit joining prerequisites for the prior planned
cross-fragment duplicate case: same channel, sequence +1 modulo 256 and
matching FRMT/RCCVER, following ADR-0025. Preserve ADR-0031's completeness
uncertainty: those conditions join a candidate run, and repeated single-entry
fields can still make its record interpretation ambiguous. The duplicate test
checks one assembled text document and both fragment locations; it must not
claim proven completeness or governing authority from the joining conditions.

## Planned checks and remaining work

TEST-DATA batch 6 specifies nested-parent isolation, distinct spelling/property
witnesses, invalid values, duplicate slots/counters, zero/out-of-domain entries,
missing intervals, unavailable membership, X exception boundaries, original
locations and byte preservation. Expected scopes and sets must be independently
enumerated from small supplied fixtures, not derived by the same matcher.
These are acceptance specifications, not executable tests. Full definition
interpretations, effective-value/type rules, severity/profile policy, remaining
registry/validator and other family-2 areas still precede family 3. No parent
is complete merely because these counter children have been drafted.
