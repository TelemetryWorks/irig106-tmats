# INT-043 Option B owner decision — 2026-11-23

**Owner decision recorded, AI recheck of the amended wording pending.**
Joey chose Option B after #31 was merged at head
`43da06c33020fa50fbe63564594d328a572563d9` (merge `7bf3155`). This overrides
the strict recommendation; the owner will route the amended wording for an
independent AI check, and final approval will be recorded afterwards.

## Decision and rationale

Accept a nonempty all-ASCII-digit algorithm designator whose canonical digit
string equals 2: strip leading zeroes, retain 0 for an all-zero spelling,
and compare strings, with no integer conversion (the INT-038 approach).
Thus 2-, 02-, 002- and arbitrarily long leading-zero variants identify
algorithm 2 and may verify positively when the digest and other prerequisites
succeed. This is the owner's explicit value-token decision, not an automatic
extension of index semantics.

Every accepted spelling other than 2- always emits a located warning named
NONCANONICAL-DESIGNATOR, including when the digest mismatches. It cites
106-24R1 Chapter 6 section 6.2.3.11 f, page 6-22 (constant string 2-) and
Chapter 9 Table 9-2 MESSAGE DIGEST/CHECKSUM, page 9-13. Keep raw bytes and
spelling. Upper-case digest warnings remain independent under INT-020.
The rationale is tolerance of real-world writers with visible noncanonical
spelling, rather than losing otherwise useful checksum verification.

The rejected strict alternative supported only exact 2- and reported 02/002
unsupported. It relied on Chapter 6's printed constant and avoided adding
numeric equivalence beyond the representation. Joey preferred tolerance
plus an always-emitted warning, explicitly overriding that recommendation.

Other numeric values, including zero/all-zero and 20/0020, remain unsupported;
never coerce them to 2. The no-hyphen G\SHA:0; remains the separate unverified
placeholder. Stamping always writes canonical 2- and lower-case hex.
Leading-blank 2- remains unavailable; an en dash is Invalid under INT-037;
+2- and 2.0- are malformed; upper-case hex follows INT-020. A defined raw
exclusion still permits COMPUTE independently of stored-value verification.

## Evidence still pending

The amended Option B wording has no independent AI concurrence yet. The
prior strict-option review is not approval of this choice. Requirements and
TEST-DATA cases remain planned; the register and trace show owner decision
recorded with recheck pending, not final approval or implementation.
Source freshness, full development analysis and executable tests remain.
The earlier checksum draft is preserved as historical alternatives/review;
this dated owner decision supersedes its strict recommendation.
