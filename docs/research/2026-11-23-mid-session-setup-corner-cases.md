# Mid-session setup changes: corner-case catalogue for owner discussion

Date: 2026-11-23. **Discussion only: no option or recommendation in this
catalogue is approved.** It records the existing rules separately. INT-045
itself is approved: initial Option A plus the guarded owner mid-session
extension, AI CONCURS on #41 at 79b94eb. That does not approve INT-028/029/
031/032 governance proposals, remaining INT-034 details, or implementation.
Full configuration timelines remain in 0.2; discussion does not move them
into 0.1. A proposed consumer applicability map is not a new library API.

Order is a tentative estimate of encounter likelihood: routine retransmission
and interleaving first, then damaged/burst changes, then legacy/specialised
cases. No real-recording corpus was inspected and no measured frequency is
claimed. Several cases can occur together; the ordering can be revised with
field evidence. Existing numbers stay stable when moved: 09 is raised for
early integrity-policy discussion; M1–M3 are additional discussion handles,
not requirement IDs. No new exception or default is approved here.

## Sources and the approved baseline

Unless stated otherwise, citations are to [106-24R1 Chapter 11, January 2025](https://www.trmc.osd.mil/wiki/download/attachments/335389015/chapter11.pdf?api=v2).
These source keys apply to every case below:

| Key | Edition, section and printed pages | What the text establishes |
| --- | --- | --- |
| S1 | 106-24R1 Ch11 §11.2.7.2, pp. 11-50–51, Figure 11-34 | Setup describes configuration; fragments increment n+1; SRCC indicates change; unchanged subsequent setup clears SRCC; changed setup precedes new/changed data; preceding change event required; minimum TMATS requirements still apply; ASCII/XML cannot mix in one session |
| S2 | 106-24R1 Ch11 §11.2.3.2–3, pp. 11-24/26; Table 11-4, pp. 11-6–7 | Initial static-setup-before-first-time ordering; time types 0x11/0x12 where defined, not a printed universal mid-session terminator |
| S3 | 106-24R1 Ch11 §11.2.1.1 b/f, pp. 11-5/8 | Channel identity and per-channel sequence counters; baseline channel-0 restriction uses "as of 106-17" |
| S4 | 106-24R1 Ch11 §11.2.7.3 a, pp. 11-51–52 | Recording events are defined through setup; event packets may occur after first time and before the last root index, not a universal setup terminator |
| S5 | 106-24R1 Ch11 §11.2.7.4, pp. 11-56–61 | File recording-index mechanism; explicitly undefined for Chapter 11 packet streaming |
| S6 | [106-24R1 Ch6](https://www.trmc.osd.mil/wiki/download/attachments/335389015/chapter6.pdf?api=v2) §6.2.3.11 f, pp. 6-22–23; Ch9 Table 9-2 MESSAGE DIGEST/CHECKSUM, p. 9-13 | G\SHA digest/exclusion rules; checksum and configuration applicability are different questions |
| S7 | [Project-archived 106-13 Ch10](https://github.com/TelemetryWorks/rcc-106-standards/releases/download/106-13/chapter10.pdf) §10.6.1.1 b, p. 10-29; §10.6.3.2, p. 10-44 | Older channel wording and initial time ordering; SHA-256 e3740c7ac3099236f4e97cbe5b84c169340e6d4a4222769619e83ea0f8176fde |
| S8 | [106-24R1 Ch9](https://www.trmc.osd.mil/wiki/download/attachments/335389015/Chapter9.pdf?api=v2) Table 9-4, TIME DATA TYPE FORMAT / TIME FORMAT / TIME SOURCE, pp. 9-71–72; Ch11 §11.2.1.1 i, p. 11-9 | `R-x\TTF-n`, `R-x\TFMT-n`, `R-x\TSRC-n` describe time metadata; RTC is free-running during a session, not a TMATS-encoded correlation-reset command |

S1 gives a one-way event-before-update requirement, not an event identity,
end marker, inverse event-implies-update rule or recovery algorithm. Do not
promote an ordinary 0x02 recording event into F7's still-unidentified event.

Existing **approved** baseline: INT-011 joins per channel; INT-045 separately
seals initial/SRCC=1 phases at eligible trusted time packets. FIRST-fragment
SRCC determines update eligibility. Cut text is Incomplete. Otherwise Complete
sealed text followed by same-channel +1, matching FRMT/RCCVER continuation
becomes finally Ambiguous unless G\SHA verifies the sealed text alone.
Mid-session Complete is labelled `time-boundary seal (owner policy)`; both
readings/findings remain. There is no backward rejoin. INT-039 retains scalar
duplicates per established document. INT-030/F7 requires verified event
identification, and reports indeterminate meanwhile. Approval is not execution.

## 01. SRCC=0 retransmission of an unchanged setup

**Case:** The recorder repeats the same configuration without a change.
**Standard:** S1 says a subsequent unchanged setup clears SRCC to 0.
**Current rule:** INT-045 excludes SRCC=0-first retransmissions from its
mid-session extension. INT-029's byte/configuration equivalence and sharing
rules remain proposed; a periodic time packet does not silently close this
candidate under the SRCC=1 rule.
**Options/recommendation:** A keep the accepted description while separately
tracking the repeat's evidence and finality; B suspend semantic output during
every repeat. Recommend A only through a reviewed equivalence/governance
contract, never by assuming that SRCC=0 proves unchanged bytes or meaning.

## M1. Unchanged SRCC=0 repeat makes the INITIAL seal ambiguous

**Case:** After the initial time seal, an unchanged SRCC=0 candidate on
channel 0 continues +1 with the same FRMT/RCCVER.
**Standard:** S1 permits unchanged setup with cleared SRCC; S3 sequence
continuity is not a record delimiter. 106-24R1 Ch9 §9.4.2 p. 9-2 governs
single-entry attributes, not record boundaries. The time-seal guard is policy.
**Current rule:** The INITIAL sealed record becomes finally Ambiguous under
approved INT-045 unless its own G\SHA verifies. Excluding the following
SRCC=0 candidate from mid-session closure does NOT exempt the earlier record
from the premature-seal guard. Both readings and finding remain; no rejoin.
**Options/recommendation:** A add a BYTE-IDENTICAL following-candidate exception:
keep the sealed record Complete, labelled `identical retransmission`; B use
the weaker signal of an SRCC=0 FIRST fragment repeating the single-entry G
attributes; C retain the approved guard without another exception. Recommend
A for owner consideration only if the WHOLE following candidate's TMATS byte
extent is independently established and identical, not a matching prefix.
Byte identity is literal TMATS text (including layout/unknown/checksum
bytes), not packet headers/CSDWs or logical field equivalence. Treat B as
information rather than proof. A/B are UNDECIDED; until decided,
C applies as approved. Byte identity proves equality, not integrity or full
required-attribute validity, and chooses no new boundary for the repeat.

## 09. The update's G\SHA fails

**Case:** A checksum mismatch, malformed checksum or indeterminate exclusion
appears in otherwise framed text.
**Standard:** S6 defines the digest over its exact byte domain, not a general
rule that checksum failure means a truncated configuration.
**Current rule:** Checksum states remain distinct; failure cannot supply
INT-045's positive sealed-text exception. With qualifying continuation the
final outcome is Ambiguous; without it, structural Complete is not integrity.
**Options/recommendation:** A quarantine semantic adoption on integrity failure
(the original catalogue recommendation). The independent reviewer DISAGREES
with A as the default and recommends B: adopt an otherwise eligible setup
with a PROMINENT integrity finding, consistently for INITIAL and UPDATE
records; quarantine becomes an opt-in strict caller mode. Carry B as the
recommendation for owner discussion, retaining A as the strict option. No
default is decided: B does not waive incomplete/ambiguous/trust guards, change
a checksum failure to success, or approve the still-open governing setup
selection. Never restamp/normalise input to pass or silently preserve an old
setup only for checksum-failing updates. See 03/04 for separate adoption rules.

## M2. A second setup arrives during the initial phase

**Case:** Before the first time packet, the producer writes two complete-looking
setup records on the same channel with +1 and matching FRMT/RCCVER; the
second has SRCC=1.
**Standard:** S1 describes changed setup and its preceding event; S2 allows
only static Format 1 before first time. Neither supplies a record-start marker
from the SRCC transition; F7's event encoding remains unidentified.
**Current rule:** Matching same-channel +1 fragments still join under INT-011
before the initial seal. The two well-framed payloads are not automatically
two established documents. Repeated single-entry G attributes retain INT-039
and proposed INT-034 duplicate/split evidence; no last-record winner. A later
SRCC=1 does not upgrade a 0-first candidate's extension eligibility.
**Options/recommendation:** A retain both readings and withhold a guessed
initial description; B introduce a reviewed SRCC-transition/producer-boundary
rule; C accept explicit external record boundaries with labelled evidence.
Recommend A now. B/C need their own owner decision and review; do not invent
an event type or waive initial ordering to make the sequence look valid.

## M3. Only TFMT/TSRC/TTF changes: does time correlation reset?

**Case:** Channel IDs and data layouts stay the same, but `R-x\TFMT-n`,
`R-x\TSRC-n` or `R-x\TTF-n` changes.
**Standard:** S8 Table 9-4 names TIME FORMAT, TIME SOURCE and TIME DATA TYPE
FORMAT at pp. 9-71–72. Ch11's free-running RTC is not a command to retain or
reset a consumer correlation model; these rows define no such reset algorithm.
**Current rule:** FIRST-fragment SRCC=1 uses approved INT-045 closure; a 0-first
candidate stays outside even if its metadata differs. Governance/correlation
transition is not decided. TSRC is a category, not a time-channel association;
TFMT defaults do not prove clock continuity or override packet evidence.
**Options/recommendation:** A invalidate/re-establish AFFECTED correlation state
at an explicitly reviewed setup-adoption point, using packet clock evidence;
B retain correlation only with explicit evidence it is unchanged; C reset
all time state unconditionally. Recommend A for owner review, allowing B when
proved. Neither metadata change nor this recommendation resets wire RTC,
selects an epoch/default clock, or silently adopts a new setup; see 03/04.

## 02. Update fragments interleaved with periodic time

**Case:** A time packet arrives before all update fragments do.
**Standard:** S1 defines fragmentation; S2's explicit first-time ordering is
initial-only. Mid-session closure at time is owner policy, not printed law.
**Current rule:** INT-045 seals the SRCC=1 candidate anyway. A cut attribute
is Incomplete; later +1 continuity is continuous-but-split, never rejoined.
**Options/recommendation:** A spool affected output and expose the incomplete
candidate; B offer raw/diagnostic output only until the caller chooses a
reviewed policy. Recommend A with explicit resource limits; neither option
silently delays the approved seal or repairs the input.

## 03. Data between update start and close: old setup or new?

**Case:** Data arrives while the new description is still being assembled.
**Standard:** S1 puts new setup before new/changed data, but does not provide
a complete adoption algorithm for unchanged interleaved traffic.
**Current rule:** INT-045 closes/classifies, not governs. Proposed INT-028
would apply a complete setup from the packet after its LAST fragment, not
necessarily from the later time seal; INT-031's fallback behavior is unapproved.
**Options/recommendation:** A buffer the affected range, retaining offsets,
then resolve an explicit reviewed applicability map before publishing; B keep
using old setup as a labelled assumption; C speculative new decoding with
rollback. Recommend A. Neither "new from first fragment" nor "old until the
time marker" is currently approved as a blanket answer.

## 04. Time and decode consumers switch at different points

**Case:** One consumer has adopted new TMATS while another still uses old.
**Standard:** S1 describes a shared recording configuration; it does not
specify consumer cache coordination or a per-packet provenance schema.
**Current rule:** INT-028/031/032 adoption and ungoverned ranges remain
proposed. INT-045's candidate outcome can be revised after later continuity.
**Options/recommendation:** A share one reviewed applicability map and immutable
setup identity/revision, labelling each packet with setup, phase, outcome,
basis and any assumption; B let each consumer choose independently. Recommend
A, with spooled outputs reflecting revisions before publication. The exact
switch offset, publication finality and API still need owner decisions.

## 05. A NEW channel produces data before the update closes

**Case:** The old setup has no description for a newly active channel.
**Standard:** S1 requires changed setup before new/changed data; S3 defines
channel identity. These facts do not authorise borrowing another channel.
**Current rule:** No approved governance rule chooses old/new TMATS here;
INT-045 cannot invent the missing channel description or time association.
**Options/recommendation:** A retain/quarantine those packets pending a reviewed
new-setup mapping and report ordering uncertainty; B use explicit external
configuration as a labelled assumption. Recommend A. Do not automatically
backfill early packets merely because the closed candidate later names them.

## 06. The update removes channels, but their packets still arrive

**Case:** Buffered or continuing old-channel data crosses the update.
**Standard:** S1 describes the new configuration and its ordering; it does
not specify a grace period for removed channels or delayed arrival repair.
**Current rule:** General channel/range checks and INT-028 adoption are still
pending. Removing a channel does not erase its earlier setup or packet bytes.
**Options/recommendation:** A retain historical setup identities and test each
packet against an explicit reviewed applicability interval; B allow a
producer-documented overlap as a labelled exception. Recommend A; do not
invent a timestamp-based grace period or silently discard packets.

## 07. A channel ID stays the same but its meaning changes

**Case:** A channel changes format, rate or time metadata without changing ID.
**Standard:** S1 makes configuration part of setup, not the numeric ID alone;
S3 defines identities, not a rule for preserving decoder state across changes.
**Current rule:** INT-045 does not carry time associations/defaults across
setups. The release contract requires explicit per-record evidence; wider
configuration comparison in INT-029 is proposed.
**Options/recommendation:** A version metadata and decoder/time state by the
chosen setup, with an explicit reviewed transition; B keep caches solely by
channel ID. Recommend A. How to drain/reset state and label in-flight results
is an open consumer contract, not permission to invent TMATS defaults.

## 08. A premature time seal happens to end at a semicolon

**Case:** The text looks complete, then matching +1 continuation arrives.
**Standard:** S1 has no universal end marker; S3's sequence continuity alone
cannot distinguish a new record from continuation.
**Current rule:** INT-045 changes the final outcome to Ambiguous with both
readings and a located possible-premature-seal finding. Verified sealed-text
G\SHA can keep Complete as classification evidence only; policy label remains.
**Options/recommendation:** A retain/revise findings in the spool and withhold
unqualified publication; B allow a caller's explicit labelled interpretation
without changing stored candidates. Recommend A. No option reopens the seal
or adopts the joined reading as an approved repair.

## 10. Truncated update at EOF

**Case:** Recording stops while an attribute or fragment is incomplete.
**Standard:** S1 has fragmentation but no physical EOF completeness guarantee;
Ch9 §9.4.2 p. 9-2 requires a semicolon terminator.
**Current rule:** INT-035 retains unterminated text without an explicit value;
INT-019 refuses the undefined checksum domain. INT-034 proposes Incomplete
for a cut run. An otherwise complete `;`-ending EOF run keeps its labelled
Complete outcome; EOF alone is not a downgrade.
**Options/recommendation:** A retain the damaged candidate/range and publish no
unqualified replacement setup; B explicit externally evidenced assumption.
Recommend A; whether old setup may continue is still INT-031 owner work.

## 11. Update during recording-index or ordinary event traffic

**Case:** Index/event packets interleave with setup fragments.
**Standard:** S4 permits event placement after first time and before final
root index; S5 defines file indexes and says streaming indexes are undefined.
Neither is a universal setup terminator or proof of F7's event identity.
**Current rule:** Other-channel traffic does not interrupt INT-011 joining;
same-channel non-setup packets can stop that join. INT-045 is separate.
**Options/recommendation:** A retain channel/order/provenance, applying each
packet's own reviewed contract; B use any event/index as setup closure.
Recommend A. Channel-use conformance is separate; never infer session EOF
from an index encountered mid-file or treat 0x02 as verified F7.

## 12. SRCC=1 setup with no preceding identified configuration event

**Case:** The update arrives, but the reader cannot establish its prerequisite.
**Standard:** S1 p. 11-51 requires the configuration-change event BEFORE the
new setup. It does not identify its packet encoding or fully define "prior".
**Current rule:** Approved INT-030/F7 reports indeterminate until identity is
verified; "no identified event" is not yet proof that a known event is absent.
**Options/recommendation:** A retain candidate event/order evidence and an
indeterminate result; B enforce a missing-event finding once an independently
reviewed identity/order contract exists. Recommend A now, B only after that
review. Do not silently reject valid setup bytes or guess a nearby event type.

## 13. Update changes FRMT or RCCVER

**Case:** Text/XML format or the recorded-data version changes across fragments.
**Standard:** S1 defines FRMT/RCCVER and forbids ASCII/XML mixing in one
session. S3 per-channel sequence does not override field disagreement.
**Current rule:** INT-011 stops joining on FRMT/RCCVER change; INT-045's
matching-field continuation guard is not proof of a split when fields differ.
XML stays unsupported; version declarations are not inferred from G\106.
A FRMT 0-to-1 or 1-to-0 change within the SAME asserted session also gives
the ASCII/XML MIXING VIOLATION finding, independently of boundary/completeness.
Do not suppress that finding by treating a format change as an inferred session.
**Options/recommendation:** A retain separate candidates and edition/format
findings, keeping applicability unresolved; B accept a separately evidenced
new session only if the caller asserts it. Recommend A by default. Never
invent a session reset to excuse mixed format or clear conflicting evidence.

## 14. Two updates back-to-back

**Case:** Another SRCC=1 setup starts before the previous one can be trusted.
**Standard:** S1 defines sequence and SRCC, not an unambiguous record-start
marker; Ch9 §9.4.2 p. 9-2 defines scalar uniqueness, not a delimiter for records.
**Current rule:** Without a time boundary, qualifying fragments join and
repeated scalar evidence remains under INT-039/034. With a boundary, later
matching +1 continuity invokes INT-045's final ambiguity/verified-SHA guard.
**Options/recommendation:** A preserve both readings and withhold a guessed
winner; B accept explicit reviewed producer boundary evidence with a label.
Recommend A until B exists. A second SRCC=1 bit alone is not a record restart.

## 15. Configuration event without a following update

**Case:** An apparent change notification is never followed by new setup.
**Standard:** S1 requires event before an update, not the inverse implication
or a maximum delay. S4 ordinary recording events have other meanings.
**Current rule:** F7 identity remains unverified; no approved rule manufactures
an update from a nearby 0x02 packet or from elapsed time.
**Options/recommendation:** A report an unmatched event candidate as information/
indeterminate evidence; B enforce a producer-specific timeout only through an
explicit reviewed contract. Recommend A. First verify event identity before
calling it a configuration event, let alone a standard violation.

## 16. Lost, duplicated or reordered setup fragments

**Case:** A packet gap, repeat or reordering disrupts a multi-packet update.
**Standard:** S1 requires n+1 fragmentation order; S3 supplies per-channel
modulo-256 sequence, not a packet-loss reconstruction algorithm.
**Current rule:** INT-011 retains discontinuity and stops joining; INT-034's
precise gap/boundary precedence remains proposed. Bytes/provenance survive.
**Options/recommendation:** A report partial/ambiguous evidence without
synthesising or reordering bytes; B ingest a separately repaired source only
as an explicit, attributable caller action. Recommend A. A legitimate 255->0
wrap is not a gap; matching timestamps do not prove the missing text.

## 17. Legacy update moves to a different setup channel

**Case:** A pre-106-17 recording uses a different channel for the next setup.
**Standard:** S3 gives the baseline's "as of 106-17" channel-0 wording;
S7 p. 10-29 instead prints "as of 106-13". Those restrictions on channel 0
alone do not define cross-channel governing-setup selection.
**Current rule:** INT-011 keeps channels independent; INT-045 chooses no
cross-channel winner. The release's edition-dependent channel check remains
separate; "pre-106-17" alone is not a blanket conformance certificate.
**Options/recommendation:** A retain separate descriptions and audit the exact
edition/copy plus producer scope; B assume newest setup on any channel wins.
Recommend A and bring the source wording difference to owner review. No
existing channel policy or interpretation is silently changed here. The
[per-edition source review](2026-11-23-channel-zero-edition-sources.md) includes
the older single-multiplexer scope and exact 106-13 p. 10-29 wording: channel
0x0000 is reserved "as of 106-13". This is a separate conformance question,
not a joining rule, cross-channel winner or blanket pre-106-17 exemption.

## 18. SRCC bits disagree among fragments

**Case:** A first-fragment 1 is followed by 0, or a 0-first repeat contains 1.
**Standard:** S1 defines the bit's configuration-change meaning but not a
majority/last-fragment aggregation rule.
**Current rule:** Approved INT-045 uses FIRST-fragment SRCC for its extension;
0-first retransmissions stay outside. Individual bits/disagreement remain.
**Options/recommendation:** A expose disagreement separately from eligibility
and ask the caller to retain/quarantine uncertain adoption; B infer actual
configuration from the last bit. Recommend A. This discussion does not reopen
first-fragment eligibility or approve INT-029's configuration comparator.

## 19. Missing, untrusted or unsupported time boundary

**Case:** No eligible time arrives, its first candidate is corrupt, or only an
unsupported/reserved time type is present.
**Standard:** S2 initial time requirements and edition-specific types do not
supply a trusted marker when framing/version support is unavailable.
**Current rule:** INT-045 never substitutes IPTS/secondary timestamps. An
untrusted initial first-time marker is not replaced by the next trusted one;
mid-session trust findings persist; live runs stay provisional until justified
closure, within the budget. EOF is actual input termination only.
**Options/recommendation:** A retain uncertainty with bounded spooling and typed
operational failure; B caller-supplied reviewed external evidence/assumption.
Recommend A absent B; never manufacture EOF or reuse a previous clock guess.

## 20. Open candidates or buffered data exceed memory budget

**Case:** Many channels or a long update stall retention/publication.
**Standard:** S1/S2 give no application memory budget or eviction permission.
**Current rule:** L2-ROB-003/L3-ROB-004–006 are drafted resource requirements:
finite aggregate accounting, typed limit error, no silent run loss or false
EOF. Detailed accounting/resume behavior still needs review/implementation.
**Options/recommendation:** A explicit bounded operational failure with accepted/
rejected ownership and output status; B explicit consumer-managed bounded
spool under its own contract. Recommend A as the library boundary, B only
with a reviewed consumer plan. No hidden eviction or library file I/O.

## 21. New asserted session, split file or late attachment

**Case:** A reader encounters an apparent new start while old state exists.
**Standard:** S1 defines the recording session as .RECORD to .STOP; S2 uses
session/recording wording, not a guarantee that every file/socket starts one.
**Current rule:** INT-045 requires caller-asserted, labelled session starts;
each assertion has its own initial phase. It does not infer starts from q=0,
offset zero, sync, SRCC, a filename or the first packet observed.
**Options/recommendation:** A keep session identities/provenance distinct and
report unresolved old candidates without inventing EOF; B accept explicit
external session boundaries as caller assertions. Recommend A plus B when
actually supplied; do not stitch recordings together by matching IDs.

## 22. G\SHA in the continuation would verify the joined reading

**Case:** A later fragment contains a digest that fits the hypothetical whole.
**Standard:** S6 specifies a digest byte domain, not permission to join across
an owner-policy time seal or select one ambiguous record interpretation.
**Current rule:** INT-045's exception requires verification of SEALED text.
The reviewer's diagnostic-only suggestion does not change that exception.
**Options/recommendation:** A note the hypothetical match as diagnostic
information, retaining locations and unadopted status; B promote the joined
hypothesis to a governing record. Recommend A, never B automatically: no
rejoin, classification upgrade or authoritative joined-integrity claim.

## How to use this catalogue

Walk one case at a time with the owner. Record any chosen behavior separately
with its authority and the ADR-0033 review required for a new interpretation;
do not turn these recommendations into executable defaults. Particularly
settle cases 03/04 before assigning a setup to interleaved data or publishing
consumer output. The catalogue is a focused discussion set, not proof that
all real recorder behaviors have been found or that requirements are complete.

Source inspection checked the cited baseline packet/field/event/index
passages and the project-archived 106-13 channel paragraph. This is not an
executed parser, validator, time decoder, corpus-frequency or end-to-end test.
No real-recording corpus, local uncommitted work or sibling implementation
is claimed covered. Development analysis and tests remain outstanding.
