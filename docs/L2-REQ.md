# irig106-tmats — Level 2 Requirements

> **Status: foundations through counter batches drafted (family 2, batches 1-6).**
> Foundations derive the approved restructure/source store; framing and
> recovery use owner-approved INT-035. Character policies INT-036/037 were
> approved by Joey, 2026-11-22, under ADR-0033. Name structure follows here;
> INT-038/039 were approved by Joey, 2026-11-22. Counter batch 6 proposes
> INT-040; remaining view/registry, CLI and later areas follow;
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

INT-035 is **approved** by Joey, 2026-11-21, with independent AI concurrence
referenced to PR #13 under ADR-0033. Approval does not complete the analysis,
character/layout prerequisites or executable verification. Full code-name grammar, nonprintable-byte
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

**Rationale**: Owner-approved INT-035 reconciles the valid-input delimiter syntax (106-24R1 Chapter 9 section 9.4.2, page 9-2) with the owner-directed lossless recovery in L1-READ-001/003. It deliberately does not claim to reconstruct the writer's intended configuration. Owner sign-off and the PR #13 independent AI concurrence are recorded; implementation evidence remains pending.

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


#### L2-READ-006

**Parent**: L1-READ-003

**Statement**: The reader SHALL emit one information-level located finding for each maximal consecutive run of empty or layout-only terminated regions, merging the run into one source span; unterminated layout or padding at EOF SHALL remain silent, and all bytes SHALL remain retained.

**Rationale**: Joey's 2026-11-21 sign-off on INT-035 settles the open diagnostic policy under ADR-0033. EOF silence follows the owner-directed treatment of Chapter 9 section 9.4.1, page 9-2. This is product recovery policy, not a new rule that the standard mandates an informational diagnostic.

**Verification Method**: Test (T)


## Character batch 3: interpreted bytes and source identity

INT-036 and INT-037 were approved by Joey, 2026-11-22, under ADR-0033,
with the independent AI recheck concurring on PR #16 head `891f77f` (see the
[sign-off record](research/2026-11-22-int036-int037-owner-signoff.md)). They do
not amend the approved INT-035 framing/run policy or supply implementation,
completed analysis or tests. Name structure is derived in batch 4; duplicate
classification and full known-code validation remain subsequent work.

#### L2-READ-007

**Parent**: L1-READ-001

**Statement**: The reader SHALL retain original source bytes while obtaining interpreted field bytes through a separate projection, preserving the association of every interpreted field and character finding with original source ranges; discarding characters for interpretation SHALL NOT discard them from storage or default output, and human-display renderers SHALL use raw text rather than the filtered semantic projection.

**Rationale**: ADR-0002/0003 and approved INT-036 reconcile 106-24R1 Chapter 9 section 9.4.1, page 9-2, with lossless retention. This is not a normalization edit, and semantic projection does not change the actual colon/semicolon framing of INT-035.

**Verification Method**: Test (T)

#### L2-READ-008

**Parent**: L1-READ-006

**Statement**: Code-name interpretation SHALL ignore the approved nonprintable set and surrounding ordinary blanks without removing embedded ordinary blanks; value interpretation SHALL retain ordinary blanks; controls omitted between printable bytes inside a code name SHALL produce a located information finding, NUL SHALL produce one located information finding per affected field, and the ASCII comparator SHALL operate only on the interpreted input supplied to it without trimming source itself.

**Rationale**: Approved INT-036 supplies the character-class details for section 9.4.1; section 9.4.2, page 9-2, treats blanks as intentional. L1-READ-006 already requires surrounding code-name blanks to be ignored. Internal blanks still need code-name-validity findings; they are not silently erased to repair a key.

**Verification Method**: Test (T)

#### L2-READ-009

**Parent**: L1-READ-003

**Statement**: Non-ASCII field bytes and supported detections of incompatible encoding markers SHALL produce located findings while original bytes remain available; the reader SHALL NOT silently strip a BOM, transcode input, guess UTF-16 from zero-byte patterns or treat failed character interpretation as an explicit usable field value.

**Rationale**: Chapter 9 section 9.4.1 specifies 7-bit ASCII. Approved INT-037 defines the limited marker policy, not a Unicode TMATS format. Ordinary unmarked raw input can remain partly interpretable; a detected leading unsupported marker blocks semantic lookup for that document as specified in the approved interpretation, without dropping its source. Marker detection uses assembled-text offset zero before padding/control omission; fields containing high bytes are Invalid for semantic use, while ADR-0003's explicitly labelled on-request Latin-1/lossy views remain available.

**Verification Method**: Test (T)


## Code-name batch 4: structure before semantic validation

