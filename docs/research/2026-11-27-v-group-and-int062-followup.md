# V grouping and INT-062 follow-up: family 2 batch 31

Date: 2026-11-27. Documentation-first derivation and proposed INT-063;
amended INT-062 remains pending owner sign-off and amended-wording AI review.
Earlier dated research records remain history, not updated specifications.

## Source inspected

106-24R1 Chapter 9 section 9.5.13 p. 9-171 defines only V-x\ID and V-x\VN
as standard V headers, with other attributes in V-x\acr\attribute-string.
VN and the private-path acronym are three-character vendor identifiers;
vendor equipment details are not defined by the standard. Table 9-2 DATA
SOURCE ID G\DSI-n p. 9-11 defines descriptive textual source names and
explicitly links to V-x\ID. Source PDF from the [official archive](https://www.trmc.osd.mil/wiki/download/attachments/335389015/Chapter9.pdf?api=v2),
SHA-256 `dfc5ebfb3e6f02f58389126863d43a45d89c7e2d0533a1c690d9537531c740cb`.

ADR-0008 accepts source/vendor grouping. ADR-0002/0003 retain all raw items;
ADR-0026 keeps namespaces and all candidates explicit. Approved INT-038
handles occurrence identity, INT-039 known scalar duplicates, INT-041
unavailable interpretation and INT-048 record ambiguity/cut facets. ASCII
comparison comes from section 9.4.2 p. 9-2, not INT-036's name projection.

## V inventory and source context

L2-EXT-003/004 and L3-EXT-009–016 derive the V portion of L1-EXT-003.
Observed occurrences/private acronym namespaces remain enumerable without
inventing vendor confirmation. Preserve header-only contexts, every private
item and the complete opaque suffix/value, including unfamiliar nested paths
or digits. No counter, private field range, vendor plug-in or clock rule is
inferred from that shape.

Keep local ID/VN candidates and their evaluated field states. This batch can
receive reviewed field results rather than decide new blank/character/value
rules; unknown validation is unavailable, not automatically Invalid. Known
scalar multiplicity remains Ambiguous. Header absence is missing binding
evidence, not a newly invented required-presence violation.

A usable V ID can match G\DSI-n textual identities in the same document
under the existing ASCII rule. The V occurrence and G entry indices need
not be equal. Source labels 01 and 1 are textually different; neither index
canonicalization nor INT-055's decimal field rule converts them into one
source. A local source label and successful G target resolution are separate
facts. Zero/one/many source candidates never select an arbitrary winner.

Namespace/group views retain every contributing occurrence and source span;
equal source/vendor labels do not erase identities or permit another V
occurrence to supply missing headers. One assembled record remains one
document. All source, record, encoding and cut qualifications stay attached.

## Proposed INT-063: owner summary

The source defines VN as identifying the vendor and controlling private
interpretation, while each private path also carries its vendor acronym.
It gives no recovery precedence for missing/duplicate/unusable VN or a path
that disagrees. Raw inventory and confirmed attribution must be separate.

- **A — recommend:** keep every observed namespace, but confirm the vendor
  only if one usable VN agrees with the path. Otherwise keep unavailable
  attribution with evidence; no default, duplicate winner or source rewrite.
- **B — path-first:** use the path acronym as labelled attribution even when
  VN is missing/bad/disagreeing, preserving header states/findings. More
  permissive, but chooses a recovery authority not printed in the section.
- **C — usable-header-first:** one usable VN supplies semantic attribution
  even for disagreeing private paths, labelled and with raw namespace kept.
  No usable unique header still means no selected vendor. The VN definition
  supports this emphasis, but mixed-vendor occurrences may be misattributed.

All are proposed options, not signed policies or exact report wording.
INT-063 needs owner sign-off plus independent AI check before affected-case
implementation. Four Option-A fixtures are explicitly BLOCKED.

## INT-062 amendments requested after #63

Registry-only A has a real cost: X targeting vendor-private V items or other
non-registry codes always remains cannot-evaluate, even with the original
present. Standard V ID/VN are in the vocabulary; the private fields are not.
A is still recommended, but that limitation is now explicit in the register.

New hybrid **C** tries the registry first, and permits document-present names
ONLY for groups with no registry vocabulary, labelled `document-defined
vocabulary`. It is not a generic fallback on a registry miss. The owner still
needs to settle group/private-namespace granularity: V has standard ID/VN
but no standard private-item vocabulary. No blanket V fallback is approved
by describing the hybrid. Nonunique/missing grammar, index and qualification
guards still apply; merely seeing a private name cannot invent a decoder.

Coordinator direction also fixes the catalog edition: use the 106-24R1
baseline until edition deltas exist. Keep that registry basis visible beside
the existing INT-044 edition-selection/fallback basis; do not overwrite a
raw older G\106 or pretend the catalog covers an older spelling. The
coordinator-supplied legacy R\DST/R\CDT example is cannot-evaluate when the
legacy original lacks baseline metadata, never an automatic alias/repair.
The baseline CHANNEL DATA TYPE row is R-x\CDT-n (Table 9-4 p. 9-50); this
note makes no unsupported claim about a particular historical edition's
rename date/table. Exact historical mappings await cited delta work.

Catalog edition and lookup vocabulary are not new edition fallback rules:
INT-044's labels/reasons and original declaration evidence remain unchanged.
This does not decide the A/B/C vocabulary choice or approve INT-062; amended
wording and added fixtures need independent review before the owner signs.
INT-061's unindexed-original choices also stay pending.

## Planned checks and gates

TEST-DATA adds V occurrence/header/source/namespace retention and proposed
policy cases, plus INT-062 cost/hybrid/baseline cases. Fixtures supply source-
cited names and independent item/state expectations; they do not declare a
whole minimal snippet conforming or unknown private syntax standard-defined.
No required-presence rule, acronym alphabet, new result schema or public
rendered wording is selected by these grouping contracts.

Only source-excerpt/fingerprint, trace, identifier/link and whitespace checks
run locally. No code, workflow, dependency, build/test, private sample, other
repository or publishing action is involved. #59 remains held for std and
requires rebase/recount before its eventual re-review. Pattern, extraction,
record-access/governance, allocation and report-format gates remain open.
Source freshness, deep development analysis and executable tests still
precede implementation; neither X nor V derivation is implementation proof.
