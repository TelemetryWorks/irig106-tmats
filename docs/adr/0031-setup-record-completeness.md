---
status: accepted
date: 2026-09-27
decision-makers: Joey
---

# A setup record is complete, incomplete, or ambiguous — each with its basis

## Context and Problem Statement

The standard gives no end-of-record marker: "A single setup record may span
multiple consecutive packets. When spanning multiple packets, the sequence
counter shall increment in the order of segmentation of the setup record,
n+1" (Chapter 11 §11.2.7.2). ADR-0025 and INT-011 (accepted) join
consecutive fragments and **end** a record "at an intervening packet, a
sequence gap, a CSDW change, or the end of input". INT-031 (proposed) calls
a record **incomplete** after "a sequence gap, missing or untrusted fragment,
a CSDW change within it, or the end of the file". The same three conditions
mean "finished" in one entry and "broken" in the other.

The team's spec alignment review (`docs/research/2026-09-27-spec-alignment-review.md`)
found the contradiction: "The documents do not yet provide an executable
decision rule that consistently distinguishes complete, incomplete, and
ambiguous runs."

Two further facts make a pure packet rule impossible:

- The sequence number cannot separate two records: "Each channel in a
  session shall have its own sequence counter" (§11.2.1.1 f), so two
  records back to back on channel `0x0000` continue one count.
- The text can: every code-name attribute ends with a semicolon, and
  "Semicolons are not allowed in any data item" (Chapter 9 §9.4.2). A body
  whose last non-blank byte is not `;` is cut inside an attribute.

## Decision Drivers

* No condition may mean "complete" in one rule and "incomplete" in another
* Uncertainty is reported, never resolved by guessing
* A record that is not proven complete must not silently govern data

## Considered Options

* **Three outcomes, each with the evidence that decided it**
* Two outcomes (complete, incomplete), treating every doubtful case as
  incomplete
* Keep the run rule and treat every end as complete

## Decision Outcome

Chosen option: **three outcomes** — the owner's decision, 2026-09-27 ("Yes
to all three, go ahead", accepting this record among six recommendations
from the review). The precise rules are register entry INT-034 (proposed),
under the two-person rule of ADR-0022.

- **Complete**: a run of joined fragments (INT-011's joining rule) whose body
  ends at an attribute boundary and which ends at another packet — basis
  "followed by another packet" — or at the end of input — basis "end of
  input", labelled, since a file may have been cut exactly at a boundary.
- **Incomplete**: the body is cut inside an attribute (wherever the run
  ends); a fragment is unreadable or untrusted; or a sequence gap splits
  text that continues across it (a missing middle fragment).
- **Ambiguous**: the evidence allows two readings — a sequence gap or a CSDW
  change where the text before it ends at an attribute boundary (two
  records, or a lost fragment that ended at a boundary); or a gap-free run
  whose text repeats a single-entry attribute such as `G\106` (two records
  back to back, or one malformed record). Both readings and the split point
  are reported.

Complete records govern (INT-028). Incomplete and ambiguous records do not
govern by default and keep their bytes (INT-031); a caller may accept an
ambiguous split, and every report then says so.

### Consequences

* Good: every run gets exactly one outcome, and the reason for it.
* Good: the tests the review asks for — a complete final record at end of
  input, a truncated record, a missing middle fragment, adjacent complete
  records — each have a defined expected outcome (`docs/TEST-DATA.md`,
  `setup_record_completeness_outcomes`).
* Bad: the assembler reads the text's final bytes, so completeness depends
  on the body format; an XML body (FRMT 1, not read, ADR-0015) is reported
  as of unknown completeness.
* Supersedes the ending half of ADR-0025's boundary rule and of INT-011; the
  joining half stands.
