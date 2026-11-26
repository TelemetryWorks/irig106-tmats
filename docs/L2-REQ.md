# irig106-tmats — Level 2 Requirements

> **Status: foundations through content-routing boundaries drafted (family 2, batches 1-23).**
> INT-056–060 are approved (Joey, 2026-11-26; final AI CONCUR at #52 3fcbd2c).
> INT-054/055 are approved (Joey, 2026-11-25; final AI CONCUR at #51 ebd5c98).
> INT-012 is reconfirmed; INT-044 Option B is approved (Joey, 2026-11-23; AI CONCURS on #36 at 6f6e080).
> INT-042 Option A was approved by Joey, 2026-11-23, with independent AI recheck concurrence.
> INT-018/019/020 are reconfirmed; INT-043 Option B is approved (Joey, 2026-11-23; AI CONCURS at #32 head 8941ca1).
> Foundations derive the approved restructure/source store; framing and
> recovery use owner-approved INT-035. Character policies INT-036/037 were
> approved by Joey, 2026-11-22, under ADR-0033. Name structure follows here;
> INT-038/039/040 were approved by Joey, 2026-11-22; INT-041 was approved
> by Joey, 2026-11-23; remaining registry/conditions, views, CLI and later areas follow;
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

**Rationale**: ADR-0023 supplies explicit/defaulted/missing/invalid/ambiguous states. Approved INT-039 chooses conservative multiplicity even for equal repeats; this is not a new default or an assertion that Chapter 9 explicitly specifies resolver precedence. Batch 7 derives the zero/one-candidate core; complete condition/type rules and scope-dependent definition coverage remain later work. Owner-approved F1 (Joey, 2026-11-23; INT-021, originally ADR-0034 coordinator direction) keeps duplicate SHA integrity indeterminate independently of generic view states.

**Verification Method**: Test (T)


## Counter batch 6: declared scope and distinct entry identities

This batch derives the counter portion of L1-REG-005, L1-VAL-002 and
L1-VAL-006, following ADR-0023/0026 and approved INT-038/039. INT-040 was
approved by Joey, 2026-11-22, under ADR-0033, with independent AI recheck
concurrence at PR #24 head `176fdf4` (see the
[sign-off record](research/2026-11-23-int040-owner-signoff.md)). Full analysis
and executable tests remain outstanding; approval is not implementation. These are
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

**Rationale**: ADR-0026 and approved INT-038 already require scoped distinct numeric identities. Approved INT-040 counts structurally identified entries independently of payload validity and duplicate declarations; other findings remain visible. Chapter 9 section 9.5.1 a, page 9-5, requires no missing values, so equal cardinality alone is insufficient. A counter result is not whole-document conformance or permission to repair source. Explicit child scopes remain evaluable despite parent-domain violations or unusable ancestor counts; the child's own rule/count and membership prerequisites still apply, without an ancestor-validity gate. X attachment does not create D membership, and WFP value uniqueness remains a separate value rule.

**Verification Method**: Test (T)

#### L2-VAL-002

**Parent**: L1-VAL-006

**Statement**: Counter evaluation SHALL report its counter/indices pass, rule citation, parent scope and supporting locations, and SHALL report cannot-evaluate when applicability, the effective counter value or required membership cannot be established without guessing; it SHALL NOT infer a missing count or claim success from an incomplete member set, and every scope compatible with unassigned evidence's readable canonical parent facts SHALL report partial/cannot-evaluate, never pass.

**Rationale**: ADR-0022/0023 already require cannot-evaluate for unavailable operands. INT-039 makes duplicate scalar counters Ambiguous even when values agree. A cited default may be used only through a reviewed effective-value rule that permits it; neither suffix spelling nor the largest member supplies one. Approved INT-040 preserves structural evidence separately from blocked or failing semantic checks.

**Verification Method**: Test (T)


## Effective-value batch 7: evidence before fallback

These requirements extend L1-VIEW-004's zero/one-candidate branches and the
source/default portion of L1-REG-002. ADR-0023 and RELEASE-0.1.md already
settle the five semantic states and non-inserting defaults. INT-041 was
approved by Joey, 2026-11-23, under ADR-0033, with independent AI concurrence
on PR #26 head `0fc0bc8` (see the
[sign-off record](research/2026-11-23-int041-owner-signoff.md)). INT-039/040
remain approved; full analysis and executable tests remain outstanding.
Edition selection, complete condition grammar, dynamic range rules, the
registry generator, presence/profile validation and time-channel APIs remain
separate derivations. General effective values/registry keep their existing
release scope; this batch does not promote them wholesale into 0.1.

#### L2-REG-002

**Parent**: L1-REG-002

**Statement**: Field/default interpretation SHALL retain the source text and edition/table/row citation, distinguish a Default-field origin from a prose origin, and distinguish reviewed absence of a default from unavailable interpretation; applying a default SHALL require a reviewed value, applicability rule and identified edition basis.

**Rationale**: 106-24R1 Chapter 9 section 9.5.1 a, page 9-6, defines defaults for absent attributes. ADR-0022 requires source and interpretation layers; ADR-0023 carries default evidence without inserting it. Table 9-4 TIME FORMAT, page 9-72, gives Default A with TIMEIN applicability; other editions or fields must not inherit it by analogy.

**Verification Method**: Test (T)

#### L2-VIEW-003

**Parent**: L1-VIEW-004

**Statement**: For a supported reviewed scalar definition and identified document/scope/basis, a singular read SHALL preserve INT-039 multiplicity, return Explicit only for one usable locally interpreted value, and return Invalid with raw evidence for one demonstrated character/type/range failure; present empty content SHALL NOT be treated as absence, and unavailable decision prerequisites SHALL be reported without guessing a semantic value state.

**Rationale**: Approved INT-035/036 retain present empty values; INT-037 makes high-byte fields Invalid; INT-039 preserves every scalar duplicate. ADR-0023 separates effective values from whole-document validation. Approved INT-041 distinguishes unavailable knowledge from proved invalid data, without choosing a Rust enum/wrapper or revising the five supported-field states.

**Verification Method**: Test (T)

#### L2-VIEW-004

**Parent**: L1-VIEW-004

**Statement**: With zero actual candidates for a supported field, resolution SHALL return Defaulted only from a verified applicable default and otherwise Missing with the absence/default-unavailability reason; default evidence SHALL include its origin, edition citation and basis without a fabricated input span, and resolution SHALL NOT insert or replace bytes or suppress independent findings.

**Rationale**: Chapter 9 section 9.5.1 a, page 9-6, conditions default use on absence; RELEASE-0.1.md requires verified TIMEIN applicability and edition basis for TFMT A. Invalid or ambiguous present fields do not become absence. ADR-0028 labels edition basis; missing physical required attributes, cross-field inconsistencies and packet evidence remain separate from a Defaulted value.

**Verification Method**: Test (T)


## Condition batch 8: declared operands and visible uncertainty

This derives condition metadata under L1-REG-004 and reporting under
L1-VAL-006. ADR-0022/0023 already require a shared evaluator and visible
cannot-evaluate outcomes. INT-042 Option A was approved by Joey, 2026-11-23,
under ADR-0033, with independent AI recheck concurrence on PR #28 head
`d69d275` (see the [sign-off record](research/2026-11-23-int042-owner-signoff.md)).
Existing INT-035 through INT-041 approvals are unchanged. Full development
analysis and executable tests remain outstanding; these are planned obligations,
not implemented checks.

Complete condition grammar, compound operators/precedence, cyclic default
or condition dependencies, link construction, edition selection and the full
presence/profile validator remain later derivations. This batch does not
promote general registry/validation into 0.1; the bounded time view consumes
only its reviewed applicable subset. See the
[draft source review](research/2026-11-23-condition-scope-draft.md).

#### L2-REG-003

**Parent**: L1-REG-004

**Statement**: Every executable condition SHALL bind its source/citation and reviewed interpretation to declared operand scopes, link traversal and default visibility, and SHALL distinguish a physical-presence predicate from a predicate requiring an interpreted value; unreviewed mappings SHALL remain unavailable rather than be inferred from a source-name abbreviation or another occurrence.

**Rationale**: ADR-0022 defines the shared condition contract. 106-24R1 Chapter 9 section 9.5.1 a, page 9-6, distinguishes Allowed/Required usage from defaults. Table 9-3 TRANSMIT ANTENNA TYPE T-x\AN1, page 9-18, uses an explicit T-x\ID presence condition; Table 9-4 TIME FORMAT R-x\TFMT-n, page 9-72, instead compares R\CDT with TIMEIN. Approved INT-042 makes presence/value operands distinct without treating a missing value operand as false. INT-008/009's existing abbreviated-reference interpretations keep their own pending review obligations.

**Verification Method**: Test (T)

#### L2-VAL-003

**Parent**: L1-VAL-006

**Statement**: Evaluating a reviewed condition SHALL produce true, false or cannot-evaluate with its rule, scope, operand evidence and edition basis; unavailable scope, link selection or required value operands, and zero exact presence matches with compatible related damaged evidence, SHALL produce cannot-evaluate with a separate pass-labelled finding, never a false-condition verdict, winner selection or silent default activation.

**Rationale**: L1-REG-004 and ADR-0022 already forbid true/false for missing or invalid value operands; INT-039/041 extend the visible guard to ambiguity and unavailable interpretation. Section 9.5.1 a, page 9-6, states: "If the attribute is specified in the TMATS file, it must contain valid information". This sentence sits within the R/R Ch 10 Status bullet: strong evidence separating physical specification from validity in that context, not a general definition of specified. Approved INT-042 Option A interprets presence separately. INT-042 permits proved absence to answer a reviewed physical-presence predicate, but zero exact matches alone cannot prove absence when related damaged evidence remains. A condition result is not whole-document validity, and a true presence predicate does not certify its present values.

**Verification Method**: Test (T)


## Registry batch 9: source provenance and interpretation gates

This batch derives L1-REG-001/003/006 from ADR-0004/0005/0022/0027 and
ADR-0033's owner-approved review rule. No new standard interpretation is
proposed. It specifies evidence obligations, not an existing registry,
extractor, hash implementation or CI job. INT-042's separately requested
matching detail is derived in L3-VAL-012, with its approved states unchanged.

A semantic approval does not imply completed development analysis or tests.
The requirements below keep those gates separate and keep source coverage
visible without manufacturing executable interpretations. Complete extraction,
relationship generation, edition deltas and condition grammar remain later
batches. General registry scope stays at its existing roadmap releases; only
the reviewed bounded time-view subset is brought forward to 0.1.
See the [source/gate derivation record](research/2026-11-23-registry-gates-draft.md).

#### L2-REG-004

**Parent**: L1-REG-001

**Statement**: Every built-in definition and executable rule SHALL retain an edition-specific source identity and section or table/row citation, with the source text supporting its keywords, ranges, conditions and defaults; source coverage SHALL distinguish an executable reviewed interpretation, a visibly not-yet-interpreted source entry and an explicit exclusion with its reason.

**Rationale**: ADR-0004/0022 require cited source and interpretation layers; Chapter 9 section 9.5.1 a, pages 9-5–9-6 (106-24R1), identifies usage fields, while definition prose is also authoritative. ADR-0022's completeness/coverage check must not silently drop rows or turn raw extraction into permission to execute a rule. Existing L2-REG-002 preserves default origin. Full table extraction and relationship completeness are not completed by this provenance child.

**Verification Method**: Test (T)

#### L2-REG-005

**Parent**: L1-REG-003

**Statement**: Each executable registry interpretation SHALL bind its draft author, dated owner sign-off, independent AI review evidence and source-text hash to the exact source and interpretation revision reviewed; a source change SHALL flag re-review and SHALL NOT inherit executable approval merely because its code-name pattern or interpretation identifier is unchanged.

**Rationale**: ADR-0033 supersedes only the two-human rule; L1-REG-003 and ADR-0022 retain source-hash freshness. A semantic decision's date and an exact-head AI-check reference are evidence, not completion of implementation analysis/tests. Source changes are not erased by recalculating a digest or carrying forward an approved label.

**Verification Method**: Test (T)

#### L2-REG-006

**Parent**: L1-REG-006

**Statement**: Every executable nonliteral interpretation SHALL resolve to its permanent interpretation-register entry, cited sources, chosen behaviour/reason, design and review evidence, completed development analysis and required executable verification; generation SHALL NOT silently activate an unlisted, open, proposed, stale or otherwise unready interpretation, while retaining its source and explicit unavailable status for coverage reporting.

**Rationale**: ADR-0027 and the register's existing development convention require deep analysis and intensive tests before implementation, in addition to ADR-0033 approval. A planned test name or research draft is not passing verification. This derives the dependency/evidence gate, not a new review policy or permission to implement the current approved-but-unanalyzed entries. These gates also apply to reader/validator behaviour tied to register entries, including hand-written implementations: INT-021 (F1) and INT-030 (F7) are subject to ADR-0033, not exceptions. Joey's 2026-11-23 approval plus AI concurrence at #30 head 13173b0 complete their semantic review; freshness, analysis and verification remain.

**Verification Method**: Test (T)


## Checksum batch 10: recognition and authoritative verification

This initial checksum slice derives L1-SUM-001/002 using reconfirmed
INT-018/019/020 and approved INT-021/F1. Joey's 2026-11-23 reconfirmation
and AI concurrence at main 30e42a3 clear INT-018/019/020's semantic review,
not their development analysis or tests. These are planned obligations.
Full stamping/edit derivation, flex compatibility and checksum API/exit-code
shapes remain later batches. INT-043 Option B is approved (Joey, 2026-11-23; AI CONCURS at #32 head 8941ca1).

#### L2-SUM-001

**Parent**: L1-SUM-001

**Statement**: Authoritative checksum computation SHALL identify the G\SHA item through the approved projected code name, exclude its original raw name-to-terminator interval, and hash every other supplied document byte unchanged; unavailable framing/completeness or an ambiguous exclusion SHALL refuse an authoritative checksum instead of guessing a byte domain.

**Rationale**: 106-24R1 Chapter 9 Table 9-2 MESSAGE DIGEST/CHECKSUM, page 9-13; Chapter 6 section 6.2.3.11 f, page 6-22; INT-018/019/021. Projection recognizes meaning but does not normalize the hashed document. A complete terminated malformed VALUE does not by itself invalidate the exclusion: VERIFY may be malformed while COMPUTE is allowed. Raw-byte hashes may be separately labelled; they cannot substitute for an authoritative TMATS checksum with an undefined exclusion. This does not implement setup-record assembly or declare unknown completeness complete.

**Verification Method**: Test (T)

#### L2-SUM-002

**Parent**: L1-SUM-002

**Statement**: Checksum verification SHALL preserve raw candidates, framing/character findings and completeness evidence while applying reviewed value projection and digest rules; it SHALL distinguish a matching digest from malformed input, indeterminate exclusion, unavailable interpretation and an unverified placeholder, never present those as positive integrity or silently repair the input.

**Rationale**: Reconfirmed INT-018/019/020 and approved F1/INT-021 preserve the lossless source and separate missing knowledge from invalidity. Chapter 6 section 6.2.3.11 f, pages 6-22–6-23, defines the 2- digest and illustrates G\SHA:0;. This slice fixes no public result enum; INT-043 Option B's semantic review is complete, with development evidence outstanding. Existing absent, mismatch and unknown-algorithm reporting obligations remain.

**Verification Method**: Test (T)


## Stamp batch 11: the minimal 0.1 verified transaction

This derives the bounded L1-SUM-004 stamp and transaction prerequisites under
L1-WRT-007 from ADR-0029, approved INT-018/019/020/021/043 and RELEASE-0.1.
It selects no new standard interpretation. Requirements/cases remain planned;
semantic approval does not supply completed analysis or executable tests.
Full editing, X deletion/renumbering (F2), repair suggestions (F3), flex and
CLI/API result schemas remain later work. No broad 0.5 editing feature is
promoted into 0.1 by this minimal transaction slice.

#### L2-SUM-003

**Parent**: L1-SUM-004

**Statement**: A requested stamp SHALL be the final step of a validated transaction, hashing the final emitted bytes outside one identified SHA item, writing canonical 2- plus 64 lower-case hex digits inside that item and verifying by re-reading the output; unresolved checksum-domain ambiguity SHALL refuse a successful stamp without guessing a target or repairing bytes implicitly.

**Rationale**: ADR-0029 and 106-24R1 Chapter 9 Table 9-2 MESSAGE DIGEST/CHECKSUM, page 9-13; Chapter 6 section 6.2.3.11 f, page 6-22. Separators/newlines outside the excluded item contribute to the digest. F1 permits an exact-item removal repair before stamping, not a duplicate winner chosen by stamp. INT-043's noncanonical input tolerance changes neither emitted algorithm spelling nor final-byte verification.

**Verification Method**: Test (T)

#### L2-WRT-002

**Parent**: L1-WRT-007

**Statement**: The minimal stamp transaction SHALL validate its supplied edit targets against their document revision, reject ambiguous/overlapping/stale explicit edit sets, apply permitted explicit repairs atomically as a new revision and rebuild/re-read affected state before stamping; failure SHALL leave the original document untouched and SHALL NOT return a successfully stamped result.

**Rationale**: ADR-0029 supplies exact-target, atomic, rebuild and re-read semantics; L1-WRT-004 forbids automatic changes. This derives prerequisites for 0.1 stamping only, not all general edit/renumbering operations. Detailed target API shapes and unresolved interpretation-dependent edits require their own reviewed contracts. The library gains no file I/O or promise of rolling back a caller's external writer.

**Verification Method**: Test (T)


## Setup-payload batch 12: bounded CSDW decoding before assembly

This derives the payload-boundary part of L1-CH10-001, field reporting under
L1-CH10-002 and literal version reporting under L1-CH10-006. It does not
complete those parents or implement assembly, completeness, edition selection,
session checks, packet slicing/checksums or time views. No packet/file I/O or
new default is introduced. Requirements and acceptance cases remain planned.

The concrete baseline below is 106-24R1 Chapter 11 Figure 11-34. Historical
layout routing and the complete declaration/fallback contract remain separate
batches. INT-012's literal Format 1 decision was reconfirmed by Joey, 2026-11-23,
with AI concurrence on #34 head e352323 after seven-edition source review.
Development analysis/tests and the other interpretations' review gates remain. See the
[source and review note](research/2026-11-23-setup-payload-draft.md).

#### L2-CH10-001

**Parent**: L1-CH10-001

**Statement**: The library SHALL decode a supplied Format 1 packet-body payload under an identified supported CSDW layout, retain the original CSDW and body bytes with locations, reject a payload shorter than that layout's CSDW without fabricating fields, and pass only the bytes after each fragment's CSDW to later assembly/body interpretation.

**Rationale**: 106-24R1 Chapter 11 section 11.2.7.2 a and Figure 11-34, page 11-50, place a 32-bit CSDW at the beginning of each Format 1 body. ADR-0025 separates packet slicing from fragment assembly; a caller supplies body bounds excluding filler/checksum. Sufficient CSDW bytes establish a decodable header, not a complete setup record.

**Verification Method**: Test (T)

#### L2-CH10-002

**Parent**: L1-CH10-002

**Statement**: CSDW reporting SHALL expose raw and interpreted format/change/version fields through the shared irig106-types definitions, preserve all original bits and body bytes, and report an XML body unsupported without parsing it as code-name TMATS; local field decoding SHALL NOT itself assert configuration change, record completeness or session conformance.

**Rationale**: ADR-0009 requires shared packet-level types instead of local copies. Figure 11-34 and its field definitions (106-24R1 Chapter 11 page 11-50) assign FRMT bit 9, SRCC bit 8 and RCCVER bits 7–0. ADR-0015 keeps XML unsupported. Interpreting SRCC against earlier records or disagreement across fragments needs separate reviewed rules; it is not another interpretation of the single bit.

**Verification Method**: Test (T)

#### L2-CH10-003

**Parent**: L1-CH10-006

**Statement**: A supported cited RCCVER reading SHALL preserve the wire byte and report the literal edition assigned by its table, reserved codes as unknown under that reading and any archive-reuse observation separately labelled; it SHALL NOT transform a literal version declaration into a newer edition, an unbounded future claim or a TMATS edition/fallback selection.

**Rationale**: ADR-0032 and INT-012 distinguish table reading from archive observation and fallback policy. 106-24R1 Chapter 11 section 11.2.7.2 a, page 11-51, states 0x0E = RCC 106-22 and 0x0F–0xFF reserved. INT-016's historical not-declared handling and INT-017's fallback remain their own pending-review dependencies; this child does not silently select a historical layout or claim those reviews complete.

**Verification Method**: Test (T)


## Edition-basis batch 13: declarations, policy and selection uncertainty

This derives L1-EDN-005's declaration separation and L1-EDN-002's labelled
basis selection. INT-012 Format 1 literal reading is reconfirmed; INT-014/
015/016/017 retain their existing decisions and pending review/development
gates. INT-044 Option B is approved under ADR-0033 (Joey, 2026-11-23;
AI CONCURS on #36 at 6f6e080). It is not implemented.
Historical-layout routing, full edition coverage/deltas and compatibility
validation remain later derivations. No new supported-edition claim is made.

#### L2-EDN-001

**Parent**: L1-EDN-005

**Statement**: Declaration reporting SHALL preserve G\106's raw occurrences, source locations and reviewed candidate-edition reading separately from each setup record's version declaration and its code-space/table basis; ambiguous or unavailable declaration evidence SHALL remain visible and SHALL NOT be replaced by an applied rules edition or a differently scoped version byte.

**Rationale**: 106-24R1 Chapter 9 Table 9-2 RCC IRIG 106 REVISION LEVEL, page 9-10, names the generating edition; Chapter 11 section 11.2.7.2 a page 11-51 describes Format 1 recorded-data compliance. ADR-0028/0032 and L3-CH10-008 preserve these distinct meanings. INT-014/015/016 remain reviewed-rule dependencies rather than a license to infer older formats or silently parse arbitrary year strings.

**Verification Method**: Test (T)

#### L2-EDN-002

**Parent**: L1-EDN-002

**Statement**: Rules-basis selection SHALL apply a valid caller override, else a usable reviewed G\106 declaration, else the reviewed fallback policy for missing/unrecognised declarations or explicitly labelled ambiguous G\106 under INT-044, using eligible Format 1 RCCVER and finally baseline 106-24R1; it SHALL retain the selected edition, basis, reason and supporting evidence without rewriting declarations, conflating code spaces or treating unavailable knowledge as a missing declaration.

**Rationale**: ADR-0028/0032 provide the precedence and labelled fallback; INT-017 chooses 0x0E -> 106-24R1 and 0x0D -> 106-20 as POLICY, distinct from INT-012. Owner-decided INT-044 Option B adds the explicit ambiguous-declaration reason without relabelling it missing/unrecognised or selecting a candidate; its review gate is complete, while analysis/tests remain. Review and rule-coverage gaps stay visible; selecting a basis does not establish that its full validator exists or that the document conforms.

**Verification Method**: Test (T)

## Assembly-contract batch 14: joining and provenance before completeness

This derives only joining and source mapping within L1-CH10-004.
INT-045 is a separate layered phase-closure rule above INT-011 joining;
its initial A and guarded owner mid-session extension are approved (AI
CONCURS on #41 at 79b94eb). A closed phase cannot silently accept later fragments backward.
A joined candidate run is not proof of one complete setup record. Completeness,
ambiguous splits and session rules still need subsequent derivations and
review of INT-034/028/031. INT-011 is reconfirmed by Joey, 2026-11-23, with AI CONCURS on #38 at
c623b15 under ADR-0033; analysis/tests remain. No interpretation gains
approval through a requirement or a supplied fixture binding.

#### L2-CH10-004

**Parent**: L1-CH10-004

**Statement**: The library SHALL form independent candidate fragment runs per channel from caller-supplied bytes and provenance in recording order using the INT-011 joining prerequisites, retain each run's start reason and why each join was permitted or stopped, and logically concatenate only the TMATS text portions as ordered borrowed spans without mandatory coalescing, extra payload copy, insertion, removal or interpretation at packet/block boundaries; a joined run SHALL remain distinct from a completeness or governance verdict.

**Rationale**: 106-24R1 Chapter 11 section 11.2.7.2 page 11-50 states that one setup record may span consecutive packets and that sequence increments n+1 in segmentation order. Section 11.2.1.1 f defines per-channel sequence counters. ADR-0025 supplies the library boundary; INT-011 supplies the additional same-channel/FRMT/RCCVER interpretation. ADR-0031 supersedes the old assumption that stopping a run proves a record finished. Joey's 2026-11-23 INT-011 clarification makes other-channel packets provenance/information only, never a stop condition. Channel-0 conformance is separate from joining. This is not a packet reader or an end-marker invention.

**Verification Method**: Test (T)

#### L2-CH10-005

**Parent**: L1-CH10-004

**Statement**: Each candidate run SHALL preserve all supplied fragment provenance and raw CSDWs and provide an exact mapping from assembled text bytes and ranges back to their originating packet bytes, including ranges crossing packet boundaries; a summary SHALL NOT discard disagreement or substitute one fragment's metadata for another's evidence.

**Rationale**: ADR-0025 and L1-CH10-004 require file offset, channel, sequence, relative time counter, CSDW fields and data-type version plus a body-offset provenance map. ADR-0003's owned bytes/spans and L1-READ-003's located diagnostics require evidence after concatenation. FRMT/RCCVER agreement does not establish SRCC agreement, a valid configuration-change event or a governing record. No new SRCC aggregation policy is selected here.

**Verification Method**: Test (T)

## Completeness-contract batch 15: finality, streaming evidence and budgets

These partial children continue L1-CH10-004 and L1-ROB-001; they do not finish
family 2 or approve INT-034/028/031. INT-011's reconfirmation covers joining,
not completeness or governance. The reviewer consequences relayed by the
owner after #38 are recorded here; unresolved streaming-use policy stays open.

#### L2-CH10-006

**Parent**: L1-CH10-004

**Statement**: A closed per-channel candidate run SHALL carry its final completeness outcome or unavailable-format/rule reason, its START and stop evidence, and all relevant competing readings; an otherwise complete semicolon-terminated run at supplied EOF SHALL be complete with an explicit end-of-input basis, not downgraded solely because EOF cannot prove the recording was never cut.

**Rationale**: ADR-0031/INT-034 distinguish evidence from certainty. Chapter 9 section 9.4.2 page 9-2 supplies semicolon framing and Chapter 11 section 11.2.7.2 page 11-50 supplies fragmentation without an end marker (106-24R1). INT-011's per-channel joining is reconfirmed; "followed by another packet" means same-channel. Precise classification precedence, leading-damage handling and XML's unknown completeness remain review dependencies, not solved by this requirement.

**Verification Method**: Test (T)

#### L2-CH10-007

**Parent**: L1-CH10-004

**Statement**: An open run SHALL expose a PROVISIONAL observation distinct from final completeness, with its channel, observed text extent and boundary evidence; observing a provisional attribute boundary SHALL NOT stop joining, assert a complete record or silently establish a governing configuration.

**Rationale**: Owner-relayed reviewer consequence, 2026-11-23, enables the streaming-consumer design discussion without treating other-channel data as an end marker. INT-034 records the explicit open question of pre-EOF semantic use; INT-028/031 govern final descriptions and remain proposed. A temporary lack of input is not EOF. No current-only-complete parsing rule is silently relaxed by adding an observation.

**Verification Method**: Test (T)

#### L2-ROB-003

**Parent**: L1-ROB-001

**Statement**: The assembler SHALL expose and enforce an explicit finite aggregate memory budget for retained per-channel open-run state and SHALL report a typed limit error when a requested operation would exceed that budget, without silently dropping, merging or finalizing any run.

**Rationale**: Owner-relayed reviewer consequence after #38, 2026-11-23. Independent channels can keep several runs open while unrelated traffic continues. L1-ROB-001 requires bounded, non-aborting handling; lossless retention forbids hidden eviction. Accounting must include retained text and metadata, not just one channel's payload. The accounting model, numeric defaults if any, and post-error resume API require review before implementation; the budget is resource policy, not a TMATS default or record-size rule.

**Verification Method**: Test (T)


## Retransmission interpretation verification (owner-signed INT-046–053)

Owner direction, 2026-11-23, requires the whole matrix to be tested. The
decisions are owner-signed, Joey, 2026-11-25, with independent AI CONCUR
under ADR-0033 at PR #45 head `73396bc312e24cfccaf16de9ae7a72609faf75b6`.
QM/INT-053 adds a separate bounded sequence contract without expanding the
pairwise product. Source freshness, development analysis and executable
verification remain required before implementation.

#### L2-REG-007

**Parent**: L1-REG-006

**Statement**: Implementation of retransmission interpretations INT-046–053 SHALL verify every member of the versioned exhaustive pairwise decision matrix and the named bounded QM sequence cases, asserting sealed outcome/basis, following outcome, findings and governing effect against independently established expectations; unresolved owner decisions SHALL block readiness rather than count as passing implementation coverage, and named fixtures SHALL cover the owner's explicit examples.

**Rationale**: Joey's 2026-11-23 whole-matrix direction extends ADR-0027's interpretation verification and ADR-0033's review gate. The matrix cites 106-24R1 Chapter 11 section 11.2.7.2 SRCC and Chapter 6 section 6.2.3.11 f checksum domain, with INT-011/018–021/034/039/043/045 dependencies. A documentation expander, sampled property test or oracle sharing the implementation path cannot demonstrate these outcomes. Physically inconsistent input observations need explicit rejection/consistency checks, separately reported from realizable byte/provenance fixtures.

**Verification Method**: Test (T)


## Time-view metadata batch 16: channel scope and field evidence

This derives the 0.1 TIMEIN/secondary-header subset of L1-VIEW-007 from the
owner direction of 2026-11-21 and RELEASE-0.1. It composes approved
INT-035–039/041 and the existing effective-value requirements; it implements
no Rust API. No new interpretation or owner choice is selected. Association,
NSB splitting and the remaining general views need separate derivations.
Source: 106-24R1 Chapter 9 section 9.5.4, Table 9-4, rows/pages cited below.

#### L2-VIEW-005

**Parent**: L1-VIEW-007

**Statement**: Within a supplied document/revision and recorder occurrence, the bounded view SHALL identify confirmed TIMEIN entries from a usable CHANNEL DATA TYPE result and retain their recorder/entry identity separately from the actual TRACK NUMBER/CHANNEL ID result; all relevant unresolved or duplicate candidates SHALL remain inspectable without fabricating a channel ID, merging entries or selecting a winner from source order.

**Rationale**: Table 9-4 CHANNEL DATA TYPE R-x\CDT-n, page 9-50, supplies TIMEIN; TRACK NUMBER/CHANNEL ID R-x\TK1-n, page 9-48, supplies the actual ID. RELEASE-0.1 explicitly separates x, n and the wire channel ID. INT-038 canonicalises code-name indices, while INT-039 preserves scalar ambiguity. A declared entry index or the first matching channel is not a substitute for the TK1 value. This derives metadata selection, not the whole general channel/link view in L1-VIEW-002.

**Verification Method**: Test (T)

#### L2-VIEW-006

**Parent**: L1-VIEW-007

**Statement**: Each confirmed TIMEIN entry SHALL expose its time data type format, time format and time source as separate located effective-field results in that same recorder/entry scope, and every observed channel entry SHALL support its own secondary-header time-format read without requiring TIMEIN; all reads SHALL retain applicable edition/definition evidence and independent applicability or consistency findings without inferring a packet format, clock association or enabled secondary header from a different field.

**Rationale**: Table 9-4 TIME DATA TYPE FORMAT R-x\TTF-n, pages 9-71–72; TIME FORMAT R-x\TFMT-n and TIME SOURCE R-x\TSRC-n, page 9-72; SECONDARY HEADER TIME FORMAT R-x\SHTF-n, page 9-51. The first three use the TIMEIN condition; SHTF is a per-channel declaration. ADR-0023 and L2-VIEW-003/004 supply field states and default evidence. The owner brought these fields forward to 0.1, while measurement time-word views remain 0.2.

**Verification Method**: Test (T)

#### L2-VIEW-007

**Parent**: L1-VIEW-007

**Statement**: Every bounded time-view result SHALL retain its source document/revision, recorder/entry scope, source locations and supplied assembly/edition context or explicit limitations, preserve every original byte and finding, and keep locally read metadata separate from any claim that a setup governs or a data channel has an evidenced time-source association.

**Rationale**: RELEASE-0.1's API/uncertainty contract and ADR-0002/0003 require byte-faithful views and stable source ownership. L1-VIEW-004 effective values are brought forward for this subset. Incomplete, retained-cut, ambiguous or unsupported setup evidence cannot become an unqualified usable view; INT-048's cut facet keeps its blocking force. This does not select open INT-028/031 general governance, create a timeline, approve provisional semantic use, or expose a stable Rust symbol before API review.

**Verification Method**: Test (T)


Approved INT-056 (Joey, 2026-11-26; final CONCUR at #52 3fcbd2c) distinguishes
preserved source diagnostics from bad-data findings emitted on an assumption result:
Quiet suppresses exactly Warn's bad-data findings of any severity, keeping
them readable from field results/source diagnostics and leaving states
unchanged. Every Quiet result carries `suppressed: N bad-data findings`, even 0.
This later caller policy qualifies result presentation,
not the no-byte-loss/no-state-upgrade obligation.

## Source-bit batch 17: explicit recorder declaration before derivation

This derives L1-VIEW-005's owner-approved 0.1 subset. It composes the
existing effective-field/canonical-scope contracts and approved
INT-035–039/041/042. No Rust API or new interpretation is implemented.
Owner-signed INT-055 supplies the numeric field-value expectations, including
R\N condition operands and numeric-looking enumeration codes. The former
pending-owner fixture blocker is removed and final ADR-0033 concurrence
is recorded at #51 ebd5c98; source freshness, development analysis and
executable tests still gate implementation. INT-054 similarly records CHE context without silent filtering.
Sources: 106-24R1 Chapter 9 section 9.5.4 Table 9-4 NUMBER OF SOURCE BITS,
page 9-33; Chapter 11 section 11.2.1.1 b(1), page 11-5.

#### L2-VIEW-008

**Parent**: L1-VIEW-005

**Statement**: For a supplied recorder/entry identity and full channel-ID result, the bounded view SHALL derive multiplexer source ID and source-local channel ID only from one explicit, usable, applicable R-x\NSB declaration in that recorder/document/revision scope and a usable full channel ID; missing, invalid, ambiguous, unsupported or otherwise unavailable declaration evidence SHALL preserve uncertainty without applying a default source-bit count.

**Rationale**: NUMBER OF SOURCE BITS defines 0–13 most-significant channel-ID bits and permits explicit zero for one source; Chapter 11 defines the 2-byte channel-ID field and remaining low bits. RELEASE-0.1 permits splitting only on a valid explicit declaration. Approved INT-039 keeps identical scalar repeats Ambiguous; INT-041 preserves unavailable prerequisites. Presence applicability uses the reviewed INT-042 condition contract, not a guessed R-x\ID value or a borrowed recorder. This does not select a governing setup or change channel enumeration.

**Verification Method**: Test (T)

#### L2-VIEW-009

**Parent**: L1-VIEW-005

**Statement**: A source-bit result SHALL retain the original full channel ID, the independently identified recorder/entry and document/revision, the explicit declaration and its source/basis, the derived widths/values or blocking reason, and all qualification/findings; it SHALL NOT rewrite channel identifiers, coalesce channels by a derived local ID, infer a clock association or silently select governing context.

**Rationale**: The same Chapter 9/11 rows define a numerical partition, not a clock reference or time-source association. RELEASE-0.1 forbids association inference from multiplexer grouping and requires declaration provenance. Batch 16 already distinguishes actual TK1 from entry n. This contract operates on supplied identity/context; DSI context is derived in batch 18 and CHE enumeration follows owner-signed INT-054, with final independent CONCUR at #51 ebd5c98.

**Verification Method**: Test (T)


## DSI-context batch 18: source identification without clock selection

The owner-directed bounded time view includes DATA SOURCE ID as context
(RELEASE-0.1 Table 9-4 source list; follow-up to #48). This derives that
context read for a supplied recorder/entry, not a general link graph or a
channel-enumeration policy. Approved INT-035–039/041 and existing effective
reads apply. CHE and numeric field-value choices are recorded in owner-signed INT-054/055;
final independent CONCUR is recorded at #51 ebd5c98. This context contract does not choose
a general link graph or governing setup.

INT-055 covers R-x\N, NUMBER OF TRACKS/CHANNELS (106-24R1 Chapter 9
section 9.5.4 Table 9-4 p. 9-32, range 1–65536), whose R\N > 0 condition
is used by TK1/CDT/SHTF/DSI/CHE (pp. 9-48–51). Tests may now specify parsed
operand expectations as well as isolated supplied-condition units; neither
may assume an absent/unknown condition true. Numeric codes TTF/SHTF/TFMT
(pp. 9-51, 9-71–72) accept 02 as 2 with a warning under this owner decision,
not by borrowing index syntax. Final independent AI CONCUR is recorded at #51 ebd5c98.

#### L2-VIEW-010

**Parent**: L1-VIEW-007

**Statement**: A bounded time-view entry SHALL expose its own R-x\DSI-n DATA SOURCE ID as located source context or its raw evidence and unavailable/state reason, preserving document/revision and recorder/entry scope and the existing effective-field rules; equal or suggestively named DSI text SHALL NOT select or evidence a clock, substitute for channel identity, merge entries or establish that a setup governs.

**Rationale**: 106-24R1 Chapter 9 section 9.5.4 Table 9-4 DATA SOURCE ID, page 9-49, defines the source identifier with its own usage/link fields; RELEASE-0.1 explicitly treats it as context, not a time-channel reference. Batch 16 binds fields by canonical x,n rather than TK1, so isolated context composition can receive evaluated applicability without parsing numeric field values. Parsed-R\N applicability expectations now follow owner-signed INT-055, with final independent CONCUR at #51 ebd5c98. Raw lookup and locally usable supported-field results remain distinct from general link resolution, length/blank rule interpretation or whole-document authority.

**Verification Method**: Test (T)

## Owner-signed CHE and numeric syntax (family 2, batch 19)

INT-054/055: Joey, Option A, 2026-11-25. INT-054/055 have final AI CONCUR at #51 ebd5c98.
These specify local effective reads/enumeration, not the full later validator.
Source freshness, development analysis and executable verification still gate
implementation; no new requirement is verified by this documentation.

#### L2-VIEW-011

**Parent**: L1-VIEW-004

**Statement**: Decimal-integer effective reads, including condition operands and numeric-looking enumeration codes, SHALL apply INT-055's bounded raw ASCII-digit grammar and canonical comparison, preserve raw values/locations and warning findings, check the field's cited range after canonicalisation, and retain existing multiplicity/character guards and uniform INT-041 leading-blank unavailability without repairing source or guessing unusable operands.

**Rationale**: Owner Option A supplies lexical policy absent from the cited ranges, including NSB/TK1/R\N and TTF/SHTF/TFMT codes (106-24R1 Table 9-4 pp. 9-32–33, 9-48, 9-51, 9-71–72). It extends canonical-string comparison to values expressly; it does not change the code-name-index grammar or G\SHA's structured rule. INT-055 excludes binary/masks, floating-point, hex and all other non-decimal representations. Numeric-looking codes require an explicit reviewed enumeration table, never a bit pattern; see the complete exclusion/Range audit inventory.

**Verification Method**: Test (T)

#### L2-VIEW-012

**Parent**: L1-VIEW-007

**Statement**: The time view SHALL still enumerate a confirmed TIMEIN entry with disabled CHE, reporting its own located CHE context and effective state without implicit filtering; missing, duplicate, invalid or unavailable CHE SHALL retain the existing state/evidence rules and independent findings, and any caller filter SHALL be explicit without changing the underlying entries.

**Rationale**: Owner Option A, INT-054, distinguishes inspection from generation of packets. Table 9-4 CHANNEL ENABLE p. 9-49 describes T/F and packet generation, not a library's enumeration API. CHE never substitutes for TIMEIN classification or overrides setup qualification.

**Verification Method**: Test (T)

## Association uncertainty and caller assumptions (family 2, batch 20)

This derives the owner-approved RELEASE-0.1 association contract of
2026-11-21, not a new TMATS link or evidence classifier. Source inspection
and boundaries are in the [batch note](research/2026-11-26-association-assumption-requirements.md).
INT-039/041/054/055 preserve field evidence; they supply no clock reference.
Owner answers of 2026-11-26 now specify caller policy in INT-056–060
(final independent CONCUR at #52 3fcbd2c); no stable Rust API is selected here.

#### L2-VIEW-013

**Parent**: L1-VIEW-007

**Statement**: For a supplied data-channel identity and setup document/revision context, the bounded time view SHALL report association_unestablished when no adequate reviewed evidence establishes a time-channel association, retain relevant scoped candidates, their metadata qualifications and the reason, and distinguish independently established conflicting association evidence from an evidenced association without choosing a candidate by occurrence, similarity, format, enable state or count alone.

**Rationale**: RELEASE-0.1 explicitly defines association_unestablished/conflicting/evidenced as distinct from field/link lookup states. 106-24R1 Chapter 9 section 9.5.4 Table 9-4 NSB p. 9-33, TK1 p. 9-48, DSI/CHE p. 9-49, CDT p. 9-50, SHTF p. 9-51 and TTF/TFMT/TSRC pp. 9-71–72 describe local metadata, not a cited data-channel-to-clock reference. The owner's no-inference contract governs this view. It does not assert that the standard can never define another relationship; any future proof path must cite its actual source and reviewed semantics.

**Verification Method**: Test (T)

#### L2-VIEW-014

**Parent**: L1-VIEW-008

**Statement**: A time-view answer using an explicit caller choice without association proof SHALL retain the choice, its caller-supplied assumption basis and scoped identities on every answer derived from it, alongside the unchanged underlying association uncertainty, target states and selected Strict/Warn/Quiet level, with bad-data finding emission governed by INT-056; it SHALL NOT relabel that choice as association_evidenced, silently propagate it to another setup/revision or make otherwise unusable setup evidence usable or governing.

**Rationale**: Owner direction 2026-11-21, the owner answers of 2026-11-26 (INT-056–060, final CONCUR at #52 3fcbd2c), and RELEASE-0.1 permit labelled caller assumptions for the bounded time view, not silent guesses or global configuration overrides. Other assumption uses in L1-VIEW-008 remain 0.2. This does not approve proposed INT-028/031/032/033, select a governing setup or authorise provisional semantic use; source and eligibility limitations remain visible.

**Verification Method**: Test (T)

## Byte-faithful output and normalization boundary (family 2, batch 21)

This refines L2-WRT-001/L3-WRT-001 from batch 1. Accepted ADR-0002/0003
make immutable original bytes authoritative; ADR-0010 permits bytes or
caller-supplied writers, not library-owned I/O. The [batch note](research/2026-11-26-lossless-output-requirements.md)
records source/contract boundaries and planned oracles. No new interpretation,
normalization algorithm, edit API or Rust output signature is selected here.

#### L2-WRT-003

**Parent**: L1-WRT-001

**Statement**: Unedited default output through either bytes or a caller-supplied writer SHALL preserve the full original source byte sequence independently of parsed/typed validity or prior read-only operations; success SHALL require complete delivery under the supplied writer's contract, and output failure SHALL leave the document unchanged without claiming successful output or rolling back the caller's external sink.

**Rationale**: ADR-0002 chooses byte-faithful replay instead of reconstructing the typed model; ADR-0003 owns original bytes; ADR-0010 leaves files and output destinations to callers. L3-WRT-001 already requires complete source replay and fallible writer errors. This requirement makes the success/failure and source-versus-sink boundaries testable, without authorising local publishing, packet writing or automatic repair.

**Verification Method**: Test (T)

#### L2-WRT-004

**Parent**: L1-WRT-002

**Statement**: The output boundary SHALL distinguish unedited default replay from a caller's explicit normalization request; parsing, registry loading, validation, effective values, queries and output presentation preferences SHALL NOT implicitly request canonical ordering, line-ending conversion, counter recomputation or other source changes.

**Rationale**: L1-WRT-002 and ADR-0002 require caller authorization before normalization. Chapter 9 section 9.4.2 allows free attribute order and case-insensitive interpretation; neither grants automatic byte rewriting. This is the opt-in gate only. Transformation definitions, canonical ordering and supported counter-rewrite behavior need their own reviewed derivations before the full normalization capability is implemented; this partial child does not complete L1-WRT-002 or reduce the 0.1 scope.

**Verification Method**: Test (T)

## Core-backed setup slicing (family 2, batch 22; review correction)

ADR-0036 supersedes the temporary tmats reader in ADR-0019/0025. The CLI
uses header-only PacketFacts (irig106-types), borrowed DATA spans and
core PacketInfo integrity/trust sideband carried alongside them; only TMATS CSDW/fragment
slicing and assembly remain here. See the [decision record](research/2026-11-26-core-reader-span-rework.md).
Existing IDs are re-pointed, not reused for a new local reader. Chapter 11
facts remain reference evidence. No report/rendered format is selected.

#### L2-CLI-003

**Parent**: L1-CLI-008

**Statement**: The tmats CLI SHALL obtain packet finding, validated DATA extents, packet trust and primary/secondary-header checksum evidence from core-supplied integrity/trust evidence (PacketInfo sideband: secondary_integrity per RDG-008, data-checksum state, admission evidence, ranges), carried ALONGSIDE header-only PacketFacts from irig106-types and borrowed DATA spans, without implementing another packet reader, header/profile validator or checksum algorithm, and SHALL preserve core's evidence rather than synthesize a stronger verdict or choose among RDG-008 readings.

**Rationale**: Owner architecture 2026-11-22 and coordinator rework decisions, recorded in ADR-0036. Core is available; ADR-0019's temporary-reader condition no longer applies. Exact compatible dependency/API binding remains implementation work. The TMATS library remains I/O-free; the CLI adapter consumes core facts and passes spans/provenance/evidence to it without inventing a duplicate PacketFacts definition or adding integrity/trust fields to it.

**Verification Method**: Test (T)

#### L2-CLI-004

**Parent**: L1-CLI-008

**Statement**: Over core's borrowed DATA spans, TMATS SHALL read the first four logical bytes as the setup CSDW and retain the remaining logical bytes as borrowed text fragments, accepting block-split CSDWs/text directly without coalescing or extra payload copy; it SHALL preserve source mappings and core integrity evidence unchanged, including data checksum NotChecked, and SHALL NOT verify packet data checksums locally.

**Rationale**: Chapter 11 section 11.2.7.2/Figure 11-34 p. 11-50 defines each packet's CSDW before text; sections 11.2.1.1 d and 11.2.1.4 define DATA excluding filler/checksum. Core supplies those DATA spans; tmats must not reconstruct them from raw headers. ADR-0036 records direct non-contiguous input and core-owned verification. Library assembly stays per-channel under INT-011/045, independent of physical block boundaries. The coordinator's data-checksum feature request is for core; owner scheduling is pending.

**Verification Method**: Test (T)


## Content-based input routing (family 2, batch 23)

The content selector in accepted ADR-0019 remains applicable after ADR-0036
supersedes its temporary packet reader. This decomposes L1-CLI-001 only;
packet finding/trust remains core's responsibility. See the [source and
boundary note](research/2026-11-26-content-routing-requirements.md).
No command/report format, recovery mode or new interpretation is selected.

#### L2-CLI-005

**Parent**: L1-CLI-001

**Statement**: The CLI SHALL select recording input when the successfully acquired source begins with the complete Chapter 10 packet sync pattern and standalone TMATS text otherwise, independently of filename/extension; selection SHALL neither validate the selected content nor search for a later packet sync pattern.

**Rationale**: ADR-0019 and CLI section 3 choose this content predicate. Chapter 11 (106-24R1) section 11.2.1.1 a, p. 11-5 specifies sync 0xEB25; section 11.1, p. 11-1 specifies little-endian structural fields, giving initial bytes 25 EB. This is the existing input-mode selector, not a local reader, trust decision or damaged-file recovery algorithm. A source shorter than the complete pattern cannot satisfy it.

**Verification Method**: Test (T)

#### L2-CLI-006

**Parent**: L1-CLI-001

**Statement**: Input acquisition and routing SHALL preserve original bytes and source identity, delegate recording packet work to core and standalone text interpretation to the TMATS reader, and preserve acquisition/downstream failure without manufacturing a successful empty document or silently retrying the other input mode.

**Rationale**: ADR-0003/0010 preserve raw source and keep library I/O caller-owned; ADR-0036 requires one core reader. A successful empty source differs from an acquisition failure. Neither content selection nor forwarding core evidence establishes a complete, trusted or governing setup. Dependency availability and recovery policy remain separate gates; types/core 0.2 must be published before tmats 0.1.

**Verification Method**: Test (T)
