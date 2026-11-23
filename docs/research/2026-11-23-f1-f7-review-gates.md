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
