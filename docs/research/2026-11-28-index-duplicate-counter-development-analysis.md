# Development analysis: INT-038/039/040 — 2026-11-28

Written maintainer analysis for three APPROVED interpretations: canonical
index identity, scalar multiplicity and structural counter membership. Their
decisions stay unchanged. The analysis itself still needs owner/independent
review and intensive executable tests. It is not parser implementation or
completed development analysis for any other entry.
Inspected main d8b18764e5c11c30f234b51200cbe4f98de5ce96 after #84; #85 merge reported by the coordinator, not present in this inspected snapshot.

## Source freshness and scope

Re-fetched the standards archive manifest on 2026-11-28; recomputed all 19
PDF hashes against it before re-reading relevant passages. Fourteen Chapter 9
editions span 106-04..24R1; five companion handbooks supply context. Full
hashes and URLs below identify the actual bytes. PDF text locates the printed
code-name cells/paragraphs; wrapped columns were read as columns, not accepted
as repaired TMATS input. No rendered-image comparison is claimed.

| Source / archive release asset | SHA-256 |
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

For every listed Chapter 9 edition, re-read 9.4.2 pp. 9-2–3: unique names,
single versus hyphen-numbered entries, free attribute order and standalone
COMMENT. These define structure, not leading-zero identity, integer width,
duplicate recovery or validity-based membership filtering. Those are the
project's approved choices, not newly discovered mandates.

Baseline source bearings (106-24R1): 9.5.1 a/Table 9-1 p. 9-5 for 1..N and
no missing values; Table 9-2 G\PN p. 9-10 and G\COM p. 9-13; Table 9-4
R-x\TK1-n p. 9-48 and R-x\COM p. 9-84; Table 9-7 D-x\MN-y-n p. 9-115,
D-x\MML\N-y-n / D-x\MNF\N-y-n-m / D-x\WP-y-n-m-e p. 9-117 and WFP
value uniqueness p. 9-118. Section 9.5.14 p. 9-172 supplies X occurrence
noncontiguity and original-index association. None chooses a checksum winner
or allows unknown private syntax to define a counter automatically.

## Cross-edition source differences

These are the actual D-group code-name rows, not diagram references alone.
The counter identity uses x/y/n/m and WP adds e in the inspected hierarchy;
field spelling/source column layout must not flatten those separate scopes.

| Edition | D MML/MNF/WP source row(s) | Evidence boundary |
|---|---|---|
| 106-04 | Table 9-6 p. 9-54 (full PDF p. 232) | Maximum field sizes, not the later printed range metadata |
| 106-05 | Table 9-6 p. 9-58 | Maximum field sizes, not modern ranges |
| 106-07 | Table 9-6 p. 9-65 | Maximum field sizes, not modern ranges |
| 106-09 | Table 9-6 pp. 9-73–74 | Location count row precedes fragment count/WP row; maximum field sizes |
| 106-11 | Table 9-6 p. 9-61 | Maximum field sizes |
| 106-13 | Table 9-8 p. 9-57 | MFS 4 for location count and MFS 1 for fragment count are not ranges |
| 106-15 | Table 9-7 p. 9-114 | Printed ranges 1–9999 locations / 1–8 fragments |
| 106-17 | Table 9-7 p. 9-114 | Printed ranges; general no-missing-values text located at 9.5.1 a p. 9-5 |
| 106-19 | Table 9-7 p. 9-114 | Same inspected count hierarchy/ranges, no imported edition equivalence |
| 106-20 | Table 9-7 chapter-relative p. 114 | PDF prints chapter-relative number; count hierarchy/ranges retained |
| 106-22 | Table 9-7 p. 9-116 | Same inspected hierarchy/ranges |
| 106-23 | Table 9-7 p. 9-116 | Same inspected hierarchy/ranges |
| 106-24/24R1 | Table 9-7 p. 9-117 | Baseline rows used by current planned fixtures |

The explicit no-missing-values sentence was located in inspected 106-17+
9.5.1 a, not the earlier logical-format/attribute-description text. The
X-x/ORIGCODE exception was located in 106-17/19/20 p. 9-156, 106-22 p. 9-170,
106-23 p. 9-171 and 106-24/24R1 p. 9-172; no equivalent clause was located
in the earlier Chapter 9 text search. This is not proof that arbitrary
historical extension grammar is available, or permission to apply baseline
contiguity/range rules without a reviewed basis. Known source gaps remain
visible. Historical table numbering and MFS must not masquerade as modern
range evidence; numeric conversion policy does not supply a missing range.

