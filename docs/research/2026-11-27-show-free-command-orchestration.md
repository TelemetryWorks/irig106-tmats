# Show-free command orchestration: family 2 batch 43

Date: 2026-11-27. Documentation-only integration acceptance, not executable
command coverage, an owner decision or a new standard interpretation.

## Why another integration layer

The existing requirements separately describe supplied arguments, content
routing, core packet traversal, TMATS assembly, scoped enumeration, extraction,
checksum computation and output delivery. Those component contracts do not
alone demonstrate that the CLI connects the same source and revision through
the intended library calls, or returns the actual failure to a mounted host.

L1-CLI-009 and ADR-0034 W3 require a shared host-callable runner plus individual
command/render APIs and thin main. ADR-0010 puts file acquisition at the edge;
ADR-0013 requires one hand-rolled parser. ADR-0036 puts the only recording
reader in core and retains borrowed DATA spans and separate core evidence.
These are existing architecture decisions, not a new CLI design.

The new L2-CLI-016/017 and L3-CLI-062..067 derive cross-stage source binding,
per-invocation context and acceptance evidence from those obligations. They
complement, rather than replace, the component and mounting tests. No Chapter
9 code name, field grammar, default or packet interpretation is added here.

## Unheld computation boundary

Use an explicitly supplied reviewed internal command request and scoped
source/document/revision. A runner must not turn a different candidate with
the same bytes into the selected one. Raw extraction and checksum-domain
computation retain original bytes, provenance and existing exclusions;
interpreted field projections are not source serialization.

Core header facts, separate trust/integrity/admission evidence and record
qualifications remain attached. G\SHA and core data-checksum results cannot
upgrade one another. Core admission refusals/drain reports remain unchanged
and distinct from assembler limits; overlapping byte accounts are not added.
This does not invent availability of source extents or permission to adopt
a setup. Stamp's explicit output revision and re-verification stay separate.

Per-invocation options and source bindings cannot be silently inherited from
a previous success or failure. Approved INT-060 independently requires each
association query to carry its choice explicitly. Missing edition override
still permits the existing reviewed fallback policy, rather than inheriting
the prior call's override. Caller-managed explicit context is allowed; this
is not a cache-flushing policy, new reset API or concurrency guarantee.

Acquisition, traversal, selection/access, computation and delivery failures
retain their actual stage/source evidence. A successful computation does not
prove its bytes reached the supplied sink. No continuation/retry/rollback
behavior or numeric exit classification is invented by observing this
boundary. Existing contracts still govern partial delivery and host lifetime.

## Acceptance without choosing report formats

The 18 planned TEST-DATA cases specify internal source bindings, unchanged
qualifications, raw byte oracles, injected failures and subsequent explicit
calls. Compare runner command handoff against the individual-command path
before rendering. Do not derive diagnostic prose, finding order, assumption
labels, suppression markers, JSON fields, stream placement or numeric exits.

Synthetic stage adapters can establish wiring mechanics only. They cannot
qualify a real core version, source ownership, checksum algorithm, renderer
or implemented command. Later acceptance must name actual text/recording
paths, reviewed requests and independent results; the help/version scaffold
and a component-only suite are insufficient. No duplicate parser/reader or
project/development dependency is introduced. Heavy executable checks belong
in GitHub CI once code exists; this batch runs local documentation checks only.

## Holds and the next readiness review

No report/rendered-output format is derived here. Public argument grammar,
record numbering/defaults, first-fragment versus completion order, semantic
access under proposed INT-031, strict scope, stamp destination policy, flex
qualification and std/#59 retain their existing gates. This batch does not
change the owner's extract-only rules into a general command policy.

The resource-policy blocker now has an explicit reader-progress row: retained
BlockPool credits are in scope, but cap/early-copy/typed-failure remains
PENDING OWNER, as requested after #78. Normalization INT-064/065/066 also
await owner decisions. None of those holds is silently decided here.

Following review of this last identified show-free integration slice, reassess
the family-2 coverage audit against current requirements and owner answers.
Do not promise family-3/parser readiness from counts or prototype CI. Source
freshness, development analysis, real executable coverage and applicable
owner gates remain necessary; full documented 0.1 scope is unchanged.
