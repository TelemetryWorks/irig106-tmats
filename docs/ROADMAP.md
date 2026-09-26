# irig106-tmats Roadmap

> **This roadmap is forward-looking only.** Completed work is not tracked here —
> it lives in `CHANGELOG.md` (release history), `docs/L1-REQ.md` /
> `docs/L2-REQ.md` / `docs/L3-REQ.md` (the normative requirements), and
> `docs/TRACE-MATRIX.md` (verification status), all backed by git history.
>
> **Do not mint requirement IDs (`L2-*`, `L3-*`) in this file.** An ID is born
> only when its requirement is written in `docs/L2-REQ.md` / `docs/L3-REQ.md`;
> describe intended work in prose and let the requirement process own the IDs.
> Provisional IDs minted in a roadmap collide with real assignments once the
> requirement documents catch up.
>
> Likewise, do not record test counts, line counts, or requirement counts here.
> They drift the moment they are written; the trace matrix is the source of
> truth for coverage.

## Where the project stands

The original implementation was a prototype. A review against IRIG 106
Chapter 9 found that its core design could not be made correct incrementally:
it lost data on a parse/serialize round trip, mapped several P-group code names
to the wrong meanings, and hand-coded validation that disagreed with its own
attribute registry. It is preserved at the git tag `prototype-0` and will be
removed from `main` when the new implementation's first code lands.

The project is being rebuilt **documentation first**: nothing below is
implemented until the design phase has produced reviewed use cases,
architecture, ADRs, and L1/L2/L3 requirements.

## Queued for the next release (`[Unreleased]`)

`[Unreleased]` is emptied at each release cut; whatever sits above the most
recent dated section in `CHANGELOG.md` is the live queue.

## Design phase (before any new code)

Produced in this order, each reviewed before the next begins:

1. **Use cases and concept of operations** — who reads, writes, validates,
   and extends TMATS, from which sources (Chapter 10 setup records, standalone
   `.tmt` files, telemetry network metadata), and what they need back.
2. **Architecture and data flow** — the lossless ordered attribute store, the
   code-name grammar, the layered attribute registry, the validator, the
   edit/suggestion model, and the Chapter 10 setup-record boundary.
3. **ADRs** — written for the decisions taken so far (`docs/adr/`); two
   architecture ADRs (0020, 0021) are proposed pending the architecture
   review.
4. **L1 → L2 → L3 requirements** — L1 drafted, the trace-matrix script and
   test-marker convention in place (CI-checked); L2 and L3 follow the
   architecture review.
5. **Standards baseline** — the RCC 106 archive (`TelemetryWorks/rcc-106-standards`)
   in place, so every registry entry can cite an edition, table, and row.

### Decisions

Decisions taken so far are recorded in `docs/adr/` (index in
`docs/adr/README.md`). Still to decide: whether `irig106-cli` mounts the
`tmats` commands directly, which would mean `irig106-tmats-cli` also exposes
its command implementations as a library under the same lockstep and semver
rules (ADR-0011).

## Specification coverage plan

A section-by-section review of IRIG 106-24R1 Chapter 9 (the baseline
edition) against `docs/ARCHITECTURE.md`, `docs/adr/`, and `docs/L1-REQ.md`
(2026-09-26) found the chapter covered at the section level except for the
items below. They are additions and clarifications, not changes of
direction; each is folded into the architecture, the ADRs, and the L1
requirements before L2 is written. New requirement IDs are assigned in
`docs/L1-REQ.md`, not here.

Checked and not a gap: the Q group's "the first item is numbered 0"
(§9.5.10) refers to element and field offsets, which are values; Q code-name
indices are 1-based like every other group's, so the index-contiguity rule
stands.

### Coverage work items

1. **Prove attribute-level completeness mechanically.** No requirement yet
   says the built-in registry defines every code name in Tables 9-2 through
   9-11 of each supported edition. Add an L1 registry requirement for it,
   verified by a CI check that compares the Chapter 9 table extractor's
   output with the registry: every code name is either defined or explicitly
   excluded with a reason. This check is the standing proof of coverage and
   keeps it current when a new edition is published.
2. **Treat the H group as an extension mechanism.** *Applied with T5
   (ADR-0027, L1-EXT-004, INT-006).* §9.5.12 reserves H for
   user-defined airborne-hardware attributes, tied to G by `H\TA`, with
   `H\ST-n` determining how the rest are interpreted — the same pattern as
   the V group. Extend ADR-0008 and the extensibility requirement to cover H
   (grouping by test item and system type; user-supplied definitions).
3. **Decide the scope of Appendices 9-D and 9-E, and of conversions.**
   Appendix 9-D (floating-point formats) is referenced by `C-d\FPF` and
   S-group enumerations; validating those names is already covered, while
   interpreting raw bits is `irig106-decode`'s work and should be stated as a
   non-requirement. Appendix 9-E (derived-parameter grammar) governs the
   value of `C-d\DPA` when `C-d\DPAT` is `A`. *Resolved by team review T2
   (2026-09-26): the library parses and validates derived-algorithm syntax
   and describes it; evaluation, engineering-unit conversion, and the
   interpretation of floating-point bit patterns are non-requirement NR-007.* Evaluating any C-group conversion to engineering units is stated as
   out of scope either way (it is implied today but not written).
4. **Follow every link, not only the pictured ones.** §9.5.1 b says "Not all
   valid paths are shown" and that all are documented in the "Links to /
   Links from" fields (for example the R-group filtering and overwrite links
   into P, D, and B). Reword the channel-view requirement to follow every
   registry link.
5. **State how nonprintable characters are interpreted.** §9.4.1: TMATS is
   7-bit ASCII and "nonprintable characters will be discarded by the
   destination agency". Keep them in the bytes (lossless), ignore them for
   meaning, and report them.
6. **Warn on `OTH` without an explanation.** Many enumerations offer "OTH –
   Other, define in comments", and §9.2 places nonstandard values in each
   group's comments. **Owner decision needed:** whether a missing comment is
   a warning by default.
7. **Check the §9.4.2 recommendations.** Blanks should not appear in code
   names, keywords, or link values, and link and measurement names should use
   only capitals, digits, and `_`. **Owner decision needed:** warnings by
   default, matching the decision on recommended maximum lengths.
