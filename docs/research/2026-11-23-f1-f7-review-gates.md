# F1/F7 interpretation review gates — 2026-11-23

PR #30 review requested explicit consistency between registry evidence gates
and coordinator-recommended reader/validator behaviour. The gates apply to
behaviour tied to register entries even when implemented by hand rather
than generated. ADR-0033 owner sign-off and an independent AI check are
required before implementation. Neither INT-021 nor INT-030 records them.
Coordinator decisions in ADR-0034 remain the recommendations, not completed
interpretation approval. No owner sign-off or AI concurrence is recorded here.

## F1 / INT-021 — summary for the owner

Recommendation: when one document has two or more G\SHA items, keep every
occurrence and every byte, report checksum verification indeterminate and
refuse to stamp until the caller fixes the duplicate input explicitly. Never
select a first/last item or claim successful verification, even if the items
agree. Damaged recordings remain readable; checksum trust and stamping wait.

Basis: 106-24R1 Chapter 9 Table 9-2 MESSAGE DIGEST/CHECKSUM, page 9-13,
defines the G\SHA exclusion but not duplicate recovery (INT-021 also cites
Chapter 6 section 6.2.3.11 f through INT-018/019). Coordinator decision,
2026-11-21, ADR-0034, chooses conservative indeterminate/refusal semantics.

Owner options: approve that recommendation under ADR-0033, subject to the
independent AI check; or request a different explicit duplicate policy with
its exclusion rules and risks reviewed before implementation. Recommendation
is approval of F1. Its cost is that duplicates require an explicit fix before
stamping; its benefit is avoiding unverifiable winner/exclusion guesses.
F1 is in 0.1 checksum/stamping scope, so this pending approval is a release
blocker, not an implicit deferral or exception to readiness gates.

## F7 / INT-030 — summary for the owner

Recommendation: require verified identification of the setup-record
configuration-change event; until then report the event-presence check
indeterminate and never guess its packet type. Approval would authorize this
uncertainty behaviour, not assert that the event has been identified or
complete L1-CH10-005's event-presence check.

Basis: 106-24R1 Chapter 11 section 11.2.7.2, page 11-51, requires a setup
record configuration change event packet before the new setup record. Its
concrete packet identity remains unverified in INT-030. The coordinator
recommendation dated 2026-11-21 is ADR-0034; do not silently equate the event
with the separately described Format 2 recording event packet.

Owner options: approve the conservative uncertainty policy, subject to an
independent AI check; or request a different evidence-backed policy for
review. Recommendation is approval of F7. Its cost is an incomplete event
check until identity is verified; its benefit is no false pass/failure from
a guessed packet identity. Future identification still needs cited evidence,
analysis, tests and the applicable interpretation review.

## Separator-spanning damage clarification

The reviewer also requested that a high-byte run capable of covering a
component separator leave the split unknown. For `T-1\xFFD:a;`, the run
could cover `\I`; no guessed split may prove ID absent. Treat this as
wrong-arity-style unassigned evidence under INT-040, retaining every scope
compatible with unambiguously readable facts. L3-VAL-012 and a planned case
record the guard; no key, split or parent enumeration is introduced.

This is documentation and planned evidence only. Existing source-hash,
completed analysis and executable verification gates remain; no implementation,
completed deep analysis or executed fixture tests are claimed by this record.


## Further review follow-up — 2026-11-23

This dated addendum supersedes the baseline's both-checks-pending wording:
the owner relayed independent AI **CONCUR** on F1 and F7 in PR #30 at head
`13173b04faac989fa581b8e9ebbd0df6c80d6b98`. Owner sign-off remains pending
for both, so neither is approved for implementation. This is the owner-relay
reference, not a claim of a separately posted review or review of this new
head's edge wording. Coordinator recommendations remain unchanged.

### Pre-ADR-0033 review is not grandfathered

For entries INT-001 through INT-034, owner acceptance predating ADR-0033
with Review: pending still requires the independent AI check before
implementation. Preserve the owner's recorded acceptance; it does not fill
the missing AI-check slot. Other open/coordinator entries do not thereby
become owner-approved. Existing source freshness, completed development
analysis and executable verification remain additional gates.

Alongside F1/INT-021, explicit **0.1 checksum review blockers** are:

- **INT-018:** G\SHA is a code name in any letter case, never matching text
  inside another value.
- **INT-019:** an unterminated G\SHA is malformed; never report a match or
  invent an exclusion end.
- **INT-020:** upper-case hex matching the digest gets a match with a warning
  for Chapter 6's lower-case requirement; stamping writes lower case.

Their existing owner acceptance is recorded, but their AI checks remain
pending; the owner will route them next. F1/F7's concurrence covers neither
these entries nor their checks by implication.

### F1 edge cases

A damaged name compatible with G\SHA (e.g. `G\SH\xFFA`) under L3-VAL-012
also makes verification indeterminate, including beside one exact SHA item.
The presence of that exact item does not establish checksum-item uniqueness.
Preserve all bytes/evidence; do not convert diagnostic compatibility into a
usable key. This leaves INT-019's exact unterminated-item malformed rule intact.

Refuse stamping ambiguous input, but allow the ADR-0029 explicit edit that
removes a duplicate by naming one exact revision-bound item. Rebuild/re-read
the result; stamping occurs only after ambiguity is fixed, possibly as the
same transaction's last step. Never silently pick a survivor or block the
explicit repair itself. Remaining damaged/duplicate evidence still blocks a
stamp. TEST-DATA records planned uncertainty/repair cases, not executed tests.

### F7 analysis notes — no new identification rule

1. SRCC=1 on a session's first setup record is reported separately, not
   swallowed by the event-presence unknown result.
2. Fragment SRCC disagreement must remain visible. ADR-0025 requires matching
   FRMT/RCCVER, not SRCC; assembly success is not proof of SRCC agreement.
   Do not pick first/last/any-bit summary semantics without reviewed analysis.
3. A nearby Format 2 event packet is a candidate only; without verified
   identification it never satisfies the configuration-change event rule.
4. Once the event is identified, prior needs its own cited interpretation of
   ordering/scope. Neither proximity nor identity alone settles that meaning.

These notes guide future analysis; they are not completed analysis, chosen
fragment-summary/prior semantics or executable evidence. Earlier source
citations and ADR-0025/0029/0033 remain the governing references.


## Owner sign-off recorded — 2026-11-23

Joey approved F1/INT-021 and F7/INT-030 under ADR-0033, with independent AI
CONCUR at PR #30 head `13173b04faac989fa581b8e9ebbd0df6c80d6b98`.
This supersedes the earlier pending-owner descriptions above. See the
[approval record](2026-11-23-int021-int030-owner-signoff.md) for exact-item
repair, nearby candidate-only events and remaining evidence. INT-018/019/020
still need their separate AI checks; source freshness, analysis and executable
verification remain. No completed implementation or new-head review is claimed.


## INT-018/019/020 reconfirmation recorded — 2026-11-23

Joey reconfirmed all three under ADR-0033, preserving their original
2026-09-26 owner acceptance, with independent AI CONCUR at register-review
main `30e42a3eac791f2f794e5fe239c59590eb6682b8`. This supersedes the
earlier descriptions of their pending AI checks and removes their 0.1
checksum blockers at the interpretation-review gate only. The combined
[approval record](2026-11-23-int021-int030-owner-signoff.md) holds both this
reconfirmation and F1/F7's approval with its separate #30-head AI reference.
Analysis, freshness and executable tests remain. INT-043's proposed 02-
algorithm-token decision is separate and still pending owner/AI review.
