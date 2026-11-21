# irig106-tmats — Level 2 Requirements

> **Status: foundation and scanner-framing batches drafted (family 2, batches 1-2).**
> Foundations derive the approved restructure/source store; framing and
> recovery add a proposed interpretation requiring owner sign-off. Remaining
> code-name, character, CLI and later-area derivations follow;
> a parent with children here is not necessarily fully decomposed yet.

## Purpose

This document establishes the Level 2 (L2) SHALL-statement requirements. L2
requirements are architectural derivations of the L1 requirements in
`L1-REQ.md`: they specify *how* each L1 obligation is structurally satisfied,
without yet prescribing implementation details (those belong to L3).

## Conventions

L2 identifiers follow the format `L2-<CATEGORY>-<NNN>`. Each L2 is a level-4
heading (`#### L2-XXX-NNN`) and declares exactly one parent on a line of the
form `**Parent**: L1-XXX-NNN`; when an L2 is motivated by several L1s, the
primary one is the parent and the others are named in the rationale. Metadata
fields (Statement, Rationale, Verification Method, and Evidence where the
method is not Test) carry the same meaning as in `L1-REQ.md`. Numbering is
monotone within each category; retired identifiers are never reused.

**Status and verification artifacts** are tracked in
[`docs/TRACE-MATRIX.md`](TRACE-MATRIX.md), regenerated from test markers and
parent links by `scripts/build-trace-matrix.py`. This file holds only spec
content.

---

## Foundation batch: restructure and source store

These obligations derive ADR-0002/0003/0010/0011, coordinator-decided
ADR-0034 and the owner's W1 override in ADR-0035. They select no new interpretation of the
standard. In particular, source retention is not a decision about malformed
attribute boundaries, nonprintable-character meaning or recovery grammar.
Those scanner rules need their own cited requirements and, where applicable,
interpretation drafts and owner sign-off under ADR-0033 before code.