8. **Declare the cover sheet out of scope.** Appendix 9-B concerns exchanging
   physical media; record it as a non-requirement.
9. **Complete the citations.** Appendix 9-F cites IEEE 1588-2008, RFC 1305,
   and RCC 200-16; add RFC 1305 and RCC 200-16 to the standards archive's
   "cited but not mirrored" list (RCC 200 may instead be mirrored, since it is
   an RCC document).

Also carried into L2 (already covered at L1): the §9.4.2 scientific-notation
regular expression, line breaks as insignificant between attributes, one
mission configuration per document, and the D-group location types and
subframes removed as of 106-11 as an edition delta.

### Team design review (in progress)

The team reviewed `docs/ARCHITECTURE.md`, the ADRs, and `docs/L1-REQ.md`
against 106-24R1 Chapter 9 (Appendices 9-A to 9-F) and Chapters 6 and 11.
Their summary: keep the lossless document, owned byte buffer, generated
registry, explicit edits, and library/CLI separation; the main gaps are in
semantic validation, appendix coverage, and setup-record handling. They
raised seven priorities plus further comments; the text is recorded verbatim
in `docs/research/2026-09-26-team-design-review.md`. Each priority is
checked against the archived standard, mapped to the existing documents and
the coverage items above, and planned here. Items are added as they arrive.
Every interpretation the review produces goes into `docs/INTERPRETATIONS.md`,
and **every entry there requires intensive testing and deep analysis during
development** (owner, 2026-09-26): a written analysis across every archived
edition it touches, and tests beyond the focused one, before its code is
written.

**T1. Make the registry an executable specification, with reviewed
interpretations.** *Applied 2026-09-26: `docs/ARCHITECTURE.md` section 4,
`docs/diagrams/data-flow.svg` and `registry-pipeline.svg`, ADR-0022 and
ADR-0023 (with status pointers on ADR-0004, 0005, 0006), and L1-REG-002 to
004, L1-VIEW-004, L1-VAL-005 and 006. Implementation follows L2.* Confirmed against 106-24R1: `C-d\DPNO` has its default
only in prose ("Default is 1.", Table 9-11; the chapter has 107 `Default:`
fields and further prose-only defaults), and `G\106` is "Required when:
Always" (Table 9-2), which a pass over present attributes cannot report
missing. Also found: `C-d\DPNO`'s condition names `C\DCT` without an index,
so the occurrence it refers to needs an explicit interpretation. Mapping:
strengthens ADR-0004, ADR-0006, the registry requirement (L1-REG-002), and
the validation requirements (L1-VAL-001/002); overlaps coverage item 1;
nothing conflicts. Plan:

1. *Two layers per registry entry.* A **source layer** holding the table row
   exactly as printed (parameter, code name, usage-attribute text, definition
   prose) with its citation, produced by the table extractor and never edited
   by hand; and an **interpretation layer** holding reviewed, executable
   rules — conditions in a small defined condition language, the default and
   where it came from (Default field or prose), typed ranges, links — with
   reviewer, review date, and a note wherever the interpretation is not
   literal. A CI check requires every source row to have an interpretation or
   an explicit "not yet interpreted" status; each interpretation stores a
   hash of its source text so a changed row (for example in a new edition) is
   flagged for re-review.
2. *Condition semantics, defined once in a new ADR:* occurrence scope (same
   occurrence, linked occurrence, or document-wide, declared per rule);
   whether a condition follows a link; a missing or invalid dependency makes
   the condition "cannot evaluate", reported as its own finding, never
   silently true or false; whether conditions see defaulted values, declared
   per rule.
3. *Four validation passes* replacing ADR-0006's single pass: present
   attributes (known, allowed, range and type); presence (Required-when and
   R/R Ch 10 Status per applicable occurrence — this detects a missing
   `G\106`); counters and index contiguity; relationships (links resolve,
   keys unique). Each finding names its pass.
4. *Effective values* without changing the document: explicit (value and
   location), defaulted (value and citation), missing, invalid (raw text and
   reason), or ambiguous (for example, conflicting duplicates). Defaults are
   never inserted into the stored bytes (ADR-0002).
5. *Proof and tests:* the completeness check (coverage item 1) also covers
   prose defaults; every reviewed interpretation has tests; the first
   regression tests are "`G\106` absent is reported" and "`C-d\DPNO` absent
   is defaulted to 1".

Documents affected: two new ADRs (two-layer registry with condition
semantics; validation passes and effective values). ADR-0006 keeps its
decision — registry-driven validation with severity policy and user rules —
and its status line becomes "mechanism superseded by" the new validation
ADR, following the rule that records are superseded, not rewritten;
ADR-0004 gains a similar pointer. L1-REG-002 is strengthened, and new L1
requirements cover missing-required detection and effective values.
**Decided (owner, 2026-09-26): a two-person rule for interpretation
reviews.** Every interpretation records two different people: its author and
an independent reviewer. An interpretation drafted with tooling or an AI
assistant still needs both people; the tool counts as neither. The CI check
from step 1 enforces it: an interpretation whose author and reviewer are
missing or identical fails, and a re-review after a source change needs both
again.

**T2. Add explicit Appendix 9-E (derived parameter) support.** Verified
against 106-24R1 Appendix 9-E and Table 9-11:

- *Two forms* (§E.1): function style (`C-d\DPAT` = `N`; `C-d\DPA` names an
  operator, a function, or a custom algorithm such as `NEWALG` in §E.9.d, with
  ordered inputs `C-d\DP-n` and constants `C-d\DPC-n`; order carries meaning —
  "the division algorithm assigns the first input measurement as the
  dividend") and formula style (`C-d\DPAT` = `A`; `C-d\DPA` holds an
  expression; inputs and constants are not used).
- *Precedence differs from C*: §E.7 says the grammar "strictly speaking, does
  not match the C language". Table E-6 binds `& ^ |` tighter than `* / %`,
  and `+ -` tighter than `<< >>`. Checked: Table E-6 and the Yacc `%left`
  declarations (Figures E-2/E-3) agree, lowest to highest `,` `?:` `||` `&&`
  `== !=` relational `<< >>` `+ -` `* / %` `|` `^` `&` `**` `! ~` unary minus.