This batch derives L1-READ-002/005 and the code-name-diagnostic part of
L1-READ-003. INT-035/036/037 remain approved and unchanged; INT-038 was
approved by Joey, 2026-11-22, under ADR-0033, with the independent AI recheck
concurring on PR #20 head `29707da` (see the
[sign-off record](research/2026-11-22-int038-owner-signoff.md)). Approval does
not supply implementation, full development analysis or executable tests.
The examples establish structural obligations, not a completed catalogue,
registry matcher, counter validator, duplicate classifier or extension resolver.

#### L2-READ-010

**Parent**: L1-READ-002

**Statement**: Code-name interpretation SHALL preserve the group, any supplied occurrence index, ordered path segments and every index attached to each segment, with original-source correspondence; literal digits in a code element SHALL NOT be mistaken for indices, and absent components SHALL NOT be invented; approved INT-038 identity equality and hashing SHALL use canonical decimal strings, including when X carried indices are compared with the original indices.

**Rationale**: 106-24R1 Chapter 9 section 9.4.2, pages 9-2/3, and section 9.5.1 a with Table 9-1, page 9-5, establish hierarchical code names and multiple entries. Table 9-7 MEASUREMENT NAME `D-x\MN-y-n`, page 9-115, demonstrates nested indices; NUMBER OF FRAGMENTS `D-x\MNF\N-y-n-m`, page 9-117, demonstrates multiple path levels. Approved INT-038 specifies index spelling/identity; neither syntax nor a parsed number proves validity in a counter's scope.

**Verification Method**: Test (T)

#### L2-READ-011

**Parent**: L1-READ-005

**Statement**: The reader SHALL recognize the baseline Chapter 9 group identifiers and standalone COMMENT code independently of whether an individual name matches a known definition, retaining unknown names and H/V/X material without claiming that recognition establishes conformance or extension association.

**Rationale**: 106-24R1 Table 9-1, page 9-5, enumerates the thirteen identifiers; section 9.4.2, page 9-3, defines COMMENT. Sections 9.5.12/13, page 9-171, and 9.5.14, page 9-172, reserve H and define vendor/extension conventions that a restrictive generic grammar must not erase. Detailed extension behavior remains a later derivation under L1-EXT; ADR-0002 still retains everything.

**Verification Method**: Test (T)

#### L2-READ-012

**Parent**: L1-READ-003

**Statement**: Code-name interpretation failures SHALL produce located diagnostics while preserving the framed item's raw name, value and order; incomplete or failed structural matches SHALL NOT become usable partial keys, and absence of a known definition alone SHALL NOT be equated with malformed code-name syntax; approved INT-038 noncanonical index spellings and normalization-only full-key collisions SHALL produce located information findings during reading, without selecting or rewriting an item; the collision finding SHALL trigger for a canonical-key group with two or more distinct raw index-spelling tuples and list every occurrence in that group, exact repeats included.

**Rationale**: L1-READ-001/003, ADR-0002/0003 and INT-035 retain source even when meaning fails. Chapter 9 section 9.4.2, page 9-2, advises against blanks in code names; approved INT-036 ignores only surrounding blanks and controls for meaning. Thus a damaged `G\COM Time 12:30;` cannot be silently accepted as `G\COM`. Unknown definitions and open H/V/X naming space require a distinct outcome from a demonstrated syntax error.

**Verification Method**: Test (T)


## Candidate batch 5: duplicate classification and exact lookup

This derives the duplicate part of L1-READ-003, exact-name lookup under
L1-VIEW-001 and the multiplicity part of L1-VIEW-004. INT-039 was approved
by Joey, 2026-11-22, under ADR-0033, with independent AI concurrence on
PR #22 head `b22d45c` (see the
[sign-off record](research/2026-11-22-int039-owner-signoff.md)). INT-035/036/037/038
remain approved unchanged. Full development analysis and executable tests
remain outstanding; approval does not provide implementation evidence.
Pattern-query grammar, complete effective-value resolution, link selection, time association, counter rules and edits remain
subsequent derivations; these children do not complete those L1 parents.

#### L2-READ-013

**Parent**: L1-READ-003

**Statement**: The reader SHALL classify repeated instances of a definition-bound scalar attribute by complete canonical key, report the duplicate with every occurrence located, and preserve all items; unknown cardinality and standalone COMMENT collections SHALL NOT be silently treated as duplicate scalar definitions, nor SHALL duplicate classification suppress approved normalization findings.

**Rationale**: 106-24R1 Chapter 9 section 9.4.2, page 9-2, describes unique names and single/multiple-entry attributes; page 9-3 introduces COMMENT for interjected comments. Approved INT-039 settles repeat handling where that text does not specify recovery. Scalar cardinality must come from a cited, reviewed definition for the applicable edition, not a missing/assumed suffix. Group \COM fields remain single-entry per the standard (Table 9-2 G\COM, page 9-13; Table 9-4 R-x\COM, page 9-84), unlike standalone COMMENT. The owner knowingly retained duplicate handling for writers that repeat long remarks; raw lookup keeps every line. ADR-0002 retains source; INT-038's early collision finding remains distinct.

