# Content-based input routing: family 2 batch 23

Date: 2026-11-26. Documentation-first derivation, pending independent PR
review. No new interpretation, owner approval or implementation is claimed.

## Existing authority and retained scope

L1-CLI-001 requires accepting recording or standalone TMATS text, determined
by content. Accepted ADR-0019 and CLI section 3 already select the predicate:
a source beginning with Chapter 10 sync is a recording; anything else is
text. ADR-0036 supersedes the temporary reader/checksum responsibilities in
ADR-0019, not this content selector. No new reader is proposed here.

Source inspection used the project's 106-24R1 Chapter 11 PDF, downloaded
from the [official archive](https://www.trmc.osd.mil/wiki/download/attachments/335389015/chapter11.pdf?api=v2),
SHA-256 `0f63792dcac39f6010df371ee0afc46a396017918f4155cba9ebab6c2d6d263e`.
Section 11.2.1.1 a, printed p. 11-5 specifies sync value 0xEB25;
section 11.1, printed p. 11-1 specifies little-endian structural fields.
Together they give the initial byte pair 25 EB (hex). The standard supplies
the sync representation; the accepted ADR supplies this CLI routing policy.
It is not a claim that the standard defines CLI detection or recovery.

## Boundary before packet work

The router examines only the first two logical source bytes. A complete
match selects the core recording path, even if the remainder is damaged,
empty, rejected or not trusted by core. Core alone finds packets and
supplies header-only PacketFacts plus separate PacketInfo evidence, DATA
spans and source ranges. TMATS consumes those under L3-CLI-004–013.
There is no tmats-local scan for sync later in the source, no raw header
validation and no fallback packet/checksum implementation.

A nonmatch selects the TMATS text reader with original bytes unchanged.
A later 25 EB pair, misleading filename or human-readable header does not
change that predicate. Text selection alone does not establish valid text,
a document, usable field values or a complete/governing setup. In particular,
empty and one-byte acquired sources select text without promising success.
The reader's existing raw-preservation/interpretation rules remain in force.

## Acquisition and failure

A source acquisition failure is not an acquired zero-byte source. Do not
turn the former into an empty successful document. Likewise, a recording
that core rejects remains recording-selected; an invalid standalone source
remains text-selected. Automatically retrying the other mode would change
the accepted selector into an undocumented recovery algorithm.

Acquisition remains CLI/caller-owned, with no library file I/O. The selected
consumer sees the original prefix as part of its source, not just the bytes
after detection. Borrowed ownership follows ADR-0036: owners stay alive,
DATA spans need not be coalesced, and a route probe does not require reading
or copying the entire recording. This slice does not select an exact Rust
routing/acquisition signature or inspect other repositories' APIs.

## Requirements and test oracles

L2-CLI-005/006 decompose L1-CLI-001; L3-CLI-014–021 make its selection,
preservation and failure boundaries independently testable. TEST-DATA lists
named planned fixtures, not executable coverage. Routing tests use literal
prefix expectations and consumer spies/injected outcomes. A spy records
which path receives the source and whether its bytes/identity changed.
It is not a packet reader, checksum implementation or alternate core oracle.

A bounded prefix sweep can cover every two-byte prefix (65,536 small
cases), plus empty and all 256 one-byte inputs, with a single expected
recording prefix. Keep expected bytes independent of production constants.
Run executable coverage only when implementation exists, in bounded CI;
this docs PR does not execute or claim those future tests.

Separate fixtures cover actual core integration and text interpretation;
router fixtures do not prove either parser correct. A routing failure
fixture injects downstream failure and checks that the other consumer was
never invoked. No large local allocation or full recording scan is needed.

## Gates deliberately left open

No owner decision blocks derivation of the retained selector. Before
implementation, review these requirements, check source freshness and the
published types/core 0.2 prerequisites, and complete development analysis
and executable tests. Green prototype checks do not satisfy redesign tests.

The owner's report/rendering gate still applies: this slice chooses no
text/JSON schema, findings presentation, assumption-label layout, suppressed
marker rendering, public error labels or numeric exit classes. Bring those
options to the owner before deriving them. It chooses no stdin sentinel,
argument grammar, archive/compression wrapper, resynchronization strategy,
record-selection ordering or new recovery/adoption policy. Standalone text
routing also does not settle provisional setup consumption or governing
selection. Those remain their own requirements and approval gates.
