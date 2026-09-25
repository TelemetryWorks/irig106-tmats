# Editions, extensibility, and performance (2026-09-25)

Research record of the design discussion that set the edition strategy, the
extensibility model, and the performance stance. Section 1 is the owner's
direction, section 2 the answer given (both word for word), section 3 the
Chapter 9 text the answer relied on, quoted from the archived 106-23 edition.
Decisions drawn from it are in `docs/ROADMAP.md`, `docs/USE-CASES.md`, and
`docs/ARCHITECTURE.md`.

## 1. The owner's direction (verbatim)

On the core design:

> I am not sure the correct answer is here. Most likely your recommendation is
> probably correct but I am wanting to get back to basics and make sure we are
> aligned with the irig106 standard and make sure we maintain maximum
> performance in these projects and make sure we are as efficient as possible.
> We need to make the library so the user will be able to extend it as well
> because I suspect that the basic TMATS project here will not contain all the
> attribute definitions they need as they either extend the tmats definitions
> or even flat out violate the standards. What do you think about that?

On the specification source:

> Is there anything in this project which talks about version support
> already, like maybe irig106-docs? Also is there a way to support multiple
> versions in the code? Also what is the difference between 106-22 and
> 106-23? What are all the versions available and which should we
> realistically support and can we document the deltas in all the versions?

On scope:

> Drop XML until real XSD, Move WASM out, Remove fake features, Lets get back
> to basics for sure and map out a proper roadmap to what our feature releases
> are and so I can test this against real data ASAP. Can we capture somewhere
> the features for XML and WASM and other fake features so I do not forget
> about them and what they were for? I do not know what #3 Repair suggestions
> only means so I did not select it, but maybe I should so I need better
> understanding.

On shared types: "Use irig106-types now".

## 2. The answer given (verbatim)

These are the right questions, and I don't want to answer the version ones from memory. I'll check what the ecosystem already documents, then look up the editions from the source.

I have enough to answer your questions properly. Here's what I found and what I'd recommend.

### Versions: what exists, and what to support

**Existing docs:** irig106-docs has nothing on versions. The only version table in the ecosystem is in `irig106-time/docs/ROADMAP.md`, and it covers Chapter 10 only. It's also partly from memory: it misses **106-20** and **106-24**. Nothing tracks Chapter 9 (TMATS) changes.

**The editions.** I checked RCC's site and the edition PDFs directly. TMATS was in the standard well before 2004 (106-00 already has a Chapter 9). The ones that matter for Chapter 10 recordings are:

**106-04, 05, 07, 09, 11, 13, 15, 17, 19, 20, 22, 23, 24**, with **106-24 Revision 1** (January 2025) as the current edition on RCC's site.

**Chapter 9 changes in the recent editions:**