Older G/R rows and their cardinality context were also inspected. Raw group
COM fields remain distinct from standalone COMMENT. Neither the common
COMMENT prose nor later scalar tables specify equal-repeat recovery. The
owner explicitly chose collection versus scalar ambiguity, so no handbook
or older sample is used to silently reopen that policy.

RCC 123-16 p. 2-3 describes a single-entry attribute appearing once and
multiple entries identified by hyphen-numbered identifiers. RCC 124-13
p. 2-2 and 124-17/19/22 p. 2-6 describe indexed entry types; relevant counter/
measurement discussions and examples were searched/read alongside these.
They corroborate the distinction but do not decide leading zeros, damaged
membership or which duplicate wins. No sample parser implementation was
copied, run or treated as authority. No old size statement becomes a new
project cap. The source hashes above cover the handbook editions inspected.

## INT-038: identity without repair or numeric allocation

Structural identity uses reviewed index POSITIONS after INT-036 name
projection, not every digit substring. TK1, MF1 and G\106 contain literal
keyword digits; only declared hyphen indices get canonical digit strings.
Keep position order and path segments: D occurrence x, list y, measurement n,
location m and fragment e are not one undifferentiated trailing number.
Unknown H/V/X-private grammar does not acquire that structure by resemblance.

A nonempty digit token loses leading zeros only in its identity projection;
all-zero strings retain canonical '0'. Source spelling and source intervals
remain. Equality/hash use canonical strings, not u32/u64/usize or big integer
conversion; no array/loop is sized by the represented number. Hash equivalence
must hold under the same hasher; no stable serialized hash value is promised.
Zero remains readable identity with a separate domain finding where required.
The ten-RAW-digit rule for FIELD VALUES in INT-055 does not limit indices.

Per-token noncanonical information compares projected digits with canonical
digits; controls between digits independently trigger INT-036 omission.
Collision information is a separate full-key grouping: two DISTINCT raw
index-spelling tuples trigger it, then EVERY occurrence is listed, including
exact repeats. Letter case/value differences alone do not change the tuple.
Original token spellings include their retained internal omitted bytes;
projection must not erase that evidence or substitute projected spellings.
Whole-name edge layout is not an extra index token. Internal/adjacent SP
cannot be trimmed into an index, and a high byte cannot be dropped to form one.

X/original carried-index comparisons use the same identity when a reviewed
mapping establishes the positions, but X and D remain different full keys.
This neither resolves X attachment nor approves INT-061/062 private splitting.
A new canonical spelling collision is observable before scalar cardinality
is available; it is not postponed until the duplicate or counter passes.

Rejected alternatives: numeric machine types make identity target-dependent;
raw spelling identity separates 01 from 1 contrary to the owner's decision;
big integers add unnecessary conversion/storage; silent rewriting loses the
hash/edit source; coalescing the collision group hides the evidence.

## INT-039: cardinality, not value agreement, determines ambiguity

Bind cardinality to a reviewed definition/edition. A missing numeric suffix
alone does not establish single-entry semantics for arbitrary unknown names.
Within one document, compare the complete canonical key; each full indexed
slot may still be scalar. Two occurrences remain two even if raw values,
projected values or digest/equality evidence agree. An invalid/empty candidate
is not filtered to create a valid-looking unique value; defaults never replace
multiplicity. Keep item IDs, spans, order and candidate-specific findings.

Standalone COMMENT is an ordered collection by owner policy, not an explicit
standard guarantee of repeated-comment semantics. G\COM/R-x\COM remain scalar;
long remarks split across duplicate group fields stay Ambiguous, without
joining. All raw lines survive. Unknown cardinality stays unclassified, not
valid or scalar by guess. INT-038 collision and INT-039 duplicate findings
are independent: one raw digit-spelling tuple can still be duplicate, and
unknown cardinality does not suppress a known canonical-spelling collision.

One assembled Format 1 record is one document, not one per fragment. The
cross-fragment fixture explicitly supplies same channel, sequence +1 modulo
256 and matching FRMT/RCCVER, with no intervening layered closure boundary.
INT-011/045 still decide runs/boundaries; this analysis does not invent them.
Repeating G\PN across those fragments is a duplicate; the same field in two
separate records is not. Never join them for counting because bytes agree.

