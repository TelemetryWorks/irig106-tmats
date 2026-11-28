# Session-format and configuration-change evidence: family 2 batch 38

Date: 2026-11-27. Docs-first requirements and detector clarifications.

## Scope, sources and what is not decided

L1-CH10-005 requires session-format, configuration-event and channel-rule
checks. Existing CSDW/assembly/core integration children do not fully derive
that parent. L2-CH10-008/009 and L3-CH10-022–027 derive observations and
uncertainty, not new packet routing, event identity or a configuration timeline.
The parent remains only partially decomposed/ready while source gates remain.

Source inspected: 106-24R1 Chapter 11 section 11.2.7.2, Figure 11-34,
printed pp. 11-50–51. FRMT defines ASCII/XML; the note disallows both within
the same session. SRCC refers to the recording session defined as .RECORD
to .STOP; the following note requires the configuration-change event before
the new setup. No event packet identity is established by that text.
[Official PDF](https://www.trmc.osd.mil/wiki/download/attachments/335389015/chapter11.pdf?api=v2),
inspected SHA-256:
`0f63792dcac39f6010df371ee0afc46a396017918f4155cba9ebab6c2d6d263e`.

ADR-0025's TMATS assembly survives ADR-0036's core-only reader decision;
INT-030/F7 is owner-approved with its existing AI concurrence, not newly
approved here. Its notes explicitly retain unverified event identity,
first-record SRCC=1, per-fragment disagreement and prior-ordering review.
INT-045's caller-asserted session starts and specific first-fragment SRCC
closure eligibility do not establish a general session detector or bit summary.

## Format observations and supplied scopes

The check receives decoded format observations with their raw provenance,
trust/qualifications and explicit session context. ASCII/XML coexistence can
be observed from CSDW metadata without parsing unsupported XML as text.
Observing a format does not declare the candidate Complete or usable.
Unknown FRMT is not ASCII; missing session context is not a clean pass.

Caller assertions keep their basis. Two asserted separate sessions are not
merged because they share a file, channel or nearby timestamps. A result with
only a successfully supplied prefix cannot prove no mixture in unseen input.
The check preserves original observations and does not choose a winning
format when one candidate contains conflicting evidence. No general SRCC or
boundary inference is introduced to make the input easier to summarize.

## F7 uncertainty and channel ownership

Given an already established change trigger, require verified event identity
before any determinate event-presence decision. A nearby generic Format 2
recording-event packet is a candidate only; neither its presence nor its
absence settles the check. Once identity exists, prior-ordering/scope still
needs its own cited reviewed interpretation. Do not invent missing-event
violations by searching for a guessed type.

Retain every fragment's SRCC and first-record context. Matching FRMT/RCCVER
never establishes agreement on SRCC; no OR/AND/first/last summary is chosen.
SRCC=0 is not proof of byte equality. First-record SRCC=1 remains separate
located evidence, not automatically settled as missing event or valid update.

Core owns packet admission, integrity and edition-dependent channel policy.
L1-CH10-005 still says 106-17, whereas the owner-relayed reminder after #66
says channel-0-only from 106-11. That discrepancy requires archived per-edition
source reconciliation before implementing the parent channel check; this
batch does not silently accept the stale threshold or introduce a local one.
No other repository or archived older edition is inspected in this batch.
Supplied core evidence is passed through, not re-derived here. This is a
source/dependency gate, not a newly selected owner interpretation.

## Detector follow-up carried after #70

Current L3-READ-034/035 and four planned cases now state:

- skip unterminated suffixes, even when a code-looking sequence is visible;
- a proposal within cut-qualified text retains its Incomplete or
  Ambiguous-with-cut qualification; no diagnostic makes it usable/governing;
- proposed INT-031 still gates diagnostic access to qualified records;
- the 0.1 recognizer vocabulary includes the Appendix 9-C code names/index
  patterns, with an independently listed coverage inventory and visible
  readiness failure for any gap, never a silently missed warning.

The stems D-x\MML\N, D-x\MNF\N and D-x\WP are patterns, not complete
literal names without indices. The existing Appendix 9-C p. C-8 excerpt
contains D-1\MML\N-1-1, D-1\MNF\N-1-1-1 and D-1\WP-1-1-1-1; the
inventory must cover the rest of the excerpt too. Existing table/arity and
INT-038 rules still apply; no private grammar or omitted index is guessed.
Source Ch9 SHA-256:
`dfc5ebfb3e6f02f58389126863d43a45d89c7e2d0533a1c690d9537531c740cb`.
The detector remains suspect under S1; these clarifications neither authorize
partial semantic use nor close the separate real-recording review gate.

## Planned verification and existing holds

Sixteen new session cases use independently supplied decoded observations,
context and expected uncertainty; four detector follow-ups cover the requested
boundaries/coverage. These are planned fixtures, not executed conformance.
They add no rendered format, diagnostic severity mapping, JSON schema or
exit code. Classification tests retain their own source and interpretation
gates rather than being simulated by a second reader or guessed event type.

Local work is source inspection and lightweight trace/parent/ID, fixture-link,
source-fingerprint, inventory and whitespace checks. No Rust build/test,
reference run, install, sample access, packaging or release operation is run.
Heavy checks use GitHub CI. New requirements remain Draft with no Evidence.
Flex reference selection/qualification, std, file-order confirmation,
historical reviews, INT-061/062/063, pattern, extraction, allocation,
INT-031 access/governance and output-format holds remain unchanged.
