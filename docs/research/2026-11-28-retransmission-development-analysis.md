# Development analysis: INT-046–053 — 2026-11-28

Eight APPROVED interpretations share one retransmission matrix and evidence
boundary: QI identity, Q0 consistency, QD ambiguity, QB/QE separation, QC/QP
raw changes, QH/QG integrity mode, QS unreadable SRCC and QM comparison target.
This written draft needs owner/independent analysis review and intensive
executable tests. It changes no approved rule, matrix member/class, adoption
policy, source bytes, Design/Review sign-off or implementation status.
Inspected main 3a0d9cb after #93. The two #93 fixture-wording/BOM-only nits
are carried separately in this PR, without changing those five analyses' rules.

## Sources, historical scope and freshness

Re-fetched the archive manifest on 2026-11-28 and checked 45 unique PDF
hashes against it: the 32 sources used by the checksum-domain analysis plus
14 historical Chapter 10/11 assets, sharing the 106-04 full volume once.
Re-extracted/read the located two-page Format 1/CSDW passages for every
packet edition and baseline Ch11 §11.2.1.1 f p. 11-8 separately. Reused the
[checksum source/passages review](2026-11-28-checksum-domain-development-analysis.md)
for Chapter 6/9 and supplemental RCC 123/124 context; fingerprints were
revalidated, not silently assumed from that note. No rendered-image review
is claimed. The optional older-font extraction limitation in that source
review remains; missing extraction is never evidence for inventing a rule.

