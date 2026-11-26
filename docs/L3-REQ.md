# irig106-tmats — Level 3 Requirements

> **Status: foundations through extraction/safe-output boundaries drafted (family 2, batches 1-25).**
> INT-056–060 are approved (Joey, 2026-11-26; final AI CONCUR at #52 3fcbd2c).
> INT-054/055 are approved (Joey, 2026-11-25; final AI CONCUR at #51 ebd5c98).
> INT-012 is reconfirmed; INT-044 Option B is approved (Joey, 2026-11-23; AI CONCURS on #36 at 6f6e080).
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

The full sentence is source evidence. Owner-signed INT-055 now supplies a
numeric blank rule, with final independent CONCUR at #51 ebd5c98, retaining INT-041's existing
leading-blank guard; other letter/structured-field blank rules stay separate.
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


## Setup-payload batch 12: raw spans and independent field evidence

Planned obligations, not implementation. Baseline layout is explicitly
106-24R1 Figure 11-34; historical era/declaration routing remains later work.
INT-012 was reconfirmed by Joey, 2026-11-23, with AI CONCURRED at #34 head
e352323. Source freshness, deep analysis and executable verification remain.

**L3-CH10-001** · Parent: L2-CH10-001 · Verification: T
*For the identified Figure 11-34 layout, payload decoding SHALL require at least four bytes, preserve the input and supplied provenance on failure and report a located short-CSDW result for lengths 0–3 without reading unavailable bytes, zero-padding or inventing a version/format/change flag.*

Source: 106-24R1 Chapter 11 section 11.2.7.2 a, Figure 11-34, page 11-50.
The raw CSDW spans logical payload offsets [0,4), which may cross borrowed
DATA spans under ADR-0036; no contiguous intermediate is required.
A short CSDW is not a supported
empty document. Unknown/unavailable layout knowledge is a separate blocked
interpretation, not proof of malformed bytes or permission to use baseline
semantics silently. No concrete Rust result/diagnostic enum is fixed here.

**L3-CH10-002** · Parent: L2-CH10-001 · Verification: T
*With a complete baseline CSDW, the fragment-body span SHALL be exactly [4,payload_length), preserving every byte in that supplied interval including layout, high bytes and trailing NUL/0x1A; removing the CSDW SHALL NOT trim the text, insert separators, transcode it or identify the fragment as a complete setup record.*

The caller supplies a packet-body slice already bounded by Data Length;
filler and data checksum are outside it (ADR-0025). This function does not
search raw packets or remove bytes by padding heuristics. Each fragment has
its own CSDW: later assembly logically concatenates text spans, not embedded
CSDWs. Under ADR-0036, [4,payload_length) is a logical interval represented
by borrowed subspans, not a requirement for one contiguous byte slice.
BOM meaning is checked at assembled-text offset zero under INT-037, not
independently stripped at every fragment boundary.

**L3-CH10-003** · Parent: L2-CH10-002 · Verification: T
*The baseline CSDW SHALL be read as a little-endian 32-bit wire value using the shared layout definitions, reporting RCCVER bits 7–0, SRCC bit 8 and FRMT bit 9 with their raw word/byte provenance; host-native casts, inferred byte order and duplicate local wire-code types SHALL NOT supply the interpretation.*

Sources: 106-24R1 Chapter 11 section 11.1 page 11-1 (least-significant byte
first); Figure 11-34 and field text page 11-50; ADR-0009. Shared decoder
integration needs light adapter tests against independently supplied bytes,
not expected values computed through the same shared decoder. Shared API
names/version availability are not asserted by this documentation batch.

**L3-CH10-004** · Parent: L2-CH10-002 · Verification: T
*Under the baseline layout, FRMT=0 SHALL identify Chapter 9 ASCII-format body bytes for later complete-record processing, while FRMT=1 SHALL report XML unsupported and retain the entire body without feeding it to the code-name scanner or silently returning an empty parsed TMATS document.*

Figure 11-34 field text page 11-50 defines both flag values; ADR-0015 excludes
XML support. A four-byte-only ASCII payload has an empty fragment body,
not evidence of complete valid TMATS; a four-byte-only XML payload still
reports XML unsupported. This selects no XML dialect or conversion policy.

**L3-CH10-005** · Parent: L2-CH10-002 · Verification: T
*The local SRCC value SHALL be reported as the supplied fragment's change indication, without inferring actual configuration difference, the session's first record, event presence or agreement with other fragments; per-fragment evidence SHALL remain available for later summary/session analysis.*

Source: Figure 11-34 SRCC field definition page 11-50. ADR-0025 requires
FRMT/RCCVER agreement for joining; it does not require SRCC agreement.
INT-030's approved uncertainty policy still supplies no event identity or
meaning of prior. This batch chooses no SRCC aggregation across fragments.

**L3-CH10-006** · Parent: L2-CH10-002 · Verification: T
*The decoded report SHALL retain reserved bits 31–10 and the original four-byte CSDW unchanged, without clearing them, reconstructing a sanitized word or presenting successful field extraction as packet/document validity.*

Figure 11-34 page 11-50 marks these bits reserved. This preservation child
selects no new reserved-bit severity/rejection policy and does not waive a
future cited validator rule. A raw nonzero reserved bit must not shift FRMT,
SRCC or RCCVER, disappear from source access or trigger a fabricated default.

**L3-CH10-007** · Parent: L2-CH10-003 · Verification: T
*With the baseline RCCVER table identified, the report SHALL retain its code and literal reading: 0x07 through 0x0E name RCC 106-07, 09, 11, 13, 15, 17, 19 and 22 respectively; 0x00–0x06 and 0x0F–0xFF SHALL retain their reserved/unknown classification under that table, without being rejected or relabelled as a known newer edition merely for being unrecognized.*

Source: 106-24R1 Chapter 11 section 11.2.7.2 a/Figure 11-34, pages 11-50–11-51; ADR-0032/INT-012. This mapping is Format 1 only.
Any later archive-reuse note is labelled observation with its source/basis,
not part of the declared literal. No code 0x0F is invented for 106-23. The
pre-106-07 absence/CH10VER question (INT-016) is a later declaration wrapper;
this baseline wire classification does not claim historical-field presence.

**L3-CH10-008** · Parent: L2-CH10-003 · Verification: T
*Payload/version reporting SHALL keep CSDW wire declarations, packet data-type-version metadata, G\106 text and any applied interpretation basis distinct, and SHALL NOT infer one declaration from another, select a fallback edition, report their difference as an automatic conflict or assign a time source/association from the decoded version byte.*

ADR-0028/0032 preserve declaration versus policy. Edition selection and
historical layouts need separate reviewed requirements and INT-014/015/016/
017 evidence; this child consumes no fabricated G\106 value. Raw bytes,
provenance and incomplete/unsupported outcomes stay visible to those callers.

Code-space guard: Format 4 uses its own stale RCCVER table (106-24R1
section 11.2.7.5/Figure 11-43, page 11-62), 0x0C = RCC-106-17 and
0x0D–0xFF reserved; never borrow Format 1's table. The packet-header Data
Type Version table (section 11.2.1.1 e, page 11-6) separately maps 0x0A to
RCC 106-22, while Format 1 maps that byte to RCC 106-13. Figure 11-41 in
106-24R1 is an index header; use the correct edition/structure citation.
Fallback rules 0x0E -> 106-24R1 and 0x0D -> 106-20 belong to INT-017 policy,
not literal decoding, and require their own review/evidence before execution.



## Edition-basis batch 13: evidence before precedence

Planned obligations. INT-044 Option B is approved (Joey, 2026-11-23;
AI CONCURS on #36 at 6f6e080);
existing INT-014/015/016/017 review statuses are unchanged. Tests below supply reviewed fixture bindings where
necessary; they are not production-registry or cross-edition analysis evidence.

**L3-EDN-001** · Parent: L2-EDN-001 · Verification: T
*The declaration report SHALL retain document/revision and occurrence identity, each raw G\106 value and location, candidate editions or unrecognised/unavailable/ambiguous reason, and the separate setup-record version reading with code-space/table citation and original wire code; a rules-basis decision SHALL NOT overwrite those observations.*

Sources: Chapter 9 Table 9-2 page 9-10 and Chapter 11 section 11.2.7.2 a
page 11-51 (106-24R1); ADR-0028. Same-year candidate editions from one
attribute are not repeated scalar items. INT-039 still makes actual repeated
G\106 items Ambiguous, even when values agree; raw lookup keeps every item.

**L3-EDN-002** · Parent: L2-EDN-001 · Verification: T
*G\106 reading SHALL use only an available reviewed edition-specific binding and retain all candidate editions it establishes; one year naming multiple editions SHALL use a newest shared-rules basis only when reviewed evidence establishes those candidates share the applicable Chapter 9 rules, not by arbitrarily choosing a candidate or borrowing a baseline parser.*

INT-014 covers the year/candidate decision; INT-015 covers unrecognised older
spellings. Their owner decisions are recorded but independent review remains
pending. No unknown value format, high-byte/blank projection or missing rule
is silently reclassified as a known year or data-level unrecognised verdict.
Historical absence/CH10VER semantics remain INT-016's separate dependency.

**L3-EDN-003** · Parent: L2-EDN-002 · Verification: T
*A valid explicit caller override SHALL take precedence as an override-labelled rules basis while retaining every declaration/candidate and independent finding; an unusable override SHALL report its blocking reason rather than silently fall through or pretend the caller declared G\106.*

ADR-0028 supplies override precedence. An override resolves the caller's
choice of rules, not source ambiguity or data validity. It supplies no
permission to mutate G\106, RCCVER, body bytes or their provenance.

**L3-EDN-004** · Parent: L2-EDN-002 · Verification: T
*Without an override, a usable reviewed G\106 rule basis SHALL be selected as declared with the original declaration and candidate/shared-rule evidence; a differing Format 1 RCCVER SHALL remain separately reported and SHALL NOT automatically produce a conflict finding or change that declared basis.*

Source: ADR-0028/0032. A raw year that cannot distinguish revisions is not
rewritten as a direct revision declaration merely because one shared-rules
basis is applied. User-defined discrepancy rules are separate from automatic
edition selection. Unsupported candidate interpretation remains unavailable.

**L3-EDN-005** · Parent: L2-EDN-002 · Verification: T
*With no override and an established missing or reviewed unrecognised G\106 declaration, fallback SHALL be labelled policy with its reason: eligible Format 1 RCCVER 0x0D selects 106-20 rules and 0x0E selects 106-24R1 rules under INT-017; without a usable eligible RCCVER the baseline fallback SHALL remain labelled 106-24R1 policy, not a declaration.*

ADR-0032/INT-017 are the policy authority; INT-017's own review gate remains
pending and is not cleared by INT-012 reconfirmation. Test raw 0D/0E literal
readings independently from these supplied policy outputs. Format 4's table
and header Data Type Version cannot supply this Format 1 policy operand.
Unknown interpretation knowledge is not proof of missing/unrecognised data.

**L3-EDN-006** · Parent: L2-EDN-002 · Verification: T
*Under owner-decided INT-044 Option B, Ambiguous G\106 without an explicit override SHALL retain every occurrence and duplicate finding, select no declaration candidate, and use the unchanged INT-017 fallback policy with the explicit label `fallback — reason: ambiguous G\106 (n candidates)`; n SHALL count every scalar occurrence, including identical duplicates, not distinct values or edition readings.*

This NEW reason is never missing/unrecognised. INT-017 supplies the same
Format 1 0x0E -> 106-24R1 and 0x0D -> 106-20 policy, its existing lower-code
readings, else baseline when no usable eligible RCCVER exists; no new mapping.
Identical-value agreement is information only, not a declared basis. A valid
override takes precedence while preserving ambiguity. Separate records or
documents do not form duplicates. A single high-byte/blank value remains
subject to INT-037/041, not this branch. One year with several candidate
editions remains INT-014's separate case. INT-044's review gate is complete;
analysis, source freshness and executable tests remain outstanding. Mixed recognised/unrecognised duplicates
retain ambiguity first: `G\106:zz;G\106:24;` has n=2 and this reason.

**L3-EDN-007** · Parent: L2-EDN-002 · Verification: T
*Every selection result or blocked selection SHALL carry its document/record scope, relevant caller/declaration/policy evidence, applied edition if selected, basis label and precise reason; a selected edition without available reviewed rule coverage SHALL remain a coverage limitation, never a false claim of validation or a silently substituted supported edition.*

Defaulted views and validation findings keep the basis/fallback label under
ADR-0023/0028 and INT-041, including INT-044's explicit ambiguity reason.
CLI text and JSON SHALL make that fallback basis/reason at least as prominent
as a cannot-evaluate result. JSON SHALL use stable machine code
`ambiguous_g106` with numeric `candidates`, separate from its display label
and counting occurrences as L3-EDN-006 specifies. Edition selection changes no source bytes and
clears no other findings. Full out-of-range compatibility-check selection
under L1-EDN-006 is a later derivation, not an implied fallback here.

## Assembly-contract batch 14: candidate runs and exact source mapping

Planned, not implemented. INT-011 is reconfirmed by Joey, 2026-11-23,
with AI CONCURS on #38 at c623b15; development gates remain;
INT-034's precise completeness rules remain proposed under ADR-0031/0033.
These children describe the joining/provenance stage within a phase only.
INT-045 separately records approved time-packet phase closure (AI CONCURS
on #41 at 79b94eb); it adds no per-channel joining predicate. A later +1 fragment
across a phase boundary is continuous-but-split, never rejoined backward.
These children do not settle complete/incomplete/ambiguous classification, adjacent-record splits, SRCC
aggregation, configuration-event identity or a configuration timeline.

**L3-CH10-009** · Parent: L2-CH10-004 · Verification: T
*Assembly input SHALL retain caller-supplied recording order, channel identity, per-channel sequence evidence, data type, raw CSDW and fragment text bounds; joining SHALL maintain each channel's independent run(s), and SHALL retain intervening OTHER-channel packets as provenance/information only, never as a stop condition.*

INT-011's owner clarification (Joey, 2026-11-23) interprets consecutiveness
per channel, not globally. The +1 modulo-256 test detects a same-channel
packet's consumed sequence number; a setup-only filtered list must not hide
that evidence. No global packet-adjacency predicate is required. Missing
channel/sequence evidence stays unavailable, not fabricated from file-offset
proximity or equal timestamps. This freezes no API shape. Packet slicing
remains outside the library (ADR-0010/0025); L3-CH10-001/002 supply its bounds.

**L3-CH10-010** · Parent: L2-CH10-004 · Verification: T
*Under an available reviewed INT-011 binding, two consecutive fragments within the same channel SHALL join only when both have data type 0x01, the same channel, sequence numbers differing by +1 modulo 256, and matching decoded FRMT and RCCVER; the assembler SHALL record the source evidence for those predicates without replacing either raw CSDW.*

Source: Chapter 11 section 11.2.7.2 and section 11.2.1.1 f (106-24R1),
ADR-0025 and INT-011. Sequence 255 then 0 satisfies +1; 255 then 1 does not.
This tests wire-code agreement, not equivalent fallback rule editions.
SRCC is not an added joining predicate; preserve each SRCC reading and leave
its disagreement/summary interpretation visibly unresolved for later review.

**L3-CH10-011** · Parent: L2-CH10-004 · Verification: T
*A SAME-CHANNEL intervening packet, per-channel sequence discontinuity, FRMT/RCCVER change or end of supplied input SHALL stop the affected channel's joining with its actual evidence retained; OTHER-channel packets SHALL NEVER stop, mix or interrupt that run; none of those stop reasons alone SHALL prove a complete record, a missing fragment or a chosen adjacent-record split.*

INT-011's 2026-11-23 owner clarification makes an intervening packet
same-channel only; the +1 sequence test already detects it. Other channels
continue their own runs independently. Channel 0x0000 from 106-17 is an
edition-dependent conformance check (Chapter 11 section 11.2.1.1 b), not a
joining rule; nonzero setup-channel evidence is preserved in every edition.
ADR-0031 still governs completeness. Keep unreadable/untrusted evidence;
this selects no recovery or completeness result. INT-034 needs run-start
as well as stop evidence to consider leading truncation (L3-CH10-016).

**L3-CH10-012** · Parent: L2-CH10-004 · Verification: T
*The candidate text SHALL be the exact concatenation, in recording order, of each joined fragment's supplied text spans after its own CSDW; no CSDW, header, filler, packet checksum or separator SHALL be inserted into that text, and no code name, value or encoding marker SHALL be interpreted separately merely because it crosses a fragment boundary.*

Chapter 11 Figure 11-34/section 11.2.7.2 page 11-50 and ADR-0025 establish
per-packet CSDWs followed by fragment text. Boundaries do not add a newline,
colon or semicolon. Once completeness is established by its own reviewed
contract, scanner/INT-035 framing, INT-037 offset-zero marker recognition and
checksum exclusion rules use assembled text coordinates, not packet-local
coordinates. This authorizes no parse/checksum of a run as if proven complete.

ADR-0036 requires this concatenation to be logical over borrowed spans,
not a mandatory copy into a contiguous allocation. The assembler takes
non-contiguous DATA directly; reading across a block does not create a new
packet/CSDW or alter INT-011/045 joins/closure.

**L3-CH10-013** · Parent: L2-CH10-005 · Verification: T
*Every assembled text byte SHALL map to exactly its original packet and byte offset, retaining the corresponding recording/file identity and fragment identity; the mapping SHALL cover the concatenated text without gaps or fabricated bytes and SHALL preserve all supplied fragment metadata independently of any record-level summary.*

ADR-0025 requires offset provenance; ADR-0003 retains original source bytes.
Packet-relative text starts include that fragment's own four-byte CSDW.
The mapping uses core-supplied DATA spans and source origins, not a guessed
fixed header length; block splits may occur within a packet (ADR-0036). Empty
text contributes no byte interval but does not discard fragment metadata.

**L3-CH10-014** · Parent: L2-CH10-005 · Verification: T
*A requested half-open assembled-text range SHALL map to its ordered source subranges, splitting at span/block and packet-fragment boundaries and retaining every contributing byte; an invalid/out-of-bounds range SHALL report its invalid request rather than clamp, wrap or identify unrelated packet bytes.*

This is an offset-mapping contract, not text normalization. A boundary offset
belongs to the following nonempty interval for byte lookup; the end offset
of the whole body is not a byte. Range [k,k) contains no source bytes. No
numeric width or public Rust type is frozen here. Arithmetic must respect
L1-ROB-001; resource-policy thresholds remain a separate bounded design.

**L3-CH10-015** · Parent: L2-CH10-005 · Verification: T
*Located scanner/value findings over an assembled span SHALL retain that assembled span and all contributing packet subranges, without selecting only the first or last fragment; missing/defaulted values without input spans SHALL NOT acquire invented packet locations.*

ADR-0025's provenance map serves L1-READ-003 diagnostics; effective-field
provenance follows INT-041. A field split in a code name, value or terminator
is still located in the one assembled text. Raw byte spans, not interpreted
character counts, determine the subranges after INT-036 control projection.
This does not change finding severity or authorize scanning incomplete data
as a usable document.

**L3-CH10-016** · Parent: L2-CH10-004 · Verification: T
*The joining report SHALL retain candidate-run identity, fragment membership, START reason alongside stop reasons and the status of its joining interpretation separately from later completeness, declaration and checksum results; successful concatenation SHALL NOT claim a usable document, authoritative digest, governing configuration or interpretation approval.*

Each run retains an evidenced START reason, e.g. "start of supplied input"
or restart after a same-channel interruption/sequence or FRMT/RCCVER change;
these reasons do not assert that the beginning is intact. Other-channel
traffic never creates a new start for an existing run. Implementation of
reconfirmed INT-011 still requires completed analysis and executable checks.
INT-034
requires its own owner sign-off and independent check for the precise rules;
INT-039 duplicate classification applies to a document only after assembly
scope is established. Cross-fragment duplicate evidence must not silently
select a split or a scalar winner. These dependencies remain visible even
when fixture-provided reviewed bindings make individual joining tests pass.

## Completeness-contract batch 15: observation versus authority

Planned obligations; INT-034's precise rules and INT-028/031 governance are
not approved. Their supplied reviewed fixture bindings isolate the cases
below without completing any interpretation gate. The open-run observation
is PROVISIONAL, not a fourth final completeness outcome or a joining stop.

**L3-CH10-017** · Parent: L2-CH10-006 · Verification: T
*A closed-run report SHALL retain channel/run identity, observed raw text extent, START and stop evidence, final complete/incomplete/ambiguous outcome or unavailable reason, and every competing boundary reading with locations; classification SHALL NOT discard bytes or choose an implicit split.*

ADR-0031/INT-034 define the three final ASCII outcomes in principle. FRMT=1
remains unsupported/unknown completeness, not parsed as ASCII. An unavailable
reviewed classification rule cannot be presented as a positive completeness
verdict. This does not approve the precise proposed branch precedence.

**L3-CH10-018** · Parent: L2-CH10-006 · Verification: T
*Under a supplied reviewed INT-034 classification binding, an otherwise complete run whose text ends in semicolon at supplied EOF SHALL be Complete with labelled basis "end of input"; "followed by another packet" SHALL refer only to a SAME-CHANNEL packet that actually ends joining, never another channel's traffic.*

Owner-relayed review consequence, 2026-11-23. EOF alone must not turn a valid
final record into incomplete or ambiguous. Preserve the START observation
without interpreting "start of supplied input" as known leading damage.
Independently established damage/ambiguous split evidence remains reported;
its precedence needs review. INT-045's layered time boundary is a separate
reason, not relabelled as this ordinary same-channel joining stop. At that
boundary, cut text is INCOMPLETE with a located finding, and a later
same-channel +1 fragment is continuous-but-split, never rejoined backward;
an otherwise Complete semicolon-ending seal followed by same-channel +1,
matching-FRMT/RCCVER continuation becomes finally AMBIGUOUS with both
readings and a located "possible premature time-boundary seal" finding,
unless reviewed, unambiguous G\SHA verifies the sealed text alone. That exception
retains Complete as classification evidence, never a backward rejoin; every
mid-session Complete under this layer carries `time-boundary seal (owner policy)`.
Revised outcomes/findings remain visible to spool-before-publish consumers.
INT-045 is approved (Joey, 2026-11-23; AI CONCURS on #41 at 79b94eb). This EOF case ends exactly at `;`;
trailing padding/controls require reconciliation with INT-035/036, not a new
unreviewed trim operation. Finality is not proof of source data validity.

**L3-CH10-019** · Parent: L2-CH10-006 · Verification: T
*Incomplete/ambiguous classification evidence SHALL retain the affected spans, gap/trust/boundary facts and all competing readings without concatenating across a missing segment, synthesizing a terminator or selecting one repeated scalar occurrence as the intended record boundary.*

INT-034's precise branch conditions remain proposed. INT-039's duplicate
field rule and INT-044's ambiguous-declaration fallback do not establish a
record split or override assembly ambiguity. Locating evidence is mandatory;
deciding which classification wins when several kinds coexist is still an
open owner-review item. No positive governing or checksum verdict follows.

**L3-CH10-020** · Parent: L2-CH10-007 · Verification: T
*While a run remains open, a boundary observation such as "text ends at ;, run still open" SHALL be labelled PROVISIONAL with its channel, observed extent and current evidence; further qualifying same-channel fragments SHALL remain joinable, and other-channel packets SHALL neither finalize nor restart the run.*

The observation can change when more text arrives; it never asserts EOF or
closes a run to make a streaming result available. An in-value suffix remains
visible rather than being discarded to expose the preceding `;`. Proposed
semantic snapshots would need exact revision/extent and invalidation rules;
this requirement exposes evidence without approving that interpretation.

**L3-CH10-021** · Parent: L2-CH10-007 · Verification: T
*Any consumer-facing report carrying provisional setup evidence SHALL retain that provisional status and its basis separately from the governing-record decision; absent a reviewed provisional-use contract, it SHALL NOT present the observation as a complete, verified or automatically governing setup record.*

Open owner choice in INT-034: opt-in, explicitly labelled semantic snapshots
for irig106-time's CLI versus final-only semantic use. Recommend opt-in
snapshots to avoid waiting until EOF while requiring later changes and every
assumption to remain visible. This recommendation is undecided; it requires
reconciling the current complete-only parse/checksum boundary and INT-028/031,
including what to do with outputs already emitted when later text invalidates
a snapshot. No default permission or new public API is selected here.

**L3-ROB-004** · Parent: L2-ROB-003 · Verification: T
*The open-run budget SHALL be finite and explicit across all retained channels, with a documented byte-accounting model covering retained text, fragment/run metadata, provenance and any assembler-retained snapshots; an operation SHALL use non-wrapping arithmetic to check its additional charge and SHALL NOT accept retained state beyond the budget.*

The model must bound retention, including empty-fragment/empty-text metadata
and copies, not just concatenate-length totals or one run at a time. Numeric
limits, allocator overhead accounting and ownership of caller-held snapshots
remain review work; no implementation may advertise a hard byte bound from
an incomplete model. A fixture-provided accounting model tests the contract,
not a production model's accuracy. No Chapter 9 maximum is invented.

**L3-ROB-005** · Parent: L2-ROB-003 · Verification: T
*Budget exhaustion SHALL return a typed limit error identifying the configured limit, charged state and rejected operation's channel/run evidence, retaining or returning accepted/rejected bytes and provenance through an explicit failure path rather than silently dropping data; arithmetic overflow SHALL report a typed limit failure rather than wrap or panic.*

The operation may not silently evict another channel, truncate its text,
reclassify a run as complete or pretend an error was EOF. Detailed failure
atomicity, resumability and caller ownership need review before code; the
caller must be able to identify what was accepted versus rejected. This
requirement does not permit unlimited error-buffer retention or local spill
I/O (ADR-0010). It selects no concrete Rust error name or numeric threshold.

**L3-ROB-006** · Parent: L2-ROB-003 · Verification: T
*A caller-visible budget failure SHALL remain distinct from TMATS malformed-data findings and complete/incomplete/ambiguous/provisional observations, and reporting the failure SHALL NOT manufacture a joining stop, a governing record or a successful checksum result.*

Resource exhaustion is an operational limitation, not evidence that the
recording ended or was invalid. Other runs may be reported only with their
retained actual state; no hidden "keep the first channel" policy is allowed.
Recovery/resume and any opt-in disposal action need their own explicit
contract. Passing a limit test does not prove all L1-ROB-001 input handling.


## Whole-matrix verification contract (owner-signed INT-046–053)

These are future implementation obligations. Owner sign-offs dated 2026-11-25
are recorded with independent AI CONCUR under ADR-0033 at PR #45 head
`73396bc312e24cfccaf16de9ae7a72609faf75b6`. No executable coverage is claimed
by the checked-in documentation artifacts. Source freshness, development
analysis and executable verification remain implementation gates.

**L3-REG-014** · Parent: L2-REG-007 · Verification: T
*The implementation test harness SHALL deterministically enumerate every Cartesian-product member of the versioned retransmission matrix, currently 139,968, and assert exact domain membership, unique case identity and full equivalence-class membership without sampling or omission; each realizable member SHALL exercise actual bytes/provenance and independently checked expected sealed classification/basis, following classification, located findings and governing effect, while inconsistent observations SHALL receive separately counted consistency/rejection tests and unresolved owner questions SHALL fail readiness visibly.*

The matrix artifacts live in `docs/research/retransmission-matrix/`; class
compression must not hide members. Independent expectations require reviewed
rules/fixtures, not calling the documentation generator or production
classifier to compute an expected answer. No assertion that every nominal
nonconflicting tuple is realizable is made: witness construction must expose
further constraints, never silently skip tuples. Changes to dimensions/rules
require a reviewed matrix revision and a fresh coverage reconciliation.

**L3-REG-015** · Parent: L2-REG-007 · Verification: T
*Whole-matrix enumeration SHALL be supplemented by named byte/provenance fixtures for SRCC=0 with differing content, SRCC=1 with identical content, checksum only on the following copy, and checksum only on the sealed copy; independent digest oracles and boundary/character mutations SHALL verify raw equality/exclusion, all inherited findings, no silent rejoin or byte loss and no positive F1 verification, with test markers naming INT-046 and its applicable interpretation dependencies.*

Use independently computed hashlib digests, not the implementation or Chapter
6's unreproducible example digest. Mutations include SHA-value/layout/other
raw differences, partial-prefix matches, projected G\SHA names, unknown
attributes, leading layout, value-contained text, missing terminators,
uppercase hex, noncanonical algorithm 2, BOM/damage, F1 duplicates, differing
FRMT/RCCVER/channel and actual EOF. Property-based mutation complements the
full enumeration, never replaces it. The pairwise artifact separately preserves
owner-signed baseline with QD/INT-048 before QI/INT-046: field ambiguity
forces assembly Ambiguous, never upgraded by identity/equality/digest, while
cut evidence and INT-045 findings remain visible. A retained cut facet SHALL
keep Incomplete's full blocking force: no ambiguous-split acceptance, including
proposed INT-031 if later approved, SHALL make that cut text usable or governing. Named compound fixtures
SHALL cover non-SHA field ambiguity, cut-plus-ambiguity, and both candidates;
input-state flattening must not be mistaken for exhaustive byte-shape coverage.
QM composition is owner-signed in INT-053 and verified separately by
L3-REG-016; pairwise enumeration SHALL NOT be reported as whole-sequence
verification. This remains planned, not tested today.


**L3-REG-016** · Parent: L2-REG-007 · Verification: T
*The implementation SHALL verify INT-053 with named bounded sequence fixtures of an original established governing setup and up to three later copies, comparing each with the currently governing identity or explicitly unresolved target, promoting only a copy established to have validly taken control, and preserving every prior finding and historical governing setup used for earlier decoded data; equality to the immediately previous copy SHALL be information only and SHALL NOT change eligibility, governing or classification.*

Fixtures SHALL cover B→D, H→B with a definitely unadopted H copy, a third-only
checksum, changed SRCC claims, verified takeover, definite non-takeover and
unresolved takeover (warn-mode integrity failure and Ambiguous). Named
`qm_unresolved_then_valid_takeover_resolves_target` SHALL cover unresolved C1
followed by independently established valid takeover C2, then C3 comparing
with C2 while C1 findings and unresolved history remain preserved. Named
`qm_unresolved_persists_through_non_takeover` SHALL cover unresolved C1,
definite non-takeover C2 and an unresolved target for C3. A mutation
holding governing/candidate/adoption evidence fixed while changing only
previous-copy equality SHALL leave decision outputs unchanged. Evidence of
unresolved takeover SHALL NOT select an old/new target by position or checksum;
QD's adoption block alone is not proof of the recorded data's old governor.
The three-copy bound is a test bound, not a production history/memory default.
Assertions use independently specified control evidence and observable
provenance, not a production state machine as its own oracle. These are
planned tests; independent AI CONCUR is recorded at PR #45 head `73396bc312e24cfccaf16de9ae7a72609faf75b6`.
Source freshness, development analysis and executable verification remain
required before implementation.


## Time-view metadata batch 16: scoped channel and format reads

These are future implementation/test obligations for the owner-approved 0.1
subset, not existing time-view interfaces or passing coverage. Their sources
and boundary are recorded in the [batch note](research/2026-11-25-time-view-metadata-requirements.md).
INT-035–039/041 are approved. Owner-signed INT-055 subsequently supplies the
numeric field-value rule (final AI CONCUR at #51 ebd5c98). This batch does not
choose an association field, packet-type correspondence or governing algorithm.

**L3-VIEW-012** · Parent: L2-VIEW-005 · Verification: T
*Time-view field binding SHALL use the same source document/revision and the full canonical recorder/entry index pair (x,n), using the INT-036/038 projected code-name identity while retaining each raw spelling and normalization finding; neither equal suffixes on different recorders nor equal IDs in different documents SHALL combine field evidence.*

An entry's code-name index is not its actual channel ID. Location and raw
lookup still return every occurrence. Index canonicalisation never rewrites
the input, changes value bytes or authorises a record/revision join.

**L3-VIEW-013** · Parent: L2-VIEW-005 · Verification: T
*An entry SHALL be classified as confirmed TIMEIN only from a singular usable CHANNEL DATA TYPE result equal to TIMEIN in its own scope; missing, invalid, ambiguous or unavailable CDT evidence SHALL remain explicitly unconfirmed with its candidates/reason, without inferring TIMEIN from TTF/TFMT/TSRC presence, an ID or source order.*

Citation: 106-24R1 Table 9-4 CHANNEL DATA TYPE, page 9-50. Approved INT-039
multiplicity applies even to identical CDT repeats; INT-041's unavailable
read is not Missing or a fabricated Invalid verdict. Relevant unconfirmed
entry evidence stays inspectable rather than disappearing as empty success.

**L3-VIEW-014** · Parent: L2-VIEW-005 · Verification: T
*The channel identity result SHALL read R-x\TK1-n from the entry's own scope, preserve its effective state and original value/location, and require a usable in-range declaration before exposing an actual channel ID; a missing, invalid or ambiguous ID SHALL NOT be replaced by n, zero, a neighbouring entry or a packet's ID.*

Citation: Table 9-4 TRACK NUMBER/CHANNEL ID, page 9-48, range 1–65535.
An entry can have a confirmed TIMEIN CDT while its actual ID is unusable;
keep that scoped evidence with the identity limitation. Known counter or
other findings are retained, not silently used to create missing entries.

**L3-VIEW-015** · Parent: L2-VIEW-005 · Verification: T
*A request to identify time metadata by an actual channel ID SHALL retain all matching scoped entry candidates and any required recorder/source-scope limitation; multiple candidates SHALL NOT be merged, deduplicated or selected by first/last occurrence, and an absent match SHALL NOT synthesize an entry.*

This is an uncertain selection contract, not a new standards conformance
rule asserting global TK1 uniqueness across recorders. Each candidate keeps
its own CDT/format results; do not manufacture scalar-value ambiguity by
merging independent scopes. Caller context cannot silently cross a revision.

**L3-VIEW-016** · Parent: L2-VIEW-006 · Verification: T
*For a confirmed TIMEIN entry, reads of R-x\TTF-n, R-x\TFMT-n and R-x\TSRC-n SHALL bind separately to that exact recorder/entry scope and use their own reviewed definitions, states and raw locations; duplicate or missing evidence in one field SHALL NOT replace the other fields or be repaired with data from recorder-level extras or another entry.*

Citation: Table 9-4 TIME DATA TYPE FORMAT, pages 9-71–72; TIME FORMAT and
TIME SOURCE, page 9-72. A typed view cannot depend on prototype extra-vector
routing. Supported local reads remain available with independent findings;
they do not claim whole-document validity or a governing record.

**L3-VIEW-017** · Parent: L2-VIEW-006 · Verification: T
*The view SHALL keep time data type format, time representation, source category and secondary-header representation distinct, and SHALL NOT use one field's code as another field's interpretation, a packet data-type byte, a time-channel ID, a correlation reset or an epoch/default clock.*

The four distinct Table 9-4 rows above are the citations; SHTF is on page
9-51. For example TTF=2 and TFMT=2 are different declarations. Reporting an
applicable TFMT default alongside a cross-field/network inconsistency never
overrides packet evidence (RELEASE-0.1); this batch does not define a new
packet mapping or inconsistencies not already reviewed.

**L3-VIEW-018** · Parent: L2-VIEW-006 · Verification: T
*Each time-field read SHALL compose L3-VIEW-005–011 without weakening multiplicity, character validity or unavailable-prerequisite handling, preserving Explicit, Defaulted, Missing, Invalid and Ambiguous evidence and any separate cannot-evaluate reason; no default SHALL replace a present invalid, blank-unavailable or duplicate value.*

General text projection remains INT-036 without byte edits; numeric
values instead use INT-055 raw-byte checks: every control is Invalid.
INT-037 high bytes remain Invalid; INT-039 duplicates remain Ambiguous.
INT-055 uniformly retains leading-blank unavailability for decimal-integer
values/numeric codes; trailing/embedded blanks are Invalid. Other letter-value
blank handling remains separate, without a sixth semantic field state. An explicit local TFMT on non-TIMEIN retains its field
result and applicability finding, not a false absence or confirmed TIMEIN.

**L3-VIEW-019** · Parent: L2-VIEW-006 · Verification: T
*Absent TFMT SHALL yield Defaulted A only with a supported reviewed Table 9-4 TIME FORMAT definition, identified edition/basis and established TIMEIN applicability; otherwise it SHALL preserve the absence/default-unavailability reason, and any default SHALL retain its exact edition/table/row citation without an input span or inserted bytes.*

Citation: 106-24R1 Table 9-4 TIME FORMAT, page 9-72; apply INT-041 and
L3-VIEW-008–011. An ambiguous CDT operand leaves applicability unknown;
Missing with that reason is not proof that the default is inapplicable.
Keep required-presence and other independent findings. No TSRC, TTF, SHTF,
UTC/IRIG-B, association or NSB default is invented by this TFMT rule.

**L3-VIEW-020** · Parent: L2-VIEW-006 · Verification: T
*The secondary-header time-format read SHALL address R-x\SHTF-n in each observed channel entry independently of whether its CDT is TIMEIN, preserve that field's own effective state and definition evidence, and SHALL NOT treat declaration presence as proof that a packet has a secondary header or supplies a clock association.*

Citation: Table 9-4 SECONDARY HEADER TIME FORMAT, page 9-51. A non-TIMEIN
channel can expose SHTF; an absent/unusable declaration stays absent/unusable,
not a value inferred from a nearby time channel or packet header.

**L3-VIEW-021** · Parent: L2-VIEW-007 · Verification: T
*Every field/candidate result SHALL retain the source document/revision, recorder/entry identity, original item spans and any supplied assembled-body-to-fragment mapping, together with edition-selection basis, default citation or limitation; invoking the view SHALL NOT change bytes, item ordering, multiplicity or findings, or silently reuse results on a different revision.*

For standalone TMATS, no packet provenance is fabricated. For assembled
input, use the existing provenance map rather than inventing a packet-local
span for a field crossing fragments. Rust ownership/types remain a later
reviewed API choice; revision identity and no stale interpretation are binding.

INT-056's clarified policy keeps findings readable in field results AND
source diagnostics, while Quiet suppresses exactly Warn's bad-data finding
set of any severity. Level, states and `suppressed: N bad-data findings`
(even 0) remain visible on every Quiet result. Final independent CONCUR at #52 3fcbd2c; development gates remain.

**L3-VIEW-022** · Parent: L2-VIEW-007 · Verification: T
*A view obtained with incomplete, retained-cut, ambiguous, unsupported-format or otherwise unestablished setup eligibility SHALL preserve that limitation and SHALL NOT present local metadata as an unqualified usable or governing time setup; no metadata result SHALL prove a data-channel/time-source association or select a governing candidate from competing records.*

This composes RELEASE-0.1 and approved INT-048's retained-cut blocking force;
it does not reopen retransmission decisions. Local diagnostic/raw inspection
is not semantic adoption. A supplied governing context is retained as such,
not independently selected or upgraded by a time-field read. The later
association/NSB and general governance derivations remain separate work.


## Source-bit batch 17: validated declaration and lossless numerical partition

These are future obligations, not a new implementation or passing tests.
The [batch note](research/2026-11-25-source-bit-requirements.md) records source
inspection, arithmetic examples and remaining DSI/CHE boundaries. This slice
splits a supplied identity; it does not decide which channels to enumerate.
Owner-signed INT-055 supplies the numeric field-value expectations, including
R\N condition operands and numeric-looking enumeration codes. The former
pending-owner fixture blocker is removed and final ADR-0033 concurrence
is recorded at #51 ebd5c98; source freshness, development analysis and
executable tests still gate implementation. INT-054 similarly records CHE context without silent filtering.

**L3-VIEW-023** · Parent: L2-VIEW-008 · Verification: T
*The source-bit declaration SHALL bind to the same document/revision and canonical recorder x as the supplied entry, use only the recorder-level R-x\NSB scalar and its own reviewed definition/applicability evidence, and SHALL NOT be borrowed from another recorder, record, revision, entry-suffixed lookalike or unrelated raw attribute.*

Citation: 106-24R1 Table 9-4 NUMBER OF SOURCE BITS, page 9-33; its condition
uses specified R\ID. Apply existing scoped conditions/INT-042 and report
unknown applicability rather than assuming true. INT-036/038 name projection
retains source spellings and findings; it does not change NSB value bytes.

**L3-VIEW-024** · Parent: L2-VIEW-008 · Verification: T
*A declaration SHALL supply a source-bit count only when its field result is Explicit, locally valid in the supported reviewed 0–13 domain and applicable in the supplied scope; out-of-range, malformed, blank-unavailable, ambiguous or unsupported-definition evidence SHALL block derivation with its own raw candidates/reason, without clipping, wrapping, trimming or selecting one occurrence.*

Citation: Table 9-4 NUMBER OF SOURCE BITS, page 9-33. Generic effective states
and INT-041 remain authoritative; lack of a rule is not proved Invalid data.
Identical NSB repeats still have INT-039 field ambiguity and INT-048 assembly
consequences. The baseline definition is not silently borrowed for other editions.

**L3-VIEW-025** · Parent: L2-VIEW-008 · Verification: T
*The full channel-ID operand SHALL be the usable actual channel ID of the supplied recorder/entry, retaining its field state and provenance; entry index n, an already split local ID or an inferred packet value SHALL NOT substitute for missing, invalid or ambiguous R-x\TK1-n evidence.*

Citation: Table 9-4 TRACK NUMBER/CHANNEL ID, page 9-48, range 1–65535;
Chapter 11 section 11.2.1.1 b, page 11-5, supplies the 16-bit field width.
This does not parse packet headers or make reserved full ID zero a valid
TK1 value. Derived zero-valued components are separate numerical results.

**L3-VIEW-026** · Parent: L2-VIEW-008 · Verification: T
*For valid explicit N and full 16-bit numeric channel ID C, the result SHALL use source width N and local width W=16-N, with source ID floor(C/2^W) and local ID C modulo 2^W, preserving C; N=0 SHALL yield source ID 0 and local ID C, and calculation SHALL NOT overflow, wrap or perform an invalid native-width shift.*

Citation: Chapter 11 section 11.2.1.1 b(1), page 11-5, specifies high bits
for source and remaining low bits for local channel; Table 9-4 NSB p. 9-33
bounds N. Zero components, including local ID 0 for a nonzero full ID, are
not revalidated as standalone TK1 fields or reclassified as reserved full
channel zero. This fixes numeric meaning, not a public Rust integer type.

**L3-VIEW-027** · Parent: L2-VIEW-008 · Verification: T
*With no usable explicit declaration or full-ID operand, the view SHALL retain the unsplit full-ID evidence and the precise blocking state/reason without fabricated source/local components, and SHALL NOT interpret absent NSB as explicit zero, a defaulted count, or proof of a single source.*

The explicit-zero case is an actual declaration under RELEASE-0.1, not a
sentinel for unknown metadata. A caller assumption is not relabelled Explicit
TMATS; assumption APIs need their separate derivation. Keep every independent
required-presence, applicability, duplication and encoding finding. In the
reviewed baseline, absent NSB with specified R\ID SHALL also retain the
required-presence finding: Table 9-4 NUMBER OF SOURCE BITS p. 9-33 has
Required when: Allowed. A no-split result does not suppress that finding.

**L3-VIEW-028** · Parent: L2-VIEW-009 · Verification: T
*Each derived split SHALL retain the full C, source/local widths and values, original NSB and TK1 evidence, document/revision and recorder/entry scope, definition citation, edition-selection basis and any supplied fragment provenance, without editing source bytes or silently carrying the split to another revision or recorder.*

Standalone text acquires no packet provenance. A labelled fallback basis
stays labelled, and unknown supported-layout/definition knowledge blocks a
qualified split rather than being replaced by the baseline. No document or
source-layout cache may obscure which explicit declaration supplied N.

**L3-VIEW-029** · Parent: L2-VIEW-009 · Verification: T
*Equal derived local IDs or source IDs SHALL NOT merge otherwise distinct full/scoped channel identities or establish a data-channel/time-channel association; source-bit results SHALL NOT select a time channel, assign TSRC/DSI semantics to numeric components, reset correlation or prove that the supplied setup governs.*

RELEASE-0.1 explicitly rejects association inference from multiplexer grouping.
This derives no DSI field reader or global channel-uniqueness rule. Assembly,
retained-cut and governing-context limitations remain visible under batch 16;
a locally calculable partition alone does not make an unusable setup usable.

**L3-VIEW-030** · Parent: L2-VIEW-009 · Verification: T
*Verification SHALL include focused parsed-input fixtures plus a bounded generated check of every valid full ID 1–65535 with explicit N=0–13 against an independently specified 16-character bit-string partition oracle; it SHALL assert width/range bounds, exact reconstruction of C and unchanged raw identity/provenance without deriving expected values from the production split routine.*

The finite arithmetic product is 917,490 cases, streamed rather than retained
as a large matrix. Exercise the real reviewed numerical derivation, with
focused field/scope cases separately covering parse and qualifier behavior;
exhaustive arithmetic alone cannot verify that binding. Run substantive
verification in CI after implementation exists. No such runtime check is
implemented or claimed by this documentation batch.


## DSI-context batch 18: entry-scoped source evidence

Future obligations only. See the [batch note](research/2026-11-25-dsi-context-requirements.md)
for source inspection, numeric fixture dependencies and the subsequently owner-signed CHE policy.
DSI context reads take canonical recorder/entry scope, not a parsed TK1 ID;
no new numeric parser, field syntax, clock link or stable Rust API is defined.

INT-055 covers R-x\N, NUMBER OF TRACKS/CHANNELS (106-24R1 Chapter 9
section 9.5.4 Table 9-4 p. 9-32, range 1–65536), whose R\N > 0 condition
is used by TK1/CDT/SHTF/DSI/CHE (pp. 9-48–51). Tests may now specify parsed
operand expectations as well as isolated supplied-condition units; neither
may assume an absent/unknown condition true. Numeric codes TTF/SHTF/TFMT
(pp. 9-51, 9-71–72) accept 02 as 2 with a warning under this owner decision,
not by borrowing index syntax. Final independent AI CONCUR is recorded at #51 ebd5c98.

**L3-VIEW-031** · Parent: L2-VIEW-010 · Verification: T
*The context read SHALL bind to the supplied document/revision and canonical recorder/entry pair (x,n) for R-x\DSI-n, preserving raw code-name spellings and index-normalisation evidence; recorder-level R-x\ID, G\DSI-n, another entry's DSI or a matching TK1/DSI value SHALL NOT substitute for that native scoped field.*

Citation: 106-24R1 Table 9-4 DATA SOURCE ID p. 9-49. INT-036/038 govern
code-name projection/identity, not arbitrary numeric field-value syntax.
The operation requires no value-based channel lookup; incomplete actual-ID
interpretation remains a separate limitation, not a fabricated ID.

**L3-VIEW-032** · Parent: L2-VIEW-010 · Verification: T
*DSI context SHALL compose the existing scalar effective-read contract, retaining every matching occurrence and independent finding; duplicates, even equal, SHALL stay Ambiguous with INT-048 assembly consequences, and missing or unavailable DSI evidence SHALL NOT be filled from recorder, time-source or neighbouring-channel values.*

Use L2-VIEW-001–004 and approved INT-039/041. Physical absence differs from
present empty text (INT-035). No new DSI default is introduced. Keep
required-presence/applicability findings where the reviewed rule establishes
them; this context read does not run or replace a complete link validator.

**L3-VIEW-033** · Parent: L2-VIEW-010 · Verification: T
*Raw DSI context SHALL remain available with original value bytes/location and the existing strict-character projection/effective-state evidence; a semantic string SHALL be exposed as usable only under its own supported reviewed definition, and unknown blank/length/definition prerequisites SHALL report unavailability rather than trimming, truncating, guessing a rule or manufacturing Explicit/Invalid.*

Citation: Table 9-4 DATA SOURCE ID p. 9-49; approved INT-036/037/041 and
Chapter 9 section 9.4.1–2. This batch does not select a new length-counting
or link-value blank policy. Labelled raw/display views are not a usable
semantic value; controls, high-byte findings and raw text remain preserved.

**L3-VIEW-034** · Parent: L2-VIEW-010 · Verification: T
*DSI context SHALL NOT be interpreted as a channel ID, multiplexer source-bit count/component, TSRC category, time-format code or clock-selection reference, even if its text resembles such a value; equal DSI strings across entries SHALL retain independent scopes without proving a data-channel/time-channel association or authorising a merge.*

RELEASE-0.1 explicitly rejects clock association from equal DSI, recorder
occurrence or multiplexer grouping. Keep any independently reported key/link
findings; this is not a claim that arbitrary duplicates conform to the
standard. General link relationships remain their separate derivation.

**L3-VIEW-035** · Parent: L2-VIEW-010 · Verification: T
*DSI results SHALL retain source document/revision, recorder/entry identity, raw item locations and any supplied fragment provenance, edition/definition basis and limitations without rewriting source bytes, normalising display text, reordering/dropping candidates or silently carrying values to another revision.*

Standalone data acquires no packet locations. Supplied incomplete,
retained-cut, ambiguous or unsupported setup context retains batch 16's
qualification and blocking force; a plausible source label does not make
such input usable or select which record governs.

**L3-VIEW-036** · Parent: L2-VIEW-010 · Verification: T
*The DSI context read SHALL report its supplied entry's field evidence independently of a complete clock-association/link traversal result and SHALL NOT choose a referenced time entry, infer source continuity or select a governing setup because a DSI field is present, missing, duplicated or locally usable.*

The standard row's Links from/to remain cited evidence, not an implemented
time-reference algorithm. This requirement defines a context operation on
an already supplied entry; it specifies no CHE-dependent enumeration or
filter policy and no unreviewed numeric field conversion.

## Owner-signed CHE and numeric syntax (family 2, batch 19)

INT-054/055 carry owner sign-off, Joey, 2026-11-25, Option A. INT-054/055 have
final AI CONCUR at #51 ebd5c98. Implementation/development gates remain in force.

**L3-VIEW-037** · Parent: L2-VIEW-011 · Verification: T
*A decimal-integer field-value or numeric-code read SHALL check the raw value bytes for 1–10 ASCII digits before zero-stripping; a leading blank SHALL be unavailable uniformly under INT-041, trailing/embedded blanks SHALL be Invalid, and any raw control, non-digit, sign, empty or longer value SHALL be Invalid without clipping, wrapping, repair or INT-036 omission into usable numeric data.*

INT-055 is owner policy, not a ten-digit limit printed in Chapter 9.
Keep independent character findings and proved Invalid failures visible;
INT-037 high bytes remain Invalid. Eleven zeros fail before canonicalisation.
Binary/bit-pattern/mask, floating-point, hexadecimal and every other
non-decimal representation are excluded, including numeric-only spellings.
Numeric codes require an explicit reviewed enumeration table, never a binary
pattern; see the complete exclusion inventory/Range audit.
Letter enum codes and G\SHA structured syntax keep separate rules.

**L3-VIEW-038** · Parent: L2-VIEW-011 · Verification: T
*A syntactically accepted numeric spelling SHALL compare by its canonical digit string with leading zeros stripped (all zero becomes 0), emit a located warning for each noncanonical spelling, retain raw bytes and warnings through later range/membership failures, and check the applicable cited range or enumeration before yielding Explicit.*

Thus NSB 04 is 4 and numeric enum 02 matches 2 with warning, not by repair.
No host parser may accept extra syntax; use u64 or canonical-string
comparison. u32 is insufficient for ten digits (maximum 9,999,999,999);
no unbounded integer conversion is needed. Field ranges remain Table 9-4 NSB 0–13, TK1 1–65535, R\N 1–65536
(pp. 9-33, 9-48, 9-32); unsupported definitions stay unavailable.

**L3-VIEW-039** · Parent: L2-VIEW-011 · Verification: T
*A condition using a numeric field operand SHALL consume that scoped effective result under INT-055; missing, invalid, ambiguous or unavailable evidence SHALL remain cannot-evaluate with its reason, never an assumed zero/positive value or selected duplicate, while raw located binding remains independently inspectable.*

R\N > 0 applicability for TK1/CDT/SHTF/DSI/CHE uses Table 9-4 pp. 9-48–51.
Known valid R\N=01 yields 1 plus warning; 0 is out of range, not a valid
false condition. A supplied evaluated unit predicate is not parser coverage.

**L3-VIEW-040** · Parent: L2-VIEW-012 · Verification: T
*A confirmed TIMEIN entry SHALL remain enumerated when its own CHE is Explicit F, with that disabled context visible; Explicit T or absent/unusable CHE SHALL NOT silently filter it or alter its CDT classification, scoped identity or other metadata.*

Source: 106-24R1 Table 9-4 CHANNEL ENABLE p. 9-49, T True / F False;
owner policy INT-054. Explicit caller filtering is a separate operation.

**L3-VIEW-041** · Parent: L2-VIEW-012 · Verification: T
*The CHE context SHALL preserve Missing with absence/default evidence, Ambiguous with every duplicate even when equal, demonstrated Invalid with raw evidence, or an INT-041 unavailable reason, alongside independently established required-presence/applicability findings and INT-048 assembly consequences.*

No missing/ambiguous/invalid declaration is silently enabled or disabled.
Required when Allowed depends on evaluated R\N > 0; unknown is not true.

**L3-VIEW-042** · Parent: L2-VIEW-012 · Verification: T
*CHE enumeration/context and any explicit caller filter SHALL preserve original bytes, occurrence order, document/revision, entry scope, locations and setup qualifications, without making incomplete, retained-cut or ambiguous input governing or inferring packet-generation eligibility from enumeration.*

This composes batch 16 provenance/qualification and INT-054, not a new
streaming/adoption rule or permission to override disabled packet findings.

## Association uncertainty and caller assumptions (family 2, batch 20)

Future obligations derived from RELEASE-0.1's owner direction, 2026-11-21.
They specify evidence retention/composition, not a TMATS association field,
consumer clock-selection algorithm, Rust API or implementation readiness.
Owner-signed INT-056–060 settle the caller policies; INT-056–060 have final
independent CONCUR at #52 3fcbd2c. See the [batch note](research/2026-11-26-association-assumption-requirements.md).

**L3-VIEW-043** · Parent: L2-VIEW-013 · Verification: T
*An association query SHALL retain the supplied data-channel identity, its recorder/entry scope where established, setup document/revision and supplied governing-context basis or limitation, without inventing scope, borrowing another record's TIMEIN entries or using entry index n as an actual TK1 channel ID.*

Use the existing scoped metadata and actual-ID results, not a new ID parser.
INT-055 governs decimal operands; missing/ambiguous/unavailable input identity
remains a query limitation with all candidates, not a guessed channel or an
unqualified empty result. This operation does not choose which setup governs.

**L3-VIEW-044** · Parent: L2-VIEW-013 · Verification: T
*The association result SHALL retain its association_unestablished, association_conflicting or association_evidenced meaning and supporting reason separately from each field's Explicit/Defaulted/Missing/Invalid/Ambiguous or unavailable evidence and from L1-VIEW-006 lookup/link states; an unusable field or several possible TIMEIN entries SHALL NOT alone establish or resolve an association.*

The semantic names come from RELEASE-0.1; they do not freeze Rust symbols.
Absent adequate evidence gives unestablished. Conflicting requires evidence
of competing associations, not arbitrary classification from candidate count.
INT-059 reserves conflicting for competing evidence; several unproven
candidates are unestablished. INT-058 requires each future evidence rule
to have its own approved INT; no 0.1 evidence rule exists.

**L3-VIEW-045** · Parent: L2-VIEW-013 · Verification: T
*In the absence of adequate reviewed association evidence, zero, one or several possible TIMEIN entries SHALL leave the association unestablished with the relevant candidates/reason retained; first/lowest/nearest/only/enabled occurrence, equal DSI labels, shared recorder or NSB components, TSRC categories and explicit/defaulted format values SHALL NOT select or prove an association.*

Sources: 106-24R1 Chapter 9 Table 9-4 NSB p. 9-33; TK1 p. 9-48;
DSI/CHE p. 9-49; CDT p. 9-50; SHTF p. 9-51; TTF/TFMT/TSRC
pp. 9-71–72, plus the owner's RELEASE-0.1 no-inference direction.
CHE F remains enumerated under INT-054. Caller filtering cannot turn one
remaining candidate into proof or erase the underlying uncertainty/evidence.

**L3-VIEW-046** · Parent: L2-VIEW-013 · Verification: T
*An evidenced association SHALL require evidence identifying its origin, applicable recording/setup document/revision and channel identities, and an individually owner-approved evidence INT defining the reviewed semantics establishing that relationship; unavailable, unreviewed, stale, unknown vendor/extension or merely suggestively named attributes SHALL remain raw/context evidence rather than proof.*

INT-058 requires an owner-signed INT for EVERY future evidence source,
including later-edition fields or packet-level rules, plus ADR-0033 review
and development gates. There is no 0.1 association-evidence rule. Order,
only-candidate status, TSRC, DSI, NSB and observed time packets are NEVER
evidence. RELEASE-0.1 allows future reviewed semantics but defines none.
This requirement is a proof-admission guard, not an implemented resolver or
permission to treat a synthetic fixture as standard-defined positive proof.
No affirmative baseline association-proof fixture is invented in this batch.

**L3-VIEW-047** · Parent: L2-VIEW-013 · Verification: T
*Association candidates/results SHALL retain every relevant occurrence, original diagnostic evidence and field-state/default/edition evidence, source locations and supplied fragment mapping, and setup eligibility limitations; neither a plausible clock match nor a caller selection SHALL discard ambiguity, retained-cut blocking, invalid metadata or source uncertainty; retained input diagnostics SHALL remain separate from bad-data finding emission on an assumption result under INT-056.*

INT-039/048 scalar/assembly ambiguity and INT-041 unavailable reads remain
separate. Source-bit splitting, CHE context and DSI reads are context only.
Standalone TMATS gains no fabricated packet provenance. A candidate result
must not assert a usable governing setup from diagnostic/raw inspection.

**L3-VIEW-048** · Parent: L2-VIEW-014 · Verification: T
*An assumption-bearing answer SHALL require an explicit caller choice and retain its supplied basis, selected target identity and input setup/data-channel scope distinctly from standards evidence, without silently choosing the first candidate, inferring missing choice/basis or relabelling the underlying unestablished/conflicting association as evidenced.*

This is a semantic provenance requirement, not a new public input struct or
validation grammar for an assumption label. A missing/ambiguous target keeps
its limitation; choosing an actual ID cannot silently pick among its scopes.

**L3-VIEW-049** · Parent: L2-VIEW-014 · Verification: T
*Every time-view answer derived using a caller assumption SHALL carry that dependency's choice, basis, selected level and source/setup/revision identities, including any copied, selected or rendered result produced by this view; a separately obtained unassumed read SHALL NOT silently inherit a prior query's assumption.*

Only time-view outputs are in this slice; this does not define JSON/CLI
schemas or the consumer's timestamp/correlation computation. Consumers must
retain the assumption on their own further derived answers per L1-VIEW-008.
INT-060 requires an explicit choice on EVERY query: no remembered or
session-wide default, even for the same setup/revision. Default Warn is
only a level default and never supplies a target.

**L3-VIEW-050** · Parent: L2-VIEW-014 · Verification: T
*A caller assumption SHALL preserve the underlying candidate set, association state/reason and original input diagnostics, applying INT-056 bad-data finding emission for its selected level while keeping field states unchanged, and SHALL NOT upgrade an invalid, unavailable, ambiguous or cut result, select a governing setup, override packet evidence or establish a time-correlation/reset policy.*

An isolated unit fixture may supply a preclassified conflicting association
and test provenance composition; that is not coverage of a future evidence
classifier or an actual TMATS proof path. Proposed general governance and
ambiguous/provisional acceptance interpretations remain outside this slice.

**L3-VIEW-051** · Parent: L2-VIEW-014 · Verification: T
*Association queries and assumption-bearing reads SHALL preserve raw bytes, item ordering/multiplicity and revision identity, and SHALL NOT silently carry an association, selected target or assumption into a different recording, setup record or revision without new scope-valid evidence or an explicit newly scoped caller choice that remains labelled as an assumption.*

A changed record is not made equivalent by a shared channel ID or DSI label.
Earlier answers keep their original scope/basis; a new query does not rewrite
historical provenance. Full timelines remain 0.2 and are not constructed here.


## Owner-signed caller-choice policies (INT-056–060)

Joey, 2026-11-26; final independent CONCUR under ADR-0033 on #52
head 3fcbd2c for INT-056–060. Development/source/test gates remain. The policies
are owner decisions, not a new standard field. They do not waive development,
source freshness or executable-test gates.

**L3-VIEW-052** · Parent: L2-VIEW-014 · Verification: T
*For an eligible selected target with Invalid, duplicate or unavailable ID/format data, Strict SHALL refuse with a typed bad-data reason; Warn SHALL be the default level and accept as a labelled caller assumption with all bad-data findings attached; Quiet SHALL accept as a labelled caller assumption while suppressing exactly the bad-data findings Warn would attach, of any severity, leaving unrelated findings unaffected by this policy.*

INT-056 separates acceptance of a choice from field validity. Each bad-data
kind has one named fixture per level, with ID and format arms. Typed reason
categories are semantic obligations, not frozen Rust enum names.

**L3-VIEW-053** · Parent: L2-VIEW-014 · Verification: T
*Every accepted bad-data choice SHALL retain its assumption label, selected level and target's original Invalid/Ambiguous/unavailable states and reasons; Quiet SHALL leave the suppressed findings readable from the target's field results and source diagnostics without upgrading states or association evidence. Every Quiet result SHALL carry `suppressed: N bad-data findings` (including N=0), counting exactly Warn's bad-data findings suppressed on that result, alongside the recorded level and assumption label where accepted.*

INT-056 suppression is local to the assumption result, not a severity filter
or a mutation of input/field diagnostics. Information/warning/error findings
are all suppressed if and only if they belong to Warn's bad-data set.
The marker is mandatory even with a clean target/zero suppressed findings.
Dependent results retain it; every level keeps setup/retained-cut and
governing limitations, and suppression never hides an independent typed refusal.

**L3-VIEW-054** · Parent: L2-VIEW-014 · Verification: T
*A choice outside the existing scoped TIMEIN candidates of the data channel's recorder SHALL be refused with a typed eligibility reason at every level, including non-TIMEIN entries, another recorder or a nonexistent ID; a valid scoped entry selection with bad ID/format SHALL instead follow INT-056 without fabricating a usable ID.*

INT-057 permits addressing an existing candidate by canonical entry identity;
a missing/ambiguous ID-only lookup cannot silently choose or create one.
Same numeric IDs on different recorders do not satisfy the scope guard.

**L3-VIEW-055** · Parent: L2-VIEW-014 · Verification: T
*A choice SHALL be refused as contradicting evidence ONLY when a separate owner-approved evidence INT establishes a different association for that data channel in the applicable scope; no heuristic, unreviewed rule or target-data defect SHALL trigger this refusal, and Strict/Warn/Quiet SHALL NOT bypass a genuine established contradiction.*

INT-058 defines the admission/refusal gate, not proof. No evidence source
exists in 0.1, so this branch is not currently triggerable. Every future
source needs its own owner-signed INT describing exactly what it proves,
independent check and readiness gates. Observed time packets are not evidence.

**L3-VIEW-056** · Parent: L2-VIEW-013 · Verification: T
*Several unproven candidates SHALL yield association_unestablished, while association_conflicting SHALL be reserved for competing evidence admitted through its own approved evidence rules; candidate count, metadata similarity or heuristic packet observations SHALL NOT substitute for such evidence.*

INT-059 preserves the difference between possible matches and proved claims.
A supplied preclassified conflict in a unit tests composition only; it must
not be presented as an actual 0.1 evidence-admission path.

**L3-VIEW-057** · Parent: L2-VIEW-014 · Verification: T
*Every query using a caller choice SHALL carry that choice explicitly, with no remembered/session-wide default or implicit reuse even in the same setup/revision; an omitted level SHALL use Warn, while an omitted target SHALL never be filled from an earlier query or from the Warn level default.*

INT-060 and INT-056 distinguish explicit choice from default warning level.
Dependent answers from that query still retain its provenance; a new query
needs its own explicit choice even when it selects the same target.

## Byte-faithful output and normalization boundary (family 2, batch 21)

These refine the batch-1 replay rule L3-WRT-001, not a new writer or edit
implementation. Accepted ADR-0002/0003/0010 and L1-WRT-001/002 govern;
see the [batch note](research/2026-11-26-lossless-output-requirements.md).
No normalizer grammar/default, public Rust signature or implicit repair is
approved. Existing reader/interpretation states do not authorize byte loss.

**L3-WRT-004** · Parent: L2-WRT-003 · Verification: T
*Successful unedited default output SHALL equal the original document source sequence byte-for-byte and in full, including source gaps, delimiters, layout/padding, empty values, duplicate/unknown items and retained malformed or unterminated regions, without rebuilding output from indexed keys, projected characters or interpreted values.*

The source is the document's retained buffer, not the caller's later-mutated
input allocation. A vector comparison is the oracle; matching only a hash,
item multiset or typed model cannot prove byte identity. This does not
require the parser to return a document for every unsupported input.

**L3-WRT-005** · Parent: L2-WRT-003 · Verification: T
*A retained document's unavailable/invalid field evidence, unsupported encoding/format qualification or incomplete/ambiguous setup qualification SHALL NOT cause the unedited output path to discard, replace or semantically re-encode its source; replay SHALL retain the qualification rather than present serialization as conformance, positive integrity or governing-setup evidence.*

INT-035–039/041/048/055 separate raw retention from interpretation. Copying
bytes does not validate an unknown field, repair a number or complete a
truncated setup. The requirement applies where a retained document exists;
it does not require constructing one after an input/format rejection.

**L3-WRT-006** · Parent: L2-WRT-003 · Verification: T
*Read-only operations, including exact/effective lookup, default resolution, edition fallback, field projection, validation and caller-assumption views, SHALL leave the source bytes used by default output unchanged; neither cached/canonicalized metadata nor suppressed result findings SHALL become the source for serialization.*

INT-056 Quiet affects its result's bad-data findings, not source storage.
An unedited zero-padded numeric value or defaulted TFMT is never rewritten
as canonical digits or an inserted attribute by ordinary output. An explicit
edit creates its own revision under the existing transaction contract.

**L3-WRT-007** · Parent: L2-WRT-003 · Verification: T
*Output adapters SHALL report success only after the entire original source sequence is delivered according to the caller-supplied writer contract; legal partial acceptance SHALL result in delivery of the remaining bytes in order or an explicit failure, never duplicated/skipped bytes, truncated success or a non-progressing unbounded retry.*

Concrete trait signatures and adapter implementation remain API review work.
If the supplied interface already guarantees complete writes, test its error
boundary without inventing a second short-write API. A mock sink with known
accepted prefixes provides an independent delivery oracle.

**L3-WRT-008** · Parent: L2-WRT-003 · Verification: T
*A supplied writer's failure SHALL remain an observable output failure with its cause retained rather than success or a fabricated complete buffer; the source document/revision and its findings SHALL remain unchanged, and any bytes already accepted by the caller's sink SHALL NOT be represented as rolled back by the library.*

This refines L3-WRT-001's existing fallible-writer error requirement. A later
retry is a caller choice with an appropriate sink, not an automatic append
that hides a partial first attempt. It creates no promise of atomic files,
flush/durability, filesystem rollback or automatic retry policy.

**L3-WRT-009** · Parent: L2-WRT-003 · Verification: T
*The serialization operation SHALL use only the supplied document and output destination, without selecting files, standard streams, environment configuration, network endpoints or packet destinations; emitting a document source SHALL NOT add or rebuild setup CSDWs, packet headers, trailers or recording framing.*

ADR-0010 and L2-IO-001 define the I/O boundary. For an assembled document,
the retained TMATS body is the source; fragment provenance is metadata, not
bytes automatically reinserted around output. Payload generation and packet
writing retain their own release scope and component ownership.

**L3-WRT-010** · Parent: L2-WRT-004 · Verification: T
*Without a distinct explicit caller normalization request, output SHALL remain default replay even when supported metadata suggests canonical case/order, consistent newlines, corrected counters or other repairs; findings, defaults, view/render modes or a checksum verification request SHALL NOT count as authorization to apply them.*

The gate follows L1-WRT-002, not a new implicit repair heuristic. Explicit
checksum stamping remains its existing separate transaction and must never
be silently invoked by default serialization. No normalization transform's
actual byte order/case/counter rules are selected by this gate.

**L3-WRT-011** · Parent: L2-WRT-004 · Verification: T
*A caller's explicit normalization request SHALL remain a separate operation with its requested transformation scope identifiable; it SHALL NOT silently broaden into unrequested repairs or be reported as default byte-faithful output, and an unreviewed/unsupported transform SHALL NOT be applied by guessing its semantics.*

This specifies authorization/scope separation, not a complete normalizer
implementation or a particular failure enum/API. Transformation-specific
rules, source citations and acceptance fixtures remain required in a later
family-2 slice. No absent policy is chosen by “canonical” as an output label.

## Core-backed setup slicing (family 2, batch 22; review correction)

ADR-0036 re-points the former reader requirements to core. These IDs now
specify integration with header-only PacketFacts, PacketInfo sideband and borrowed DATA spans, not a second
reader/checksum implementation. Source facts/examples are retained in the
[original note](research/2026-11-26-packet-slicing-requirements.md) and
[correction](research/2026-11-26-core-reader-span-rework.md).

**L3-CLI-004** · Parent: L2-CLI-003 · Verification: T
*The CLI SHALL use irig106-core for packet finding and primary-header trust/checksum results, consuming header-only PacketFacts (irig106-types), borrowed DATA spans and core-supplied integrity/trust evidence (PacketInfo sideband: secondary_integrity per RDG-008, data-checksum state, admission evidence, ranges) carried ALONGSIDE PacketFacts, without a local sync scanner, header-checksum implementation, packet traversal fallback or duplicate reader.*

The owner one-reader architecture supersedes the temporary ADR-0019 reader.
The CLI owns acquisition/integration; the TMATS library performs no I/O.
Unavailable core capability is a dependency gap, not permission for a fallback.

**L3-CLI-005** · Parent: L2-CLI-003 · Verification: T
*TMATS SHALL forward core's secondary-header integrity evidence unchanged, including RDG-008 byte/word readings and core's qualifications/verdicts, without calculating, choosing, rejecting or upgrading one reading through a local secondary-checksum rule.*

The Ch11 printed byte-sum and reference examples remain source evidence,
not a tmats acceptance test or algorithm. Core owns RDG-008 resolution;
copying references/scalar metadata is not a new interpretation of its verdict.

**L3-CLI-006** · Parent: L2-CLI-003 · Verification: T
*The adapter SHALL consume core's already identified DATA extents as borrowed span sequences and check only its own logical span/slice bounds with checked arithmetic, without re-parsing raw headers to reconstruct packet, secondary-header, filler or checksum boundaries.*

Local CSDW/text access must not wrap, clamp, overrun a supplied span or invent
missing bytes. Such span-contract checks are not a duplicate core header/
profile validator. Preserve core facts/source identity with each span sequence.

**L3-CLI-007** · Parent: L2-CLI-003 · Verification: T
*Packet-profile, version, packet-size and trust facts SHALL come from core and remain qualified as core supplied them, with no tmats reimplementation of packet caps/version tables or fallback inference from G\106/RCCVER.*

Ch11 Table 11-3 source limits (134,217,728-byte setup packet; 524,288-byte
ordinary packet) remain reference facts. A 256-span maximum-setup fixture
is coverage, not a new 256-span production cap or maximum assembled-record
size. No dependency version supporting uninspected APIs is assumed here.

**L3-CLI-008** · Parent: L2-CLI-004 · Verification: T
*For core-provided Format-1 DATA with at least four logical bytes, TMATS SHALL read logical [0,4) as CSDW and [4,D) as text over the borrowed spans, even when either region crosses block boundaries; shorter logical DATA SHALL not fabricate a CSDW/text value, and D=4 SHALL produce only an empty text range, not a completeness claim.*

Source: Chapter 11 section 11.2.7.2/Figure 11-34 p. 11-50. The four-byte
prefix is removed ONCE per packet, not once per span/block. Other packet
types do not acquire Format-1 CSDW interpretation by resembling text.

**L3-CLI-009** · Parent: L2-CLI-004 · Verification: T
*TMATS SHALL NOT verify packet data checksums and SHALL pass core's data-integrity evidence through as-is, including NotChecked until core provides verification, without converting it to checked, failed, absent or verified based on local checksum arithmetic.*

Coordinator decision 2; ADR-0036. Ch11's body-plus-filler checksum facts
remain a CORE feature request, whose implementation timing the owner decides.
G\SHA/flex over TMATS is separate and unchanged; it does not supply missing
packet integrity evidence. No optional local checksum fallback is authorised.

**L3-CLI-010** · Parent: L2-CLI-004 · Verification: T
*Core-supplied integrity/trust evidence (PacketInfo sideband: secondary_integrity per RDG-008, data-checksum state, admission evidence, ranges), carried ALONGSIDE header-only PacketFacts, SHALL retain its original values, readings and qualifications through CSDW/text slicing and assembly; logical byte access SHALL NOT promote NotChecked or uncertain/failed evidence to successful verification or infer time closure/governing authority from it.*

PacketFacts (irig106-types) contains HEADER FACTS ONLY, no trust/integrity
fields; do not add fields or silently embed sideband state there. Use core's
PacketInfo evidence, not a tmats verdict assembled from header subfields. This
selects no failed-payload adoption, report severity/format or RDG-008 rule.
Core DATA excludes filler/checksum; TMATS neither rechecks nor adds them.

**L3-CLI-011** · Parent: L2-CLI-004 · Verification: T
*The adapter and assembler SHALL retain the ordered source-span references and PacketFacts header metadata, DATA-derived CSDW and PacketInfo sideband/provenance evidence for every logical text range, splitting mappings at block/span as well as packet-fragment boundaries and preserving exact origin/lifetimes without a contiguous payload intermediate.*

An input owner/borrow must keep core blocks alive while spans are used; no
block recycling or dangling/rebound slice is permitted. Logical adjacency
does not mean adjacent backing allocations. Public lifetime/API signatures
remain reviewed implementation work, not a new local PacketFacts type.

**L3-CLI-012** · Parent: L2-CLI-004 · Verification: T
*The TMATS assembler SHALL accept non-contiguous DATA spans directly and retain logical per-channel text concatenation without required coalescing or extra payload byte copy; block splits SHALL NOT be interpreted as packet boundaries, new CSDWs, delimiters, join stops or complete records.*

Coordinator decision 3. Scalar CSDW decoding and bounded span/provenance
metadata are not payload coalescing. Explicit owned standalone/document
materialization remains a separate path, never required just to slice or
assemble span-backed input. INT-011/045 logical packet boundaries remain.

**L3-CLI-013** · Parent: L2-CLI-004 · Verification: T
*Integration verification SHALL include CSDW-straddling and text-straddling block cases, unchanged core evidence including RDG-008 readings/NotChecked, and a 256-span maximum-setup-packet case, asserting exact logical bytes/provenance and zero extra payload copying without implementing core checksum/reader algorithms as test oracles.*

Large backing belongs to bounded GitHub CI, with no 128-MiB local allocation.
Use arithmetic/descriptor checks locally and core fixtures or explicit facts
for integration. Metadata allocation is not hidden payload coalescing.


## Content selection and acquisition boundary (family 2, batch 23)

Derived from accepted ADR-0019's retained input selector, ADR-0036's
core-only reader and ADR-0003/0010. Source facts, exclusions and independent
oracles are in the [batch note](research/2026-11-26-content-routing-requirements.md).
These are planned contracts, not implemented CLI coverage.

**L3-CLI-014** · Parent: L2-CLI-005 · Verification: T
*On successfully acquired source bytes, the selector SHALL choose recording exactly when logical source offsets [0,2) equal 25 EB (hex); otherwise it SHALL choose standalone TMATS text, without scanning later offsets or interpreting headers to make that selection.*

Chapter 11 (106-24R1) section 11.2.1.1 a p. 11-5 supplies 0xEB25;
section 11.1 p. 11-1 supplies structural little-endian order. ADR-0019
supplies the routing policy. This two-byte selector is not core packet finding.

**L3-CLI-015** · Parent: L2-CLI-005 · Verification: T
*A successfully acquired empty or one-byte source SHALL take the standalone-text route because it lacks the complete prefix, without padding, reading beyond bounds or treating this route as a successful parse or an empty-document result.*

A lone 25 or EB is insufficient. Any reader framing/encoding findings and
failure remain that reader's result; selection does not repair partial input.

**L3-CLI-016** · Parent: L2-CLI-005 · Verification: T
*Renaming the same source, changing its extension or placing sync-like bytes later in it SHALL NOT alter the content-selected route; path metadata SHALL NOT substitute for the initial byte predicate.*

A .tmats file with the prefix selects recording; a .ch10 file without it
selects text. Handling embedded recordings or damaged prefixes would need
separate reviewed recovery behavior, not an implicit search here.

**L3-CLI-017** · Parent: L2-CLI-006 · Verification: T
*Recording-selected input SHALL go to core's packet reader and the existing TMATS integration boundary, preserving core-supplied integrity/trust sideband alongside header-only PacketFacts and borrowed DATA spans, without a local packet scanner, header/checksum validator or mandatory DATA coalescing.*

L3-CLI-004–013 govern the adapter. PacketInfo sideband carries integrity,
admission and ranges; PacketFacts itself gains no trust fields. Unavailable
core capability remains a dependency gap, not permission for another reader.

**L3-CLI-018** · Parent: L2-CLI-006 · Verification: T
*Standalone-text-selected input SHALL reach the TMATS reader with its original byte sequence and source mapping unchanged, without implicit UTF-8 decoding, lossy/Latin-1 conversion, BOM stripping, newline conversion or byte removal in the router.*

ADR-0003 and INT-035–038 govern source preservation and interpreted views.
Raw routing does not decide whether bytes yield usable interpreted fields.
L1-CLI-007's prohibition on modifying input files remains applicable.

**L3-CLI-019** · Parent: L2-CLI-006 · Verification: T
*An acquisition failure SHALL remain distinguishable from successfully acquired empty input, and downstream failure or unavailable trust SHALL remain distinguishable from route selection; neither SHALL manufacture a successful document, trigger an automatic retry as the other input mode or upgrade setup usability/governance.*

The initial predicate in ADR-0019 does not authorize a parse-failure fallback.
No new recovery/adoption policy, public error spelling or exit-code mapping
is selected. Existing completeness, integrity and governing gates still apply.

**L3-CLI-020** · Parent: L2-CLI-006 · Verification: T
*Acquisition SHALL remain at the CLI/caller integration boundary, with the TMATS library I/O-free; routing SHALL preserve the source prefix for the selected consumer rather than consume/drop it, and SHALL keep the source alive for borrowed access under the existing ownership contract.*

ADR-0010 and ADR-0036. The source owner supplies bytes; routing does not
create a new file reader in the TMATS library or require a full-file copy.
This does not select a stdin convention, argument syntax or streaming API.

**L3-CLI-021** · Parent: L2-CLI-005 · Verification: T
*Input-routing verification SHALL use independently specified prefix cases and observable consumer/acquisition boundaries, asserting the selected route, unchanged forwarded bytes and absence of the other consumer on downstream failure, without implementing a second packet reader as its oracle.*

Use bounded synthetic buffers and injected consumer outcomes in CI. Compare
the selected route with literal source-derived cases, not the production
selector called again to calculate expectations. Core integration fixtures
remain separate from this selector's tests.


## CLI mounting and orchestration boundary (family 2, batch 24)

These refine the existing runner/individual-API contracts rather than
select command arguments, report fields or numeric process statuses.
See the [authority and fixture note](research/2026-11-26-cli-mounting-requirements.md).
Packet ownership/evidence requirements in L3-CLI-004–013 remain unchanged.

**L3-CLI-022** · Parent: L2-CLI-001 · Verification: T
*The host-callable runner SHALL use the supplied invocation arguments and writable streams rather than implicitly replacing them with process arguments or process stdout/stderr; successful and unsuccessful invocations SHALL return to the host without terminating its process.*

L3-CLI-001 already establishes `run(args, stdout, stderr) -> ExitClass`.
Mounting preserves that contract; the host owns its top-level command
namespace and process status. This selects no argv representation or prefix
stripping convention and no rendered output or exit-code mapping.

**L3-CLI-023** · Parent: L2-CLI-001 · Verification: T
*The standalone entry point and mounted whole-runner path SHALL share the public runner's command execution and rendering behavior, without independent copies of command logic; process-status translation SHALL remain at the standalone binary or mounting host's outer process boundary.*

ADR-0034 W3, L3-CLI-002. A host must be able to keep executing after the
runner returns failure. Future comparison checks use the same supplied
command inputs, not assumptions about unreviewed CLI syntax or output bytes.

**L3-CLI-024** · Parent: L2-CLI-002 · Verification: T
*Individual command APIs SHALL be callable without entering `main`, invoking the whole runner or duplicating command computation in the host; their returned report data SHALL remain separate from rendering to supplied writers.*

L3-CLI-003 already requires reports rather than direct printing. Verify a
real implemented command through both mounting paths once its own contract
is reviewed; a help-only scaffold is not evidence for all commands.
Command-specific report types and public signatures remain future work.

**L3-CLI-025** · Parent: L2-CLI-002 · Verification: T
*Calling a renderer through the reusable API SHALL write through the caller-supplied writer rather than acquire process output streams, and SHALL preserve output failure under the reviewed writer contract without terminating the host or claiming successful delivery.*

ADR-0010, L3-CLI-003 and L2-WRT-003 govern the boundary. This does not pick
text/JSON formats, finding order, assumption labels, suppression presentation
or numeric exit mappings. A failing supplied writer exercises delivery only;
report/output policy still requires the owner's review before derivation.

**L3-CLI-026** · Parent: L2-CLI-007 · Verification: T
*For a mounting host's packet pipeline, the host SHALL perform data-type dispatch from core-supplied packet facts and route setup fragments to the TMATS adapter; that adapter's Format-1 check SHALL be only an input-kind guard, not a reader, dispatcher, packet-type inference or raw-header reparse.*

ADR-0030/0036 and the post-#55 clarification. Content selection for an
entire source (L3-CLI-014–021) is a separate CLI boundary, not something
repeated for each already identified packet. No type is guessed from DATA.

**L3-CLI-027** · Parent: L2-CLI-007 · Verification: T
*If a non-Format-1 packet is incorrectly supplied to the fragment adapter, the guard SHALL return an observable, typed wrong-input-kind refusal to the calling host, carrying the supplied data type, and SHALL prevent Format-1 CSDW/text slicing and appending from that call; it SHALL NOT return silent success or a silent no-op, guess another packet type, read another packet or promote any core trust/integrity state.*

L3-CLI-008 already prevents other types from acquiring Format-1 semantics.
Format 1 names the packet data type, not a requirement that the CSDW FRMT
bit be zero. Passing that guard does not establish supported text encoding;
the separate FRMT/ASCII/XML rules remain in force.
The typed refusal exposes the wrong-input-kind reason and supplied data type
without selecting a public variant/field spelling, rendered format or recovery
policy. Header-only PacketFacts remains separate from PacketInfo
trust/integrity evidence;
matching a type alone does not establish admission, completeness or governance.

**L3-CLI-028** · Parent: L2-CLI-007 · Verification: T
*Host orchestration SHALL retain and supply the non-fragment packet observations and caller provenance needed by INT-011/045 separately from Format-1 fragment slicing; dispatch to another consumer SHALL NOT erase a same-channel stop, an applicable time-boundary observation, other-channel provenance or the qualifications on that evidence.*

INT-011 joins per channel; INT-045 adds layered closure. A time observation
may affect open TMATS candidates even when its payload goes to the time
consumer. Preserve core's trust and edition qualifications and caller-
asserted session start. No automatic session inference, new time recognizer,
new stop rule or provisional/governing-use policy is selected here.

**L3-CLI-029** · Parent: L2-CLI-007 · Verification: T
*The mounting boundary SHALL retain core-supplied header facts, separate integrity/trust sideband, source identity and ordered borrowed DATA spans through TMATS slicing/assembly, without requiring a second traversal, a full-source copy or payload coalescing merely to mount the CLI library.*

ADR-0036 and L3-CLI-006/010–013 retain ownership and no-extra-copy duties.
This does not choose an exact cross-crate API or require core to depend on
TMATS. Published types/core 0.2 compatibility remains a release prerequisite,
not a claim that dependency APIs were inspected or exercised by this batch.


## Byte-exact extraction and safe destination boundary (family 2, batch 25)

Derived from L1-CLI-004/007, ADR-0002/0003/0010/0036 and L2-WRT-003.
The [batch note](research/2026-11-26-extraction-requirements.md) distinguishes
mechanical byte extraction from pending record-selection and output policies.

**Implementation BLOCKED on owner decisions**: `-o` overwrite policy;
fail-closed identity-check policy for pipes/devices/stdout/network shares;
and extraction eligibility under integrity/encoding findings. These are
pending choices, not approved defaults; source/development/test gates remain.

**L3-CLI-030** · Parent: L2-CLI-008 · Verification: T
*For one explicitly selected eligible setup record, extraction SHALL deliver its original logical TMATS text in fragment order, excluding each packet's four-byte CSDW and all packet headers, filler and packet checksums; physical DATA-span/block boundaries SHALL NOT insert, remove or duplicate output bytes.*

Chapter 11 (106-24R1) section 11.2.7.2/Figure 11-34 p. 11-50 supplies
CSDW-before-text framing. Core supplies DATA extents excluding trailers;
L3-CLI-006/008 determine logical slicing. Extraction does not reconstruct
packet boundaries or repeat CSDW removal once per physical span.

**L3-CLI-031** · Parent: L2-CLI-008 · Verification: T
*Extraction SHALL preserve the selected record's text bytes exactly, including layout, case, unknown/vendor/extension attributes, duplicates, high/control bytes and preserved suffix/padding where present in that record's original text; read-only interpretation or findings SHALL NOT cause byte filtering, text conversion, normalization, repair or checksum stamping; INT-036/037 projections and INT-055 raw-byte numeric checks SHALL be VIEW-ONLY and SHALL NOT change extracted bytes.*

ADR-0002/0003 and L3-WRT-004–010 govern replay. Raw preservation does not
make any field or setup usable; test byte preservation separately from
eligibility. Fixtures with malformed content exercise the mechanical sink
boundary without asserting that those records are CLI-eligible. Controls
omitted from interpreted views, numeric Invalid verdicts and a BOM detected
after fragment assembly never authorize changing the raw extraction bytes.
Extraction eligibility under those findings awaits the owner decision.

**L3-CLI-032** · Parent: L2-CLI-008 · Verification: T
*Extraction SHALL receive and retain the explicit selected record/source identity and its established eligibility basis, without silently substituting the first, latest or governing record, merging distinct records, or treating absent/unresolved selection as a successfully extracted empty record.*

This pins an internal precondition, not the public record-selection rule.
Ordinal ordering, incomplete/ambiguous access and multi-record destination
behavior remain for their own reviewed derivations. A failure to establish
that precondition cannot be repaired by choosing another record implicitly.

**L3-CLI-033** · Parent: L2-CLI-008 · Verification: T
*Extraction delivery SHALL use the existing fallible byte-writer contract, preserving byte order across partial writes and exposing sink failure rather than claiming complete extraction; a failure SHALL leave source bytes/provenance unchanged and SHALL NOT trigger an automatic edit or second source selection.*

L2-WRT-003 and L3-WRT-007/008 supply the delivery/failure contract,
including progress and retained failure causes. A partial external sink is
not a modified source. No new atomic file protocol, rollback, cleanup policy,
flush/durability guarantee or error rendering is introduced by this
requirement.

**L3-CLI-034** · Parent: L2-CLI-009 · Verification: T
*For a regular-file destination the tool opens itself, CLI filesystem glue SHALL open without truncating, compare the opened handle's file identity with the input-file identities, and only then, if distinct and authorized by the reviewed overwrite policy, truncate through that SAME handle; it SHALL NOT close and reopen by pathname between the identity check and truncation. A known input alias SHALL be refused observably before destructive access, and inability to establish input preservation SHALL NOT authorize truncation.*

L1-CLI-007 applies to same-path, relative/absolute aliases, symbolic links
and hard links where supported. String inequality is insufficient. The
implementation must review platform file-identity and acquisition/race
behavior. A pathname preflight check alone is insufficient; comparison must
use the non-truncating opened destination handle before truncation/writing.
Exact platform APIs remain implementation analysis. The owner decisions on
`-o` overwrite and fail-closed identity checks for pipes/devices/stdout/network
shares BLOCK implementation; no such destination policy, public error
spelling or numeric status is selected here.

**L3-CLI-035** · Parent: L2-CLI-009 · Verification: T
*For destinations the tool opens itself, successful or failed CLI extraction and other output-producing commands SHALL leave input-file contents unchanged, and SHALL NOT acquire input files for in-place writing, truncation, replacement or implicit repair as part of producing output; shell redirection performed outside the tool SHALL be out of scope for this file-acquisition guarantee.*

ADR-0010 keeps this at the CLI/caller boundary; the TMATS library still
receives bytes/writers rather than filenames. The prohibition also covers
alias destinations and writer/acquisition failures for tool-opened outputs.
The tool cannot undo truncation already performed by a shell before it runs.
Delivery to supplied writers remains fallible under L3-WRT-007/008; this
does not settle the pending stdout/other-destination identity policy.
It is not an approval of pending general edit commands or a packet-writing
implementation.

**L3-CLI-036** · Parent: L2-CLI-009 · Verification: T
*Destination acquisition failure or input-alias refusal SHALL remain observable to the caller without being reported as completed output, silently redirecting to another destination or modifying an input to recover.*

L1-CLI-007 and the existing failure boundary motivate this observable
contract, not a new report schema, filesystem overwrite mode or fallback
stdout policy. A host-callable invocation still returns to its host under
L3-CLI-001/022; numeric process-status mapping remains separate.

**L3-CLI-037** · Parent: L2-CLI-008 · Verification: T
*Extraction verification SHALL compare delivered bytes to an independent literal source/fragment oracle and verify source immutability on success and failure, including core DATA-span partitions, without deriving expected bytes from the production extraction/writer path or adding a packet reader/checksum implementation to the oracle.*

Use bounded synthetic spans and injected sink outcomes for mechanical tests;
use isolated temporary files for CLI identity/acquisition integration. Actual
record selection/eligibility requires its own approved fixtures. No local
large-record scan or payload allocation is required by this docs batch.
