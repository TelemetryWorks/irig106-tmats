# irig106-tmats — Interpretation register

> **Status: hand-written during the design phase.** Once the registry's
> interpretation files exist (ADR-0022), this document is generated from
> them and CI checks that it is current (ADR-0027).

The IRIG 106 standard is the authority, but it is not always consistent,
complete, or explicit. Each entry here records one place where the design
had to choose: the citations that conflict or fall short, quoted exactly;
the behaviour chosen; why; who wrote and who reviewed it; and the test that
pins it. Nothing in the code may rely on an interpretation that is not
listed here.

## Conventions

- **Identifiers** are `INT-NNN`, permanent, never reused.
- **Sources** are quoted verbatim from the archived standard
  (`TelemetryWorks/rcc-106-standards`), with edition, table or section, and
  page where it helps. Baseline: 106-24R1 unless stated.
- **Design** is the owner's decision on the behaviour: `accepted`,
  `suspect` (accepted but held in doubt until checked against real data —
  `docs/ROADMAP.md`, "Suspect findings"), `proposed` (a behaviour drafted
  while documenting, not yet reviewed by the owner), or `open` (no behaviour chosen;
  an owner follow-up in `docs/ROADMAP.md`, "Follow-ups for the owner" — the
  implementation must not choose one).
- **Review** follows ADR-0033 (owner decision, 2026-11-21), superseding
  ADR-0022's two-human rule. Agents draft each interpretation with citations;
  one human sign-off by the owner plus an independent AI reviewer as a
  second, non-human check approves it. Record draft author, owner sign-off
  and independent AI review with their dates and evidence; review stays
  `pending` until both checks are recorded. Re-review changed source text.
- **Coordinator-decided** design entries record coordinator direction and
  its date; the owner may override them. Such direction does not substitute
  for the interpretation's owner sign-off, independent AI review or evidence.
- **Test** names the focused test that pins the behaviour. The test carries
  a `/// Interpretations: INT-NNN` doc comment directly above `#[test]`;
  `scripts/build-trace-matrix.py` lists every entry and the tests that name
  it, and reports entries without one.
- **Origin** is the team-review item or finding that raised it
  (`docs/ROADMAP.md`).
- **Development** — every entry is marked **intensive testing and deep
  analysis required** (owner direction, 2026-09-26). An interpretation is
  where the design departs from a literal reading of the standard, so it is
  where a mistake is most likely and least visible. Before its code is
  written and before its status can change, the entry needs:
  1. **A written analysis** — a dated record in `docs/research/`, linked
     from the entry's **Analysis** field, that re-reads every cited source in
     every archived edition the entry touches (not only the baseline), looks
     for further sources that bear on it (other tables, appendices, examples,
     the RCC 123 and 124 handbooks), lists the edge cases, and states whether
     the chosen behaviour still holds.
  2. **Intensive tests** — beyond the focused test: one test per edge case
     the analysis lists; a property or fuzz test wherever inputs can be
     generated (values, case, ordering, indices, editions); a fixture from
     each archived edition the entry touches; and, where local sample
     recordings exercise it, a check against real data (`docs/TEST-DATA.md`;
     local only, never in CI). All carry the entry's marker.
  3. **The owner and independent AI reviewer** (Review) check the analysis
     as well as the rule. Updating decision metadata does not mark that work
     complete or waive the analysis and tests.
  The trace matrix shows each entry's analysis and tests; an entry with
  either missing is not ready.
- **Analysis** links the written analysis, or says `not yet written`.

---

## Links and relationships

### INT-001

**Title**: The R group's data-link names reach the Q group, although R's "Links to:" omits Q

**Sources**:
- Table 9-4, `R-x\CDLN-n`: "Links to: P-d\DLN, B-x\DLN, S-d\DLN"; `R-x\EV\DLN-n`: "Links to: P-d\DLN, B-x\DLN, S-d\DLN".
- Table 9-10, `Q-d\DLN`: "Links from: R-x\CDLN, R-x\EV\DLN-n".
- §9.5.1 b (h): "The tie from the R group to the Message Data group (S) or Message Structure Definition Group (Q) is from the Channel Data Link Name, Sub-Channel Name, or Network Name (R) to the Data Link Name (S) or Data Link Name (Q)."

**Behaviour**: `R-x\CDLN-n` and `R-x\EV\DLN-n` link to `Q-d\DLN` as well as to `P-d\DLN`, `B-x\DLN`, and `S-d\DLN`.

**Reason**: Two of the three sources state the tie; generating links from the R side alone would leave valid Q links unresolved.

**Design**: accepted (owner, 2026-09-26) · **Review**: pending · **Origin**: T5

**Test**: `r_channel_data_link_name_resolves_to_q_group`

**Development**: intensive testing and deep analysis required · **Analysis**: not yet written

### INT-002

**Title**: Sub-channel and network names tie to B, S, and Q, although no "Links" field says so

**Sources**:
- §9.5.1 b (g): "The tie from the R group to the B group is from the Channel Data Link Name or Sub-Channel Name (R) to the Data Link Name (B)."
- §9.5.1 b (h), quoted in INT-001.
- Table 9-4: `R-x\ANM-n-m` "ARINC 429 bus sub-channel name.", `R-x\UCNM-n-m` "Specify the UART sub-channel name.", `R-x\MCNM-n-m` "Specify the message sub-channel name.", `R-x\ENAM-n-m` "Specify the Ethernet network name.", `R-x\CBM-n-m` "Specify the CAN bus sub-channel name." — none has a "Links to:" field.
- Tables 9-8, 9-9, 9-10: `B-x\DLN`, `S-d\DLN`, and `Q-d\DLN` do not list any of them under "Links from:".
- Table 9-8, `B-x\BT-i` enumeration: "1553 1553 bus", "A429 ARINC 429 bus".
- Table 9-9, `S-d\DLN`: "Allowed when: R\CDT is either "UARTIN" or "MSGIN" or "ETHIN" or "FBCHIN"".
- Table 9-10, `Q-d\DLN`: "Allowed when: R\CDT is either MSGIN or 1394IN or ETHIN or FBCIN".

**Behaviour**: `R-x\ANM-n-m` links to `B-x\DLN`; `R-x\UCNM-n-m` to `S-d\DLN`; `R-x\MCNM-n-m` and `R-x\ENAM-n-m` to `S-d\DLN` or `Q-d\DLN` (both allowed for their channel types; a name matching both is ambiguous, ADR-0026). `R-x\CBM-n-m` links to nothing: no group accepts CAN data (`CANIN` appears in no B, S, or Q condition), so an unmatched CAN sub-channel name is not reported as an unresolved link.

**Reason**: The prose is the only source of these ties; the target of each is chosen from the groups whose own conditions or enumerations accept that channel's data.

**Design**: accepted (owner, 2026-09-26) · **Review**: pending · **Origin**: T5

