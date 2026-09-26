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

## Priority 2 (verbatim)

> 2. Add explicit Appendix 9-E support.
>
> Derived parameters need more than a string-valued C-d\DPA attribute. Appendix 9-E defines function and formula forms, operator precedence, quoted measurement names, dependencies on other derived
> measurements, and triggering information. These are not addressed explicitly in the current architecture or L1 requirements.
>
> Add requirements for parsing and validating expressions, exposing their measurement dependencies, and representing trigger semantics. Runtime evaluation can remain with irig106-decode; this library should
> supply an interpretable description.
>
> This also affects the scanner: C-1\DPA:A?B:C; contains a legitimate colon inside the value. Only the first colon separates the code from its value. The expression parser must follow the appendix’s
> precedence rules, which explicitly differ from C. Source: Chapter 9 Table 9-11, Appendix 9-E §§E.1–E.7 (https://www.irig106.org/docs/106-24R1/chapter9.pdf).

## Priority 3 (verbatim)

> 3. Distinguish a setup-record packet from a complete setup record.
>
> Chapter 11 expressly permits one setup record to span multiple consecutive packets. The current design says each setup packet is read independently, while assigning reassembly outside the library and
> providing only a minimal CLI packet reader.
>
> That leaves a valid recording that the proposed CLI cannot correctly extract, validate, or checksum. Define an explicit assembly contract: packet fragments and their provenance enter an assembler; a
> complete TMATS body enters the document parser. Ownership can remain with the CLI initially and move to irig106-core.
>
> Also specify payload slicing using data length, excluding packet filler/checksum and handling the optional secondary header. This belongs in the derivation of L1-CH10-001 and L1-CLI-003
> (/C:/Users/Joey/Documents/GIT-GitHub/telemetryworks/irig106-tmats/docs/L1-REQ.md:158). Source: Chapter 11 §11.2.7.2 and §11.2.1.1 (https://www.irig106.org/docs/106-24R1/chapter11.pdf).

## Priority 4 (verbatim)

> 4. Define counter scopes, key namespaces, and ambiguous links precisely.
>
> The blanket wording in L1-VAL-002 is too broad:
> - X-group occurrence indices are explicitly permitted to be noncontiguous.
> - Nested counters apply within particular parent-index combinations.
> - Key uniqueness cannot mean that every key-bearing attribute across all groups has a different value: linked P and D data-link names intentionally match.
> - Duplicate link targets need an “ambiguous” result; preserving duplicates but silently choosing one would undermine the lossless design.
>
> Make each counter declare its governed index and parent scope, and each link declare its target namespace and cardinality. Apply case-insensitive comparison to keywords and link values, as well as code
> names; L1-READ-004 currently specifies only code names.
>
> Sources: Chapter 9 §§9.4.2, 9.5.1, 9.5.14; Tables 9-6 and 9-7 (https://www.irig106.org/docs/106-24R1/chapter9.pdf). Affected requirement: L1-VAL-002
> (/C:/Users/Joey/Documents/GIT-GitHub/telemetryworks/irig106-tmats/docs/L1-REQ.md:278).

## Priority 5 (verbatim)

> 5. Record standards inconsistencies instead of assuming tables are mechanically complete.
>
> There is a concrete example in the baseline: Table 9-4’s R-x\CDLN-n target list omits Q, but Table 9-10’s Q-d\DLN explicitly links from R-x\CDLN; §9.5.1(b) also describes the R-to-Q relationship.
> Generating links from only the recorder-side list would miss valid Q links.
>
> Keep an interpretation register containing the conflicting citations, the chosen behavior, and a focused test. Generate relationships from reviewed definitions considering both directions.
>
> Also correct the architecture’s statement that H-group relationships will be added when its tables are transcribed: §9.5.12 defines only H\TA and H\ST-n, reserving the remaining structure for
> organizations. There is no comprehensive standard H table waiting to be imported. Sources: Chapter 9 Tables 9-4/9-10 and §9.5.12 (https://www.irig106.org/docs/106-24R1/chapter9.pdf); architecture
> link-graph description (/C:/Users/Joey/Documents/GIT-GitHub/telemetryworks/irig106-tmats/docs/ARCHITECTURE.md:156).

## Priority 6 (verbatim)

> 6. Separate TMATS edition, recording-format version, and selected validation rules.
>
> G\106 identifies the edition used to generate the TMATS file and contains only two year digits. It cannot distinguish 106-24 from 106-24R1. The setup CSDW identifies recording-data compliance, and the
> 106-24R1 table still lists its latest assigned RCCVER value as 0x0E = 106-22.
>
> Therefore, differing labels should not automatically become an edition-conflict finding. Preserve both declarations and their meanings, then report the selected validation edition and any explicit
> fallback separately.
>
> ADR-0016 (/C:/Users/Joey/Documents/GIT-GitHub/telemetryworks/irig106-tmats/docs/adr/0016-edition-strategy.md:36) also needs to reconcile “unknown editions are not guessed” with automatically validating
> pre-2004 files against 106-04. That can be useful compatibility checking, but should not be presented as validation against the file’s actual edition. Sources: Chapter 9 Table 9-2
> (https://www.irig106.org/docs/106-24R1/chapter9.pdf), Chapter 11 §11.2.7.2 (https://www.irig106.org/docs/106-24R1/chapter11.pdf).
