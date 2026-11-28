---
status: accepted
date: 2026-11-28
decision-makers: Joey
---

# Text/JSON reports and exit classes

Owner direction, verbatim:

26 (report format) A, matching irigtime (docs/cli-report-schema.md, Q-CLI-EXIT):
- readable text by default; JSON on request;
- results on stdout, operational diagnostics on stderr;
- a minimal versioned JSON schema with: stable finding codes, severity and locations; explicit unavailable states and record qualifications; assumption/default/fallback distinctions; the caller-choice level; Quiet's mandatory 'suppressed: N bad-data findings' (including 0);
- additive changes bump the minor version, breaking changes the major;
- exit codes: 0 success, 1 failed check, 2 usage, 3 input, 4 output;
- complete JSON on exit 0 and 1; partial output discarded on 4;
- the exact flag spelling for JSON follows irigtime's existing flag (cite it), not a new invention.

Verified existing spelling: `--format json`; explicit text is `--format text`.
Pinned peer source: irig106-time `5899806ae684ed38d8cad8f422cabb3d47c6be0f`,
[report contract](https://github.com/TelemetryWorks/irig106-time/blob/5899806ae684ed38d8cad8f422cabb3d47c6be0f/docs/cli-report-schema.md),
[argument parser, lines 97–136](https://github.com/TelemetryWorks/irig106-time/blob/5899806ae684ed38d8cad8f422cabb3d47c6be0f/apps/irig106-time-cli/src/args.rs#L97),
and [Q-CLI-EXIT](https://github.com/TelemetryWorks/irig106-time/blob/5899806ae684ed38d8cad8f422cabb3d47c6be0f/docs/analysis/Q-CLI-EXIT-2026-11-12.md).
The parser reads --format with text/json values; no --json alias is invented.

TMATS uses the owner's major/minor schema policy, not an automatic copy of
the peer's schema_revision convention, envelope or time-specific units.
The existing byte-exact raw/extract contracts and host-callable runner remain;
format selection does not change source, semantic eligibility or findings.
Detailed schema artifacts/renderers and command applicability are reviewed
implementation work, not an unresolved report-format owner choice. Q25's
INT-034 sign-off is recorded in its own PR. This ADR implements no code,
schema artifact, dependency, release or publishing operation.

Coordinator reading of Q26 “matching irigtime” (2026-11-28), pending owner
confirmation: adopt the
[irigtime ASCII-only JSON stdout rule](https://github.com/TelemetryWorks/irig106-time/blob/5899806ae684ed38d8cad8f422cabb3d47c6be0f/docs/cli-report-schema.md#L53-L54).
Non-ASCII characters use Unicode escapes, including surrogate pairs.
Raw show/extract remain byte-exact. This is a coordinator reading, not a
separate quoted Joey sign-off or an additional ASCII-only text/diagnostic
requirement. The original owner Q26 text above is unchanged. Non-UTF-8 TMATS
source-byte representation inside JSON remains a schema-review item;
Unicode escaping specifies characters, not arbitrary source-byte decoding.
