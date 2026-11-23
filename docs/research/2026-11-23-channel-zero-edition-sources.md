# Channel 0: per-edition evidence for the INT-011/INT-045 review

Date: 2026-11-23. **Source-review input only; no new conformance threshold,
joining predicate or interpretation approval.** Read each cited RCC chapter
copy; do not apply a later edition's retrospective wording to every recording.
Page numbers below are printed pages, not PDF indices.

| Edition and inspected source | Channel-ID section/page | Observed text/context |
| --- | --- | --- |
| [106-07 Ch10](https://www.irig106.org/docs/106-07/chapter10.pdf) | §10.6.1.1 b and following note, p. 10-32 | Channel 0 reserved for computer-generated messages; the SINGLE-MULTIPLEXER note says Computer Generated Data packets only use 0 and no other data type can use 0 |
| [106-09 Ch10](https://www.irig106.org/docs/106-09/chapter10.pdf) | §10.6.1.1 b and following note, p. 10-30 | Same reserved-channel wording and single-multiplexer qualification; retain that scope |
| [106-11 Ch10](https://www.irig106.org/docs/106-11/chapter10.pdf) | §10.6.1.1 b, p. 10-30 | Reserved channel 0 inserts computer-generated messages; this paragraph is not the later Format-1/4 wording |
| [PROJECT-ARCHIVED 106-13 Ch10](https://github.com/TelemetryWorks/rcc-106-standards/releases/download/106-13/chapter10.pdf) | §10.6.1.1 b(2)–(3), p. 10-29 | Explicitly says "as of 106-13" and only Format 1 setup on reserved 0; the next clause separately addresses Formats 0 and 2–7 on nonzero channels |
| [106-15 Ch10](https://www.irig106.org/docs/106-15/chapter10.pdf) | §10.6.1.1 b(2)–(3), p. 10-26 in this copy | Also says "as of 106-13", Format 1 on reserved 0, with a separate nonzero-channel clause |
| [106-17 Ch11](https://www.irig106.org/docs/106-17/chapter11.pdf) | §11.2.1.1 b(2)–(3), p. 11-5 | Says "as of 106-17", allowing Format 1 setup or Format 4 Streaming Configuration on reserved 0 |
| [106-19 Ch11](https://www.irig106.org/docs/106-19/chapter11.pdf) | §11.2.1.1 b(2)–(3), p. 11-5 | Same 106-17/Format-1-or-4 wording |
| [106-20 Ch11](https://www.irig106.org/docs/106-20/chapter11.pdf) | §11.2.1.1 b(2)–(3), p. 11-5 | Same 106-17/Format-1-or-4 wording |
| [106-22 Ch11](https://www.irig106.org/docs/106-22/chapter11.pdf) | §11.2.1.1 b(2)–(3), p. 11-5 | Same 106-17/Format-1-or-4 wording |
| [106-23 Ch11](https://www.irig106.org/docs/106-23/chapter11.pdf) | §11.2.1.1 b(2)–(3), p. 11-5 | Same 106-17/Format-1-or-4 wording |
| [106-24 Ch11](https://www.trmc.osd.mil/wiki/download/attachments/351465801/chapter11.pdf?api=v2) | §11.2.1.1 b(2)–(3), p. 11-5 | Same 106-17/Format-1-or-4 wording |
| [106-24R1 Ch11](https://www.trmc.osd.mil/wiki/download/attachments/335389015/chapter11.pdf?api=v2) | §11.2.1.1 b(2)–(3), p. 11-5 | Same 106-17/Format-1-or-4 wording |

The project-archived 106-13 PDF has SHA-256
`e3740c7ac3099236f4e97cbe5b84c169340e6d4a4222769619e83ea0f8176fde`.
It states at p. 10-29 that channel 0x0000 is reserved and, "as of 106-13",
is used to insert only Format 1 setup records. This is not p. 10-26 of the
separate irig106.org copy. Earlier [chapter-copy fingerprints](2026-11-23-initial-time-boundary-proposal.md#source-fingerprints-and-limits-of-this-check)
identify the other inspected copies; the 106-13 row here deliberately uses
the PROJECT archive instead of that older note's irig106.org copy.

## Proposed input to the edition-dependent conformance check

Bind a channel-use check to the actual edition/source profile, exact passage,
raw declaration evidence and relevant scope such as single multiplexer.
Keep G\106, CSDW RCCVER and packet Data Type Version distinct; do not invent
a profile from numeric similarity or an unlabelled fallback. Profile selection
and any unresolved historical reading need owner/independent review before
implementation. This proposal records source evidence, not a new mapping.

In particular, a restriction on what channel 0 may carry is not automatically
a complete statement that EVERY setup record must be on channel 0. Nor does
"before 106-17" by itself prove nonzero setup-channel use is conforming.
The older single-multiplexer note and later nonzero-channel clauses must be
read in their own scopes. Do not silently resolve that interpretation here.

INT-011 joining stays per-channel and independent even when conformance is
uncertain or violated. INT-045's approved closure guards remain unchanged;
this evidence feeds a separate per-edition channel-use review, not a new
reason to mix, discard or refuse joining fragments. The existing baseline
106-17+ channel-0 time-packet finding is not silently extended/repealed.

Recommend this explicit source profile over applying one baseline cutoff to
all historical editions. That recommendation remains UNAPPROVED. No corpus,
implementation or executed conformance test is claimed by reading PDFs.
