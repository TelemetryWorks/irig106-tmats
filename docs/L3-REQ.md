# irig106-tmats — Level 3 Requirements

> **Status: foundations through stamp-transaction batches drafted (family 2, batches 1-11).**
> INT-042 Option A was approved by Joey, 2026-11-23, with independent AI recheck concurrence.
> INT-018/019/020 are reconfirmed; INT-043 Option B is approved (Joey, 2026-11-23; AI CONCURS at #32 head 8941ca1).
> These are implementation obligations, not existing interfaces or completed
> tests. INT-035/036/037/038/039/040/041/042 are approved; INT-042 sign-off is dated 2026-11-23.
> Remaining derivations and development evidence still precede implementation.

## Purpose

This document establishes the Level 3 (L3) SHALL-statement requirements. L3
requirements decompose each L2 into **implementation-level obligations**:
specific type and function names, exact values and thresholds, and behaviour
under edge conditions. L3s are the level at which requirements become
directly testable against code.

## Conventions

L3 identifiers follow the format `L3-<CATEGORY>-<NNN>` and use a compact
two-line format:

> **L3-XXX-NNN** · Parent: L2-XXX-NNN · Verification: T
> *Statement.*

A requirement verified other than by Test adds a trailing
`· Evidence: <artifact>` segment on the same line. Verification method
abbreviations: **T** = Test, **A** = Analysis, **I** = Inspection,
**D** = Demonstration. L3 categories mirror the L2 category they refine;
`LIB` and `CLI` are reserved for obligations that apply only to the library
crate or only to the `tmats` crate.

L3 entries are added only where there is genuine implementation detail that
cannot be inferred from the L2 statement.

**Status and verification artifacts** are tracked in
[`docs/TRACE-MATRIX.md`](TRACE-MATRIX.md). This file holds only spec content.

---

## Foundation batch: implementation obligations

These entries derive only the foundation L2s. Check names in TEST-DATA.md
are prospective; there are no new test markers or evidence claims here.
The current prototype's similarly purposed APIs are not accepted evidence.

### Workspace and CLI boundaries

**L3-REL-001** · Parent: L2-REL-001 · Verification: T
*The root Cargo.toml SHALL contain workspace configuration, not a `[package]`, with members `crates/irig106-tmats` and `apps/irig106-tmats-cli`; both packages SHALL inherit `workspace.package.version`, and the root `[workspace.dependencies]` entry for `irig106-tmats` SHALL use `path = "crates/irig106-tmats"` and the matching exact `=X.Y.Z` version, inherited by the CLI with `workspace = true`.*

The future manifest check reads actual TOML structures and inheritance,
including the CLI library and binary targets, rather than matching a version
substring. Fuzz remains outside the workspace; its dependency path, trace
source roots and CI/package paths must be reviewed during the restructure.
This is the owner-decided target (ADR-0035), not a claim that files moved.

**L3-CLI-001** · Parent: L2-CLI-001 · Verification: T
*The CLI library SHALL export `run(args, stdout, stderr) -> ExitClass`; `args` and both writable streams SHALL be supplied by the host, and returning an `ExitClass` SHALL leave the host running on successful and unsuccessful invocations.*

`ExitClass` names a semantic return type; numeric process-status mappings
are not selected by this batch. Argument representation and command-specific
forms follow with the argument-parser derivation, not an implicit argv read.

**L3-CLI-002** · Parent: L2-CLI-001 · Verification: T
*The `tmats` binary SHALL obtain process arguments and standard streams, call the same public `run` used by hosts, and map its result at the process boundary; it SHALL NOT duplicate command parsing, command execution or rendering.*

A future subprocess driver tests that control returns after a runner call;
calling a process-exiting implementation must fail that check. Binary versus
host behavior is compared for the same supplied arguments and streams.

**L3-CLI-003** · Parent: L2-CLI-002 · Verification: T
*Public command APIs SHALL return report values without directly printing, and public render APIs SHALL accept those reports plus a supplied `std::io::Write`; a host SHALL be able to use them without invoking `main` or the whole runner.*

Command-specific report types are defined with their later requirements.
The boundary tests exercise a real report/renderer when one is implemented;
empty stubs or help-only behavior cannot verify every show mode.

**L3-IO-001** · Parent: L2-IO-001 · Verification: I
*The TMATS library public input APIs SHALL take caller bytes rather than paths or process state, and output APIs SHALL return bytes or accept a supplied writer; the library SHALL NOT acquire files, network connections, stdin/stdout/stderr or environment configuration, or exit the process.*

The future inspection must name the reviewed APIs and reachable acquisition
paths. No Evidence line is provided until that inspection is performed;
passing a memory-writer test alone does not prove the absence of other I/O.

### Source ownership and raw access

**L3-READ-001** · Parent: L2-READ-001 · Verification: T
*`TmatsDocument` SHALL own its source bytes with no lifetime parameter tied to the caller's input and expose `source_bytes() -> &[u8]` without mutable source access; changing or dropping the caller's buffer after parsing SHALL NOT change the document's bytes.*

This name retains the reviewed root-document concept, not the prototype's
public group-field representation. Borrowed byte slices remain bounded by
the borrow of the document that owns them.

**L3-READ-002** · Parent: L2-READ-001 · Verification: T
*Raw code-name/value access SHALL return borrowed byte slices of the owned source; byte retention SHALL NOT depend on UTF-8 decoding or lossy string conversion, and requesting a decoded view SHALL NOT alter those bytes.*

The acceptance fixture includes zero, high-bit and invalid-UTF-8 bytes as
arbitrary source data. It is not asserted to be conforming TMATS; meaning
and diagnostics remain obligations of the scanner/view batches.

**L3-READ-003** · Parent: L2-READ-002 · Verification: T
*An item's raw span and available code-name/value spans SHALL refer into the owned source, and item enumeration SHALL preserve input order and multiplicity even when derived keys compare equal or have no interpretation.*

A malformed item may lack an interpreted key or a component span; source
retention must not require fabricating one. Scanner rules must later define
the actual item boundaries rather than treating this as a recovery rule.

**L3-WRT-001** · Parent: L2-WRT-001 · Verification: T
*The unedited default writer SHALL write the full `source_bytes()` sequence in order with no added or removed byte; it SHALL NOT insert a newline, rebuild delimiters, recompute counters, fold case or omit retained malformed material.*

The round-trip oracle is the independently supplied original byte vector,
not a second serialization of the same model or a subset of typed fields.
A fallible supplied writer must return its error instead of claiming success.

### Span safety

**L3-ROB-001** · Parent: L2-ROB-001 · Verification: T
*`Span` SHALL have private bounds and a checked constructor `Span::new(start, len, source_len) -> Option<Span>` that succeeds only if `start <= source_len` and `len <= source_len - start`, without an unchecked `start + len` calculation.*

Zero-length ranges at source end are valid; overflow-sized or out-of-bounds
ranges return `None`. The tests use integer-boundary values without trying
to allocate source buffers of those sizes.

**L3-ROB-002** · Parent: L2-ROB-001 · Verification: T
*Access through a `Span` SHALL validate against the actual supplied source slice and return a borrowed slice or a non-panicking rejection; it SHALL NOT trust a length supplied earlier as proof that the current source has that length.*

Check access to an empty source, exact-end ranges and a shorter source than
the constructor length. This does not define cross-document item identities
or edit revision rules, which need their own later requirements.


## Scanner batch 2: implementation obligations

The recovery entries below follow owner-approved INT-035. Acceptance cases
are specified in TEST-DATA.md; none is an implemented or traced test yet.

**L3-READ-004** · Parent: L2-READ-003 · Verification: T
*For each semicolon-terminated candidate region, the scanner SHALL search for the first colon only within that region, excluding the semicolon, and retain separate source ranges for the complete region, code-name bytes, value bytes and both delimiters when present.*

A colon in a later region must not make an earlier missing-colon region
appear well formed. Component spans identify raw bytes, not a claim that
the code name or its value is semantically valid.
A zero-length value span in `X:;` is present, not an absent item; `:v;`
requires an empty-code-name finding. These are framing distinctions, not
approval of `X` as a known attribute or of an empty value for any definition.

**L3-READ-005** · Parent: L2-READ-003 · Verification: T
*Bytes after the first colon and before the terminating semicolon SHALL remain the raw value, including later colons, spaces and line endings; CR/LF SHALL NOT terminate an attribute, and neither code-like text nor quote characters SHALL create an alternate terminator or an implicit attribute split.*

A future suspected-semicolon finding under L1-READ-007 may suggest an edit;
it must not change framing or apply it. This does not add quoted strings or
an escape syntax to TMATS delimiters.

**L3-READ-006** · Parent: L2-READ-004 · Verification: T
*When a nonempty candidate region not classified as permitted layout has a semicolon but no colon, the reader SHALL retain that region as uninterpreted source material with a diagnostic spanning it and resume at the byte after that semicolon; a nonempty final region not classified as permitted layout and lacking a semicolon SHALL be retained with a missing-terminator finding and SHALL NOT be fabricated into a terminated attribute or exposed as an explicit value by lookups.*

This follows owner-approved INT-035 (Joey, 2026-11-21). Permitted inter-attribute layout must be classified
by its own character rules before claiming a layout-only suffix is malformed.
L3-READ-009/010 specify the approved empty-region and EOF-silence policy
(INT-035). Full malformed code-name diagnostics remain a prerequisite,
including rejection of a missing separator masked by a later colon.

**L3-READ-007** · Parent: L2-READ-004 · Verification: T
*The framing result SHALL carry both retained source material and framing diagnostics with byte ranges in that source; one malformed region SHALL NOT erase earlier or later retained material or replace its findings with empty success.*

For assembled setup records, later provenance requirements map these ranges
to packets. This entry does not declare an incomplete record usable.

**L3-READ-008** · Parent: L2-READ-005 · Verification: T
*ASCII comparison SHALL map byte values `0x61..=0x7A` to their corresponding `0x41..=0x5A` values for comparison only and leave every other byte unchanged; the comparator itself SHALL NOT trim, decode/re-encode or alter the source as a side effect.*

Test mixed case, punctuation and high bytes as comparator inputs. Equality
of raw comparator inputs does not validate their attribute semantics. Before
comparison, code-name interpretation must ignore surrounding blanks per
L1-READ-006 while retaining the full raw span; this is separate from the
byte-comparator operation. The character batch must handle CR/LF and other
nonprintables without conflating them with ordinary value blanks.

**L3-ROB-003** · Parent: L2-ROB-002 · Verification: T
*The framing cursor SHALL remain within the source bounds, consume each discovered semicolon before the next region, and terminate at EOF; on an input of length N it SHALL finish after at most N+1 region steps, including empty-region handling, without allocating an N-sized fixture merely to test integer limits.*

The bound concerns region advancement, not an unapproved performance budget
or a claim about all subsequent parsing/validation work. Empty input and
runs consisting solely of semicolons must terminate without repeated state.


**L3-READ-009** · Parent: L2-READ-006 · Verification: T
*For a maximal adjacent run of empty or layout-only terminated regions, the reader SHALL emit exactly one information-level finding whose half-open span begins at the first region's start and ends immediately after the last region's semicolon; a nonempty attribute or non-layout malformed region SHALL end the run.*

`;;;;` yields one finding over `[0, 4)`. Mixed empty/layout-only terminated
regions can share that run; a present zero-length value or an empty code
name is not such a region. Original bytes and region advancement remain
unchanged. Character-batch rules determine which bytes count as layout.

**L3-READ-010** · Parent: L2-READ-006 · Verification: T
*An EOF suffix classified as unterminated layout or padding SHALL produce no finding, SHALL remain retained, and SHALL NOT extend the span of a preceding terminated empty-region run or become an explicit lookup value.*

Silence applies to the layout/padding suffix only, not to an unterminated
attribute containing other material. No tests or implementation are supplied
by these requirements; use the acceptance cases in TEST-DATA.md.


## Character batch 3: implementation obligations (interpretations approved)

INT-036/037 were approved by Joey, 2026-11-22, under ADR-0033; the
independent AI recheck concurred on PR #16 head `891f77f` (see the
[sign-off record](research/2026-11-22-int036-int037-owner-signoff.md)).
Existing INT-035 run findings and EOF silence are unchanged. This batch
specifies which bytes classify as layout/nonprintables; implementation,
full development analysis and executable tests remain outstanding.

