# Argument parser contract and session-scope owner gates: family 2 batch 39

Date: 2026-11-27. Docs-only derivation plus #71 review follow-ups.

## Unheld parser mechanism

Accepted ADR-0013 requires hand-rolled, table-driven parsing and tests for
every flag/value/usage error. ADR-0034 W3 and L1-CLI-009 require a host-callable
runner plus individual command APIs. L2-CLI-014/015 and L3-CLI-052–057 derive
that mechanism without choosing new command grammar, public error types,
rendering, exit numbers or option defaults.

The parser consumes supplied tokens and returns internal request/usage
outcomes. It performs no command/source I/O, printing or process exit and
never acquires global argv/environment to repair an incomplete request.
Source acquisition and content routing remain separate. An input token is
not TMATS text; INT-036 projections and INT-055 numeric-field rules are not
silently reused for CLI arguments. Platform token representation and any
unreviewed option decoding remain review gaps rather than lossy conversion.

A synthetic table can test the mechanism, but product coverage requires an
independent inventory and cases for all actually implemented reviewed rows.
Help/version alone does not implement the documented command set. Unknown
under-table tokens or decoder failures cannot become guessed commands,
editions, record selectors or clock choices. Existing mounted and binary
paths share parsing; explicit individual requests need no reparse of argv.

Sixteen planned cases assert internal outcomes and I/O/execution boundaries.
No argument parsing crate, new test dependency or executable parser is added.
No new owner choice blocks these mechanism requirements; held grammar and
output policies must be reviewed before their table entries are activated.

## Mixture-finding scope — PENDING OWNER, A recommended

The review after #71 identifies a derivation gap: L1-CH10-005 scopes the
finding to Complete setup records; the new children had allowed decoded
candidate/fragment FRMT evidence without restricting formal eligibility.
The owner-relayed instruction for this batch explicitly leaves the choice
pending, with the coordinator recommending A. It is not an owner approval.

- **A, recommended:** formal mixture finding only across Complete records;
  retain non-Complete candidates' FRMT as qualified evidence without issuing
  that formal finding. This follows the existing L1 scope without discarding
  metadata or making an unsupported XML body usable.
- **B:** broaden formal finding eligibility to non-Complete candidates based
  on decoded FRMT. This could expose mixing despite unsupported/incomplete
  bodies, but changes the L1 scope and needs the owner's decision.

L2-CH10-008 and L3-CH10-022/023 now distinguish retained observations from
finding eligibility and explicitly block the scope choice. The planned
Complete-ASCII plus Incomplete-XML case is BLOCKED PENDING OWNER, with A's
recommended no-formal-finding result recorded as a proposal only. No new
interpretation is executable without the existing ADR-0033 readiness gates.

## Recording versus session

106-24R1 Ch11 Figure 11-34 p. 11-50 prohibits ASCII/XML coexistence within a
session; SRCC defines the recording session as .RECORD to .STOP. Existing
INT-045 permits labelled caller session-start assertions, never inference.
Treat an entire recording as one session only with an explicit labelled
caller assertion or actual supplied session evidence. With neither, scope
is unavailable, not no mixture. This does not approve the pending A/B choice.

Paired fixtures hold the recording bytes/header facts fixed: no assertion
or session evidence yields unavailable scope; adding the explicit whole-file
assertion yields available caller-asserted scope. Formal finding eligibility
remains independently gated, and no control command is claimed observed.
Current note/fixtures preserve all record qualifications and access holds.

## Channel-0 source re-verification — evidence, not L1 approval

Read the specifically cited rcc-106-standards release assets and independently
computed their SHA-256 values; all seven match the prefixes relayed by the
reviewer. Inspected the header Channel ID sections and surrounding notes,
using printed page numbers as well as PDF page positions. No other code
repository, reference tool or credentials are inspected. PDFs/extracted
working text stay outside the repository; only this source record is added.

