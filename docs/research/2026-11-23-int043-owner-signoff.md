# INT-043 owner approval record — 2026-11-23

Owner decision: Joey, **Option B**, 2026-11-23, under ADR-0033. The owner
relayed that the independent AI recheck **CONCURS** with the amended Option B
wording in [PR #32](https://github.com/TelemetryWorks/irig106-tmats/pull/32)
at head `8941ca1c2c436ca453f09e7fd530031136f84c57`, merged at
`8ab9b9854b7001d1bff9d1b3bf4b7d85515f20ae`. INT-043 is now **APPROVED**.
The PR/head and owner relay are the check reference; no separately posted
review or concurrence on this new PR is claimed.

## Approved choice and warning clarification

ASCII-digit designators whose canonical digit string is 2 identify algorithm
2, without integer conversion. Leading-zero spellings can verify positively
when other prerequisites succeed, but always emit warning
NONCANONICAL-DESIGNATOR citing Chapter 6 section 6.2.3.11 f page 6-22 and
Chapter 9 Table 9-2 page 9-13 (106-24R1). Raw bytes stay unchanged; stamps
write canonical 2-. Other numeric values remain unsupported, and existing
blank/control/high-byte/upper-case rules remain as recorded.

Reviewer clarification relayed with approval: emit that warning for every
located item/candidate with a readable noncanonical-2 token even if F1
duplicates, damage, a BOM or undefined exclusion blocks verification. Do not
suppress/coalesce warnings with the integrity result. Diagnostic recognition
under those guards restores no semantic lookup or checksum exclusion and
never authorizes a positive verdict. Findings retain their individual locations.

## Remaining evidence

The earlier owner-decision record is preserved as history; its pending AI
status is superseded here. The new warning clarification and new PR still
need exact-head independent review. Approval is not completed deep analysis,
source freshness, implementation or passing executable tests. Family 2
continues with the bounded stamp-transaction requirement slice; all earlier
interpretation/development gates and unresolved F2/F3 choices remain.