**L3-READ-011** · Parent: L2-READ-007 · Verification: T
*For interpretation only, the reader SHALL omit bytes `0x00..=0x1F` and `0x7F` from a field projection while retaining all original bytes; it SHALL NOT reinterpret an omitted byte as a space, delimiter or missing value, and CR/LF SHALL NOT terminate attributes.*

This is the approved INT-036 mapping, not an implemented filter. High bytes
are outside this omission set. Recognized leading encoding-marker handling
under INT-037 precedes semantic projection, so BOM-marked input is not converted by
merely discarding its zero bytes.

**L3-READ-012** · Parent: L2-READ-007 · Verification: T
*Raw field access and default emission SHALL remain byte-exact, human-display renderers SHALL use raw text with explicitly labelled display decoding/escaping rather than filtered semantic text, and locations derived from projections SHALL refer to the original source, including skipped-byte gaps, rather than use shortened projection offsets as if they were source offsets.*

A projection may be non-contiguous in the source. Preserve its raw enclosing
span and enough correspondence to locate contributing bytes; do not fabricate
a contiguous raw value from an interpreted string.

**L3-READ-013** · Parent: L2-READ-008 · Verification: T
*After the approved nonprintable omission, code-name interpretation SHALL trim only leading/trailing `0x20` bytes, retain internal `0x20` bytes for code-name validation, and leave value `0x20` bytes in place; the L3-READ-008 comparator SHALL continue folding ASCII case without trimming its own input.*

An empty interpreted name still receives a finding, not a fabricated key.
TAB and other controls in COMMENT/description values are omitted, so `a\tb`
becomes `ab`, never `a b`; raw display follows L3-READ-012.
A value projected to zero bytes is present, not absent; whether that empty
value satisfies a particular attribute definition is separate validation.

**L3-READ-014** · Parent: L2-READ-006 · Verification: T
*Layout/padding-only classification for INT-035 SHALL accept only `0x20`, `0x00..=0x1F` and `0x7F`; a maximal terminated run so classified SHALL use the approved one-information-finding rule, and an unterminated EOF suffix so classified SHALL remain silent and retained.*

A colon, high byte or other printable byte prevents layout-only classification.
This must not turn a genuinely unterminated attribute into silent padding.
These regions are not fields, so NUL padding does not add a field finding to
the approved run information finding or silent EOF suffix (approved INT-036).

**L3-READ-015** · Parent: L2-READ-009 · Verification: T
*At assembled TMATS text offset zero, the reader SHALL recognize leading `EF BB BF`, `FE FF`, `FF FE` or `00 00 FE FF` byte signatures as unsupported encoding markers, retain the entire source and a located marker finding, and withhold document semantic lookup instead of stripping the prefix or transcoding the body.*

UTF-32LE `FF FE 00 00` is caught by its `FF FE` prefix. Under ADR-0025,
text begins after each Format 1 CSDW is removed and fragments are assembled;
recognition must work across fragment boundaries, not separately per packet.
For standalone text, zero is the first supplied byte. Inspect original
assembled bytes before control/padding omission: a marker after CR/LF or NUL
padding is midstream, not a leading marker.

These byte patterns follow Unicode's BOM table, cited in INT-037; no claim
about Chapter 9's own definition of BOM is made. Recognition is prefix-based,
not proof of a complete Unicode encoding, and never sets a recorder edition.
No inference from alternating zeros is permitted for unmarked input.

**L3-READ-016** · Parent: L2-READ-009 · Verification: T
*Outside the recognized leading marker, bytes `0x80..=0xFF` SHALL remain raw with located non-ASCII findings, not be omitted or decoded as UTF-8/Latin-1 automatically; an affected field SHALL be Invalid with no usable interpreted value while unaffected raw material and findings remain available.*

A Latin-1 `Temp °C` field containing `B0` is Invalid by this rule.
ADR-0003's explicitly labelled on-request `as_latin1` and lossy
(`as_str_lossy`) display views remain available without semantic acceptance.
This approved INT-037 behavior does not prove that every unmarked Unicode
stream can be detected; NUL fields receive L3-READ-018 findings. Explicit
caller-side conversion is outside this byte parser and must not be represented as original recorded bytes.


**L3-READ-017** · Parent: L2-READ-008 · Verification: T
*When omitted controls lie between printable bytes within an interpreted code name, the reader SHALL emit a located information finding identifying the omitted source bytes; leading/trailing name layout SHALL stay silent except for the NUL-field rule.*

`CO\r\nMMENT` projects to COMMENT with a finding on the internal CR/LF.
Retain original source correspondence. This approved INT-036 finding does
not itself invalidate a name or replace subsequent code-name validation.

**L3-READ-018** · Parent: L2-READ-008 · Verification: T
*Each code-name or value field containing one or more NUL bytes SHALL emit exactly one located information finding for that field, identifying all its NUL locations, including at field edges or in a value whose projection is empty.*

The internal-name omission finding applies independently when its condition
also holds. This approved INT-036 rule exposes zero-interleaved inputs
without guessing their encoding or asserting UTF-16; it is not a NUL-padding
finding for the non-field regions governed by L3-READ-014.


## Code-name batch 4: components, recognition and failures

These are planned obligations, not implemented APIs. INT-038 was approved
by Joey, 2026-11-22, under ADR-0033; the independent AI recheck concurred
on PR #20 head `29707da` (see the
[sign-off record](research/2026-11-22-int038-owner-signoff.md)). Full analysis
and executable tests remain prerequisites. Existing character projection,
raw display, marker gating and recovery rules apply.

