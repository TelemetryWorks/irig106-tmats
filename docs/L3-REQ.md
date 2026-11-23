# irig106-tmats — Level 3 Requirements

> **Status: foundations through counter batches drafted (family 2, batches 1-6).**
> These are implementation obligations, not existing interfaces or completed
> tests. INT-035/036/037/038/039/040 are approved; INT-040 sign-off is dated 2026-11-22.
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
The zero/one-candidate five-state resolver is not fully derived here.

**L3-VIEW-004** · Parent: L2-VIEW-002 · Verification: T
*Ambiguous scalar resolution SHALL NOT substitute a default, infer an absent value, coalesce equal candidates or become explicit after reordering input or changing diagnostic severity; raw output SHALL continue to reproduce every retained byte.*

Approved INT-039 refines ADR-0023's ambiguous state for repeated slots.
Order changes reporting order only. This does not define link selectors,
packet evidence, caller assumptions or a data-channel/time-source association.

**L3-READ-033** · Parent: L2-READ-013 · Verification: T
*General scalar-duplicate findings SHALL coexist with the approved per-token normalization and canonical-group collision information findings, and SHALL NOT turn duplicate G\SHA integrity into a positive verdict or permit stamping.*

INT-038 already requires collision information for distinct raw index-spelling
tuples and lists exact repeats too. Scalar duplicates can exist even with
only one raw spelling tuple. Coordinator-decided F1 (ADR-0034, owner may
override; L1-SUM-002) separately requires indeterminate integrity and stamping
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
