# Minimal stamp-transaction derivation — 2026-11-23

Family 2 batch 11 derives the already-approved 0.1 minimal transaction from
L1-SUM-004, L1-WRT-007 and ADR-0029. This is requirements/source inspection,
not completed development analysis or executable verification. No new
interpretation or general edit API is selected. F2/F3 remain open.

## Authorities and boundaries

ADR-0029 validates exact revision-bound targets, rejects stale/ambiguous/
overlapping edits, applies atomically, rebuilds affected state and verifies
re-read emitted bytes. Its final-step stamp covers raw bytes outside the SHA
item, including inserted line breaks. Sources: 106-24R1 Chapter 9 Table 9-2
MESSAGE DIGEST/CHECKSUM page 9-13; Chapter 6 section 6.2.3.11 f page 6-22.
INT-018/019/020, approved F1 and approved INT-043 supply checksum behavior.

An exact duplicate-removal edit precedes post-repair stamp eligibility.
Remaining duplicate/damaged-name/BOM/undefined-domain evidence still blocks
stamping. No edit is silently inferred, no stale target retargeted and no
input bytes changed on failure. Verified checksum output is not whole-file
conformance. The caller's external writer rollback/publication is not a
property promised by a pure library transaction.

Insertion/style inputs must be established, not defaulted to a new newline
or position. This batch does not choose mixed/no-style fallback or promote
the full general editing feature. Those details remain later derivations.
The same holds for flex compatibility, CLI error schemas and open X edits.

## Independent final-byte oracle

A future fixture starts with G\PN:TEST; and explicitly supplies CR/LF before
and after a final inserted G\SHA:2-h; item. The full output bytes are:

`G\PN:TEST;\r\nG\SHA:2-h;\r\n`

The raw preimage is `G\PN:TEST;\r\n\r\n`, retaining both outside-exclusion
CR/LF sequences. Python standard-library hashlib.sha256 generated the oracle
below, independently of the future scanner/stamper. h is replaced by these
64 lower-case hex digits; no printed Chapter 6 example digest is used.

Preimage hex: `475c504e3a544553543b0d0a0d0a`

SHA-256 hex: `07eb8a2f81e689923b4d3524ba6b49a8386b3fbf0c9c1f75f582ea3786c7245f`

## Remaining evidence

TEST-DATA supplies planned target-validation, exact repair, style, final-byte
hash, canonical output, diagnostic-retention and failure-injection cases.
They require implementation and execution later; this note is not a passing
suite. Full source/edition/handbook analysis and applicable interpretation
readiness gates still apply before code. The stamp-specific requirement
slice does not complete all L1-WRT-007 or the rest of family 2.