**L3-READ-019** · Parent: L2-READ-011 · Verification: T
*Standalone COMMENT SHALL be recognized by whole-name ASCII-insensitive comparison after approved character projection, without fabricating a group occurrence or path; a name with additional suffix/path material SHALL NOT be accepted as COMMENT by prefix matching.*

Source: 106-24R1 section 9.4.2, page 9-3. This recognizes the name, not a
new comment terminator or permission to embed semicolons in values.

**L3-READ-020** · Parent: L2-READ-011 · Verification: T
*Group recognition SHALL distinguish G, T, R, M, P, D, B, S, Q, C, H, V and X using ASCII case folding and preserve the supplied spelling; an unrecognized group SHALL remain retained and SHALL NOT be mapped to another group or discarded.*

Source: 106-24R1 Table 9-1, page 9-5. Recognition of a group is not
recognition of every attribute within it, nor approval of a name's shape.

**L3-READ-021** · Parent: L2-READ-010 · Verification: T
*For a cited code-name pattern, structural interpretation SHALL consume the whole interpreted name, distinguish literal code-element characters from hyphen-delimited index positions, and keep occurrence-index absence distinct from any supplied index rather than substitute a default occurrence.*

`G\106` contains a numeric literal code element, not an index; `R-1\TK1-3`
has literal TK1 and a separate index 3. Sources: Table 9-2 RCC IRIG 106
REVISION LEVEL, page 9-10; Table 9-4 TRACK NUMBER/CHANNEL ID, page 9-48.
Pattern references are definitions, not permission to insert their omitted
indices into file names. This requirement does not define the full grammar
of uncatalogued vendor/hardware/extension names.

**L3-READ-022** · Parent: L2-READ-010 · Verification: T
*Every parsed index SHALL retain its position and segment attachment, with original-byte spans/correspondence; path boundaries, index order and multiplicity SHALL NOT be flattened, reordered or truncated when constructing a structured key.*

For `D-1\MN-1-2`, keep occurrence 1, path MN and that segment's indices
[1, 2]; distinguish [2, 1] and [1, 2, 3]. For `D-1\MNF\N-2-3-4`, keep
segments MNF then N, with [2, 3, 4] on N. Sources: Table 9-7 MEASUREMENT
NAME, page 9-115, and NUMBER OF FRAGMENTS, page 9-117. These describe
name structure, not measurement links or valid counter contents.

**L3-READ-023** · Parent: L2-READ-010 · Verification: T
*At a standard pattern's index position, the approved INT-038 reader SHALL accept a nonempty ASCII decimal-digit token, preserve its complete raw spelling, and use its canonical decimal string for identity equality and hashing without fixed-width or big-integer conversion; leading zeros SHALL NOT create a different index identity, and checked numeric conversion SHALL NOT wrap or truncate.*

Canonicalization strips leading zeros and retains one `0` for an all-zero
token. Use the INT-036 projection before reading digits and retain original
source correspondence; omitted controls inside index digits still receive
the approved internal-name finding. Full-key equality/hashing must preserve
component and index boundaries (L3-READ-022). An all-zero token has identity
zero; parsing it does not make it a legal entry index. Signs, decimal points, exponents and template letters do not
match this approved index grammar. This rule concerns index tokens only,
not numeric data values. Range/contiguity checks need separately cited,
scoped rules; no inferred upper bound or allocation of every intervening
index is permitted just because a large index was supplied.

**L3-READ-024** · Parent: L2-READ-012 · Verification: T
*A demonstrated code-name syntax failure SHALL retain the item and a located finding without exposing a successfully parsed prefix as the item's usable key; the reader SHALL continue at the existing INT-035 boundary rather than repair punctuation or consume another region.*

Internal ordinary blanks are not removed to manufacture a valid standard
name. `G\COM Time 12:30;` remains raw-framed at its colon, with a name
finding; it does not become G\COM. Empty components or unfinished index
positions in a supplied standard pattern likewise cannot become a partial
match. INT-036 internal-control/NUL findings remain independently visible;
INT-037 failed character interpretation must not be bypassed by key parsing.

**L3-READ-025** · Parent: L2-READ-011 · Verification: T
*An unmatched definition SHALL preserve available structural information and raw-name access without being labelled malformed solely for being unknown; H/V/X or other unfamiliar names that cannot be unambiguously decomposed SHALL retain opaque source material rather than be forced into a standard-pattern key.*

Sources: 106-24R1 sections 9.5.12/13, page 9-171, and 9.5.14, page 9-172;
ADR-0002. This is a retention/classification boundary. Dedicated extension
interpretation, association and registry overrides remain later work. No
conformance claim is made for an arbitrary unknown-name example.

**L3-READ-026** · Parent: L2-READ-012 · Verification: T
*A structurally read name SHALL NOT by itself establish known-definition validity, entry range/contiguity, value validity, uniqueness or link resolution; equal interpreted keys SHALL retain every original item instead of selecting or merging an occurrence during parsing.*

Approved INT-038 makes zero-padded decimal spellings equal, so both originals
must remain available for later duplicate/ambiguity handling; the narrower
normalization-only collision finding is required now by L3-READ-029. Counters and
parent scopes follow ADR-0026 and Chapter 9 section 9.5.1 a; X has the cited
noncontiguity exception in section 9.5.14. No association or counter default
is invented by this batch, and no typed getter is declared implemented.
`\N` count/no-missing-values checks over DISTINCT numeric identities are
derived in counter batch 6 below, including parent scope and the X
noncontiguity exception; neither occurrences nor raw spellings are counts.
Those requirements do not supply completed rule interpretations or code.


**L3-READ-027** · Parent: L2-READ-012 · Verification: T
*Each projected decimal index token whose spelling differs from its canonical string SHALL produce a located information finding on its original token span, without changing retained bytes; canonical `0` SHALL be distinguished from noncanonical multi-digit zero spellings such as `00`.*

This approved INT-038 visibility also covers nonzero leading-zero tokens
such as `01`. Internal/adjacent ordinary blanks within the name do not become
valid digit tokens by trimming; whole-name edge layout still follows INT-036.

**L3-READ-028** · Parent: L2-READ-010 · Verification: T
*For section 9.5.14 X-to-original index comparisons, the occurrence and carried indices for the embedded original path SHALL use the same projected canonical decimal-string identity as the original; extra extension indices SHALL remain separate and matching indices SHALL NOT alone establish a resolved association.*

Source: 106-24R1 Chapter 9 section 9.5.14, page 9-172. Approved INT-038
settles the spelling comparison: 1 and 01 agree, but all raw spellings remain.
This does not collapse X and its original into the same full code-name key.

**L3-READ-029** · Parent: L2-READ-012 · Verification: T
*When a canonical full-key group contains two or more DISTINCT raw index-spelling tuples, the reader SHALL report a located information finding listing EVERY original occurrence in that group, exact repeats included, retaining their input order, raw bytes and values without selecting, consolidating or claiming positive uniqueness.*

This approved INT-038 collision finding is in addition to per-token
noncanonical-spelling findings. Group complete keys after approved
case/control projection and canonicalization; determine distinct tuples from
the original-source spelling of each corresponding index token. One raw tuple
alone does not trigger this finding, even if repeated. Code-letter case or
value differences alone do not create another index-spelling tuple. Equal
index numbers in different paths/groups are not full-key collisions. This
clarification was requested with Joey's 2026-11-22 sign-off. General duplicate
single-entry/value-conflict classification remains for the later batch; it does not defer this finding.


## Candidate batch 5: scalar duplicates and exact-name results

These are planned obligations, not implemented interfaces or executable
verification. INT-039 was approved by Joey, 2026-11-22, under ADR-0033,
with independent AI concurrence on PR #22 head `b22d45c` (see the
[sign-off record](research/2026-11-22-int039-owner-signoff.md)). Full analysis
and executable tests remain outstanding. One Format 1 setup record spanning
several packets is ONE document after ADR-0025 assembly, not one document per
packet. Separate successive records are not merged; this batch does not
decide record completeness or governance.

**L3-READ-030** · Parent: L2-READ-013 · Verification: T
*Duplicate grouping SHALL use a complete canonical key including group, supplied occurrence, ordered path and every index position, within one document; it SHALL retain each matching item identity and source location once per occurrence, including byte-identical repeats.*

Approved INT-036/038 supply projection and decimal-string identity. Do not
group by value, path prefix, entry index alone or across distinct documents.
Source: 106-24R1 section 9.4.2, page 9-2; ADR-0002/0003. After ADR-0025
assembly, G\PN in fragment 1 and again in fragment 2 of one Format 1 record
is a duplicate in one document; preserve both locations and their fragment
provenance. A packet boundary does not reset the duplicate group.

**L3-READ-031** · Parent: L2-READ-013 · Verification: T
*For a slot established as scalar by a cited reviewed definition, two or more occurrences of its complete canonical key SHALL produce a located duplicate finding referencing every occurrence, irrespective of whether raw or interpreted values agree; distinct index combinations in a multiple-entry pattern SHALL remain separate slots.*

