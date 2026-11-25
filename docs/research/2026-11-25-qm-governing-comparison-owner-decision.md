# QM: currently governing setup is the comparison target

Date: 2026-11-25. Owner: Joey. Source: follow-up owner decision in the
repository thread, adding QM to PR #45 after head
`d661d6e42833ac367d14f1016bb483f68d60fb0b`.

## Owner wording (verbatim)

> Each later setup copy is compared with the CURRENTLY GOVERNING setup: the original sealed record, or the latest copy that validly took control. A copy that did not take control does not become the comparison target.
>
> Guard 1: if it is unresolved whether a copy took control (e.g. a changed copy failing integrity in warn mode, or Ambiguous), later copies report the comparison target as unresolved. Never pick one.
>
> Guard 2: raw byte-equality to the immediately previous copy is recorded as INFORMATION only. It never changes eligibility, governing or classification.
>
> Every earlier copy's findings are preserved and never overwritten. Earlier governing setups stay available for data already decoded under them.

## Register and verification scope

INT-053 records this owner sign-off under ADR-0033. Independent AI review is
PENDING; no concurrence, implemented state machine or passing runtime tests
are claimed. The earlier same-day record's “QM pending” is dated history,
now superseded by this decision, not an active open question.

The pairwise Cartesian product stays at 139,968 members / 39,072 classes;
its serialized members/classes are unchanged. QM is a separate sequence
contract, with named bounded fixtures in the current matrix note and
TEST-DATA, not a multiplication of independent pairwise inputs. A sequence
must retain an actual governing-candidate identity or an unresolved target,
plus independent adoption evidence and historical findings. Multiplying
pairwise dimensions alone would not test that state.

The fixtures cover at most three later copies after an original governing
setup, including B→D, H→B, a third-only checksum, changed SRCC, valid takeover,
definite non-takeover, unresolved takeover, previous-copy information-only
comparison, and retained prior decoding provenance. These are planned cases,
not full arbitrary-length sequence coverage or an adoption algorithm.

A known refusal/non-takeover keeps the established target; an unresolved
takeover does not. QD's block on new adoption does not, by itself, prove which
setup governed the recorder's later data. Neither warn-mode findings nor raw
identity resolve that missing fact. A verified adoption assertion must come
from the separately reviewed applicability/transition rules, not this matrix.
