# irig106-tmats — Level 3 Requirements

> **Status: foundation and scanner-framing batches drafted (family 2, batches 1-2).**
> These are implementation obligations, not existing interfaces or completed
> tests. INT-035 review and remaining L2/L3 derivations follow before implementation.

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
*The root manifest SHALL keep workspace members `.` and `irig106-tmats-cli`, declare the real-release version once in `workspace.package.version`, and have both packages inherit it; the CLI dependency SHALL use `path = ".."` and the exact `=X.Y.Z` version matching that shared value.*

The future manifest check reads the actual TOML structure, including the CLI
library and binary targets, rather than testing whether a version substring
happens to appear somewhere in the file. Fuzz remains outside the workspace.

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

The recovery entries below depend on proposed INT-035. Acceptance cases
are specified in TEST-DATA.md; none is an implemented or traced test yet.

**L3-READ-004** · Parent: L2-READ-003 · Verification: T
*For each semicolon-terminated candidate region, the scanner SHALL search for the first colon only within that region, excluding the semicolon, and retain separate source ranges for the complete region, code-name bytes, value bytes and both delimiters when present.*

A colon in a later region must not make an earlier missing-colon region
appear well formed. Component spans identify raw bytes, not a claim that
the code name or its value is semantically valid.

**L3-READ-005** · Parent: L2-READ-003 · Verification: T
*Bytes after the first colon and before the terminating semicolon SHALL remain the raw value, including later colons, spaces and line endings; neither code-like text nor quote characters SHALL create an alternate terminator or an implicit attribute split.*

A future suspected-semicolon finding under L1-READ-007 may suggest an edit;
it must not change framing or apply it. This does not add quoted strings or
an escape syntax to TMATS delimiters.

**L3-READ-006** · Parent: L2-READ-004 · Verification: T
*When a candidate region has a semicolon but no colon, the reader SHALL retain that region as uninterpreted source material with a diagnostic spanning it and resume at the byte after that semicolon; a nonempty final region not classified as permitted layout and lacking a semicolon SHALL be retained with a missing-terminator finding and SHALL NOT be fabricated into a terminated attribute.*

This is proposed INT-035. Permitted inter-attribute layout must be classified
by its own character rules before claiming a layout-only suffix is malformed.
Those rules and full malformed code-name diagnostics are not settled here.

**L3-READ-007** · Parent: L2-READ-004 · Verification: T
*The framing result SHALL carry both retained source material and framing diagnostics with byte ranges in that source; one malformed region SHALL NOT erase earlier or later retained material or replace its findings with empty success.*

For assembled setup records, later provenance requirements map these ranges
to packets. This entry does not declare an incomplete record usable.

**L3-READ-008** · Parent: L2-READ-005 · Verification: T
*ASCII comparison SHALL map byte values `0x61..=0x7A` to their corresponding `0x41..=0x5A` values for comparison only and leave every other byte unchanged; it SHALL NOT trim, decode/re-encode or alter the source as a side effect.*

Test mixed case, punctuation and high bytes as comparator inputs. Equality
of raw comparator inputs does not validate their attribute semantics.

**L3-ROB-003** · Parent: L2-ROB-002 · Verification: T
*The framing cursor SHALL remain within the source bounds, consume each discovered semicolon before the next region, and terminate at EOF; on an input of length N it SHALL finish after at most N+1 region steps, including empty-region handling, without allocating an N-sized fixture merely to test integer limits.*

The bound concerns region advancement, not an unapproved performance budget
or a claim about all subsequent parsing/validation work. Empty input and
runs consisting solely of semicolons must terminate without repeated state.
