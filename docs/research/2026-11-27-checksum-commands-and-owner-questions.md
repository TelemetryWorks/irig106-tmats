# Checksum command computation and owner questions: family 2 batch 35

Date: 2026-11-27. Docs-only derivation; no implementation or owner decision.

## Scope and existing authority

L1-CLI-005 requires checksum, verify, stamp and explicit flex operations.
CLI.md requires thin command layers over reusable library computations.
L2-CLI-012/013 and L3-CLI-045–051 derive command-specific preservation of
the existing library contracts for explicitly supplied eligible documents.
They add no report enum, presentation format, public record selector or
multi-record default. This is a partial command boundary until those held
choices and flex's own compatibility derivation are complete.

Source/authority inspected:

- L2-SUM-001/002 and L3-SUM-001–005, with approved INT-018–021/043,
  establish raw exclusion, authoritative compute and distinct verify results.
- L2-SUM-003/L2-WRT-002 and L3-SUM-006–010 establish the final-byte stamp
  transaction, exact revision-bound repair, re-read verification and findings.
- Their standards sources remain 106-24R1 Ch9 Table 9-2 MESSAGE
  DIGEST/CHECKSUM p. 9-13 and Ch6 section 6.2.3.11 f pp. 6-22–23.
  This batch derives command propagation, not a new interpretation of them.
- ADR-0014 is the accepted non-standard flex compatibility decision;
  ADR-0029 refines stamp. Existing research
  [irig106.org tools](2026-09-25-irig106org-tools.md) describes the flex
  algorithm/quirks, but is not newly executed compatibility evidence.
- ADR-0034 W3 and existing mounting contracts require reusable computation;
  ADR-0036 keeps core's packet reader/evidence outside these wrappers.

A complete terminated malformed SHA value may still have a defined compute
domain. Commands must not collapse that into verify's malformed result or
refuse compute solely because the value is malformed. Conversely, an undefined
or ambiguous domain never becomes an authoritative result via a raw-hash
fallback. Matching digests never prove whole-document validity or governing use.

Warnings survive blocked results, including noncanonical algorithm-2 tokens.
Stamp keeps noncanonical-name findings and the explicit-rename recommendation;
it never silently renames or deletes a duplicate. Permitted exact-item removal
can precede stamp through the existing revision-bound transaction. Success is
final-byte verified output, not an intermediate buffer or delivery guarantee.
Flex stays explicitly requested, non-standard and outside G\SHA. Its exact
algorithm/source and independent compatibility fixtures still need derivation;
these wrapper requirements cannot certify that compatibility by mocking it.

## Owner brief: proposed INT-031 access/invalidation

Current register status: **proposed, awaiting owner; independent review
pending**. It has a proposed default and an optional exception, rather than
an approved A/B decision table. This brief does not amend or approve either.

- **Default proposal:** an incomplete or ambiguous setup supplies no usable
  description. Keep its raw bytes/findings, stop carrying the old setup
  forward, and leave subsequent data ungoverned until a complete setup can
  govern under its own reviewed rules. This avoids silently applying an old
  configuration after a possible change. It can prevent decoding otherwise
  salvageable data; retained evidence remains available for inspection under
  the separately reviewed access contract.
- **Optional proposed exception:** a caller explicitly accepts a particular
  ambiguous split, with the choice/assumption labelled. This permits informed
  recovery but risks using the wrong reading; there is no automatic winner.
  It is not permission for an incomplete record. Approved INT-048 gives any
  retained cut facet Incomplete's full blocking force even when the aggregate
  classification is Ambiguous: no opt-in can make cut text usable/governing.
  The original Ambiguous qualification/findings are never upgraded to Complete.
- **Recommendation for the owner:** keep the no-description/no-silent-carry
  default. If allowing the ambiguity-only exception, require an explicit
  caller choice and retained labels/evidence, never cut text or an automatic
  fallback. The option remains unapproved until the owner decides and the
  independent check under ADR-0033 is complete. Implementation details and
  INT-028/032 governing/assumption dependencies still require their own review.

Retention, raw inspection, semantic access and governing use are distinct.
The requirement references now name proposed INT-031 explicitly; keeping
bytes or enumerating a candidate does not approve its access exception.
Blocked-result command tests can inject evaluated refusals rather than read
an otherwise inaccessible ambiguous/incomplete document.

## Owner brief: what file order means

The merged batch-34 requirements derive **first-fragment order**. Example:
A starts at source offset 100 and ends at 800, while independent-channel B
starts at 300 and ends at 400. First-fragment order is A then B; completion
order is B then A. Both preserve each record's own fragment membership.
These are project-ordering options, not competing Chapter 11 packet rules.

**Recommendation:** confirm first-fragment order. It follows where each
record begins in the file and stays independent of run length, channel and
boundary/finalization timing. Completion order is useful for live arrival of
finished runs, but differs from the source order users inspect. This choice
is being put to the owner; no new owner approval is recorded in this PR.

First-fragment order does not authorize unbounded buffering or decide when
streaming consumers publish provisional/final results. Those resource and
live-emission choices remain separate. The example receives already evaluated
per-channel runs, not invented conformance for a particular recording edition.

## Planned checks and remaining gates

Nineteen new planned cases cover scoped source/revision, text-only byte
domain, compute/verify distinction, every refused/placeholder outcome,
noncanonical warnings, flex separation, explicit duplicate repair, final-byte
verification and failure versus delivery. Independent SHA oracles use hashlib,
not the production command. Mocked propagation alone proves no digest or
flex correctness. Actual integration must exercise eligible real documents.

Local checks cover trace/parent/ID links, case coverage and whitespace only;
heavy checks use GitHub CI. No Rust builds/tests, installs, packaging, sample
access, external repository inspection or release operation runs here.
No new requirement has Evidence. Existing std, historical review,
INT-061/062/063, pattern, extraction, allocation, INT-031 access/governance
and report-format holds remain unchanged. Report/rendered-format derivation
must stop for the owner before selecting findings, assumptions or suppressed
markers. Earlier dated research remains history.