INT-048 makes established field ambiguity assembly Ambiguous, regardless of
identity/digest; retain cut facets with Incomplete's full blocking force.
F1 separately makes duplicate SHA verification indeterminate and refuses stamp;
field Ambiguous is not that checksum verdict and cannot erase the repair-by-
exact-item option. Raw extraction and diagnostic inventory remain distinct
from owner-held semantic/governing access. No new rendering/severity policy.

Rejected alternatives: first/last wins changes results under reorder; equality
coalescing hides multiplicity; valid-only filtering hides damaged declarations;
joining group COM remarks repairs text without permission. The approved cost
is refusal of a convenient singular result for benign repeats until explicit
correction, not data loss or a global parsing failure.

## INT-040: membership, counter usability and scope uncertainty

The reviewed member rule specifies literal group/path, exact index arity,
parent positions and governed position. Several property witnesses or repeated
spellings of one member contribute one canonical identity, even with Invalid
payload or an Ambiguous property slot. That observation is separate from
all field, domain and record findings. A matching count is not validation
success. Count cardinality and 1..N coverage/domain independently: N=2 with
{1,3} is not acceptable merely because its size is two; zero/out-of-domain
members remain visible rather than removed to make the set pass.

The counter's own applicable effective value must be usable. Missing,
Invalid, Ambiguous or unavailable counters block evaluation; do not infer N
from the largest member or line count. MML=0 and MNF=9 are outside their own
BASELINE ranges; a parsed digit sequence alone does not make either usable.
A cited reviewed default, if any, comes from its separate effective-value
rule. INT-055 raw numeric syntax and bounds are distinct from INT-038 indices.

Malformed evidence is relevant only when separately framed projected literal
group/path matches the declared rule. A bad governed token with readable
parents blocks that known scope. An unreadable parent or wrong arity becomes
unassigned rule-level evidence, not a guessed member. Every compatible scope
is partial/cannot-evaluate EVEN when its already known set matches N. Use
only unambiguously mapped readable parent facts; no guessed alignment or
enumeration of hypothetical parent values can hide that guard.

For D-1\WP-1-1-x-2, scopes (1,1,1,1) and (1,1,1,2) are compatible while a
scope with x=2 is contradictory. The bad m position cannot be guessed from e.
Literal-family mismatch, value-contained code-like text, unframed suffix or
an X extension does not create a D member. Do not import INT-042's damaged-
literal presence-predicate wildcard into this exact-literal counter rule.
Other malformed-name findings remain; this rule does not certify that all
possible damaged names were recovered or the whole document is conforming.

Child counts can use explicit parent indices despite an ancestor's domain
violation/unusable count: m=0 or m above the parent count does not hide the
child's independent MNF evaluation. 0 and 00 form one parent identity. This
never makes the parent conforming. X attaches but creates no original D
member; absent original is a link problem, extension-only indices stay
unclassified. WFP VALUE uniqueness/order is not member-identity cardinality
and remains a separate rule, including proposed INT-033/default collisions.

Missing-member diagnostics use bounded observed sets/gap evidence, never
iteration to a 40-digit index or unbounded error expansion. Resource limits
remain separately observable, not cannot-evaluate silently reported as pass.
No count repair, renumbering, source truncation or forced coalescing is added.
Counter normalization still needs its own owner-approved transform policy.

Rejected alternatives: valid-only counting makes broken entries disappear;
line/property counting inflates N; max-index guessing conceals gaps; deleting
zero/out-of-range members manufactures success; ignoring compatible malformed
parent evidence falsely certifies a partial set. Approved structural counting
keeps both usable structural facts and independent uncertainty/invalidity.

## Independent edge oracles and intensive-test plan

All cases below are PLANNED, not executed parser/validator tests. Independently
list candidate IDs, raw spans, canonical keys, parent sets and verdicts before
production code exists. Do not derive expected membership from its matcher.

