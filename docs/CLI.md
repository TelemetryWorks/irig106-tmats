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

## 2. Commands (from UC-17)

| Command | Does | Release |
|---------|------|---------|
| `tmats show FILE` | Channel summary (default): channel ID, data type, enabled, data source — `idmptmat -c` | 0.1 (R-group items); 0.2 (resolved links) |
| `tmats show --tree FILE` | Hierarchy: data sources → recorders → channels, then every group by occurrence and code path — `idmptmat -t` | 0.1 |
| `tmats show --raw FILE` | The TMATS exactly as stored — `idmptmat -r` | 0.1 |
| `tmats extract FILE.ch10 -o OUT` | Write the setup-record TMATS bytes to a file | 0.1 |
| `tmats checksum FILE` | Print the `G\SHA` value (`2-` + 64 lower-case hex) | 0.1 |
| `tmats checksum --flex [--include ...] FILE` | irig106.org flex signature (non-standard; UC-16) | 0.1 |
| `tmats verify FILE` | Compare the embedded `G\SHA` with the computed value | 0.1 |
| `tmats stamp FILE -o OUT` | Write a copy with `G\SHA` inserted or updated | 0.1 |
| `tmats validate FILE` | Validation report (UC-06/07) | 0.3 |
| `tmats diff A B` | Compare two documents (UC-11) | 0.2 |
| `tmats check FILE.ch10` | Check every packet against the setup record that governs it (UC-18) | 0.2 |

## 3. Behaviour already decided

- **Input by content, not extension.** A file that starts with the Chapter 10
  packet sync pattern is a recording; anything else is TMATS text.
  L2-CLI-005/006 derive this selector and acquisition/failure boundary; route
  selection is not packet validation or permission to retry the other mode.
- **The configuration timeline.** From 0.2, `show` lists every setup record
  with its kind (first, repeat, change), its findings, and what changed
  (UC-19; L1-CH10-008), and `check` runs the packet check (UC-18).
- **Every setup record.** A recording's setup records are all reported, in
  file order (the `idmptmat` first-packet-only limitation, defect D2, is not
  repeated). Each record's output is labelled with its position and packet
  offset.
- **Recordings are memory-mapped**, and packet finding/trust comes from
  `irig106-core` (ADR-0036), with borrowed DATA spans into TMATS slicing/
  assembly. The temporary CLI reader is superseded; no header or packet-data
  checksum implementation is added here. DATA need not be contiguous or
  copied/coalesced; input blocks remain alive for the borrows.
- **Plain text by default, JSON on request**, with a documented, versioned
  JSON schema so scripts and `irig106-cli` can rely on it.
- **Labels are the code names actually read.** A value is never shown under
  another code name (defect D4).
- **Extraction preserves bytes.** L2-CLI-008 derives exact extraction of one
  explicitly selected eligible record; record eligibility/selection stays
  separate. L2-CLI-009 requires preserving input files, including aliases,
  before destructive destination access for tool-opened outputs; shell
  redirection is outside that acquisition guarantee. Implementation is BLOCKED
  on owner decisions for `-o` overwrite, fail-closed identity checks for
  pipes/devices/stdout/network shares, and extraction eligibility under
  integrity/encoding findings. No pending policy is selected.
- **Nothing is changed in place.** Commands that produce modified TMATS
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
  of every help and error message (which the ASCII-only rule below needs).
  What it costs: no generated shell completions and more parsing code to
  own; that code is kept table-driven and every flag, value, and usage error
  has a test. The scaffold's `--version` / `--help` handling already follows
  this approach.

## 4. Proposed rules (to confirm)

- **ASCII-only output.** Everything the tool writes to stdout and stderr is
  ASCII — data, diagnostics, and help — so a Windows console at an OEM code
  page never garbles it (a lesson from `mie-decoder`, whose CLI enforces the
  same rule). TMATS bytes are written through unchanged only by `show --raw`
  and `extract`, which are byte-exact by definition.
- **Exit codes** (draft): `0` success; `1` the check failed (`verify`
  mismatch, `validate` findings at or above the failure threshold); `2` usage
  error; `3` input could not be read or is not TMATS/Chapter 10; `4` output
  could not be written. To be fixed before 0.1 and never renumbered.
- **Streams:** results on stdout, diagnostics on stderr; `-o FILE` writes the
  result to a file instead of stdout.

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
(flags, output layouts, exit codes) is written with the L2/L3 requirements.