This is approved INT-039. `G\PN` is PROGRAM NAME (106-24R1 Table 9-2,
page 9-10); `G\DSI-1` and `G\DSI-2` illustrate distinct numbered entries
in section 9.4.2, page 9-2. Repeating the same numbered slot is different
from supplying several different numbered entries. Severity/profile mapping
is deferred; lowering severity cannot manufacture a unique value.

**L3-READ-032** · Parent: L2-READ-013 · Verification: T
*Unknown or unreviewed cardinality SHALL remain explicitly unclassified for scalar-duplicate conformance, while raw multiplicity remains visible; standalone COMMENT items SHALL be exposed as a retained ordered collection rather than diagnosed as duplicate scalar attributes merely for repeating.*

The COMMENT collection is an approved INT-039 reading of section 9.4.2,
page 9-3; it does not extend to group `\COM` fields. The owner confirmed
that repeated `G\COM`, `R-x\COM`, etc. are single-entry duplicates per the
standard, with Ambiguous singular values and every occurrence in the finding.
See Table 9-2 COMMENTS, page 9-13, and Table 9-4 COMMENTS, page 9-84.
Some writers split long remarks across repeated fields; that practice does
not authorize joining them or treating them as standalone COMMENT. Raw
lookup still returns every line. This also does not exempt G\PN.
No cardinality is inferred solely from absent indices or unfamiliar private
H/V/X names. This does not suppress INT-038's applicable collision findings.

**L3-VIEW-001** · Parent: L2-VIEW-001 · Verification: T
*An exact lookup using a complete interpreted key SHALL return all matching items in input order with original item IDs, name/value spans and byte access; an index or typed cache SHALL NOT replace this result with a deduplicated or last/first-wins map.*

Case and index spelling use the approved comparator/projection rules. A
value's high bytes, invalid type or empty content do not erase its raw
candidate when the name has a usable key. A malformed/unsupported name with
no usable interpreted key remains accessible through the source/item store;
it is not fabricated into an exact-key match. If INT-037 marker gating blocks
interpreted lookup, report that block and its finding rather than return
empty success or claim the key is absent; raw source access remains available.

**L3-VIEW-002** · Parent: L2-VIEW-001 · Verification: T
*On a document with interpreted lookup available, absence of an exact-key match SHALL be represented as an empty candidate collection, distinct from one matching item with a zero-length raw value; returning candidates SHALL NOT itself label their values explicit, defaulted or valid.*

`G\PN:;` has one present raw candidate; whether its value satisfies the
cited definition is separate. Empty candidate collection is a raw lookup
outcome, not authority to invent a default or prove a link unresolved.

**L3-VIEW-003** · Parent: L2-VIEW-002 · Verification: T
*For a definition-bound scalar slot, singular resolution with two or more matching occurrences SHALL be ambiguous and list every original candidate/location, even if all raw bytes agree, interpreted values compare equal, or only one candidate would independently be valid.*

This approved INT-039 multiplicity gate precedes value-based candidate
selection. Preserve each candidate's invalid findings; ambiguity must not
hide the bad input or present one candidate as the chosen explicit value.
Batch 7 below derives the zero/one-candidate core; complete condition/type
interpretations and executable verification are still outstanding.

**L3-VIEW-004** · Parent: L2-VIEW-002 · Verification: T
*Ambiguous scalar resolution SHALL NOT substitute a default, infer an absent value, coalesce equal candidates or become explicit after reordering input or changing diagnostic severity; raw output SHALL continue to reproduce every retained byte.*

Approved INT-039 refines ADR-0023's ambiguous state for repeated slots.
Order changes reporting order only. This does not define link selectors,
packet evidence, caller assumptions or a data-channel/time-source association.

**L3-READ-033** · Parent: L2-READ-013 · Verification: T
*General scalar-duplicate findings SHALL coexist with the approved per-token normalization and canonical-group collision information findings, and SHALL NOT turn duplicate G\SHA integrity into a positive verdict or permit stamping.*

INT-038 already requires collision information for distinct raw index-spelling
tuples and lists exact repeats too. Scalar duplicates can exist even with
only one raw spelling tuple. Owner-approved F1 (Joey, 2026-11-23; INT-021,
originally ADR-0034; L1-SUM-002) separately requires indeterminate integrity and stamping
refusal. Checksum mechanics and edits remain later requirements.


## Counter batch 6: member sets and evaluation evidence

Requirements below remain unimplemented. INT-040 was approved by Joey,
2026-11-22, under ADR-0033; the independent AI recheck concurred on PR #24
head `176fdf4` (see the [sign-off record](research/2026-11-23-int040-owner-signoff.md)).
INT-038/039 remain approved; full analysis and executable tests remain gates.
These obligations do not complete registry generation, link metadata, the
four-pass validator or the full five-state effective-value resolver.

**L3-REG-001** · Parent: L2-REG-001 · Verification: T
*Counter metadata SHALL bind a cited counter pattern to its member patterns and governed index position, holding the document/group occurrence and every declared parent index fixed; different fixed parent tuples SHALL produce independent member sets and evaluations.*

For Table 9-7 NUMBER OF MEASUREMENT LOCATIONS, page 9-117,
`D-x\MML\N-y-n` governs location m in `D-x\MNF\N-y-n-m` within (x,y,n).
For NUMBER OF FRAGMENTS on the same page, that latter counter governs
fragment e in `D-x\WP-y-n-m-e` within (x,y,n,m), as declared by the reviewed
rule. It does not count all D fields or every occurrence of a child property.

**L3-REG-002** · Parent: L2-REG-001 · Verification: T
*Counter declarations SHALL identify valid pattern/index positions and their parent bindings, carry the source for any contiguity exception, and remain unreviewed/unavailable rather than invent a binding when the source or scope is unresolved.*

ADR-0026 and 106-24R1 section 9.5.14, page 9-172, permit noncontiguous X
occurrence x matching original objects. Do not create an X counter, apply
1-to-N to those occurrences, or exempt all extension-only indices by analogy.
An unknown/private pattern is not counter membership merely by sharing a prefix.
An X item never becomes a D member by embedding D's path: section 9.5.14
attaches to an original and does not create it. Missing originals are separate link
problems; extension-only indices remain unclassified without their own rule.

**L3-VAL-001** · Parent: L2-VAL-001 · Verification: T
*Within each declared scope, governed entries SHALL contribute a set of canonical numeric index identities, retaining all supporting items/locations; repeated spellings, exact repeats and different member properties for the same governed identity SHALL contribute that identity only once.*

Approved INT-038 supplies string identity: 1 and 01 are one index. Approved
INT-040 counts an unambiguously identified entry even when its value is
Invalid or its scalar slot is Ambiguous; those findings remain independent.
An index zero remains observed evidence, not a silently discarded or valid
positive entry. Membership must not be guessed from an unusable name/scope.
Canonical parent identities 0 and 00 are the same scope. Preserve/report parent-domain errors,
but evaluate a child's own usable count from its explicit parent indices even
if m=0, m exceeds MML\N, or the ancestor MML\N is unusable; do not infer the
ancestor bound or require the parent to be valid before inspecting the child.

**L3-VAL-002** · Parent: L2-VAL-001 · Verification: T
*For an applicable contiguous 1-to-N rule with a usable counter N and complete membership evidence, validation SHALL independently check distinct-set cardinality, positive indices bounded by N and absence of gaps; cardinality equality or the greatest index equalling N SHALL NOT alone establish success.*

A set {1,3} against N=2 has the right size but violates the domain; a set
{1,3} against N=3 has the right maximum but lacks 2. Zero/excess indices
stay located evidence. If a reviewed rule permits N=0, only the empty set
satisfies that rule's 1-to-N domain; this does not invent a zero default or
make zero legal for counters whose cited range excludes it.

**L3-VAL-003** · Parent: L2-VAL-001 · Verification: T
*Counter/index comparison and gap reporting SHALL operate on retained canonical identities without wrapping, truncating or allocating/enumerating up to a supplied index value; missing intervals SHALL be represented compactly with explicit inclusive index bounds rather than expanded into an unbounded item list.*

Canonical nonnegative decimal strings can be ordered by digit length then
lexicographically at equal length; preserve component boundaries and INT-038
identity/hash semantics. Work/storage must follow supplied members and compact
findings, not a 40-digit index's magnitude. No public ordering API or new
resource-budget number is selected here; detailed budget policy remains later.

**L3-VAL-004** · Parent: L2-VAL-002 · Verification: T
*Missing, Invalid or Ambiguous counter operands, unresolved applicability, unreviewed membership rules or required members whose governed identity/scope cannot be established SHALL produce cannot-evaluate with the blocking basis, never a guessed N, fabricated empty member set or positive result from partial evidence; every scope compatible with unassigned evidence SHALL report partial/cannot-evaluate, NEVER pass.*