The acceptance specifications are in [TEST-DATA.md](TEST-DATA.md#family-2-batch-1-foundation-acceptance-specifications).
They are planned checks, not executable tests or verification evidence.
L3 names below define the implementation-facing detail for this batch only.
Before implementing or accepting a touched L1 parent, complete its remaining
decomposition in the subsequent family-2 batches. Passing only the structural
children below cannot establish all show modes, JSON schema behavior, parser
recovery or whole-library robustness; do not use that partial rollup as a
release-acceptance claim.

### Workspace and CLI boundaries

#### L2-REL-001

**Parent**: L1-REL-001

**Statement**: The workspace SHALL have a virtual root manifest with the library in `crates/irig106-tmats/` and the CLI in `apps/irig106-tmats-cli/`, one shared real-release version in `[workspace.package]`, and an exact-version library path dependency declared in `[workspace.dependencies]` and inherited by the CLI.

**Rationale**: ADR-0011 and owner-decided ADR-0035, 2026-11-21, superseding W1 in ADR-0034. The reference model is Praevis-Quant/praevis-thetadata; the CLI is for testing and simple examples, with full 0.1 scope and W3 unchanged. Prototype retirement is W2: remove it in the restructure change, splitting oversized review work or returning to the owner. This does not authorize publishing placeholders as functional releases or remove the first-upload exception in RELEASING.md.

**Verification Method**: Test (T)

#### L2-CLI-001

**Parent**: L1-CLI-009

**Statement**: The CLI crate SHALL expose a host-callable entry point taking caller arguments and output streams and returning an exit classification, without terminating the host process; the binary SHALL delegate to it and translate the result to process status only at its outer boundary.

**Rationale**: Coordinator-decided W3 (ADR-0034). A host needs the same documented exit semantics as the binary without process ownership. Numeric exit mappings, command arguments and output schemas remain for the command-specific batch; L1-CLI-006 remains the separate output-schema/exit-code obligation.

**Verification Method**: Test (T)

#### L2-CLI-002

**Parent**: L1-CLI-009

**Statement**: The CLI library SHALL expose individual command and rendering APIs in addition to the shared runner; command computation SHALL return report data rather than print, and rendering SHALL write to caller-supplied streams.

**Rationale**: Coordinator-decided W3 and docs/CLI.md: the ecosystem host may mount the whole runner or reuse individual commands/renderers. Raw, tree and summary report behavior still needs command-specific derivations under L1-CLI-002; this child establishes the reusable boundary only. L1-IO-001 also motivates keeping input acquisition at the CLI edge.

**Verification Method**: Test (T)

#### L2-IO-001

**Parent**: L1-IO-001

**Statement**: The TMATS library SHALL accept source bytes from the caller and expose bytes, views or output through caller-supplied writers; opening files, reading process arguments, obtaining environment-based configuration, selecting standard streams and terminating a process SHALL remain outside the library.

**Rationale**: ADR-0010 and ADR-0034 separate the library, reusable CLI and thin binary. A supplied writer may perform I/O for its caller; that does not authorize the library to select files, stdout or stderr itself.

**Verification Method**: Inspection (I)

### Owned source and ordered items

#### L2-READ-001

**Parent**: L1-READ-001

**Statement**: A parsed document SHALL own one immutable original byte buffer; raw code-name and value access SHALL reference ranges in that buffer rather than require per-value strings or a caller-owned input buffer to remain alive.

**Rationale**: Accepted ADR-0003. Original byte retention implements the owner-directed lossless policy of ADR-0002, including non-UTF-8 input; it does not establish what those bytes mean or silently normalize them. Scanner outcomes must retain that source when they report malformed input (L1-READ-003).

**Verification Method**: Test (T)

#### L2-READ-002

**Parent**: L1-READ-001

**Statement**: The document SHALL retain an ordered item sequence referring to the source buffer; derived keys, indexes and typed views SHALL NOT replace that sequence or discard unknown, duplicate or malformed source material.

**Rationale**: ADR-0002 makes the sequence the source of truth. Classification and boundary rules are derived in the next scanner batch; this requirement does not prescribe a new delimiter-recovery heuristic. L1-READ-002/003 and L1-VIEW-001 will also depend on this foundation.

**Verification Method**: Test (T)

#### L2-WRT-001

**Parent**: L1-WRT-001

**Statement**: Default serialization of an unedited document SHALL emit its original byte buffer in full, including bytes outside successfully interpreted attributes; rebuilding output from typed fields or canonical keys SHALL NOT be the default write path.

**Rationale**: ADR-0002, supported by L1-WRT-002: explicit normalization is a separate request and later derivation. Identity applies even to retained malformed bytes; it does not assert standards compliance or make an incomplete setup record usable.

**Verification Method**: Test (T)

#### L2-ROB-001

**Parent**: L1-ROB-001

**Statement**: Source-span creation and access SHALL validate bounds with checked arithmetic, returning a non-panicking rejection for an invalid range instead of wrapping, indexing unchecked or truncating a range into a different one.

**Rationale**: Accepted ADR-0003 guards spans with newtypes and non-panicking access. This is a storage safety obligation, not approval of the still-proposed ADR-0020/0021 in full; scanner termination, input limits and the remaining robustness rules need subsequent derivations.

**Verification Method**: Test (T)


## Scanner batch 2: byte framing and conservative recovery

INT-035 is **proposed**, with owner sign-off and independent AI review
pending under ADR-0033. These are requirements for review, not permission
to implement that interpretation. Full code-name grammar, nonprintable-byte
meaning, character diagnostics, duplicate classification, counter/link rules
and suspect-semicolon suggestions remain separate obligations.

#### L2-READ-003

**Parent**: L1-READ-006

**Statement**: The byte scanner SHALL delimit an attribute at an actual semicolon and identify its code-name/value separator at the first colon before that semicolon; later colons SHALL remain value bytes, and the scanner SHALL preserve the original source and delimiter spans.

**Rationale**: 106-24R1 Chapter 9 section 9.4.2, page 9-2, defines colon then semicolon delimitation and disallows semicolons inside data items. L1-READ-006 and ADR-0024 require later colons to remain values. Malformed-region recovery is INT-035, not an invented delimiter or general code-name parser.

**Verification Method**: Test (T)

#### L2-READ-004

**Parent**: L1-READ-003

**Statement**: For a semicolon-bounded region missing its colon, an empty code name, or an unterminated final non-layout region, the reader SHALL retain the raw bytes and a located diagnostic, continuing after an actual semicolon where available; it SHALL NOT reinterpret an internal colon as an invented semicolon or silently return an empty successful document.

**Rationale**: Proposed INT-035 reconciles the valid-input delimiter syntax (106-24R1 Chapter 9 section 9.4.2, page 9-2) with the owner-directed lossless recovery in L1-READ-001/003. It deliberately does not claim to reconstruct the writer's intended configuration. Owner sign-off and independent AI review remain pending.

**Verification Method**: Test (T)

#### L2-READ-005

**Parent**: L1-READ-004

**Statement**: Comparison of already identified code-name, keyword and link byte sequences SHALL fold only ASCII letter case, without modifying the original bytes or using locale-dependent or Unicode case conversion; source order SHALL NOT decide their equivalence.

**Rationale**: 106-24R1 Chapter 9 section 9.4.2, page 9-2, specifies case-insensitive TMATS and free attribute order. L1-READ-004 narrows comparison to ASCII folding. This does not decide whether an unknown/non-ASCII sequence is a valid code, keyword or link, nor trim intentional value blanks.

**Verification Method**: Test (T)

#### L2-ROB-002

**Parent**: L1-ROB-001

**Statement**: The scanner SHALL make bounded forward progress on arbitrary input bytes: each scan step SHALL advance its source cursor or finish, and recovery SHALL NOT revisit the same boundary indefinitely.

**Rationale**: A malformed byte must not cause a loop. This derives the termination part of L1-ROB-001 for framing only; it does not select the proposed ADR-0021 indexing/checksum pipeline or establish whole-library robustness.

**Verification Method**: Test (T)
