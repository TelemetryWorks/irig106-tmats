# Family 2 batch 21: byte-faithful output and normalization boundary

Date: 2026-11-26. Follows merged #52 (reviewed head 3fcbd2c; merge cd63e07).
Requirements and planned acceptance specifications only. This continues the
0.1 writer obligation before implementation; no code, new interpretation,
normalization algorithm or passing runtime test is claimed.

## Scope and already-decided authority

Batch 1's L2-WRT-001/L3-WRT-001 already require full original-byte replay and
returning a supplied writer's failure. This slice makes that invariant
observable across malformed source, read-only operations, caller sinks and
normalization authorization. L2-WRT-003 (L1-WRT-001) supplies the delivery
contract; L2-WRT-004 (L1-WRT-002) supplies the separate opt-in boundary.
L3-WRT-004–011 decompose those obligations. No owner decision blocks them.

Accepted ADR-0002: the ordered source remains authoritative and ordinary
writing reproduces it; normalization is explicit. ADR-0003: the document
owns the bytes, while typed/lossy/Latin-1 views do not replace them.
ADR-0010: callers own I/O; the library takes bytes and supplied destinations.
ADR-0029 already separates explicit stamp transactions and makes no promise
of rolling back a caller's external sink. Existing fallible output is an
error boundary, not a new filesystem-safety or process-exit policy.

The full 0.1 scope still includes L1-WRT-002. This is a PARTIAL derivation:
it does not decide canonical sort order, line-ending style, counter mutation
rules, unknown-field placement or a normalized-output API. Those transforms
need their own reviewed requirements and tests before implementation; they
are not made implicit by selecting a pretty/tree view. The slice does not
mark family 2 complete or authorize proceeding to implementation prematurely.

## Standards evidence and byte/meaning boundary

Source: official 106-24R1 Chapter 9
<https://www.trmc.osd.mil/wiki/download/attachments/335389015/Chapter9.pdf?api=v2>.
Previously downloaded baseline SHA-256 rechecked locally:
`dfc5ebfb3e6f02f58389126863d43a45d89c7e2d0533a1c690d9537531c740cb`.
Section 9.4.1 p. 9-2 distinguishes physical format/layout from interpreted
content. Section 9.4.2 pp. 9-2–3 defines code:value framing, arbitrary
attribute order/case-insensitive interpretation and standalone COMMENT.
The no-loss writer policy is an OWNER/ADR guarantee, not a claim that every
retained malformed byte sequence conforms to Chapter 9.

INT-035–039 preserve framing, strict character interpretations, numeric
index identity and duplicate evidence. INT-041 guards unavailable reads;
INT-048 preserves assembly ambiguity/cut blocking; INT-055 distinguishes
numeric semantic interpretation from raw spelling. None authorizes a writer
to strip controls, encoding markers or padding, canonicalize case/digits,
fix a missing semicolon, insert defaults or merge scalar duplicates.
INT-056 Quiet changes its result attachments, never input bytes/diagnostics.

Fixtures must separate “document retained with diagnostics” from “no
retained document returned”: the replay contract does not force a parser to
accept every format. Unsupported encoding/XML examples test retained-source
behavior where supplied; they are not new semantic-format support.
For assembled input, source means the assembled TMATS text body, not packet
headers or fragment CSDWs. Replay does not create a Chapter 10 recording.

## Independent planned oracles

Use explicit byte vectors as expected output, not a second serializer,
canonical model, hash-only comparison or typed-field reconstruction.
Include CR/LF and lone CR, high/control bytes, layout-only/empty input,
unknown/V/X fields, duplicates, empty values, trailing padding and a cut
suffix. Deliberately vary query order, fallback/default results and Quiet
findings while asserting the identical original vector before and after.
Known semantic failures remain visible; byte identity is not validation.

For sink output, a caller-controlled fake records accepted bytes and failure
positions independently. Exercise finite short accepts and errors before
any byte, mid-source and at completion; do not report success after a sink
failure. Preserve the source and acknowledge whatever prefix escaped to the
external sink. No file, flush, rollback or hidden retry promise is invented.
A fresh caller-supplied sink may be used for an explicit retry test.

Normalization gate fixtures assert absence of transformations without a
request and separation of an explicit reviewed request; they do not pretend
to know unreviewed canonical bytes. An unsupported transformation cannot be
silently supplied by the serializer. Full transformation fixtures remain
future work; this is not a reduced release scope.

## Concurrence follow-up folded into this slice

Record INT-056–060 final ADR-0033 concurrence at the owner-relayed #52
reviewed/merged PR head `3fcbd2cb0e911d1bc23e4cc3a5b53d503e5d7e23`.
The actual GitHub main merge commit is
`cd63e07` (resolved to its full hash in the register); the reviewed PR head
and merge commit are distinct references, not silently interchanged.
This clears interpretation-review status only. Analysis fields stay “not
yet written”; source freshness and meaningful executable tests still gate
implementation. Earlier dated research stays historical.

## Checks and remaining work

Local audits cover IDs/parents and planned fixture links, source hash,
byte-oracle and sink-failure boundaries, register status/citation integrity,
trace regeneration/--check and whitespace. No Rust build/runtime tests or
heavy local checks run; GitHub CI supplies heavy verification and run IDs
are in the PR. No green prototype CI result verifies these new requirements.
Normalization transformations, remaining CLI/robustness/release derivations,
source freshness and implementation evidence remain to be completed.