A number that parses but violates its own counter range is Invalid and
blocks that counter's evaluation: Table 9-7 page 9-117 gives MNF\N range
1–8 and MML\N range 1–9999, so 9 and 0 respectively are not usable N.
This does not block a child's independent count merely because an ancestor
is unusable; the child's own rule/applicability and count must still be known.
Count-value parsing and allowed defaults require separate reviewed rules;
INT-038's index grammar is not a count-value parser. All duplicate counter
candidates remain visible under INT-039. An unrelated malformed field must
not be assigned to a counter scope by guess. Partial observed evidence can
be reported as such without claiming a complete membership check.

Diagnostic membership is exact, not fuzzy: use a separately framed projected
name and exact literal group/path elements, allowing invalid or empty numeric
tokens only as diagnostic slots. If literals do not match or no separate name
is framed, the item is unrelated to this rule's membership/blocking; keep its
other findings. Matching literals with wrong index arity are related but
unassigned rule-level evidence: report cannot-evaluate coverage, never guess
which tokens are missing or extra. Do not search inside values, strip internal
blanks, repair separators or unwrap X paths.
With the declared arity, readable matching parent tokens plus an unreadable
(or empty) governed token block that scope with cannot-evaluate. If a parent
token is unreadable, retain unassigned rule-level evidence and cannot-evaluate
coverage, without guessing a scope or claiming complete rule coverage.
A scope is compatible if none of the evidence's readable canonical parent
facts contradicts that scope at an unambiguously mapped position. Unknown
positions impose no restriction; wrong arity must not be guessed into an
alignment to exclude a scope. Thus `D-1\WP-1-1-x-2` blocks pass for every
scope (1,1,1,m), even if a scope's observed set already equals its usable N.
Keep those observations as partial evidence, not successful evaluation.
Apply this guard to evaluated scopes from retained rule-level evidence;
never enumerate unknown parent values. This is the tightening requested with
owner sign-off, recorded 2026-11-23.
Readable zero/excess parents remain explicit scopes, not unreadable ones.
This diagnostic classification never creates a usable lookup key or an index.
Exact source witnesses must identify each decision, including unrelatedness.

**L3-VAL-005** · Parent: L2-VAL-002 · Verification: T
*Each counter mismatch, index-domain/gap finding and cannot-evaluate result SHALL identify its counters-and-indices pass, reviewed rule/citation, fixed parent tuple, counter provenance and available member evidence, with original-source locations retained.*

No single synthetic span may pretend that disjoint items were adjacent in
source. Findings for defaults must cite their actual reviewed source, not a
fabricated location. A structurally correct count does not clear entry-value,
duplicate, presence, link or assembly findings or certify document conformance.
D-x\WFP-y-n-m-e FRAGMENT POSITION value uniqueness (Table 9-7, page 9-118)
is a separate VALUE rule, outside INT-040; distinct e identities do not prove
those values are unique. This batch neither implements nor waives that rule.

**L3-VAL-006** · Parent: L2-VAL-001 · Verification: T
*Counter validation SHALL leave all original bytes, item order, index spellings, declared counts and existing findings unchanged; it SHALL NOT recompute/insert counters, fill missing entries or renumber members as a validation side effect.*

ADR-0002 and L1-WRT-002 separate explicit normalization from validation.
Approved INT-038/039 findings remain visible even when distinct cardinality
matches N. Repair/edit APIs and their effect on counters are later work.


## Effective-value batch 7: supported-field states and unavailable reads

These are planned obligations, not implemented APIs or a complete registry.
Existing character, ambiguity and uncertainty rules stay approved unchanged.
INT-041 was approved by Joey, 2026-11-23, under ADR-0033, with independent
AI concurrence on PR #26 head `0fc0bc8` (see the
[sign-off record](research/2026-11-23-int041-owner-signoff.md)). Full analysis
and executable tests remain outstanding; no sixth field-value state or
concrete Rust result representation is selected by this batch.

**L3-REG-003** · Parent: L2-REG-002 · Verification: T
*Each effective-field/default rule SHALL bind its original source text, reviewed interpretation and exact edition/section/table/row citation, including whether a default comes from the Default field or definition prose and the occurrence scope/applicability on which it depends.*

ADR-0022 supplies the two-layer contract. The existing source-review/freshness
gates remain prerequisites; merely storing a string called default is not
review evidence. Full registry extraction and edition deltas remain later.

**L3-REG-004** · Parent: L2-REG-002 · Verification: T
*The resolver SHALL distinguish a reviewed rule stating that no default exists from a missing/unreviewed rule, and SHALL NOT borrow a baseline default for an unsupported edition or apply another occurrence's condition without a declared scope.*

Source: 106-24R1 section 9.5.1 a, page 9-6; ADR-0022/0028. A supplied
identified fallback basis stays labelled fallback, never a forged declaration.
This batch consumes edition selection; it does not choose its algorithm.

**L3-VIEW-005** · Parent: L2-VIEW-003 · Verification: T
*Effective-field results SHALL retain the document/revision, complete field scope and supplied interpretation basis or its unavailability reason; known scalar multiplicity SHALL preserve INT-039 Ambiguous before attempting to choose among candidates by their value validity or default availability.*

Every candidate remains available with its own findings. A known Ambiguous
result does not need a successful value conversion to prove multiplicity.
No scopes, successive records or interpretations from different editions
are silently combined to obtain a convenient answer. An Invalid result under
a fallback edition must carry that fallback label, applied edition and basis
reason; invalidity does not erase the uncertainty of the chosen rule basis.

For future blank-rule review, 106-24R1 Chapter 9 section 9.4.2, page 9-2, states:

> Leading, trailing, and embedded blanks are assumed to be intentional; they can be ignored in most cases but should not be used in code names, keywords, and data items used as links, such as measurement name.

The full sentence is source evidence, not approval of a new value-blank rule.
Keep raw bytes and the unavailable-rule guard below; name-edge blank handling
remains the separate approved INT-036 rule.

**L3-VIEW-006** · Parent: L2-VIEW-003 · Verification: T
*One actual candidate whose reviewed local character/type/range interpretation succeeds SHALL yield Explicit with the interpreted value, original raw value and source location/correspondence, without silently changing its spelling or representing that value as defaulted.*

Explicit means a usable given value under those local rules, not that every
Allowed/Required condition, cross-field rule, packet or document is valid.
Independent findings remain visible. Conditional constraints needed to decide
local validity must themselves be evaluable; otherwise INT-041 applies.
For TFMT, lowercase `a` is Explicit A with raw `a` preserved under section
9.4.2, page 9-2. Present TFMT on a known non-TIMEIN channel is Explicit
locally with an applicability finding; do not ignore it as an absent field.

**L3-VIEW-007** · Parent: L2-VIEW-003 · Verification: T
*One candidate with a demonstrated character/type/range failure SHALL yield Invalid with original raw evidence and the reason; zero-length or control-only projected content SHALL remain a present candidate and SHALL NOT trigger an absence default.*

Whether an empty projected value is valid depends on the reviewed definition;
this is not a blanket empty-text rejection. INT-037 high bytes stay Invalid.
A legitimate zero or false-like value is not absence. INT-036 omission/NUL
information is not itself an automatic invalidity verdict for every field.
Section 9.4.2, page 9-2, treats value blanks as intentional: `R-1\TFMT-3: A;`
is unavailable until a blank-handling rule is reviewed, NOT Invalid just
because an unreviewed exact enum comparison rejects the leading blank.
Do not silently trim the value or treat it as absent.

**L3-VIEW-008** · Parent: L2-VIEW-004 · Verification: T
*With no actual candidate, a verified default whose reviewed applicability is established under an identified edition basis SHALL yield Defaulted with the interpreted value, exact source citation, Default-field/prose origin and applicability evidence, without an original input value span.*

For baseline TIMEIN, 106-24R1 Table 9-4 TIME FORMAT R-x\TFMT-n, page 9-72,
prints Default A. An explicit A and a defaulted A are different states with
different evidence. This contract does not infer TIMEIN, an edition or a
clock association merely to activate that default.

**L3-VIEW-009** · Parent: L2-VIEW-004 · Verification: T
*With no candidate for an otherwise supported field and no verified applicable default, the result SHALL be Missing with the absence reason, distinguishing no declared default, false applicability and unestablished default applicability/basis; it SHALL NOT invent a value or report unestablished applicability as confirmed false.*

This follows RELEASE-0.1.md's missing/indeterminate-reason contract. A present
bad value or duplicate group never enters this absence branch. Carry condition
cannot-evaluate findings alongside this Missing result, not just a false flag. Missing a reviewed field binding itself is the
separate INT-041 unavailable-read case. Two CDT occurrences make its operand
Ambiguous under INT-039, so absent TFMT is Missing with an unknown-condition
reason and cannot-evaluate finding, not a false-condition or Defaulted result.
Consumers must inspect that reason rather than infer default applicability.

