# Character interpretation drafts — 2026-11-21

Baseline analysis draft for INT-036/037, not a completed analysis, sign-off,
independent-review result or executable-test report. It follows approved
INT-035 without changing actual framing, merged run findings or EOF silence.

## Sources inspected

- 106-24R1 Chapter 9 sections 9.4.1 and 9.4.2, printed page 9-2, specify
  7-bit ASCII, nonprintable discard before configuration translation,
  CR/LF for readability, intentional blanks and case-insensitive meaning.
  [Official Chapter 9](https://www.trmc.osd.mil/wiki/download/attachments/335389015/Chapter9.pdf?api=v2)
  ([archive](https://github.com/TelemetryWorks/rcc-106-standards/releases/download/106-24R1/Chapter9.pdf));
  archive SHA-256 `dfc5ebfb3e6f02f58389126863d43a45d89c7e2d0533a1c690d9537531c740cb`.
- [RFC 20](https://www.rfc-editor.org/rfc/rfc20.html), sections 2 and 4.1/4.2,
  supplies the ASCII positions: control columns 0/1, DEL at 7/15 and SP
  at 2/0. DEL is not a control in the strict sense; SP is described as
  normally non-printing. Chapter 9's intentional-blank rule therefore needs
  explicit treatment rather than a generic is-whitespace/delete filter.
- [Unicode BOM FAQ](https://www.unicode.org/faq/utf_bom.html#BOM), BOM table,
  gives UTF-8 EF BB BF and UTF-16 FE FF / FF FE signatures. These are
  marker-recognition facts, not Chapter 9 permission to support Unicode.

## INT-036 proposal and alternatives

Retain original bytes, frame on their actual separators, and omit 00-1F/7F
only in the semantic projection. Keep ordinary spaces in values; trim them
only around an interpreted code name. CR/LF and a lone CR are nonprintables,
not separators. Preserve original spans/correspondence despite shorter views.

This can join printable parts separated by a control: it is the proposed
reading of discard-before-translation, not a claim the raw source contained
that joined spelling. Treating controls as spaces instead changes the text;
deleting all whitespace destroys intentional value blanks. Leaving controls
in semantic keys instead makes equivalent configuration spellings differ.

Use exactly the proposed space/control/DEL set for layout classification,
so INT-035 remains one information finding per terminated empty run and
silent retained EOF layout/padding. High bytes and delimiters are excluded.
Ordinary ignored controls do not add standalone findings. This choice needs
owner sign-off; the requirement to retain bytes is already accepted.

## INT-037 proposal, ambiguity and alternatives

A recognized leading marker blocks semantic lookup with a located finding,
while raw bytes remain. Never strip a BOM or transcode automatically. Other
high bytes are retained and diagnosed at the affected field; do not silently
interpret UTF-8 or Latin-1, even if the bytes form a valid external encoding.
Automatically removing BOMs would silently change accepted input policy;
automatic transcoding changes the source used for spans/checksums.

Marker recognition at offset zero is deliberately limited. A UTF-16 stream
without a BOM can be indistinguishable from ASCII interleaved with ignored
controls. Do not infer an encoding from alternating zeros: the proposed raw
ASCII policy may interpret those bytes after control omission. It must not
claim to have detected/rejected every UTF-16 file. If the caller knows the
encoding, conversion belongs outside this parser with separate provenance.
Rejecting suspicious zero patterns would require another reviewed policy.
A prefix match is not proof of the full encoding (including longer signatures
sharing a prefix); the finding must say unsupported marker, not assert a
fully identified Unicode document. No Unicode support is delivered here.

## Proposed verification and remaining work

TEST-DATA.md batch 3 covers raw/projected offsets, CR/LF inside/before names,
lone CR, code-name versus value blanks, zero-length projections, all byte
classes, EOF padding, terminated layout runs, high bytes, recognized BOMs,
partial/midstream prefixes and unmarked zero-interleaved inputs.
Expected projections and raw spans must be independently specified.

No code or tests are supplied. Full code-name grammar, indices and unknown-
code/duplicate semantics remain subsequent batches. The line-break hint
stays planned only. Cross-edition and relevant handbook/sample analysis
remain outstanding; no real-data evidence is claimed. Both interpretations
need independent AI review and Joey's sign-off under ADR-0033 before use;
full analysis and tests are additional implementation gates.
