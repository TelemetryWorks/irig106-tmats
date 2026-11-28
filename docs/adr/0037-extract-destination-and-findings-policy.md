---
status: accepted; strict scope refined by 0038 (2026-11-28)
date: 2026-11-27
decision-makers: Joey
---

# Extract destination and findings policy

Owner direction, verbatim:

1. Existing, unrelated -o file: REFUSE by default; overwrite only with --force. Write in place per L3-CLI-034 (open without truncating, compare identity, truncate through the same handle). NO temp-file+rename.
2. Destinations without a provable identity: stdout and pipes are ALLOWED (they can't be an input). A regular file whose identity can't be proven (e.g. some network shares) is refused unless --force.
3. Eligibility under findings (data-checksum mismatch, secondary-header failure, INT-045 cut, INT-048 ambiguous, INT-037 marker, INT-055 invalid): EXTRACT ANYWAY, byte-exact, reporting every finding; --strict refuses. No always-refuse cases.

Flag spellings (--force, --strict) are the owner's words; the rest of the argument grammar stays on hold.
