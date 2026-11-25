# CHE/numeric independent check and unresolved owner choices

Date: 2026-11-25. Reviewer @thread:thr_qsvwmtnn8f checked PR #51 at
`ebf9ec454da6107e73d27c928390b25025cb9b57`: CHANGES REQUESTED.
ADR-0033: INT-054 CONCUR; INT-055 CONCERNS. This is a later review record;
the original dated owner decision record remains unchanged.

## Owner choices before INT-055 can be approved

1. **Leading blanks:** enumerate the exception set. Reviewer recommendation:
   every numeric field/code with a leading blank is unavailable under
   INT-041; trailing/embedded blanks are Invalid. Current examples TFMT/NSB
   do not settle TK1, R\N, SHTF or TTF. No broader set is approved yet.
2. **Controls:** choose (A) approved INT-036 projection first, so actual
   CR/LF in NSB `0\r\n4` disappears before the numeric read, retaining
   findings; or (B) raw-byte checking, making any control Invalid. The
   current draft uses A. Maintainer recommends A for consistency with the
   existing interpreted-value projection, subject to explicit owner choice.
3. **Scope:** reviewer recommends decimal-integer fields and numeric-looking
   enumeration codes only. Floating-point and hexadecimal values would get
   separate reviewed rules. Current ALL-numeric wording can reject printed
   conforming representations, e.g. INPUT THRESHOLD -12.5 (Table 9-4
   p. 9-52) and command word 0A3F (p. 9-48). Do not narrow it silently.

The maintainer recommends the reviewer's choices for 1 and 3 and projection
first for 2. These are recommendations, not owner decisions. Preserve the
current draft expectations until the owner settles them, with their unresolved
status prominent; no dependent implementation is authorized by this record.

## Confirmed details and correction

The reviewer confirms CHE enumeration/state behavior, all prior fixture
clarifications, range boundaries, empty Invalid, ten digits before stripping
zeros, and exact owner quotations. Add u64 or canonical-string comparison
to L3-VIEW-038: ten digits can be 9,999,999,999, exceeding u32.

Reviewer reports CI run 45643065398 successful at the reviewed head,
including macOS and CodeQL; the local trace/whitespace checks were clean.
Those checks do not verify the replacement's planned requirements. INT-054
still needs source freshness, development analysis and executable verification;
INT-055 additionally needs owner clarification and independent recheck.
