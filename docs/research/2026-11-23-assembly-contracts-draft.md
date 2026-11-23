# Family 2 batch 14: fragment joining and byte provenance

Date: 2026-11-23. Status: requirement/acceptance-specification draft.
No implementation or executed Rust checks; no new interpretation approval.

## Source and existing decisions

106-24R1 Chapter 11 section 11.2.7.2 page 11-50:

> A single setup record may span multiple consecutive packets. When spanning multiple packets, the sequence counter shall increment in the order of segmentation of the setup record, n+1.

Figure 11-34 places a CSDW at the start of each packet body. Section
11.2.1.1 f defines a per-channel sequence counter. ADR-0025 retains packet
slicing at the CLI boundary and assembly over supplied bytes in the library;
its joining interpretation is INT-011. ADR-0031 supersedes the old ending
rule: a run boundary does not prove a record is complete.

The chapter does not define an end-of-record marker. Same-channel,
matching-FRMT/RCCVER joining is the existing interpretation, not a new quote
from Chapter 9. This draft does not change that rule or add SRCC as a joining
predicate. Every raw CSDW survives; disagreement cannot be hidden behind a
first/last-fragment summary. Its eventual aggregate semantics remain open.

## Bounded requirement slice

L2-CH10-004/005 and L3-CH10-009 through 016 decompose candidate joining and
exact provenance under L1-CH10-004. The scanner needs one assembled byte
coordinate system before interpreting a split code name/value or BOM. Source
mapping must still locate each byte in its originating packet, even with
optional headers and ranges crossing fragments. This motivates the slice
before completeness and session checks without implementing any stage.

A setup-only filtered list does not establish consecutive packets. The input
must preserve adjacency evidence, including intervening-packet observations;
no public API shape is selected. Concatenation contributes no CSDW, padding,
filler or separator bytes of its own. Supplied text slices govern mapping.
Packet integrity and trusted slicing remain separate caller/reader duties.

The planned oracle uses `G\COM:a` + `b;` -> `G\COM:ab;`. Packet starts 100
and 136 plus header lengths 24 and 36 plus each CSDW's 4 bytes put text at
128 and 176. Thus assembled value [6,8) maps [134,135) and [176,177).
These constants are independently calculated from supplied fixture bounds,
not obtained from the proposed assembler. A light Python check validates
these byte counts/ranges only; it does not test repository implementation.

## Interpretation review summaries for the owner

**INT-011 (existing owner-accepted joining rule, AI check pending):** join
only consecutive setup packets on the same channel, sequence +1 modulo 256,
matching FRMT/RCCVER. An intervening packet, gap or field change stops the
run, but does not by itself say whether a record finished or broke. This
batch preserves the existing wording/acceptance; independent review is still
needed before implementation. A passing supplied-binding test cannot approve it.

**INT-034 (precise completeness rules still proposed):** distinguish complete,
incomplete and ambiguous runs, rather than guessing from a packet boundary.
Its detailed rules and interactions with repeated scalar attributes need
their own bounded derivation, owner sign-off and independent AI check. This
batch does not request approval of an undecomposed completeness algorithm.
INT-028/031 governance remains a later dependency; no timeline is introduced.

INT-039 duplicate classification remains document-local. The same-channel,
sequence +1, FRMT/RCCVER conditions explain why a cross-fragment fixture is
one candidate run, not why it must be one complete governing document.
INT-044 approval changes edition-basis handling after document scope exists,
not record classification. Retain every repeated scalar and possible split.

## Remaining work

Completeness classification and its precedence, XML's unknown completeness,
untrusted input recovery, fragment-SRCC summary and session-level rules are
not derived here. Preserve those gaps rather than invent defaults. Existing
INT-034 wording about trailing blanks must be reconciled with INT-035/036
padding/control handling in its own review. Assembly does not authorize
parsing incomplete candidates or computing an authoritative checksum on them.
Full L1-CH10-004 acceptance requires the later children and meaningful tests,
not just this partial trace rollup. Deep edition/handbook analysis and source
freshness remain development gates even after interpretation sign-off.