**Test**: `sub_channel_and_network_names_resolve_to_their_groups`

**Development**: intensive testing and deep analysis required · **Analysis**: not yet written

### INT-003

**Title**: "All valid paths are documented" in the Links fields — they are not

**Sources**:
- §9.5.1 b: "Not all valid paths are shown. All valid paths are documented in "Links to:" and "Links from:" attributes."
- INT-001 and INT-002, which contradict it.

**Behaviour**: Relationships are generated from three sources — "Links to:", "Links from:", and the ties of §9.5.1 b — and each relationship found in fewer than all of the sources that could state it has its own register entry. The registry generator fails when such a relationship has none.

**Reason**: No single source is complete; the check keeps every gap visible and decided.

**Design**: accepted (owner, 2026-09-26) · **Review**: pending · **Origin**: T5

**Test**: `one_sided_relationship_without_register_entry_fails_generation`

**Development**: intensive testing and deep analysis required · **Analysis**: not yet written

### INT-004

**Title**: Index letters in code-name references are placeholders

**Sources**:
- Table 9-6, `P-d\DLN`: "Links to: D-x\DLN, B-d\DLN".
- Table 9-8 names the attribute `B-x\DLN`.

**Behaviour**: A reference matches by group and code path; the index letters (`x`, `d`, `n`, …) name positions, not particular letters, so "B-d\DLN" is `B-x\DLN`.

**Reason**: The group occurrence letter differs between tables for the same attribute; matching by letter would lose the P → B link.

**Design**: accepted (owner, 2026-09-26) · **Review**: pending · **Origin**: T4

**Test**: `pcm_data_link_name_links_to_bus_data_link_name`

**Development**: intensive testing and deep analysis required · **Analysis**: not yet written

### INT-005

**Title**: The channel data type selects among overlapping link targets

**Sources**:
- Table 9-8, `B-x\DLN`: "Links from: R-x\CDLN, P-d\DLN, R-x\EV\DLN-n".
- Table 9-6, `P-d\DLN`: "Links to: D-x\DLN, B-d\DLN".
- Table 9-4, `R-x\CDT-n` enumeration: "PCMIN PCM Input", "1553IN 1553 Input", "429IN ARINC 429 Input", "UARTIN UART Input", "MSGIN Message Data Input", "1394IN IEEE-1394 Input", "ETHIN Ethernet Input", "FBCHIN Fibre Channel Input", among others.
- The S and Q conditions quoted in INT-002.

**Behaviour**: For `R-x\CDLN-n`, the channel data type `R-x\CDT-n` of the same channel selects the target group: `PCMIN` → P; `1553IN`, `429IN` → B; `UARTIN` → S; `MSGIN`, `ETHIN`, `FBCHIN` → S or Q; `1394IN` → Q; any other type has no data-link target. Candidates in other groups are excluded; several remaining candidates make the link ambiguous (ADR-0026).

**Reason**: Bus data carried in a PCM stream shares the P group's data-link name, so a PCM channel would otherwise match both P and B.

**Design**: accepted (owner, 2026-09-26) · **Review**: pending · **Origin**: T4

**Test**: `pcm_channel_carrying_bus_data_resolves_to_p_group`

**Development**: intensive testing and deep analysis required · **Analysis**: not yet written

### INT-006

**Title**: `H\TA` ties the H group to the G group by test item

**Sources**:
- §9.5.12: "The only H group attributes defined in this standard are the following: a. Test Item (code name H\TA) - specifies the item under test and ties the H group to the G group. b. Airborne System Type (code name H\ST-n) - identifies the airborne systems being described in the current file and determines how the rest of the attributes in the H group will be interpreted."
- Table 9-2: "TEST ITEM G\TA Allowed when: Always".

**Behaviour**: `H\TA` links to `G\TA` by value. `H\TA` and `H\ST-n` are built in; every other H attribute is organisation-defined, supplied through the user overlay (ADR-0008) and preserved when no definition is given.

**Reason**: The standard gives the tie in prose without naming the attribute that carries it on the G side; `G\TA` is G's test-item attribute. There is no H table to transcribe; §9.7 says IHAL serves "the purpose originally intended for the Airborne Hardware Attributes (H) group …, which has never been implemented".

**Design**: accepted (owner, 2026-09-26) · **Review**: pending · **Origin**: T5

**Test**: `h_group_test_item_ties_to_g_group`

**Development**: intensive testing and deep analysis required · **Analysis**: not yet written

## Conditions and keywords

### INT-007

**Title**: The Q group's condition names `FBCIN`, which is not a channel type

**Sources**:
- Table 9-10, `Q-d\DLN`: "Allowed when: R\CDT is either MSGIN or 1394IN or ETHIN or FBCIN".
- Table 9-4, `R-x\CDT-n` enumeration: "FBCHIN Fibre Channel Input"; no `FBCIN`.
- Table 9-9, `S-d\DLN`: "… or "FBCHIN"". The same `FBCIN` appears in 106-22, 106-23, 106-24, and 106-24R1.

**Behaviour**: The condition is read as `FBCHIN`. A document whose `R-x\CDT-n` is `FBCIN` gets the usual unknown-keyword finding.

**Reason**: Read literally, the Q group could never be used for Fibre Channel data, which the S group and the enumeration both support.

**Design**: accepted (owner, 2026-09-26) · **Review**: pending · **Origin**: T5

**Test**: `q_group_allowed_for_fibre_channel`

**Development**: intensive testing and deep analysis required · **Analysis**: not yet written

### INT-008

**Title**: `C-d\DPNO`'s condition names `C\DCT` without an index

**Sources**:
- Table 9-11, `C-d\DPNO`: "Allowed when: C\DCT is "DER"" and "Default is 1."

**Behaviour**: `C\DCT` means `C-d\DCT` of the same C occurrence; the default of 1 is taken from the prose.

**Reason**: Conditions are written for people; the only occurrence that makes sense is the attribute's own (ADR-0022, section 4.2 of the architecture).

**Design**: accepted (owner, 2026-09-26) · **Review**: pending · **Origin**: T1

**Test**: `derived_occurrences_default_to_one_in_same_occurrence`

**Development**: intensive testing and deep analysis required · **Analysis**: not yet written

### INT-009

**Title**: Counters named without indices in conditions

**Sources**:
- Table 9-7, `D-x\WFT-y-n-m-e` (fragment transfer order) and `D-x\WFP-y-n-m-e` (fragment position): "Allowed when: D\MNF\N > 1"; `D-x\WFP-y-n-m-e` also has "Range: 1 - D\MNF\N".

**Behaviour**: An unindexed counter in a condition means the counter of the same parent-index combination as the attribute being checked — for `D-x\WFP-y-n-m-e`, `D-x\MNF\N-y-n-m`, the fragment count of the same location.

**Reason**: Counters count within their parent (ADR-0026); any other occurrence would make the condition meaningless.

