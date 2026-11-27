# Flex source qualification and compatibility acceptance: family 2 batch 36

Date: 2026-11-27. Docs-only derivation, not a reference execution or port.

## Authority and evidence actually available

L1-SUM-003 and accepted ADR-0014 require bit-for-bit irig106lib flex
compatibility, labelled non-standard, deliberately preserving D7. UC-16
and the 2026-09-25 investigation already record that decision's purpose.
This batch reads those repository documents and the existing checksum/
command contracts. No other repository, executable, credentials or sample
recordings are accessed, and no source is vendored or dependency added.

The historical [reference-tool investigation](2026-09-25-irig106org-tools.md)
section 4 names enI106_Tmats_Signature, uppercase-line Fletcher-32 summation,
default COMMENT/group COM/G/V/R filters, inclusion flags and version/opcode
identity. TEST-DATA D7 records duplicate MPOC4 and omitted DPOC4 in the filter.
ADR-0014 explicitly accepts the quirk. These are reference compatibility
tokens, not new claims about Chapter 9 attribute meanings or validity.

This is existing research, not newly pinned primary-source verification.
It does not fully specify arithmetic initialization/width, reduction/carry,
byte packing, odd-length behavior, control/high-byte handling or malformed
line boundaries. Do not fill those gaps from memory or the standard SHA
algorithm. A pinned relevant source snapshot and reproducible environment
must precede implementation and independent compatibility vectors.

## Derived obligations and what remains incomplete

L2-SUM-004/005 and L3-SUM-011–016 make source fidelity and oracle acceptance
explicit. The parent is not fully implementable merely because children now
exist: reference qualification and complete byte/arithmetic derivation remain
unfinished. No new product algorithm choice is made in this requirements PR.

- Use original supplied source/options with the reference's own preparation;
  do not silently replace it with effective values, INT-036 control omission,
  canonical indices/numeric values, repaired text or SHA exclusion ranges.
- Keep the exact default/inclusion filters and D7, without extending them to
  plausible neighboring tokens or repairing the historical reference typo.
- Reproduce the reference arithmetic and per-line sum, not whole-document
  Fletcher, SHA or accidental host-width operations. Pin the source-derived
  algorithm/options identity independently of public rendering.
- Fix independent vectors with exact bytes/options/results and provenance;
  verify arithmetic independently as well as comparing with the qualified
  reference. Production code or wrapper mocks cannot generate its own oracle.
- Keep undefined/platform-sensitive reference cases as explicit readiness
  gaps. If source analysis exposes a product choice, bring options to the
  owner before encoding one. Do not infer acceptance of arbitrary bytes from
  the lossless parser's ability to retain them.

Sixteen planned flex cases cover provenance, raw/projection separation, byte
preparation, line framing, filters/options, D7, arithmetic boundaries,
permutation and duplicate-line behavior and non-standard identity. Numeric
expected vectors remain BLOCKED on pinned-reference analysis/oracle
qualification, not a new owner choice about whether flex is supported.
Reference runs and source-based detail belong to later reviewed development;
no claim is made that all malformed/high-byte cases are defined.

## Independence of packet data checksums and G\SHA

The owner-relayed follow-up after #68 is explicit in L3-CLI-010/047 and
CLI.md: core PacketInfo's data-checksum state is passed through unchanged.
A G\SHA result neither upgrades nor is upgraded by core Mismatch/NotChecked.
PacketFacts stays header-only, with the evidence alongside it.

The byte domains differ. A G\SHA match does not validate a packet data
checksum. A core Mismatch/NotChecked does not by itself change an independently
evaluated G\SHA result. Preserve both observations and findings; do not
collapse them into a single clean/dirty verdict. This also does not authorize
reading a packet rejected by admission or using an inaccessible record.
The three planned command tests can supply independent evaluated outcomes;
a future real integration test must satisfy its source/access gates first.

Flex remains separate from both: it is not placed in G\SHA, used as standard
verification, or allowed to upgrade record qualification or packet integrity.
This restates ADR-0036 and existing owner checksum contracts, not a new
packet-checksum implementation or quarantine/adoption policy.

## Validation and remaining gates

Local work consists of source inspection and lightweight trace/parent/ID,
fixture-link, inventory and whitespace checks. No Rust builds/tests, C
reference builds, source downloads, installs, packaging, artifact verification
or publishing commands run here. GitHub CI supplies heavy checks; its current
prototype passes do not qualify a new flex implementation.

No new requirements have Evidence, and no planned vector is executed coverage.
Proposed INT-031 still gates incomplete/ambiguous record access. File-order
owner confirmation, std, historical reviews, INT-061/062/063, pattern,
extraction, allocation, governing and report-format holds remain unchanged.
This batch defines no exact human label, JSON field, public flag syntax or
exit code. Source inspection/independent test work must finish before code;
family 3's restructure/parser implementation is not started here.
