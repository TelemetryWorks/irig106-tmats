# CHE and numeric field-value syntax: owner decisions

Date: 2026-11-25. Joey's Option A decisions, after #50 (head 658c2a6;
merge commit 6740272). INT-054/055 are owner-signed; their independent AI
check under ADR-0033 is pending. This records decisions and planned tests,
not completed cross-edition development analysis or runtime verification.

## Owner direction (verbatim)

> 1. CHE (Option A): a TIMEIN entry whose R-x\CHE-n says disabled is STILL enumerated in the time view, with CHE reported as context. It is never silently filtered; callers can filter. A missing, duplicate or invalid CHE follows the existing absent/ambiguous/invalid rules.
>
> 2. Numeric field-value syntax (Option A). Applies to all numeric field values (NSB, TK1, R\N as a condition operand, and others) and numeric-looking enumeration codes (TTF/SHTF/TFMT 0/1/2, etc.):
> - Read a nonempty run of ASCII digits only.
> - Leading zeros are accepted with a warning finding, canonicalised by stripping them, consistent with INT-043 and INT-038. So 04 = 4 and 02 matches code 2.
> - A sign (+/-), any blank (leading, trailing or embedded), any non-digit, or more than 10 digits makes the value INVALID, never repaired or clipped. Keep the leading-blank rule from INT-041 where it already applies (unavailable), and make the interaction explicit.
> - Check the range after canonicalisation; out of range is Invalid, never wrapped.

Owner follow-up for this PR (verbatim):

> (a) time_view_formats_bind_to_entry_not_extras checks raw, located binding only. Any arm asserting TTF Explicit/Invalid state follows the new numeric-code rule.
> (b) Pin time_view_local_format_on_non_timein_is_not_absent to a letter TFMT code (e.g. B).

## Sources and interpretation boundary

Inspected the [official 106-24R1 Chapter 9](https://www.trmc.osd.mil/wiki/download/attachments/335389015/Chapter9.pdf?api=v2),
section 9.5.4 Table 9-4: CHANNEL ENABLE p. 9-49 (T/F, enabled source
required to generate packets); NUMBER OF TRACKS/CHANNELS p. 9-32 (1–65536),
NUMBER OF SOURCE BITS p. 9-33 (0–13), TRACK NUMBER/CHANNEL ID p. 9-48
(1–65535); CDT/CHE/DSI/TK1/SHTF conditions pp. 9-48–51, and numeric
TTF/SHTF/TFMT codes pp. 9-51, 9-71–72. Source fingerprint already verified
in batch 16: `dfc5ebfb3e6f02f58389126863d43a45d89c7e2d0533a1c690d9537531c740cb`.
This turn inspected the official text; it did not download/hash an archive
anew or inspect another repository. Recheck archived source freshness during
development. Section 9.4.1/INT-036/037 governs character projection;
section 9.4.2/INT-041 preserves the existing leading-blank unavailable guard.

INT-054 separates a library enumeration policy from a packet-generation
constraint; missing/duplicate/invalid context never implies enabled. INT-055
supplies owner lexical policy; the ranges alone do not require digit-only
input or a ten-digit cap. Zero-padding is an interpreted comparison with a
warning, never a byte edit. The cap applies before zero-stripping. Existing
INT-041 leading-blank unavailable cases (TFMT ` 2`, ` A`) retain precedence;
the existing NSB leading-blank case (` 4`, L3-VIEW-024) likewise remains
unavailable. Trailing/embedded
blanks in numeric tokens are Invalid. No broader blank exemption is invented.

### Breadth to examine in independent review and development analysis

The owner's explicit ALL numeric-field scope is retained. Table 9-4 also
prints hexadecimal fields (e.g. START-RESUME COMMAND WORD p. 9-48) and
floating-point INPUT THRESHOLD (p. 9-52, −999.9 to 999.9). The general policy
conflicts with spellings those definitions admit. Do not silently narrow it
to NSB/TK1/unsigned counters or silently exempt representation families.
The independent reviewer should examine this breadth; full per-definition
coverage, edition checks and any required owner follow-up precede code.
G\SHA remains its separately reviewed structured grammar (INT-018–021/043),
not a whole digit-only field; code-name indices retain INT-038. Control
projection composes INT-036, while high-byte input remains INT-037 Invalid.

## Planned verification and remaining gates

Former owner-choice blockers are removed and parsed expectations specified;
AI review, intensive analysis, source freshness and executable tests still
block implementation. Existing supplied-condition unit cases remain useful,
with explicit parsed R\N integration arms (valid 1, zero/out-of-range,
missing, duplicate, invalid/unavailable). Binding-only tests assert raw
locations, not numeric state. Non-TIMEIN local TFMT uses letter B.

New planned fixtures cover disabled/enabled/missing/duplicate/invalid CHE,
explicit caller filtering, signs/blanks/non-digits/empty/10-versus-11 digits,
zero-padding warnings, boundaries, numeric-code matching, operand uncertainty,
projection findings and byte preservation. Future CI should generate bounded
numeric mutations independently of the runtime parser, pin each cited range
and assert raw round-trip identity. This note is not the required full
cross-edition analysis, and no runtime checks or generated parser tests ran.