- *Lexical rules* (§E.4–E.5, Lex figures): measurement names of
  alphanumerics plus `$ _ .` (for example `A00.1`); names quoted with `"` or
  `'` (for example `'Air Speed'`); names such as `00A1` versus hexadecimal
  `0x…`; decimal and scientific constants; case-insensitive throughout.
  Table E-9 lists "selected" functions (`sin`, `atan2`, `pow`, …), so other
  names are allowed (custom algorithms).
- *Dependencies*: derived measurements may use other derived measurements
  (§E.5; §E.9.d chains `XA` → `XB` → `XC` → `DMD`), and appear only in the C
  group (§E.1).
- *Triggers*: `C-d\DPTM` (trigger measurand) and `C-d\DPNO` (occurrences,
  default 1 in prose); with a single input and no trigger, "there is only one
  input, which must trigger the calculation" (§E.9.c).
- *Scanner*: a colon is legitimate inside a value — §E.6.b example h is
  `A<B || B<<C ? D : E`, so `C-1\DPA:A?B:C;` is valid. Only the first colon
  of an attribute separates code name from value. Also found: the standard's
  own example `C-6\DCN :DMC;` (§E.9.c) has a blank before the colon, so blanks
  around the code name must be tolerated for interpretation (while kept in
  the bytes).
- *Errata to interpret (two-person review)*: Table E-3 prints the equality
  operator as `= =` while the Lex grammar recognises `==`; the figure numbers
  in the body differ from the List of Figures.

Mapping: not covered today (ARCHITECTURE and L1 treat `C-d\DPA` as a plain
string); it answers coverage item 3's open question about Appendix 9-E with a
recommendation; the scanner rule refines ARCHITECTURE point 1 and ADR-0021;
no conflict with any decision. Plan:

1. *A derived-expression component* in the library: a lexer and parser for
   formula style implementing Table E-6 exactly (with the verified Yacc
   declarations as the cross-check), and a binder for function style
   (operator or function name, ordered inputs, constants). Output is an
   interpretable description — an expression tree or a bound function call —
   with spans back to the `C-d\DPA` bytes.
2. *Validation*: syntax errors with locations; arity of known Table E-9
   functions; unknown function names reported as custom algorithms (a
   warning, not an error); inputs and constants used only in the style that
   allows them; unresolved measurement names; and cycles among derived
   measurements.
3. *Dependencies*: for each derived measurement, the measurements it reads,
   resolved to telemetry measurements (R, M, D, B, S groups) or other derived
   measurements (C group), as a graph in the link-graph layer, with cycle
   detection.
4. *Trigger semantics*: the trigger measurand (explicit, or implied by a
   single input), occurrences with its prose default of 1 (an effective value,
   T1), reported with each derived measurement.
5. *No evaluation*: computing values stays with `irig106-decode`, which
   consumes the description; stated as a non-requirement, together with
   engineering-unit conversion (coverage item 3).
6. *Scanner rule*: the first `:` in an attribute ends the code name; every
   later `:` belongs to the value, which ends at `;` (semicolons are not
   allowed in data items, §9.4.2). Blanks around the code name are ignored for
   interpretation and kept in the bytes.
7. *Tests*: every expression and TMATS example in Appendix 9-E (§E.6.b e–h,
   §E.9.a–d, function and formula styles) as fixtures; a precedence test per
   level of Table E-6, including the cases where C would differ; the
   colon-in-value and blank-before-colon cases.

Documents affected: ARCHITECTURE (the derived-expression component, the
dependency graph, the scanner rule), a new ADR (Appendix 9-E: parse,
validate, and describe; no evaluation), ADR-0021 (status pointer for the
scanner rule), L1 (derived-parameter parsing, validation, dependencies,
triggers; the scanner rule; a non-requirement for evaluation).
**Decided (owner, 2026-09-26):** the library parses, validates, and describes
derived parameters; `irig106-decode` evaluates them. Added to the plan: a
reference evaluator that exists only in the tests, to prove the Table E-6
precedence (for example that `2 + 3 & 1` means `2 + (3 & 1)`), and never
ships; and the reading of the errata — `==` is the equality operator (the
grammar is the machine-readable source) and `= =` is accepted with a warning.
*Applied 2026-09-26: `docs/ARCHITECTURE.md` section 5, the data-flow and
derived-parameters diagrams, ADR-0024 (with a status pointer on ADR-0021),
L1-DER-001 to 005, L1-READ-006, and NR-007.*

**T3. Distinguish a setup-record packet from a complete setup record.**
Verified against 106-24R1 Chapter 11:

- *Spanning* (§11.2.7.2): "A single setup record may span multiple
  consecutive packets. When spanning multiple packets, the sequence counter
  shall increment in the order of segmentation of the setup record, n+1."
  The packet body of each Format 1 packet begins with the CSDW, so every
  fragment carries its own CSDW. The maximum packet size is 512 KB
  (§11.2.7.3).
- *No end marker*: nothing marks the last fragment of a setup record, so the
  boundary between one multi-packet record and the next is not defined by the
  standard and needs a reviewed interpretation.
- *Slicing* (§11.2.1.1–11.2.1.4): a 24-byte header (sync `0xEB25`); a 12-byte
  secondary header when packet-flags bit 7 is 1; Data Length "includes
  channel-specific data … and data but does not include packet trailer filler
  and data checksum"; filler of `0x00` or `0xFF` for 32-bit alignment;
  packet-flags bits 1–0 declare an 8-, 16-, or 32-bit data checksum; header,
  secondary-header, and data checksums are defined. The TMATS text is the
  body from offset 4 (after the CSDW) to Data Length.
- *Sequence numbers* are per channel, 8-bit, and roll over after `0xFF`.
- *CSDW* (Figure 11-34) — this confirms the bit positions left open by
  ADR-0009: bits 31–10 reserved; bit 9 FRMT (0 ASCII, 1 XML); bit 8 SRCC;
  bits 7–0 RCCVER. "It is not permissible to have both ASCII and XML
  Chapter 9 TMATS attributes in the same session." When SRCC is 1, "a setup
  record configuration change event packet shall be inserted into the
  stream" before the new setup record.
