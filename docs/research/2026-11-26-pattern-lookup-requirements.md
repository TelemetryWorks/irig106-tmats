# Pattern lookup: family 2 batch 26

Date: 2026-11-26. Requirements/acceptance-specification draft for independent
review. No executable lookup coverage or new interpretation approval claimed.

## Existing authority and source inspection

L1-VIEW-001 and UC-03 require finding by code name and code-name pattern,
including every R-1\TK1-n. Batch 5's L2-VIEW-001 and L3-VIEW-001/002
already derive exact-key candidate retrieval, raw locations and empty-value
versus absence. This batch adds the missing structural-pattern child rather
than a new value resolver or report format.

Inspected 106-24R1 Chapter 9 section 9.4.2, printed p. 9-2, defines
single/multiple-entry names, numbered entries and case-insensitive TMATS.
Table 9-4 TRACK NUMBER/CHANNEL ID, printed p. 9-48, gives R-x\TK1-n.
The [official Chapter 9 PDF](https://www.trmc.osd.mil/wiki/download/attachments/335389015/Chapter9.pdf?api=v2)
has SHA-256 `dfc5ebfb3e6f02f58389126863d43a45d89c7e2d0533a1c690d9537531c740cb`.
That row supplies the example's literal path/index structure; approved
INT-036/038 supply name projection and canonical index identity. Its numeric
value range is not the grammar for occurrence/entry index identity.

## Query semantics without a new query language

A query supplies a cited reviewed structural pattern and explicit bindings
for some or all index positions. For the TK1 row, bind recorder occurrence
x to identity 1 and leave channel-entry n unbound. The result contains each
matching source occurrence for that recorder, in document order. Bind n as
well for the same ordered items as an exact-key query.

This describes a semantic request, not a string parser. Public Rust query
representation, wildcard spelling, escaping, regex/glob syntax and data
structures remain API review/implementation work. It does not approve
recursive wildcard paths, substring searches or arbitrary private-grammar
inference. Use reviewed pattern metadata already needed by structural name
parsing; the 0.1 query does not depend on completing the full 0.2 registry
or add the 0.3 user-definition overlay feature.

Literal path components and index arity must match completely. The 1 in
literal TK1 is not an unbound index, and an unbound entry position does not
match an extra path segment or a value. Numeric identity compares canonical
digit strings, so 01 and 1 denote the same bound occurrence, without a
machine-integer conversion. INT-055's ten-digit field-value limit must not
leak into code-name index identity; retain raw spelling/findings and leave
range validation separate.

## Candidates and evidence

Keep all source occurrences, including exact repeats and normalized-index
collisions. Return original item IDs, name/value spans, bytes and available
fragment provenance. Never sort entries numerically or deduplicate them just
to answer a pattern. Returning raw candidates is not singular resolution,
validation, a default or a data-channel/time-channel association.

A value containing high/control bytes, an empty value or an invalid numeric
field is still retrievable when its name has a usable complete key. Name
projection is interpreted metadata; query operations do not change source
bytes or replay. A malformed/opaque name without a complete usable key must
remain raw evidence, not a guessed pattern match.

INT-037's leading encoding-marker gate makes interpreted lookup unavailable,
not a successful empty query. Unsupported pattern metadata/bindings likewise
cannot masquerade as zero matches. If matching is available and finds zero
usable-key items, the empty collection still does not prove a condition is
false: INT-035 excludes unterminated suffixes from lookup, while INT-042 and
L3-VAL-010–012 can use compatible related evidence to make a separate
physical-presence predicate cannot-evaluate. No damaged-name repair is added.

One assembled setup record is one lookup document under INT-039. Preserve
cross-fragment duplicates, keep separate records/documents isolated, and do
not borrow another recorder's missing entry or a default from a view cache.
Queries do not decide which record governs or change INT-011/045 boundaries.

## Review and implementation gates

L2-VIEW-015 and L3-VIEW-058–064 are a partial completion of the raw lookup
contract; they do not implement general linked views, vendor/X association,
full registry generation or validation. No new standard interpretation or
owner choice is needed for this retained raw candidate operation.

Only the existing approved INT-035–039 behavior is relied upon here. A
future link/reference implementation relying on pre-ADR-0033 INT-004 (index
letters across different table references) still needs its independent AI
check before implementation; this pattern query does not approve that entry
or infer links by matching reference letters.

Source freshness, development analysis, concrete API review and meaningful
executable tests remain implementation gates. Extraction's three owner
blockers and the owner report/rendered-format hold remain unchanged. No
pattern result presentation or public error spelling is selected here.

## Planned independent verification

TEST-DATA lists bounded synthetic cases with explicitly listed expected
item identities/order. Match the inspected TK1 row without interpreting its
values. Vary binding, source order, duplicate occurrences, noncanonical index
spelling and omitted controls in a name. Include a 40-digit index to prevent
accidental integer conversion or numeric-field digit limits.

An independent fixture list supplies expected IDs/ranges. Fully binding a
pattern can additionally be compared to exact lookup, but the two paths
could share the same bug; that equality alone is not the oracle. Compare
both against literal expectations. Fragment provenance fixtures split the
same source into two consecutive same-channel packets with sequence +1 and
matching FRMT/RCCVER, so ADR-0025/INT-011 identify ONE assembled record;
separate-record fixtures keep the source documents distinct.

Future bounded CI should vary permutations and span partitions, assert no
source mutation, and preserve every duplicate occurrence. No local parser
build, property sweep or cross-repository API inspection is performed by
this docs-only PR; all named cases remain planned, not executed coverage.