**Design**: accepted (owner, 2026-09-26) · **Review**: pending · **Origin**: T1, T4

**Test**: `fragment_position_allowed_only_when_its_own_location_has_fragments`

**Development**: intensive testing and deep analysis required · **Analysis**: not yet written

### INT-010

**Title**: Equality is `==`, although Table E-3 prints `= =`

**Sources**:
- Appendix 9-E, Table E-3: "= = Equal To A = = B"; Table E-6: "= = != Left to right".
- Appendix 9-E grammar: "| expression '==' expression".

**Behaviour**: `==` is the equality operator; `= =` is accepted with a warning.

**Reason**: The grammar is the machine-readable source; the table's spacing is typographic (ADR-0024).

**Design**: accepted (owner, 2026-09-26) · **Review**: pending · **Origin**: T2

**Test**: `spaced_equality_operator_is_accepted_with_warning`

**Development**: intensive testing and deep analysis required · **Analysis**: not yet written

## Chapter 10 setup records

### INT-011

**Title**: Where one multi-packet setup record ends

**Sources**:
- Chapter 11 §11.2.7.2: "A single setup record may span multiple consecutive packets. When spanning multiple packets, the sequence counter shall increment in the order of segmentation of the setup record, n+1". No end-of-record marker is defined.

**Behaviour**: Fragments are joined while they are consecutive data type `0x01` packets on one channel whose sequence numbers increase by one modulo 256 and whose CSDWs agree on FRMT and RCCVER. Whether the joined run is a complete record is decided by INT-034 (ADR-0031): an intervening packet, a sequence gap, a CSDW change, or the end of input stops the joining, but none of them alone proves the record complete. *(Revised 2026-09-27: the earlier text also made these conditions the record's end, which contradicted INT-031; team spec alignment review.)*

**Reason**: The standard says how fragments follow each other but not how a record ends (ADR-0025, ADR-0031).

**Design**: accepted (owner, 2026-09-26) · **Review**: pending · **Origin**: T3

**Test**: `multi_packet_setup_record_is_assembled` (`docs/TEST-DATA.md`)

**Development**: intensive testing and deep analysis required · **Analysis**: not yet written

### INT-012

**Title**: RCCVER codes are read as their table states; later editions' reuse is a note

**Sources**:
- Chapter 11, setup-record CSDW (106-24R1, Figure 11-34): "0x0E = RCC 106-22", "0x0F through 0xFF = Reserved"; the same list in 106-22, 106-23, and 106-24.
- 106-19 and 106-20 Chapter 11: "0x0D = RCC 106-19", "0x0E through 0xFF = Reserved".

**Behaviour**: `0x0E` is reported as "RCC 106-22" with the labelled note "unchanged through 106-24R1: 106-23, 106-24, and 106-24R1 assign no newer code"; `0x0D` as "RCC 106-19" with the note "106-20 assigns no newer code"; reserved values as an unknown edition. `G\106` stays the primary edition source (ADR-0016). *(Revised 2026-09-27, ADR-0032: previously "106-22 or later".)*

**Reason**: The table names one edition per code. That later editions kept the code is an observation about the archived editions, not a promise about future ones (team spec alignment review). RCCVER declares the recording-format version, not the TMATS edition; see INT-016 and INT-017 (ADR-0028).

**Design**: accepted (owner, 2026-09-26; wording revised by the owner's decision of 2026-09-27) · **Review**: pending · **Origin**: T3; team spec alignment review

**Test**: `rccver_codes_read_as_their_table_states`

**Development**: intensive testing and deep analysis required · **Analysis**: not yet written

### INT-034

**Title**: A joined run is complete, incomplete, or ambiguous

**Sources**:
- Chapter 11 §11.2.7.2: "A single setup record may span multiple consecutive packets. When spanning multiple packets, the sequence counter shall increment in the order of segmentation of the setup record, n+1". No end-of-record marker is defined.
- §11.2.1.1 f: "Each channel in a session shall have its own sequence counter providing a unique sequence number for that channel" — two records back to back on one channel continue one count.
- Chapter 9 §9.4.2: each attribute is a code name and data item ended by a semicolon; "Semicolons are not allowed in any data item (including comment items)."

**Behaviour**: For each run joined under INT-011:
- **complete** — no gap or CSDW change inside it, the body's last non-blank byte is `;`, and the run is followed by another packet (basis "followed by another packet") or by the end of input (basis "end of input", labelled);
- **incomplete** — the body is cut inside an attribute (its last non-blank byte is not `;`), a fragment is unreadable or untrusted, or a sequence gap splits text that continues an attribute across it;
- **ambiguous** — a sequence gap or a CSDW change where the text before it ends at `;`, or a gap-free run whose text repeats a single-entry attribute (for example `G\106` or `G\PN`); both readings and the split point are reported.
Complete records govern (INT-028); incomplete and ambiguous records do not by default (INT-031). An XML body (FRMT 1) is of unknown completeness.

**Reason**: The packets alone cannot tell where a record ends; the text's final byte and repeated single-entry attributes are the only evidence the standard gives, and neither is proof in every case.

**Design**: accepted in principle (three outcomes, owner, 2026-09-27, ADR-0031); these rules proposed · **Review**: pending · **Origin**: team spec alignment review, 2026-09-27

**Test**: `setup_record_completeness_outcomes`

**Development**: intensive testing and deep analysis required · **Analysis**: not yet written

## Errata in the standard's own examples

### INT-013

**Title**: Appendix 9-C ends 18 attributes with `:` instead of `;`

**Sources**:
- Appendix 9-C, page C-8 (106-24R1), for example "D-1\MML\N-1-1:2: D-1\MNF\N-1-1-1:1: D-1\WP-1-1-1-1:14;"; the same 18 in every edition since 106-17.
- §9.4.2: attributes are "CODE:value;".

**Behaviour**: The attribute is kept exactly as read (the first colon ends the code name) and a warning, "possible `;` typed as `:`", is reported with a suggested edit (L1-READ-007, ADR-0026).

**Reason**: Nothing is split silently (ADR-0002, ADR-0007).

**Design**: **suspect** (owner, 2026-09-26; `docs/ROADMAP.md`, S1) · **Review**: pending · **Origin**: T4

**Test**: `appendix_9c_example_reports_suspected_semicolons` (`docs/TEST-DATA.md`)

**Development**: intensive testing and deep analysis required · **Analysis**: not yet written

## Editions and versions

### INT-014

**Title**: `G\106` gives two year digits, so one value can name two editions

**Sources**:
- Table 9-2 (106-24R1), `G\106`: "Version of RCC IRIG 106 standard used to generate this TMATS file. The last 2 digits of the year should be used. Use a leading 0 if necessary." "Range: 0 to 99".
- 106-24 and 106-24R1 are both published editions; 106-24R1 did not change Chapter 9 (ADR-0016).

**Behaviour**: The value is read as a year and mapped to every archived edition of that year: `24` → 106-24 and 106-24R1; `96` → 106-96. A value without its leading zero (`4`) is read the same way, with a note. When all candidates share the same Chapter 9 rules, the newest is named as the rules applied (`24` → 106-24R1); when their rules differ, the choice needs its own entry here.

**Reason**: The field cannot say more than the year, and "should" makes even the two-digit form a recommendation.

**Design**: accepted (owner, 2026-09-26) · **Review**: pending · **Origin**: T6

**Test**: `g106_24_names_106_24_and_106_24r1`

**Development**: intensive testing and deep analysis required · **Analysis**: not yet written

### INT-015

**Title**: `G\106` had no defined format before 106-17

**Sources**:
- 106-05 and 106-07 Chapter 9, `G\106`: "VERSION OF IRIG 106 STANDARD USED TO GENERATE THIS TMATS FILE" (no format).
- 106-13 Chapter 9, `G\106`: "Version of RCC IRIG 106 standard used to generate this TMATS file." with a maximum field size of 2.
- 106-17 Chapter 9: the first edition with "The last 2 digits of the year should be used."

**Behaviour**: Only a value that reads as the year of an archived edition (INT-014) is recognised. Any other form — `106-05`, `2005`, `IRIG 106-05` — is reported as "unrecognised", preserved exactly, and not interpreted; the fallback of ADR-0028 then applies, and the caller may override.

**Reason**: Older files may use forms the standard never defined; reading them would be a guess.

**Design**: accepted (owner, 2026-09-26) · **Review**: pending · **Origin**: T6

**Test**: `g106_other_forms_are_unrecognised_and_preserved`

**Development**: intensive testing and deep analysis required · **Analysis**: not yet written

### INT-016

**Title**: What the setup record's version byte declares, and when it did not exist

**Sources**:
- 106-05 Chapter 10, computer-generated Format 1 CSDW: "Reserved. (Bits 31-0) are reserved."
- 106-07 Chapter 10: "IRIG-106 Chapter 10 Version (CH10VER). (1 Byte) indicates which IRIG-106 Chapter 10 release version the recorder requirements and following recorded data are applicable to and comply with." "0x00 thru 06 = Reserved", "0x07 = IRIG-106-07", "0x08 thru 0xFF = Reserved".
- 106-24R1 Chapter 11: "RCC 106 Version (RCCVER). Bits 7-0 specify which RCC release version applies and to which the following recorded data complies with." "0x0E = RCC 106-22", "0x0F through 0xFF = Reserved".

**Behaviour**: The byte is reported as the **recording-format version declared**, never as the TMATS edition. A value from `0x00` to `0x06` is reported as "not declared (the field did not exist before 106-07, or is reserved)", not as an error; defined codes are read from the newest table (106-24R1), since later editions only added codes.

**Reason**: The field describes the recorder and recorded data, and 106-04 and 106-05 recordings carry no version at all.

**Design**: accepted (owner, 2026-09-26) · **Review**: pending · **Origin**: T6

**Test**: `zero_csdw_version_is_not_declared`

**Development**: intensive testing and deep analysis required · **Analysis**: not yet written

### INT-017

**Title**: The fallback edition taken from RCCVER

**Sources**:
- 106-24R1 Chapter 11 RCCVER codes: "0x07 = RCC 106-07" … "0x0D = RCC 106-19", "0x0E = RCC 106-22".
- Owner decision (2026-09-26): when `G\106` is missing or unrecognised and there is no override, use a labelled fallback (ADR-0028).

**Behaviour**: Codes `0x07` to `0x0C` apply the rules of the edition each names. For `0x0D` ("RCC 106-19") the rules of 106-20 apply, and for `0x0E` ("RCC 106-22") those of 106-24R1, each as a labelled **policy**: "the newest archived edition that still uses this code" (INT-012; owner, 2026-09-27, "Yes, use 106-20"). With no usable RCCVER, the baseline applies. The report says "fallback" and gives the reason. *(Revised 2026-09-27, ADR-0032: the policy is now named as one; the effect for `0x0E` is unchanged.)*

**Reason**: Applying the newest edition that still uses a code avoids reporting attributes introduced by that edition as unknown in recordings that could only declare the older code; the label keeps the choice visible as policy, not as something RCCVER says.

**Design**: accepted (owner, 2026-09-26) · **Review**: pending · **Origin**: T6

**Test**: `missing_g106_falls_back_to_rccver_edition_labelled`

**Development**: intensive testing and deep analysis required · **Analysis**: not yet written

## Edits and the checksum

### INT-018

**Title**: `G\SHA` is recognised in any letter case, and never inside a value

**Sources**:
- §9.4.2: "For alphanumeric data items, including keywords, either upper or lower case is allowed; TMATS is not case sensitive."
- Chapter 6 §6.2.3.11 f: "If the TMATS includes a G\SHA code name, all text between the "G\SHA" and the following semicolon, inclusive, shall be discarded for the purposes of digest calculation."

**Behaviour**: The excluded range starts at a `G\SHA` **code name** — the start of an attribute, in any letter case (`g\sha`, `G\sha`) — and never at the same characters inside another attribute's value, such as a comment.

**Reason**: The standard speaks of the code name; `irig106lib`'s substring search treats text in a value as the item (defect D6).

**Design**: accepted (owner, 2026-09-26) · **Review**: pending · **Origin**: T7

**Test**: `g_sha_code_name_in_any_case_and_not_in_values`

**Development**: intensive testing and deep analysis required · **Analysis**: not yet written

### INT-019

**Title**: A `G\SHA` item with no following semicolon

**Sources**:
- Table 9-2: "except the characters from "G\SHA:" to the following ";" (inclusive)".
- Chapter 6 §6.2.3.11 f: "all text between the "G\SHA" and the following semicolon, inclusive".

**Behaviour**: An error. Verification reports "malformed" and never "match"; a suggested edit adds the terminator. No digest is presented as the document's checksum.

**Reason**: Without a following semicolon the excluded range is undefined, so any digest would rest on a guess.

**Design**: accepted (owner, 2026-09-26) · **Review**: pending · **Origin**: T7

**Test**: `unterminated_g_sha_is_malformed`

**Development**: intensive testing and deep analysis required · **Analysis**: not yet written

### INT-020

**Title**: Upper-case hexadecimal in a `G\SHA` value

**Sources**:
- Chapter 6 §6.2.3.11 f: "The message digest is a string of 64 lower-case hexadecimal characters, prefixed with the constant string "2-"".
- Table 9-2: "SHA2-256 shall be represented as "2-" followed by 64 hex characters." "Range: integer followed by "-" followed by hex characters".

**Behaviour**: A value whose digits match the computed digest except for letter case is reported as a match with a warning that Chapter 6 requires lower case. A stamp always writes lower case.

**Reason**: Table 9-2 allows hex characters without a case; Chapter 6 fixes lower case; §9.4.2 makes letter case insignificant.

**Design**: accepted (owner, 2026-09-26) · **Review**: pending · **Origin**: T7

**Test**: `upper_case_g_sha_matches_with_warning`

**Development**: intensive testing and deep analysis required · **Analysis**: not yet written

### INT-021

**Title**: Two or more `G\SHA` items in one document

**Sources**:
- Table 9-2 and Chapter 6 §6.2.3.11 f (quoted in INT-018 and INT-019) speak of one item: "If the TMATS includes a G\SHA code name".
- Table 9-2: the code name `G\SHA` carries no index.

**Behaviour**: **Coordinator-decided F1, 2026-11-21; owner may override.** Reject duplicate SHA items as indeterminate; list every occurrence, preserve every byte, never claim positive verification, and refuse stamping until the input is fixed (ADR-0034).

**Reason**: The standard does not say which item, if any, holds the checksum, or which text is excluded, when there are several.

**Design**: coordinator-decided (F1, 2026-11-21; owner may override) · **Review**: pending · **Origin**: T7

**Test**: `duplicate_g_sha_items` (planned; not yet written)

**Development**: intensive testing and deep analysis required · **Analysis**: not yet written

### INT-022

**Title**: Removing an attribute that X extensions point to

**Sources**:
- §9.5.14: "If the file is being edited by a TMATS editor, it would notice the association and preserve it even if the editor doesn't know what the code means. Thus if the measurements were re-numbered and the index was 1-5 instead of 1-2, the extension code could be updated to preserve the link."
- §9.5.14: "The "x" values must match the index of the original code word therefore no new values may be added."

**Behaviour**: **Open — owner follow-up F2.** No behaviour is chosen until the owner decides.

**Reason**: The standard covers renumbering the original, not removing it.

**Design**: open (follow-up F2, 2026-09-26) · **Review**: pending · **Origin**: T7

**Test**: `removing_an_extended_attribute` (written once F2 is decided)

**Development**: intensive testing and deep analysis required · **Analysis**: not yet written

### INT-023

**Title**: `=` typed where `:` belongs, as in Chapter 6's own example

**Sources**:
- Chapter 6 §6.2.3.11 (106-24R1), `.TMATS WRITE` and `.TMATS READ` examples: "G\DSI\N=18;".
- §9.4.2: attributes are written "CODE:value;".

**Behaviour**: **Open — owner follow-up F3.** Reading follows §9.4.2 regardless: with no colon the item is reported as a missing delimiter (L1-READ-003) and kept exactly. Whether to also offer a suggested edit replacing `=` by `:` is not decided.

**Reason**: The standard's own example uses the form, so real files may too; a suggestion could help or could mislead.

**Design**: open (follow-up F3, 2026-09-26) · **Review**: pending · **Origin**: T7

**Test**: `chapter_6_example_equals_sign_is_missing_delimiter`

**Development**: intensive testing and deep analysis required · **Analysis**: not yet written

## Channels and packets

### INT-024

**Title**: Which packet data types a channel may carry

**Sources**:
- Chapter 11 Table 11-4 (106-24R1): data-type families of eight codes (for example "0x08 PCM Data, Format 0" to "0x0B - 0x0F PCM Data, Format 3 - Format 7"), except "0x78 Controller Area Network Bus" (one code) and "0x79 Fibre Channel Data, Format 0", "0x7A Fibre Channel Data, Format 1", "0x7B – 0x80 … Reserved"; no `0x22` is listed.
- Table 9-4: `R-x\CDT-n` enumeration (PCMIN … TMOUT) and one data-type-format attribute per type, for example `R-x\PDTF-n` "PCM data type format. Enumeration equates to format number in Chapter 11"; `R-x\TTF-n`, `R-x\VTF-n`, and others "equates to format number in Chapter 10"; `R-x\UTF-n` and `R-x\FCTF-n` without that sentence.

**Behaviour**: A reviewed table maps each `R-x\CDT-n` keyword to its Table 11-4 family, and the channel's data-type-format value to the format within it (`docs/TMATS-IN-CHAPTER-10.md` section 3.3). TMOUT has no packet data type. The packet check (L1-CH10-007) reports a packet whose data type is not the one the table gives.

**Reason**: The families are irregular (CAN, Fibre Channel, the missing `0x22`), and several format attributes still cite Chapter 10 after the packet formats moved to Chapter 11, so the mapping cannot be computed and must be reviewed.

**Design**: proposed (2026-09-26, awaiting the owner) · **Review**: pending · **Origin**: F4 section 3

**Test**: `channel_data_type_maps_to_packet_data_types`

**Development**: intensive testing and deep analysis required · **Analysis**: not yet written

### INT-025

**Title**: `R-x\TDTF-n`'s condition names `TSPIN`, which is not a channel type

**Sources**:
- Table 9-4, `R-x\TDTF-n`: "Allowed when: R\CDT is "TSPIN"".
- Table 9-4, `R-x\CDT-n` enumeration: "TSPIIN TSPI/CTS Input"; no `TSPIN`.

**Behaviour**: The condition is read as `TSPIIN`, as INT-007 reads `FBCIN` as `FBCHIN`.

**Reason**: Read literally, the TSPI/CTS data type format could never be allowed.

**Design**: proposed (2026-09-26, awaiting the owner) · **Review**: pending · **Origin**: F4 section 3

**Test**: `tspi_data_type_format_allowed_for_tspiin`

**Development**: intensive testing and deep analysis required · **Analysis**: not yet written

### INT-026

**Title**: Two recorder polarity conditions name `P-d\CDT`, which does not exist

**Sources**:
- Table 9-4, `R-x\RPS-n` (recorder polarity setting): "Allowed when: P-d\CDT is "PCMIN"".
- Table 9-4, `R-x\MFF\RPS-n-m` (recorder polarity setting for a filtered minor frame): "Allowed when: When P-d\CDT is "PCMIN"".
- No `CDT` attribute exists in the P group (Figure 9-6); the channel data type is `R-x\CDT-n`, and the PCM data type attributes around these two are "Allowed when: R\CDT is "PCMIN"" (seven of them in 106-24R1).

**Behaviour**: Both conditions are read as `R\CDT` of the same channel.

**Reason**: Both attributes are in the R group's PCM block beside attributes conditioned on `R\CDT`; `P-d\CDT` cannot be evaluated.

**Design**: proposed (2026-09-26, awaiting the owner) · **Review**: pending · **Origin**: F4 section 3

**Test**: `recorder_polarity_allowed_for_pcm_channels`

**Development**: intensive testing and deep analysis required · **Analysis**: not yet written

### INT-027

**Title**: `C-d\DCN`'s "Links from:" lists `R-x\AMN-n-m` twice

**Sources**:
- Table 9-11, `C-d\DCN`: "Links from: R-x\AMN-n-m, R-x\AMN-n-m, M-x\SI\MN-n, M-x\BB\MN, D-x\MN-y-n, B-x\UMN1-i, B-x\UMN2-i, B-x\UMN3-i, B-x\MN-i-n-p, S-d\MN-i-n-p, R-x\DMN-n-m, Q-d\MMNM-i-n-m, Q-d\SMMNM-i-n-m-o".

**Behaviour**: The duplicate is ignored; the analysis checks whether another R-group measurement-name attribute was meant (the list names `R-x\DMN-n-m` separately) and, if one is found, it becomes a link source here.

**Reason**: A repeated source adds nothing, but may hide an omitted one.

**Design**: proposed (2026-09-26, awaiting the owner) · **Review**: pending · **Origin**: F4 section 3

**Test**: `measurement_name_sources_for_conversions`

**Development**: intensive testing and deep analysis required · **Analysis**: not yet written

## Configuration over a recording

### INT-028

**Title**: Which setup record governs a packet

**Sources**:
- Chapter 11 §11.2.7.2: "the new setup record packet will be committed to the stream prior to any new or changed data packets being committed to the stream".
- Chapter 10 §10.5.1: the setup record "as the first packets in the recording"; Table 10-9.
- Chapter 11: "A single setup record may span multiple consecutive packets."

**Behaviour**: A packet is governed by the most recent complete setup record before it in file order, from the packet after that record's last fragment. A repeated record with the same bytes keeps the same description. Packets before the first complete record have none (`docs/TMATS-IN-CHAPTER-10.md` sections 5 and 6). Recordings written as several simultaneous files are not covered here.

**Reason**: The standard orders setup records before the data they describe but never states the governing rule outright.

**Design**: proposed (2026-09-26, awaiting the owner) · **Review**: pending · **Origin**: F4 section 5

**Test**: `each_packet_is_governed_by_the_preceding_setup_record`

**Development**: intensive testing and deep analysis required · **Analysis**: not yet written

### INT-029

**Title**: Repeated, changed, and inconsistent setup records

**Sources**:
- Chapter 11 §11.2.7.2: SRCC "indicates if the recorder configuration contained in the previous setup record packet(s) of the current recording session (defined as .RECORD to .STOP) has changed"; "The next setup record packet(s) committed to the stream, if not changed from this new setup record, shall clear the SRCC bit to 0."
- Chapter 9 §9.4.2: "For alphanumeric data items, including keywords, either upper or lower case is allowed; TMATS is not case sensitive"; "Leading, trailing, and embedded blanks are assumed to be intentional"; "Attributes may appear in any order."

**Behaviour**: Each complete record is compared with the governing record:
- **identical** — the same bytes (used to share one description);
- **same configuration** — different bytes, but the same attributes: code names compared regardless of order ("Attributes may appear in any order") and, with values, regardless of case ("TMATS is not case sensitive"), as L1-READ-004 and ADR-0026 require; blanks are kept ("assumed to be intentional");
- **changed configuration** — attributes added, removed, or with different values, reported attribute by attribute (L1-WRT-005);
- **indeterminate** — either record has malformed attributes, or the only differences are in attributes the registry does not define, so no defensible judgement is possible; the textual differences are reported.
Findings: SRCC 1 on the first record; SRCC 0 with a changed configuration (an unannounced change, which still governs from there); SRCC 1 with an identical record or the same configuration (a change bit with nothing changed); SRCC 0 with the same configuration but different bytes is information, not a finding. An indeterminate comparison raises no SRCC finding either way. Whether a difference only in descriptive attributes (such as `G\COM` or `G\OD`) is a configuration change is for the analysis. *(Revised 2026-09-27 from byte identity alone, which would have called reordered attributes a change; team spec alignment review.)*

**Reason**: The standard ties SRCC to "the recorder configuration", and Chapter 9 lets the same configuration be written in any order and case; byte identity alone confuses rewriting with changing.

**Design**: proposed (2026-09-26, revised 2026-09-27, awaiting the owner) · **Review**: pending · **Origin**: F4 section 5; team spec alignment review

**Test**: `setup_record_kinds_and_srcc_findings`, `srcc_compares_configuration_not_bytes`

**Development**: intensive testing and deep analysis required · **Analysis**: not yet written

### INT-030

**Title**: The "setup record configuration change event packet" is not defined

**Sources**:
- Chapter 11 §11.2.7.2: "Prior to the new setup record being committed to the stream, a setup record configuration change event packet shall be inserted into the stream."
- No other passage of Chapter 10, Chapter 11, or Chapter 9 (106-24R1) names the packet type, format, or content of that event packet.

**Behaviour**: **Coordinator-decided F7, 2026-11-21; owner may override.** Require verified event identification and report indeterminate meanwhile; never guess a packet type (ADR-0034). This does not identify the event packet or complete L1-CH10-005's check.

**Reason**: Checking for a packet the standard does not identify would rest on a guess (for example, that it is a Format 2 recording event packet).

**Design**: coordinator-decided (F7, 2026-11-21; owner may override) · **Review**: pending · **Origin**: F4 section 5

**Test**: `configuration_change_event_packet` (planned; event identification still unverified)

**Development**: intensive testing and deep analysis required · **Analysis**: not yet written

### INT-031

**Title**: After a setup record that cannot be assembled, nothing governs

**Sources**:
- Chapter 11 §11.2.7.2: "A single setup record may span multiple consecutive packets. When spanning multiple packets, the sequence counter shall increment in the order of segmentation of the setup record, n+1"; "the new setup record packet will be committed to the stream prior to any new or changed data packets".
- The standard does not say what applies after a record that cannot be read.

**Behaviour**: A setup record INT-034 finds incomplete or ambiguous is reported and builds no description by default (a caller may accept an ambiguous split, labelled); the previous description is not carried forward, and packets after it are ungoverned until the next complete record. The fragment bytes are kept.

**Reason**: A new setup record means the configuration may have changed; applying the old description to data the new one describes would decode it wrongly without a word.

**Design**: proposed (2026-09-26, awaiting the owner) · **Review**: pending · **Origin**: F4 section 6

**Test**: `nothing_governs_after_an_incomplete_setup_record`

**Development**: intensive testing and deep analysis required · **Analysis**: not yet written

### INT-032

**Title**: Packets that no setup record governs

**Sources**:
- Chapter 10 §10.5.1 and Table 10-9: the setup record is "the first packets in the recording".
- Chapter 11: "Only static Computer-Generated Data, Format 1 packets may precede the first time data packet."

**Behaviour**: Packets before the first complete setup record, or after one that cannot govern (INT-031, an XML record), are reported with the range affected and are not decoded by default. A caller may assign a description to them explicitly; every report then says the description was assumed, not declared.

**Reason**: The standard forbids such packets but real recordings may contain them; decoding them silently with a guessed configuration would hide the problem.

**Design**: proposed (2026-09-26, awaiting the owner) · **Review**: pending · **Origin**: F4 section 6

**Test**: `ungoverned_packets_are_reported_and_assumptions_labelled`

**Development**: intensive testing and deep analysis required · **Analysis**: not yet written

## Measurements

### INT-033

**Title**: Fragment positions that default to the same value

**Sources**:
- Table 9-7, `D-x\WFP-y-n-m-e`: "A number from 1 to N specifying the position of this fragment within the reconstructed binary data word. 1 corresponds to the most significant fragment. Each fragment position from 1 to N must be specified only once." "Default: 1".
- Appendix 9-C (106-24R1): `D-3\MNF\N-1-1-1:2;` with fragments at `D-3\WP-1-1-1-1:113;` and `D-3\WP-1-1-1-2:121;` and no `D-3\WFP-…`; Table C-2's prose: "the 10 msbs indicated as M and the 6 lsbs as L".

**Behaviour**: Each absent position takes its default, as defaulted values (ADR-0023). When two or more fragments of one location end up with the same position, validation reports it and the fragment order is ambiguous: the library does not infer the order from the fragment index, the word order, or prose. A caller may supply the order explicitly; every report then labels it as an assumption.

**Reason**: The default makes a measurement with several fragments invalid unless every position is given; inferring an order would be a guess that decodes wrongly when it is wrong.

**Design**: proposed (2026-09-26, awaiting the owner) · **Review**: pending · **Origin**: F4 section 7

**Test**: `defaulted_fragment_positions_are_ambiguous`

**Development**: intensive testing and deep analysis required · **Analysis**: not yet written


## Scanner recovery

### INT-035

**Title**: Recover malformed framing only at an actual semicolon

**Sources**:
- 106-24R1 Chapter 9 section 9.4.1, page 9-2: CR/LF may improve readability; nonprintable characters are discarded before translation into configuration information. Original bytes remain retained under ADR-0002.
- Section 9.4.2, page 9-2: "The code name appears first, delimited by a colon."
- Same section/page: "Semicolons are not allowed in any data item"; the section specifies semicolon termination and intentional leading/trailing/embedded blanks.
- Appendix 9-E section E.9.c has `C-6\DCN :DMC;`, motivating L1-READ-006's interpretation of surrounding code-name blanks without changing source bytes.
- L1-READ-001/003 and ADR-0002 require retained bytes, located findings and continued recovery. These are product policies, not additional standard text. INT-019 requires an unterminated SHA item to remain malformed.

**Behaviour**: **Approved by Joey, 2026-11-21, under ADR-0033.** Bound candidate regions by actual semicolons and search for the first colon inside each region only. Line endings are never attribute terminators. Retain a missing-colon region with a located finding and resume after its semicolon. An empty code name (`:v;`) gives a finding. `X:;` is well-framed with a present zero-length raw value, distinct from an absent item; framing alone does not validate the code name or its semantics. Retain an unterminated non-layout suffix with a finding, never fabricate a terminator or expose that suffix as an explicit value in lookups (consistent with INT-019). Do not split on code-like value text, invent delimiters, apply edits or substitute empty success for retained material and findings.

A stray semicolon intended as part of a damaged value can produce false
candidate boundaries. Later code-name-validity, duplicate and unknown-code
findings are the defence, not proof that the intended attributes were recovered.
For example, `G\COM Time 12:30;` is raw-framed at its later colon but must be
flagged by the code-name-validity batch; framing cannot detect that the
writer may have omitted an earlier colon. Do not repair it by guessing.

**Empty-region policy (owner-decided, 2026-11-21)**: emit one information-
level located finding for each maximal run of consecutive empty or layout-
only terminated regions, with one merged source span from the run's first
byte through its last terminating semicolon. `;;;;` therefore produces one
finding over `[0, 4)`. A nonempty attribute or non-layout malformed region
ends the run. Present empty values (`X:;`) and empty code names (`:v;`) are
not empty regions. Unterminated layout or padding at EOF is silent per
section 9.4.1; it remains preserved outside the terminated run's span.

**Character-batch prerequisites**: classify an EOF suffix consisting solely of NUL/0x1A/space padding
without a missing-terminator finding (section 9.4.1); specify CR/LF
inside or before a code name, a lone CR, a UTF-8 BOM and UTF-16 handling.
Do not guess an encoding or transcode the source. Reconcile interpretation
of surrounding code-name blanks with the raw comparison helper: L3-READ-008
must not trim its own input, but the code-name view ignores surrounding
blanks under L1-READ-006 and preserves the original span.

**Reason**: The standard describes valid framing but not recovery. An actual semicolon is a reproducible boundary that avoids borrowing another region's colon; this policy preserves uncertainty rather than claiming the writer's intent was reconstructed.

**Design**: approved (Joey, owner sign-off, 2026-11-21) · **Review**: approved under ADR-0033; draft author: maintainer agent; owner sign-off: Joey, 2026-11-21; independent AI check: CONCURS on PR #13 at e622311, relayed by the owner 2026-11-21 · **Origin**: family 2 batch 2

**Test**: `missing_colon_recovers_at_semicolon`, `unterminated_suffix_is_retained`, `code_like_value_text_does_not_split`, `empty_region_run_has_one_information_finding`, `unterminated_layout_at_eof_is_silent` (planned, not yet written)

**Development**: intensive testing and deep analysis required · **Analysis**: not yet written

**Draft analysis**: [baseline draft and remaining work](research/2026-11-21-scanner-framing-recovery.md)

**Sign-off and AI-check reference**: [owner approval record](research/2026-11-21-int035-owner-signoff.md), referencing [PR #13](https://github.com/TelemetryWorks/irig106-tmats/pull/13) at `e622311690fc5812f689c9d77a19e21447aab6fd` and the owner's dated relay of the independent check.

The baseline draft does not complete cross-edition/handbook/sample analysis
or testing. Approval records the interpretation decision; character-batch
prerequisites, full analysis and executable tests remain implementation gates.
Do not use annotated Appendix 9-E blocks as conforming fixtures; trailing
annotations become the next region's code-name bytes (see TEST-DATA.md).
The line-break/code-name suspect-terminator suggestion is planned separately
in ROADMAP.md, not implemented or a permission to split source.


## Character interpretation (approved, 2026-11-22)

### INT-036

**Title**: Omit ASCII nonprintables only from semantic projections

**Sources**:
- 106-24R1 Chapter 9 section 9.4.1, page 9-2: "Nonprintable characters will be discarded by the destination agency" before translating attributes to configuration information; CR/LF may improve readability.
- Section 9.4.2, same page: "Leading, trailing, and embedded blanks are assumed to be intentional". L1-READ-006 already requires surrounding code-name blanks to be ignored.
- RFC 20 sections 2 and 4.1/4.2 distinguish ASCII control positions, DEL and SP; see the cited draft analysis. ADR-0002/0003 require original byte retention; INT-035 supplies approved empty-run findings and EOF silence.

**Behaviour**: **Approved by Joey, 2026-11-22, under ADR-0033.** Frame on original bytes; omit `00-1F` and `7F` only from interpreted code-name/value projections. Preserve every raw byte and source location. Treat CR/LF, including a lone CR, as omitted characters, never delimiters. Trim ordinary `20` spaces only around interpreted code names, retain internal name spaces for validity checking, and retain value spaces. Do not convert controls to spaces or discard high bytes. A value projected to zero bytes remains present, not absent.

When omitted controls occur between printable bytes inside an interpreted
code name (for example `CO\r\nMMENT`), emit a located information finding
on the omitted bytes. Leading/trailing layout around a name stays silent,
except that NUL anywhere in a code-name or value field emits one located
information finding per affected field, identifying all its NUL locations.
The NUL rule also applies at field edges and when the projected value is empty;
it does not replace an independently applicable internal-name finding.

TAB and other controls in text values (COMMENT, descriptions) disappear from
the interpreted view: `a\tb` becomes `ab`, never `a b`. Human-display
renderers must use retained raw text, with any display decoding/escaping
explicitly labelled, rather than present the filtered projection as original.
Expose raw spans so interpreted spelling is never confused with original bytes.

Classify only spaces plus that nonprintable set as layout/padding for INT-035;
apply its one-information-finding run rule and silent EOF rule. These are
regions, not fields: NUL padding there does not add a field finding. Other
omitted controls in values add no character finding by themselves. Known
leading encoding markers follow INT-037 before projection. Unknown-code and
embedded-blank validity checks still follow.

**Reason**: Chapter 9 requires nonprintable discard for meaning and intentional blanks, while product policy retains bytes. The precise set and separate raw/projected operations need a reviewable interpretation rather than locale/Unicode whitespace heuristics.

**Design**: approved (Joey, owner sign-off, 2026-11-22) · **Review**: approved under ADR-0033; draft author: maintainer agent; owner sign-off: Joey, 2026-11-22; independent AI recheck: CONCURRED on PR #16 at `891f77f`, relayed by the owner 2026-11-22 · **Origin**: family 2 batch 3; review amendments requested 2026-11-21

**Test**: `control_projection_preserves_raw_spans`, `nul_fields_have_one_information_finding_each`, `code_name_and_value_blanks_differ`, `character_classes_keep_int035_run_policy` (planned, not yet written)

**Development**: intensive testing and deep analysis required · **Analysis**: not yet written

**Draft analysis**: [baseline draft and 2026-11-21 review amendments](research/2026-11-21-character-interpretation-drafts.md#review-amendments--2026-11-21)

**Sign-off and AI-check reference**: [owner approval record](research/2026-11-22-int036-int037-owner-signoff.md), referencing [PR #16](https://github.com/TelemetryWorks/irig106-tmats/pull/16) at `891f77f137c9e90ecc15416a84b11d67b6bbde86` and the owner's 2026-11-22 relay of the independent recheck. Approval records the interpretation decision, not implementation, completed analysis or executable tests; those development gates remain outstanding.

### INT-037

**Title**: Diagnose Unicode markers without automatic transcoding

**Sources**:
- 106-24R1 Chapter 9 section 9.4.1, page 9-2, specifies 7-bit ASCII; it does not define Unicode or BOM handling.
- Unicode BOM FAQ, BOM table, lists UTF-8 `EF BB BF`, UTF-16BE/LE `FE FF` / `FF FE`, and UTF-32BE/LE `00 00 FE FF` / `FF FE 00 00`; this is evidence for byte signatures, not a Chapter 9 compatibility rule.
- ADR-0002/0003 preserve raw bytes; L1-READ-003 requires non-ASCII findings. Approved INT-036 defines the separate ASCII control projection.

**Behaviour**: **Approved by Joey, 2026-11-22, under ADR-0033.** At text offset zero, recognize `EF BB BF`, `FE FF`, `FF FE` and `00 00 FE FF`; UTF-32LE `FF FE 00 00` is caught by its `FF FE` prefix. Emit a located unsupported-encoding-marker finding, retain all source bytes and block document semantic lookup. Offset zero means the start of the **assembled TMATS text**, after removal of every Format 1 packet's CSDW and other framing under ADR-0025, not packet/file offset zero. Recognize markers after assembly even when split across fragment packets, before omitting controls or padding. For a standalone input it is the first supplied text byte. Do not strip markers, guess the full encoding or transcode the body.

Outside that leading marker, retain and diagnose `80-FF` bytes. An affected
field has no usable interpreted value: **Invalid**, including a Latin-1
`Temp °C` field containing byte `B0`. Raw bytes remain available. ADR-0003's
explicitly labelled, on-request `as_latin1` and lossy (`as_str_lossy`) views
remain available to callers; they do not turn Invalid into a valid semantic
value. A midstream or partial signature is not a leading BOM. A marker after
leading CR/LF or NUL padding is midstream: do not skip padding and retry the
leading-marker check; ordinary character/field findings apply.

Do not guess UTF-16 from alternating zero bytes. Without a BOM, some UTF-16
bytes can resemble ASCII with controls that INT-036 omits; its per-field
NUL information findings expose most such inputs without guessing an encoding.
This policy does not claim complete encoding detection. A caller aware of another encoding
must convert outside the parser with separate provenance. Never call that
converted data the original TMATS bytes, and never infer an edition from a BOM.

**Reason**: Silently accepting/removing markers or decoding Unicode introduces an undocumented input format and changes the basis for byte locations/checksums. A narrow, explicit detection policy exposes that uncertainty without inventing broad Unicode support.

**Design**: approved (Joey, owner sign-off, 2026-11-22) · **Review**: approved under ADR-0033; draft author: maintainer agent; owner sign-off: Joey, 2026-11-22; independent AI recheck: CONCURRED on PR #16 at `891f77f`, relayed by the owner 2026-11-22 · **Origin**: family 2 batch 3; review amendments requested 2026-11-21

**Test**: `leading_encoding_marker_blocks_semantic_lookup`, `split_marker_is_recognised_after_assembly`, `marker_after_padding_is_midstream`, `high_bytes_are_not_silently_decoded`, `unmarked_zero_patterns_do_not_select_encoding` (planned, not yet written)

**Development**: intensive testing and deep analysis required · **Analysis**: not yet written

**Draft analysis**: [baseline draft and 2026-11-21 review amendments](research/2026-11-21-character-interpretation-drafts.md#review-amendments--2026-11-21)

**Sign-off and AI-check reference**: [owner approval record](research/2026-11-22-int036-int037-owner-signoff.md), referencing [PR #16](https://github.com/TelemetryWorks/irig106-tmats/pull/16) at `891f77f137c9e90ecc15416a84b11d67b6bbde86` and the owner's 2026-11-22 relay of the independent recheck. Approval records the interpretation decision, not implementation, completed analysis or executable tests; those development gates remain outstanding.