**Verification Method**: Test (T)

#### L2-VIEW-001

**Parent**: L1-VIEW-001

**Statement**: Exact-name lookup SHALL return every item matching the complete canonical key within one document, in input order, with item identity, original name/value bytes and locations; it SHALL distinguish no match from a present empty value and SHALL NOT discard candidates because their values fail interpretation.

**Rationale**: ADR-0002/0003 keep source items authoritative; approved INT-036/038 define projection and key equality. The first-child scope is exact lookup only, not a pattern language or an implementation choice of index data structure. Chapter 9 section 9.4.2, page 9-2, permits any source order; preserving that order for reporting must not make it a winner-selection rule.

**Verification Method**: Test (T)

#### L2-VIEW-002

**Parent**: L1-VIEW-004

**Statement**: A singular effective-value request for a definition-bound scalar slot with two or more matching items SHALL return ambiguous with every original candidate and its findings, without collapsing equal values, filtering invalid candidates, falling back to a default or selecting by source order; one candidate SHALL still require ordinary validity checks before being called explicit.

**Rationale**: ADR-0023 supplies explicit/defaulted/missing/invalid/ambiguous states. Approved INT-039 chooses conservative multiplicity even for equal repeats; this is not a new default or an assertion that Chapter 9 explicitly specifies resolver precedence. Full zero/one-candidate resolution and scope-dependent defaults are later work. Coordinator-decided F1 keeps duplicate SHA integrity indeterminate under ADR-0034, independently of generic view states.

**Verification Method**: Test (T)


## Counter batch 6: declared scope and distinct entry identities

This batch derives the counter portion of L1-REG-005, L1-VAL-002 and
L1-VAL-006, following ADR-0023/0026 and approved INT-038/039. Proposed
INT-040 addresses membership when entry values are invalid or duplicated;
it needs owner sign-off and independent AI review under ADR-0033. These are
requirements for the existing planned validation scope, not a promotion of
the general registry/validator into 0.1. Link metadata, complete rule/type
interpretations and other family-2 areas remain separate derivations.

#### L2-REG-001

**Parent**: L1-REG-005

**Statement**: Each counter rule SHALL identify its edition/citation, counter pattern, governed member patterns and index position, fixed parent-index tuple and contiguity policy or cited exception; applicability and membership SHALL come from a reviewed declaration rather than a global suffix or group-name heuristic.

**Rationale**: 106-24R1 Chapter 9 section 9.5.1 a after Table 9-1, page 9-5, supplies the 1-to-N rule; ADR-0026 requires parent scope. Table 9-7 NUMBER OF MEASUREMENT LOCATIONS `D-x\MML\N-y-n` and NUMBER OF FRAGMENTS `D-x\MNF\N-y-n-m`, page 9-117, illustrate different governed positions. Section 9.5.14, page 9-172, exempts X occurrence indices from contiguity, not every nested index in an extension. No counter is created merely because a path ends in N.

**Verification Method**: Test (T)

#### L2-VAL-001

**Parent**: L1-VAL-002

**Statement**: For each applicable reviewed counter rule and fixed parent tuple, validation SHALL compare the usable declared count with DISTINCT canonical numeric identities of governed entries, checking both cardinality and the declared index-domain/contiguity rule while retaining all occurrence evidence; it SHALL NOT substitute raw item count, greatest observed index or another parent's entries for that set.

**Rationale**: ADR-0026 and approved INT-038 already require scoped distinct numeric identities. Proposed INT-040 counts structurally identified entries independently of payload validity and duplicate declarations; other findings remain visible. Chapter 9 section 9.5.1 a, page 9-5, requires no missing values, so equal cardinality alone is insufficient. A counter result is not whole-document conformance or permission to repair source. Explicit child scopes remain evaluable despite parent-domain violations or unusable ancestor counts; the child's own rule/count and membership prerequisites still apply, without an ancestor-validity gate. X attachment does not create D membership, and WFP value uniqueness remains a separate value rule.

**Verification Method**: Test (T)

#### L2-VAL-002

**Parent**: L1-VAL-006

**Statement**: Counter evaluation SHALL report its counter/indices pass, rule citation, parent scope and supporting locations, and SHALL report cannot-evaluate when applicability, the effective counter value or required membership cannot be established without guessing; it SHALL NOT infer a missing count or claim success from an incomplete member set.

**Rationale**: ADR-0022/0023 already require cannot-evaluate for unavailable operands. INT-039 makes duplicate scalar counters Ambiguous even when values agree. A cited default may be used only through a reviewed effective-value rule that permits it; neither suffix spelling nor the largest member supplies one. Proposed INT-040 preserves structural evidence separately from blocked or failing semantic checks.

**Verification Method**: Test (T)
