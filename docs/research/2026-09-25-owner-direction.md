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
