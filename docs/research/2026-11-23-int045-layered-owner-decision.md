# INT-045: initial sign-off and owner mid-session closure extension

Date: 2026-11-23. Owner: Joey. Initial Option A signed under ADR-0033;
independent AI CONCURS on #40 at 93eb6e2. The mid-session extension is the
owner's decision, overriding the original recommendation. Status:
**owner decision recorded (initial: A signed; mid-session: owner extension);
AI recheck of the amended wording pending**. No implementation is claimed.

> INT-045 OPTION A is APPROVED for the INITIAL setup record.

> a mid-recording setup record (SRCC=1) is ALSO closed by the next trusted recognised time packet, the same rule as the initial record.

## Separate layers and safety consequence

INT-011 remains the reconfirmed per-channel joining rule: +1 modulo 256,
matching FRMT/RCCVER, other-channel traffic never interrupts that joining.
INT-045 is a separate layer that closes record-candidate phases on a time
packet. It does not rewrite INT-011's predicates; the register adds only a
cross-reference there. Candidate phase identity prevents a later otherwise
joinable fragment from crossing backward over a closure.

The initial phase closes immediately before its first eligible trusted time
packet. A mid-session SRCC=1 candidate similarly closes before the next
trusted recognised time packet on any channel. Repeated SRCC=1 fragment bits
alone do not restart that candidate; disagreement remains retained and needs
its own SRCC-summary interpretation. SRCC=0-only candidates are not silently
included in this owner extension. Data interleaved mid-session is not by
itself a terminator or an initial-prefix ordering violation.

At closure, INT-034 assesses the text actually observed. For example:

| In recording order | Retained consequence |
| --- | --- |
| channel 0 setup q=10, SRCC=1, text `COMMENT:up` | Active update candidate, 10 text bytes |
| trusted recognised time packet, channel 42 | Seal the candidate; cut attribute -> INCOMPLETE with a located finding |
| channel 0 setup q=11, matching FRMT/RCCVER, text `date;` | Continuous-but-split relation to the sealed candidate; retain both sides and boundary evidence, NEVER rejoin into `COMMENT:update;` |

A boundary does not prove valid metadata, a unique document or governing
configuration. Repeated scalar/split uncertainty remains; later fragments
are not appended backward to invent duplicates or a completed digest. The
continuous-but-split observation also applies if the first candidate ended
at `;`; continuity alone does not prove whether a later candidate is a valid
new record or a continuation. No implicit winner, split repair or old-record
fallback is authorised. INT-028/031 governance and the rest of INT-034 remain
separately subject to review. General pre-boundary snapshots remain undecided.

## Guard refinements carried from the independent review

- Known session start is a CALLER ASSERTION retained as labelled provenance,
  not inferred from file offset, filename, sync, q=0, SRCC or packet contents.
  Each asserted session start begins a distinct initial phase. The source's
  session/recording wording remains visible; file boundaries infer neither.
- Before the initial first time packet, **every non-Format-1 packet** is an
  ordering violation with a finding and withheld adoption. This includes
  Computer-Generated Formats 0/2/3/4 as well as ordinary data. Mid-session
  interleaving is outside that initial-only prefix restriction.
- An untrusted initial FIRST time packet withholds adoption; retain that
  first candidate and never replace it with the next trusted time packet.
  Earlier untrusted observations also remain blocking evidence for a mid-
  session candidate; later trust cannot erase a gap or corruption finding.
- Eligible types are 0x11 and, only where defined from 106-17, 0x12. Before
  106-17, 0x12 is reserved/ineligible. IPTS and secondary-header timestamps
  never count. Source NONE can still identify a time packet; clock availability
  is distinct from structural packet trust. Framing/bounds and applicable
  packet checksums must be trusted.
- A time packet on channel 0 in 106-17+ receives a separate channel-use
  conformance finding (Chapter 11 section 11.2.1.1 b). It is not silently
  reported compliant or reinterpreted as setup; the closure rule is on any
  channel. Format/version support remains separately reviewed.
- With no eligible marker, live candidates remain open/provisional and
  budgeted; actual EOF keeps its labelled INT-034 outcome. Source NONE,
  resource exhaustion and pauses do not manufacture EOF or hide missing time.

## Archived 106-13 citation with an unambiguous fingerprint

Read [the archived RCC 106-13 Chapter 10 PDF](https://www.spiraltechinc.com/otis/106-13/chapter10.pdf),
section 10.6.3.2, **printed p. 10-44 / PDF page 56** (191-page copy):

> Only static Computer-Generated Data, Format 1 packets may precede the first time data packet in the recording.

SHA-256: `7722e2fe4a595b507ac14931115174eea2083c46b334bd0c2af2a311d53bc475`.
This is RCC publication text served by the archive, not vendor commentary.
The distinct IRIG-hosted 188-page copy inspected for #40 puts that sentence
on printed p. 10-41 and has SHA-256
`aeed3ed5e33350e2a3e963809a934c2026113610c2608f56c62f6955bec79aa4`.
Do not transfer pagination or fingerprints between copies. The earlier
research record remains unchanged history.

Other cited sources remain 106-24R1 Chapter 11 sections 11.2.3.2 p. 11-24,
11.2.3.3 p. 11-26, Table 11-4 pp. 11-6–7, and 11.2.7.2 pp. 11-50–51.
The initial ordering is printed; the mid-session time boundary is OWNER
POLICY, not a newly found standard-mandated terminator.

## Rejected recommendation and owner override

The #40 recommendation left mid-session candidates provisional absent an
independently justified boundary: the standard has no observable universal
terminator; identifying changed data can depend on the new TMATS; unaffected
traffic can continue; the event PRECEDES the update rather than terminating
it. Event identity is still unverified (INT-030/F7), so a nearby 0x02 is not
promoted to that event. These reasons are retained, not disproved by policy.

Joey overrides that recommendation for SRCC=1 with the **next trusted TIME
packet**, not the broader original Option C's next arbitrary time/data
packet. This gives a deterministic bounded streaming boundary, but periodic
time may split a legitimate multi-fragment update. Report INCOMPLETE and
continuous-but-split instead of silently waiting for more fragments or
repairing the input. All bytes and uncertainty remain visible. Initial A's
sign-off/concurrence is recorded; the extension and guard refinements await
independent AI recheck before final amended approval. Freshness, development
analysis and executable tests remain required afterward.
