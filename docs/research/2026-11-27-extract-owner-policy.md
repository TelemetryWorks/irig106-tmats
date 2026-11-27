# Extract owner direction: 2026-11-27

The owner decisions are preserved **verbatim** in
[ADR-0037](../adr/0037-extract-destination-and-findings-policy.md), accepted
2026-11-27, Joey. This removes the three owner-decision holds introduced by
#57. It is a policy record and planned test update, not an implementation or
claim of completed independent review/source/development/executable evidence.

## Scope of the decision

Extract defaults to byte-exact delivery despite findings, reporting every
finding. --strict refuses on findings, with no always-refuse finding class.
The six explicitly named kinds and their combination are covered by planned
cases: core data-checksum mismatch, secondary-header failure, INT-045 cut,
INT-048 ambiguity, INT-037 marker and INT-055 invalidity. A known cut prefix
is available original raw data, not an absent extent requiring invented bytes.
No semantic/governing qualification or core integrity state is upgraded.

Raw extract permission is separate from general semantic access under proposed
INT-031. The owner does not approve that proposal's ambiguous-split option or
make a cut record usable. The command can report retained findings without
inserting them into the byte-exact output. Exact diagnostic formatting and
stream layout remain on hold; no new rendering is chosen here.

Core still owns packet finding/trust/checksums and supplies the actual raw
DATA/text extents; tmats does not guess untrusted lengths or add a second
reader. If no selected raw extent exists at all, that is an operational
availability failure, not an always-refuse finding category. Never confuse
that with extracting a supplied cut prefix or a supplied zero-length extent.

## Destinations and delivery

An existing unrelated regular -o destination refuses unless --force.
Authorized writes open without truncation, compare identities and use the
same handle for truncation/write; no pathname reopen or temp-file+rename.
Known input aliases still refuse, including with --force. Stdout/pipes are
allowed without proof; an unproven regular identity requires explicit --force.
That exception must not be presented as a proved distinct identity or promise
of protection already performed. It does not authorize a known input overwrite.

--force and --strict govern different decisions: force does not bypass strict
finding refusal, and strict does not authorize overwriting an unrelated file.
Eligibility/policy refusal precedes destructive destination access. A failed
in-place delivery may leave a partial unrelated output under the existing
fallible-writer contract; no rename fallback, rollback or durability promise
is added. Shell redirection remains outside tool-opened acquisition safety.

No device grammar, additional flag spelling/placement/combination syntax,
non-Unicode token policy, record default or multi-output format is decided.
The owner's exact --force/--strict spellings are recorded; other argument
grammar remains held. Extract's decision is not generalized to stamp or other
commands, nor to arbitrary library semantic reads.

## Requirements and planned evidence

Update existing L2-CLI-008/009 and L3-CLI-030–037; new L3-CLI-058–061 capture
the owner policies and acceptance. IDs 052–057 from #72 remain unchanged.
This separate PR was authored while #72 was open, then updated onto main
after #72 merged. Both parser requirements and pending scope decisions are
preserved; the extraction owner decisions do not amend #72's review record.

Fifteen new planned case rows specify per-kind default/strict variants,
combined findings, no-findings strict, cut-prefix extraction, all-finding
reporting outside the raw stream, unrelated overwrite, aliases, stdout/pipes,
unproven regular identity, orthogonal force/strict and failure behavior.
Synthetic inputs, injected sideband and isolated destinations suffice; no
real recording or new dependency is needed. Policy labels are not passing
implementation evidence. Platform identity/race and core raw-access integration
still need analysis and executable tests, not a new owner choice overriding
these three accepted decisions.

Earlier dated #57 research is historical and unchanged. Current CLI/ROADMAP,
requirements and TEST-DATA remove the stale three-choice hold while retaining
review/testing and unrelated owner decisions. No new standards interpretation
or register approval is claimed by recording this product policy.

## Verification

Light local checks cover verbatim owner text, trace regeneration/drift,
unique IDs/parents, reserved-ID avoidance, planned case links and whitespace.
No Rust builds/tests, reference runs, installs, sample access, packaging or
publishing actions run locally. Heavy checks use GitHub CI. No background
job is started. Independent review of this exact head remains required.
