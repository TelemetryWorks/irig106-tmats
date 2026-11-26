# PR #54 factual correction: header facts and separate core sideband

Date: 2026-11-26. Reviewer/owner relay at head 879946b supplies the concrete
type boundary: PacketFacts belongs to irig106-types and contains HEADER FACTS
ONLY. It has no trust/integrity fields. No field is added or proposed here.
Earlier research records remain history; this corrects their shorthand.

The CLI consumes borrowed DATA spans and PacketFacts, plus core-supplied
integrity/trust evidence carried ALONGSIDE them as PacketInfo sideband:
secondary_integrity per RDG-008, data-checksum state, admission evidence and
ranges. Forward it unchanged; do not infer it from header facts, merge it
into PacketFacts, compute/check a checksum in tmats or choose a secondary
reading. CSDW fields come from logical DATA, not PacketFacts header metadata.

Reword L2-CLI-003, L3-CLI-004/010 and live architecture references, keeping
other ownership/zero-copy decisions unchanged. A planned integration case
holds the same header facts while varying sideband evidence and asserts
unchanged separate propagation, not a new PacketFacts structure.

## Release dependency order

The owner relay requires irig106-types 0.2 and irig106-core 0.2 published
BEFORE tmats 0.1. Core 0.1.0 being published, or core 0.2 being merged, does
not satisfy these published-version prerequisites. The release plan now
records both. Data-checksum verification remains a separate core feature
request with timing decided by the owner, not a new tmats implementation or
mandatory verification gate before this integration can pass NotChecked.

This is a docs-only correction based on the supplied review facts, not a
claim that another repository's source/API was inspected or dependency
versions were installed. No manifests, shared-type definitions, runtime,
report/JSON formats, policy/settings or publishing workflows are changed.
Local checks cover exact type/evidence wording, prerequisite statements,
fixture links, trace --check and whitespace; GitHub CI supplies heavy checks.
No runtime integration, borrow/performance result or publishing is claimed.
