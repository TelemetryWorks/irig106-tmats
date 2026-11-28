# Remaining tmats owner decisions — 2026-11-28

Joey signed the decisions preserved verbatim in [ADR-0038](../adr/0038-owner-lookup-streaming-and-cli-policies.md).
This docs-only record removes the corresponding owner-choice holds, not the
source/API, analysis/review or executable-verification gates. No code,
manifest, dependency, reference execution, publishing or core change occurs.
The pending #59 records std separately. The owner-open INT-034 precedence
and report-format questions remain untouched.

## Flex qualification pin

The owner selected original bbaggerman/irig106lib, not the atac fork. The
maintainer selected this exact qualification snapshot through the public
GitHub API on 2026-11-28 (HEAD as inspected, not a moving branch contract):

- Repository: https://github.com/bbaggerman/irig106lib
- Revision: `1e9c5d731566c20868e94d6617faa3e1f4d594a4` (committer date 2024-10-18).
- Entry under qualification: enI106_Tmats_Signature; flags and source API
  must be checked together before accepting vectors. Pinning alone neither
  qualifies the build/API nor supplies expected output values.

| Reference file at that revision | SHA-256 of inspected bytes |
|---|---|
| [src/i106_decode_tmats.c](https://github.com/bbaggerman/irig106lib/blob/1e9c5d731566c20868e94d6617faa3e1f4d594a4/src/i106_decode_tmats.c) | `2aeee0e61f42619292061398d06e8a641040017b9a92fad2c6621d745265a177` |
| [src/i106_decode_tmats.h](https://github.com/bbaggerman/irig106lib/blob/1e9c5d731566c20868e94d6617faa3e1f4d594a4/src/i106_decode_tmats.h) | `7fa5357bb3ae88dabcc4b9256e00ef9106fa3929bacdf2cb8b222c8ade34a8dd` |
| [src/i106_decode_tmats_common.h](https://github.com/bbaggerman/irig106lib/blob/1e9c5d731566c20868e94d6617faa3e1f4d594a4/src/i106_decode_tmats_common.h) | `f5841b5e36672d65e81da7b131693828c3e78f8661dce894d6b4fec6b8e965ee` |

Only revision/metadata/file hashes are recorded here; reference source is
not copied into the repository. Complete build/transitive-source identity,
output values and arithmetic/API qualification belong to bounded GitHub CI
with synthetic inputs only and no new project dependency. No local compile
or reference run occurred. Divergent revisions/forks become labelled known
differences, not silent substitutes. Numerical vectors remain UNQUALIFIED
until actual reference outputs and independent arithmetic checks are recorded.

## Decision boundaries and planned evidence

- Exact literal lookup returns raw text for unknown/vendor names too;
  registry-only scope limits wildcard patterns, not raw source access.
  One '*' occupies one complete name part or index; no partial/recursive
  wildcard or regex. Known projection/index policies remain; no private
  name grammar is inferred. Remaining CLI argument forms are not invented.
- First contributing fragment orders inventory; evidenced closure/takeover
  separately governs. No ordering by completion or backdating.
- Zero per-packet steady-state allocation applies to the tmats host's
  non-setup path. Measure with an untimed allocation counter after warm-up;
  setup allocations, initialization and rendering are separately scoped.
  Pool refill/bookkeeping on that steady-state host path cannot be hidden.
- Retained DATA pool credits stay inside the resource domain. The cap must
  leave a verified reader-progress reserve for admitted frames under the
  actual core API; no arbitrary percentage, one-block reserve or numeric
  default is selected by this note. Exceeding the cap fails/reports the run
  with the typed assembly-budget error; releases its in-scope ownership
  truthfully and lets core traversal continue. No silent eviction, invented
  EOF/Complete, intact-run retry, early coalescing or automatic core drain.
  Caller-held references and core capacity must be included in the progress
  proof; a synthetic credit formula is not that production proof.
- Mixture finding uses only Complete records in an evidenced/asserted session;
  Complete ASCII plus Incomplete XML keeps qualified XML evidence without
  the formal mixture finding. Whole-recording-as-session is explicit/labelled,
  never inferred. Packet/channel conformance remains core evidence, with the
  edition wording from the existing archived source review, not a new reader.
- Strict extraction refuses the listed bad-data categories/class, not every
  finding or severity. Informational notes alone do not refuse. Typed class
  membership must be explicit/reviewed, not inferred from message/severity;
  all findings/raw bytes remain, and default extract still proceeds.
- Explicit '-' selects stdin; no omitted-path/error fallback. This chooses
  no other public argument form or rendered usage/exit code.
- Criterion retention is rejected. Remove the prototype benchmark dependency
  during the restructure; future benchmarks are informational ignored CI
  tests with paired untimed allocation/copy evidence, no new dependency.

The planned fixture additions and updated earlier rows assert these facts
independently; they are not implementation or measured performance evidence.
Other resource-accounting defaults/caller ownership and recovery details
still need verified design, without reopening the selected progress policy.

## Carried review follow-ups

This PR carries the eleven final concurrences at #87 e0a0cce, the explicit
F2 default refusal for unresolved/ambiguous/cannot-evaluate possibly attached
X items, and the BLOCKED (precedence arm) fixture label. The final-check
record names the reviewer and exact head; no new-head concurrence is claimed.
