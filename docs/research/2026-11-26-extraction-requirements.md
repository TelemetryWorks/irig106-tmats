# Extraction, input preservation and remaining family-2 work

Date: 2026-11-26. Family 2 batch 25; proposed requirements for independent
review, not an implemented command or approval of unresolved record policy.

## Review follow-up: owner decisions blocking implementation

The coordinator is asking the owner about three choices. All three are
BLOCKING implementation, not settled defaults:

- `-o` overwrite policy for an existing destination.
- Fail-closed identity checking for pipes/devices/stdout/network shares.
- Extraction eligibility under integrity/encoding findings.

The raw-byte fixtures establish preservation only; they do not approve
extraction in the presence of those findings. Existing source, development,
independent review and executable-test gates remain.

## Existing authority

L1-CLI-004 requires extracting setup-record TMATS bytes to a file;
L1-CLI-007 prohibits input-file modification. The CLI command table and UC-17
already include `extract FILE.ch10 -o OUT`. ADR-0002/0003 retain original
bytes, ADR-0010 assigns filesystem acquisition to CLI/caller glue, and
ADR-0036 supplies the sole core reader and non-contiguous borrowed DATA.
The batch-21 fallible writer contract supplies delivery/progress semantics.
No command spelling, report format or new interpretation is chosen here.

