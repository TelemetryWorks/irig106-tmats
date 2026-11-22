# irig106-tmats — Level 3 Requirements

> **Status: foundations, framing and character batch drafted (family 2, batches 1-3).**
> These are implementation obligations, not existing interfaces or completed
> tests. INT-035 is approved; INT-036/037 were approved by Joey, 2026-11-22. Remaining derivations
> and development evidence still precede implementation.

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
