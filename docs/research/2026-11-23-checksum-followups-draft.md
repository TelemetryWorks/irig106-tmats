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


## PR #31 review amendment — 2026-11-23

This addendum supersedes the earlier proposed malformed label for 02- and
any overly broad refusal to compute on a malformed stored value. The owner
relayed AI CONCUR on INT-018/019/020 follow-up wording at #31 head b449673,
except COMPUTE eligibility; AI CONCERNS on INT-043 concerned the label.
The amended wording still needs exact-head review; INT-043 remains proposed.

COMPUTE requires a defined, unambiguous exclusion and complete input, not a
valid stored digest. A terminated 2-ABC or 63-digit digest is malformed for
VERIFY but permits COMPUTE. The zero placeholder is an instance of the same
rule, not an exception. Refuse authoritative COMPUTE for missing terminator,
known truncation, duplicates/compatible damage, blocking BOM or an enlarged
value with swallowed-attribute evidence; do not infer delimiter loss solely
from a short digest. All refusal/ambiguity findings stay visible.

Recommend exact supported identity 2-, without numeric equivalence. 02-,
002- and 0- fit the Table 9-2 integer-hyphen-hex range, so label them unknown/
unsupported algorithm designators, not malformed on spelling alone. 02/002
get a finding that they resemble 2 with leading zeros; 0 remains unsupported
without pretending zero equals 2. No normalization or positive verification.
Alternative remains numeric acceptance of 02/002 as 2, with a finding; this
would broaden supported identity and still needs explicit approval. Pending
owner sign-off/recheck, this proposed classification remains unavailable.

Leading-blank 2- is unavailable under the blank rule. A UTF-8 en dash in 2–
is Invalid under INT-037, not an unsupported ASCII designator; +2- and 2.0-
are malformed. Upper-case digits after valid 2- follow INT-020. Stamping
always writes canonical 2-. A defined exclusion permits COMPUTE irrespective
of those stored-value classifications. Planned rows pin both sides of this
compute/verify distinction using the unchanged independent hashlib preimages.