Source inspection: 106-24R1 Chapter 11 section 11.2.7.2/Figure 11-34,
printed p. 11-50, puts the CSDW before each packet's setup text and permits
one record across several packets. The [official PDF](https://www.trmc.osd.mil/wiki/download/attachments/335389015/chapter11.pdf?api=v2)
SHA-256 is `0f63792dcac39f6010df371ee0afc46a396017918f4155cba9ebab6c2d6d263e`.
Core supplies DATA excluding headers/filler/packet checksum; existing
L3-CLI-006/008 perform logical CSDW slicing. Extraction consumes their
results rather than parsing packet bytes again.

## One selected record, original text

The mechanical input is one explicitly selected record with an established
extraction-eligibility basis. Append its original text spans in fragment
order. Remove the four-byte CSDW once per packet through the existing
adapter, not once per physical block. Do not include headers, packet fill,
checksums or another setup record. Do not add a newline, label, BOM, JSON
wrapper or human-readable diagnostic to the extracted text bytes. Do not
strip a BOM, including EF BB BF immediately after the first CSDW when the
marker is split across fragment packets: INT-037 detects it at offset zero
of the assembled text. Detection/eligibility is separate from byte replay.

This is byte replay, not a rendered report format. Malformed names, unknown
attributes, non-ASCII bytes, original layout and preserved padding cannot
vanish merely because a view cannot interpret them. In particular, do not
reconstruct output from typed fields, canonicalize values, stamp a checksum
or invoke normalization as a side effect of extraction. INT-036/037
projections and INT-055 raw-byte numeric checks are VIEW-ONLY. A TAB/control
byte omitted from a COMMENT interpreted value is still extracted; a numeric
value made Invalid by a raw control retains every byte. No projection or
verdict authorizes a source rewrite.

Raw byte preservation and record eligibility are separate. Mechanical sink
fixtures may contain malformed text without asserting that the CLI may
select that record. This batch does not approve incomplete/ambiguous access,
XML interpretation, automatic governing selection, new ordinal ordering or
combining several selected records into one destination. Those need their
own reviewed derivations. No eligible record may be silently substituted
when the requested selection is absent or unresolved.

## Input-file safety and failure

The CLI, rather than the library serializer, owns destination acquisition.
For destinations the tool opens itself, it must enforce the existing
prohibition on modifying input files before destructive access. Shell
redirection outside the tool is out of scope for this acquisition guarantee;
the tool cannot reverse truncation a shell performed before invocation.
Checking only whether two path strings differ is insufficient: relative
and absolute names, links and platform aliases can name the same file.

L3-CLI-034 specifies the race-free order for tool-opened regular files:
open without truncating, compare the opened handle's identity with the
inputs, and only if distinct and permitted by the approved overwrite policy,
truncate through that SAME handle. Never close and reopen the pathname
between checking identity and truncating/writing. A preflight pathname check
alone is insufficient. A known alias produces an observable refusal before
any destructive access. Platform APIs and acquisition analysis remain work;
no general pipe/device/stdout/network-share identity policy is approved by
this regular-file ordering. That policy and `-o` overwrite are owner blockers.
Public error names, rendering and numeric process status remain undecided.

A sink/acquisition failure is not extraction success. Keep the source and
its provenance unchanged, preserve the writer failure and return to the
host. Do not redirect output, select another record or repair input as
recovery. Previously written bytes may remain in a distinct output sink
under the existing writer contract; no atomicity, rollback, cleanup or
durability guarantee is added here. General overwrite policy for an unrelated
existing destination is not selected by forbidding input aliases.

## Planned independent verification

L2-CLI-008/009 and L3-CLI-030–037 derive this boundary. TEST-DATA lists
named planned cases with literal source/fragment oracles. Split DATA at
several positions, including synthetic arbitrary partitions inside CSDW and inside text, then compare to
an independently written expected byte literal, not production extraction.
Inject bounded partial writes and failures using the existing writer oracle.
No extra packet reader or packet-checksum implementation is permitted.
The CSDW-straddling fixture is synthetic. With core 0.2 default 512 KiB blocks,
the CSDW is always in the first block. It is not a claim that production
block boundaries split the CSDW.

Filesystem integration uses isolated temporary files and supported
relative/absolute/symbolic/hard-link aliases; record exact input bytes
before and after success and failure. Use platform-specific capability
checks for link fixtures rather than treating an unsupported link operation
as a successful safety test. Hosts and standalone tools share the same
command computation and input-preservation obligation for destinations they
open themselves. Instrument non-truncating open, identity comparison and
same-handle truncation; a pathname reopen must fail the ordering check.

The docs PR runs only trace/whitespace/link/source-hash checks locally.
Executable commands, platform safety and actual extraction coverage remain
unverified until implementation and GitHub CI; prototype CI is not evidence
that the redesigned command exists.

## Remaining-work estimate requested after #56

Estimate after this slice: roughly 10–14 further family-2 batches, subject to
review splits and owner decisions. This is a planning range, not a promise
that a fixed count or parent-with-children closes all 0.1 requirements.
The audit uses RELEASE-0.1's acceptance inventory and TRACE-MATRIX's live
parents; the 47 previously undecomposed L1 leaves include later-release work
and therefore cannot be used as a remaining 0.1 batch count.

| Remaining cluster | Expected derivation work / gate |
| --- | --- |
| Raw lookup and V/X extensions | Complete lookup/pattern and extension linkage contracts; review any interpretation dependencies |
| Normalization | Actual transformation rules beyond the opt-in gate; owner review of changes to user bytes |
| Setup completeness, edition/session rules | Resolve remaining INT-014–017/028/031/034 dependencies where applicable; keep approved layered closure/ambiguity rules |
| Flex and checksum CLI | Reviewed compatibility algorithm and command integration beyond SHA/stamp byte transactions |
| Record selection and argument parsing | File-order/addressability, command/flag/error cases, multi-record behavior and destination handling |
| Show/report/rendered output | STOP for owner choices on findings, assumption labels, suppression marker and schemas before derivation |
| Robustness and performance | Remaining limits/recovery, fuzz/coverage and measured benchmark obligations |
| Release acceptance | WASM/platform/MSRV, dependency/consumer checks and distribution evidence requirements |

Some clusters need multiple small batches; others can share a reviewable PR.
Cross-check partial children as well as missing L2 parents before declaring
family 2 complete. The twelve-family order then puts ADR-0035 restructure
in family 3 and source store/scanner/recovery implementation in family 4.
The restructure does not itself implement the parser. Source freshness,
owner/independent interpretation checks and development/executable-test gates
still apply; no parser implementation or restructure begins in this PR.