**L3-VIEW-010** · Parent: L2-VIEW-004 · Verification: T
*Effective reads SHALL leave source bytes, item count/order and earlier findings unchanged; a Defaulted value SHALL NOT satisfy a separate physical-presence obligation or overwrite cross-field/packet evidence.*

Section 9.5.1 a, page 9-6, notes required recorder attributes can still be
required physically even with a default. RELEASE-0.1.md requires missing
TFMT at TIMEIN/TTF=2 to stay Defaulted A with the cited inconsistency finding;
never silently substitute another format or override recorded packet facts.
Time-view/packet implementation and complete presence validation remain later.

**L3-VIEW-011** · Parent: L2-VIEW-003 · Verification: T
*When a read would require an unavailable reviewed definition, unsupported interpretation basis or unresolved evidence needed to decide local validity, it SHALL report unavailable/cannot-evaluate with the blocking reason and retained raw candidates rather than fabricate Explicit, Defaulted, Missing or a proved Invalid-data verdict.*

This is approved INT-041, a guard on attempting a supported-field read, not a
new semantic field-value state. Preserve any already established INT-039 Ambiguous outcome or demonstrated
Invalid failure; lack of another rule must not erase known evidence. INT-037 marker blocking likewise is not evidence of an
absent key; raw access stays available. No public enum shape is fixed here.


## Condition batch 8: scoped evidence and predicate prerequisites

Planned obligations only. INT-042 Option A was approved by Joey, 2026-11-23,
under ADR-0033, with independent AI recheck concurrence on PR #28 head
`d69d275` (see the [sign-off record](research/2026-11-23-int042-owner-signoff.md)).
Earlier approved interpretations are unchanged; full development analysis and
executable tests remain outstanding. No complete evaluator/registry is claimed.

**L3-REG-005** · Parent: L2-REG-003 · Verification: T
*Each executable condition SHALL retain its original source text, edition/section/table/row, interpretation/review identity and a declared binding for each operand, including same-occurrence, linked-occurrence or document-wide scope, fixed index relationships, link traversal and whether defaulted values are eligible.*

Source: ADR-0022, L1-REG-004 and 106-24R1 section 9.5.1 a, page 9-6.
A document-wide binding must identify the intended operand set and reviewed
selection/aggregation semantics; it is not permission to choose an arbitrary
match. This batch specifies no aggregation operator. Same occurrence must
include the relevant nested indices, not only the top-level group index.
Unindexed prose cannot become a wildcard or mean occurrence 1 automatically.

**L3-REG-006** · Parent: L2-REG-003 · Verification: T
*The condition representation SHALL distinguish physical-presence predicates from interpreted-value predicates and SHALL reject an unavailable predicate/reference mapping as non-executable until reviewed; presence SHALL read actual framed, canonically matched items, not inserted defaults or value truthiness.*

Approved INT-042 supplies the distinction. A terminated empty value remains
present under INT-035; an unterminated suffix is not an explicit item for
lookups, but related suffix evidence can block a false presence answer. An
encoding-blocked semantic lookup cannot prove absence (INT-037).
Known physical absence is usable presence evidence, not a missing VALUE
operand; equality/inequality still need a usable interpreted value.
This does not reinterpret every occurrence of the English word specified
without reviewing the row, predicate and scope binding.

**L3-VAL-007** · Parent: L2-VAL-003 · Verification: T
*Condition evaluation SHALL use only the declared document/revision, edition basis and scoped operand bindings; missing bindings, unavailable scope or an unresolved/ambiguous required link target SHALL yield cannot-evaluate with the retained reason and available candidates, without searching unrelated scopes for a usable replacement.*

A known empty candidate set in an established scope differs from an unknown
target scope. Presence at an unresolved linked target is unknown, not false.
This consumes a supplied reviewed link result; it neither implements link
resolution nor infers a data-channel/time-channel association. Document-wide
and linked scopes must not silently degrade to same-occurrence lookup.

**L3-VAL-008** · Parent: L2-VAL-003 · Verification: T
*For a reviewed physical-presence predicate in an established lookup scope, one or more actual matches SHALL mean true, zero matches without compatible related damaged evidence SHALL mean false, and zero matches with such evidence SHALL mean cannot-evaluate, retaining every occurrence and its independent findings; empty, Invalid or Ambiguous values SHALL NOT make those actual matches absent, and a default alone SHALL NOT establish physical presence.*

This is approved INT-042, not a new value-resolution precedence. Duplicate
scalar items remain Ambiguous under INT-039 even though their physical
presence is established. Presence true is not permission to use either value,
claim local validity or suppress duplicate, invalidity or applicability findings.
A blocked lookup cannot establish presence or absence. A related unterminated
suffix remains outside lookup under INT-035 but blocks false: `T-1\ID:a`
at EOF gives cannot-evaluate, so a dependent T-1\AN1 is not judged not allowed.
Likewise `T-1\ID\xFF:a;` is related damaged-name evidence, not an exact match.
Retain its raw bytes and finding; do not remove the high byte to manufacture
an item. A known exact match still proves presence alongside related damage.

