# `tmats` — Command-Line Design

> **Status: draft for review (design phase).** This is the design reference
> for the `tmats` command (crate `irig106-tmats-cli`). The crate exists as a
> scaffold (`--version`, `--help`); no command is implemented until this
> document and the matching requirements are reviewed. Use case: UC-17 in
> `docs/USE-CASES.md`. Release rules: `docs/RELEASING.md`.

## 1. Purpose and scope

`tmats` is a focused tool for looking at, extracting, and checking TMATS. It
is the Rust successor to irig106.org's `idmptmat` (whose three output formats
it keeps) and covers the viewing and checksum functions of `igDisplayTMATS`.
It is **not** the ecosystem's general tool: that is `irig106-cli`, which uses
the same library. Every command is a thin layer over library calls; anything
a command computes, the library can compute for other callers.

### Reusable mounting boundary

The `irig106-cli` host may mount `run(args, stdout, stderr) -> ExitClass`
or call individual commands and renderers (L1-CLI-009; ADR-0034 W3). The
host owns process lifetime; only its outer boundary or standalone main maps
a returned classification to process status. Command computation is shared.

For a packet pipeline, the orchestrating host does data-type dispatch using
core-supplied facts. The TMATS adapter's Format-1 check is only a guard, not
a second reader or dispatcher. Non-fragment assembly observations required
by INT-011/045 remain available even when another consumer gets that
packet's DATA. L2-CLI-007 and L3-CLI-022–029 refine these boundaries; they
select no argument grammar, report format, exit mapping or adoption policy.

### Checksum command computation boundary

L2-CLI-012/013 derive checksum, verify, explicit flex and stamp computation
for a supplied eligible document/revision. They preserve library verdicts,
findings and transaction failures; they do not define rendered output or
selection defaults. Owner-signed INT-031 gates incomplete/ambiguous access.
A successful stamp result still needs successful delivery under the separate
input-preserving output contract. ADR-0037 settles extract's destination and
findings policies; that approval is not extended to stamp's remaining policy.
Core PacketInfo data-checksum state passes through unchanged: a G\SHA result
neither upgrades nor is upgraded by core Mismatch/NotChecked. Their separate
results do not bypass packet admission or record-access gates.

## 2. Commands (from UC-17)

| Command | Does | Release |
|---------|------|---------|
| `tmats show FILE` | Channel summary (default): channel ID, data type, enabled, data source — `idmptmat -c` | 0.1 (R-group items); 0.2 (resolved links) |
| `tmats show --tree FILE` | Hierarchy: data sources → recorders → channels, then every group by occurrence and code path — `idmptmat -t` | 0.1 |
| `tmats show --raw FILE` | The TMATS exactly as stored — `idmptmat -r` | 0.1 |
| `tmats extract FILE.ch10 -o OUT` | Byte-exact extraction; --force and --strict are owner-approved spellings (2026-11-27, ADR-0037); remaining grammar held | 0.1 |
| `tmats checksum FILE` | Print the `G\SHA` value (`2-` + 64 lower-case hex) | 0.1 |
| `tmats checksum --flex [--include ...] FILE` | irig106.org flex signature (non-standard; UC-16) | 0.1 |
| `tmats verify FILE` | Compare the embedded `G\SHA` with the computed value | 0.1 |
| `tmats stamp FILE -o OUT` | Write a copy with `G\SHA` inserted or updated | 0.1 |
| `tmats validate FILE` | TMATS text rules: syntax, required fields, valid values, duplicates and cross-references (UC-06/07); recording or text input by content | 0.2+ increments |
| `tmats diff A B` | Compare two documents (UC-11) | 0.2 |

