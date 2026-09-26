# Team design review (2026-09-26)

The team's review of `docs/ARCHITECTURE.md`, the ADRs, and `docs/L1-REQ.md`,
recorded **word for word** as it was supplied by the project owner. The plan
for each item — what was verified against the archived standard, how it maps
to the existing documents, and how it will be addressed — is in
`docs/ROADMAP.md`, "Team design review". Items are appended here as they
arrive.

## Summary (verbatim)

> From the team:
> The foundation is sound, but I would not yet approve it as a complete TMATS design. Keep the lossless document, owned byte buffer, generated registry, explicit edits, and library/CLI separation. The main
> gaps are in semantic validation, appendix coverage, and setup-record handling.
>
> I reviewed ARCHITECTURE.md, all 21 ADRs, L1 requirements, and supporting design documents against the manifest-verified 106-24R1 Chapter 9—including Appendices 9-A through 9-F—plus Chapters 6 and 11 where
> needed. This is a baseline design review, not a completed row-by-row audit of every supported edition or a performance measurement.
>
> These are my priorities before implementation. There are seven plus some additional comments when you are ready.

## Priority 1 (verbatim)

> 1. Make the registry an executable specification, with reviewed interpretations.
>
> ADR-0004 and ADR-0006 correctly centralize definitions, but storing the seven usage fields is insufficient. Conditions need explicit semantics: which occurrence they refer to, whether they follow a link,
> what happens when a dependency is missing or invalid, and whether defaults participate.
>
> For example, Table 9-11 gives C-d\DPNO a default of 1 in its definition prose, rather than a separate Default field. A transcription limited to usage fields misses it. Likewise, validation that only
> visits existing attributes cannot detect an absent required attribute such as G\106.
>
> I recommend a registry containing both the original cited text and reviewed executable rules, with separate validation passes for existing attributes, missing required attributes, counters, and
> relationships. Effective values should distinguish explicit, defaulted, missing, invalid, and ambiguous, without inserting defaults into the stored document. This strengthens L1-REG-002 and L1-VAL
> (/C:/Users/Joey/Documents/GIT-GitHub/telemetryworks/irig106-tmats/docs/L1-REQ.md:230). Sources: Chapter 9 §9.5.1(a), Tables 9-2 and 9-11 (https://www.irig106.org/docs/106-24R1/chapter9.pdf).
