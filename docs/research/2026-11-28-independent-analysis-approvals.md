# Independent analysis-review APPROVEs — 2026-11-28

Reviewer: @thread:thr_qsvwmtnn8f. The coordinator relayed the independent
APPROVEs and requested this record on 2026-11-28. These are ANALYSIS-review
records, separate from the interpretations' existing semantic Design/Review
sign-offs. Owner acceptance of the analyses stays PENDING, as do intensive
executable tests and source/integration readiness. A merged analysis PR or
an independent APPROVE does not waive either gate or clear S1 SUSPECT.

## Analysis review records

| Independent APPROVE | Exact reviewed head | Interpretations |
|---|---|---|
| [#84](https://github.com/TelemetryWorks/irig106-tmats/pull/84) | [`195c4913e215b46c6b05ca342d96a3d6a3ec8b1f`](https://github.com/TelemetryWorks/irig106-tmats/pull/84/commits/195c4913e215b46c6b05ca342d96a3d6a3ec8b1f) | INT-035, INT-036, INT-037 |
| [#86](https://github.com/TelemetryWorks/irig106-tmats/pull/86) | [`3a1b5903087ae89fba161c348b5276debbc4e1f3`](https://github.com/TelemetryWorks/irig106-tmats/pull/86/commits/3a1b5903087ae89fba161c348b5276debbc4e1f3) | INT-038, INT-039, INT-040 |
| [#90](https://github.com/TelemetryWorks/irig106-tmats/pull/90) | [`bb9ba7612773c5c90d7f9d9d6b9823ad09829a85`](https://github.com/TelemetryWorks/irig106-tmats/pull/90/commits/bb9ba7612773c5c90d7f9d9d6b9823ad09829a85) | INT-041, INT-042, INT-044 |
| [#93](https://github.com/TelemetryWorks/irig106-tmats/pull/93) | [`3fb65dbfd801365d1ade7ca5b8fa4b4c3a3b0d07`](https://github.com/TelemetryWorks/irig106-tmats/pull/93/commits/3fb65dbfd801365d1ade7ca5b8fa4b4c3a3b0d07) | INT-018, INT-019, INT-020, INT-021, INT-043 |
| [#94](https://github.com/TelemetryWorks/irig106-tmats/pull/94) | [`cc416dd5132914766f87b53a1ff611a7a5e38f33`](https://github.com/TelemetryWorks/irig106-tmats/pull/94/commits/cc416dd5132914766f87b53a1ff611a7a5e38f33) | INT-046, INT-047, INT-048, INT-049, INT-050, INT-051, INT-052, INT-053 |
| [#95](https://github.com/TelemetryWorks/irig106-tmats/pull/95) | [`9f2734b2d3d884e85c50557d6d06db1fb9207ff7`](https://github.com/TelemetryWorks/irig106-tmats/pull/95/commits/9f2734b2d3d884e85c50557d6d06db1fb9207ff7) | INT-001, INT-002, INT-003, INT-004, INT-005, INT-006, INT-007, INT-008 |
| [#96](https://github.com/TelemetryWorks/irig106-tmats/pull/96) | [`6e98255d046f5279fa7ba0c46f41fa8ded6f52d6`](https://github.com/TelemetryWorks/irig106-tmats/pull/96/commits/6e98255d046f5279fa7ba0c46f41fa8ded6f52d6) | INT-054, INT-055, INT-056, INT-057, INT-058, INT-059, INT-060 |
| [#97](https://github.com/TelemetryWorks/irig106-tmats/pull/97) | [`372a2502529563ab1ac1399de77e2bd62cc9c1e3`](https://github.com/TelemetryWorks/irig106-tmats/pull/97/commits/372a2502529563ab1ac1399de77e2bd62cc9c1e3) | INT-061, INT-062, INT-063, INT-064, INT-065, INT-066 |
| [#98](https://github.com/TelemetryWorks/irig106-tmats/pull/98) | [`a01bc8387d8e19f71564cd153094de5a225c7d90`](https://github.com/TelemetryWorks/irig106-tmats/pull/98/commits/a01bc8387d8e19f71564cd153094de5a225c7d90) | INT-009, INT-010, INT-012, INT-013, INT-014, INT-015, INT-016, INT-017 |
| [#99](https://github.com/TelemetryWorks/irig106-tmats/pull/99) | [`98d0b8a359ecce27fac79786559d70d47e930642`](https://github.com/TelemetryWorks/irig106-tmats/pull/99/commits/98d0b8a359ecce27fac79786559d70d47e930642) | INT-011, INT-028, INT-029, INT-030, INT-031, INT-032, INT-034, INT-045 |
| [#100](https://github.com/TelemetryWorks/irig106-tmats/pull/100) | [`7619cb0fed0fb93916655f27e7c9a95bca212b31`](https://github.com/TelemetryWorks/irig106-tmats/pull/100/commits/7619cb0fed0fb93916655f27e7c9a95bca212b31) | INT-022, INT-023, INT-024, INT-025, INT-026, INT-027, INT-033 |

For #84 and #86, the coordinator's explicit approval-relay instruction is
recorded here; the exact heads also appear in their archived PR descriptions.
The later reviews were delivered with exact-head details in the coordinator/
reviewer messages. This does not claim GitHub's formal review API contains
those cross-thread reviews or fabricate a verbatim review quotation.

## Checksum follow-up scope

INT-018/019/020/021/043 were independently approved at #93 head 3fb65db.
The oracle-description correction and named BOM-only COMPUTE case landed
in #94 (commit 4fbaa25, reviewed as part of #94 head cc416dd); that review
explicitly approved both follow-ups. Their Analysis fields therefore record
#93 plus the #94 delta. The later push to the already-merged #93 branch is
not a reviewed main head and is not used as evidence.

## Limits and checks

Only the Analysis-review metadata changes in the register; source text,
behavior, Design/Review sign-offs and the written analyses themselves remain
unchanged. Owner ANALYSIS acceptance and intensive tests remain pending for
every entry. Regenerate the trace to carry these facts, not to upgrade the
zero-of-487 verification status. No implementation, owner decision, dependency,
workflow, release or publishing action is performed.

Family 3 waits for the owner's explicit start-the-restructure instruction.
A mechanical-only plan was prepared locally, outside tracked files;
neither that plan nor any restructure work is pushed by this documentation PR.