- *Channel 0* (§11.2.1.1 b): "as of 106-17" channel ID `0x0000` carries only
  setup records and streaming-configuration records.
- *Also found — edition codes*: RCCVER defines `0x07` = 106-07 through
  `0x0E` = 106-22 and reserves `0x0F`–`0xFF`; the header's Data Type Version
  stops at `0x0A` = 106-22. The CSDW therefore cannot distinguish 106-22,
  106-23, and 106-24, and 106-20 has no code. `irig106-time` maps `0x0F` to
  106-23, which contradicts the standard; recorded here for the
  `irig106-types` work (ADR-0009), not changed in this repository.
- *Also found — multiplexer source bits*: `R-x\NSB` gives the number of
  channel-ID high bits that identify a multiplexer source (§11.2.1.1 b), so
  channel views (UC-05) must split channel IDs accordingly.

Mapping: contradicts UC-02 ("each is read independently") and under-specifies
L1-CH10-001, L1-CLI-003, ADR-0019, and the `G\SHA` checksum (which must cover
the assembled record, not one fragment); consistent with ADR-0010 if the
assembler takes bytes, not files. Plan:

1. *Assembly contract.* Fragments enter an assembler with their provenance —
   file offset, channel ID, sequence number, relative time counter, CSDW
   fields, data-type version, and the fragment's TMATS bytes. A complete
   setup record leaves it: the concatenated TMATS body, one CSDW summary, and
   a map from every byte of the body back to its packet and offset (for
   diagnostics). Only complete records enter the document parser.
2. *Assembly rules* (a reviewed interpretation, because the standard defines
   no end marker): fragments of one record are consecutive data type `0x01`
   packets on one channel whose sequence numbers increase by one modulo 256
   and whose CSDWs agree on FRMT and RCCVER; a record ends at an intervening
   packet, a sequence discontinuity, a CSDW change, or end of input. Anything
   ambiguous is reported, never guessed.
3. *Slicing rules* for the packet reader: body after the header and the
   optional secondary header; TMATS = body from offset 4 to Data Length;
   filler and data checksum excluded; header and secondary-header checksums
   verified (the length fields cannot be trusted otherwise — this narrows
   ADR-0019's "checksums out of scope"); data checksum verified and reported
   when present.
4. *Checksums*: `G\SHA` and the flex signature are computed over the
   assembled record.
5. *Session rules* reported across setup records: ASCII and XML not mixed;
   SRCC = 1 preceded by a configuration-change event packet; a setup record
   on a channel other than `0x0000` in a 106-17-or-later recording (a
   warning).
6. *Edition codes*: RCCVER `0x0E` reads as "106-22 or later"; reserved values
   are reported as unknown; `G\106` stays the primary edition source
   (ADR-0016).
7. *Channel IDs*: channel views apply `R-x\NSB` to separate the multiplexer
   source ID from the channel ID.
8. *Tests* (synthesized recordings): one-packet and multi-packet records,
   sequence rollover `0xFF` → `0x00`, secondary header present and absent,
   filler, each checksum width, back-to-back records, a configuration change
   with SRCC = 1, an XML record, a sequence gap, a corrupt header checksum.

Documents affected: `docs/USE-CASES.md` (UC-02, UC-17), ARCHITECTURE (the
assembler and the slicing rules; system-context and data-flow diagrams), a
new ADR (the setup-record assembly contract), ADR-0019 (status pointer:
header checksums now verified), L1-CH10-001 and L1-CLI-003 revised, new L1
requirements for assembly, provenance, slicing, and session rules.
**Decided (owner, 2026-09-26): the assembler lives in the library**
(fragments with provenance in, complete records out; no I/O); packet slicing
stays in the CLI's reader until `irig106-core` exists. The multi-packet case
has a named acceptance test, `multi_packet_setup_record_is_assembled`
(`docs/TEST-DATA.md`), and a diagram (`docs/diagrams/setup-record-assembly.svg`).
*Applied 2026-09-26: `docs/USE-CASES.md` (UC-02, UC-17), `docs/ARCHITECTURE.md`
section 6, the system-context and data-flow diagrams, ADR-0025 (with a status
pointer on ADR-0019), L1-CH10-001 and L1-CLI-003 revised, L1-CH10-004 to 006,
L1-CLI-008, L1-VIEW-005.*

**T4. Define counter scopes, key namespaces, and ambiguous links
precisely.** Verified against 106-24R1 Chapter 9:

- *X-group occurrences*: "The values of "x" in "X-x" are not necessarily
  contiguous" (§9.5.14) — an exception to "have no missing values"
  (§9.5.1 a).
- *Nested counters*: 49 distinct counters carry parent indices (for example
  `D-x\MN\N-y`, `B-x\NML\N-i-n-p`, `Q-d\NSF\N-i-n-m-o`) and count items only
  within one parent-index combination; 37 more carry only the group
  occurrence index. Some conditions name counters with no index at all
  ("Allowed when: D\MNF\N > 1"), which is T1's occurrence-scope question.
- *Keys are per attribute, not per file*: `P-d\DLN` lists "Links from: …
  R-x\CDLN …" and "Links to: D-x\DLN, B-d\DLN", and `D-x\DLN` lists "Links
  from: P-d\DLN". Both are keys under "Any attribute with a Links from: is a
  key and must be unique in the TMATS file" (§9.5.1 a), yet they carry the
  same value by design — uniqueness can only mean among the values of the
  same attribute.
- *Case*: "For alphanumeric data items, including keywords, either upper or
  lower case is allowed; TMATS is not case sensitive" (§9.4.2) — so keywords
  and link values compare case-insensitively, not only code names
  (L1-READ-004 names only code names).
- *The two ends of a link can disagree*: `R-x\CDLN-n` and `R-x\EV\DLN-n`
  list "Links to: P-d\DLN, B-x\DLN, S-d\DLN", yet `Q-d\DLN` lists "Links
  from: R-x\CDLN, R-x\EV\DLN-n" — Q appears only on the receiving side. The
  B group's own table names the attribute `B-x\DLN`, while `P-d\DLN` points
  to "B-d\DLN".
- *Link targets overlap*: `B-x\DLN` is linked from both `R-x\CDLN` and
  `P-d\DLN` ("Links from: R-x\CDLN, P-d\DLN, …"), and `P-d\DLN` "Links to:
  D-x\DLN, B-d\DLN". Bus data carried in a PCM stream therefore shares the P
  group's data-link name, and a recorder channel naming that stream matches
  both a P and a B group. What distinguishes them is the channel data type
  (`R-x\CDT-n`); the S and Q groups' own conditions ("Allowed when: R\CDT is
  either …") point the same way.
- *Also found — an erratum in the standard's own example* (**suspect**, see
  below): Appendix 9-C, page C-8 of 106-24R1, prints `D-1\MML\N-1-1:2:
  D-1\MNF\N-1-1-1:1: D-1\WP-1-1-1-1:14;` — colons where semicolons are meant.
  Re-verified at the owner's request: **18** places (a first count of 12 came
  from a scan that skipped every second one), all after a D-group counter in
  D-1 and D-2; D-3 and D-4 are correct; no other delimiter anomaly in the
  appendix; the page image agrees with the text layer; and the same 18 occur
  in every edition since the example appeared in 106-17. A scanner following
  §9.4.2 reads each as one attribute whose value contains the next ones.
  (A first reading that some counters appear with two index depths was an
  artefact of PDF line wrapping, not the standard; the table extractor must
  join wrapped code names.)