| Edition | Chapter 9 changes (from each edition's "Changes in This Edition" page) |
|---|---|
| 106-22 | TG-171 added a **new Q group** (Message Structure attributes). TG-173 added a CRC-presence attribute. |
| 106-23 | TG-189: "various corrections and enhancements" to Chapter 9 (no details given at that level). |
| 106-24 | TG-197: Chapter 9 changes were "limited to the CRC parameter in Table 9-6." |
| 106-24R1 | No Chapter 9 changes (the revision only touched Chapter 2). |

So the difference between 106-22 and 106-23 is TG-189's corrections, and we'd have to diff the attribute tables to get specifics. It also means the crate is behind: 106-23 defines **G, T, R, M, P, D, B, S, Q, C, H, V and X** groups, and the crate only knows nine of them.

**What I'd support:** the TMATS syntax rules (§9.4.2) haven't changed, so **parsing can be version-independent**. Anything back to 106-04 parses the same way. Only the *attribute definitions* vary by edition, so:
- **Baseline: 106-24R1**, the current edition.
- **Validation for 106-04 through 106-24:** every registry entry records which edition introduced, changed or removed it.
- **Pre-2004 files:** parse them, then validate against the oldest known edition with a warning.

**Documenting the deltas: yes, and mostly automatically.** Every edition's Chapter 9 PDF is publicly downloadable. A script can pull the attribute codes out of each edition's tables and diff them, producing a `VERSION-DELTAS.md`. The "Changes in This Edition" task summaries then add the narrative.

### How multiple versions work in code
Each registry entry carries `introduced`/`deprecated`/`removed` plus any per-edition keyword or length differences. The validator picks the edition in this order:
1. A caller override.
2. `G\106` in the file.
3. The CSDW version field from the Chapter 10 setup packet.

One compact table covers all editions; there's no separate copy per edition.

### Extensibility: I agree, and the standard backs you up
Chapter 9 has two extension mechanisms of its own:
- **V group (§9.5.13):** vendor-specific attributes, in the form `V-x\<3-letter vendor>\<anything>`.
- **X group (§9.5.14):** extensions attached to existing attributes. `X-1\MYORG\D\MN\SGAIN-1-2:10.75;` extends `D-1\MN-1-2`. The spec says editors should keep this link intact even when they don't understand the code.

For files that go further and simply break the standard, the design needs four layers:
1. **Never lose anything.** An ordered list of every attribute is the source of truth, so unknown or illegal attributes survive a round trip byte-for-byte. This is why I recommended rebuilding the core: the current typed structs can't keep what they don't know about.
2. **Layered registry.** The built-in spec registry sits underneath, and users add their own definitions or overrides on top, either loaded from a file at runtime or built in code.
3. **Validation policy.** Per-rule severity overrides (downgrade to a warning or turn off), profiles, and a public trait for custom rules. "Our files violate X on purpose" becomes a one-line setting.
4. **First-class V/X support.** X attributes get linked to their parent, so renumbering a measurement also updates its extensions.

### Performance
- **The scale is small.** TMATS files are kilobytes to a few megabytes and are read once per recording, which is tiny next to Chapter 10 packet decoding.
- **We'll still keep it efficient.** A single-pass byte scanner, one owned buffer with offsets into it (zero-copy without the lifetime parameters that caused trouble earlier), a hash index so lookups are constant-time, and typed values parsed only when someone asks for them.
- **Criterion benchmarks against real files will guard that.**

### "Repair → suggestions only", explained
Today `repair()` **edits your document silently by default**. For example, if `R-1\N:8` says there are 8 channels but only 6 are defined, it rewrites the counter to 6. That hides the most useful evidence you have: the file may have been truncated, and 8 was the truth. "Suggestions only" means repair never edits anything. It returns a list like "`R-1\N` says 8, but only 6 channels are defined; possible fix: set 6", and you apply the ones you choose. **I'd recommend it.** It's safer for flight-test metadata, and it fits the "never lose anything" rule.

### Keeping track of the dropped features
They'll go in two places:
- **ROADMAP → "Deferred":** real XML from RCC's TMATS XSD, a separate `irig106-tmats-wasm` crate, `no_std`, and rich diagnostics. For each: what it was for and what bringing it back requires.
- **An ADR for each removal**, recording why.

### Proposed releases, aimed at testing on real data soon

| Release | What's in it |
|---|---|
| **0.1 Read** | Lossless parser (ASCII TMATS plus the Chapter 10 setup-record payload, including the bit-9 ASCII/XML flag), lookup by code name, V/X support, byte-faithful writer, edition detection, and an `examples/tmats-dump` tool to run on your files. |
| **0.2 Registry** | Spec-derived registry for 106-24R1, typed accessors, and channel resolution (R → P/B/S/Q → D → C). |
| **0.3 Validate** | Registry-driven validation with user registries, severity policy and custom rules. |
| **0.4 Editions** | Registry deltas for 106-04 through 106-24, plus `VERSION-DELTAS.md`. |
| **0.5 Generate** | Builder output that passes validation, and repair as suggestions. |

`Irig106Version` and `DataTypeCode` move into `irig106-types` now, as you chose.

### Later answers in the same discussion (verbatim)

On whether repair is still a feature:

**3. Repair.** In the new design there's no separate "repair" module. Repair falls out of three pieces we need anyway:
- **Diagnostics can carry a suggested edit.** For example: "`R-1\N` says 8, but only 6 channels are defined; possible fix: `Set R-1\N = 6`".
- **The document gets a general edit API** (set, insert or remove an attribute). Generating and editing TMATS need it regardless.
- **An explicit `apply(doc, &chosen_edits)`.**

So yes, there's still a way to repair, but only as a deliberate step where you pick the edits. There won't be a `repair()` that decides for you. A CLI flag like `--apply-fixes` could come later on top of the same pieces.

The owner's answers that settled this: "Yes and if we need to sort of start from scratch to make sure we do not have any hidden surprises later that is probably fine so we can go slow, methodical and incremental." — "yes I can get some, but I will need some time to gather them so I would like for you to go ahead and test without for now. I think doing both is good." — "Yes since you recommend. Do we want an option to repair? This does not make sense to have but I want to ask. Also back to the starting over question, would this even be a feature in the new design." — "After we get the plan established I do want to get back to architecture, adrs, requirements, roadmap, data flow, use cases, etc before we write to much code so we understand what this project actually does and will do in the future."

Subsequent refinements (the `tmats` CLI in 0.1 in place of the `examples/`
dump tool, `G\SHA` and the flex signature) are in `docs/ROADMAP.md` and
`docs/research/2026-09-25-irig106org-tools.md`.

## 3. Chapter 9 text relied on (106-23, verbatim)

### §9.4.2 Logical Format (excerpt)

> Each attribute appears in the file as a unique code name and as a data item.
> The code name appears first, delimited by a colon. The data item follows,
> delimited by a semicolon. Thus, an attribute is formatted as A:B; - where A
> is the code name and B is the data item, in accordance with (IAW) the tables
> in Section 9.5. Numeric values for data items may be either integer or
> decimal. Scientific notation (see note below) is allowed only for the
> specific data items defined for its use in the tables in Section 9.5. For
> alphanumeric data items, including keywords, either upper or lower case is
> allowed; TMATS is not case sensitive. All defined keyword values are shown
> as upper case and enclosed in quotes in the tables in Section 9.5. Leading,
> trailing, and embedded blanks are assumed to be intentional; they can be
> ignored in most cases but should not be used in code names, keywords, and
> data items used as links, such as measurement name. Semicolons are not
> allowed in any data item (including comment items). Any number of
> attributes may be supplied within a physical record. Attributes may appear
> in any order.

> Any numeric data item expressed in scientific notation must conform to the
> following regular expression:
> `([-+]?(([0-9]+\.?[0-9]*)|([0-9]*\.[0-9]+)))([eE][-+]?[0-9]{1,3})`

> The code name COMMENT may be used to interject comments to improve
> readability. The comment data items, such as G\COM, are intended to convey
> further details within the TMATS file itself. Comments must follow the
> attribute logical format, as shown below: `COMMENT: This is an example of a
> comment;`

### §9.4.3 XML format — differences from the code-name form

> a. There is a C group for each data link instead of only one C group in the
> TMATS file. b. The schema has no counter ("\N") attributes; they are not
> needed in XML. c. Keyword attribute values are expanded for readability in
> the schema. d. Date and time formats are different; the schema uses the XML
> standard date and time formats (not the ones in Section 9.5). e. Text
> entries in the XML schema may contain semicolons; the code name format uses
> the semicolon as a delimiter. f. The inherent structure of an XML schema
> implies order, while the code name format allows the attributes to be given
> in any order.

### Table 9-1, Telemetry Attribute Groups

G General Information · T Transmission Attributes · R Recorder-Reproducer
Attributes · M Multiplex/Modulation Attributes · P PCM Format Attributes ·
D PCM Measurement Description · B Bus Data Attributes · S Message Data
Attributes · Q Message Structure Definition Attributes · C Data Conversion
Attributes · H Airborne Hardware Attributes · V Vendor-Specific Attributes ·
X TMATS eXtension Attributes.

> Within the structure, a lower-case letter, for example, n, p, or r,
> indicates a multiple-entry item with the index being the lower-case letter.
> The range of these counters is from one to the number indicated in another
> data entry, usually with the appendage \N, and have no missing values.

### §9.5.1 a — the seven Usage Attributes fields

> The Usage Attributes column within each table describes how a particular
> attribute is to be used, when it is allowed, etc. If there are enumerations
> for the attribute, the enumeration values and their descriptions will
> appear in this column. There are 7 possible fields within this column for
> each attribute.
>
> - R/R Ch 10 Status: This describes special rules for creating TMATS files to
>   support setup of a Chapter 10 recorder. A value of "R" requires that the
>   attribute be specified in the TMATS file whenever the attribute is
>   allowed. A value of "RO" indicates that when an applicable data type or
>   group is used, the attribute must be specified in the TMATS file. A value
>   of "RO-PAK" indicates the attribute must be specified when the Data
>   Packing Option (R-x\PDP-n) is either UNPACKED (UN) or PACKED (PFS). If the
>   attribute is specified in the TMATS file, it must contain valid
>   information.
> - Allowed when: This describes when an attribute is allowed to be specified
>   inside of a TMATS file.
> - Required when: This describes when an attribute must be specified inside
>   of a TMATS file. If the Required condition is "When Allowed", then it must
>   be specified when the "Allowed when" condition is met.
> - Links to: Specifies a list of attributes that the attribute links to by
>   value.
> - Links from: Specifies a list of attributes that link to this attribute by
>   value. Any attribute with a Links from: is a key and must be unique in the
>   TMATS file.
> - Range: This describes the values or ranges that may be specified. A range
>   might be specified with exact values or may reference the value of
>   another TMATS attribute. The range may also be simply a number of
>   characters that represents the recommended maximum length of the value.
>   Where possible, the valid ranges for numbers are specified, however each
>   range should be consulted as to their specific capabilities. There are
>   several special values for Range: Enumeration (the value must be one of
>   the values listed in the description column of the attribute); Floating
>   Point (a legal floating point, integer, or scientific notation value);
>   xxx.xxx.xxx.xxx (an IP address where each "xxx" is a value from 0-255);
>   Hexadecimal (0-9 and A-F or a-f); Binary (0-1); Binary pattern (0, 1, or
>   "X" for don't care); "X" (the character "X"); MM-DD-YYYY-HH-MI-SS (a date
>   and time).
> - Default: This identifies the default value required to process a TMATS
>   file when the file itself does not contain the attribute.
>
> In previous versions of this document, there existed code name tags
> \*R-CH10\*, \*RO-CH10\* and \*RO-CH10-PAK\*. These have been removed in favor
> of the above attribute column. If the R/R Ch10 Status field is "R", then the
> attribute must be included in the TMATS file if all other conditions apply
> even if it has a default.

### §9.5.1 b — Group relationships

> a. Data Source ID is unique within a General Information group (G). It ties
> the Transmission group (T) or the Recorder-Reproducer group (R) or both to
> the G group and to the Multiplex/Modulation group (M).
> b. The tie from the M group to a PCM group (P) is the Data Link Name.
> c. The tie from the P group to an embedded P group is another Data Link
> Name.
> d. The tie from the M group to the Data Conversion group (C) for an analog
> measurement is the Measurement Name.
> e. The tie from the P group to the PCM Measurement Description group (D) or
> Bus group (B) is the Data Link Name.
> f. The tie from the R group to the P group is from the Channel Data Link
> Name (R) to the Data Link Name (P).
> g. The tie from the R group to the B group is from the Channel Data Link
> Name or Sub-Channel Name (R) to the Data Link Name (B).
> h. The tie from the R group to the Message Data group (S) or Message
> Structure Definition Group (Q) is from the Channel Data Link Name,
> Sub-Channel Name, or Network Name (R) to the Data Link Name (S) or Data
> Link Name (Q).
> i. The tie from either the R, D, B, or S group to the Data Conversion group
> is the Measurement Name.

### §9.5.13 Vendor-Specific Attributes (V) and §9.5.14 TMATS eXtension Attributes (X)

> The only V-group attributes defined in this standard are the following.
> a. Data Source ID (code name V-x\ID) - specifies the Data Source ID
> consistent with the General Information group and ties the V group to the G
> group. b. Vendor Name (code name V-x\VN) - a three-character acronym that
> identifies the specific vendor and determines how the rest of the
> attributes in the V group are interpreted. All other code names for
> vendor-specific attributes will have the form: `V-x\acr\attribute-string`
> where: acr is the three-character acronym identifying a specific vendor.
> attribute-string is any attribute that applies to this vendor.

> The TMATS may be extended using X attributes. The format is described
> below: `X-x\ ORGANIZATION \ORIGCODE\EXTENSION_CODE-i-j-m-n:Value;`
> Everything to the right of ORGANIZATION that matches an existing TMATS code
> is used to associate the extension with an existing object defined by the
> TMATS file. The ORIGCODE contains the original group identifier (i.e.,
> G,D,P, etc.) followed by a "\" and the original code that is to be extended
> (may include more "\" characters, but no "-"). The EXTENSION_CODE
> identifies the specific extension and shall be unique (i.e., not
> overlapping any existing TMATS code name). The value of "-x" must match the
> first level index (P-x, etc.) value and the "-i-j" (the number of indexes
> defined by the original code) must match the same number of indexes in
> this extension code. The remaining "-m-n" values are unique to the
> extension. For example, to extend a D section measurement:
> `D-1\MN-1-2:MEAS1;` To add a new extension code name for Sensor Gain, the
> following would define the extension: `X-1\MYORG\D\MN\SGAIN-1-2:10.75;` In
> this example, the -1 in the "X-1" and "-1-2" corresponds to the "-1" and
> "-1-2" in the original "D-1\MN-1-2" code word. If the extension has more
> indexes than the original code, then the indexes of the original code link
> to the same number of left most indexes of the extension code. The value of
> ORGANIZATION should be a unique name that identifies the organization that
> defined the extension. The advantage of this extension is that software
> that is processing the TMATS will know that these codes refer to a
> particular item in the file (like a measurement or recorder). For software
> that recognizes the codes, it can process them. Otherwise they can be
> ignored. If the file is being edited by a TMATS editor, it would notice the
> association and preserve it even if the editor doesn't know what the code
> means. Thus if the measurements were re-numbered and the index was 1-5
> instead of 1-2, the extension code could be updated to preserve the link.
> The values of "x" in "X-x" are not necessarily contiguous. The "x" values
> must match the index of the original code word therefore no new values may
> be added.

## 4. Editions archived

`TelemetryWorks/rcc-106-standards` (public; PDFs as GitHub Release assets;
`manifest.toml` with every original URL and SHA-256) holds 106-59 through
106-24R1 — 106-59, 60, 62, 64, 65, 69, 71, 73, 80, 86, 93, 96, 99, 00, 01, 04,
05, 07, 09, 11, 13, 15, 17, 19, 20, 22, 23, 24, 24R1 — plus RCC 123-09/16/20,
RCC 124-13/15/19/22, and the TMATS XML schemas from 106-07 onward. Three
documents TRMC lists under "106" are not telemetry standards (106-59 C-band
radar, 106-64 RF sources, and a 1961 timing plan published as 106-62). The
RCC 124-17 handbook could not be retrieved. Nothing newer than 106-24R1 was
published at the time of retrieval.

Text-extraction note for registry generation: `pdftotext -raw` yields
Chapter 9 table rows one attribute per line (for example
`PCM CODE P-d\D1 R/R Ch 10 Status: RO Define the data format code.`), whereas
`-layout` interleaves the table columns.
