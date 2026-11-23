# Declaration and rules-basis derivation — 2026-11-23

Family 2 batch 13 derives the declaration/basis part of L1-EDN-005 and
L1-EDN-002. It is a source/alternatives record, not completed deep analysis
or executable evidence. INT-012 reconfirmation is recorded separately.
INT-014/015/016/017 retain their own pending review gates. INT-044 is proposed.

## Source and code-space inspection

[106-24R1 Chapter 11](https://www.trmc.osd.mil/wiki/download/attachments/335389015/chapter11.pdf?api=v2):

| Code space | Citation | Important boundary |
| --- | --- | --- |
| Format 1 setup-record RCCVER | section 11.2.7.2 a, Figure 11-34/table, pages 11-50–11-51 | 0D names RCC 106-19, 0E names RCC 106-22 |
| Format 4 Streaming Configuration Record RCCVER | section 11.2.7.5, Figure 11-43/table, page 11-62 | stale table: 0C names RCC-106-17; 0D–FF reserved, not Format 1's newer mapping |
| Packet-header Data Type Version | section 11.2.1.1 e, page 11-6 | 0A names RCC 106-22; Format 1 0A instead names RCC 106-13 |

The reviewer referenced Format 4 Figure 11-41; the current source labels it
Figure 11-43. Figure 11-41 in this edition is an index header. Cite the
structure and edition-correct figure, never a bare reusable version-byte map.
L3-CH10-008 and new planned cases enforce separation without implementing a
Format 4 parser in this slice.

106-24R1 Chapter 9 Table 9-2 RCC IRIG 106 REVISION LEVEL, page 9-10, describes
G\106 as the generating edition and recommends two year digits. Section
9.4.2 page 9-2 describes unique names/single-entry attributes. Neither supplies
a duplicate-G\106 rules-selection algorithm. Approved INT-039 preserves
scalar duplicates as Ambiguous, including identical values.

## Existing declaration and fallback dependencies

INT-014 reads one year as its archived-edition candidates; a supplied reviewed
same-rule equivalence can select a newest shared-rules basis without claiming
that the source year itself declared a revision. INT-015 handles unrecognised
spellings. Their owner acceptance predates ADR-0033 and their AI review is
still pending; no new approval is claimed. INT-016's historical absent/
CH10VER field semantics remain a separate declaration wrapper.

ADR-0028/0032 and INT-017 define precedence and labelled policy. In particular,
Format 1 0E -> 106-24R1 RULES and 0D -> 106-20 RULES are policy, not literal
INT-012 declarations. INT-017's owner decision is already recorded, but its
independent review is pending. INT-012 concurrence on literal tables does
not approve fallback policy or any other code space. Fixtures supply reviewed
policy bindings to isolate the contract, not assert production readiness.

## INT-044 — owner choice, pending sign-off

**Option A, recommended:** duplicate G\106 without a valid explicit override
blocks automatic edition/rules selection. Keep all Ambiguous declaration
candidates/findings; report selection unavailable/cannot-evaluate. Even equal
years stay duplicate scalars. A caller override can choose rules while source
ambiguity stays visible. This costs an explicit override rather than letting
a fallback continue, but does not invent a declaration or missing-input state.

**Option B:** retain all ambiguity but continue through a labelled Format 1/
baseline fallback with reason ambiguous G\106. More convenient, but expands
the owner-decided fallback for missing/unrecognised G\106 to a new case.

The recommendation preserves INT-039's distinction between provided competing
declarations and actual absence. It is not selected silently: INT-044 remains
proposed, owner sign-off and independent AI check pending under ADR-0033.
One scalar year with several candidate editions is INT-014's different case.
The report/basis data structures and policy-input API remain unchosen.

## Evidence and remaining work

New TEST-DATA cases separate raw declaration reports, selection precedence,
policy outputs, unknown code spaces, ambiguity, overrides and coverage limits.
They are planned, not executable tests; source inspection proves no API or
validator implementation. Historical layout selection, full edition/delta
coverage, compatibility checks and remaining family-2 areas need separate
derivations. Deep touched-edition/handbook analysis, source freshness and
intensive tests remain implementation gates even after semantic approval.
