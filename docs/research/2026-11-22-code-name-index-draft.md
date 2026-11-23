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

## Review amendments — 2026-11-22

The owner relayed **CONCERNS** from the independent AI check on INT-038
at [#19](https://github.com/TelemetryWorks/irig106-tmats/pull/19) head
`b4cbae36e08ee2e89cb6981fc6e0dcd3dcc4ade6`. Numeric identity with raw spelling
retained was sound; the owner requested the following amendments before
sign-off. The baseline above is preserved as history; the amended register
and requirements govern. Owner sign-off and amended-draft recheck remain
pending under ADR-0033, with full development analysis/tests still required.

### Visible canonicalization and early collision findings

Recommend reporting normalization-only full-key collisions during reading,
not waiting for general duplicate classification. Emit located information
for each noncanonical projected token (`01`, `00`), keeping canonical `1`
and `0` distinct from noncanonical spellings. When full keys become equal
only by canonicalizing index spellings, list every original occurrence in a
located information finding now, without selecting a spelling, merging items
or deciding value agreement. Those findings are separate from per-token
normalization findings. Counter semantics and broader duplicates remain later.

Alternative: postpone collisions until the duplicate-classification batch.
That would leave a known identity collision invisible in the intermediate
reader and make downstream users repeat its detection. Early information is
recommended because it exposes a chosen interpretation without changing bytes
or prematurely deciding broader duplicate severity/semantics. Same indices in
different paths/groups are not full-key collisions; X and D stay distinct.

### Projected digits and canonical strings

Apply INT-036 first: `R-1\r\n0\TK1-1` has occurrence identity 10 and the
approved internal-omission information finding on the CR/LF. Its raw index
still spans the controls. Ordinary internal blanks such as `R- 1\` and
`R-1 \` are not trimmed into an index. Whole-name edge layout remains the
approved exception; TAB is discarded for meaning, never replaced by a space.

Strip leading zeros from the projected digit string, retaining one `0` for
all-zero tokens. Identity equality and hashing use that canonical string,
never fixed-width or big-integer conversion. Keep component/index boundaries
when using these identities in a full key; no collision-free hash promise
or numeric-ordering API is introduced. A separately requested bounded numeric
conversion can fail explicitly. The 40-digit fixture uses string equality
and equal-hash input expectations independent of any integer implementation.

### X embedded original-path indices and deferred counters

Re-inspected 106-24R1 Chapter 9 section 9.5.14, page 9-172: X's occurrence
must match the original's first index; the carried original indices match
the original's corresponding indices, while extra extension indices remain
separate. Apply canonical decimal strings to these comparisons too. Thus
`X-1\MYORG\D\MN\SGAIN-1-2` agrees on those positions with
`D-01\MN-01-02`, without establishing uniqueness or resolving the association.
This does not move the carried suffix indices onto different path segments.

Explicitly defer the `\N` count and no-missing-values requirements/checks to
the counter batch. They must count DISTINCT numeric identities, not padded
spellings or repeated item occurrences, within the cited parent scope and
with X's noncontiguity exception. No counter acceptance is established here.
