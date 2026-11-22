# Code-name structure and decimal-index draft — 2026-11-22

Baseline analysis for family 2 batch 4 and proposed INT-038. This is not a
completed cross-edition analysis, independent AI check, owner sign-off or
executable-test report. INT-035/036/037 remain approved unchanged.

## Sources inspected

Baseline: RCC 106-24R1 Chapter 9, January 2025.
[Official PDF](https://www.trmc.osd.mil/wiki/download/attachments/335389015/Chapter9.pdf?api=v2)
([archive](https://github.com/TelemetryWorks/rcc-106-standards/releases/download/106-24R1/Chapter9.pdf)).

- Section 9.4.2, pages 9-2/3: code names versus values, hyphen-indexed
  multiple entries and standalone COMMENT. Section 9.5.1 a and Table 9-1,
  page 9-5: hierarchy, thirteen groups and index placeholders.
- Table 9-2, page 9-10: G\PN (PROGRAM NAME) and G\106 (RCC IRIG 106
  REVISION LEVEL); the digits in 106 are code-element text, not an index.
- Table 9-4, page 9-48: R-x\TK1-n (TRACK NUMBER/CHANNEL ID); the 1 in TK1
  is literal, the group occurrence and suffix n are separate positions.
- Table 9-7, page 9-115: D-x\MN-y-n (MEASUREMENT NAME). Page 9-117:
  D-x\MNF\N-y-n-m (NUMBER OF FRAGMENTS). Multiple indices must retain
  their segment attachment and order; a single trailing-index model loses it.
- Sections 9.5.12/13, page 9-171: H and V naming are not an exhaustive
  built-in attribute list. Section 9.5.14, page 9-172: X can contain the
  original multi-level code path and defines a noncontiguity exception.
  Its D-1\MN-1-2 and X-1\MYORG\D\MN\SGAIN-1-2 examples are isolated
  code-name illustrations here, not proof of a complete valid configuration.

The source check covers the baseline passages above. It is not an audit of
every code row or edition. No annotated Appendix 9-E block is used as a
conforming-file fixture. No prototype mapping is used as an authority.

## Structural recognition versus validation

Keep group recognition, structural decomposition, known-definition matching
and semantic validation separate. Recognizing R does not make every R name
valid; lack of a built-in definition does not prove a vendor/hardware code
malformed. Keep raw names and any unambiguous components when full meaning
is unavailable. Never manufacture a usable prefix key from a broken name.

Known-pattern matching must consume the full interpreted name. G\106 is not
G with index 106; R-1\TK1-3 has no implicit conversion of entry index 3 to
channel ID. Blank/control projection and high-byte rejection follow the
already-approved character policies, with original offsets retained.
Counter and link semantics need their own reviewed requirements. This batch
neither chooses an extension association nor approves a universal grammar
for every private attribute string.

## INT-038 alternatives and recommendation

Recommend structurally accepting decimal digit tokens, keeping raw spelling,
and comparing mathematical identity independently of host integer size.
Leading-zero spellings then compare equal; they never overwrite one another.
Zero remains parseable with semantic validity checked separately. A long
digit token is retained without allocating a collection indexed by its value;
a bounded numeric conversion may fail explicitly without losing the token.

Alternatives: keep digit spelling as identity (01 differs from 1); reject
leading zeros/zero lexically; or cap every index at a machine integer limit.
Spelling identity risks hiding duplicate references to the same numbered
entry. Rejection loses useful structure in damaged input. A host-width cap
makes behavior depend on target architecture. The chosen draft has a cost:
zero-padded names share an identity, so duplicate handling must list every
source occurrence and may make a later typed lookup ambiguous. It cannot
choose a preferred spelling or normalize the document silently.

Chapter 9 describes numbered entries, but does not settle those spelling and
representation choices explicitly. They need owner sign-off and independent
AI review under ADR-0033. No existing accepted INT-004 placeholder behavior
or counter-scope policy is changed or marked newly reviewed by this draft.

## Remaining development evidence

TEST-DATA's batch 4 cases specify multiple indices and path levels, literal
digits, missing/extra material, internal blanks, group/COMMENT recognition,
unknown/private names, leading-zero collisions, zero and very large tokens.
Expected components and source offsets must be independent of the eventual
parser. Use bounded generated digit strings and malformed inputs; do not
allocate huge fixtures or enumerate every index through a supplied maximum.

Full cross-edition/handbook/sample analysis and executable tests remain
outstanding. Additional family-2 batches still need duplicate classification,
lookup and counter/link semantics, extensions, Chapter 10, time views,
checksums, CLI, robustness and release obligations before family 3 begins.
