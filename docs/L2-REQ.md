# irig106-tmats — Level 2 Requirements

> **Status: foundation batch drafted for review (family 2, batch 1).**
> This batch derives the approved restructure and source-store foundations.
> Scanner grammar/recovery, remaining CLI behavior and later areas follow;
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

These obligations derive ADR-0002/0003/0010/0011 and coordinator-decided
ADR-0034 (owner may override). They select no new interpretation of the
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

**Statement**: The workspace SHALL retain the library package at the repository root and the CLI package in `irig106-tmats-cli/`, with one shared version, an exact matching CLI-to-library dependency, and no third crate or move to `crates/` for the restructure.

**Rationale**: ADR-0011 and coordinator-decided W1 (ADR-0034). Prototype retirement is W2: remove it in the restructure change, splitting oversized review work or returning to the owner. This does not authorize publishing placeholders as functional releases or remove the first-upload exception in RELEASING.md.

**Verification Method**: Test (T)

#### L2-CLI-001

**Parent**: L1-CLI-006

**Statement**: The CLI crate SHALL expose a host-callable entry point taking caller arguments and output streams and returning an exit classification, without terminating the host process; the binary SHALL delegate to it and translate the result to process status only at its outer boundary.

**Rationale**: Coordinator-decided W3 (ADR-0034). A host needs the same documented exit semantics as the binary without process ownership. Numeric exit mappings, command arguments and output schemas remain for the command-specific batch; this child alone does not cover L1-CLI-006.

**Verification Method**: Test (T)

#### L2-CLI-002

**Parent**: L1-CLI-002

**Statement**: The CLI library SHALL expose individual command and rendering APIs in addition to the shared runner; command computation SHALL return report data rather than print, and rendering SHALL write to caller-supplied streams.

**Rationale**: Coordinator-decided W3 and docs/CLI.md: the ecosystem host may mount the whole runner or reuse individual commands/renderers. Raw, tree and summary report behavior still needs command-specific derivations; this child establishes the reusable boundary only. L1-IO-001 also motivates keeping input acquisition at the CLI edge.

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
