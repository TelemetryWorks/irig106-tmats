# Reusable CLI mounting: family 2 batch 24

Date: 2026-11-26. Documentation-first derivation; independent PR review
and implementation gates remain. No new interpretation or approval claimed.

## Existing decisions and the carried clarification

L1-CLI-009 cites the owner's 2026-09-26 direction in owner-direction
section 25: reusable CLI code for the greater irig106-cli, avoiding duplicate
command implementations. ADR-0034 W3 retains both a host-callable
`run(args, stdout, stderr) -> ExitClass` that never exits the process and
individual command/render APIs with a thin standalone main. L2-CLI-001/002
and L3-CLI-001–003 already establish those boundaries.

The coordinator's instruction after the approved #55 is recorded verbatim:

> Note for the CLI-mounting slice (L1-CLI-009): state that the orchestrating host does the data-type dispatch and the adapter's Format-1 check is only a guard.

This clarifies accepted ADR-0030: each tool owns its joining loop, with core
independent of TMATS. ADR-0036 supplies the one-reader ownership and borrowed
DATA integration. It does not put the joining loop inside core or authorize
a second packet reader in the mounted CLI library.

## Two mounting paths, one computation

A host can mount the whole runner using explicit invocation arguments and
supplied writable streams. It owns the surrounding command namespace and
the outer process. The mounted runner returns its exit classification; it
does not call process exit even for a failed invocation. The standalone
binary obtains its process arguments/streams and uses that same runner.

Alternatively, a host can call individual command APIs and receive report
data, then invoke a renderer with its supplied writer. That path does not
require calling main or parsing the whole runner's arguments. It does not
require the host to implement the command again. Computation and rendering
remain distinct under the previously accepted contracts.

These statements do not select argument representation, subcommand-prefix
stripping, command signatures, report fields, renderer layout, finding order,
JSON schema or exit mappings. In particular, merely naming a renderer here
does not approve a format. The owner gate on those choices is unchanged.
The existing writer failure boundary applies without selecting new output
error spelling, rollback behavior or console presentation.

## Packet pipeline and guard

For a packet pipeline, core supplies header-only PacketFacts and separate
PacketInfo integrity/trust evidence, admission/ranges and borrowed DATA.
The host dispatches by supplied packet facts. The TMATS fragment adapter
checks that its input is Format 1, then only slices the four logical CSDW
bytes and remaining text under the existing adapter requirements.

Chapter 11 (106-24R1) section 11.2.7.2/Figure 11-34, printed p. 11-50,
supplies the Format-1 CSDW/text fact. Source inspected: the official
[Chapter 11 PDF](https://www.trmc.osd.mil/wiki/download/attachments/335389015/chapter11.pdf?api=v2),
SHA-256 `0f63792dcac39f6010df371ee0afc46a396017918f4155cba9ebab6c2d6d263e`.
The standard supplies packet-format facts, not a CLI mounting API. The
accepted architecture supplies responsibility and process boundaries.

Format 1 is the packet data type; it is distinct from the CSDW FRMT bit
selecting ASCII or XML. The adapter guard must not conflate them or claim
that a type match establishes a supported text encoding. Existing format
handling and unavailable-format reasons remain unchanged.

The guard is defensive: an incorrectly dispatched input cannot acquire
Format-1 semantics or append its DATA as TMATS. It does not sniff payload
bytes, infer another packet type, validate raw headers, find more packets,
verify checksums or choose among RDG-008 readings. Matching the expected
type does not establish admission or setup completeness/governance.

Whole-source content routing from batch 23 remains distinct from this
packet-level dispatch. Neither asks TMATS to rescan core's identified
packets. No cross-repository source/API inspection, dependency installation
or release action is performed by this documentation slice.

## Preserve boundary observations

A fragment-only filter must not hide observations required by the approved
assembly contracts. INT-011 needs same-channel interruption evidence and
other-channel provenance; INT-045's layered time closure needs qualified
recognised time observations, including those on another channel. The host
can route that time packet's payload to its time consumer while supplying
the applicable boundary observation to the TMATS assembler.

This is responsibility, not an exact API/event type: keep core trust/edition
qualifications and caller-asserted session-start provenance. Do not invent
new packet recognition, a time decoder inside TMATS, a same-channel-only
time closure rule or session detection from offset zero. Incorrect fragment
input being guarded does not suppress observations supplied through the
assembly boundary. Joining, completeness and governing selection remain
separate, with all existing approvals and unresolved gates intact.

## Planned verification and remaining gates

L3-CLI-022–025 refine the prior runner/command/render boundaries;
L2-CLI-007 and L3-CLI-026–029 derive host dispatch and adapter integration.
TEST-DATA lists planned cases; no Rust implementation/coverage is claimed.
Future fixtures use an instrumented host/consumer boundary and explicit
facts/evidence rather than a second packet reader or checksum oracle.

A subprocess host driver must prove control returns after both successful
and unsuccessful real runner invocations. Supplied memory/failing writers
prove stream ownership and delivery behavior without pinning rendered
bytes. A real reviewed command must be exercised when implemented; help
or empty stubs alone cannot complete the mounting requirements.

Source freshness, development analysis, compatible published types/core 0.2
and executable tests remain implementation gates. Report/rendered output
formats remain an owner-decision gate. Streaming provisional-use, governing
adoption and ambiguous/cut acceptance remain their own gates; mounting a
consumer does not silently settle them. No owner decision blocks review of
this existing boundary derivation, and no new INT is proposed.
