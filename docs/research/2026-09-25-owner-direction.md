# Owner direction log (2026-09-25)

The project owner's instructions and answers that shaped the redesign,
recorded **word for word** and in order, so none of it lives only in a chat
session. Each entry notes where the resulting decision is recorded. Questions
put to the owner are paraphrased in brackets; the owner's words are quoted
exactly, including spelling.

## 1. The redesign request

> irig106-tmats:
> - Remove number of test tracking from the ROADMAP,
> - the ROADMAP should be forward looking only (Grab the text from https://github.com/joey-huckabee/mie-decoder/blob/main/docs/ROADMAP.md to document this fact),
> - Fix the clippy warnings and formatting drift.
> - Do we need the build.rs in this repo?
> - Let's go ahead and bump up the project edition to 2024 and rust-version to the minimum version for 2024.
> - I need to question everything in this repo because you are smarter now and I want to make sure we are on the correct path for a great project.
> - Similar to this project https://github.com/joey-huckabee/mie-decoder I need the requirements to be broken down into L1.md, L2.md, L3.md, trace matrix and use the script to build it.
> - We need ADRs
> - Keep the PROJECT_STRUCTURE.md up to date.
> - Grab the CLAUDE.md from https://github.com/joey-huckabee/mie-decoder in particular the Git conventions. The rest of the CLAUDE.md we need to make our own.
> - Please remove all references to CLAUDE from the commit history because it shows up in my GitHub repo. I need it removed from the remote as well. CLAUDE shows up as a contributor now and it would be nice if that was removed.
> - Grab the AGENTS.md from https://github.com/joey-huckabee/siat-foreign-analysis, make it compatible with this project
> Dont touch any of the other projects until we can make some of these changes there as well.

Answers that followed: do the whole list; rewrite and force-push the history;
no Claude co-author trailer on any commit. Recorded in: `docs/ROADMAP.md`,
`CLAUDE.md` (Git conventions), `AGENTS.md`, `docs/PROJECT_STRUCTURE.md`;
the prototype review is in `docs/research/2026-09-25-prototype-review.md`.

## 2. Core design, versions, scope, shared types

[Asked: rebuild or patch; which spec edition; which features to cut; where
the shared types go.]

Recorded verbatim in `docs/research/2026-09-25-editions-and-extensibility.md`
section 1, with the answer given in section 2. Shared types: "Use irig106-types
now".

## 3. Approval of the plan

> 1. Do you approve this direction and release plan? Yes and if we need to sort of start from scratch to make sure we do not have any hidden surprises later that is probably fine so we can go slow, methodical and incremental.
> 2. Do you have real TMATS or Chapter 10 files I can test against? yes I can get some, but I will need some time to gather them so I would like for you to go ahead and test without for now. I think doing both is good.
> 3. Should "repair" become suggestions only? Yes since you recommend. Do we want an option to repair? This does not make sense to have but I want to ask. Also back to the starting over question, would this even be a feature in the new design.
> 4. I want to capture all the RCC 106 Telemetry Standards, probably as Git LFS files? Or some sort of proper GitHub binary storage for these. But not in the TMATS repo. Make a suggestion. Make sure we preserve the original links as well.
> 5. After we get the plan established I do want to get back to architecture, adrs, requirements, roadmap, data flow, use cases, etc before we write to much code so we understand what this project actually does and will do in the future.

[Asked: archive storage, owner and visibility, coverage.] Answers: "New repo
+ Release assets"; "TelemetryWorks/rcc-106-standards, public"; "All
editions, full + chapters". Recorded in: `docs/ROADMAP.md` (design phase),
`CLAUDE.md`, the `rcc-106-standards` README.

## 4. Other documents and test data

> Do you think we should capture the coding guides and other documents as well?

[Asked: which captures to do; whether to include MIL-STD-1553B.] Answers:
"Extend standards archive", then:

> Leave out the 106-96 installer, We will have links to the data samples and test oracles and they will be downloaded locally for test then we will simulate the different examples for our CICD pipeline and fuzzing tests instead of trying to manage the data set. I think you need a username and password anyway (which I have)

MIL-STD-1553B: "No, references only". Recorded in: `docs/TEST-DATA.md`.

## 5. igDisplayTMATS and the test tool

Recorded verbatim in `docs/research/2026-09-25-irig106org-tools.md`
section 1.

## 6. The CLI, signatures, and the conversation file

> Take the plan to fold this into the design documents along with the newly added CONVERSATION.md information (do not commit this file) and enhance the plan. Let me know what the additions from the CONVERSATION.md file will be. We def want a rust version of idmptmat and the three output format versions and we should add in the different signature functionality into our CLI tool - Give me a better name than idmptmat.

> I assume the idmptmat tool will be a new crate in this tmats project? Is that the correct assumption? we should align the CLI version with the Lib version at all times.

> We want to test for the known but in idmptmat so that does not happen to our cli tmats tool

> Make sure we understand how publishing to crates.io will happen or if that will happen for the cli tool

> irig106-cli repo and crates.io is a more robust and complete CLI tool instead of a simple TMATS only CLI tool.

Recorded in: `docs/USE-CASES.md` (UC-15, UC-16, UC-17), `docs/TEST-DATA.md`
(D1–D7), `docs/RELEASING.md`, `docs/ROADMAP.md`, `CLAUDE.md`.

## 7. Names, the placeholder, and the architecture

> irig106-tmats-cli will be the crate name and `tmats` will be the binary and command name. Fix The placeholder: irig106-tmats 0.0.1 on crates.io still links to irig106-write; the first real publish fixes that. I dont necessarily want to follow the legacy tools Pipeline sketch. Lets make our architecture better, faster, less error prone. What can we do better here?

Recorded in: `docs/ARCHITECTURE.md` (sections 1–2), `docs/RELEASING.md`
(the placeholder). The placeholder fix is a 0.0.2 publish with the corrected
`repository` link; it waits on a refreshed crates.io token.

## 8. Capturing everything

> Make sure everything word for word is captured in our documentation because I think every word is important. Then Let me know when we can delete the CONVERSATION.md file - but make sure we will not lose any of the important decisions or information which is relavent to our project. The good stuff should be written down in our docs and not just held in memory. I will have to refresh the token later so bring it up again on the next round. I will be away from my keyboard for a while.

> Are there any SVGs we can generate representing the diffent aspects of this architecture? A picture is worth a 1000 words remember.

Recorded in: this directory (`docs/research/`) and `docs/diagrams/`.

## 9. The CLI's argument parsing

> Give me a Token reminder for the irig106-tmats 0.0.2 placeholder. I will be away from my keyboard for a while.
> Go with hand-rolled argument parsing like mie-decoder

Recorded in: `docs/CLI.md` (section 3), `docs/ROADMAP.md`, `CLAUDE.md`.

## 10. Opening files and writing TMATS

> a. Should the library ever open whole Chapter 10 files itself? The library should not, the simple test CLI should be able to open the file enought to read the tmats packet or a tmats file. I am guessing we need to write a TMATS file as well, but that will be a much later release I assume. What do you think?

Answer given: agreed that the library never opens files; writing TMATS *text*
is in 0.1 (the library produces bytes, the CLI writes the file); creating or
modifying content is 0.5; putting TMATS into a Chapter 10 recording is a
setup-record payload from the library plus packet writing in `irig106-write`.
Recorded in: `docs/USE-CASES.md` (section 3, "The filesystem boundary";
open question 1 resolved).

## 11. The remaining use-case questions

> go with your suggestions for 2, 3, and 4

The suggestions accepted: (2) a value over a *recommended* maximum length is a
warning by default, adjustable by severity policy; (3) `irig106-ch10-reader`
reports a mid-recording setup-record change by default, in one line naming
what changed; (4) the test corpus reproduces the irig106.org vendor samples
first, then the owner's program files. Recorded in: `docs/USE-CASES.md`
(section 7, UC-06, consumer notes) and `docs/TEST-DATA.md`.

## 12. The CLI design decisions

> go with your suggestions for all five

Accepted: all setup records by default with `--record N`; flex `--include`
flags mapped one-to-one onto `irig106lib`'s and its `OO-SSSSSSSS` output;
no `idmptmat`-compatible mode (semantic comparison only); JSON schema in
`docs/schema/` with a `schema_version` field, breaking changes are breaking
releases; `cargo-dist` for release binaries. Recorded in: `docs/CLI.md`
(section 5), `docs/RELEASING.md`, `docs/PROJECT_STRUCTURE.md`.

## 13. Who reviews registry interpretations (2026-09-26)

> Two-person rule for interpretation reviews

Recorded in: `docs/ROADMAP.md` ("Team design review", T1): an author and an
independent reviewer, both people, enforced by the registry CI check.

## 14. Scope of Appendix 9-E (2026-09-26)

> What do you recommend

> Go with your recommendation and apply T2

Recommendation adopted: the library parses, validates, and describes derived
parameters; `irig106-decode` evaluates them; a reference evaluator exists only
in the tests, to prove the Table E-6 precedence; the errata are read as "`==`
is the operator; `= =` is accepted with a warning". Recorded in:
`docs/ROADMAP.md` (T2, coverage item 3), `docs/ARCHITECTURE.md` section 5,
ADR-0024, `docs/L1-REQ.md`.

## 15. Where the setup-record assembler lives (2026-09-26)

> Go with your recommendation and apply T3, and make sure we have a test for One setup record can span multiple consecutive packets. This should also be documented with a picture (svg)

Recommendation adopted: the assembler is in the library (fragments with
provenance in, complete record out; no I/O); packet slicing stays in the CLI's
reader until `irig106-core` exists. The multi-packet case has a named
acceptance test (`docs/TEST-DATA.md`) and a diagram
(`docs/diagrams/setup-record-assembly.svg`). Recorded in: `docs/ROADMAP.md`
(T3), `docs/USE-CASES.md`, `docs/ARCHITECTURE.md` section 6, ADR-0025,
`docs/L1-REQ.md`.

## 16. `build.rs` and the registry (2026-09-26)

> I read this "spec-cited registry generated by a script (no `build.rs`);" When I originally said no `build.rs` that was for applying the version to the project from the TOML file which is not necessary anymore. If we need to use the `build.rs` filr for the registry that sounds like the right path, but I will let you tell me if that is correct.

> Keep ADR-0005 and record it in the log

Clarification recorded: the prototype's `build.rs` generated the attribute
registry from `data/attributes.toml` — each attribute tagged with the IRIG 106
edition that introduced it — so it was the same job the new registry does, not
a project-version step. ADR-0005 is kept: the registry is generated by a
script into checked-in Rust source, with a CI `--check`, and no `build.rs`.
Reasons given: extraction from the Chapter 9 PDFs needs `pdftotext` and the
archived PDFs, so it is an offline script in any case; the two-person
interpretation review (ADR-0022) needs reviewers to see the exact generated
code change, which a build script hides; dependents would otherwise compile
and run a build script and its TOML parser on every clean build; some
organisations flag or ban build-time code; generated source in the tree is
searchable and debuggable. `build.rs`'s one advantage — never stale — is
provided by the `--check` job. `build.rs` would be preferred only if the
generated data were large enough to bloat the repository or varied by build
target; neither applies.

## 17. Recording work for other repositories (2026-09-26)

> How do we record this in the other projects so this work does not get lost?

> Leave the roadmap item for now.

The cross-repository items stay in `docs/ROADMAP.md`, "Work for other
repositories" (X1–X7); no issues are filed in the other repositories yet.

## 18. Applying T4, and the Appendix 9-C erratum kept suspect (2026-09-26)

> Apply T4 now but,
> Can you confirm this? I have doubts that you found so many errors.
> - The standard's own example has an error. On page C-8 of Appendix 9-C, colons are printed where semicolons belong, e.g. D-1\MML\N-1-1:2: D-1\MNF\N-1-1-1:1: …. There are at least 12 of these, all after
> D-group counters, and I confirmed them on the page image. Read strictly by the rules, each one turns several attributes into one attribute whose value contains the rest.
> Which will affect this
> 5. Suspected missing semicolon: when that pattern appears, the library reports "possible ; typed as :" and suggests an edit. It never splits the text silently. The Appendix 9-C test fixture keeps the
>  standard's text exactly and checks for these reports.

> Lets still mark this as suspect so when we run against real data we dont forget I have some concerns.

Re-verification, from scratch:

- **The count was wrong; the finding stands.** The first scan's pattern
  consumed the start of each following attribute and so skipped every second
  occurrence. A non-overlapping scan finds **18** places in 106-24R1
  Appendix 9-C where an attribute ends with `:` instead of `;`, all after a
  D-group counter (`D-1\MML\N-…`, `D-1\MNF\N-…`, `D-2\MML\N-…`,
  `D-2\MNF\N-…`), all in D-1 and D-2. D-3 and D-4 use `;` correctly.
- **The text layer and the page agree.** The same lines extract `;` correctly
  after other attributes in the same font, and the rendered page C-8 shows
  `:2:` and `:1:` beside correctly printed `;`.
- **No other delimiter anomaly.** Of the 364 attribute starts in the
  appendix's code-name example, every one follows a `;` or a line break
  except these.
- **Every edition since the example appeared.** The same 18 occur in 106-17,
  106-19, 106-20, 106-22, 106-23, 106-24, and 106-24R1; 106-05 to 106-15 do
  not contain this example at all.

Owner decision: apply T4, and keep the erratum and the diagnostic that
depends on it marked **suspect** until checked against real recordings.
Recorded in: `docs/ROADMAP.md` (T4, and "Suspect findings to confirm against
real data"), `docs/TEST-DATA.md`, ADR-0026, `docs/L1-REQ.md` (L1-READ-007).