Mapping: refines L1-VAL-002 and L1-READ-004, ADR-0022 (the interpretation
layer), ADR-0023 (passes 3 and 4), and the "ambiguous" effective-value state
of T1; affects the Appendix 9-C spec fixture; no conflict. Plan:

1. *Counter declarations* in the interpretation layer: which code pattern
   and which index position a counter governs, and its parent scope (the
   indices held fixed). Contiguity from 1 to N applies per parent-index
   combination unless the registry records a cited exception (the X group).
2. *Link declarations*: each linking attribute names its target attribute(s)
   — its namespace, for example `R-x\CDLN-n` → `P-d\DLN`, `B-x\DLN`,
   `S-d\DLN`, `Q-d\DLN` — a **selector** where the targets overlap (for
   `R-x\CDLN-n`, the channel data type `R-x\CDT-n` of the same channel), and
   its cardinality (exactly one, at most one, or many). The declarations are
   built from both the "Links to:" and the "Links from:" fields; where the two
   sides disagree (Q above) the interpretation records how it was resolved,
   under the two-person review. Resolution returns **resolved**,
   **unresolved**, or **ambiguous** (several candidates after the selector,
   all listed); the library never picks one.
3. *Key uniqueness per namespace*: a key must be unique among the values of
   the same attribute, in the scope the registry declares (whole document or
   within a parent occurrence); equal values across linked attributes are the
   link, not a conflict.
4. *Comparison rules*: code names, keywords, and link values compare
   case-insensitively (ASCII case folding); original spelling is kept. Blanks
   in link values are compared as written and reported (coverage item 7).
5. *Suspected missing semicolon* (**suspect**): when a value contains a
   colon followed, after optional blanks, by a complete code name and its own
   colon, report "possible `;` typed as `:`" as a warning with a suggested
   edit (ADR-0007) — never split silently (lossless). A colon inside a value
   is otherwise legitimate (`C-1\DPA:A?B:C;`, section 5.4 of the
   architecture), so only a following code name triggers it. The Appendix 9-C
   fixture keeps the standard's text verbatim and its test expects exactly
   the 18 diagnostics.
6. *Tests*: X-group occurrences 1 and 5 (no gap reported); nested counters
   across several parent combinations; `P-d\DLN` equal to `D-x\DLN` (no
   conflict); two P groups with the same data-link name (ambiguous link, both
   candidates); a PCM channel whose stream carries bus data (P and B share the
   name; the selector resolves to P); keyword and link-value case variants;
   the Appendix 9-C errata; a derived expression with colons (no diagnostic).

Documents affected: ARCHITECTURE (a section on counters, keys, links, and
comparison), a new ADR (counter scope, link namespace and cardinality, key
uniqueness, comparison rules), ADR-0022 and ADR-0023 (status pointers),
L1-VAL-002 and L1-READ-004 revised, new L1 requirements for ambiguous links
and the suspected-missing-semicolon diagnostic, the link-graph diagram
(cardinality and ambiguity), and `docs/TEST-DATA.md` (the Appendix 9-C
errata).
*Applied 2026-09-26 at the owner's direction: `docs/ARCHITECTURE.md` section
7, `docs/diagrams/link-resolution.svg` (new) and `link-graph.svg`, ADR-0026 (with status pointers on ADR-0022
and ADR-0023), L1-READ-004 and L1-VAL-002 revised, L1-REG-005, L1-VIEW-006,
L1-READ-007, and `docs/TEST-DATA.md`. The erratum and L1-READ-007 stay
suspect (below).*

**T5. Record standards inconsistencies instead of assuming tables are
mechanically complete.** Verified against 106-24R1 Chapter 9:

- *R to Q*: `R-x\CDLN-n` (Table 9-4) lists "Links to: P-d\DLN, B-x\DLN,
  S-d\DLN"; `Q-d\DLN` (Table 9-10) lists "Links from: R-x\CDLN,
  R-x\EV\DLN-n"; §9.5.1 b (h) says "The tie from the R group to the Message
  Data group (S) or Message Structure Definition Group (Q) is from the
  Channel Data Link Name, Sub-Channel Name, or Network Name (R) to the Data
  Link Name (S) or Data Link Name (Q)". `R-x\EV\DLN-n` omits Q the same way.
- *Also found — ties that exist only in prose*: §9.5.1 b (g) and (h) name
  the R group's sub-channel and network names as sources of ties to B, S,
  and Q, yet `R-x\ANM-n-m` (ARINC 429), `R-x\UCNM-n-m` (UART),
  `R-x\MCNM-n-m` (message), `R-x\ENAM-n-m` (Ethernet), and `R-x\CBM-n-m`
  (CAN) have **no "Links" field**, and the B, S, and Q data-link names do not
  list them under "Links from:". Reading both directions of the fields still
  misses these ties.
