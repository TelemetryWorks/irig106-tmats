# INT-011: independent joining per channel

Date: 2026-11-23. Decision-maker: Joey. Context: #37 merged/approved at
fabbc06; the independent AI check returned CONCERNS on "intervening packet".
Status: owner decision recorded, AI recheck of amended wording pending.
This is not completed ADR-0033 review or implementation/analysis evidence.

## Owner decision

> 'intervening packet' means a SAME-CHANNEL packet only, which the +1 sequence test already detects;
> packets from OTHER channels between fragments are recorded as provenance and information only, and NEVER stop the join;
> joining is PER CHANNEL and INDEPENDENT: if setup-record fragments arrive on more than one channel (pre-106-17 recordings allowed non-zero channels), each channel's fragments form that channel's own run(s), and runs on different channels never mix or interrupt each other.

This clarifies the existing joining interpretation using 106-24R1 Chapter 11
section 11.2.7.2 page 11-50 (fragment sequence n+1) and section 11.2.1.1 f
(per-channel sequence counters). The quoted source wording is not rewritten.
Same-channel FRMT/RCCVER agreement remains required. This changes no literal
RCCVER mapping, SRCC predicate, checksum or completeness rule.

The reviewer's additional suggestion is recorded: retain each run's START
reason, e.g. "start of supplied input", alongside stop reasons and source
evidence, so INT-034 can consider leading truncation. A first observed
fragment need not be the intact beginning of a record. Restart reasons can
identify an observed same-channel interruption or sequence/FRMT/RCCVER change;
other-channel traffic never creates a restart for the open run.

The channel-0 restriction from 106-17 (Chapter 11 section 11.2.1.1 b) is an
edition-dependent conformance check, not a joining rule. Preserve nonzero
setup runs even when the separate edition check reports nonconformance.
Earlier recordings' independent nonzero-channel setup runs are not combined.

## Consequences and planned evidence

Input/provenance must distinguish channel identities and preserve per-channel
sequence evidence. Global physical packet adjacency is not required. A
setup-only filter cannot erase consumed same-channel sequence evidence.
Other-channel packets remain provenance/information, not bytes in the run's
assembled text or a reason to stop it. A same-channel interruption affects
only that channel; every other channel's current run remains independent.

Planned cases cover interleaved PCM/time packets, two interleaved fragmented
setup channels, a same-channel non-setup interruption, evidenced start reasons
and edition-dependent channel checks separate from joining. Exact source
mapping remains packet-based despite interleaving. These are acceptance
specifications, not executed implementation tests.

INT-034's precise completeness rules still need their own review/derivation.
Its "followed by another packet" basis cannot create a global joining stop
contrary to amended INT-011. Leading-truncation handling remains INT-034's
work; retaining a START reason is not a new completeness algorithm. Historical
ADR-0025/0031 and the batch-14 research draft remain unchanged as records;
the interpretation register and current requirements carry the amendment.