Per the [2026-11-27 roadmap direction](ROADMAP.md#small-increments-after-01-owner-direction-2026-11-27), recording/setup
cross-check tooling (UC-18) belongs to `irig106-validator`, rather than a
`tmats check` command. `tmats validate` checks TMATS text rules and uses the
existing content-based recording/text input route. The 0.1 scope is unchanged.

## 3. Behaviour already decided

- **Input by content, not extension.** A file that starts with the Chapter 10
  packet sync pattern is a recording; anything else is TMATS text.
  L2-CLI-005/006 derive this selector and acquisition/failure boundary; route
  selection is not packet validation or permission to retry the other mode.
- **The configuration timeline.** From 0.2, `show` lists every setup record
  with its kind (first, repeat, change), its findings, and what changed
  (UC-19; L1-CH10-008). Recording/setup cross-check tooling (UC-18) belongs
  to `irig106-validator` per the 2026-11-27 roadmap direction.
- **Every setup record.** A recording's setup records are all reported, in
  file order (the `idmptmat` first-packet-only limitation, defect D2, is not
  repeated). Each record's output is labelled with its position and packet
  offset. L2-CLI-010/011 derive internal enumeration and provenance while
  leaving rendered labels, public numbering/selection and live emission
  policy for owner review. Complete is not a governing/eligibility verdict;
  owner-signed INT-031 gates incomplete/ambiguous record access.
- **Recordings are memory-mapped**, and packet finding/trust comes from
  `irig106-core` (ADR-0036), with borrowed DATA spans into TMATS slicing/
  assembly. The temporary CLI reader is superseded; no header or packet-data
  checksum implementation is added here. DATA need not be contiguous or
  copied/coalesced; input blocks remain alive for the borrows.
- **Plain text by default, JSON on request**, with a documented, versioned
  JSON schema so scripts and `irig106-cli` can rely on it.
- **Labels are the code names actually read.** A value is never shown under
  another code name (defect D4).
- **Extraction preserves bytes.** ADR-0037 records the owner policy of
  2026-11-27: extract the explicitly selected original bytes despite findings,
  reporting every finding; --strict refuses the listed bad-data kinds.
  Owner Q18 (2026-11-28) selects listed bad-data kinds/class, not informational notes. There is no
  always-refuse finding class. Cut/ambiguous/invalid/marker qualifications
  and core integrity evidence stay unchanged; raw extraction does not make
  them semantically usable or governing.
- **Extract destination policy.** An existing unrelated -o file refuses
  unless --force. Open without truncation, compare identities and truncate/
  write through the same handle, with no temp-file+rename. Known input aliases
  refuse even with --force. Stdout/pipes are allowed without identity proof;
  a regular file with unproven identity refuses unless --force. The exception
  never claims identity was proved. Tool-opened output and external shell
  redirection remain distinct. These three #57 holds are removed; source,
  development and executable verification remain. Other argument forms need their reviewed request table; report formats
  now follow Q26/ADR-0039. This does not authorize a new raw-byte encoding.
- **Inputs are not edited in place.** Commands that produce modified TMATS
  (`stamp`, and later fix application) write to a new output file.
- **`stamp` is a verified transaction.** It hashes the final output bytes
  outside the `G\SHA` item (a line break it inserts is included), writes
  lower-case hex, re-reads the output, and fails without writing if the
  result does not verify (ADR-0029). F1 is owner-approved, 2026-11-23
  (INT-021; coordinator origin ADR-0034): duplicate `G\SHA` items are indeterminate;
  list every occurrence, preserve every byte, never claim positive verification,
  and refuse stamping until the input is fixed.
- **Editions shown as the file declares them.** `show` and `validate` print
  the TMATS edition declared (`G\106`, raw and read), each setup record's
  recording-format version (CSDW, raw and read), and — for `validate` — the
  edition whose rules were applied with its basis (override, declared, or
  fallback). `--edition` is the override. A compatibility check is labelled
  as one (ADR-0028). Approved INT-044 Option B (Joey, 2026-11-23; AI CONCURS on #36 at 6f6e080)
  keeps duplicate `G\106` Ambiguous and uses the separate label
  `fallback — reason: ambiguous G\106 (n candidates)` without an override.
  Text and JSON make this basis/reason at least as prominent as cannot-evaluate;
  every occurrence and duplicate finding remains visible. JSON carries stable
  machine code `ambiguous_g106` and numeric `candidates` separately from the
  display label. Mixed recognised/unrecognised duplicates use this reason too.
- **Both versions in every report.** `tmats --version` prints
  `tmats X.Y.Z (irig106-tmats X.Y.Z)`; JSON output carries both.
- **Every reference-tool defect is a regression test** (D1–D7,
  `docs/TEST-DATA.md`), for example summary and tree output over one, two,
  three, and many data sources (D1).
- **Argument parsing is hand-rolled, with no dependency** (owner decision,
  2026-09-25: "Go with hand-rolled argument parsing like mie-decoder"). The
  CLI adds no argument-parsing crate such as `clap`. What this buys: no
  dependency or its MSRV and semver churn, a small binary, and full control
  of every help and error message. JSON report encoding has the coordinator reading of Q26
  documented in section 4, pending owner confirmation; raw/extract output stays byte-exact.
  What it costs: no generated shell completions and more parsing code to
  own; that code is kept table-driven and every flag, value, and usage error
  has a test. The scaffold's `--version` / `--help` handling already follows
  this approach.
  L2-CLI-014/015 derive parser mechanism/acceptance from host-supplied tokens;
  they choose no new grammar, defaults, report rendering or exit mappings.

## Owner policy additions — 2026-11-28

ADR-0038 records Q4/5: exact literal raw lookup for ANY name, including
vendor/unknown; wildcard patterns use only reviewed registry metadata.
Each * matches one whole name part or index, e.g. R-*\TK1-*; no partial
wildcards or regex in 0.1. No other query/CLI escaping form is invented.
Q23 permits stdin only through an explicit '-' operand, never an implicit
fallback. Q14 confirms first-fragment inventory order, separately from
closure/takeover. Report layout, schema/labels/exit mapping follow owner Q26/ADR-0039; implementation remains reviewed work.

## 4. Owner Q26 A report contract — 2026-11-28

[ADR-0039](adr/0039-text-json-reports-and-exit-classes.md) records the owner
text verbatim and immutable irigtime citations. Readable text is the default;
`--format json` selects JSON and `--format text` selects text, matching the
peer's actual parser. No --json alias is introduced. Results use stdout;
operational diagnostics use stderr, without corrupting structured/raw output.

The minimal published JSON schema carries stable finding codes, severity/
locations, explicit unavailable states and record qualifications, separate
assumption/default/fallback evidence, caller-choice level and Quiet's
mandatory `suppressed: N bad-data findings`, including N=0. Keep the existing
schema_version identity and independent schema versioning; additive changes
increment minor and breaking changes major. Schema/artifact/layout/catalogue
implementation is review work, not a remaining owner-format hold.

Exit classes are 0 success, 1 failed check, 2 usage, 3 input and 4 output.
JSON on 0/1 is complete; discard partial or unsuccessfully delivered output
on 4. The host runner returns a class without exiting. This selects no new
check threshold, --fail-on flag or simultaneous-failure tie rule.

Show's existing 0.1 raw/tree/R-channel-summary forms and CLI-002/006 are
unblocked. Text/JSON share semantic facts, while byte-exact raw/extract output
keeps its original byte contract. Command/mode/format applicability must be
explicit in the reviewed request table, not silently ignored, wrapped or
converted. Full resolved-link/timeline views stay later scope.

**Coordinator reading of Q26 “matching irigtime” (2026-11-28), pending
owner confirmation — ASCII-only JSON stdout.** Matching
[irigtime’s pinned report contract](https://github.com/TelemetryWorks/irig106-time/blob/5899806ae684ed38d8cad8f422cabb3d47c6be0f/docs/cli-report-schema.md#L53-L54),
JSON stdout is ASCII-only: non-ASCII characters use Unicode escapes,
including surrogate pairs for supplementary characters. This is JSON
representation, not a source projection or a rule to repair raw bytes.
Raw show/extract stay byte-exact. This coordinator reading awaits owner
confirmation and adds no ASCII-only text/diagnostic rule. How non-UTF-8 TMATS
source bytes appear inside JSON remains for schema review; character escaping
does not define a byte-to-character decoding or justify a lossy conversion.


## 5. Decisions made on 2026-09-25

The owner accepted all five proposals ("go with your suggestions for all
five"):

1. **Selecting a setup record:** every setup record in a recording is shown
   by default; `--record N` (1-based, file order) shows only the N-th.
2. **Flex signature flags:** `--include` takes a comma-separated list of
   `comments`, `vendor`, and `g`, or `all`, mapping one-to-one onto
   `irig106lib`'s `TMATS_SIGFLAG_INC_COMMENT`, `_INC_VENDOR`, `_INC_G`, and
   `_INC_ALL`. Output keeps `irig106lib`'s `OO-SSSSSSSS` form (opcode, then
   signature, in hex) so values compare directly with `igDisplayTMATS`.
3. **No `idmptmat`-compatible text mode.** The test harness compares with
   `idmptmat` semantically (channels, types, data sources, hierarchy), so
   `tmats` inherits neither its layout nor its defects.
4. **JSON schema:** kept in `docs/schema/`, versioned independently of the
   crates; every JSON document carries a `schema_version` field. A breaking
   schema change is a breaking release of both crates (docs/RELEASING.md).
5. **Release binaries:** built and uploaded by `cargo-dist` (Windows, Linux,
   macOS binaries and SHA-256 checksums from one tag), set up before 0.1.

No CLI design decisions remain open; the command-by-command reference
(reviewed request tables, schema artifacts, output layouts and renderer/exit acceptance) follows the owner Q26 policy and L2/L3 requirements.
