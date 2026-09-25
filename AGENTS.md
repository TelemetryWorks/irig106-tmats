# AGENTS.md

The working context for this repository lives in [CLAUDE.md](CLAUDE.md), which
is written for any coding agent rather than one in particular. Read it first.

The two things most likely to trip an agent up, in short:

1. **The standard is the authority, and memory is not a source.** Every code
   name, keyword, range, and rule must cite IRIG 106 Chapter 9 (edition,
   table, row). The prototype this project replaces (git tag `prototype-0`)
   mapped several P-group code names to the wrong meanings and wrote tests
   that asserted those wrong meanings — plausible, confident, and wrong. If
   you cannot cite it, raise it rather than encode it.
2. **The project is in a documentation-first redesign.** Use cases,
   architecture, ADRs, and L1/L2/L3 requirements come before code, and the
   decisions already taken are listed in `docs/ROADMAP.md`. Whether the
   library should repair, normalize, or reorder a user's TMATS is a decision
   to raise rather than settle: the default is that nothing the user wrote is
   ever changed or dropped.