Use the declared literal family and index positions for diagnostic relevance,
with INT-040's compatible-scope guard: a scope is compatible when no readable
canonical parent fact contradicts it; unknown positions impose no restriction and
wrong arity must not be guessed into an alignment. An unterminated suffix may
be only a proper prefix of a literal: after `COMMENT:x;`, EOF at `T-1\I`
or `T-1\` blocks false for each predicate whose name it could complete to,
subject to readable-parent compatibility. Evaluate compatibility for the
requested predicate; do not enumerate completions or invent lookup items.
A readable contradictory literal/parent still excludes that predicate.

For this diagnostic guard, a damaged name has uninterpretable name bytes
that prevent an exact key while its readable literal/parent facts remain
compatible with the declared family. High bytes can occur anywhere in the
literal, including `I\xFFD` and `ID\xFF`, not only at its end. Keep those
bytes as damage evidence, not as characters to silently drop into a usable
key. ASCII controls already use the approved INT-036 projection; do not
reclassify projected controls as high-byte damage. This is not permission
to accept unrelated printable literals such as ID2. Do not search inside values
or repair names. Unrelated literal families and demonstrably incompatible parents do not block false for this predicate;
retain their independent findings. Do not enumerate hypothetical parents.

Match exact items through INT-036/038 projection, ASCII case folding and
canonical numeric identities; keep normalization findings and raw spellings.
All evidence belongs to the same document/setup record under INT-039 and
ADR-0025/0031 assembly; other records/documents never count or block absence.
One assembled setup record may span packets without becoming several scopes.

For a reviewed is-not-specified predicate, complement only true and false;
cannot-evaluate remains cannot-evaluate. This narrow negated-presence rule
selects no general compound-condition grammar or Boolean precedence.

**L3-VAL-009** · Parent: L2-VAL-003 · Verification: T
*A predicate requiring a value SHALL consume only a usable Explicit operand or a Defaulted operand permitted by its declared policy; Missing, Invalid, Ambiguous or unavailable operands and disallowed defaults SHALL yield cannot-evaluate with their reasons/evidence, never a guessed comparison result.*

Absence does not make inequality true; invalid content is not an unequal
usable value. Defaulted provenance and fallback edition labels stay visible.
An explicit-only policy does not relabel a Defaulted operand as Explicit or
as physical presence. INT-041 guards unavailable interpretations; blank
handling not yet reviewed stays unavailable, not an automatic mismatch.

**L3-VAL-010** · Parent: L2-VAL-003 · Verification: T
*Each cannot-evaluate condition SHALL produce its own finding naming the invoking validation pass, condition/citation, affected scope, blocking operand or dependency, edition basis and available source locations/candidates; known absence and cited defaults SHALL NOT acquire fabricated input spans.*

The invoking pass is one of ADR-0023's four passes, not a new fifth pass.
An evaluator used outside validation still returns its uncertainty/evidence;
this obligation does not require a fictitious validation pass for that call.
Do not flatten unknown into false when feeding Allowed/Required/default
applicability. Existing independently proved errors remain visible, and an
unknown condition must not generate a violation claiming the condition was
proved true or false. Detailed severity/deduplication policy remains later.

**L3-VAL-011** · Parent: L2-VAL-003 · Verification: T
*Condition evaluation SHALL leave document bytes, occurrence order, raw spellings and earlier findings unchanged and SHALL retain the operand provenance that supports a true or false result as well as a cannot-evaluate result.*

A result supplies evidence to the calling rule, not authority to delete an
inapplicable field, insert a default, repair a link or select a duplicate.
Do not certify whole-document validity from a locally true condition. This
batch leaves compound-expression grammar, dependency cycles and budgets to
subsequent reviewed requirements rather than importing Appendix 9-E's
measurement-expression operators into usage conditions by analogy.


**L3-VAL-012** · Parent: L2-VAL-003 · Verification: T
*Diagnostic damaged-name matching SHALL preserve the order of every readable character and treat each maximal high-byte run as zero or more unknown bytes, matching the whole candidate literal against the whole declared literal; an unterminated suffix SHALL additionally permit the unread EOF remainder, subject to readable-parent compatibility, without creating a usable key.*

Owner-requested INT-042 implementation clarification, 2026-11-23. Apply
INT-036 projection and ASCII case folding first; a high byte is 0x80–0xFF,
not an ASCII control. Keep each intervening readable character as a required
literal character in order, not a character set or a sortable bag. A run
can explain missing characters or no characters: `I\xFF` is compatible with
ID, `I\xFFD` with ID, and `ID\xFF` with ID. `D\xFFI` is incompatible with
ID because the readable characters are reversed. Without a high-byte run,
a terminated I is not compatible with ID; ordinary printable suffixes are
not implicitly optional. Do not turn ID2 into ID or search inside values.

This is deterministic existential matching: compatible if at least one
whole-literal alignment satisfies those constraints, not a guessed repaired
spelling. It is a diagnostic uncertainty guard only; none of these damaged
names becomes an exact lookup match. Preserve source bytes and locations.
Use declared literal components/index positions and INT-040's readable-parent
compatibility guard; do not let a literal wildcard override a contradictory
readable parent, guess wrong-arity alignment or enumerate possible parents.
An unframed EOF suffix remains diagnostic evidence, not an INT-035 item.

If a run can cover a component separator, do not assume the observed split
is complete. For example `T-1\xFFD:a;` can hide `\I`, making the component
split unknown rather than proving a different group/path. Retain unassigned
structural evidence like wrong arity under INT-040: every scope compatible
with the readable parts is guarded, never a guessed split, inferred parent
or false absence from one selected alignment. Use only unambiguously mapped
readable parent facts to exclude scopes; unknown positions impose no
restriction. Do not enumerate candidate splits or hypothetical parent values
or manufacture a usable key. Existing exact-match presence still wins when
independently established; with zero exact matches, compatible unassigned
evidence yields cannot-evaluate.

Work follows supplied name/pattern lengths using bounded matching states,
without enumerating replacements or recursively exploring an exponential
number of alignments. Concrete budget values remain a later derivation.



## Registry batch 9: reviewable source and readiness evidence

Planned obligations deriving existing decisions, not implemented tooling.
No new interpretation or change to earlier approval statuses is selected.
The source/gate derivation record lists the authorities and remaining work.

**L3-REG-007** · Parent: L2-REG-004 · Verification: T
*Each built-in source entry SHALL retain its edition, exact section or table/row identity, source text including usage and definition prose, and the source dependencies of each interpreted keyword/range/condition/default; it SHALL NOT substitute another edition's similarly named row or omit prose merely because a usage field exists.*

Sources: ADR-0004/0022; 106-24R1 Chapter 9 section 9.5.1 a, pages 9-5–9-6.
A row reference must be identifiable by its code-name pattern and parameter,
not only an unstable extracted row number. Supporting prose from another
section keeps its own citation. This sets no new table-extractor file format.

**L3-REG-008** · Parent: L2-REG-004 · Verification: T
*The registry coverage check SHALL account for each source row in its declared edition/input set as executable, not yet interpreted, or explicitly excluded with a reason, and SHALL report an unaccounted source entry as a coverage failure rather than silently removing it.*

ADR-0022 supplies this completeness/coverage distinction. Exclusion requires
an identifiable source and reason; it is not a fabricated valid rule or an
excuse to advertise excluded coverage as supported. This check consumes a
source inventory; completeness of extracting that inventory from all tables,
prose-only definitions and relationships remains a separate requirement.

**L3-REG-009** · Parent: L2-REG-005 · Verification: T
*Review metadata SHALL identify the interpretation revision, draft author, owner and sign-off date, independent AI-check result/date and evidence reference, and the exact source-text dependencies whose hash was reviewed; approval SHALL NOT be inferred from a pending review, a coordinator decision or a link to a moving branch alone.*

Source: ADR-0033 and L1-REG-003. An owner-relayed AI result may identify a
PR and immutable head as existing sign-off records do; do not fabricate a
separately posted GitHub review. The owner is the human signatory; the AI
check is recorded as non-human. This creates no agent-attribution trailers.
These gates apply to reader/validator behaviour tied to register entries,
including hand-written behaviour outside the generator. F1 (INT-021) and
F7 (INT-030) were approved by Joey, 2026-11-23, under ADR-0033, with
independent AI CONCUR at PR #30 head `13173b0` (see the
[owner approval record](research/2026-11-23-int021-int030-owner-signoff.md)). Their semantic review
is complete; source freshness, analysis and executable verification remain.

**L3-REG-010** · Parent: L2-REG-005 · Verification: T
*Freshness checking SHALL compare the current cited source dependencies with the source-text hash bound to review, flag every affected interpretation for re-review on a difference or unavailable hash, and prevent the stale interpretation from executing as currently approved.*

Hash input must cover the actual source dependencies, including definition
prose and any supporting section used by the rule, with a documented stable
encoding and boundaries; concatenation ambiguity or case/whitespace folding
must not conceal source changes. Changing the reference edition/source is
also a dependency change even if some text is identical. Hash algorithm and
storage format remain implementation choices to document and review; this
batch selects neither. Old approval evidence stays historical, not deleted.

**L3-REG-011** · Parent: L2-REG-006 · Verification: T
*Every executable interpretation dependency SHALL name an existing register entry and preserve its chosen behaviour, reason, citations, design/review status and focused test linkage; an unlisted or unresolved dependency SHALL fail executable readiness and remain visible as unavailable rather than be replaced by an inline heuristic.*

ADR-0027 requires no unregistered interpretation. Source-only and excluded
entries remain reportable; not every extracted row must immediately execute.
A rule depending on multiple interpretations needs all applicable gates,
not approval of only one. A test name without an executable test marker is
planned coverage, not implemented evidence under the trace conventions.

**L3-REG-012** · Parent: L2-REG-006 · Verification: T
*Executable readiness SHALL distinguish semantic approval, source freshness, completed linked development analysis and required verification evidence, reporting every blocking prerequisite without converting a draft analysis or planned acceptance case into completed evidence.*

Existing development conventions require source re-reading across touched
editions/handbooks, edge cases, intensive tests and applicable local samples.
Do not label currently approved INT-035–042 implemented merely because their
semantic approval is recorded. Required focused/edge/property or fuzz and
edition fixtures must be traced to the interpretation; applicability of local
sample evidence stays explicit and samples remain outside CI. This batch
does not run that analysis, implement those tests or mandate real samples
where none exercises the interpretation.
This readiness boundary also applies before implementing register-dependent
reader/validator behaviour, not merely before generating a registry rule.
No coordinator policy, including F1/F7, bypasses the approval/AI-review gate.
Earlier owner-accepted entries (INT-001–034) with Review: pending still need
the independent AI check before implementation. Existing owner acceptance
is preserved, not treated as completing the missing check. For 0.1 checksums,
INT-018/019/020 were reconfirmed by Joey, 2026-11-23, with independent AI
CONCUR at main `30e42a3`; F1 also has its owner/AI approval recorded. Their
semantic review gates are complete, while analysis/freshness/tests remain.

**L3-REG-013** · Parent: L2-REG-006 · Verification: T
*Registry generation/checking SHALL refuse to emit an executable rule with a failed readiness dependency and SHALL report its entry, blocking interpretation/source evidence and reason; its source coverage status SHALL remain visibly non-executable, without replacing a checked-in reviewed rule silently or rewriting source text to make a gate pass.*

ADR-0005 requires checked-in generation and a drift check, not downstream
build-time generation. This specifies the refusal boundary for unready rules;
complete output determinism, atomic file replacement, extractor diagnostics,
user overlays and generator CLI details remain later derivations. No current
CI workflow, registry artifact or prototype build script changes here.


## Checksum batch 10: projected recognition, raw exclusion and guards

Planned obligations, not executable checks. INT-018/019/020 were reconfirmed
by Joey, 2026-11-23, under ADR-0033, with AI CONCUR at main 30e42a3.
Source freshness, deep development analysis and executable tests remain gates.

**L3-SUM-001** · Parent: L2-SUM-001 · Verification: T
*Checksum recognition SHALL match the complete projected G\SHA name under INT-036/038, map it to the raw first contributing G/g byte through its following semicolon inclusive, exclude exactly that raw interval and hash all bytes outside it, including leading layout; noncanonical raw name spelling SHALL produce a located finding without rewriting it.*

Source: INT-018 and 106-24R1 Chapter 9 Table 9-2 page 9-13; Chapter 6
section 6.2.3.11 f page 6-22. Internal omitted name controls remain inside
raw exclusion; leading layout before the name is outside. X carried paths
and COMMENT value text are not this complete name. A suspected missing
prior terminator yields a suggestion-only framing finding, never a new
boundary/exclusion inside the previous value.

**L3-SUM-002** · Parent: L2-SUM-001 · Verification: T
*Authoritative COMPUTE SHALL refuse only when its document/exclusion domain is undefined or ambiguous, including a missing terminator, known truncation, duplicate SHA occurrences, compatible name damage, a blocking BOM or evidence of an enlarged mid-document value swallowing a following attribute; a complete, terminated SHA item with a malformed VALUE alone SHALL NOT block COMPUTE when that domain is established.*

INT-019's unterminated-item case is normally EOF. Missing-delimiter evidence
such as `G\SHA:2-h\r\nG\PN:TEST;` may leave the intended exclusion ambiguous;
retain that framing evidence and refuse authoritative COMPUTE without repairing
or choosing a boundary. Do not infer a missing delimiter or truncated record
merely from a bad digest length. For complete known input, `G\SHA:2-ABC;`
or a 63-hex-digit value has a defined raw exclusion: VERIFY is malformed,
COMPUTE is allowed. Value validity and exclusion identity are separate checks.
The same general rule permits computing with the zero placeholder, an unknown
algorithm token or unavailable value interpretation when exclusion/completeness
are established; none makes verification positive. Other findings coexist.
A separately exposed raw-byte hash stays non-authoritative when the TMATS
domain is undefined; never present it as the authoritative document checksum.

**L3-SUM-003** · Parent: L2-SUM-002 · Verification: T
*Verification SHALL report indeterminate for a compatible damaged SHA name or a leading BOM blocking semantic lookup, preserving raw evidence without a guessed exclusion; it SHALL report an exact unterminated SHA or known truncated record malformed and SHALL retain simultaneous duplicate findings without a positive result.*

Sources: INT-018/019/021, INT-037/042 and L3-VAL-012. An exact item's known
presence does not establish checksum-item uniqueness beside compatible damage.
No common status precedence hiding a proved error is chosen by this batch.

**L3-SUM-004** · Parent: L2-SUM-002 · Verification: T
*For a reviewed usable SHA-256 value, verification SHALL use INT-036's value projection, compare against an independently computed digest, report matching upper-case hex with its lower-case warning, retain located CR/LF omission information, and leave raw input unchanged; value blanks without a reviewed handling rule SHALL remain unavailable rather than be trimmed or automatically rejected.*

INT-020 reconfirmation does not choose blank handling or normalize the hashed
bytes. A projected value mismatch can be reported only when its interpretation
and exclusion are usable. INT-043's owner-decided Option B accepts ASCII-digit
algorithm tokens whose canonical string is 2, without integer conversion.
Every accepted noncanonical spelling always emits NONCANONICAL-DESIGNATOR
as a warning citing Chapter 6 section 6.2.3.11 f and Chapter 9 Table 9-2;
raw bytes stay unchanged and upper-case hex still follows INT-020. Option B
is approved under ADR-0033 (Joey, 2026-11-23; AI CONCURS on #32 at
8941ca1). Emit the warning for every located noncanonical-2 token even when
F1 duplicates, damage, BOM or undefined exclusion blocks verification; retain
all locations/blockers without turning diagnostic recognition into an exact
key or usable exclusion. Source freshness, analysis and tests remain gates.

**L3-SUM-005** · Parent: L2-SUM-002 · Verification: T
*G\SHA:0; SHALL be reported unverified with the explicit reason placeholder/no embedded digest, never match or mismatch; as an instance of L3-SUM-002's general defined-exclusion rule its complete unambiguous exclusion SHALL permit COMPUTE, and an explicitly requested stamp MAY replace the placeholder only with final-byte hashing and verification under ADR-0029.*

Source: 106-24R1 Chapter 6 section 6.2.3.11 f, pages 6-22–6-23, and the
INT-020 follow-up. This is a reported reason, not a fixed Rust representation
or proof of whole-document validity. Detailed stamp/edit contracts remain a
later slice; placeholders do not bypass F1 or unknown-exclusion guards.
The placeholder is not a COMPUTE carve-out: complete malformed digest values
also permit COMPUTE under that general rule. A stamp always writes canonical
`2-` and lower-case digits, never an input algorithm spelling such as `02-`.


## Stamp batch 11: exact targets, final-byte hashing and verified success

Planned obligations deriving ADR-0029 and the approved checksum policies.
No implementation, completed analysis, broad edit API or new interpretation.

**L3-WRT-002** · Parent: L2-WRT-002 · Verification: T
*The stamp transaction SHALL validate supplied item identities against the exact input revision and reject stale/cross-document IDs, ambiguous name-only targets and explicit edits touching the same item or overlapping bytes, reporting the implicated edits and locations without retargeting or applying a partial set.*

ADR-0029 permits a caller to identify one exact duplicate or expressly every
occurrence; bare name ambiguity never implies first/last. This does not
invent stable IDs across revisions. New supplied text must satisfy the
existing edit framing/name checks, including no semicolon inside a value;
failed target/text validation leaves the original intact.

**L3-WRT-003** · Parent: L2-WRT-002 · Verification: T
*After validating explicit edits, the transaction SHALL apply them to a new revision, rebuild affected derived state and re-read the resulting bytes before final stamping; it SHALL preserve the original revision and all bytes outside explicit edit/stamp ranges, and SHALL NOT silently repair unrelated attributes, counters, links, ordering or spelling.*

Source: ADR-0029 and L1-WRT-007. Rebuilt state reflects edited bytes rather
than stale offsets or a patched report. A checksum result does not certify
whole-document conformance or erase existing findings. This fixes no general
edit-container representation and claims no external-file rollback semantics.

**L3-SUM-006** · Parent: L2-SUM-003 · Verification: T
*Stamp eligibility SHALL be checked on the post-repair revision: remaining duplicate or compatible damaged SHA evidence, a blocking BOM, undefined exclusion or known truncation SHALL refuse the stamp; a caller's valid explicit removal of one exact duplicate SHALL remain permitted before that eligibility check.*

F1/INT-021 and L3-SUM-002 preserve the uncertainty guards. No blanket refusal
of a repair solely because its initial revision had duplicates; no silent
survivor selection. Known exact SHA presence alone does not clear compatible
damage. Open F2/F3 interpretations are not bypassed by a stamp request.

**L3-SUM-007** · Parent: L2-SUM-003 · Verification: T
*Before hashing, the transaction SHALL identify the one final SHA slot and its raw exclusion, or an explicitly requested insertion target, with the final separator/line-ending bytes established from the document's applicable style; unavailable insertion/style prerequisites SHALL be reported instead of silently choosing a new layout.*

ADR-0029 requires the document's own separators/line endings. This batch
selects no mixed/no-style fallback, implicit insertion position or new edit
syntax; detailed style resolution is later work. An existing complete slot
can be replaced explicitly, including an unverified placeholder, without
normalizing unrelated text. No TMATS default or missing field is invented.

**L3-SUM-008** · Parent: L2-SUM-003 · Verification: T
*The stamp SHALL compute SHA-256 over every final emitted byte outside the identified SHA name-through-semicolon interval, including inserted separators and newlines outside it, and write only canonical 2- plus 64 lower-case hex digits into the final value before verification.*

Sources: 106-24R1 Chapter 9 Table 9-2 page 9-13; Chapter 6 section
6.2.3.11 f page 6-22; ADR-0029. Never hash an earlier document version or
the projected/normalized source. An accepted 02/002 algorithm token in input
does not change canonical output; noncanonical diagnostics remain available
for the input on which they were observed.

**Analysis note — flex boundary:** Under L1-SUM-003, the irig106.org flex
signature stays a separately labelled, non-standard compatibility value.
A stamp never writes it into G\SHA or treats it as the SHA-256 digest.

**L3-SUM-009** · Parent: L2-SUM-003 · Verification: T
*Successful stamping SHALL require re-reading and verifying the actual final output bytes against the stamped digest, with retained output/revision provenance; any target, emission, reconstruction or verification failure SHALL return failure without claiming a verified stamp or mutating the original input.*

ADR-0029 verifies emitted output, not an earlier in-memory digest or a second
call over the wrong preimage. This is transactional success, not a promise
that a caller's external writer can undo partial I/O. Failure injection must
cover the final verification path. Full writer/CLI publication contracts are
separate; this library operation performs no file publication.

**L3-SUM-010** · Parent: L2-SUM-003 · Verification: T
*Stamp verification and diagnostic collection SHALL retain every located noncanonical-2 warning and independent input/blocking finding, even when the transaction is refused; a warning or positive checksum result SHALL NOT be presented as whole-document validation or permission to select an ambiguous source item.*

Approved INT-043's clarification forbids suppressing warnings on F1, damage,
BOM or undefined-exclusion paths. Diagnostic-only token observations remain
located evidence, not usable lookup keys or exclusions. A stamp that emits
canonical 2- need not fabricate a noncanonical warning for its new value;
retain input warnings with their original-revision provenance.


**Analysis note — noncanonical name slots:** Stamping a value into `g\sha:`
or `G\S\r\nHA:` keeps INT-018's located noncanonical-name finding prominent
in the stamp result. Recommend an explicit rename edit first; a stamp alone
never silently renames the slot or makes the name canonical. This recommendation
adds no automatic repair or new refusal gate when stamping is otherwise eligible.
