# IRIG 106 alignment review — 2026-09-27

Review of local HEAD `da59b01`. Recommendations below are review findings,
not accepted design decisions or replacements for the interpretation register.

## Assessment and scope

The documentation-first redesign is substantially better aligned with the
standard than the executable prototype. The current library should not be
used as a compliance authority or to rewrite authoritative TMATS. This is a
targeted review, not an exhaustive audit of every attribute or historical edition.

Primary sources inspected directly:

- [106-24R1 Chapter 9, January 2025](https://www.trmc.osd.mil/wiki/download/attachments/335389015/Chapter9.pdf?api=v2):
  section 9.4.2, Table 9-2, and Table 9-6.
- [106-24R1 Chapter 11, January 2025](https://www.trmc.osd.mil/wiki/download/attachments/335389015/chapter11.pdf?api=v2):
  section 11.2.7.2, Figure 11-34 and its field definitions.

The architecture's ordered source store, preservation of unknown and malformed
input, explicit edits, reviewed source-backed registry, effective values, and
separate presence-validation pass directly address prototype failure modes.
Separating the TMATS edition from the recording-format declaration is also
the right direction. These are design strengths, not implemented capabilities.

## Confirmed executable defects

### High: PCM interpretation loses valid attributes

`src/parse.rs:623` assigns D1 to bit rate, D2 to encoding, F1 to words per
frame, F2 to bits per word, and F3 to sync pattern. Chapter 9 Table 9-6 defines
D1 as PCM code, D2 as bit rate, F1 as common word length, F2 as word transfer
order, and F3 as parity. The registry and tests repeat the erroneous mapping.

A temporary executable linked against the built library parsed this fragment:

```text
G\PN:TEST;G\106:17;G\DSI\N:1;G\DSI-1:REC;G\DST-1:STO;
P-1\DLN:PCM;P-1\D1:NRZ-L;P-1\D2:1000000;
P-1\F1:16;P-1\F2:M;P-1\F3:OD;
```

Observed typed values: bit rate absent, encoding `1000000`, words per frame
16, bits per word absent, sync pattern `OD`. Serialization dropped D1 and F2.
These individual field meanings were checked against the baseline; this
fragment is not asserted to be a complete conforming recording configuration.

### High: keyword validation rejects standard values

`src/version.rs:265` gives G\DST the keywords REC/TEL/MUL/PRE. Table 9-2,
DATA SOURCE TYPE row, includes STO. The probe above produced TMATS-V030 for
STO. It also produced TMATS-V032 for the numeric D2 bit rate and V060 findings
for D1/F2, illustrating why a clean or failing report is not reliable evidence
of conformity. `tests/version_registry_tests.rs:177` explicitly blesses REC.

### High: setup-record format and edition handling are incorrect

`src/ch10.rs:31` treats bits 9–31 as reserved, and `decode_setup_payload`
always invokes the ASCII parser. Figure 11-34 assigns bit 9 to ASCII/XML
format; only bits 10–31 are reserved. A probe decoded `[12, 2, 0, 0]` and
re-encoded `[12, 0, 0, 0]`, losing the format bit.

`src/ch10.rs:162` supplies RCCVER as a parser override. A payload with RCCVER
0x0B and G\106:17 retained the raw `17` but reported source_version V106_15.
The redesign correctly distinguishes these declarations; the implementation
does not. `src/types_bridge.rs` also stops at 106-17.

### High: lenient recovery silently discards the whole document

`src/parse.rs:756` returns an empty document after any tokenizer failure in
lenient mode. With `G\PN:TEST;G\106:17;BROKEN`, an explicit Lenient probe
returned program=None and zero diagnostics. This violates the redesign's
preservation/diagnostic contract; malformed input is not itself a conforming
TMATS example, so this finding is a recovery defect, not a claim that the
standard requires lenient acceptance.

## Design issues to resolve before implementation

### SRCC must not be judged from byte identity alone

INT-029 (`docs/INTERPRETATIONS.md:595`) labels SRCC=0 with different body
bytes an unannounced configuration change. Chapter 11 section 11.2.7.2
defines SRCC in terms of recorder configuration. Chapter 9 section 9.4.2
allows attribute reordering and case-insensitive interpretation. Reordering
equivalent attributes can therefore trigger a false finding under this rule.

Recommendation: retain byte identity for deduplication, distinguish textual
changes from configuration changes, and return an indeterminate result when
unknown attributes prevent a defensible semantic comparison. Pin equivalent
ordering/case examples in the eventual tests. This is a proposed refinement,
not an accepted interpretation.

### Preserve the literal RCCVER declaration separately from inference

INT-012 reports 0x0E as "106-22 or later"; INT-017 says it covers every later
edition and selects the newest baseline. Chapter 11 explicitly assigns
0x0E to 106-22 and reserves 0x0F–0xFF. The absence of newer codes does not
establish an unlimited future compatibility guarantee.

Recommendation: report the literal 106-22 declaration, separately explain
that later inspected editions retain this code, and identify any newer
validation basis as policy/inference. The existing labelled-fallback design
is useful; the open-ended field interpretation overstates the evidence.

### Assembly needs a precise distinction between ending and truncation

INT-011 (`docs/INTERPRETATIONS.md:260`) ends a run at a sequence gap, CSDW
change, or EOF. INT-031 (`:631`) lists these among incomplete-record cases.
Chapter 11 section 11.2.7.2 permits segmented records but supplies no explicit
end marker. The documents do not yet provide an executable decision rule
that consistently distinguishes complete, incomplete, and ambiguous runs.

Recommendation: define these states before L2/L3 or assembler code. Cover a
complete final record at EOF, a genuinely truncated record, a missing middle
fragment, and adjacent complete records. Preserve uncertainty instead of
treating every end-of-run condition as proof of completeness or truncation.

## Verification and release implications

`cargo test --workspace --all-features --offline` passed, with one integration
test and one doctest ignored. The integration test is
`generate_validates_with_zero_errors`, explicitly ignored because generated
output fails the prototype validator. Temporary probes were removed; no
library code or tests were changed. A first probe used strict parsing for the
malformed tail and stopped at its expected parse error; the corrected probe
selected Lenient explicitly and produced the result recorded above.

Passing tests currently protect several wrong mappings. The trace matrix
records 78 L1 requirements, no L2/L3 decomposition, and two verified L1 leaves;
those two concern releases, not TMATS conformity. This percentage must not be
presented as standards coverage.

Priorities: make the README's prototype/redesign status explicit; resolve the
three design questions above through the existing interpretation process;
then implement the lossless parser and a small, independently verified
registry slice with standards-derived positive and negative fixtures. Expand
coverage by edition and attribute family only as evidence is added. Existing
prototype findings should become regression cases for the replacement, not
claims that the planned implementation already fails.