| Source asset | SHA-256 |
|---|---|
| [106-04 106-04.pdf](https://github.com/TelemetryWorks/rcc-106-standards/releases/download/106-04/106-04.pdf) | `a75ef16a77ecb4838c0c9678b7d483049e84c94d7a49a3edb7f7199ce98914c8` |
| [106-05 chapter9.pdf](https://github.com/TelemetryWorks/rcc-106-standards/releases/download/106-05/chapter9.pdf) | `ccdc77ccdd2f185a33e7ad1ed168375564432ae6d541172fef88ce0b783ea6b1` |
| [106-07 chapter9.pdf](https://github.com/TelemetryWorks/rcc-106-standards/releases/download/106-07/chapter9.pdf) | `ef8783a59dee91db9b1fd7ac1802617cb87f5f52e2ea5de395bc42224b8f66de` |
| [106-09 chapter9.pdf](https://github.com/TelemetryWorks/rcc-106-standards/releases/download/106-09/chapter9.pdf) | `461ae43340ff623212fdcd8bf1d6bc3fad12e479d0dc162e3f5534ae8e2e9ccc` |
| [106-11 chapter9.pdf](https://github.com/TelemetryWorks/rcc-106-standards/releases/download/106-11/chapter9.pdf) | `238b6469f5cac00e2d776b96869ed21414e1af6637e666b9f4cdf0a46a48d4e8` |
| [106-13 Chapter9.pdf](https://github.com/TelemetryWorks/rcc-106-standards/releases/download/106-13/Chapter9.pdf) | `33028a325390d013c4e694c8ead75b2ee0bc52786d2477ca795733b422bc2051` |
| [106-15 Chapter9.pdf](https://github.com/TelemetryWorks/rcc-106-standards/releases/download/106-15/Chapter9.pdf) | `f8c6ec06d389765ef783277e235ca1b8b9740fc88ca6299f464ca32ca24fb579` |
| [106-17 Chapter9.pdf](https://github.com/TelemetryWorks/rcc-106-standards/releases/download/106-17/Chapter9.pdf) | `9c726e5cd5e3883601fd70b18556ebef02a736c7a30e1909c33eccd12bebdaac` |
| [106-19 Chapter9.pdf](https://github.com/TelemetryWorks/rcc-106-standards/releases/download/106-19/Chapter9.pdf) | `0f0a7536e1858e31475535a592726f7943a085ccc2e7b3a033c20e8aa682047d` |
| [106-20 Chapter9.pdf](https://github.com/TelemetryWorks/rcc-106-standards/releases/download/106-20/Chapter9.pdf) | `add7a48f0ff0a1ef3840ecefcf9191dea2edb8605bc6de4dc981556d722c3019` |
| [106-22 Chapter9.pdf](https://github.com/TelemetryWorks/rcc-106-standards/releases/download/106-22/Chapter9.pdf) | `773bb32f54bf18e29a6cf4f5d5e998d094d668202889edb6ee56179bb424c6f7` |
| [106-23 Chapter9.pdf](https://github.com/TelemetryWorks/rcc-106-standards/releases/download/106-23/Chapter9.pdf) | `32b74561ac3e5842dfdff15a27c234ebcbb6817f8cfca3513eaf9897ace1b19a` |
| [106-24 Chapter9.pdf](https://github.com/TelemetryWorks/rcc-106-standards/releases/download/106-24/Chapter9.pdf) | `7e9253a9f2cbb2a55c9e3e702583c46fc9b0a8d69410928a491e9638036381c9` |
| [106-24R1 Chapter9.pdf](https://github.com/TelemetryWorks/rcc-106-standards/releases/download/106-24R1/Chapter9.pdf) | `dfc5ebfb3e6f02f58389126863d43a45d89c7e2d0533a1c690d9537531c740cb` |
| [rcc-123-16 123-16_IRIG_106_Chapter10_Programmers_Handbook.pdf](https://github.com/TelemetryWorks/rcc-106-standards/releases/download/rcc-123-16/123-16_IRIG_106_Chapter10_Programmers_Handbook.pdf) | `aedb41412f22283f2246e25c3b55befe5527af617ec83f8285e00b899f5191ab` |
| [rcc-124-13 124-13_TMATS_Handbook.pdf](https://github.com/TelemetryWorks/rcc-106-standards/releases/download/rcc-124-13/124-13_TMATS_Handbook.pdf) | `b83e071b43c1842cf5f84437a70b3275ff6af045a8ffa4a77f0def95ed2301aa` |
| [rcc-124-17 124-17_TMATS_Handbook.pdf](https://github.com/TelemetryWorks/rcc-106-standards/releases/download/rcc-124-17/124-17_TMATS_Handbook.pdf) | `8622c2789591ed891f02e0fce9121cf09fd3097b94412491dcdc62e2d00f7006` |
| [rcc-124-19 124-19_TMATS_Handbook.pdf](https://github.com/TelemetryWorks/rcc-106-standards/releases/download/rcc-124-19/124-19_TMATS_Handbook.pdf) | `f16ffedbc89b4bbb8f7c51e01706c9079330c4b1c753a87f1f3f37e3bf70ab5e` |
| [rcc-124-22 124-22_TMATS_Handbook.pdf](https://github.com/TelemetryWorks/rcc-106-standards/releases/download/rcc-124-22/124-22_TMATS_Handbook.pdf) | `99e47e0a945560466e628098717f254c30dcd16ab77fc045ee53abfa4376d228` |
| [106-05 chapter6.pdf](https://github.com/TelemetryWorks/rcc-106-standards/releases/download/106-05/chapter6.pdf) | `e4e032eade65b2124301bdd56d267c3a21849c071c08e6f9b512d469de9dd0cd` |
| [106-07 chapter6.pdf](https://github.com/TelemetryWorks/rcc-106-standards/releases/download/106-07/chapter6.pdf) | `08b5e9b277d74361cfea59d3037cd93d5cccaef3e27936aeed34367ff0cb7498` |
| [106-09 chapter6.pdf](https://github.com/TelemetryWorks/rcc-106-standards/releases/download/106-09/chapter6.pdf) | `6fa953068df57a868a301cf8000478e0885bc972c6b5370d837a357987794719` |
| [106-11 chapter6.pdf](https://github.com/TelemetryWorks/rcc-106-standards/releases/download/106-11/chapter6.pdf) | `f4b66ff1c6636bbcf106db33b4de6c068034df7e1235b6979af73cf25f6787ba` |
| [106-13 chapter6.pdf](https://github.com/TelemetryWorks/rcc-106-standards/releases/download/106-13/chapter6.pdf) | `6cb908f9fb59959b277f57e6aeea5d2f9f9ccef42ad366b3dde2e13682227a9d` |
| [106-15 chapter6.pdf](https://github.com/TelemetryWorks/rcc-106-standards/releases/download/106-15/chapter6.pdf) | `602984a66afa370b2f38040f0571cb0627ea89d342c2ce9edc78cc27dba216f8` |
| [106-17 chapter6.pdf](https://github.com/TelemetryWorks/rcc-106-standards/releases/download/106-17/chapter6.pdf) | `b8a3bc7159cd58d02e8e4ac0e4602873a4110689d852b09e55a820e5fed0ae1b` |
| [106-19 chapter6.pdf](https://github.com/TelemetryWorks/rcc-106-standards/releases/download/106-19/chapter6.pdf) | `f5bf451e2d0567f10cda2bf03c927b3bf1387b07693019e1d6c27814248f0abe` |
| [106-20 chapter6.pdf](https://github.com/TelemetryWorks/rcc-106-standards/releases/download/106-20/chapter6.pdf) | `02124882a382f41e80fbe31bfb20d7ac6252416100b4c3cb41da974b7313e30f` |
| [106-22 chapter6.pdf](https://github.com/TelemetryWorks/rcc-106-standards/releases/download/106-22/chapter6.pdf) | `5b260104b4ee9fb70f4d65948d7723435af4a2d6b2f264a9a1d934be3ea3c49e` |
| [106-23 chapter6.pdf](https://github.com/TelemetryWorks/rcc-106-standards/releases/download/106-23/chapter6.pdf) | `855442db01cbfd503904bfe245c2c5bde3ef724b35b3fee8b1a0c71258767a03` |
| [106-24 chapter6.pdf](https://github.com/TelemetryWorks/rcc-106-standards/releases/download/106-24/chapter6.pdf) | `c05ab889aa1b4886bcebdb9b814913c709812fb563f055654a536396c9ae70f2` |
| [106-24R1 chapter6.pdf](https://github.com/TelemetryWorks/rcc-106-standards/releases/download/106-24R1/chapter6.pdf) | `762d96347855f2d19289b71af03d6acb618f6534825400767a760bfcd8b8c893` |
| [106-05 chapter10.pdf](https://github.com/TelemetryWorks/rcc-106-standards/releases/download/106-05/chapter10.pdf) | `7aa84ef996df5ecd59da4159b82415ef2ba791a81dd20266c7d96a29adbf6bae` |
| [106-07 chapter10.pdf](https://github.com/TelemetryWorks/rcc-106-standards/releases/download/106-07/chapter10.pdf) | `50d125bc744001d803dcb8cadcb5c2038940ca980098afeb0ebcd25601d8e582` |
| [106-09 chapter10.pdf](https://github.com/TelemetryWorks/rcc-106-standards/releases/download/106-09/chapter10.pdf) | `1f93423591c43277009762c31c816358aead90bea9b570a4afc4c68e84ee0ec6` |
| [106-11 chapter10.pdf](https://github.com/TelemetryWorks/rcc-106-standards/releases/download/106-11/chapter10.pdf) | `8cef16928df5d3f93dfff6f95806e9923b41927413090b0844bf28313065c7a7` |
| [106-13 chapter10.pdf](https://github.com/TelemetryWorks/rcc-106-standards/releases/download/106-13/chapter10.pdf) | `e3740c7ac3099236f4e97cbe5b84c169340e6d4a4222769619e83ea0f8176fde` |
| [106-15 chapter10.pdf](https://github.com/TelemetryWorks/rcc-106-standards/releases/download/106-15/chapter10.pdf) | `cace968a4a796a56895eff36073840ae2db3a9c57dd33794d0c348e95fe0c204` |
| [106-17 chapter11.pdf](https://github.com/TelemetryWorks/rcc-106-standards/releases/download/106-17/chapter11.pdf) | `7297c782e0368e72ac783a72edccaa04eed28a8ea0e8ab7c61ea39379f516750` |
| [106-19 chapter11.pdf](https://github.com/TelemetryWorks/rcc-106-standards/releases/download/106-19/chapter11.pdf) | `ac06990ab1b98594cf70fd82815ae2d1efb6d7fb58bf329cf2dcc71a8b64ad9d` |
| [106-20 chapter11.pdf](https://github.com/TelemetryWorks/rcc-106-standards/releases/download/106-20/chapter11.pdf) | `e66370ce21b4ed44ed45f32e4785667398cf401e358f03e6f07494f06615748d` |
| [106-22 chapter11.pdf](https://github.com/TelemetryWorks/rcc-106-standards/releases/download/106-22/chapter11.pdf) | `e3e85ed09ddec95d49e9e1851ac4234b5df35b6340d9d91f355b1c598543d0a6` |
| [106-23 chapter11.pdf](https://github.com/TelemetryWorks/rcc-106-standards/releases/download/106-23/chapter11.pdf) | `ed30d02fe27cd89d9172ada824251fd6cd748bc87cb63c15f371152fa3274781` |
| [106-24 chapter11.pdf](https://github.com/TelemetryWorks/rcc-106-standards/releases/download/106-24/chapter11.pdf) | `d26e7926f4b2e7977df5e6210044cfc0d4c1191d45a66b71da2d530bf68de6a8` |
| [106-24R1 chapter11.pdf](https://github.com/TelemetryWorks/rcc-106-standards/releases/download/106-24R1/chapter11.pdf) | `0f63792dcac39f6010df371ee0afc46a396017918f4155cba9ebab6c2d6d263e` |

### Facts versus policy

Baseline Ch11 §11.2.7.2 Figure 11-34 pp. 11-50–51 supplies these facts:
Format 1 may span consecutive packets; sequence increments n+1; SRCC 0/1
asserts unchanged/changed; changed setup precedes changed data; a configuration
change event precedes the new setup. It does not print a duplicate-recovery,
identity-exception, quarantine or governing-target selection algorithm.
Those are the separately owner-signed INT-046–053 choices, not extra words
attributed to the standard. The 2026-11-25 sign-offs and final AI check at
#45 73396bc remain the authority for those choices.

Baseline Ch11 §11.2.1.1 f p. 11-8 makes sequence per-channel, modulo 256;
FF→00 is continuity. Sequence need not start at 00. Other-channel packets
neither interrupt nor join that channel's run under INT-011. Sequence alone
is not a record delimiter, trust proof or source partition.

Ch9 §9.4.2 p. 9-2 supplies attribute framing/case/order context; Table 9-2
G\SHA p. 9-13 and Ch6 §6.2.3.11 f pp. 6-22–23 supply checksum byte domain.
A digest excludes the raw SHA item; equal digests therefore do not prove
whole-text byte identity. INT-018–021/043 preserve damaged names, undefined
exclusions, placeholder/unsupported states, duplicates and mandatory warnings.
Packet data-checksum/secondary-integrity evidence remains core-owned and
independent of the TMATS digest. Neither upgrades the other.

| Historical packet source | Re-read Format 1/CSDW location and limit |
|---|---|
| 106-04 full volume | PDF pp. 306–307, §10.6.7.1 Figure 10-34 / Format 1 §10.6.7.2.2: shared CSDW reserved, not a source for SRCC/FRMT. |
| 106-05 | pp. 10-48–49, Figure 10-34b, PDF pp. 59–60: Format 1 CSDW reserved. Never turn a zero/reserved bit into an explicit unchanged claim. |
| 106-07 | pp. 10-71–72, Figure 10-50, PDF pp. 75–76: CH10VER and SRCC; no FRMT, bits 31–9 reserved. |
| 106-09 / 11 | pp. 10-66–67, Figures 10-50 / 10-54, PDF pp. 77–78: SRCC and FRMT. |
| 106-13 | pp. 10-65–66, Figure 10-54, PDF pp. 77–78: change-before-data and next-unchanged clearing context. |
| 106-15 | pp. 10-62–63, Figure 10-54, PDF pp. 75–76: same separate declaration/format evidence. |
| 106-17 | pp. 11-44–45, Figure 11-34, PDF pp. 52–53. |
| 106-19 / 20 | pp. 11-46–47, Figure 11-34, PDF pp. 54–55. |
| 106-22 / 23 / 24 / 24R1 | pp. 11-50–51, Figure 11-34, PDF pp. 58–59. |

Use the reviewed edition/CSDW profile and core evidence; do not decode modern
fields from reserved historical bits, infer a profile from the zero low byte,
or confuse packet-header version, Format 1 RCCVER and Format 4's table.
The intended fallback edition is not proof that a historical layout/registry
is implemented. Historical/nonzero source channels retain core's edition
rules and INT-028's reviewed source-partition limitation, not a new governor.
Ch6's later SHA-256 definition is absent/deferred in earlier inspected editions
as described in the checksum analysis; no historical algorithm is guessed.
RCC 123 packet checksums/reference examples and RCC 124 searches supply no
replacement adoption authority. Private/source-byte JSON representation is
not chosen here.

## Shared evidence pipeline

Keep independent bindings for supplied session/provenance, candidate identity,
borrowed raw spans and source extents, first-fragment SRCC availability, join/
closure evidence, field ambiguity/cut facets, SHA evidence and takeover proof.
Do not reduce them to a single usable/unchanged boolean. An interpretation
unavailable for one facet is not proof that another candidate is eligible.

A useful review order, without inventing a universal error-precedence API:
first reject/hold mutually inconsistent supplied observations (Q0); retain
actual independent facets; apply established field ambiguity and blocking cut;
assess only the bounded identity/checksum classification exceptions whose
premises are known; report raw comparisons/claims; consume separately reviewed
adoption evidence for QM. All findings survive. This is the approved rule
composition, not an extra parser/dispatcher or a governing inference.

The pairwise product assumes supplied, bounded observations. A partially
arrived follower is not B merely because its prefix matches. A premature-seal
finding can arrive before whole-copy identity is available; the bounded QI
exception is unavailable until its actual whole-text extents are established.
Retain candidate identity and revised classifications under INT-034/045; do
not invent lookahead timeouts, automatic EOF, hidden buffers or raw coalescing.
The cap/progress policy and versioned snapshots retain their own contracts.

## INT-046 / QI: bounded identity changes classification only

Premises are a time seal, otherwise-Complete bounded text, same channel,
sequence +1 modulo 256 and same available FRMT/RCCVER profile. Compare every
raw text byte, not projected names/values, sorted attributes or digest equality.
For following SRCC 0/1/U, identity can retain Complete with the closure basis
and identical retransmission label. Keep the located possible premature-seal
finding and both readings; Complete never means the doubt disappeared.
Every mid-session Complete keeps time-boundary seal (owner policy).

QI is subordinate to QD: a duplicate SHA or other established field ambiguity
is Ambiguous, even when bytes/digests agree. A blocking cut cannot become
Complete. The following-only digest verifies only its own domain, not the
sealed text. Adding the first SHA item changes item inventory (R), not just
an existing SHA value (H). No automatic adoption, rejoin or SRCC inference
comes from the identity label. Previous-copy identity under QM is not this
comparison unless the governing target is independently established.

## INT-047 / Q0: nominal product is not parser coverage

The matrix retains impossible tuples and labels the consistency conflicts.
Examples: B with inconsistent SHA observations; cut text with an authoritative
verified/mismatch claim; H without matching present checksum inventory; H/W
asserted as the only difference from a cut sealed candidate. Reject or hold
these supplied observations, never synthesize their bytes to make coverage
look exhaustive. A null baseline means no coherent valid-input verdict.

Even a nonconflicting tuple is only a candidate witness request, not proof
that all combined facts can occur in actual bytes. The future implementation
harness must construct and independently qualify each realizable witness;
new conflicts require visible reconciliation, not silently skipping rows.
Consistency/rejection tests are separately counted, never parser acceptance.
The documentation generator and runtime classifier cannot be their own oracle.

## INT-048 / QD: field ambiguity precedes rescue exceptions

Use INT-039's reviewed definition/cardinality and INT-038 identity; arbitrary
unknown/vendor items are not automatically scalar. Count every real occurrence
including exact repeats. A known duplicate G\SHA or other established field
ambiguity forces assembly Ambiguous independently for each candidate, with
or without a related follower. A digest, byte identity or edition fallback
cannot make it Complete. Diagnostic validation under INT-044's labelled basis
is not a declaration winner or adoption permission.

A cut plus ambiguity retains the cut with Incomplete's full blocking force.
Q25 also preserves proven leading loss despite a final semicolon. No caller
ambiguity opt-in or digest/identity exception bypasses cut. INT-031's approved
per-query ambiguity access is not automatic governing adoption. The documented
matrix's QD/no-new-adoption result stays distinct from that semantic-access
option; neither proves the recorder actually kept the old setup.
Explicit exact-ID duplicate removal/requalification remains allowed; don't
repair the original revision implicitly or carry its verdict to edited bytes.

## INT-049 / QB/QE: separation survives equality

Different channels, sequence gaps, changed FRMT/RCCVER, same-channel stops
and actual EOF do not acquire a QI extension merely because text matches.
Record equality as information and keep all boundary/profile evidence.
EOF comparison is explicitly across inputs, not a join after EOF. A FRMT
change retains the separately scoped ASCII/XML conformance evidence without
claiming XML interpretation; the approved formal mixture check is Complete-only
within an established session. Missing session evidence is not no mixture.

Unknown profile or continuity is not SS/+1 by default. Data timestamps or
unrelated time metadata cannot fill missing candidate extents. Core ownership,
trust and source partition still apply; matching bytes neither identify an
input as the same session nor choose which historical source governs.

## INT-050 / QC/QP: raw change is not semantic reset

Report H (existing SHA-value-only), W (raw layout/whitespace-only), an
independently established semantic difference, or other raw difference.
Classifying H/W requires independent evidence for that narrow subtype; no
blank trimming or generic order/case folding proves semantic equality.
Partial/cut comparison remains unresolved, not equal by matching prefix.
SRCC=0 with a real raw difference is reported rather than trusted as unchanged;
SRCC=1 with identical bytes keeps the discrepancy. No automatic time reset,
new association or takeover follows. Descriptive-only changes use INT-029's
approved semantic evidence, not an unrestricted COMMENT/date text guess.

## INT-051 / QH/QG: prominent integrity findings, guarded eligibility

For otherwise eligible candidates, the default is prominent integrity findings,
consistently initial and mid-session. A checksum mismatch alone is not silent
quarantine, and does not prove the old setup governs. Strict quarantine is
opt-in only after its own review; this analysis creates no such mode/rule.
F1, framing, core trust, unknown source scope, QD and cut are not waived by
warn mode. A failure of the sealed-digest exception may still produce INT-045
premature-seal Ambiguous; default integrity mode does not rescue it.
All original findings, raw occurrences and independent integrity domains remain.

## INT-052 / QS: unreadable SRCC remains unreadable

U is not zero, one, a majority of later fragments or an equality-derived bit.
Raw equality may still support QI's bounded sealed classification, while
eligibility/governing inference from unreadable metadata stays withheld.
The candidate's first-fragment evidence is retained; later bits do not repair
it or create a historical field absent from that edition's layout. A Complete
classification and an unavailable eligibility inference can coexist.

## INT-053 / QM: target provenance and history

Compare each new copy with the currently governing setup, if established;
it is neither always the original seal nor always the previous arrival.
A definite non-takeover does not become the target. An unresolved takeover
makes the target unresolved; do not select old/new by position, checksum,
SRCC, raw equality or a convenient default. QD's ban on new adoption is not
proof of which setup the recorded data actually used.

Record immediately previous-copy equality with its own pair of source IDs
as information only. It cannot affect eligibility, classification or target
selection, including during unresolved history. B->D compares D with G0 if
B definitely did not take over; H->B may mean B equals the unadopted previous
H, while the governing comparison remains H(G0). A third copy adding the
only checksum verifies only itself and changes inventory, never the earlier
copies or their target. D in these sequences denotes semantic difference;
SHA-status D separately denotes duplicate checksum.

Unresolved is not permanently absorbing: a later independently established
valid takeover can resolve the next target to that copy, preserving unresolved
history and all earlier findings. A definite non-takeover during unresolved
history does not resolve it. INT-028(b) requires a separately reviewed rule
that actually proves takeover from an unresolved target; these fixtures may
supply explicitly reviewed proof as a boundary input, not invent that proof
from Complete/good digest/equal bytes. No new automatic recovery rule is
implemented or qualified by a synthetic proof adapter.

Keep prior governing setups and packet/source references for already decoded
data. Later target changes never relabel old packets or overwrite old findings.
The up-to-three-later-copies fixture bound is not a production memory/history
limit. Retention accounting, per-query access and publication remain separate.

## Artifact audit, not runtime verification

Streamed the checked-in members without importing spec.py's policy function.
Checked independent QD/QI implications, case-ID uniqueness, class IDs, counts
and preserved findings; this is a documentation consistency audit only.
No parser, assembler, runtime state machine, witness constructor, external
reference or corpus was executed. Compressed artifacts remain byte-identical.

| Observed artifact rule bucket | Members |
|---|---|
| R3B | 30 |
| R1T | 27,510 |
| R0 | 59,184 |
| R1A | 28,296 |
| R2D | 13,824 |
| R3V | 156 |
| R3A | 600 |
| R1X | 10,368 |

Total 139,968 members / 39,072 classes. R0 has 59,184 inconsistent tuples;
80,784 remaining tuples still need independently qualified byte witnesses.
Within the B/time/C/SS/otherwise-T guard, all 36 coherent members retain
possible_premature_time_boundary_seal_both_readings: 30 R3B Complete and
6 duplicate-SHA Ambiguous. These numbers describe artifacts, not coverage.
All nominal owner-question columns are empty after the recorded sign-offs;
that does not complete source, analysis, concrete schema or executable gates.

| Unmodified artifact | SHA-256 |
|---|---|
| `classes.json.gz` | `9f20a4c3d597a17d5090b8c2311c4c1c27e29feee6dbf88a26a1641df8593195` |
| `manifest.json` | `7b12fb789a6b84572a00556e913a04dc04a4743c3cb91baa23c9e369239a7924` |
| `members.jsonl.gz` | `9d082cd66859aef8b8191eb91caca989f9c189f961cc85a5d9bf3fc7e481a5f4` |
| `spec.py` | `f25f6f6b10bb0cd3d9aaf40663533941900bc4714d3ee596ac353b5a867d211c` |

## Planned compound coverage

These supplement, not replace, the full L3-REG-014 product and existing eleven
L3-REG-016 QM sequences. Future tests use independently fixed bytes, source
ranges and typed control evidence, never expected outcomes from spec.py or
the production classifier. No input is called parser-covered merely because
its documentation row or a synthetic control adapter passes.

| Planned case | Independent expected evidence | Requirements / interpretations |
|---|---|---|
| `retx_analysis_srcc0_differs_is_visible` | Bounded distinct whole raw texts plus SRCC=0 retain changed-claim discrepancy and subtype; no assumed unchanged or time reset. | L3-REG-015; INT-050 |
| `retx_analysis_srcc1_identical_is_not_takeover` | Bounded QI identity with SRCC=1 can preserve otherwise-Complete seal, with label and premature-seal finding, never automatic takeover. | L3-REG-015; INT-046/050 |
| `retx_analysis_following_only_sha_is_not_sealed_proof` | Only following copy adds the first verified SHA: inventory difference R, not H; no sealed verification or classification exception supplied by that digest. | L3-REG-015; INT-046/051 |
| `retx_analysis_sealed_only_sha_stays_domain_bound` | Sealed-only verified digest supports only the eligible sealed-text exception with its own guards, not verification/adoption of the following copy. | L3-REG-015; INT-046/051 |
| `retx_analysis_identity_all_srcc_values` | QI guards independently fixed; following 0/1/U cannot change raw identity classification; every result retains INT-045 finding, and U stays unreadable. | L3-REG-015; INT-046/052 |
| `retx_analysis_identity_is_not_prefix` | Matching prefix with incomplete following extent never proves whole B identity; comparison remains unresolved and no QI rescue is fabricated. | L3-REG-015; INT-046/050 |
| `retx_analysis_qd_dominates_identity` | Equal raw texts each contain duplicate SHA: both Ambiguous, all occurrences/identity/premature-seal evidence kept; no Complete upgrade. | L3-REG-015; INT-046/048 |
| `retx_analysis_qd_without_follower` | Known duplicate SHA and no related follower still forces assembly Ambiguous; digest agreement is irrelevant. | L3-REG-015; INT-048 |
| `retx_analysis_other_scalar_ambiguity` | Reviewed scalar duplicates other than SHA force QD; equal unknown-cardinality private items alone do not assert QD. | L3-REG-015; INT-048 |
| `retx_analysis_cut_and_ambiguity_cannot_opt_in` | Proven leading or trailing cut plus duplicate ambiguity retains blocking cut under final semicolon, identity, digest and per-query ambiguity acceptance. | L3-REG-015; INT-048 |
| `retx_analysis_r0_is_consistency_not_parser` | Cut text with authoritative V/F, or B with conflicting SHA observations, receives separately counted rejection/hold; never a fabricated byte witness. | L3-REG-014; INT-047 |
| `retx_analysis_nonconflicting_row_still_needs_witness` | Nominal non-R0 tuple with no independently qualified bytes remains a coverage gap rather than a passing parser test. | L3-REG-014; INT-047 |
| `retx_analysis_gap_rollover_are_distinct` | FF to 00 is same-channel +1; gap/other sequence is not. Raw identity across the gap is information, not QI or a rejoin. | L3-REG-015; INT-046/049 |
| `retx_analysis_other_channel_does_not_extend` | Different channel with equal bytes retains independent channel/run evidence and no identity-based extension or governing partition guess. | L3-REG-015; INT-049 |
| `retx_analysis_profile_unknown_not_ss` | Unavailable historical layout/FRMT/RCCVER is not same-profile SS; reserved 04/05 bits do not become SRCC=0. | L3-REG-015; INT-049/052 |
| `retx_analysis_profile_change_keeps_mixture_scope` | Different FRMT/RCCVER prevents QI. Non-Complete XML observation stays qualified; formal mixture finding requires Complete/session evidence. | L3-REG-015; INT-049 |
| `retx_analysis_stop_or_eof_no_hidden_join` | Equal texts across same-channel stop or actual EOF retain equality only; EOF comparison is explicitly cross-input. | L3-REG-015; INT-049 |
| `retx_analysis_sha_and_layout_delta_not_semantics` | Independently established H/W subtype changes no clock/reset/association rule and never implies whole raw equality. | L3-REG-015; INT-050 |
| `retx_analysis_integrity_default_keeps_guards` | Otherwise eligible initial/update integrity mismatch stays prominent without automatic quarantine/old-setup selection; F1/trust/cut/scope remain blockers. | L3-REG-015; INT-051 |
| `retx_analysis_no_unreviewed_quarantine` | Request for unreviewed strict quarantine cannot silently replace the signed default policy. | L3-REG-015; INT-051 |
| `retx_analysis_unreadable_srcc_equality_not_bit` | U plus equal text preserves equality; no SRCC bit, eligibility or governing target is inferred, including from later readable bits. | L3-REG-015; INT-052 |
| `retx_analysis_governing_vs_previous_pairs` | Fixed G0, definitely unadopted C1 and C2: distinguish governing comparison from previous equality in B->D and H->B sequences. | L3-REG-016; INT-053 |
| `retx_analysis_unresolved_then_proven_takeover` | Unresolved C1, independently reviewed valid C2 takeover, then C3: target resolves to C2 with earlier findings/unresolved history retained. A merely Complete/good C2 is insufficient. | L3-REG-016; INT-053 |
| `retx_analysis_unresolved_then_definite_non_takeover` | Unresolved C1 then definitely unadopted C2 leaves C3 target unresolved, not C2 or guessed G0. | L3-REG-016; INT-053 |
| `retx_analysis_previous_equality_mutation` | Hold G/candidate/control evidence fixed and change only previous-copy equality; classification/eligibility/target stay unchanged. | L3-REG-016; INT-053 |
| `retx_analysis_qd_does_not_prove_old_governor` | QD Ambiguous blocks new adoption but unresolved actual recorder takeover does not establish that old G0 governed later bytes. | L3-REG-016; INT-048/053 |
| `retx_analysis_history_survives_takeover` | G0-decoded data, valid G1 takeover, then unresolved copy: preserve every prior finding and original packet-to-governor/source reference. | L3-REG-016; INT-053 |
| `retx_analysis_raw_span_partition_invariance` | Independently fixed whole raw texts across DATA span/fragment partitions preserve classifications/comparisons with zero required coalescing and unchanged core evidence. | L3-REG-015; INT-046/049/050 |
| `retx_analysis_pending_extent_cannot_publish_identity` | Open follower lacks bounded whole identity; candidate revision and premature-seal evidence stay observable without an invented timeout, EOF or automatic governing use. | L3-REG-015; INT-046/050 |
| `retx_analysis_all_guard_findings_survive` | Combine located 02/uppercase/name spelling, F1 and cut/BOM blockers: no early return drops warnings, chooses an exclusion or upgrades core integrity. | L3-REG-015; INT-046/048/051 |

Bounded CI property families vary SRCC independently of byte identity; mutate
only previous-copy equality; partition fixed raw text without changing extents;
combine field ambiguity/cut/integrity facets; vary historical profile labels
without aliasing unavailable fields; exercise every listed product member
with consistency versus byte-witness accounting. Source, case IDs, mappings,
readings, locations and original findings stay independently specified.
Actual streaming budgets/retained-pool progress remain the approved resource
contract, not a matrix-derived numeric default. Real recordings stay local-only
and were not accessed. No new dependency or reference revision is chosen.

## Executed checks, review and next work

Locally executed: fresh manifest/PDF hash verification, targeted packet passage
re-extraction/read and shared checksum-source review, independent streaming
artifact audit, literal member/guard counts and unchanged-artifact hashes,
planned case/link/metadata checks, trace regeneration/--check, diff --check.
Source jobs finish before reporting. Heavy implementation, property/fuzz and
real compatibility checks belong in bounded GitHub CI; none ran locally.
No code, matrix regeneration, fixture witness implementation, publishing,
credential/settings change or prototype retirement occurs here.

Only INT-046–053 Analysis fields link this draft; Design/Review remains unchanged.
Owner/independent analysis review and intensive executable verification remain
pending. Other dependency INT analyses are not completed by these context reads.
Family two remains incomplete; the ordered ADR-0035 restructure has not started.
Continue related approved-INT analyses and remaining concrete schema/request/
strict-class derivations. Do not claim readiness from documented matrix counts,
shared source checks, a prototype CI pass or a synthetic adoption adapter.
