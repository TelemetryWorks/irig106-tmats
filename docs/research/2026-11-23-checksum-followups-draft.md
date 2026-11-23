# Checksum follow-ups and algorithm-token proposal — 2026-11-23

This is a requirement-derivation/source note and planned fixture oracle,
not completed deep development analysis or implementation verification.
Joey reconfirmed INT-018/019/020 under ADR-0033, with AI CONCUR at main
30e42a3; F1/F7 approval is recorded in the combined owner sign-off record.
The reviewer's new notes still need exact-head review. INT-043 is proposed.

## Sources inspected

[106-24R1 Chapter 6](https://www.trmc.osd.mil/wiki/download/attachments/335389015/chapter6.pdf?api=v2),
section 6.2.3.11 f, page 6-22, defines SHA-256, the constant prefix 2- and
exclusion through the following semicolon. The example on page 6-23 includes
G\SHA:0; then a 2- digest. It illustrates a placeholder/replacement, but
its printed checksum is not a byte-exact oracle: the reviewer reported it
unreproducible, and this work makes no claim of reproducing it.

106-24R1 Chapter 9 section 9.4.2 supplies case-insensitive interpretation;
Table 9-2 MESSAGE DIGEST/CHECKSUM, page 9-13, states 2- plus 64 hex digits
and also describes an integer followed by a hyphen. INT-036/038 projection
recognizes names; source correspondence defines RAW exclusion. Leading
layout is hashed, not normalization whitespace to discard. See INT-018/019/
020 for damaged-name, missing-boundary, control, blank and placeholder notes.

## INT-043: owner choice remains open

Recommend exact prefix 2- for usable SHA-256 input. After sign-off, 02- would
be malformed even if its digits otherwise match. This follows the printed
constant without importing INT-038's index identity into an algorithm value.
Alternative: accept numeric 02 as algorithm 2 with a noncanonical finding;
more tolerant, but a new normalization/recovery rule. The cost of strictness
is rejecting a numerically recognizable token; its benefit is one explicit
algorithm spelling. Until owner sign-off and independent AI review, this
handling stays unavailable, not a proven malformed-data verdict. Existing
INT-020 reconfirmation approves no choice between those options.

## Independent fixture oracle

Python standard-library hashlib.sha256 was run over the exact preimages
below. These are future fixture expectations, not tests of repository code.
A fixture h denotes the resulting lower-case hexadecimal digest. The first
fixture has leading CR/LF/space before a projected SHA name with internal
CR/LF; the full raw name-to-semicolon interval is excluded, leaving that
leading layout and the following newline untouched. The second preimage is
the bytes after G\SHA:0; in the placeholder fixture.

| Fixture | Preimage hex (complete bytes) | SHA-256 hex |
| --- | --- | --- |
| `projected_sha_name_excludes_raw_span` | `0d0a200a475c504e3a544553543b` | `7f8b1b2594e2f9421664329073422e5b1902b724fc519e8a796d6142beafd818` |
| `sha_zero_placeholder_is_unverified` | `475c504e3a544553543b` | `e4478640e0caef18c801adb453a53db25425b2a1246bddb80e208d2ff35c0a9e` |

Do not copy the document's whole projected text into hashlib: that would
normalize bytes outside exclusion. Future tests use these explicit byte
oracles independently of the production exclusion scanner, plus edge-case
variants, uppercase digits, CR/LF-in-value, placeholders and refusal paths.
No parser/checksum implementation tests ran here. Full touched-edition and
handbook analysis, intensive tests and stamping/transaction decomposition
remain outstanding. Family 2 continues; no library code is authorized by a
sign-off record alone.