| Archived source | Location | SHA-256 | Verified header reading |
|---|---|---|---|
| [106-05 chapter10.pdf](https://github.com/TelemetryWorks/rcc-106-standards/releases/download/106-05/chapter10.pdf) | §10.6.1.1 b, p. 10-22; PDF 26 | `7aa84ef996df5ecd59da4159b82415ef2ba791a81dd20266c7d96a29adbf6bae` | Reserves channel 0 for computer-generated messages; this header paragraph does not state a Format-1-only-on-0 occurrence rule. |
| [106-07 chapter10.pdf](https://github.com/TelemetryWorks/rcc-106-standards/releases/download/106-07/chapter10.pdf) | §10.6.1.1 b, p. 10-32; PDF 36 | `50d125bc744001d803dcb8cadcb5c2038940ca980098afeb0ebcd25601d8e582` | Single-multiplexer computer-generated packets use channel 0; distributed systems use NSB/source bits in the Channel ID. |
| [106-09 chapter10.pdf](https://github.com/TelemetryWorks/rcc-106-standards/releases/download/106-09/chapter10.pdf) | §10.6.1.1 b, p. 10-30; PDF 40 | `1f93423591c43277009762c31c816358aead90bea9b570a4afc4c68e84ee0ec6` | Same single-multiplexer restriction and distributed NSB/source-bit structure. |
| [106-11 chapter10.pdf](https://github.com/TelemetryWorks/rcc-106-standards/releases/download/106-11/chapter10.pdf) | §10.6.1.1 b and note, p. 10-30; PDF 40 | `8cef16928df5d3f93dfff6f95806e9923b41927413090b0844bf28313065c7a7` | Explicit Format 1 setup-on-0 rule; Formats 0 and 2–7 can share 0 for 2011, deprecated for removal in 2013. |
| [106-13 chapter10.pdf](https://github.com/TelemetryWorks/rcc-106-standards/releases/download/106-13/chapter10.pdf) | §10.6.1.1 b, p. 10-29; PDF 41 | `e3740c7ac3099236f4e97cbe5b84c169340e6d4a4222769619e83ea0f8176fde` | As of 106-13, channel 0 carries only Format 1; other computer-generated formats use nonzero channels. |
| [106-15 chapter10.pdf](https://github.com/TelemetryWorks/rcc-106-standards/releases/download/106-15/chapter10.pdf) | §10.6.1.1 b, p. 10-26; PDF 38 | `cace968a4a796a56895eff36073840ae2db3a9c57dd33794d0c348e95fe0c204` | Retains the as-of-106-13 channel-0 exclusive Format 1 wording. |
| [106-17 chapter11.pdf](https://github.com/TelemetryWorks/rcc-106-standards/releases/download/106-17/chapter11.pdf) | §11.2.1.1 b, p. 11-5; PDF 13 | `7297c782e0368e72ac783a72edccaa04eed28a8ea0e8ab7c61ea39379f516750` | As of 106-17, channel 0 carries Format 1 or Format 4; this changes channel-0 exclusivity, not the 2011 setup-on-0 onset. |

The decisive 106-11 note says “Format 1 Setup Record shall occur in Channel
ID 0 only”. The 106-13/17 changes concern what else may share channel 0.
The 106-07/09 distributed-source clauses support the legacy nonzero-channel
case; they are not permission to infer a distributed profile or silently
turn on core's legacy_setup_any_channel. Core's profile/admission evidence
remains authoritative at the tmats adapter; no local routing rule is added.
The 106-05 observation is scoped to its inspected header text, not a claim
of an exhaustive absence search through every sentence of the chapter.

### L1 wording choice — PENDING OWNER, A recommended

- **A, recommended:** align L1-CH10-005's setup-channel onset with verified
  106-11, while retaining core ownership and explicit legacy opt-in evidence.
  Distinguish that onset from 106-13/17 channel-0 exclusivity changes.
- **B:** defer the L1 wording change. Keep the verified source discrepancy
  and acceptance gap explicit; do not represent the present 106-17 phrase
  as proof that setup-on-0 began in 2017 or silently add a tmats checker.

The coordinator recommends A; the owner has not approved either. This batch
leaves the normative L1 statement/rationale unchanged. L2/L3/ROADMAP now
point to the completed source re-verification and pending wording decision,
not a claim that the old source gap or parent acceptance is resolved.

## Validation, execution limits and remaining work

Local work: the bounded PDF header reads/hash comparisons above, trace
regeneration/check, parent/identifier/case/inventory and whitespace checks.
No Rust builds/tests, reference runs, dependency installs, packaging, sample
recordings or publishing commands execute locally. PDF reads finished; no
background job remains. Heavy checks run in GitHub CI. Planned fixtures and
prototype CI are not implementation evidence for these new obligations.

No output format, usage-message layout, diagnostic serialization, assumed
label, suppressed marker or process-code mapping is selected. Mixture scope
and L1 wording remain recommended-pending; F7 identity/prior-ordering,
INT-031 access, std, file-order confirmation, flex reference choice/source
qualification, historical reviews, INT-061/062/063, pattern, extraction and
allocation holds remain. Earlier dated research records are historical and
unchanged. Family 3/parser implementation has not started.