| Planned case | Independent expected boundary |
|---|---|
| index_identity_40_digit_pair | 01234567890123456789012345678901234567890 and its 40-digit nonzero suffix have equal canonical identity/hash input, original spellings retained; no integer conversion |
| index_zero_is_not_field_value_zero | Arbitrarily long all-zero INDEX -> identity 0; an eleven-digit numeric FIELD VALUE is Invalid under INT-055; neither grants range validity |
| index_collision_lists_every_occurrence | D-1\MN-1-2, D-01\MN-01-02, D-1\MN-1-2: one collision group listing all three, not just two distinct spellings |
| index_case_only_has_no_spelling_collision | Case-only key variants with the same digit tuples: no INT-038 collision finding, scalar duplicate still Ambiguous |
| index_internal_control_keeps_raw_tuple | R-1 CR LF 0\TK1-1 and R-10\TK1-1: projected identity 10, internal-omission evidence and distinct raw-token collision retained, no projection-based loss of witnesses |
| scalar_valid_invalid_candidates_stay_ambiguous | Known scalar with one usable and one Invalid/empty occurrence: both kept, no winner/default |
| scalar_comment_and_group_com_differ | Repeated standalone COMMENT is collection; repeated G/R COM same slot is Ambiguous even when equal; raw all retained |
| scalar_fragment_scope_is_one_document | Explicit same-channel +1 modulo 256, same FRMT/RCCVER and no closure: cross-fragment G PN duplicates; separate records do not |
| counter_invalid_payload_is_member_evidence | N=2 with WP identities 1/2 and first payload FF: structural set {1,2}, Invalid value remains; counter [0,18), value [16,17), bad payload [33,34) |
| counter_duplicate_properties_count_once | WP/WI properties and exact/01 member repeats contribute one e identity; field ambiguity and collision findings still retained |
| counter_malformed_parent_blocks_compatible_scopes | D-1\WP-1-1-x-2 blocks all compatible (1,1,1,m), even known set {1,2} already matches N=2; x=2 parent scope unaffected by this evidence |
| counter_wrong_arity_never_guesses_alignment | Matching literal family with wrong index arity stays rule-level unassigned; only unambiguous readable positions constrain compatible scopes |
| counter_unrelated_damage_is_not_member | Unrelated literal family, code-like value text, unframed suffix and X original-path text are not D-member evidence; other findings remain |
| counter_bad_parent_does_not_hide_child | m=0/00 is one scope; parent-domain violation/unusable MML does not suppress a child with explicit indices and independently usable MNF |
| counter_range_is_not_mfs | Older printed MFS 1 is not baseline MNF range 1–8; applied baseline versus intended historical basis is labelled, unsupported edition mappings stay unavailable |
| counter_cardinality_is_not_contiguity | N=2 set {1,3} and N=3 set {1,3} both fail their respective checks; no max/count substitution, no eager array allocation |

The invalid-payload byte oracle is the existing corrected fixture:
D-1\MNF\N-1-1-1:2;D-1\WP-1-1-1-1:<FF>;D-1\WP-1-1-1-2:4; (<FF> one byte).
Use independently fixed bytes/IDs, not decoded values from production code.

For EVERY source edition above, bind a synthetic source-qualified fixture to
its own table/field metadata, preserving unsupported historical rules rather
than calling one baseline fixture universal. Add bounded properties over
leading zeros, ASCII case/control/name projections, input permutations,
repeated candidates/property witnesses, parent scopes and malformed-index
placement. Assert canonical equality/hash congruence and raw replay; scalar
ambiguity is invariant under reordering while evidence order follows input;
adding a duplicate property never creates a distinct member. Compatible
unassigned evidence can never improve a partial verdict to pass.

Cover every named test in the three register entries plus the edges above,
all character/high-byte/blank branches, parent/domain versus child/value
findings, X exclusion and independent checksum/assembly gates. Fuzz work
stays bounded in GitHub CI with the existing resource contracts; no new
threshold, test dependency or local heavy campaign is approved.

## Conclusion and remaining gates

The inspected sources support keeping INT-038/039/040 as approved, with
source absence/version differences explicitly qualified. No interpretation,
policy, range, cardinality or existing owner gate is changed by this draft.
The three Analysis links now name written work, not reviewed/implemented
behavior. Owner/independent analysis review and intensive executable tests
remain; all other outstanding analyses remain separate tasks.

Executed locally: 19 SHA checks, bounded PDF text extraction/search and
passage/cell reading, literal byte/span/set oracle consistency and doc/trace
checks only. No Rust build, parser/validator/fuzz/reference execution, real
recording access or dependencies installed. Real sample checks remain local-
only and outstanding where relevant/available; none is claimed in this cloud
assignment. The current trace still has no executable or verified redesigned
coverage. Stop at independent exact-head review before further implementation.
