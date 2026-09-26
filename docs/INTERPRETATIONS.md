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
- **Design** is the owner's decision on the behaviour: `accepted` or
  `suspect` (accepted but held in doubt until checked against real data —
  `docs/ROADMAP.md`, "Suspect findings").
- **Review** is ADR-0022's two-person rule for the executable
  interpretation: an author and a different reviewer, both people; tooling
  or AI drafting counts as neither. Until both are recorded the review is
  `pending`.
- **Test** names the focused test that pins the behaviour. The test carries
  a `/// Interpretations: INT-NNN` doc comment directly above `#[test]`;
  `scripts/build-trace-matrix.py` lists every entry and the tests that name
  it, and reports entries without one.
- **Origin** is the team-review item or finding that raised it
  (`docs/ROADMAP.md`).

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

### INT-003

**Title**: "All valid paths are documented" in the Links fields — they are not

**Sources**:
- §9.5.1 b: "Not all valid paths are shown. All valid paths are documented in "Links to:" and "Links from:" attributes."
- INT-001 and INT-002, which contradict it.

**Behaviour**: Relationships are generated from three sources — "Links to:", "Links from:", and the ties of §9.5.1 b — and each relationship found in fewer than all of the sources that could state it has its own register entry. The registry generator fails when such a relationship has none.

**Reason**: No single source is complete; the check keeps every gap visible and decided.

**Design**: accepted (owner, 2026-09-26) · **Review**: pending · **Origin**: T5

**Test**: `one_sided_relationship_without_register_entry_fails_generation`

### INT-004

**Title**: Index letters in code-name references are placeholders

**Sources**:
- Table 9-6, `P-d\DLN`: "Links to: D-x\DLN, B-d\DLN".
- Table 9-8 names the attribute `B-x\DLN`.

**Behaviour**: A reference matches by group and code path; the index letters (`x`, `d`, `n`, …) name positions, not particular letters, so "B-d\DLN" is `B-x\DLN`.

**Reason**: The group occurrence letter differs between tables for the same attribute; matching by letter would lose the P → B link.

**Design**: accepted (owner, 2026-09-26) · **Review**: pending · **Origin**: T4

**Test**: `pcm_data_link_name_links_to_bus_data_link_name`

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

### INT-006

**Title**: `H\TA` ties the H group to the G group by test item

**Sources**:
- §9.5.12: "The only H group attributes defined in this standard are the following: a. Test Item (code name H\TA) - specifies the item under test and ties the H group to the G group. b. Airborne System Type (code name H\ST-n) - identifies the airborne systems being described in the current file and determines how the rest of the attributes in the H group will be interpreted."
- Table 9-2: "TEST ITEM G\TA Allowed when: Always".

**Behaviour**: `H\TA` links to `G\TA` by value. `H\TA` and `H\ST-n` are built in; every other H attribute is organisation-defined, supplied through the user overlay (ADR-0008) and preserved when no definition is given.

**Reason**: The standard gives the tie in prose without naming the attribute that carries it on the G side; `G\TA` is G's test-item attribute. There is no H table to transcribe; §9.7 says IHAL serves "the purpose originally intended for the Airborne Hardware Attributes (H) group …, which has never been implemented".

**Design**: accepted (owner, 2026-09-26) · **Review**: pending · **Origin**: T5

**Test**: `h_group_test_item_ties_to_g_group`

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

### INT-008

**Title**: `C-d\DPNO`'s condition names `C\DCT` without an index

**Sources**:
- Table 9-11, `C-d\DPNO`: "Allowed when: C\DCT is "DER"" and "Default is 1."

**Behaviour**: `C\DCT` means `C-d\DCT` of the same C occurrence; the default of 1 is taken from the prose.

**Reason**: Conditions are written for people; the only occurrence that makes sense is the attribute's own (ADR-0022, section 4.2 of the architecture).

**Design**: accepted (owner, 2026-09-26) · **Review**: pending · **Origin**: T1

**Test**: `derived_occurrences_default_to_one_in_same_occurrence`

### INT-009

**Title**: Counters named without indices in conditions

**Sources**:
- Table 9-7, `D-x\WFT-y-n-m-e` (fragment transfer order) and `D-x\WFP-y-n-m-e` (fragment position): "Allowed when: D\MNF\N > 1"; `D-x\WFP-y-n-m-e` also has "Range: 1 - D\MNF\N".

**Behaviour**: An unindexed counter in a condition means the counter of the same parent-index combination as the attribute being checked — for `D-x\WFP-y-n-m-e`, `D-x\MNF\N-y-n-m`, the fragment count of the same location.

**Reason**: Counters count within their parent (ADR-0026); any other occurrence would make the condition meaningless.

**Design**: accepted (owner, 2026-09-26) · **Review**: pending · **Origin**: T1, T4

**Test**: `fragment_position_allowed_only_when_its_own_location_has_fragments`

### INT-010

**Title**: Equality is `==`, although Table E-3 prints `= =`

**Sources**:
- Appendix 9-E, Table E-3: "= = Equal To A = = B"; Table E-6: "= = != Left to right".
- Appendix 9-E grammar: "| expression '==' expression".

**Behaviour**: `==` is the equality operator; `= =` is accepted with a warning.

**Reason**: The grammar is the machine-readable source; the table's spacing is typographic (ADR-0024).

**Design**: accepted (owner, 2026-09-26) · **Review**: pending · **Origin**: T2

**Test**: `spaced_equality_operator_is_accepted_with_warning`

## Chapter 10 setup records

### INT-011

**Title**: Where one multi-packet setup record ends

**Sources**:
- Chapter 11 §11.2.7.2: "A single setup record may span multiple consecutive packets. When spanning multiple packets, the sequence counter shall increment in the order of segmentation of the setup record, n+1". No end-of-record marker is defined.

**Behaviour**: A record is a run of consecutive data type `0x01` packets on one channel whose sequence numbers increase by one modulo 256 and whose CSDWs agree on FRMT and RCCVER; it ends at an intervening packet, a sequence gap, a CSDW change, or the end of input. Anything ambiguous is reported.

**Reason**: The standard says how fragments follow each other but not how a record ends (ADR-0025).

**Design**: accepted (owner, 2026-09-26) · **Review**: pending · **Origin**: T3

**Test**: `multi_packet_setup_record_is_assembled` (`docs/TEST-DATA.md`)

### INT-012

**Title**: RCCVER `0x0E` means "106-22 or later"

**Sources**:
- Chapter 11, setup-record CSDW (106-24R1): "0x0E = RCC 106-22", "0x0F through 0xFF = Reserved"; there is no code for 106-20, 106-23, or 106-24.

**Behaviour**: `0x0E` is reported as "106-22 or later", reserved values as an unknown edition, and `G\106` stays the primary edition source (ADR-0016).

**Reason**: Later editions did not assign new codes, so `0x0E` cannot identify one edition.

**Design**: accepted (owner, 2026-09-26) · **Review**: pending · **Origin**: T3

**Test**: `rccver_0x0e_reads_as_106_22_or_later`

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