- *The standard's completeness claim does not hold*: §9.5.1 b says "All valid
  paths are documented in "Links to:" and "Links from:" attributes"; the two
  items above contradict it.
- *H group*: "The only H group attributes defined in this standard are …
  Test Item (code name H\TA) - specifies the item under test and ties the H
  group to the G group" and "Airborne System Type (code name H\ST-n)"; the
  rest is reserved "for those instrumentation organizations that choose to
  use the TMATS standard in this way" (§9.5.12). §9.7 adds that IHAL was
  adopted "to serve the purpose originally intended for the Airborne Hardware
  Attributes (H) group …, which has never been implemented". So H has one
  defined tie (`H\TA` to G) and no table to transcribe. The caption in
  `docs/ARCHITECTURE.md` section 3 is wrong; coverage item 2 already has it
  right.

Mapping: extends ADR-0022 (the interpretation layer gains a register of
inconsistencies) and ADR-0026 (link declarations gain a third source, the
§9.5.1 b prose); corrects `docs/ARCHITECTURE.md` section 3 and the link-graph
diagram; carries out coverage item 2 for H; no conflict. Plan:

1. *An interpretation register* — one entry per place where the standard is
   inconsistent, incomplete, or silent and the design had to choose. Each
   entry has a permanent identifier, the conflicting citations quoted
   verbatim, the chosen behaviour, the reason, the author and reviewer
   (ADR-0022's two-person rule), a status (accepted, or suspect as in S1),
   and **a focused test** that pins the behaviour. A test names its entry
   with a doc-comment marker, and the trace-matrix script reports any entry
   without a test.
2. *Relationships from three sources*: "Links to:", "Links from:", and the
   ties of §9.5.1 b. The registry generator takes the union as candidates;
   every relationship that appears in only one source must have a register
   entry, or the generator's CI check fails. Relationships are generated
   only from reviewed definitions.
3. *Extraction never assumes completeness*: the table extractor also
   reports links that name code names not defined in any table, conditions
   that name unknown attributes, and prose that defines an attribute outside
   the tables (`H\TA`, `H\ST-n`). The coverage check (coverage item 1) counts
   these, so nothing is silently dropped.
4. *H group*: `H\TA` and `H\ST-n` are built-in, with the `H\TA` → G tie
   (read as matching `G\TA`, a register entry); every other H attribute is
   organisation-defined, supplied through the user overlay like V (coverage
   item 2, ADR-0008), and preserved when no definition is given. Fix the
   architecture caption, and add H with its one tie to the link-graph
   diagram.
5. *First register entries* — the inconsistencies and interpretations
   already found, so the register starts complete:
   R → Q from both link sources; sub-channel and network names → B, S, Q
   (prose only, including which name reaches which group); the §9.5.1 b
   completeness claim; `B-x\DLN` printed as "B-d\DLN" in `P-d\DLN`; the
   channel-type selector for overlapping targets (T4); `H\TA` → G;
   `C-d\DPNO`'s unindexed `C\DCT` (T1); unindexed counters in conditions
   such as "D\MNF\N > 1" (T1, T4); `==` against `= =` (T2); the setup-record
   boundary rule and RCCVER `0x0E` as "106-22 or later" (T3); the
   Appendix 9-C colons (T4, suspect S1).
6. *Tests*: an R channel whose data-link name matches only a Q group
   resolves; a message sub-channel name resolves to its S group; `H\TA`
   ties to G; an H attribute with no user definition is preserved and
   reported as organisation-defined; a relationship present in one source
   only is reported by the generator check.

Documents affected: a new register document (for example
`docs/INTERPRETATIONS.md`, becoming generated from the registry's
interpretation files once they exist), a new ADR (the register and
three-source relationship generation), ADR-0022 and ADR-0026 (status
pointers), ARCHITECTURE (section 3 caption; a section for T5), the link-graph
diagram (H), L1 requirements for the register and for relationships from
all three sources, ADR-0008 and L1-EXT for H (coverage item 2),
`scripts/build-trace-matrix.py` (register markers), and
`docs/PROJECT_STRUCTURE.md`.
**Owner decision needed:** whether the register starts now as a hand-written
document that the registry later generates, or waits for the registry.
Recommendation: start it now, seeded with the entries in step 5, so the
findings of T1–T5 are gathered in one place before any code.
**Decided (owner, 2026-09-26): start the register now.** Two more
inconsistencies were found while writing it and entered: the Q group's
condition names the channel type `FBCIN`, which Table 9-4 does not define
(`FBCHIN`; every edition since 106-22), and no B, S, or Q group accepts CAN
data (`CANIN`), so CAN sub-channel names tie to nothing.
*Applied 2026-09-26: `docs/INTERPRETATIONS.md` (INT-001 to INT-013),
`docs/ARCHITECTURE.md` section 8 and the section 3 caption, the
link-graph (H) and registry-pipeline (register, check 5) diagrams, ADR-0027
(with status pointers on ADR-0008, 0022, 0026), L1-REG-006 to 008,
L1-EXT-004, and the register report in `scripts/build-trace-matrix.py`.
Also corrected: the T4 documents gave the D-group fragment counter as
`D-x\MNF\N-y-n`; it is `D-x\MNF\N-y-n-m` (Table 9-7).*

**T6. Separate the TMATS edition, the recording-format version, and the
validation rules selected.** Verified against the archived editions:

- *`G\106` declares the edition that generated the TMATS file, as two
  digits*: "Version of RCC IRIG 106 standard used to generate this TMATS
  file. The last 2 digits of the year should be used. Use a leading 0 if
  necessary." "Range: 0 to 99" (Table 9-2, 106-24R1). `24` cannot tell 106-24
  from 106-24R1 (their Chapter 9 is identical, ADR-0016); "should" makes the
  two-digit form a recommendation, and "0 to 99" admits `4` for `04`.
- *Also found — the format is recent*: the year rule first appears in 106-17.
  106-05 and 106-07 say only "VERSION OF IRIG 106 STANDARD USED TO GENERATE
  THIS TMATS FILE"; 106-13 adds a maximum field size of 2. Files written to
  earlier editions may hold other forms.
- *RCCVER declares what the recorded data complies with, not the TMATS
  edition*: "Bits 7-0 specify which RCC release version applies and to which
  the following recorded data complies with" (Chapter 11, 106-24R1), whose
  newest code is still "0x0E = RCC 106-22". In 106-07 the same byte was
  CH10VER: "which IRIG-106 Chapter 10 release version the recorder
  requirements and following recorded data are applicable to and comply
  with", with only `0x07` defined.
- *Also found — older recordings declare nothing*: in 106-05 the setup
  record's channel-specific data word is "Reserved. (Bits 31-0)". A 106-04 or
  106-05 recording carries no version, so a reserved or zero value there is
  "not declared", not an error.
- *Therefore the two declarations answer different questions*, and differing
  labels are normal: `G\106` = `24` with RCCVER `0x0E` is consistent (0x0E
  means "106-22 or later"). L1-EDN-002 currently reports "any disagreement
  between sources", and ADR-0016 validates pre-2004 files "against the oldest
  known edition" while saying unknown editions are "not guessed".

Mapping: revises ADR-0016 (a new ADR partly superseding it), L1-EDN-002, and
UC-04; refines INT-012 and ARCHITECTURE section 6.5; touches `docs/CLI.md`
(what `tmats` shows); no conflict with T1–T5. Plan:

1. *Two declarations, kept apart and preserved*:
   - **TMATS edition declared** — `G\106`'s raw value and its reading: the
     candidate editions (`24` → 106-24 or 106-24R1), or "unrecognised" for a
     value that is not a year of an edition (kept verbatim, never guessed).
   - **Recording-format version declared** — per setup record, the CSDW's
     RCCVER/CH10VER raw value and its reading for the recording's era
     (`0x0E` → "106-22 or later"; reserved in 106-07's table; "not declared"
     where the field did not exist).
   Neither is rewritten from the other.
2. *No automatic conflict finding*: differing declarations are shown side by
   side and are not a finding by default — TMATS written to one edition may
   legitimately describe a recording that complies with another. A user who
   wants them flagged adds a rule (ADR-0006).
3. *The validation edition, selected and labelled*: the report states the
   edition whose rules were applied and **its basis** — caller override;
   declared by `G\106`; or an explicit **fallback** with its reason (for
   example "`G\106` missing: rules of 106-22 or later taken from RCCVER" or
   "baseline 106-24R1"). When `G\106` has two candidates with identical
   Chapter 9 rules, either may be named; when their rules differ, the choice
   is a reviewed interpretation.
4. *Compatibility checking is not validation*: for pre-2004, unrecognised,
   or future editions, the library does not claim to validate against the
   file's edition. It may run a **compatibility check** against a named
   edition (106-04, or the baseline) when the caller or the default policy
   asks, and every report calls it that. This reconciles ADR-0016's two
   statements.
5. *Register entries*: `G\106`'s two digits and the 106-24/106-24R1 pair;
   `G\106` forms before 106-17; RCCVER meaning and "not declared" before
   106-07 (refining INT-012).
6. *Tests*: `24` with `0x0E` gives no finding and names 106-24R1 as the
   rules applied, declared by `G\106`; `4` is read as 106-04 with a note;
   `G\106` missing gives a labelled fallback; `96` gives a compatibility
   check against 106-04 and never "validated against 106-96"; a 106-05
   recording's zero CSDW is "not declared"; `2024` is unrecognised and
   preserved.

Documents affected: a new ADR (partly superseding ADR-0016), ADR-0016
(status pointer), L1-EDN-002 revised and new L1 requirements for the
declarations and the labelled validation basis, `docs/USE-CASES.md`
(UC-04, UC-06), ARCHITECTURE (section 6.5 and a section for T6),
`docs/INTERPRETATIONS.md`, and `docs/CLI.md`.
**Owner decision needed:** the default when `G\106` is missing or
unrecognised and no override is given — (a) apply a labelled fallback
(RCCVER's edition, else the baseline) and continue, or (b) validate nothing
edition-specific and ask for an override. Recommendation: (a), because a
labelled fallback is still useful and says plainly what it is.
**Decided (owner, 2026-09-26): (a), a labelled fallback** — RCCVER's
edition (for `0x0E`, the newest it covers), else the baseline.
*Applied 2026-09-26: ADR-0028 (with a status pointer on ADR-0016),
`docs/ARCHITECTURE.md` section 9 and section 6.5, a new diagram
`docs/diagrams/edition-basis.svg`, L1-EDN-002 revised, L1-EDN-005 and 006,
`docs/USE-CASES.md` (UC-04, UC-06), INT-014 to INT-017 (and INT-012
cross-referenced), and `docs/CLI.md`.*

### Suspect findings to confirm against real data

Findings accepted into the design but held in doubt by the owner until they
are checked against real recordings (the local-only sample data of
`docs/TEST-DATA.md`). Each stays listed here until that check is done and
its result recorded in `docs/research/`.

| # | Finding | Owner's concern | Check against real data |
|---|---------|-----------------|-------------------------|
| S1 | Appendix 9-C erratum: 18 attributes ended with `:` instead of `;` (106-17 onward), and the "possible `;` typed as `:`" diagnostic built on it (L1-READ-007, ADR-0026). | "Lets still mark this as suspect so when we run against real data we dont forget I have some concerns." (2026-09-26) | Run the reader over every local sample; count and review each "possible `;` typed as `:`" diagnostic and each value containing `: ` followed by a code name; record whether real TMATS writers produce the pattern, whether any legitimate value triggers it (false positive), and whether the default severity (warning) is right. |

## Work for other repositories

Findings and decisions made here that require changes in sibling
repositories. This repository does not change them; each item is carried to
its repository as a tracked issue (link added when filed) and removed from
this list once that repository has it.

| # | Repository | Item | Source here |
|---|------------|------|-------------|
| X1 | `irig106-time` | Edition code `0x0F` is mapped to 106-23 (`src/version.rs`), but 106-24R1 Chapter 11 Figure 11-34 defines RCCVER `0x07`–`0x0E` (106-07 to 106-22) and reserves `0x0F`–`0xFF`; `0x0E` means "106-22 or later", and 106-20 has no code. The version table in its `docs/ROADMAP.md` also omits 106-20 and 106-24. | ROADMAP T3; ADR-0025 |
| X2 | `irig106-types` | Define `Irig106Version` (`#[non_exhaustive]`, with an unknown value, RCCVER mapping per Figure 11-34), the Chapter 10/11 data-type codes and data-type-version field (Table 11-4, §11.2.1.1 e), and the Computer-Generated Format 1 CSDW (bits 31–10 reserved, bit 9 FRMT, bit 8 SRCC, bits 7–0 RCCVER); then remove the duplicate enum from `irig106-time` (fixes X1). | ADR-0009; ADR-0025 |
| X3 | `irig106-core` | When its packet reader exists, it takes over slicing from the `tmats` CLI (header, optional secondary header, Data Length, filler and checksums, header-checksum verification) and supplies setup-record fragments with provenance to this library's assembler. | ADR-0019; ADR-0025 |
| X4 | `irig106-ch10-reader` | Report a mid-recording setup-record change by default, in one line naming what changed; assemble setup records that span several packets before reporting TMATS presence or size; replace the Windows guide's external extraction advice with `tmats extract`. | USE-CASES §5 and §7 (question 3); ADR-0025 |
| X5 | `irig106-studio` | Revise the TMATS contract in `docs/INTEGRATION.md`: the edition comes from `G\106` (not `R-1\ID`), `R-1\NSS` does not exist in Chapter 9, and channel data comes from UC-05 views (including `R-x\NSB` multiplexer source bits). | USE-CASES §5; L1-VIEW-005 |
| X6 | `irig106-decode` | Evaluate derived parameters from this library's description (expression tree or bound call, derivation graph, trigger and occurrences), and own engineering-unit conversion and floating-point formats (Appendix 9-D). | ADR-0024; NR-007 |
| X7 | `irig106-types`, `irig106-time` | Push the local P6-01 commits and publish `irig106-types` 0.1.0 so `irig106-time` can drop its path dependency (from the earlier ecosystem work). | Owner direction log |

## Planned releases

| Version | Theme | Scope |
|---------|-------|-------|
| 0.1 | Read | Lossless parser for code-name (ASCII) TMATS and the Chapter 10 setup-record payload, including the CSDW format bit (ASCII vs XML). Lookup by code name. V and X groups preserved and linked. Byte-faithful writer. Edition detection. `G\SHA` compute/verify and the flex signature. First crates.io release of both crates. **`tmats` CLI**: `show` in raw, tree, and channel-summary form (the three `idmptmat` formats), `extract`, `checksum` (`G\SHA`, `--flex`), `verify`, `stamp`; every setup record in a recording; plain text and JSON output. |
| 0.2 | Registry | Spec-derived registry for the baseline edition (106-24R1) covering every group in Chapter 9 (G, T, R, M, P, D, B, S, Q, C, H, V, X). Typed accessors generated from it. Channel resolution from R through P/B/S/Q to D and C; `tmats show` summary uses the resolved links. |
| 0.3 | Validate | Registry-driven validation: required/allowed-when rules, keywords, ranges, value types, counters and index consistency, cross-group references. User registries, severity policy, custom rules. `tmats validate`. |
| 0.4 | Editions | Registry deltas for every edition from 106-04 to 106-24R1 (including renamed codes such as `R-x\DST-n` → `R-x\CDT-n`, to be confirmed), a generated `VERSION-DELTAS.md`, and edition-aware validation. |
| 0.5 | Generate and edit | Edit API (set/insert/remove), validation diagnostics carrying suggested edits with explicit apply, and a builder whose output passes validation. `tmats diff` and applying chosen fixes. |

Testing strategy for every release: spec-sourced fixtures (starting with the
Chapter 9 Appendix 9-C example), property tests (never panics on arbitrary
bytes; the attribute multiset survives a round trip), regression tests for
every defect found in the reference tools, fuzzing, and — locally only —
the irig106.org sample recordings and real program files, compared
semantically against `idmptmat` (`docs/TEST-DATA.md`).

## Deferred features

Removed from the prototype or not yet started. Each is recorded so it is not
forgotten; none has a committed version.

- **TMATS XML.** Chapter 9 §9.4.3 defines an XML form of TMATS as an XSD
  schema set published by RCC; `rcc-106-standards` holds the schemas from
  106-07 onward. The prototype had an invented `<Tmats>` mapping that was not
  that schema and lost data on a round trip. To bring it back: generate the
  mapping from the official XSDs, handle the documented
  differences from the code-name form (one C group per data link, no `\N`
  counters, expanded keywords, XML date formats, semicolons allowed in text),
  and prove round trips against the code-name form. The Chapter 10 setup
  record already signals XML payloads through the CSDW format bit, so 0.1
  detects them and reports them as unsupported rather than misparsing them.
- **WebAssembly bindings.** The prototype exposed parse/validate/serialize to
  JavaScript through `wasm-bindgen` inside this crate, intended for
  `irig106-studio`'s in-browser TMATS view. To bring it back: a separate
  `irig106-tmats-wasm` crate (`crate-type = ["cdylib", "rlib"]`) so the core
  crate's dependency graph and semver are unaffected.
- **`no_std` support.** The prototype declared a `std` feature but used `std`
  unconditionally. To bring it back: a real `no_std` + `alloc` build with an
  output sink abstraction, verified in CI on a `no_std` target. Only worth it
  if an embedded consumer appears.
- **Rich diagnostics.** The prototype's `rich-errors` feature pulled in
  `miette` with its `fancy` renderer but used nothing from it. To bring it
  back: source-span diagnostics rendered by a *binary* (CLI or example), not
  by the library, so library users do not inherit a terminal renderer.
- **Serde support.** Serialize/deserialize for the document and diagnostics,
  once the new model's public shape is stable.
- **GUI.** An `igDisplayTMATS`-style graphical viewer (raw, summary, tree,
  checksum status) belongs in `irig106-studio`, built on the same library
  calls as `tmats`; nothing GUI-related enters this repository.
- **Data Display Markup Language and IHAL.** Chapter 9 §9.6–9.7 define two
  further XML schemas (data displays and instrumentation hardware). Out of
  scope unless a consumer needs them.
