# Suspect colon-for-semicolon suggestions: family 2 batch 37

Date: 2026-11-27. Docs-only derivation and flex coordination follow-up.

## Existing authority and source inspection

L1-READ-007 already requires the possible-colon-for-semicolon warning and
one-byte suggested replacement, with ROADMAP S1 explicitly suspect. This
batch adds L2-READ-014/015 and L3-READ-034–039; it does not approve automatic
repair, a new delimiter, general edits or public rendering.

The source inspected is 106-24R1 Chapter 9:

- Section 9.4.2 p. 9-2 defines code-name/colon/data-item/semicolon framing,
  ASCII case-insensitive text and intentional blanks. It does not authorize
  a colon to terminate a value.
- Appendix 9-C p. C-8 includes the literal code-name excerpt beginning
  D-1\MML\N-1-1:2: D-1\MNF\N-1-1-1:1: D-1\WP-1-1-1-1:14;.
  This supplies concrete targeted-regression evidence, not proof of intent
  in arbitrary real COMMENT/description values.
- Table 9-2 PROGRAM NAME p. 9-10 supplies G\PN for the small synthetic
  lookahead examples. No new code meaning or default is introduced.

[Official Chapter 9 PDF](https://www.trmc.osd.mil/wiki/download/attachments/335389015/Chapter9.pdf?api=v2),
inspected SHA-256:
`dfc5ebfb3e6f02f58389126863d43a45d89c7e2d0533a1c690d9537531c740cb`.
ADR-0002/0007/0026, approved INT-035–038 and the existing name-recognition
requirements supply retention, suggestions, framing, projections and keys.
The diagnostic is an accepted project heuristic, not a new claim that the
standard defines recovery at a colon; no new interpretation is selected.

## Predicate and immutable source boundary

The triggering colon is inside an already framed value. Lookahead must
positively recognize a complete code name, after the permitted optional
blanks, and its own following colon. The item's actual name/value delimiter,
ordinary time/URL punctuation and partial names do not satisfy that rule.
Do not look across the item's semicolon or beyond source bounds.

Recognition uses existing reviewed whole-name/case/index/character rules;
it cannot fabricate a private V/X decomposition. A coverage gap must remain
visible, not be counted as exhaustive successful detection. Recognizing
lookahead does not parse a second attribute, trim the original value or
insert anything into lookup. A projected name still maps to raw evidence.

For synthetic COMMENT:before: G\PN:x;, the independently counted item span
is [0,23), value [8,22), triggering colon [14,15), following name [16,20).
The proposal replaces exactly byte 14 with semicolon. Nothing applies it:
item count, spans, bytes, lookup and checksum domain stay as originally read.
Repeated positive patterns produce independently located suggestions; a
warning never proves what the author intended.

The existing Appendix 9-C planned case is clarified to assert targeted
warnings separately from other findings in literal example/prose text. It
no longer asserts blanket no-diagnostics or treats applying all proposals
as work performed by the 0.1 detector. General explicit application remains
its later reviewed transaction scope. This is a current fixture contract,
not an edit to the older research record or the source excerpt.

## Suspect status and coverage

Sixteen planned cases cover positive locations, absent optional blank,
multiple candidates, case/control projection, time/URL negatives, partial
names, actual name delimiter, semicolon/EOF boundaries, intentional
lookalikes, unchanged output/hash domain, unavailable grammar and limits.
Fixed input/edit spans are independent of the production detector.

S1 stays suspect until separately authorized real-recording review records
observations, legitimate lookalikes/false positives and severity assessment.
No sample files are accessed here; a synthetic pass does not close that gate.
The separate proposed line-break-inside-value detector remains planned only;
these requirements do not enable it or turn CR/LF into framing terminators.

## Flex reference-qualification coordinator calls after #69

Recorded from the coordinator's direction, without upgrading it to an owner
choice of reference:

- the reference runs only in GitHub CI (bounded, no new project dependency);
- record output values and the pinned revision/hashes, never paste reference
  source into the repo;
- synthetic inputs only.

**PENDING OWNER:** which reference and revision, bbaggerman/irig106lib versus
the atac fork. Neither is selected. The ROADMAP gate and current L2/fixture
status now name this choice; numeric vectors wait for it and subsequent
pinned-source/arithmetic/oracle qualification. ADR-0014's compatibility
purpose and D7 remain accepted, but no unchosen source is silently treated
as the executable oracle. Earlier dated research stays historical.

No reference repository is inspected, run or copied by this batch. The
constraints describe later reviewed CI work, not new CI implementation,
installation, download or permission to add a dependency.

## Validation and remaining gates

Local work is source inspection and lightweight trace/ID/parent, fixture-link,
raw-span/fingerprint, inventory and whitespace checks. No Rust builds/tests,
reference runs, installs, samples, packaging or release commands run here;
heavy checks use GitHub CI. New requirements have no Evidence and planned
cases are not executed coverage.

No new owner decision blocks these diagnostic requirements. Flex source
selection, std, file-order confirmation, historical reviews, INT-061/062/063,
pattern, extraction, allocation, INT-031 access/governance and rendered output
remain held. Internal warning meaning/severity comes from L1-READ-007;
this slice chooses no rendered layout, label serialization or JSON schema.
