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

**Design**: accepted (owner, 2026-09-26) · **Review**: independent rule check CONCUR (2026-11-27) on main `c682e05babc5a9d7e6fe104e22bd278d68aba6af`; existing owner acceptance: Joey, 2026-09-26; draft author: maintainer agent; [batch evidence](research/2026-11-27-legacy-int-independent-check-batch1.md); source-freshness/development-analysis/executable-test gates remain · **Origin**: T5

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

**Design**: accepted (owner, 2026-09-26) · **Review**: independent rule check CONCUR (2026-11-27) on main `c682e05babc5a9d7e6fe104e22bd278d68aba6af`; existing owner acceptance: Joey, 2026-09-26; draft author: maintainer agent; [batch evidence](research/2026-11-27-legacy-int-independent-check-batch1.md); source-freshness/development-analysis/executable-test gates remain · **Origin**: T5

**Test**: `sub_channel_and_network_names_resolve_to_their_groups`

**Development**: intensive testing and deep analysis required · **Analysis**: not yet written

### INT-003

**Title**: "All valid paths are documented" in the Links fields — they are not

**Sources**:
- §9.5.1 b: "Not all valid paths are shown. All valid paths are documented in "Links to:" and "Links from:" attributes."
- INT-001 and INT-002, which contradict it.

**Behaviour**: Relationships are generated from three sources — "Links to:", "Links from:", and the ties of §9.5.1 b — and each relationship found in fewer than all of the sources that could state it has its own register entry. The registry generator fails when such a relationship has none.

**Reason**: No single source is complete; the check keeps every gap visible and decided.

**Design**: accepted (owner, 2026-09-26) · **Review**: independent rule check CONCUR (2026-11-27) on main `c682e05babc5a9d7e6fe104e22bd278d68aba6af`; existing owner acceptance: Joey, 2026-09-26; draft author: maintainer agent; [batch evidence](research/2026-11-27-legacy-int-independent-check-batch1.md); source-freshness/development-analysis/executable-test gates remain · **Origin**: T5

**Test**: `one_sided_relationship_without_register_entry_fails_generation`

**Development**: intensive testing and deep analysis required · **Analysis**: not yet written

### INT-004

**Title**: Index letters in code-name references are placeholders

**Sources**:
- Table 9-6, `P-d\DLN`: "Links to: D-x\DLN, B-d\DLN".
- Table 9-8 names the attribute `B-x\DLN`.

**Behaviour**: A reference matches by group and code path; the index letters (`x`, `d`, `n`, …) name positions, not particular letters, so "B-d\DLN" is `B-x\DLN`.

Matching also requires equal index-position arity. This placeholder
comparison applies only to definition-table references, never to actual
numeric document index values (whose identity follows INT-038). It does not
make different document occurrences equal or discard their raw spelling.

**Reason**: The group occurrence letter differs between tables for the same attribute; matching by letter would lose the P → B link.

**Design**: accepted (owner, 2026-09-26) · **Review**: independent rule check CONCUR (2026-11-27) on main `c682e05babc5a9d7e6fe104e22bd278d68aba6af`; existing owner acceptance: Joey, 2026-09-26; draft author: maintainer agent; [batch evidence](research/2026-11-27-legacy-int-independent-check-batch1.md); source-freshness/development-analysis/executable-test gates remain · **Origin**: T4

**Test**: `pcm_data_link_name_links_to_bus_data_link_name`, `definition_placeholder_arity_must_match`, `definition_placeholders_do_not_equal_document_indices` (planned; not yet written)

**Development**: intensive testing and deep analysis required · **Analysis**: not yet written

### INT-005

**Title**: The channel data type selects among overlapping link targets

**Sources**:
- Table 9-8, `B-x\DLN`: "Links from: R-x\CDLN, P-d\DLN, R-x\EV\DLN-n".
- Table 9-6, `P-d\DLN`: "Links to: D-x\DLN, B-d\DLN".
- Table 9-4, `R-x\CDT-n` enumeration: "PCMIN PCM Input", "1553IN 1553 Input", "429IN ARINC 429 Input", "UARTIN UART Input", "MSGIN Message Data Input", "1394IN IEEE-1394 Input", "ETHIN Ethernet Input", "FBCHIN Fibre Channel Input", among others.
- The S and Q conditions quoted in INT-002.

**Behaviour**: For `R-x\CDLN-n`, the channel data type `R-x\CDT-n` of the same channel selects the target group: `PCMIN` → P; `1553IN`, `429IN` → B; `UARTIN` → S; `MSGIN`, `ETHIN`, `FBCHIN` → S or Q; `1394IN` → Q; any other type has no data-link target. Candidates in other groups are excluded; several remaining candidates make the link ambiguous (ADR-0026).

This selection requires a usable same-channel CDT. Missing, Invalid,
Ambiguous (including duplicate equal values under INT-039), or unavailable
CDT makes target selection **cannot-evaluate**, with every candidate group
and all CDT occurrences/findings retained, never resolved or an empty/no-target
answer. Do not filter candidates by a guessed type. The Q target for FBCHIN
depends on INT-007's reviewed reading of the standard's FBCIN as FBCHIN;
a document spelling CDT=FBCIN is still an unknown keyword, not repaired.

**Reason**: Bus data carried in a PCM stream shares the P group's data-link name, so a PCM channel would otherwise match both P and B.

**Design**: accepted (owner, 2026-09-26) · **Review**: initial independent check CONCERNS (narrow, 2026-11-27) on main `c682e05babc5a9d7e6fe104e22bd278d68aba6af`; amended-wording recheck CONCUR at #81 head `699029b9e739f7bef7f27e7f67d8cac23c20d90d` (2026-11-27); existing owner acceptance: Joey, 2026-09-26; draft author: maintainer agent; [batch evidence](research/2026-11-27-legacy-int-independent-check-batch1.md); source-freshness/development-analysis/executable-test gates remain · **Origin**: T4

**Test**: `pcm_channel_carrying_bus_data_resolves_to_p_group`, `link_target_unusable_cdt_keeps_all_groups`, `link_target_fibre_channel_uses_int007` (planned; not yet written)

**Development**: intensive testing and deep analysis required · **Analysis**: not yet written

### INT-006

**Title**: `H\TA` ties the H group to the G group by test item

**Sources**:
- §9.5.12: "The only H group attributes defined in this standard are the following: a. Test Item (code name H\TA) - specifies the item under test and ties the H group to the G group. b. Airborne System Type (code name H\ST-n) - identifies the airborne systems being described in the current file and determines how the rest of the attributes in the H group will be interpreted."
- Table 9-2: "TEST ITEM G\TA Allowed when: Always" (p. 9-10).
- Section 9.4.2 p. 9-2: alphanumeric data are case-insensitive; blanks are "assumed to be intentional; they can be ignored in most cases but should not be used in code names, keywords, and data items used as links".

**Behaviour**: `H\TA` links to `G\TA` by value. `H\TA` and `H\ST-n` are built in; every other H attribute is organisation-defined, supplied through the user overlay (ADR-0008) and preserved when no definition is given.

Compare the value tie ASCII case-insensitively, without locale/Unicode or
high-byte folding. INT-036/037 character guards and INT-041 blank handling
remain in force: a leading blank is unavailable, never silently trimmed;
other unreviewed blank-bearing link values do not acquire a guessed trim or
matching rule. Multiple G\TA or H\TA occurrences are Ambiguous with every
occurrence retained, including identical repeats (ADR-0026/INT-039), never
first/last wins. B\TA or S\TA is not a substitute target. Link evaluation
never upgrades record qualifications or approves semantic access.

**Reason**: The standard gives the tie in prose without naming the attribute that carries it on the G side; `G\TA` is G's test-item attribute. There is no H table to transcribe; §9.7 says IHAL serves "the purpose originally intended for the Airborne Hardware Attributes (H) group …, which has never been implemented".

**Design**: accepted (owner, 2026-09-26) · **Review**: initial independent check CONCERNS (narrow, 2026-11-27) on main `c682e05babc5a9d7e6fe104e22bd278d68aba6af`; amended-wording recheck CONCUR at #81 head `699029b9e739f7bef7f27e7f67d8cac23c20d90d` (2026-11-27); existing owner acceptance: Joey, 2026-09-26; draft author: maintainer agent; [batch evidence](research/2026-11-27-legacy-int-independent-check-batch1.md); source-freshness/development-analysis/executable-test gates remain · **Origin**: T5

**Test**: `h_group_test_item_ties_to_g_group`, `h_test_item_ascii_match_keeps_raw`, `h_test_item_leading_blank_is_unavailable`, `h_test_item_duplicate_occurrences_are_ambiguous` (planned; not yet written)

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

**Design**: accepted (owner, 2026-09-26) · **Review**: independent rule check CONCUR (2026-11-27) on main `c682e05babc5a9d7e6fe104e22bd278d68aba6af`; existing owner acceptance: Joey, 2026-09-26; draft author: maintainer agent; [batch evidence](research/2026-11-27-legacy-int-independent-check-batch1.md); source-freshness/development-analysis/executable-test gates remain · **Origin**: T5

**Test**: `q_group_allowed_for_fibre_channel`

**Development**: intensive testing and deep analysis required · **Analysis**: not yet written

### INT-008

**Title**: `C-d\DPNO`'s condition names `C\DCT` without an index

**Sources**:
- Table 9-11, `C-d\DPNO`: "Allowed when: C\DCT is "DER"" and "Default is 1."

**Behaviour**: `C\DCT` means `C-d\DCT` of the same C occurrence; the default of 1 is taken from the prose, with its provenance explicitly labelled as Definition prose rather than a printed `Default:` usage field.

**Reason**: Conditions are written for people; the only occurrence that makes sense is the attribute's own (ADR-0022, section 4.2 of the architecture).

**Design**: accepted (owner, 2026-09-26) · **Review**: independent rule check CONCUR (2026-11-27) on main `c682e05babc5a9d7e6fe104e22bd278d68aba6af`; existing owner acceptance: Joey, 2026-09-26; draft author: maintainer agent; [batch evidence](research/2026-11-27-legacy-int-independent-check-batch1.md); source-freshness/development-analysis/executable-test gates remain · **Origin**: T1

**Test**: `derived_occurrences_default_to_one_in_same_occurrence`

**Development**: intensive testing and deep analysis required · **Analysis**: not yet written

### INT-009

**Title**: Counters named without indices in conditions

**Sources**:
- Table 9-7, `D-x\WFT-y-n-m-e` (fragment transfer order) and `D-x\WFP-y-n-m-e` (fragment position): "Allowed when: D\MNF\N > 1"; `D-x\WFP-y-n-m-e` also has "Range: 1 - D\MNF\N".

**Behaviour**: An unindexed counter in a condition means the counter of the same parent-index combination as the attribute being checked — for `D-x\WFP-y-n-m-e`, `D-x\MNF\N-y-n-m`, the fragment count of the same location.

**Reason**: Counters count within their parent (ADR-0026); any other occurrence would make the condition meaningless.

**Design**: accepted (owner, 2026-09-26) · **Review**: independent rule check CONCUR (2026-11-27) at main `c682e05`; existing owner acceptance and development/analysis/test gates retained; [batch evidence](research/2026-11-27-legacy-int-independent-check-batch2.md) · **Origin**: T1, T4

**Test**: `fragment_position_allowed_only_when_its_own_location_has_fragments`

**Development**: intensive testing and deep analysis required · **Analysis**: not yet written

### INT-010

**Title**: Equality is `==`, although Table E-6 prints `= =`

**Sources**:
- 106-24R1 Appendix 9-E, Table E-3 p. E-2: "== Equal To A==B"; Table E-6 p. E-3: "= = != Left to right". The spaced occurrence is E-6, not E-3.
- 106-24R1 Appendix 9-E grammar, p. E-5 region: "| expression '==' expression".

**Behaviour**: `==` is the equality operator; `= =` is accepted with a warning.

**Reason**: The grammar is the machine-readable source; the table's spacing is typographic (ADR-0024).

**Design**: accepted (owner, 2026-09-26) · **Review**: initial independent check CONCERNS (narrow source correction, 2026-11-27) at main `c682e05`; correction applied; recheck CONCUR at #81 head `699029b9e739f7bef7f27e7f67d8cac23c20d90d` (2026-11-27); [batch evidence](research/2026-11-27-legacy-int-independent-check-batch2.md) · **Origin**: T2

**Test**: `spaced_equality_operator_is_accepted_with_warning`

**Development**: intensive testing and deep analysis required · **Analysis**: not yet written

## Chapter 10 setup records

### INT-011

**Title**: Per-channel joining of multi-packet setup-record fragments

**Sources**:
- 106-24R1 Chapter 11 section 11.2.7.2, page 11-50: "A single setup record may span multiple consecutive packets. When spanning multiple packets, the sequence counter shall increment in the order of segmentation of the setup record, n+1". No end-of-record marker is defined.
- Chapter 11 section 11.2.1.1 f: "Each channel in a session shall have its own sequence counter providing a unique sequence number for that channel".
- Owner decision, Joey, 2026-11-23, following AI CONCERNS on #37: intervening means SAME-CHANNEL; joining is per-channel and independent. Section 11.2.1.1 b's channel-0 restriction from 106-17 is a separate edition-dependent check.

**Behaviour**: **Reconfirmed: Joey, owner decision 2026-11-23; independent AI recheck CONCURS on #38 at c623b15, under ADR-0033.**
Maintain independent candidate runs for each channel. Join consecutive data
type 0x01 fragments within that channel when sequence increases by one modulo
256 and decoded FRMT/RCCVER agree. Runs on different channels never mix or
interrupt one another, including older recordings with nonzero setup channels.

An "intervening packet" means a SAME-CHANNEL packet only; the +1 sequence
test already detects the consumed sequence number. Such a packet stops that
channel's join. Packets from OTHER channels between fragments are recorded
as provenance and information only and NEVER stop the join. Do not turn a
change of channel in global input order into a boundary for an open run.

Retain each run's START reason, e.g. "start of supplied input", alongside
stop reasons and their source evidence, so INT-034 can consider leading
truncation. A same-channel interruption, sequence discontinuity, FRMT/RCCVER
change or end of supplied input stops the relevant joining; no such reason
alone proves completeness. INT-034's precise completeness rules under
ADR-0031 remain separately pending. Channel 0x0000 from 106-17 is an edition-dependent
conformance check, never a joining predicate or reason to merge/drop runs.
SRCC disagreement remains preserved without a new joining/aggregation rule.

**Reason**: Sequence counters are per channel. Treating multiplexed traffic
on other channels as a boundary would split valid fragments and couple
independent setup runs. The owner clarifies "consecutive" within a channel;
this amends the joining interpretation, not the standard's quoted wording.
The 2026-09-27 distinction between stopping a run and proving a complete
record remains; leading-boundary evidence is now retained as well.

**Design**: approved (Joey, owner reconfirmation, 2026-11-23) · **Review**: complete under ADR-0033; draft author: maintainer agent; owner reconfirmation: Joey, decision 2026-11-23; independent AI check: initial CONCERNS on #37 at fabbc06, amended-wording CONCURS on #38 at c623b15 · **Origin**: T3; owner per-channel clarification, 2026-11-23

**Test**: `multi_packet_setup_record_is_assembled`, `other_channel_packets_do_not_stop_joining`, `two_setup_channels_have_independent_runs`, `same_channel_packet_stops_joining`, `run_start_reasons_remain_evidence`, `setup_channel_restriction_is_not_joining` (planned; not yet written)

**Development**: intensive testing and deep analysis required · **Analysis**: not yet written

**Decision record**: [owner per-channel clarification and review status](research/2026-11-23-int011-per-channel-owner-decision.md).

**Reconfirmation record**: [owner reconfirmation and exact AI reference](research/2026-11-23-int011-owner-signoff.md). Source freshness, development analysis and executable tests remain required.

**Layered closure cross-reference**: INT-045 is the separate phase-closure
layer above this per-channel joining rule. Its initial A is signed and its
guarded owner mid-session extension is approved (AI CONCURS on #41 at
79b94eb). This entry's joining predicates and approval are unchanged.

**Per-edition channel-use review input (not a new approval)**: The separate
conformance check needs the applicable edition's channel-ID source and scope,
not one baseline date cutoff. Project-archived 106-13 §10.6.1.1 b, p. 10-29,
says reserved 0x0000 "as of 106-13"; 106-24R1 §11.2.1.1 b, p. 11-5, says
"as of 106-17". See the [per-edition source table](research/2026-11-23-channel-zero-edition-sources.md),
including older single-multiplexer qualifications. This source-review item
changes no joining predicate, approved status or historical threshold policy.

### INT-012

**Title**: RCCVER codes are read as their table states; later editions' reuse is a note

**Sources**:
- Chapter 11, setup-record CSDW (106-24R1, Figure 11-34): "0x0E = RCC 106-22", "0x0F through 0xFF = Reserved"; the same list in 106-22, 106-23, and 106-24.
- 106-19 and 106-20 Chapter 11: "0x0D = RCC 106-19", "0x0E through 0xFF = Reserved".

**Behaviour**: **Reconfirmed by Joey, 2026-11-23, under ADR-0033.** `0x0E` is reported as "RCC 106-22" with the labelled note "unchanged through 106-24R1: 106-23, 106-24, and 106-24R1 assign no newer code"; `0x0D` as "RCC 106-19" with the note "106-20 assigns no newer code"; reserved values as an unknown edition. `G\106` stays the primary edition source (ADR-0016). *(Revised 2026-09-27, ADR-0032: previously "106-22 or later".)*

**Scope and analysis notes (2026-11-23)**: This is the **Format 1** setup-
record RCCVER table, Figure 11-34, not a universal version-byte decoder.
The Format 4 Streaming Configuration Record has its own table: in 106-24R1
section 11.2.7.5/Figure 11-43, page 11-62, 0x0C = RCC-106-17 and 0x0D–0xFF
remain reserved. It must never borrow Format 1's mappings. Figure 11-41 in
this edition is the Recording Index Intra-Packet Header, not the Format 4
CSDW; retain edition-specific citations rather than copying figure numbers.

The packet-header Data Type Version is a third code space: section 11.2.1.1 e,
page 11-6, assigns 0x0A = RCC 106-22. Keep it distinct under L3-CH10-008;
0x0A in the Format 1 RCCVER table instead means RCC 106-13. Raw values require
their code-space/table identity even if field names look similar.

0x0E -> 106-24R1 rules and 0x0D -> 106-20 rules are ADR-0032/INT-017 fallback
POLICY, not INT-012's literal reading. Tests must keep those separate; this
reconfirmation does not approve INT-017 or select a fallback from a Format 4
or packet-header byte. Full development analysis and tests remain outstanding.

**Reason**: The table names one edition per code. That later editions kept the code is an observation about the archived editions, not a promise about future ones (team spec alignment review). RCCVER declares the recording-format version, not the TMATS edition; see INT-016 and INT-017 (ADR-0028).

**Design**: approved (Joey, reconfirmed 2026-11-23; originally owner-accepted 2026-09-26, revised 2026-09-27) · **Review**: approved under ADR-0033; draft author: maintainer agent; owner reconfirmation: Joey, 2026-11-23; independent AI check: CONCURRED on PR #34 head `e35232391326a4e56e478c5a3a33e10892038b04` after reading seven archived-edition tables, as relayed by the owner · **Origin**: T3; team spec alignment review

**Test**: `rccver_codes_read_as_their_table_states`, `version_code_spaces_do_not_share_mapping`, `literal_rccver_is_not_fallback_policy` (planned; not yet written)

**Development**: intensive testing and deep analysis required · **Analysis**: not yet written

**Reconfirmation and AI-check reference**: [owner approval record](research/2026-11-23-int012-owner-signoff.md); AI concurrence on #34 head `e352323` after seven-edition source review is owner-relayed, not a claim that this new PR or these later notes have been independently reviewed.

### INT-034

**Title**: A joined run is complete, incomplete, or ambiguous

**Owner sign-off**: Joey, 2026-11-28, under ADR-0033; [verbatim decision](research/2026-11-28-interpretation-owner-signoffs.md#int-034).

**Sources**:
- Chapter 11 §11.2.7.2: "A single setup record may span multiple consecutive packets. When spanning multiple packets, the sequence counter shall increment in the order of segmentation of the setup record, n+1". No end-of-record marker is defined.
- §11.2.1.1 f: "Each channel in a session shall have its own sequence counter providing a unique sequence number for that channel" — two records back to back on one channel continue one count.
- Chapter 9 §9.4.2: each attribute is a code name and data item ended by a semicolon; "Semicolons are not allowed in any data item (including comment items)."

**Behaviour**: For each run joined under INT-011:
- **complete** — no gap or CSDW change inside it, the body's final substantive byte under the owner-signed boundary-layout definition below is `;`, and the run is followed by a SAME-CHANNEL packet (basis "followed by another packet") or by the end of input (basis "end of input", labelled);
- **incomplete** — the body is cut inside an attribute (a substantive unterminated suffix remains after the boundary-layout check), a fragment has unavailable required source bytes/bounds, or a sequence gap splits text that continues an attribute across it;
- **ambiguous** — a sequence gap or a CSDW change where the text before it ends at `;`, or a gap-free run whose text repeats a single-entry attribute (for example `G\106` or `G\PN`); both readings and the split point are reported.

**Owner-signed precise definitions (2026-11-28); remaining final check pending**:
- **Boundary layout/padding** is exactly ASCII SP (0x20), controls 0x00–0x1F,
  and DEL (0x7F), per INT-035/036. Thus it includes CR/LF, TAB, NUL and 0x1A,
  not Unicode whitespace or any high byte. For the owner-signed final-terminator
  check only, inspect the maximal suffix from that set after retained source
  framing: ';' plus only that suffix is semicolon-terminated. Keep every raw
  byte; do not trim field values, omit an unterminated substantive suffix or
  repair a number via this check. Padding-only/empty bodies do not establish
  a complete attribute boundary. INT-037 markers still apply before projection.
- **Gap** is failure of same-channel next sequence = (previous + 1) modulo
  256. 0xFF -> 0x00 is continuity (Ch11 11.2.1.1 f p. 11-8), not truncation.
  Other-channel packets do not create gaps; existing INT-011/045 boundaries
  remain independent. Missing observations cannot prove continuity.
- Replace generic **untrusted** with explicit source/admission availability:
  unreadable required source bytes; insufficient CSDW/text extents; or core
  structural header/admission evidence withholding a usable DATA extent.
  Do not fabricate a fragment or infer that every refused packet belonged to
  a run. Only independently established missing required run bytes support
  an incomplete-source facet. Unknown inventory remains observable as such.
  Core primary-header/admission failure is not bypassed by readable-looking
  bytes or G\SHA; actual PacketInfo evidence and bounds must be verified.
- If DATA is admitted and the required bytes/bounds remain available, core
  data-checksum Mismatch or NotChecked and secondary_integrity readings do
  not alone make the text Incomplete. Preserve findings and core evidence
  unchanged under INT-051; no tmats secondary-checksum algorithm is chosen.
  Verified likewise proves neither assembly completeness nor takeover.
  A resource refusal (HardLimit/BudgetTooSmall/TargetLimit or assembler limit)
  stays operational evidence, not a manufactured malformed/cut verdict.

**Options considered (owner selected A, 2026-11-28)**: (a) A (recommended) explicit versioned provisional snapshots
for caller use, with invalidation and no automatic governing effect; B final-
only semantic use. (b) A (recommended) the boundary-layout set and source-
availability distinction above, with known cut facets always blocking and
INT-048 field ambiguity aggregate priority; preserve independently established
leading damage and competing split evidence. B withholds the unsettled
padding/availability classification until separately reviewed, not a literal-
space-only implementation. Empty/padding-only and conflicting leading-damage
precedence still need the owner's explicit choice; do not claim this proposal
settles them. Any stronger integrity quarantine is a separate opt-in policy
requiring owner review; it must not weaken approved default INT-051.

**Review consequences (owner-relayed, 2026-11-23)**: "Followed by another
packet" means SAME-CHANNEL under reconfirmed INT-011. Other-channel traffic
never closes a run. An otherwise complete run ending at EOF in `;` is
**complete**, labelled "end of input", not downgraded solely for EOF. Retain
START evidence without treating an ordinary input-start label as proof of
leading truncation. Independently established damage, repeated scalar/split
ambiguity and unusable format evidence still require their own decisions;
EOF alone does not erase those blockers or choose their precedence.

An open run needs a separate **PROVISIONAL** observation, e.g. "text ends
at ;, run still open", for streaming consumers. This is never a joining stop
or an assertion of final completeness. Joey selected explicit versioned
provisional snapshots with visible later
invalidation on 2026-11-28; this is never automatic governance or permission
to use cut text. Preserve the snapshot version/evidence and re-evaluation.
Padding-only/leading-damage precedence remains open below.
INT-045 now records a separate owner-decided layered closure: initial setup
and SRCC=1 updates close at their eligible time boundary (approved, AI
CONCURS on #41 at 79b94eb). At that boundary a mid-attribute suffix is INCOMPLETE
with a located finding; a same-channel +1 fragment after it is recorded as
continuous-but-split and never rejoined backward. A semicolon-ending seal
followed by matching +1 continuity becomes finally AMBIGUOUS with both
readings and a located possible-premature-seal finding, unless G\SHA verifies
the sealed text alone (classification evidence only). Mid-session Complete
under INT-045 is labelled `time-boundary seal (owner policy)`. A boundary does not approve
all other precise classification/governance rules in this entry.

Aggregate open-run memory needs an explicit budget and typed limit error;
exhaustion never silently drops a channel's run or synthesizes EOF.

Complete alone does not prove governing adoption: owner-signed INT-028
composes INT-053. Owner-signed INT-031 blocks incomplete/ambiguous use by default. An XML body (FRMT 1) is of unknown completeness.

**INT-048 amendment (review clarification, 2026-11-25)**: The cut-text rule
above means **Incomplete, or aggregate Ambiguous WITH A RETAINED CUT FACET**
when established field ambiguity (including duplicate G\SHA) invokes
INT-048. The cut facet keeps Incomplete's FULL blocking force. No acceptance
of an ambiguous split, including owner-signed INT-031's labelled option,
can make cut text usable or governing. Retain the cut location and
blocking reason; the aggregate label does not relax incomplete-input policy.

**Reason**: The packets alone cannot tell where a record ends; the text's final byte and repeated single-entry attributes are the only evidence the standard gives, and neither is proof in every case.

**Design**: owner-approved in part (Joey, 2026-11-28; padding-only/leading-damage precedence open) · **Review**: complete under ADR-0033 for the signed parts; draft author: maintainer agent; owner sign-off: Joey, 2026-11-28; independent AI FINAL CONCUR at [PR #87 head `e0a0ccea5a61c64f7de9ca0e122adb01f376a3f9`](https://github.com/TelemetryWorks/irig106-tmats/pull/87/commits/e0a0ccea5a61c64f7de9ca0e122adb01f376a3f9), reviewer @thread:thr_qsvwmtnn8f, received 2026-11-28; [final-check record](research/2026-11-28-int022-066-final-concurrences.md); source freshness, written development analysis/review and intensive executable tests remain required; padding-only/leading-damage precedence remains OWNER-OPEN · **Origin**: team spec alignment review, 2026-09-27

**Test**: `setup_record_completeness_outcomes`, `eof_boundary_is_complete_and_labelled`, `other_channel_packet_cannot_finalize_run`, `open_boundary_is_provisional_not_complete`, `provisional_extension_never_closes_run` (planned; not yet written)

**Development**: intensive testing and deep analysis required · **Analysis**: not yet written

**Remaining owner item**: precedence for padding-only records and leading
damage, including their interaction with competing duplicate/split evidence
and INT-035/036 padding at EOF. No algorithm for that precedence is approved.
Versioned provisional snapshots, exact boundary padding/source availability
and modulo-256 continuity ARE owner-signed; this open item does not revoke
INT-048 retained-cut blocking force or permit automatic governance. See
[batch 15 review draft](research/2026-11-23-completeness-contracts-draft.md).

## Errata in the standard's own examples

### INT-013

**Title**: Appendix 9-C ends 18 attributes with `:` instead of `;`

**Sources**:
- Appendix 9-C, page C-8 (106-24R1), for example "D-1\MML\N-1-1:2: D-1\MNF\N-1-1-1:1: D-1\WP-1-1-1-1:14;"; the same 18 in every edition since 106-17.
- §9.4.2: attributes are "CODE:value;".

**Behaviour**: The attribute is kept exactly as read (the first colon ends the code name) and a warning, "possible `;` typed as `:`", is reported with a suggested edit (L1-READ-007, ADR-0026).

The independent check confirms the recurring colon-ended pattern across
106-17/19/20/22/23/24/24R1, absent in 106-13/15 text. The exact historical
count of 18 is line-wrap/extraction-sensitive and was not independently
reproduced as exactly 18; it is not a required detector match count. S1
remains suspect and no repair is authorized.

**Reason**: Nothing is split silently (ADR-0002, ADR-0007).

**Design**: **suspect** (owner, 2026-09-26; `docs/ROADMAP.md`, S1) · **Review**: independent rule check CONCUR (2026-11-27) at main `c682e05`; existing owner acceptance and development/analysis/test gates retained; [batch evidence](research/2026-11-27-legacy-int-independent-check-batch2.md) · **Origin**: T4

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

**Design**: accepted (owner, 2026-09-26) · **Review**: independent rule check CONCUR (2026-11-27) at main `c682e05`; existing owner acceptance and development/analysis/test gates retained; [batch evidence](research/2026-11-27-legacy-int-independent-check-batch2.md) · **Origin**: T6

**Test**: `g106_24_names_106_24_and_106_24r1`

**Development**: intensive testing and deep analysis required · **Analysis**: not yet written

### INT-015

**Title**: `G\106` year-format wording appears by 106-15

**Sources**:
- 106-05 and 106-07 Chapter 9, `G\106`: "VERSION OF IRIG 106 STANDARD USED TO GENERATE THIS TMATS FILE" (no year-format wording); both already have maximum field size 2.
- 106-13 Chapter 9, `G\106`: "Version of RCC IRIG 106 standard used to generate this TMATS file." with MFS 2, not a newly introduced size constraint.
- 106-15 Chapter 9 already states "The last 2 digits of the year should be used. Use a leading 0 if necessary." and "Range: 0-99"; 106-13 does not state that year format.

**Behaviour**: Only a value that reads as the year of an archived edition (INT-014) is recognised. Any other form — `106-05`, `2005`, `IRIG 106-05` — is reported as "unrecognised", preserved exactly, and not interpreted; the fallback of ADR-0028 then applies, and the caller may override.

The corrected source onset changes no accepted-value policy. No edition-aware
rule may require 106-17 before acknowledging the printed year-format text.
106-24R1's January 2025 publication does not make G\106=25 a recognized
edition year. The archived-edition year set remains INT-014's source.

**Reason**: Older files may use forms the standard never defined; reading them would be a guess.

**Design**: accepted (owner, 2026-09-26) · **Review**: initial independent check CONCERNS (narrow source correction, 2026-11-27) at main `c682e05`; correction applied; recheck CONCUR at #81 head `699029b9e739f7bef7f27e7f67d8cac23c20d90d` (2026-11-27); [batch evidence](research/2026-11-27-legacy-int-independent-check-batch2.md) · **Origin**: T6

**Test**: `g106_other_forms_are_unrecognised_and_preserved`

**Development**: intensive testing and deep analysis required · **Analysis**: not yet written

### INT-016

**Title**: What the setup record's version byte declares, and when it did not exist

**Sources**:
- 106-05 Chapter 10, computer-generated Format 1 CSDW: "Reserved. (Bits 31-0) are reserved."
- 106-07 Chapter 10: "IRIG-106 Chapter 10 Version (CH10VER). (1 Byte) indicates which IRIG-106 Chapter 10 release version the recorder requirements and following recorded data are applicable to and comply with." "0x00 thru 06 = Reserved", "0x07 = IRIG-106-07", "0x08 thru 0xFF = Reserved".
- 106-24R1 Chapter 11: "RCC 106 Version (RCCVER). Bits 7-0 specify which RCC release version applies and to which the following recorded data complies with." "0x0E = RCC 106-22", "0x0F through 0xFF = Reserved".

**Behaviour**: The byte is reported as the **recording-format version declared**, never as the TMATS edition. A value from `0x00` to `0x06` is reported as "not declared (the field did not exist before 106-07, or is reserved)", not as an error; defined codes are read from the newest table (106-24R1), since later editions only added codes.

Historical-layout note from the independent check: 106-07 also introduced
SRCC at bit 8, with bits 31–9 reserved; keep that edition-specific layout
separate from the later FRMT field and the declared recording-version code.
The maintainer CSDW review must verify each source/layout, not infer from
RCCVER alone.

**Reason**: The field describes the recorder and recorded data, and 106-04 and 106-05 recordings carry no version at all.

**Design**: accepted (owner, 2026-09-26) · **Review**: independent rule check CONCUR (2026-11-27) at main `c682e05`; existing owner acceptance and development/analysis/test gates retained; [batch evidence](research/2026-11-27-legacy-int-independent-check-batch2.md) · **Origin**: T6

**Test**: `zero_csdw_version_is_not_declared`

**Development**: intensive testing and deep analysis required · **Analysis**: not yet written

### INT-017

**Title**: The fallback edition taken from RCCVER

**Sources**:
- 106-24R1 Chapter 11 RCCVER codes: "0x07 = RCC 106-07" … "0x0D = RCC 106-19", "0x0E = RCC 106-22".
- Owner decision (2026-09-26): when `G\106` is missing or unrecognised and there is no override, use a labelled fallback (ADR-0028).

**Behaviour**: Codes `0x07` to `0x0C` select the intended rules basis named by each code. For `0x0D` ("RCC 106-19") the rules of 106-20 apply, and for `0x0E` ("RCC 106-22") those of 106-24R1, each as a labelled **policy**: "the newest archived edition that still uses this code" (INT-012; owner, 2026-09-27, "Yes, use 106-20"). With no usable RCCVER, the baseline applies. The report says "fallback" and gives the reason. *(Revised 2026-09-27, ADR-0032: the policy is now named as one; the effect for `0x0E` is unchanged.)*

Until edition deltas exist (0.4), the applied registry vocabulary is the
106-24R1 baseline, while the labelled basis records the policy's intended
edition. Expose that availability limitation; never claim actual 106-13
rules were applied when only baseline definitions exist. This follows the
INT-062 baseline clarification without approving its proposed X vocabulary
options. No new RCCVER mapping, renderer or fallback reason is selected.

**Reason**: Applying the newest edition that still uses a code avoids reporting attributes introduced by that edition as unknown in recordings that could only declare the older code; the label keeps the choice visible as policy, not as something RCCVER says.

**Design**: accepted (owner, 2026-09-26) · **Review**: independent rule check CONCUR (2026-11-27) at main `c682e05`; existing owner acceptance and development/analysis/test gates retained; [batch evidence](research/2026-11-27-legacy-int-independent-check-batch2.md) · **Origin**: T6

**Test**: `missing_g106_falls_back_to_rccver_edition_labelled`

**Development**: intensive testing and deep analysis required · **Analysis**: not yet written

## Edits and the checksum

### INT-018

**Title**: `G\SHA` is recognised in any letter case, and never inside a value

**Sources**:
- §9.4.2: "For alphanumeric data items, including keywords, either upper or lower case is allowed; TMATS is not case sensitive."
- Chapter 6 §6.2.3.11 f: "If the TMATS includes a G\SHA code name, all text between the "G\SHA" and the following semicolon, inclusive, shall be discarded for the purposes of digest calculation."

**Behaviour**: **Reconfirmed by Joey, 2026-11-23, under ADR-0033.** The excluded range starts at a `G\SHA` **code name** — the start of an attribute, in any letter case (`g\sha`, `G\sha`) — and never at the same characters inside another attribute's value, such as a comment.

**Review follow-up notes (2026-11-23)**: Recognise the complete code name
using INT-036/038's projected name, not a substring search. Exclude RAW bytes
from the first contributing name byte (G/g at its retained source location)
through the terminating semicolon inclusive; internal omitted controls and
name-edge trailing blanks inside that interval stay excluded as raw bytes.
Leading layout before that first name byte is hashed. Emit a located finding
when the raw name spelling differs from `G\SHA`; preserve all original bytes.

A compatible damaged name under L3-VAL-012 makes verification indeterminate,
never a repaired key or a positive result. X paths carrying G/SHA and COMMENT
value text are not the code name. If the preceding semicolon is missing,
`COMMENT:x\r\nG\SHA:...;` is still one COMMENT value; a located suspect-
terminator/framing finding is suggestion-only, never a repaired boundary.
A leading encoding marker/BOM blocks semantic identification under INT-037,
so checksum verification is indeterminate rather than guessing the exclusion.

**Reason**: The standard speaks of the code name; `irig106lib`'s substring search treats text in a value as the item (defect D6).

**Design**: approved (Joey, reconfirmed 2026-11-23; originally owner-accepted 2026-09-26) · **Review**: approved under ADR-0033; draft author: maintainer agent; owner reconfirmation: Joey, 2026-11-23; independent AI check: CONCUR on all three in register review at main `30e42a3eac791f2f794e5fe239c59590eb6682b8`, relayed by the owner 2026-11-23; follow-up wording still needs exact-head review · **Origin**: T7

**Test**: `g_sha_code_name_in_any_case_and_not_in_values`, `projected_sha_name_excludes_raw_span`, `missing_previous_semicolon_does_not_create_sha`, `x_and_comment_sha_text_is_not_item`, `leading_bom_blocks_sha_verification` (planned; not yet written)

**Development**: intensive testing and deep analysis required · **Analysis**: not yet written

**Reconfirmation and AI-check reference**: [combined owner approval record](research/2026-11-23-int021-int030-owner-signoff.md); the register review at main `30e42a3eac791f2f794e5fe239c59590eb6682b8` is the AI-check reference. Analysis and executable tests remain outstanding.

### INT-019

**Title**: A `G\SHA` item with no following semicolon

**Sources**:
- Table 9-2: "except the characters from "G\SHA:" to the following ";" (inclusive)".
- Chapter 6 §6.2.3.11 f: "all text between the "G\SHA" and the following semicolon, inclusive".

**Behaviour**: **Reconfirmed by Joey, 2026-11-23, under ADR-0033.** An error. Verification reports "malformed" and never "match"; a suggested edit adds the terminator. No digest is presented as the document's checksum.

**Review follow-up notes (2026-11-23)**: The missing following semicolon
is ordinarily an EOF case. Mid-document, a missing semicolon before another
item lets the first semicolon terminate one enlarged value; validate that
whole projected value's shape, report malformed, and never insert a boundary
or terminator automatically. A known truncated setup record is also malformed
for authoritative checksum operations, even if a retained SHA item terminates.
The refusal applies to COMPUTE as well: refuse an authoritative TMATS checksum
when truncation or an undefined exclusion makes its byte domain unavailable.
A caller's separately labelled raw-byte hash is not the document checksum.
A complete, terminated item with a malformed digest value alone does not
make that raw exclusion undefined: e.g. 2-ABC or 63 hex digits gives VERIFY
malformed but COMPUTE allowed on complete known input. Refuse COMPUTE only
for undefined/ambiguous domain, including swallowed-attribute evidence,
not merely a bad value shape. This general rule also covers the placeholder.

Keep F1 duplicate findings alongside malformed/framing findings, never drop
one to report successful verification. Suggested repairs remain unapplied.

**Reason**: Without a following semicolon the excluded range is undefined, so any digest would rest on a guess.

**Design**: approved (Joey, reconfirmed 2026-11-23; originally owner-accepted 2026-09-26) · **Review**: approved under ADR-0033; draft author: maintainer agent; owner reconfirmation: Joey, 2026-11-23; independent AI check: CONCUR on all three in register review at main `30e42a3eac791f2f794e5fe239c59590eb6682b8`, relayed by the owner 2026-11-23; follow-up wording still needs exact-head review · **Origin**: T7

**Test**: `unterminated_g_sha_is_malformed`, `eof_sha_refuses_authoritative_compute`, `mid_document_missing_terminator_fails_value_shape`, `truncated_setup_is_not_authoritative_checksum`, `malformed_sha_keeps_duplicate_finding`, `complete_bad_digest_value_allows_compute`, `defined_exclusion_with_unusable_value_still_computes` (planned; not yet written)

**Development**: intensive testing and deep analysis required · **Analysis**: not yet written

**Reconfirmation and AI-check reference**: [combined owner approval record](research/2026-11-23-int021-int030-owner-signoff.md); the register review at main `30e42a3eac791f2f794e5fe239c59590eb6682b8` is the AI-check reference. Analysis and executable tests remain outstanding.

### INT-020

**Title**: Upper-case hexadecimal in a `G\SHA` value

**Sources**:
- Chapter 6 §6.2.3.11 f: "The message digest is a string of 64 lower-case hexadecimal characters, prefixed with the constant string "2-"".
- Table 9-2: "SHA2-256 shall be represented as "2-" followed by 64 hex characters." "Range: integer followed by "-" followed by hex characters".

**Behaviour**: **Reconfirmed by Joey, 2026-11-23, under ADR-0033.** A value whose digits match the computed digest except for letter case is reported as a match with a warning that Chapter 6 requires lower case. A stamp always writes lower case.

**Review follow-up notes (2026-11-23)**: CR/LF within the checksum value
uses INT-036 projection; otherwise matching digits yield match plus the
located omission information (and the upper-case warning where applicable).
Hash raw bytes outside the exclusion, never the projected document. Blanks
within a value are unavailable until the value-blank rule is reviewed; do
not trim, declare an automatic mismatch or report Invalid from that missing
rule (INT-041).

`G\SHA:0;` is explicitly reported unverified with reason placeholder/no
embedded digest, never match or mismatch. Its complete exclusion range still
allows COMPUTE of a checksum independently of the placeholder's value;
stamping may explicitly replace it and must verify the final output. This
sets a reported status/reason, not a concrete Rust enum. Source: 106-24R1
Chapter 6 section 6.2.3.11 f, pages 6-22–6-23, including the placeholder example.
`02-` spelling is a separate decision in approved INT-043 (Option B), not
covered by this reconfirmation. Fixture digests use independent Python hashlib over
exact bytes, not Chapter 6's example digest, reported unreproducible by the
reviewer. No claim of reproducing that printed digest is made here.

**Reason**: Table 9-2 allows hex characters without a case; Chapter 6 fixes lower case; §9.4.2 makes letter case insignificant.

**Design**: approved (Joey, reconfirmed 2026-11-23; originally owner-accepted 2026-09-26) · **Review**: approved under ADR-0033; draft author: maintainer agent; owner reconfirmation: Joey, 2026-11-23; independent AI check: CONCUR on all three in register review at main `30e42a3eac791f2f794e5fe239c59590eb6682b8`, relayed by the owner 2026-11-23; follow-up wording still needs exact-head review · **Origin**: T7

**Test**: `upper_case_g_sha_matches_with_warning`, `checksum_value_controls_keep_omission_information`, `checksum_value_blanks_are_unavailable`, `sha_zero_placeholder_is_unverified`, `checksum_fixture_uses_independent_digest` (planned; not yet written)

**Development**: intensive testing and deep analysis required · **Analysis**: not yet written

**Reconfirmation and AI-check reference**: [combined owner approval record](research/2026-11-23-int021-int030-owner-signoff.md); the register review at main `30e42a3eac791f2f794e5fe239c59590eb6682b8` is the AI-check reference. Analysis and executable tests remain outstanding.

### INT-021

**Title**: Two or more `G\SHA` items in one document

**Sources**:
- Table 9-2 and Chapter 6 §6.2.3.11 f (quoted in INT-018 and INT-019) speak of one item: "If the TMATS includes a G\SHA code name".
- Table 9-2: the code name `G\SHA` carries no index.

**Behaviour**: **Approved by Joey, 2026-11-23, under ADR-0033; originally coordinator-decided F1, 2026-11-21.** Reject duplicate SHA items as indeterminate; list every occurrence, preserve every byte, never claim positive verification, and refuse stamping until the input is fixed (ADR-0034).

**F1 edge cases (review follow-up, 2026-11-23)**: A damaged name compatible
with G\SHA under L3-VAL-012, such as `G\SH\xFFA`, also makes verification
indeterminate, including alongside one exact SHA item: the exact item's
presence does not prove no further damaged SHA occurrence exists. Keep all
raw evidence, never manufacture an exact key or a positive verification.
INT-019's exact unterminated G\SHA remains malformed, not silently promoted
to an exclusion with a guessed end.

Refusing to stamp does not forbid the ADR-0029 repair transaction: the caller
may remove a duplicate by naming one exact item ID bound to the document
revision. No first/last winner or implicit deletion is chosen. Apply the
explicit edit, rebuild/re-read the result, and stamp only after the resulting
input has no blocking duplicate/damaged-SHA ambiguity. A stamp may be the
last step of that transaction, never run over the still-ambiguous input.

**Implementation gate**: Reader/validator behaviour tied to this register entry is subject to L3-REG-009/012 even when hand-written rather than generated. Joey's 2026-11-23 owner sign-off and the independent AI CONCUR at PR #30 head `13173b0` complete the semantic approval/review gate. Source freshness, completed development analysis and executable verification remain additional implementation gates; approval alone does not authorize skipping them.

**Reason**: The standard does not say which item, if any, holds the checksum, or which text is excluded, when there are several.

**Design**: approved (Joey, owner sign-off, 2026-11-23; F1 originally coordinator-decided 2026-11-21) · **Review**: approved under ADR-0033; draft author: maintainer agent; owner sign-off: Joey, 2026-11-23; independent AI check: CONCUR, relayed by the owner 2026-11-23 for PR #30 head `13173b04faac989fa581b8e9ebbd0df6c80d6b98`; source freshness, analysis and verification gates remain · **Origin**: T7

**Test**: `duplicate_g_sha_items`, `damaged_sha_name_is_indeterminate`, `exact_duplicate_removal_precedes_stamping` (planned; not yet written)

**Development**: intensive testing and deep analysis required · **Analysis**: not yet written

**Sign-off and AI-check reference**: [owner approval record](research/2026-11-23-int021-int030-owner-signoff.md); independent AI concurrence is tied to PR #30 head `13173b04faac989fa581b8e9ebbd0df6c80d6b98`, not claimed for later amended wording. Approval is not implementation or completed development evidence.

### INT-022

**Title**: Removing an attribute that X extensions point to

**Owner sign-off**: Joey, 2026-11-28, under ADR-0033; [verbatim decision](research/2026-11-28-interpretation-owner-signoffs.md#int-022).

**Earlier options packet**: [plain-English options for INT-022–034](research/2026-11-27-int022-034-owner-brief.md); superseded for the selected choices by the sign-off below.

**Sources**:
- §9.5.14: "If the file is being edited by a TMATS editor, it would notice the association and preserve it even if the editor doesn't know what the code means. Thus if the measurements were re-numbered and the index was 1-5 instead of 1-2, the extension code could be updated to preserve the link."
- §9.5.14: "The "x" values must match the index of the original code word therefore no new values may be added."

**Behaviour**: **Owner-signed A (F2):** refuse deletion while extensions attach or may attach with unresolved/ambiguous/cannot-evaluate evidence; list all attached or possibly attached X items. A cascade requires a separate explicit edit; scope remains 0.5.

**Options considered — A selected, 0.5 scope (not a 0.1 blocker)**:
- A (selected default): refuse removal while reviewed evidence attaches X
  items OR their attachment is unresolved/ambiguous/cannot-evaluate (including
  INT-061 A for G\PN), listing all attached/possibly attached X items. Do not silently override; a caller must
  explicitly choose a separately reviewed orphan-preserving or cascade plan.
- B: allow explicit original removal while preserving every X in output,
  reporting each now-orphaned extension and its former target evidence.
- C: cascade-remove attached X only when the caller explicitly requests and
  reviews that deletion set; never silently cascade.
All use reviewed INT-061/062 attachment evidence, exact revision-bound targets
and ADR-0029 transactions. Unresolved/ambiguous attachment never authorizes a
guessed cascade or a claim that no X is attached. Preserve the original
revision; any removal is explicit new-output editing. A minimizes accidental
orphaning/deletion; B helps intentional original-only edits but leaves orphans;
C keeps output internally consistent at the cost of explicitly deleting X.

**Reason**: The standard covers renumbering the original, not removing it.

**Design**: approved by owner (Joey, 2026-11-28) · **Review**: complete under ADR-0033 for the signed parts; draft author: maintainer agent; owner sign-off: Joey, 2026-11-28; independent AI FINAL CONCUR at [PR #87 head `e0a0ccea5a61c64f7de9ca0e122adb01f376a3f9`](https://github.com/TelemetryWorks/irig106-tmats/pull/87/commits/e0a0ccea5a61c64f7de9ca0e122adb01f376a3f9), reviewer @thread:thr_qsvwmtnn8f, received 2026-11-28; [final-check record](research/2026-11-28-int022-066-final-concurrences.md); source freshness, written development analysis/review and intensive executable tests remain required · **Origin**: T7

**Test**: `removing_an_extended_attribute` (planned; refusal and separately explicit cascade are now owner-decided)

**Development**: intensive testing and deep analysis required · **Analysis**: not yet written

### INT-023

**Title**: `=` typed where `:` belongs, as in Chapter 6's own example

**Owner sign-off**: Joey, 2026-11-28, under ADR-0033; [verbatim decision](research/2026-11-28-interpretation-owner-signoffs.md#int-023).

**Sources**:
- Chapter 6 §6.2.3.11 (106-24R1), `.TMATS WRITE` and `.TMATS READ` examples: "G\DSI\N=18;".
- §9.4.2: attributes are written "CODE:value;".

**Behaviour**: **Owner-signed A (F3).** Reading follows §9.4.2 regardless: with no colon the item is reported as a missing delimiter (L1-READ-003) and kept exactly. Suggest replacing `=` by `:` only under the complete-recognized-name/no-colon guard below; never apply automatically.

**Options considered (A selected, 2026-11-28)**: A (reviewer-recommended) offer a suggestion
only when bytes before '=' are a positively recognized COMPLETE code name,
with no colon in the item; preserve raw bytes and never apply automatically,
using the L1-READ-007 guard. B offers no suggestion. The existing reading
rule above is independent of this choice. Source: Chapter 6 p. 6-22.

**Reason**: The standard's own example uses the form, so real files may too; a suggestion could help or could mislead.

**Design**: approved by owner (Joey, 2026-11-28) · **Review**: complete under ADR-0033 for the signed parts; draft author: maintainer agent; owner sign-off: Joey, 2026-11-28; independent AI FINAL CONCUR at [PR #87 head `e0a0ccea5a61c64f7de9ca0e122adb01f376a3f9`](https://github.com/TelemetryWorks/irig106-tmats/pull/87/commits/e0a0ccea5a61c64f7de9ca0e122adb01f376a3f9), reviewer @thread:thr_qsvwmtnn8f, received 2026-11-28; [final-check record](research/2026-11-28-int022-066-final-concurrences.md); source freshness, written development analysis/review and intensive executable tests remain required · **Origin**: T7

**Test**: `chapter_6_example_equals_sign_is_missing_delimiter`

**Development**: intensive testing and deep analysis required · **Analysis**: not yet written

## Channels and packets

### INT-024

**Title**: Which packet data types a channel may carry

**Owner sign-off**: Joey, 2026-11-28, under ADR-0033; [verbatim decision](research/2026-11-28-interpretation-owner-signoffs.md#int-024).

**Sources**:
- Chapter 11 Table 11-4 (106-24R1): data-type families of eight codes (for example "0x08 PCM Data, Format 0" to "0x0B - 0x0F PCM Data, Format 3 - Format 7"), except "0x78 Controller Area Network Bus" (one code) and "0x79 Fibre Channel Data, Format 0", "0x7A Fibre Channel Data, Format 1", "0x7B – 0x80 … Reserved"; no `0x22` is listed.
- Table 9-4: `R-x\CDT-n` enumeration (PCMIN … TMOUT) and one data-type-format attribute per type, for example `R-x\PDTF-n` "PCM data type format. Enumeration equates to format number in Chapter 11"; `R-x\TTF-n`, `R-x\VTF-n`, and others "equates to format number in Chapter 10"; `R-x\UTF-n` and `R-x\FCTF-n` without that sentence.

**Behaviour**: A reviewed table maps each `R-x\CDT-n` keyword to its Table 11-4 family, and the channel's data-type-format value to the format within it (`docs/TMATS-IN-CHAPTER-10.md` section 3.3). TMOUT has no packet data type. The mapping stays owned by tmats; the packet check (L1-CH10-007) is consumed by irig106-validator, per the #60 ownership direction.

The reviewed mapping is the 106-24R1 baseline, not an arithmetic family
formula or universal historical table. Other declared editions retain their
INT-044/017 labelled intended basis and actual baseline-availability limit;
where no reviewed edition/format mapping exists, return cannot-evaluate.
Missing, Invalid, Ambiguous or unavailable same-channel CDT OR data-type-format
value also gives cannot-evaluate with all evidence, never a mismatch finding.
A mismatch requires an independently established usable expected mapping.
Source pages: Table 11-4 begins 11-5; analog 0x20–27 rows are on 11-7;
Table 9-4 PDTF 9-50, UTF 9-77 and FCTF 9-82.

**Options considered (owner selected A, 2026-11-28)**: A (recommended) the reviewed, edition-qualified table
with explicit cannot-evaluate for unusable operands or unavailable mappings;
B expose only observed CDT/format/packet codes and defer expected-type
comparison until each mapping is approved. A enables a qualified check but
requires maintained per-edition evidence; B avoids unsupported comparisons
but offers no determinate mismatch answer. Neither may compute the irregular
code families or turn uncertainty into a mismatch.

**Reason**: The families are irregular (CAN, Fibre Channel, the missing `0x22`), and several format attributes still cite Chapter 10 after the packet formats moved to Chapter 11, so the mapping cannot be computed and must be reviewed.

**Design**: approved by owner (Joey, 2026-11-28) · **Review**: complete under ADR-0033 for the signed parts; draft author: maintainer agent; owner sign-off: Joey, 2026-11-28; independent AI FINAL CONCUR at [PR #87 head `e0a0ccea5a61c64f7de9ca0e122adb01f376a3f9`](https://github.com/TelemetryWorks/irig106-tmats/pull/87/commits/e0a0ccea5a61c64f7de9ca0e122adb01f376a3f9), reviewer @thread:thr_qsvwmtnn8f, received 2026-11-28; [final-check record](research/2026-11-28-int022-066-final-concurrences.md); source freshness, written development analysis/review and intensive executable tests remain required · **Origin**: F4 section 3

**Test**: `channel_data_type_maps_to_packet_data_types`

**Development**: intensive testing and deep analysis required · **Analysis**: not yet written

### INT-025

**Title**: `R-x\TDTF-n`'s condition names `TSPIN`, which is not a channel type

**Owner sign-off**: Joey, 2026-11-28, under ADR-0033; [verbatim decision](research/2026-11-28-interpretation-owner-signoffs.md#int-025).

**Sources**:
- Table 9-4, `R-x\TDTF-n`: "Allowed when: R\CDT is "TSPIN"".
- Table 9-4, `R-x\CDT-n` enumeration: "TSPIIN TSPI/CTS Input"; no `TSPIN`.

**Behaviour**: The condition is read as `TSPIIN`, as INT-007 reads `FBCIN` as `FBCHIN`. A document's literal CDT=TSPIN still gets an unknown-keyword finding, never silent repair. Source: TDTF p. 9-81; TSPIIN enumeration p. 9-50.

**Options considered (owner selected A, 2026-11-28)**: A (recommended) read the condition's TSPIN as TSPIIN,
retaining unknown-keyword findings for literal document TSPIN; B leave that
condition unavailable pending formal errata rather than treating it as false.
A makes the intended TSPI field usable; B withholds the affected default/
applicability decision. Neither repairs the document.

**Reason**: Read literally, the TSPI/CTS data type format could never be allowed.

**Design**: approved by owner (Joey, 2026-11-28) · **Review**: owner sign-off recorded under ADR-0033; draft author: maintainer agent; Independent batch 3 CONCUR at `c682e05` (2026-11-27). Source freshness, written development analysis/review and intensive executable tests remain required · **Origin**: F4 section 3

**Test**: `tspi_data_type_format_allowed_for_tspiin`

**Development**: intensive testing and deep analysis required · **Analysis**: not yet written

### INT-026

**Title**: Two recorder polarity conditions name `P-d\CDT`, which does not exist

**Owner sign-off**: Joey, 2026-11-28, under ADR-0033; [verbatim decision](research/2026-11-28-interpretation-owner-signoffs.md#int-026).

**Sources**:
- Table 9-4, `R-x\RPS-n` (recorder polarity setting): "Allowed when: P-d\CDT is "PCMIN"".
- Table 9-4, `R-x\MFF\RPS-n-m` (recorder polarity setting for a filtered minor frame): "Allowed when: When P-d\CDT is "PCMIN"".
- No `CDT` attribute exists in the P group (Figure 9-6); the channel data type is `R-x\CDT-n`, and the PCM data type attributes around these two are "Allowed when: R\CDT is "PCMIN"" (seven of them in 106-24R1).

**Behaviour**: Both conditions are read as `R\CDT` of the same channel.

Source locations: RPS p. 9-51 and MFF RPS p. 9-54; the latter also
contains the doubled printed "When When". This does not define P CDT.

**Options considered (owner selected A, 2026-11-28)**: A (recommended) bind both mistaken P CDT references to
the same recorder/channel R CDT, preserving Missing/Invalid/Ambiguous/
unavailable operand uncertainty; B leave those conditions cannot-evaluate
pending formal errata. A supplies the local meaningful condition; B disables
those determinate applicability answers. Never borrow another channel.

**Reason**: Both attributes are in the R group's PCM block beside attributes conditioned on `R\CDT`; `P-d\CDT` cannot be evaluated.

**Design**: approved by owner (Joey, 2026-11-28) · **Review**: owner sign-off recorded under ADR-0033; draft author: maintainer agent; Independent batch 3 CONCUR at `c682e05` (2026-11-27). Source freshness, written development analysis/review and intensive executable tests remain required · **Origin**: F4 section 3

**Test**: `recorder_polarity_allowed_for_pcm_channels`

**Development**: intensive testing and deep analysis required · **Analysis**: not yet written

### INT-027

**Title**: `C-d\DCN`'s "Links from:" lists `R-x\AMN-n-m` twice

**Owner sign-off**: Joey, 2026-11-28, under ADR-0033; [verbatim decision](research/2026-11-28-interpretation-owner-signoffs.md#int-027).

**Sources**:
- Table 9-11, `C-d\DCN`: "Links from: R-x\AMN-n-m, R-x\AMN-n-m, M-x\SI\MN-n, M-x\BB\MN, D-x\MN-y-n, B-x\UMN1-i, B-x\UMN2-i, B-x\UMN3-i, B-x\MN-i-n-p, S-d\MN-i-n-p, R-x\DMN-n-m, Q-d\MMNM-i-n-m, Q-d\SMMNM-i-n-m-o".

**Behaviour**: The duplicate reference is a no-op. The
reviewer checked the Links-to side: only R AMN and R DMN carry Links to
C-d\DCN in 106-24R1. EV MN, EV PM MN, IPMN, BME SMN and SMF SMN carry no
such link, so this duplicate hides no omitted R source on that side. Retain
INT-003's three-source process; do not invent an additional source from a
similar name. Source: Table 9-11 p. 9-158.

**Options considered (owner selected A, 2026-11-28)**: A (recommended) deduplicate only the definition's repeated
link reference, retaining its source defect and the three-source relationship
check; B leave relationship generation blocked for this definition until
formal errata. A avoids duplicate graph edges without changing document
occurrences; B is stricter but withholds the affected conversion links. No
additional R source is inferred from a similar measurement name.

**Reason**: A repeated source adds nothing, but may hide an omitted one.

**Design**: approved by owner (Joey, 2026-11-28) · **Review**: owner sign-off recorded under ADR-0033; draft author: maintainer agent; Independent batch 3 CONCUR at `c682e05` (2026-11-27). Source freshness, written development analysis/review and intensive executable tests remain required · **Origin**: F4 section 3

**Test**: `measurement_name_sources_for_conversions`

**Development**: intensive testing and deep analysis required · **Analysis**: not yet written

## Configuration over a recording

### INT-028

**Title**: Which setup record governs a packet

**Owner sign-off**: Joey, 2026-11-28, under ADR-0033; [verbatim decision](research/2026-11-28-interpretation-owner-signoffs.md#int-028).

**Sources**:
- Chapter 11 §11.2.7.2: "the new setup record packet will be committed to the stream prior to any new or changed data packets being committed to the stream".
- Chapter 10 §10.5.1: the setup record "as the first packets in the recording"; Table 10-9.
- Chapter 11: "A single setup record may span multiple consecutive packets."

**Behaviour — owner-signed umbrella**: Compose the approved
closure, qualification, comparison and takeover rules, not “latest Complete
wins”. Compare later candidates against the CURRENTLY GOVERNING setup (the
original sealed record or latest copy that validly took control), or retain
an explicitly UNRESOLVED comparison target under INT-053. A Complete label
alone does not establish a valid takeover or its effective packet boundary.

- INT-045 supplies layered boundaries; INT-046 identity exceptions apply only
  in their approved bounded domain, never as a new adoption rule.
- INT-048 field ambiguity blocks new adoption, including equal/digest-matching
  copies; retained cut facets keep Incomplete's full blocking force.
- INT-049 never extends one candidate through a later boundary by identity.
- INT-051 integrity findings alone neither quarantine otherwise eligible
  Complete data by default nor prove the old setup still governs a changed copy.
- INT-052 unreadable SRCC supplies no governing inference. INT-053 keeps an
  unresolved takeover/target unresolved until independently valid takeover.
- Owner-signed INT-031/032 distinguish ungoverned packet ranges from unresolved
  comparison targets; no remembered caller assumption or guessed setup.

The output domain must distinguish an established governing setup,
**ungoverned (no governing setup)** and **unresolved governing/target evidence**,
with the actual packet ranges/provenance retained. The signed answers (a)–(e) below bind the boundary; a separately reviewed
independent takeover/source-partition rule still needs evidence before use.
Never infer adoption from file order, completion, SRCC, integrity or equality alone.
Multiple simultaneous files remain out of scope.

**Options considered (owner selected A, 2026-11-28)**: A selected: the composed umbrella and all five recommendations below
were signed by Joey on 2026-11-28; alternatives remain for context:

(a) **Effective governing boundary:** from which packet does a validly taking-
control setup govern: after its last fragment, after its INT-045 closure
point, or another explicitly defined boundary? Last fragment and time closure
may differ; specify how packets between those points are qualified.
**Owner-selected answer (2026-11-28):** use the evidenced closure/
valid-takeover boundary, including a trusted time packet that seals the record
rather than starting only after that packet. Do not backdate adoption to the
last fragment merely because it arrived earlier; intervening packets retain
their established or unresolved qualification, not an inferred new setup.

(b) **Unresolved comparison target:** can this Complete candidate take
control while INT-053's comparison target is unresolved, and which independent
evidence establishes that valid takeover? INT-053 already allows a later
INDEPENDENTLY ESTABLISHED valid takeover to restore a resolved target;
Complete alone is never that evidence. A separately reviewed rule must supply that proof; this sign-off does not
invent it or guess the old comparison target.
**Owner-selected answer (2026-11-28):** allow restoration only through
a separately reviewed rule establishing independent valid takeover, as
INT-053 permits. Until such evidence is established the target remains
unresolved; Complete, equality, SRCC or a digest alone is insufficient.

(c) **SRCC=1 with unverified F7 event identity:** does the indeterminate event
check block takeover, delay it pending verified evidence, or not by itself
affect eligibility? Preserve approved INT-030's indeterminate finding in all
options; neither a nearby generic event nor absence of verified identification
proves an event occurred or did not occur. Other takeover guards still apply.
**Owner-selected answer (2026-11-28):** unverified F7 identity should
not itself block/delay an otherwise independently established valid takeover.
Retain the indeterminate finding prominently; do not assert the event existed
or waive any other admission, qualification or adoption requirement.

(d) **Distributed-mux / nonzero-channel setup sources before 106-11:** define
per-source governing with independently established source partitions, or
leave that governing case out of scope pending its own reviewed rule? No
channel/source association is guessed. This scope question concerns governing,
not permission to discard or stop parsing legacy raw setup candidates; core
still owns packet admission/legacy-channel evidence.
**Owner-selected answer (2026-11-28):** leave distributed-source
GOVERNING scope unavailable pending a dedicated source-partition rule;
continue retaining/parsing legacy setup candidates and core evidence.
Do not invent which recorder/source/channel governs which data. This is
not a cut to the existing legacy text/payload-reading obligation.

(e) **File order for interleaved runs:** use first contributing fragment or
completion order, and how does that ordering relate to the effective boundary
chosen in (a)? These are distinct concepts: ordering a candidate does not
adopt it. This is the existing [file-order owner item](research/2026-11-27-checksum-commands-and-owner-questions.md#owner-brief-what-file-order-means),
now owner-selected explicitly, not an implicit default.
**Owner-selected answer (2026-11-28):** first contributing fragment
orders the candidate inventory; the separate closure/takeover point in (a)
controls when an established governing setup applies. Completion order is
not a substitute for source order or evidence of adoption.

**B (unchanged)**: defer governing application and expose only candidates/
evidence until those decisions exist. Neither option permits the superseded
proposed “last fragment then latest wins”. Multiple simultaneous files remain
out of scope. Approved INT-045/048/050/051/052/053 constrain every answer.

**Reason**: The standard orders setup records before the data they describe but never states the governing rule outright.

**Design**: approved by owner (Joey, 2026-11-28) · **Review**: complete under ADR-0033 for the signed parts; draft author: maintainer agent; owner sign-off: Joey, 2026-11-28; independent AI FINAL CONCUR at [PR #87 head `e0a0ccea5a61c64f7de9ca0e122adb01f376a3f9`](https://github.com/TelemetryWorks/irig106-tmats/pull/87/commits/e0a0ccea5a61c64f7de9ca0e122adb01f376a3f9), reviewer @thread:thr_qsvwmtnn8f, received 2026-11-28; [final-check record](research/2026-11-28-int022-066-final-concurrences.md); source freshness, written development analysis/review and intensive executable tests remain required · **Origin**: F4 section 5

**Test**: `governing_umbrella_composes_approved_guards`, `governing_complete_is_not_takeover`, `governing_unresolved_is_not_no_setup` (planned; owner choice recorded; final rule check recorded; development-analysis review/test gates apply)

**Development**: intensive testing and deep analysis required · **Analysis**: not yet written

### INT-029

**Title**: Repeated, changed, and inconsistent setup records

**Owner sign-off**: Joey, 2026-11-28, under ADR-0033; [verbatim decision](research/2026-11-28-interpretation-owner-signoffs.md#int-029).

**Sources**:
- Chapter 11 §11.2.7.2: SRCC "indicates if the recorder configuration contained in the previous setup record packet(s) of the current recording session (defined as .RECORD to .STOP) has changed"; "The next setup record packet(s) committed to the stream, if not changed from this new setup record, shall clear the SRCC bit to 0."
- Chapter 9 §9.4.2: "For alphanumeric data items, including keywords, either upper or lower case is allowed; TMATS is not case sensitive"; "Leading, trailing, and embedded blanks are assumed to be intentional"; "Attributes may appear in any order."

**Behaviour — owner-signed findings-only table**: Compare with
INT-053's target, never merely the previous copy. If the target/comparison
is unresolved, preserve that uncertainty. Report INT-050 raw subtype and any
INDEPENDENTLY ESTABLISHED semantic class, not a generic case/order-folded
“same configuration” verdict. Shared equality or digest never cures INT-048
field ambiguity or establishes takeover.

| Comparison evidence | First-fragment SRCC | Finding/evidence only |
|---|---|---|
| Byte-identical | 0 / 1 / unreadable | Retain equality; for 1 retain changed claim versus identity, not a guessed reset; unreadable stays unknown |
| SHA-value-only / layout-whitespace-only / other raw difference | 0 | Report raw subtype and unchanged-claim discrepancy; no automatic semantic equivalence or reset |
| Same raw subtypes | 1 | Report changed claim and raw subtype, no independently unproved semantic-change verdict |
| Same raw subtypes | unreadable | Retain subtype/unknown claim; never infer the bit |
| Independently established semantic difference | 0 / 1 / unreadable | Retain reviewed semantic evidence and supplied claim; do not invent evidence from generic value folding |
| Incomplete comparison / unresolved target | any | Unresolved, all available evidence; never equality, guessed subtype or no-change |

Governing effects defer to approved INT-053 and the owner-signed INT-028 umbrella;
a finding table cannot establish control. No SRCC=0-but-different “still governs
from there” shortcut remains. INT-048 Ambiguous stays blocking regardless of
comparison equality. **Owner answer (2026-11-28):** proven descriptive-only comment/date changes
are not semantic configuration changes; reviewed field semantics must prove
that class, and raw differences remain reported.

**Separate choice — comment-only/date-only changes (A selected)**: A excludes a
change proven to affect only reviewed descriptive comment/date fields from
SEMANTIC configuration change; B counts those changes as configuration
changes too; C makes that classification an explicitly supplied reviewed
per-field policy, with no silent default. **Recommend A**, but only when
reviewed definitions prove the fields are descriptive and no other change
is present. Unknown/private fields stay unresolved, not assumed descriptive.
In every option report the raw change and SRCC claim under INT-050; it is
never byte-identical retransmission or automatic takeover. A does not waive
the owner rule that an SRCC=0 copy differing in raw bytes is checked and
reported as a change. No semantic reset or governing effect is inferred.

**Options considered (owner selected A, 2026-11-28)**: A (recommended) this evidence/findings-only composition,
with semantic classes only when independently reviewed; B defer the table
until the owner defines semantic configuration equivalence. Both retain raw
bytes and approved governance guards; neither authorizes implicit adoption.

**Reason**: SRCC is a claim about configuration; raw differences alone do not define semantic equivalence. The former generic case/order-folding proposal conflicted with approved INT-050/052/053.

**Design**: approved by owner (Joey, 2026-11-28) · **Review**: complete under ADR-0033 for the signed parts; draft author: maintainer agent; owner sign-off: Joey, 2026-11-28; independent AI FINAL CONCUR at [PR #87 head `e0a0ccea5a61c64f7de9ca0e122adb01f376a3f9`](https://github.com/TelemetryWorks/irig106-tmats/pull/87/commits/e0a0ccea5a61c64f7de9ca0e122adb01f376a3f9), reviewer @thread:thr_qsvwmtnn8f, received 2026-11-28; [final-check record](research/2026-11-28-int022-066-final-concurrences.md); source freshness, written development analysis/review and intensive executable tests remain required · **Origin**: F4 section 5; team spec alignment review

**Test**: `setup_record_kinds_and_srcc_findings`, `srcc_raw_subtypes_do_not_infer_semantics`

**Development**: intensive testing and deep analysis required · **Analysis**: not yet written

### INT-030

**Title**: The "setup record configuration change event packet" is not defined

**Sources**:
- Chapter 11 §11.2.7.2: "Prior to the new setup record being committed to the stream, a setup record configuration change event packet shall be inserted into the stream."
- No other passage of Chapter 10, Chapter 11, or Chapter 9 (106-24R1) names the packet type, format, or content of that event packet.

**Behaviour**: **Approved by Joey, 2026-11-23, under ADR-0033; originally coordinator-decided F7, 2026-11-21.** Require verified event identification and report indeterminate meanwhile; never guess a packet type (ADR-0034). This does not identify the event packet or complete L1-CH10-005's check.

**Implementation gate**: Reader/validator behaviour tied to this register entry is subject to L3-REG-009/012 even when hand-written rather than generated. Joey's 2026-11-23 owner sign-off and the independent AI CONCUR at PR #30 head `13173b0` complete the semantic approval/review gate. Source freshness, completed development analysis and executable verification remain additional implementation gates; approval alone does not authorize skipping them.

**Analysis notes (review follow-up, 2026-11-23; not completed analysis)**:
- Report SRCC=1 on the session's first setup record separately; do not merge
  it into, or claim it is settled by, the event-identification result.
- Preserve fragments' individual SRCC values and disagreement. ADR-0025's
  CSDW agreement check covers FRMT/RCCVER, not SRCC; do not infer SRCC
  agreement from successful assembly or silently choose one fragment's bit.
  A summary/validation rule needs its own reviewed analysis.
- Show a nearby Format 2 recording-event packet as a candidate only, never
  as satisfying the configuration-change event rule without verified identity.
- Once the packet is identified, what prior means needs its own cited,
  reviewed interpretation (ordering/scope); proximity alone chooses none.

**Reason**: Checking for a packet the standard does not identify would rest on a guess (for example, that it is a Format 2 recording event packet).

**Design**: approved (Joey, owner sign-off, 2026-11-23; F7 originally coordinator-decided 2026-11-21) · **Review**: approved under ADR-0033; draft author: maintainer agent; owner sign-off: Joey, 2026-11-23; independent AI check: CONCUR, relayed by the owner 2026-11-23 for PR #30 head `13173b04faac989fa581b8e9ebbd0df6c80d6b98`; source freshness, analysis and verification gates remain · **Origin**: F4 section 5

**Test**: `configuration_change_event_packet` (planned; event identification still unverified)

**Development**: intensive testing and deep analysis required · **Analysis**: not yet written

**Sign-off and AI-check reference**: [owner approval record](research/2026-11-23-int021-int030-owner-signoff.md); independent AI concurrence is tied to PR #30 head `13173b04faac989fa581b8e9ebbd0df6c80d6b98`, not claimed for later amended wording. Approval is not implementation or completed development evidence.

### INT-031

**Title**: After a setup record that cannot be assembled, nothing governs

**Owner sign-off**: Joey, 2026-11-28, under ADR-0033; [verbatim decision](research/2026-11-28-interpretation-owner-signoffs.md#int-031).

**Sources**:
- Chapter 11 §11.2.7.2: "A single setup record may span multiple consecutive packets. When spanning multiple packets, the sequence counter shall increment in the order of segmentation of the setup record, n+1"; "the new setup record packet will be committed to the stream prior to any new or changed data packets".
- The standard does not say what applies after a record that cannot be read.

**Behaviour**: A setup record INT-034 finds incomplete or ambiguous is reported and builds no description by default (the owner-approved ambiguity-only opt-in must be an explicit per-query caller choice, labelled; no remembered/session default); the previous description is not carried forward, and packets after it are ungoverned until a later setup validly takes control under INT-028/053, not merely the next Complete candidate. The fragment bytes are kept.

**INT-048 limit on the acceptance option (2026-11-25)**: A retained
cut facet keeps Incomplete's full blocking force even if the aggregate
outcome is Ambiguous. The labelled ambiguous-split option above cannot make cut text usable or governing.

“Ungoverned” here means no governing setup, distinct from INT-053's
unresolved comparison target. Integrity failure on otherwise eligible COMPLETE
text uses INT-051's prominent-finding default, not this incomplete/ambiguous
cannot-govern predicate. Composes the owner-signed INT-028 umbrella and its separate takeover-proof gate.

**Options considered (owner selected A, 2026-11-28)**: A (recommended) the conservative default above plus an
explicit per-query, labelled ambiguity-only opt-in; B always refuse semantic
use of ambiguous as well as incomplete records, with no ambiguity opt-in.
A permits a deliberate caller interpretation while preserving evidence; B
requires the input ambiguity to be fixed first. Both forbid cut-facet use,
implicit old-setup carry-forward and remembered assumptions; both defer actual
takeover to INT-028/053. Raw extraction remains separately authorized.

**Reason**: A new setup record means the configuration may have changed; applying the old description to data the new one describes would decode it wrongly without a word.

**Design**: approved by owner (Joey, 2026-11-28) · **Review**: owner sign-off recorded under ADR-0033; draft author: maintainer agent; Independent batch 4 CONCUR with A at `283ffd5` (2026-11-27); per-query/cut alignment reviewed in #83. Source freshness, written development analysis/review and intensive executable tests remain required · **Origin**: F4 section 6

**Test**: `nothing_governs_after_an_incomplete_setup_record`

**Development**: intensive testing and deep analysis required · **Analysis**: not yet written

### INT-032

**Title**: Packets that no setup record governs

**Owner sign-off**: Joey, 2026-11-28, under ADR-0033; [verbatim decision](research/2026-11-28-interpretation-owner-signoffs.md#int-032).

**Sources**:
- Chapter 10 §10.5.1 and Table 10-9: the setup record is "the first packets in the recording".
- Chapter 11 section 11.2.3 p. 11-24: "Only static Computer-Generated Data, Format 1 packets may precede the first time data packet."

**Behaviour**: Packets before the first complete setup record, or after one that cannot govern (INT-031, an XML record), are reported with the range affected and are not decoded by default. A caller may assign a description to them explicitly PER QUERY (INT-060 style), with no remembered/session-wide default; every report then says the description was assumed, not declared.

XML remains unsupported for tmats 0.1 under ADR-0015. Explicit caller
assignment does not silently declare an in-file setup or settle INT-028.

**Options considered (owner selected A, 2026-11-28)**: A (recommended) expose ungoverned ranges and permit an
explicit labelled caller-assigned description on each query; B expose those
ranges but offer no assumed decoding until a declared setup is available.
A supports deliberate recovery without calling an assumption declared; B
withholds recovery decoding. Neither silently uses the nearest setup or a
session-wide default.

**Reason**: The standard forbids such packets but real recordings may contain them; decoding them silently with a guessed configuration would hide the problem.

**Design**: approved by owner (Joey, 2026-11-28) · **Review**: owner sign-off recorded under ADR-0033; draft author: maintainer agent; Independent batch 4 CONCUR at `283ffd5` (2026-11-27); per-query alignment reviewed in #83. Source freshness, written development analysis/review and intensive executable tests remain required · **Origin**: F4 section 6

**Test**: `ungoverned_packets_are_reported_and_assumptions_labelled`

**Development**: intensive testing and deep analysis required · **Analysis**: not yet written

## Measurements

### INT-033

**Title**: Fragment positions that default to the same value

**Owner sign-off**: Joey, 2026-11-28, under ADR-0033; [verbatim decision](research/2026-11-28-interpretation-owner-signoffs.md#int-033).

**Sources**:
- Table 9-7, `D-x\WFP-y-n-m-e`: "A number from 1 to N specifying the position of this fragment within the reconstructed binary data word. 1 corresponds to the most significant fragment. Each fragment position from 1 to N must be specified only once." "Default: 1".
- Appendix 9-C (106-24R1): `D-3\MNF\N-1-1-1:2;` with fragments at `D-3\WP-1-1-1-1:113;` and `D-3\WP-1-1-1-2:121;` and no `D-3\WFP-…`; Table C-2's prose: "the 10 msbs indicated as M and the 6 lsbs as L".

**Behaviour**: Each absent position takes its default, as defaulted values (ADR-0023). When two or more fragments of one location end up with the same position, validation reports it and the fragment order is ambiguous: the library does not infer the order from the fragment index, the word order, or prose. A caller may supply the order explicitly; every report then labels it as an assumption.

The cited Appendix 9-C example itself trips this duplicate-default-position
finding; fixtures must say so, not assume standard examples are conforming.

**Options considered (owner selected A, 2026-11-28)**: A (recommended) expose the default collision and ambiguous
fragment order, allowing an explicit labelled per-query caller order; B require
explicit source positions before semantic reconstruction, with no caller-order
option. A supports deliberate interpretation of the defective example; B
requires corrected input. Both keep defaults distinct from explicit values
and never infer order from index, word order or explanatory prose.

**Reason**: The default makes a measurement with several fragments invalid unless every position is given; inferring an order would be a guess that decodes wrongly when it is wrong.

**Design**: approved by owner (Joey, 2026-11-28) · **Review**: owner sign-off recorded under ADR-0033; draft author: maintainer agent; Independent batch 4 CONCUR at `283ffd5` (2026-11-27). Source freshness, written development analysis/review and intensive executable tests remain required · **Origin**: F4 section 7

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

**Development**: intensive testing and deep analysis required · **Analysis**: [written development analysis](research/2026-11-28-framing-character-development-analysis.md) (owner/independent analysis review and intensive tests pending)

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

**Numeric-value amendment (Joey, 2026-11-25)**: Owner-signed INT-055 is an
explicit exception for decimal-integer values/numeric codes: check RAW bytes,
so controls make them Invalid and cannot be omitted into a usable number.
Retain independent findings and unchanged raw bytes. Code-name/general text
projection remains unchanged. INT-055 has final independent CONCUR at #51 ebd5c98; development gates remain.

**Reason**: Chapter 9 requires nonprintable discard for meaning and intentional blanks, while product policy retains bytes. The precise set and separate raw/projected operations need a reviewable interpretation rather than locale/Unicode whitespace heuristics.

**Design**: approved (Joey, owner sign-off, 2026-11-22) · **Review**: approved under ADR-0033; draft author: maintainer agent; owner sign-off: Joey, 2026-11-22; independent AI recheck: CONCURRED on PR #16 at `891f77f`, relayed by the owner 2026-11-22 · **Origin**: family 2 batch 3; review amendments requested 2026-11-21

**Test**: `control_projection_preserves_raw_spans`, `nul_fields_have_one_information_finding_each`, `code_name_and_value_blanks_differ`, `character_classes_keep_int035_run_policy` (planned, not yet written)

**Development**: intensive testing and deep analysis required · **Analysis**: [written development analysis](research/2026-11-28-framing-character-development-analysis.md) (owner/independent analysis review and intensive tests pending)

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

**Development**: intensive testing and deep analysis required · **Analysis**: [written development analysis](research/2026-11-28-framing-character-development-analysis.md) (owner/independent analysis review and intensive tests pending)

**Draft analysis**: [baseline draft and 2026-11-21 review amendments](research/2026-11-21-character-interpretation-drafts.md#review-amendments--2026-11-21)

**Sign-off and AI-check reference**: [owner approval record](research/2026-11-22-int036-int037-owner-signoff.md), referencing [PR #16](https://github.com/TelemetryWorks/irig106-tmats/pull/16) at `891f77f137c9e90ecc15416a84b11d67b6bbde86` and the owner's 2026-11-22 relay of the independent recheck. Approval records the interpretation decision, not implementation, completed analysis or executable tests; those development gates remain outstanding.


## Decimal index identity (approved, 2026-11-22)

### INT-038

**Title**: Decimal index identity without source normalization or machine-word limits

**Sources**:
- 106-24R1 Chapter 9 section 9.4.2, page 9-2: "The number appears in the code name preceded by a hyphen." The G\DSI examples instantiate decimal indices.
- Section 9.5.1 a after Table 9-1, page 9-5, describes lower-case index placeholders and the 1-to-counter rule. Section 9.5.14, page 9-172, permits noncontiguous X occurrence indices and requires its occurrence/carried original indices to match the original code.
- Table 9-7 MEASUREMENT NAME `D-x\MN-y-n`, page 9-115, supplies two entry-index positions; NUMBER OF FRAGMENTS `D-x\MNF\N-y-n-m`, page 9-117, supplies a nested path. The text does not define leading-zero equivalence or a parser integer width.
- ADR-0002/0003 require raw retention; ADR-0026 separates scoped counter validation from source parsing. INT-036/037 define approved character handling, not numeric index identity.

**Behaviour**: **Approved by Joey, 2026-11-22, under ADR-0033.** Read index digits from the approved INT-036 code-name projection. At an index position in a standard code pattern, accept a nonempty sequence of ASCII decimal digits for structural reading. Preserve raw spelling and source correspondence. Strip leading zeros for the canonical decimal string, retaining exactly `0` for an all-zero token. Identity equality and hashing use that string, never fixed-width or big-integer conversion. Do not treat signs, fractions, exponents or definition placeholder letters as file indices. This does not prescribe the grammar of arbitrary H/V/X attribute strings.

Each projected digit token different from its canonical string emits a
located **information** finding on its original token span: both `01` and
`00` are noncanonical, while `1` and `0` are canonical. This is visible
interpretation, not a source edit. Report normalization-only full-key
collisions **now**: a canonical full-key group triggers the located
information finding when it contains two or more DISTINCT raw index-spelling
tuples. Compare original-source spellings of corresponding index tokens;
list EVERY occurrence in the group, exact repeats included, in input order.
A group with only one raw index-spelling tuple does not trigger this finding;
code-letter case or value differences alone do not change that tuple. Keep
all items; do not choose a winner. This group trigger/list clarification was
requested with the owner's 2026-11-22 sign-off. The later duplicate-
classification batch still defines single-entry/value-conflict behavior; it must not postpone this visibility.

An omitted control between index digits still triggers INT-036's internal
code-name omission finding: `R-1\r\n0\TK1-1` has occurrence identity `10`,
with the omitted CR/LF located. Never convert TAB to a space. Ordinary blanks
inside/adjacent to an index within the name (`R- 1\` or `R-1 \`) are not
trimmed into a valid index. INT-036's whole-name edge layout rule is unchanged.

When section 9.5.14's X occurrence and carried indices for the embedded
original path are compared with the original's indices, use the same
canonical strings. Thus `X-1\MYORG\D\MN\SGAIN-1-2` and original
`D-01\MN-01-02` agree on those index positions; extra extension-only indices
keep their own positions. This is index equality, not proof of resolved
association or uniqueness; X and D are not the same full key.

A zero index remains structurally readable, but is not made conforming: apply
cited range/counter rules separately. Do not impose a u32/u64/usize ceiling
on retained decimal identity; any bounded numeric accessor must report failed
conversion rather than wrap, truncate or saturate. Never allocate an array
up to the supplied index just to represent it. Preserve order and every
item when `01` and `1` produce the same key: report the normalization-only
collision now and pass all candidates to later duplicate/ambiguity rules.
The `\N` count and no-missing-values checks must count **distinct numeric
identities**, not spellings or occurrences; derivation and implementation of
those scoped checks are explicitly deferred to the counter batch.

**Reason**: Treating indices as numbers follows the cited examples while preserving questionable inputs for diagnostics. Rejecting zero during lexical reading confuses range validation with structure; a machine-size limit makes the same file parse differently across targets. Leading-zero equality is an owner-approved product interpretation, not an explicit Chapter 9 spelling rule. Its cost is that previously distinct-looking names can become duplicate candidates; Joey approved that consequence on 2026-11-22.

**Design**: approved (Joey, owner sign-off, 2026-11-22) · **Review**: approved under ADR-0033; draft author: maintainer agent; owner sign-off: Joey, 2026-11-22; independent AI recheck: CONCURS on PR #20 at `29707da`, relayed by the owner 2026-11-22 · **Origin**: family 2 batch 4; review amendments requested 2026-11-22

**Test**: `decimal_index_spelling_preserves_numeric_identity`, `noncanonical_index_is_visible`, `normalization_only_collision_is_reported_now`, `collision_group_includes_exact_repeats`, `identical_index_spellings_do_not_trigger_normalization_collision`, `projected_index_controls_keep_locations`, `canonical_string_equality_and_hashing`, `x_original_indices_use_numeric_identity`, `zero_index_is_not_range_approval`, `large_index_does_not_wrap_or_allocate_by_value` (planned, not yet written)

**Development**: intensive testing and deep analysis required · **Analysis**: [written development analysis](research/2026-11-28-index-duplicate-counter-development-analysis.md) (owner/independent analysis review and intensive tests pending)

**Draft analysis**: [sources, alternatives and 2026-11-22 review amendments](research/2026-11-22-code-name-index-draft.md#review-amendments--2026-11-22)

**Sign-off and AI-check reference**: [owner approval record](research/2026-11-22-int038-owner-signoff.md), referencing [PR #20](https://github.com/TelemetryWorks/irig106-tmats/pull/20) at `29707da26fd43ec9a871d41bfd6213afdfa3f965` and the owner's 2026-11-22 relay of the independent recheck. The record also identifies the collision-group clarification requested with sign-off. Approval records the interpretation decision, not implementation, completed analysis or executable tests; those development gates remain outstanding.


## Scalar candidate multiplicity (approved, 2026-11-22)

### INT-039

**Title**: Repeated scalar slots remain ambiguous even when values agree

**Sources**:
- 106-24R1 Chapter 9 section 9.4.2, page 9-2: "Each attribute appears in the file as a unique code name"; the same section distinguishes single-entry data from numbered multiple entries and permits arbitrary attribute order.
- Section 9.4.2, page 9-3, introduces standalone COMMENT for interjected comments but does not specify a repeated-comment cardinality or duplicate recovery algorithm.
- Table 9-2 PROGRAM NAME `G\PN`, page 9-10, gives a scalar example. A reviewed definition and its edition must establish a slot's cardinality; absence of a suffix alone is not evidence.
- ADR-0002 preserves every item; ADR-0023 defines ambiguous effective values. Approved INT-038 defines canonical grouping and separate normalization/collision information; coordinator-decided F1 in ADR-0034 already rejects duplicate SHA integrity as indeterminate.

**Behaviour**: **Approved by Joey, 2026-11-22, under ADR-0033.** For a definition-bound scalar slot, group by complete canonical code-name key within one document. Two or more occurrences give a located duplicate finding listing every original item and an ambiguous singular effective value. Keep identical repeats as separate candidates, even if raw bytes or independently interpreted values agree. Never select the first/last, discard an invalid candidate to keep a valid one, coalesce duplicates or supply a default instead. Keep candidate-specific findings visible. Severity policy does not select values; its detailed mapping is deferred.

A numbered multiple-entry pattern supplies distinct slots for distinct full
index combinations; repeating the same slot still gives multiplicity.
Unknown/unreviewed cardinality is unclassified for scalar-duplicate conformance,
not assumed singular. Raw exact lookup always keeps all matching items.
Treat standalone COMMENT as an ordered collection with no duplicate-scalar
finding merely for repeating; this owner-approved exception is not stated
as an explicit Chapter 9 repeatability guarantee. It does not apply to group
`\COM` fields: `G\COM`, `R-x\COM`, etc. are single-entry per their standard
definitions and repeated instances of the same slot are duplicates, with an
Ambiguous singular value and a finding listing every occurrence. The owner
knowingly chose this standard-based rule despite writers that split long
remarks into repeated fields; raw lookup still returns every line, without
concatenation, dropping or first/last selection. Sources: 106-24R1 Table 9-2
COMMENTS `G\COM`, page 9-13; Table 9-4 COMMENTS `R-x\COM`, page 9-84.

Under ADR-0025, one Format 1 setup record spanning several packets is ONE
document after assembly, not one document per fragment. `G\PN` in fragment
1 and again in fragment 2 is therefore a scalar duplicate in that document.
Locate both occurrences in assembled text with their fragment provenance;
keep successive separate records separate. This clarifies grouping scope,
not the rules for deciding record boundaries or completeness.

INT-038 information findings still apply independently. Reordering only
changes reporting order, not the multiplicity outcome. Generic Ambiguous
must not override F1's separate indeterminate checksum result and stamping
refusal. This policy does not establish link resolution, record completeness
or a clock association; zero/one-candidate default and validation rules follow.

**Reason**: Choosing equal repeats as one explicit value would erase multiplicity and conceal a damaged or combined setup. Conservative ambiguity makes that uncertainty visible; the cost is refusing a usable singular value for benign repeats until the caller explicitly fixes the input. Repeated standalone comments serve a different collection purpose. Joey approved both choices on 2026-11-22 and explicitly retained single-entry duplicate handling for group \COM fields rather than silently joining long remarks.

**Design**: approved (Joey, owner sign-off, 2026-11-22) · **Review**: approved under ADR-0033; draft author: maintainer agent; owner sign-off: Joey, 2026-11-22; independent AI check: CONCURS on PR #22 at `b22d45c`, relayed by the owner 2026-11-22 · **Origin**: family 2 batch 5

**Test**: `identical_scalar_repeats_are_ambiguous`, `invalid_candidate_is_not_filtered`, `comment_collection_is_not_a_scalar_duplicate`, `group_com_is_a_single_entry_duplicate`, `cross_fragment_scalar_duplicate_is_document_local`, `duplicate_and_normalization_findings_coexist` (planned, not yet written)

**Development**: intensive testing and deep analysis required · **Analysis**: [written development analysis](research/2026-11-28-index-duplicate-counter-development-analysis.md) (owner/independent analysis review and intensive tests pending)

**Draft analysis**: [sources, alternatives and remaining work](research/2026-11-22-scalar-duplicates-draft.md)


**Sign-off and AI-check reference**: [owner approval record](research/2026-11-22-int039-owner-signoff.md), referencing [PR #22](https://github.com/TelemetryWorks/irig106-tmats/pull/22) at `b22d45c4fb49c7b1a43ab6a4fd0a9d142c667f28` and the owner's 2026-11-22 relay of the independent check. The record includes the explicit group-\COM choice and the assembly-scope clarification requested with sign-off. Approval is not implementation, completed analysis or executable tests; those development gates remain outstanding.


## Counter membership evidence (approved, owner sign-off 2026-11-22)

### INT-040

**Title**: Count structurally identified entries independently of value validity

**Sources**:
- 106-24R1 Chapter 9 section 9.5.1 a after Table 9-1, page 9-5, specifies indices from one to a declared count with "no missing values"; it does not define recovery membership for invalid entry values.
- Table 9-7 NUMBER OF MEASUREMENT LOCATIONS `D-x\MML\N-y-n` and NUMBER OF FRAGMENTS `D-x\MNF\N-y-n-m`, page 9-117, count location/fragment definitions rather than arbitrary property lines. Section 9.5.14, page 9-172, exempts X occurrence contiguity.
- ADR-0023 separates present-value validity from counters/indices; ADR-0026 declares parent scope. Approved INT-038 requires DISTINCT numeric identities; INT-039 retains all duplicate/invalid scalar candidates without selecting one.

**Behaviour**: **Approved by Joey, 2026-11-22, under ADR-0033.** Given an applicable reviewed counter/member rule and unambiguously identified parent/index positions, form the observed set from structurally identified entry indices even if a member's payload is Invalid or its scalar slot is Ambiguous. Keep all evidence, but count each canonical numeric identity once across duplicate spellings, repeats and declared member properties. Do not erase an entry because its value is bad. Report value/duplicate findings independently; a matching count is only a structural observation, never a validity verdict for those entries or the document.

Retain zero/out-of-domain indices as observed evidence and report domain
violations separately; do not remove them to manufacture a valid 1-to-N set.
If required membership or scope cannot be established, report cannot-evaluate
with the limitation instead of guessing a member or claiming success from a
partial set. Likewise an unusable effective counter cannot be replaced by the
maximum observed index or item count. A cited/permitted default must come
from its own reviewed rule. An unrelated malformed item is not assigned to a
scope by guess. Diagnostic relevance requires a separately framed projected
name with exact declared literal group/path. With declared index-token arity, a bad
governed token with readable matching parents blocks that scope; an unreadable
parent gives unassigned rule-level evidence and cannot-evaluate coverage.
Wrong arity with matching literals is also unassigned rule-level evidence.
Every scope compatible with the unassigned item's readable canonical parent
facts reports partial/cannot-evaluate, NEVER pass, even if its observed set
already matches N. For `D-1\WP-1-1-x-2`, this includes every scope (1,1,1,m).
Compatibility means no contradictory readable parent position; use only
positions mapped unambiguously by the declared pattern, never a guessed
alignment to exclude a scope. Unknown positions impose no restriction.
Retain the evidence at rule level and apply this guard to scopes as evaluated;
do not enumerate hypothetical parent values. This tightening was requested
with sign-off, recorded 2026-11-23.
Nonmatching literals, unframed names or code-like value text are ignored
for that rule's membership/blocking only, with all other findings retained;
no repair, prefix guess or usable key is manufactured.

Evaluate child counts from explicit parent indices despite a parent-domain
violation or unusable ancestor count; 0/00 denote one parent identity. The
child's own applicable rule and count must be usable. A parsed count outside
its own range is Invalid and blocks its own evaluation. X items attach but
never create D members; missing originals are link problems, while extension-
only indices remain unclassified without reviewed rules. WFP value uniqueness
(Table 9-7, page 9-118) is a separate VALUE rule outside INT-040. No bytes,
counters or missing entries are repaired.

**Reason**: Counting only valid values makes a declared entry disappear when its content is broken, confusing cardinality with validity and obscuring separate defects. Counting raw lines overcounts duplicates and multiple properties. Structural evidence plus independent findings exposes both; its cost is that a count may match while the configuration remains invalid, so reports must not call that whole-document success.

**Design**: approved (Joey, owner sign-off, 2026-11-22) · **Review**: approved under ADR-0033; draft author: maintainer agent; owner sign-off: Joey, 2026-11-22; independent AI recheck: CONCURS with clarifications on PR #24 at `176fdf4`, relayed by the owner (recorded 2026-11-23) · **Origin**: family 2 batch 6

**Test**: `invalid_payload_keeps_structural_membership`, `counter_counts_distinct_identities`, `unusable_counter_is_not_inferred`, `incomplete_membership_does_not_pass`, `parsed_counter_outside_own_range_blocks`, `x_extension_does_not_create_d_member`, `invalid_parent_domain_does_not_suppress_child`, `zero_parent_spellings_share_one_scope`, `malformed_governed_token_blocks_known_scope`, `malformed_parent_is_unassigned_rule_evidence`, `compatible_unassigned_evidence_never_passes`, `incompatible_parent_does_not_inherit_unassigned_block` (planned, not yet written)

**Development**: intensive testing and deep analysis required · **Analysis**: [written development analysis](research/2026-11-28-index-duplicate-counter-development-analysis.md) (owner/independent analysis review and intensive tests pending)

**Draft analysis**: [sources, alternatives and remaining work](research/2026-11-22-counter-membership-draft.md)


**Sign-off and AI-check reference**: [owner approval record](research/2026-11-23-int040-owner-signoff.md), referencing [PR #24](https://github.com/TelemetryWorks/irig106-tmats/pull/24) at `176fdf4ebe0a69f83da1b12202a462274fe91d8e`, the owner's sign-off dated 2026-11-22 and the relayed independent recheck recorded 2026-11-23. The record identifies the requested compatible-scope tightening. Approval is not implementation, completed analysis or executable tests; those development gates remain outstanding.


## Unavailable effective-field interpretation (approved, 2026-11-23)

### INT-041

**Title**: Unavailable interpretation is not proof of missing or invalid field data

**Sources**:
- 106-24R1 Chapter 9 section 9.5.1 a, page 9-6, defines a default for when the file does not contain the attribute; Table 9-4 TIME FORMAT R-x\TFMT-n, page 9-72, supplies Default A with TIMEIN applicability. These passages do not define an API state for an unavailable rule.
- ADR-0022 requires unreviewed interpretations to stay visibly unavailable and conditions with missing/invalid operands to report cannot-evaluate. ADR-0023 defines five supported-field states; ADR-0028 labels the edition basis.
- RELEASE-0.1.md requires absent TFMT with unestablished default applicability/basis to report its missing/indeterminate reason, not invent A. Approved INT-037 blocks semantic lookup for leading encoding markers, and INT-039 keeps known scalar multiplicity Ambiguous.

**Behaviour**: **Approved by Joey, 2026-11-23, under ADR-0033.** A request whose reviewed field definition/basis is unavailable, or whose required local-validity evidence cannot be evaluated, reports an unavailable/cannot-evaluate read with the reason and retained raw candidates. Do not call unknown data Invalid merely because the decoder/rule is unavailable, and do not label unexamined or uninterpretable presence as Missing. This guards five-state resolution rather than adding a sixth field-value state; the concrete Rust representation remains a later design choice.

Keep any Ambiguous outcome already proved from reviewed scalar cardinality;
missing value-validation knowledge must not select a candidate or hide that
multiplicity. Likewise retain a demonstrated Invalid failure even if another
local constraint is unavailable; cannot-evaluate must not erase proved errors.
For a supported field actually absent, lack of a verified
applicable default remains Missing with the default-unavailability reason,
as the existing release contract requires. Known present empty/invalid data
never becomes absence for defaulting. A supported local value can be Explicit
without claiming all other applicability/presence/packet checks have passed.
Thus TFMT present on a non-TIMEIN channel is Explicit locally plus an
applicability finding, never ignored as absent. ASCII case-insensitive `a`
means Explicit A with the original `a` retained (section 9.4.2, page 9-2).
A value ` A` instead stays unavailable until a blank-handling rule is reviewed;
blanks are assumed intentional, so do not trim them or declare Invalid by
an unreviewed strict comparison. This does not change name-edge blank rules.

Absent TFMT with an Ambiguous CDT operand is Missing with an unknown-condition
reason and the condition finding, not Defaulted and not a proven false
condition. Consumers must read the Missing reason: unestablished default
applicability is uncertainty, not permission to infer an inapplicable default.
Invalid results reached under a fallback edition keep that fallback label,
applied edition and reason, just as other results keep their interpretation
basis. These edge notes were requested with the 2026-11-23 sign-off.

**Numeric-policy follow-up (owner, 2026-11-25)**: INT-055 records the
numeric field-value rule, with final independent CONCUR at #51 ebd5c98. The owner's clarification
applies leading-blank unavailability uniformly to every decimal-integer
value and numeric-looking code; trailing/embedded blanks are Invalid.
Letter-value and structured G\SHA blank rules are
not silently changed. Demonstrated high-byte/other failures remain visible.

**Reason**: Missing describes actual absence and Invalid requires evidence about the data; neither is justified solely by missing interpretation knowledge. The alternative is forcing all unsuccessful reads into those states, which conceals unsupported coverage and can trigger a false default or false data-error report. Consumers must handle blocked reads explicitly, at the cost of a guarded result rather than treating every request as a successfully resolved field.

**Design**: approved (Joey, owner sign-off, 2026-11-23) · **Review**: approved under ADR-0033; draft author: maintainer agent; owner sign-off: Joey, 2026-11-23; independent AI check: CONCURS on PR #26 at `0fc0bc8`, relayed by the owner 2026-11-23 · **Origin**: family 2 batch 7

**Test**: `unavailable_definition_is_not_data_invalidity`, `unknown_validity_operand_blocks_explicit`, `known_multiplicity_survives_unavailable_value_rule`, `lowercase_tfmt_keeps_raw_case`, `blank_enum_requires_reviewed_handling`, `present_non_timein_tfmt_is_not_absence`, `ambiguous_cdt_makes_default_condition_unknown`, `invalid_under_fallback_keeps_basis` (planned, not yet written)

**Development**: intensive testing and deep analysis required · **Analysis**: [written development analysis](research/2026-11-28-effective-presence-basis-development-analysis.md) (owner/independent analysis review and intensive tests pending)

**Draft analysis**: [sources, alternatives and remaining work](research/2026-11-23-effective-value-draft.md)


**Sign-off and AI-check reference**: [owner approval record](research/2026-11-23-int041-owner-signoff.md), referencing [PR #26](https://github.com/TelemetryWorks/irig106-tmats/pull/26) at `0fc0bc8917d4e80d1c0c3274bc8868083afb1a28` and the owner's 2026-11-23 relay of the independent check. The record includes the requested edge notes; approval is not implementation, completed analysis or passing tests, and this new PR still needs independent review.


## Presence predicates versus value predicates (approved, 2026-11-23)

### INT-042

**Title**: A reviewed is-specified predicate tests actual presence, not value usability

**Sources**:
- 106-24R1 Chapter 9 section 9.5.1 a, page 9-6, states: "If the attribute is specified in the TMATS file, it must contain valid information". The sentence sits within the R/R Ch 10 Status bullet: it is strong evidence separating physical specification from the validity obligation in that context, not a general definition of specified. It is the textual basis supporting Option A; the section also describes defaults for absent attributes. It does not formally define recovery semantics for a presence condition whose named item is malformed or duplicated.
- Table 9-3 TRANSMIT ANTENNA TYPE T-x\AN1, page 9-18: "Allowed when: T-x\ID is specified". This explicit indexed reference supplies a presence example without resolving an omitted index by guess.
- Table 9-4 TIME FORMAT R-x\TFMT-n, page 9-72, compares R\CDT with TIMEIN; that needs a usable value, unlike the presence example. Its occurrence mapping requires its own reviewed binding.
- L1-REG-004, ADR-0022/0023 and approved INT-035/036/037/038/039/040/041 require declared scope, retained actual items and visible uncertainty. This approved interpretation clarifies that a known absence is evidence for a presence predicate, while a missing VALUE operand still blocks comparison.

**Behaviour**: **Approved Option A by Joey, 2026-11-23, under ADR-0033.**
When a reviewed condition explicitly tests physical presence in an established
scope, one or more actual lookup matches means true, even
if their values are empty, Invalid or Ambiguous. Retain all candidates and
independent findings. A default never makes an absent item physically
specified. True presence does not make any value usable or select a duplicate.
Zero exact matches means false only if no compatible related damaged evidence
remains in that same document/setup record. With a related unterminated suffix
or damaged name in the same declared literal family, it means cannot-evaluate.
Thus `T-1\ID:a` at EOF and `T-1\ID\xFF:a;` both block false for T-1 ID
presence; neither becomes a usable lookup item or proves presence. T-1\AN1
must not receive a not-allowed verdict from that unknown condition.

Use INT-040's diagnostic compatible-scope guard for readable canonical parent
facts, including multiple-entry child scopes with damaged parents; unknown
positions impose no restriction and wrong arity does not justify guessed
alignment. An unterminated suffix that is a proper prefix of the literal also
blocks false for any predicate it could complete to: `COMMENT:x;T-1\I` or
`COMMENT:x;T-1\` at EOF gives cannot-evaluate for compatible T-1 ID presence,
without completing the text or creating a lookup item. Readable contradictory
literal/parent facts exclude incompatible predicates; evaluate a requested
predicate's compatibility without enumerating possible completions.

A damaged name generally has uninterpretable bytes preventing an exact key,
with readable literal/parent facts compatible with the declared family.
High bytes anywhere within or adjoining the literal qualify, e.g. both
`T-1\I\xFFD:a;` and `T-1\ID\xFF:a;`; this is not restricted to trailing
bytes. ASCII controls already pass through INT-036's projection. Preserve
raw bytes; diagnostic relevance neither removes high bytes to form a usable
key nor permits arbitrary printable-prefix matches or value-text searches.

Implementation clarification requested by the owner, 2026-11-23: after
INT-036 projection/case folding, readable characters must appear in order;
each maximal high-byte run stands for zero or more unknown bytes. Match
whole literals, with no implicit gaps except at those runs; an unterminated
EOF suffix can also lack its unread remainder. Thus `T-1\I\xFF:a;` is
compatible with ID (the run can cover D), as are I\xFFD and ID\xFF, while
D\xFFI is not. This deterministic compatibility check supplies diagnostic
evidence only, never a repaired key; L3-VAL-012 and its planned cases pin it.
Readable-parent guards still apply without invented parents or alignments.
If a high-byte run can span a component separator, the component split is
unknown: `T-1\xFFD:a;` could hide `\I` before readable D. Retain this as
unassigned structural evidence under INT-040's wrong-arity guard, compatible
with every scope the readable parts allow. Do not guess a split or a parent
alignment, and do not reject a compatible scope merely because one guessed
split fails. This does not create a parsed key or require enumerating scopes.

Incompatible parents/unrelated families do not block this predicate's false
answer; keep their findings. One known exact match proves presence even if
additional damaged evidence exists. INT-035 framing is unchanged.

Exact matches use the INT-036/038 projection, case-insensitive name matching
and canonical indices, retaining normalization information and original bytes.
Other documents or separate setup records do not contribute evidence; one
record assembled across packets is one document under INT-039/ADR-0025/0031.
Is-not-specified complements true/false only; cannot-evaluate stays unknown.

Value comparisons instead require usable Explicit or expressly permitted
Defaulted operands. Missing, Invalid, Ambiguous, unavailable or disallowed-
default operands give cannot-evaluate, including for inequality. Unknown
lookup scope, blocked lookup or an unresolved/ambiguous required link target
also gives cannot-evaluate; it is not proof that the target item is absent.
Do not infer true presence from an unterminated suffix or repair a code name.
No general rule inferring reference scope from abbreviated prose, compound
Boolean semantics or new default is selected by this interpretation.

**Reason**: Physical absence is an answer to a presence question but cannot supply a value for equality. Requiring a valid singular value for is-specified hides actual supplied fields, while treating every failed value comparison as false hides uncertainty. The approved distinction keeps both facts visible; its cost is that presence can be true while the same field is invalid or ambiguous, so consumers must not mistake presence for validity. The alternative is cannot-evaluate for presence whenever the named field lacks a usable singular value; that is conservative but conflates presence with usability. Related damaged evidence prevents a false absence verdict at the cost of additional cannot-evaluate outcomes until the input is fixed. Joey approved amended Option A, 2026-11-23; the owner-requested prefix/damage clarifications are recorded with sign-off.

**Design**: approved (Joey, owner sign-off, 2026-11-23) · **Review**: approved under ADR-0033; draft author: maintainer agent; owner sign-off: Joey, 2026-11-23; independent AI recheck: CONCURS with amended Option A on PR #28 at `d69d275`, relayed by the owner 2026-11-23 · **Origin**: family 2 batch 8

**Test**: `specified_absent_is_false`, `specified_invalid_and_duplicate_stays_true`, `presence_scope_unknown_is_not_absence`, `missing_value_does_not_make_inequality_true`, `default_does_not_make_item_specified`, `unterminated_text_is_not_specified`, `damaged_name_blocks_false_presence`, `presence_uses_canonical_name_identity`, `damaged_child_parent_blocks_compatible_scopes`, `not_specified_preserves_unknown`, `other_document_does_not_supply_presence`, `unterminated_literal_prefix_blocks_compatible_presence`, `high_byte_inside_literal_blocks_false_presence`, `projected_controls_preserve_exact_presence`, `high_byte_run_covers_missing_literal_tail`, `damaged_literal_readable_order_is_required`, `high_byte_run_can_match_empty`, `terminated_literal_has_no_implicit_gap`, `high_byte_run_hiding_separator_is_unassigned` (planned, not yet written)

**Development**: intensive testing and deep analysis required · **Analysis**: [written development analysis](research/2026-11-28-effective-presence-basis-development-analysis.md) (owner/independent analysis review and intensive tests pending)

**Draft analysis**: [sources, alternatives and remaining work](research/2026-11-23-condition-scope-draft.md)


**Sign-off and AI-check reference**: [owner approval record](research/2026-11-23-int042-owner-signoff.md), referencing [PR #28](https://github.com/TelemetryWorks/irig106-tmats/pull/28) at `d69d275340d1773d9548cf365d57db8fc05822f1` and the owner's 2026-11-23 relay of independent recheck concurrence. The record includes the requested prefix/damaged-name clarifications and source-context caveat. Approval is not implementation, completed analysis or passing tests; this sign-off-record PR still needs independent review.


## Noncanonical SHA algorithm designator (approved, 2026-11-23)

### INT-043

**Title**: Leading-zero algorithm 2 designators are accepted with a warning

**Sources**:
- 106-24R1 Chapter 6 section 6.2.3.11 f, page 6-22: "prefixed with the constant string “2-”".
- 106-24R1 Chapter 9 Table 9-2 MESSAGE DIGEST/CHECKSUM, page 9-13: "SHA2-256 shall be represented as “2-” followed by 64 hex characters"; its range also describes an integer followed by a hyphen and hex characters. This does not explicitly settle zero-padded input recovery.

**Behaviour**: **Approved under ADR-0033: Joey, Option B, 2026-11-23; independent AI recheck CONCURS on PR #32 head `8941ca1`.** The owner chose Option B, overriding the strict recommendation.

From the INT-036 projected value, read a nonempty ASCII-digit algorithm
designator before the ASCII hyphen.
Use the INT-038 canonical-string approach: strip leading zero digits,
retaining `0` for an all-zero spelling, then compare the resulting string
with `2`. Do not use fixed-width integers, big integers or numeric conversion.
This applies the owner's explicit value-token decision; INT-038 alone did
not previously authorize algorithm-value equivalence.

`2-`, `02-`, `002-` and any other all-digit spelling whose canonical string
is `2` identify algorithm 2. Verification may be positive if the digest
matches and all other verification/exclusion prerequisites are met. For every
accepted spelling other than `2-`, always emit a located warning with code
**NONCANONICAL-DESIGNATOR**, citing 106-24R1 Chapter 6 section 6.2.3.11 f,
page 6-22 (the constant string `2-`) and Chapter 9 Table 9-2 MESSAGE DIGEST/
CHECKSUM, page 9-13. The warning is independent of whether the digest matches;
acceptance never suppresses it. Retain the original token and every raw byte.
Canonical-string comparison is an interpreted view, not an input rewrite.

Review clarification recorded with approval: emit NONCANONICAL-DESIGNATOR
for every located SHA item/candidate whose readable token is a noncanonical
2, even when verification is blocked by other reasons: F1 duplicates,
compatible damage, a blocking BOM or undefined exclusion. Do not short-circuit
the warning with the integrity verdict or collapse occurrences into one.
Diagnostic token recognition under a BOM/damaged-name guard does not restore
semantic lookup, create an exact key, choose an exclusion or permit positive
verification. Keep item/token locations and all blocking findings; never
search COMMENT values or X paths for a fictitious checksum occurrence.


`0-` (including all-zero spellings) and other numeric designators whose
canonical string is not `2` are unknown/unsupported algorithms under
L1-SUM-002; never map them to algorithm 2. The distinct no-hyphen `G\SHA:0;`
placeholder retains its INT-020 status. Stamping always writes canonical
`2-` and lower-case hex, regardless of an accepted input spelling.

Edge cases are unchanged: leading-blank ` 2-` is unavailable until the blank
rule is reviewed; UTF-8 `2–` (en dash) is Invalid under INT-037; `+2-` and
`2.0-` are malformed. Upper-case digest digits follow INT-020, including an
additional upper-case warning alongside NONCANONICAL-DESIGNATOR for 02/002.
A defined exclusion permits COMPUTE independently of the stored token's
verification status (L3-SUM-002). No source bytes are repaired or dropped.

**Reason**: Joey chose tolerance of real-world writers: leading zeros still identify numeric algorithm 2, with an unavoidable warning exposing the deviation from the printed constant. The rejected strict option supported only literal 2- and reported 02/002 as unknown algorithms; its reasoning was that Chapter 6's constant spelling avoids adding numeric equivalence beyond the printed representation. The owner explicitly preferred tolerant identity with preserved raw bytes and warning, accepting the broader verification behavior. Owner approval and AI concurrence now complete the semantic review gate; analysis and implementation evidence remain outstanding.

**Design**: approved (Joey, Option B, 2026-11-23) · **Review**: approved under ADR-0033; draft author: maintainer agent; owner decision: Joey, Option B, 2026-11-23; independent AI recheck: CONCURS on PR #32 head `8941ca1c2c436ca453f09e7fd530031136f84c57`, relayed by the owner 2026-11-23; later clarification wording still requires exact-head review · **Origin**: INT-020 review follow-up, family 2 checksum slice

**Test**: `zero_padded_sha_algorithm_requires_decision`, `sha_designator_edge_classification`, `sha_stamp_uses_canonical_designator`, `sha_algorithm_identity_uses_canonical_string`, `noncanonical_sha_designator_always_warns`, `other_numeric_sha_designators_stay_unsupported`, `noncanonical_designator_warns_when_verification_blocked` (planned; not yet written)

**Development**: intensive testing and deep analysis required · **Analysis**: not yet written

**Draft analysis**: [source, alternatives and independent fixture oracle](research/2026-11-23-checksum-followups-draft.md)

**Owner decision record**: [Option B, rationale and rejected alternative](research/2026-11-23-int043-owner-decision.md). Owner Option B, 2026-11-23, is now approved with AI concurrence at #32 head `8941ca1`; see the [approval record](research/2026-11-23-int043-owner-signoff.md). The earlier decision record remains history; analysis, freshness and executable tests are still outstanding.


## Ambiguous TMATS edition declarations (proposed, 2026-11-23)

### INT-044

**Title**: Duplicate G\106 retains ambiguity with an explicitly labelled fallback basis

**Sources**:
- 106-24R1 Chapter 9 section 9.4.2 page 9-2 describes unique names/single-entry attributes; Table 9-2 RCC IRIG 106 REVISION LEVEL G\106, page 9-10, names the generating edition but defines no recovery for duplicate declarations.
- Approved INT-039 makes repeated scalar fields Ambiguous even when values agree. ADR-0028 selects override, declared edition, then labelled fallback for missing/unrecognised G\106; it does not explicitly classify ambiguous G\106 as missing/unrecognised.
- Owner decision, Joey, 2026-11-23: Option B following the independent reviewer's recommendation, with a new explicit ambiguity reason and the existing INT-017 policy unchanged.

**Behaviour**: **Approved: Joey, owner Option B decision, 2026-11-23; independent AI recheck CONCURS on #36 at 6f6e080, under ADR-0033.**
With duplicate G\106, keep the declaration Ambiguous with every occurrence
and all duplicate findings; select no candidate. Without an explicit
override, select a separate rules basis labelled
`fallback — reason: ambiguous G\106 (n candidates)`, where n counts all
scalar occurrences, including identical ones, not distinct years or edition
readings. This is a NEW reason, never missing or unrecognised. Mixed duplicates such
as `G\106:zz;G\106:24;` remain Ambiguous first, with n=2; an individual
unrecognised value does not replace the duplicate reason.

Use the SAME INT-017 policy, not a new mapping: eligible Format 1 RCCVER
0x0E selects 106-24R1 rules, 0x0D selects 106-20 rules; otherwise use the
existing policy's applicable reading or, with no usable eligible RCCVER,
the baseline. INT-017's 0x07–0x0C readings remain unchanged. No Format 4
or packet-header code supplies a Format 1 operand. Defaults and findings
under this basis carry the fallback label and ambiguity reason.

A valid explicit override still takes precedence with its override label;
all ambiguity evidence remains. An unusable override blocks selection under
L3-EDN-003, rather than silently activating fallback. Identical duplicates also fall back: their
agreement is information only, never permission to collapse occurrences or
claim a declared basis. Separate records/documents do not form duplicates.
A single G\106 with a high byte or blank follows INT-037/041 and the
reviewed declaration binding; it does not acquire this duplicate reason.
CLI text and JSON make the fallback basis/reason at least as prominent as
a cannot-evaluate result. JSON uses stable machine code `ambiguous_g106`
and a numeric `candidates` field, separate from the display label; that
count follows the same occurrence rule. No source bytes change.

This differs from one valid year naming several candidate editions under
INT-014; that rule and all other review/coverage gates remain unchanged.

**Reason**: ADR-0028 rejected "validate nothing edition-specific without an override" for missing or unrecognised G\106. Option A would therefore have validated worse inputs but refused duplicated good ones. Joey chose useful, visibly qualified checks over blocking all automatic rules selection, following the independent reviewer's recommendation. Preserving Ambiguous separately from the applied basis avoids pretending the writer selected an edition. **Rejected Option A** blocked automatic selection until an override: ADR-0028 explicitly allowed fallback for missing/unrecognised declarations only, so A avoided expanding that policy and preserved uncertainty by refusing a basis, but required an override even when duplicate years agreed. Option B extends the eligible reason explicitly, preserving that uncertainty in every report rather than selecting a candidate.

**Design**: approved (Joey, Option B, 2026-11-23) · **Review**: complete under ADR-0033; draft author: maintainer agent; owner decision: Joey, 2026-11-23, Option B; independent AI recheck: CONCURS on #36 at 6f6e080 · **Origin**: family 2 edition-basis batch 13

**Test**: `ambiguous_g106_uses_labelled_fallback`, `identical_g106_duplicates_still_fall_back`, `override_keeps_ambiguous_declaration_evidence`, `ambiguous_fallback_uses_existing_policy`, `ambiguous_fallback_labels_defaults_and_findings`, `single_damaged_or_blank_g106_is_not_duplicate`, `ambiguous_fallback_is_prominent`, `mixed_g106_duplicates_keep_ambiguity`, `ambiguous_g106_json_reason_is_structured` (planned; not yet written)

**Development**: intensive testing and deep analysis required · **Analysis**: [written development analysis](research/2026-11-28-effective-presence-basis-development-analysis.md) (owner/independent analysis review and intensive tests pending)

**Draft analysis**: [original alternatives](research/2026-11-23-edition-basis-draft.md); [owner Option B decision and conditions](research/2026-11-23-int044-owner-decision.md)

**Approval record**: [owner decision and exact AI concurrence](research/2026-11-23-int044-owner-signoff.md). Analysis, source freshness and executable tests remain outstanding.

### INT-045

**Title**: Layered time-packet closure for initial setup and SRCC=1 updates

**Sources**:
- 106-24R1 Chapter 11 section 11.2.3.2 p. 11-24: "Only static Computer-Generated Data, Format 1 packets may precede the first time data packet." Section 11.2.3.3 p. 11-26 repeats the rule for network time; Table 11-4 pp. 11-6–7 defines 0x11/0x12 and reserved time-family codes.
- Project-archived [106-13 chapter10.pdf](https://github.com/TelemetryWorks/rcc-106-standards/releases/download/106-13/chapter10.pdf), section 10.6.3.2, printed p. 10-44 (PDF page 56), contains the same ordering rule. SHA-256: `e3740c7ac3099236f4e97cbe5b84c169340e6d4a4222769619e83ea0f8176fde`. The third-party Spiral archive is a secondary copy; the distinct p. 10-41 copy inspected for #40 is the irig106.org PDF. Fingerprints/pagination are not interchangeable.
- Section 11.2.7.2 p. 11-50 puts an updated setup record "prior to any new or changed data packets being committed to the stream"; p. 11-51 puts its configuration-change event before the setup record, not at its end. This does not print a universal mid-session time-packet terminator.
- Joey, 2026-11-23: initial Option A signed under ADR-0033, with independent AI CONCURS on #40 at 93eb6e2; owner extends closure to SRCC=1 mid-recording setup candidates at the next trusted recognised time packet. Guarded mid-session extension AI recheck CONCURS on #41 at 79b94eb; approved under ADR-0033.

**Behaviour**: **Approved: Joey, initial Option A and guarded mid-session extension, 2026-11-23; independent AI CONCURS on #41 at 79b94eb under ADR-0033.**
This is a SEPARATE LAYERED closure rule on top of INT-011. INT-011 joins
qualifying fragments independently per channel; INT-045 seals candidate
phases, after which later fragments cannot be joined backward into them.
The +1/FRMT/RCCVER joining predicates and INT-011 approval are unchanged.

**Initial phase**: A known session start is a CALLER-ASSERTED, labelled
provenance fact, never inferred from file offset zero, a filename, sync,
sequence number, SRCC or the first packet observed. Each asserted session
start begins its own initial phase; retain session versus recording source
wording and do not infer a new session at a file boundary. Under an applicable
reviewed packet-format binding, close the initial candidates immediately
before that phase's first trusted recognised time packet on ANY channel.
Every preceding packet must be static Format 1 (0x01): Format 0/2/3/4 or
any other data is an ordering violation, with a finding and withheld adoption.

If the FIRST observed time-packet candidate is untrusted, withhold adoption
and retain that evidence; the next trusted one does not take over as the
initial first-time boundary. Unknown/cropped start, untrusted prefix or
unavailable packet binding likewise withholds authoritative initial adoption.
Never skip an earlier unreadable region to claim a clean prefix.

**Mid-session phase**: A candidate's SRCC=1 status comes from its FIRST
fragment, not a later fragment, majority vote or an aggregation. Such a setup
candidate is sealed immediately before the next trusted recognised time packet, on ANY channel, by explicit
owner extension. Qualifying fragments received before that boundary belong
to the active candidate; repeated SRCC=1 on its fragments does not itself
restart it. Retain SRCC disagreement as separate evidence; later bits do not
change this first-fragment eligibility. A first-fragment SRCC=0 retransmission
stays outside this extension even if a later fragment says 1.
The initial-only static-prefix guard is not a ban on ordinary mid-session
data interleaving. Earlier untrusted observations remain blocking trust
findings; a later trusted marker does not erase them. An unreadable or
untrusted first-fragment SRCC does not establish eligibility.

**Marker guards**: 0x11 Format 1 is eligible in all checked 106-07-onward
editions; 0x12 Format 2 is eligible only where defined, from 106-17. Before
106-17, 0x12 is reserved and ineligible. Reserved/unknown types, IPTS,
secondary-header timestamps, TFMT text and Ethernet-contained NTP/PTP never
count. Framing/bounds and applicable checksums must be trusted; clock
availability is separate, so source NONE does not disqualify a trusted time
packet. A time packet on channel 0 in 106-17+ gets the separate channel-use
conformance finding (Chapter 11 section 11.2.1.1 b), never silent acceptance
as compliant; channel number does not turn it into setup or another format.

**Per-edition channel-use review input (not a new approval)**: Bind the
separate channel-use check to an exact edition/source profile; do not infer
all historical behavior from the baseline's "as of 106-17" wording. The
project-archived 106-13 p. 10-29 instead says "as of 106-13". The
[per-edition source review](research/2026-11-23-channel-zero-edition-sources.md)
records the passages and scope questions for INT-011/045. It does not change
the approved marker guards, adopt an older-edition conformance predicate,
or confuse channel conformance with joining or closure eligibility.

**Safe closure and evidence**: Retain the phase, channel/run identities,
caller assertion, marker offset/type/trust and raw spans; never mix channels,
include time bytes in TMATS or silently choose a governing candidate. Apply
INT-034 at the sealed boundary. Text cut mid-attribute is **INCOMPLETE with
a located finding**, not silently held open until a later time packet. A
same-channel +1 fragment after the boundary is **continuous-but-split**:
retain both sides, sequence/CSDW evidence and the boundary link, never rejoin
or repair the sealed candidate. Continuity is not proof of a new valid record
or an instruction to choose a split/winner. Preserve duplicate/split and XML
uncertainty; no boundary alone proves usable metadata or checksum success.

**Possible premature seal, final outcome**: If an otherwise Complete
semicolon-ending sealed candidate is followed by a continuous-but-split
same-channel +1 fragment with the SAME FRMT/RCCVER, its FINAL outcome becomes
**AMBIGUOUS**. Retain BOTH readings: a complete sealed candidate followed by
a separate candidate, or one fragmented record prematurely split by time.
Emit a located **"possible premature time-boundary seal"** finding naming
the seal and subsequent fragment's locations. Never rejoin either reading.
The existing cut-mid-attribute case remains INCOMPLETE, not repaired by this
guard or by later text.

**Exception**: If reviewed, unambiguous G\SHA verification MATCHES the sealed text
alone, that otherwise Complete candidate stays **Complete**, explicitly on
that classification evidence. Verification must obey INT-018/019/020/021/043:
Missing, mismatching, malformed, unsupported, unavailable, unverified or
indeterminate verification does not qualify. Never stamp, normalise or append later bytes
to obtain a match. Every mid-session Complete produced by this layer carries
the distinct basis **`time-boundary seal (owner policy)`**, with verified
G\SHA evidence additionally named when the exception applies. It does not
prove all required attributes, record validity or governing applicability.
The comparison uses the fixed sealed-candidate byte domain as classification
evidence; it is not an authoritative integrity verdict for an unresolved
complete-record domain and does not waive the general checksum gates.
Continuity, both readings and the located finding remain visible even when
this checksum exception keeps the classification Complete.

Supporting signals are G\SHA verification (classification evidence only,
never a rejoin); missing minimum required attributes (106-24R1 section
11.2.7.2 p. 11-51); continuation content (heuristic); and packet fill (weak).
Missing requirements and heuristic/weak observations do not bypass the final
ambiguity guard or substitute for verified G\SHA. Identify the actual
reviewed requirement/locations when reporting missing attributes; unavailable
registry knowledge is not proof of absence.

A Complete report at seal is revisable when later continuity evidence
arrives: retain candidate identity and expose the revised outcome, readings
and finding. Consumers such as irig106-time's spool-before-publish can reflect
that finding before publishing. No invented lookahead timeout makes the
classification irrevocable; publication/spooling implementation remains the
consumer's contract, not library I/O or permission to repair records.

INT-039 remains document-local: repeated scalars within a candidate keep
every occurrence and duplicate finding; a later candidate across an explicit
closure is not appended to fabricate duplicates in the earlier document.
No time packet: live input stays open/provisional within its explicit budget;
actual EOF still uses INT-034's labelled outcome. Pause, budget exhaustion
and source NONE do not manufacture EOF. Dispatch a trusted boundary packet
after reporting the candidate's outcome, without requiring a whole-file scan.
Initial closure and SRCC=1 closure do not decide general snapshot permission,
INT-028/031 governance, a fallback to an old description or time associations.

**Diagnostic follow-up suggestion (classification unchanged)**: A G\SHA
located in the CONTINUATION that would verify the joined hypothesis may be
noted as diagnostic information only. It never supplies the sealed-text
exception, authorises a rejoin/adoption, upgrades Incomplete/Ambiguous, or
claims authoritative integrity for the joined reading. Keep locations and
the unadopted hypothesis label; the original candidates stay separate.

**INT-048 amendment (review clarification, 2026-11-25)**: References above
to a mid-attribute cut remaining INCOMPLETE have the strict precedence
**Incomplete, or aggregate Ambiguous WITH A RETAINED CUT FACET** under
INT-048 when duplicate SHA or other established field ambiguity is present.
This is not completion or a weaker blocker: the cut facet retains ALL of
Incomplete's blocking force. No ambiguous-acceptance option, including
proposed INT-031's labelled split if approved, can make such cut text usable
or governing. Neither identity, digest evidence nor a time seal waives it.

**Reason and rejected recommendation**: Initial Option A is grounded in the
explicit first-time ordering. The original recommendation rejected a general
mid-session next-time/data terminator: no observable universal terminator is
specified, changed-data identity can depend on TMATS, and the configuration
change event PRECEDES the update (its identity remains unverified, INT-030).
The owner overrides that recommendation for SRCC=1 using the next trusted
TIME packet specifically, not arbitrary data. This is owner policy beyond
the printed ordering; periodic time may split a legitimate multi-fragment
update. The incomplete finding and continuous-but-split evidence expose that
cost rather than silently repairing it. The historical alternatives remain
in the #40 draft; neither the event nor a nearby 0x02 is guessed as an end.

**Design**: approved (Joey, initial A and guarded mid-session extension, 2026-11-23) · **Review**: complete under ADR-0033; draft author: maintainer agent; initial owner sign-off: Joey, 2026-11-23, Option A, independent AI CONCURS on #40 at 93eb6e2; owner mid-session extension with premature-seal guard approved, 2026-11-23; independent AI recheck CONCURS on #41 at 79b94eb, resolving its prior narrow CONCERNS · **Origin**: owner question after #39 and owner extension after #40, 2026-11-23

**Test**: `initial_first_time_seals_before_dispatch`, `initial_time_type_is_edition_specific`, `non_time_timestamps_do_not_close_setup`, `initial_marker_requires_start_order_and_trust`, `initial_time_does_not_repair_incomplete_setup`, `no_initial_time_uses_eof_or_provisional`, `initial_runs_and_later_documents_do_not_mix`, `mid_session_time_seals_srcc_update`, `mid_session_cut_update_is_incomplete`, `post_boundary_plus_one_is_continuous_but_split`, `all_non_format1_initial_packets_violate_order`, `session_starts_are_asserted_and_independent`, `untrusted_first_time_cannot_be_replaced`, `channel_zero_time_has_conformance_finding`, `semicolon_seal_continuation_is_finally_ambiguous`, `verified_sha_keeps_sealed_candidate_complete`, `srcc_eligibility_comes_from_first_fragment`, `premature_seal_signals_do_not_replace_verified_sha`, `continuation_sha_is_diagnostic_only` (planned; not yet written)

**Development**: intensive testing and deep analysis required · **Analysis**: not yet written

**Draft analysis**: [original sources/options](research/2026-11-23-initial-time-boundary-proposal.md); [owner layered closure decision, source fingerprint and amended safety cases](research/2026-11-23-int045-layered-owner-decision.md).

**Approval record**: [owner approval and exact AI concurrence](research/2026-11-23-int045-owner-signoff.md). Source freshness, completed development analysis and executable tests remain required. The separate [mid-session corner-case catalogue](research/2026-11-23-mid-session-setup-corner-cases.md) is discussion only, with no approved options.



### INT-046

**Title**: Bounded byte-identity exception retains the premature-seal finding (QI)

**Sources**:
- 106-24R1 Ch11 §11.2.7.2 p. 11-50: “0 = Setup record configuration has not changed”; “1 = Setup record configuration has changed”. SRCC is a declaration, not a byte comparison.
- INT-011's per-channel joining and INT-045's layered time closure retain their cited Ch11 sources. Ch6 §6.2.3.11 f pp. 6-22–23: 'all text between the "G\SHA" and the following semicolon, inclusive'; digest equality is not equality of every raw byte.
- Owner sign-off, Joey, 2026-11-25: [verbatim decision record](research/2026-11-25-retransmission-owner-signoffs.md). This is owner policy where the standard supplies no recovery/adoption rule.

**Behaviour**: QI is owner-signed: only a bounded, otherwise-Complete candidate, time-sealed and followed on the same channel at sequence +1 with the same FRMT/RCCVER, may stay Complete on whole raw-text identity, labelled `identical retransmission` alongside its closure basis. This holds for following first-fragment SRCC 0/1/U, never by inferring a bit. Keep the located `possible_premature_time_boundary_seal_both_readings` finding, including on Complete and duplicate-SHA outcomes. **QD/INT-048 takes precedence**: field ambiguity, including duplicate G\SHA, is assembly Ambiguous and cannot be upgraded by identity, equality or a digest. Already-cut text never becomes Complete through matching a prefix. The mid-session label remains `time-boundary seal (owner policy)`. A following-only checksum cannot verify the sealed text. No backward rejoin, stamping, governing switch or cross-channel extension is authorised. QM is now owner-signed in INT-053: compare with the currently governing setup, never choose an unresolved target; immediately previous-copy equality is information only.

**Reason**: Whole-byte identity supports the bounded exception, while the retained finding exposes its uncertainty. The stricter QD owner decision explicitly overrides any interpretation that identity could rescue field ambiguity.

**Design**: approved (Joey, owner sign-off 2026-11-25; independent AI CONCUR at 73396bc) · **Review**: complete under ADR-0033; draft author: maintainer agent; owner sign-off: Joey, 2026-11-25; independent AI check: CONCUR on [PR #45 head 73396bc312e24cfccaf16de9ae7a72609faf75b6](https://github.com/TelemetryWorks/irig106-tmats/pull/45/commits/73396bc312e24cfccaf16de9ae7a72609faf75b6), reviewer @thread:thr_qsvwmtnn8f, approval relayed by the owner 2026-11-25; source freshness, development analysis and executable tests remain required before implementation · **Origin**: #44 matrix decisions, 2026-11-25

**Test**: `srcc1_identical_is_not_assumed_change`, `identical_duplicate_sha_never_verifies`, `identity_keeps_premature_seal_finding` (planned; not implemented or executed)

**Development**: intensive testing and deep analysis required · **Analysis**: not yet written

**Decision record**: [dated owner sign-off and alternatives](research/2026-11-25-retransmission-owner-signoffs.md); [explicit matrix](research/2026-11-23-retransmission-decision-matrix.md). Source freshness, development analysis and executable tests still block implementation; completed semantic review does not waive these gates.

### INT-047

**Title**: Inconsistent matrix observations remain counted but never parser coverage (Q0)

**Sources**:
- Ch11 §11.2.7.2 p. 11-50: “A single setup record may span multiple consecutive packets”; INT-019 and Ch6 §6.2.3.11 f require a defined checksum exclusion. Neither supplies a recovery rule for mutually inconsistent caller observations.
- Owner sign-off, Joey, 2026-11-25: [verbatim decision record](research/2026-11-25-retransmission-owner-signoffs.md). This is owner policy where the standard supplies no recovery/adoption rule.

**Behaviour**: Reject or hold inconsistent observations with named consistency/rejection tests. Keep every nominal matrix member listed and counted; never fabricate a realizable byte fixture or count rejection/consistency checks as parser coverage. R0 keeps an explicit conservative hold and an unavailable valid-input baseline, not a successful document verdict.

**Reason**: An exhaustive Cartesian product contains impossible observations; honest coverage separates their rejection from actual parser witnesses.

**Design**: approved (Joey, owner sign-off 2026-11-25; independent AI CONCUR at 73396bc) · **Review**: complete under ADR-0033; draft author: maintainer agent; owner sign-off: Joey, 2026-11-25; independent AI check: CONCUR on [PR #45 head 73396bc312e24cfccaf16de9ae7a72609faf75b6](https://github.com/TelemetryWorks/irig106-tmats/pull/45/commits/73396bc312e24cfccaf16de9ae7a72609faf75b6), reviewer @thread:thr_qsvwmtnn8f, approval relayed by the owner 2026-11-25; source freshness, development analysis and executable tests remain required before implementation · **Origin**: #44 matrix decisions, 2026-11-25

**Test**: `cut_observation_cannot_claim_mismatch_or_only_sha_layout_delta`, `matrix_inconsistent_observations_never_parser_coverage` (planned; not implemented or executed)

**Development**: intensive testing and deep analysis required · **Analysis**: not yet written

**Decision record**: [dated owner sign-off and alternatives](research/2026-11-25-retransmission-owner-signoffs.md); [explicit matrix](research/2026-11-23-retransmission-decision-matrix.md). Source freshness, development analysis and executable tests still block implementation; completed semantic review does not waive these gates.

### INT-048

**Title**: Field ambiguity forces assembly Ambiguous before identity or digest exceptions (QD)

**Sources**:
- 106-24R1 Ch9 §9.4.2 p. 9-2: “Each attribute appears in the file as a unique code name”; INT-039 establishes definition-bound scalar duplicates and retains every occurrence.
- Ch6 §6.2.3.11 f: “If the TMATS includes a G\SHA code name”; INT-021/F1 makes duplicates indeterminate and refuses stamping. These sources did not themselves settle assembly precedence.
- Owner sign-off, Joey, 2026-11-25: [verbatim decision record](research/2026-11-25-retransmission-owner-signoffs.md). This is owner policy where the standard supplies no recovery/adoption rule.

**Behaviour**: The owner overrides the prior deferred QD proposal NOW: a duplicate G\SHA **or other established field ambiguity forces assembly Ambiguous**, even without a related follower. Apply to each candidate independently. Byte identity, equality and a digest must NEVER upgrade that Ambiguous to Complete; QI/INT-046 is subordinate. Preserve every occurrence, raw byte, cut-text evidence and INT-045 finding. If cut text also has a known duplicate, aggregate Ambiguous records that ambiguity while retaining the cut facet; it never becomes usable Complete. R0 first rejects contradictory observations rather than inventing such facts. Unknown cardinality is not established field ambiguity; use INT-039's reviewed definitions. The former `qd_proposal` becomes the ordinary baseline, not an optional/deferred column. INT-044's labelled edition fallback does not resolve duplicate G\106 or cure this assembly hold; diagnostic validation may still use that labelled basis. F1's exact-item removal edit remains allowed; reassess the explicit edited revision before stamping.

**Retained-cut blocking rule (review clarification, 2026-11-25)**: An
aggregate Ambiguous outcome with a retained cut facet remains unusable and
ineligible to govern with Incomplete's FULL blocking force. Relabelling it
Ambiguous does not create an ambiguous-acceptance escape hatch. No option
that accepts an ambiguous split, including proposed INT-031 if later
approved, may make the cut text usable or governing. This amends the cut
wording in INT-034/045 by cross-reference, without repairing or discarding
any bytes, cut locations or findings. See the [exact-head review follow-up](research/2026-11-25-retained-cut-and-qm-review-followups.md).

**Reason**: The owner chose strict assembly safety immediately, overriding deferral. Integrity and field ambiguity now impose an assembly hold independently of time-continuation evidence; the prior less-strict baseline remains in #44 history.

**Design**: approved (Joey, owner sign-off 2026-11-25; independent AI CONCUR at 73396bc) · **Review**: complete under ADR-0033; draft author: maintainer agent; owner sign-off: Joey, 2026-11-25; independent AI check: CONCUR on [PR #45 head 73396bc312e24cfccaf16de9ae7a72609faf75b6](https://github.com/TelemetryWorks/irig106-tmats/pull/45/commits/73396bc312e24cfccaf16de9ae7a72609faf75b6), reviewer @thread:thr_qsvwmtnn8f, approval relayed by the owner 2026-11-25; source freshness, development analysis and executable tests remain required before implementation · **Origin**: #44 matrix decisions, 2026-11-25

**Test**: `duplicate_sha_without_related_follower_is_ambiguous`, `identical_duplicate_sha_never_verifies`, `cut_duplicate_keeps_cut_evidence_and_ambiguity`, `cut_facet_blocks_ambiguous_acceptance`, `other_field_ambiguity_blocks_identity_and_digest`, `following_duplicate_is_assembly_ambiguous` (planned; not implemented or executed)

**Development**: intensive testing and deep analysis required · **Analysis**: not yet written

**Decision record**: [dated owner sign-off and alternatives](research/2026-11-25-retransmission-owner-signoffs.md); [explicit matrix](research/2026-11-23-retransmission-decision-matrix.md). Source freshness, development analysis and executable tests still block implementation; completed semantic review does not waive these gates.

### INT-049

**Title**: Identity does not automatically extend across candidate boundaries (QB/QE)

**Sources**:
- Ch11 §11.2.1.1 f p. 11-8: “Each channel in a session shall have its own sequence counter”; §11.2.7.2 p. 11-50 gives segmentation sequence n+1. INT-011 joins per channel, not across caller input boundaries.
- Owner sign-off, Joey, 2026-11-25: [verbatim decision record](research/2026-11-25-retransmission-owner-signoffs.md). This is owner policy where the standard supplies no recovery/adoption rule.

**Behaviour**: No automatic extension of identity/adoption across channels, FRMT/RCCVER change, sequence gaps, explicit same-channel stops or actual EOF. Retain raw identity as information only. EOF comparisons are cross-input comparisons, never bytes secretly joined after EOF. No guessed governing setup follows from the comparison.

**Reason**: Raw equality does not establish shared scope, format interpretation or a continuous complete record.

**Design**: approved (Joey, owner sign-off 2026-11-25; independent AI CONCUR at 73396bc) · **Review**: complete under ADR-0033; draft author: maintainer agent; owner sign-off: Joey, 2026-11-25; independent AI check: CONCUR on [PR #45 head 73396bc312e24cfccaf16de9ae7a72609faf75b6](https://github.com/TelemetryWorks/irig106-tmats/pull/45/commits/73396bc312e24cfccaf16de9ae7a72609faf75b6), reviewer @thread:thr_qsvwmtnn8f, approval relayed by the owner 2026-11-25; source freshness, development analysis and executable tests remain required before implementation · **Origin**: #44 matrix decisions, 2026-11-25

**Test**: `identity_outside_guard_is_information_only`, `EOF_comparison_never_rejoins_inputs` (planned; not implemented or executed)

**Development**: intensive testing and deep analysis required · **Analysis**: not yet written

**Decision record**: [dated owner sign-off and alternatives](research/2026-11-25-retransmission-owner-signoffs.md); [explicit matrix](research/2026-11-23-retransmission-decision-matrix.md). Source freshness, development analysis and executable tests still block implementation; completed semantic review does not waive these gates.

### INT-050

**Title**: Raw changes do not invent semantic resets or resolve incomplete comparisons (QC/QP)

**Sources**:
- Ch9 §9.4.2 pp. 9-2–3 permits attribute ordering and defines framing; Ch6 §6.2.3.11 f pp. 6-22–23 hashes raw bytes with the stated exclusion. Neither specifies a consumer reset on a raw-only change.
- Owner sign-off, Joey, 2026-11-25: [verbatim decision record](research/2026-11-25-retransmission-owner-signoffs.md). This is owner policy where the standard supplies no recovery/adoption rule.

**Behaviour**: Report the raw change subtype (SHA-value-only, layout/whitespace-only, independently established semantic difference, or other raw difference), including discrepancies against SRCC=0. Do not speculate about semantic equivalence, a time-correlation reset or adoption. An incomplete following comparison remains unresolved; a matching prefix is not whole-copy identity.

**Reason**: Distinct bytes need not mean changed time semantics, and cut input cannot prove an unchanged complete copy.

**Design**: approved (Joey, owner sign-off 2026-11-25; independent AI CONCUR at 73396bc) · **Review**: complete under ADR-0033; draft author: maintainer agent; owner sign-off: Joey, 2026-11-25; independent AI check: CONCUR on [PR #45 head 73396bc312e24cfccaf16de9ae7a72609faf75b6](https://github.com/TelemetryWorks/irig106-tmats/pull/45/commits/73396bc312e24cfccaf16de9ae7a72609faf75b6), reviewer @thread:thr_qsvwmtnn8f, approval relayed by the owner 2026-11-25; source freshness, development analysis and executable tests remain required before implementation · **Origin**: #44 matrix decisions, 2026-11-25

**Test**: `srcc0_different_is_reported_change`, `raw_change_subtype_never_resets_time`, `partial_copy_never_proves_identity` (planned; not implemented or executed)

**Development**: intensive testing and deep analysis required · **Analysis**: not yet written

**Decision record**: [dated owner sign-off and alternatives](research/2026-11-25-retransmission-owner-signoffs.md); [explicit matrix](research/2026-11-23-retransmission-decision-matrix.md). Source freshness, development analysis and executable tests still block implementation; completed semantic review does not waive these gates.

### INT-051

**Title**: Prominent integrity findings are the default, subject to eligibility guards (QH/QG)

**Sources**:
- Ch6 §6.2.3.11 f pp. 6-22–23: 'all text between the "G\SHA" and the following semicolon, inclusive'; Ch9 Table 9-2 MESSAGE DIGEST/CHECKSUM p. 9-13. These define checksum evidence, not an automatic quarantine/adoption policy.
- Ch11 §11.2.7.2 pp. 11-50–51 describes configuration and changed-data ordering; it does not select a governing setup for ambiguous scope.
- Owner sign-off, Joey, 2026-11-25: [verbatim decision record](research/2026-11-25-retransmission-owner-signoffs.md). This is owner policy where the standard supplies no recovery/adoption rule.

**Behaviour**: Use prominent integrity-finding mode by default for otherwise eligible candidates, consistently for initial and mid-session records. Integrity failure alone does not silently impose quarantine or preserve an old governing setup. Strict quarantine is opt-in and requires its own review before use. Do not waive F1, framing, trust or ambiguous scope: QD/INT-048 still forces Ambiguous and blocks new adoption. This is the default integrity policy after eligibility/applicability checks, not permission to guess which setup governs a packet, to rejoin records or to publish provisional output. Unresolved general governance contracts remain unresolved.

**Reason**: The owner accepted the reviewer-recommended default while preserving eligibility safeguards and a separate review gate for stricter caller modes.

**Design**: approved (Joey, owner sign-off 2026-11-25; independent AI CONCUR at 73396bc) · **Review**: complete under ADR-0033; draft author: maintainer agent; owner sign-off: Joey, 2026-11-25; independent AI check: CONCUR on [PR #45 head 73396bc312e24cfccaf16de9ae7a72609faf75b6](https://github.com/TelemetryWorks/irig106-tmats/pull/45/commits/73396bc312e24cfccaf16de9ae7a72609faf75b6), reviewer @thread:thr_qsvwmtnn8f, approval relayed by the owner 2026-11-25; source freshness, development analysis and executable tests remain required before implementation · **Origin**: #44 matrix decisions, 2026-11-25

**Test**: `integrity_findings_default_no_silent_quarantine`, `quarantine_opt_in_requires_review`, `integrity_mode_never_waives_framing_trust_or_ambiguity` (planned; not implemented or executed)

**Development**: intensive testing and deep analysis required · **Analysis**: not yet written

**Decision record**: [dated owner sign-off and alternatives](research/2026-11-25-retransmission-owner-signoffs.md); [explicit matrix](research/2026-11-23-retransmission-decision-matrix.md). Source freshness, development analysis and executable tests still block implementation; completed semantic review does not waive these gates.

### INT-052

**Title**: Unreadable SRCC never becomes an eligibility or governing inference (QS)

**Sources**:
- Ch11 §11.2.7.2 p. 11-50: “0 = Setup record configuration has not changed”; “1 = Setup record configuration has changed”. An unreadable bit establishes neither value.
- Owner sign-off, Joey, 2026-11-25: [verbatim decision record](research/2026-11-25-retransmission-owner-signoffs.md). This is owner policy where the standard supplies no recovery/adoption rule.

**Behaviour**: Keep independently proven equality as evidence when following SRCC is U. Withhold automatic eligibility or governing inference; never infer SRCC=0/1. QI can preserve an otherwise-Complete seal classification in its bounded scope, with the finding visible, but equality is not adoption permission and QD remains dominant.

**Reason**: Known raw equality and unknown wire metadata are different evidence; consumers must see both.

**Design**: approved (Joey, owner sign-off 2026-11-25; independent AI CONCUR at 73396bc) · **Review**: complete under ADR-0033; draft author: maintainer agent; owner sign-off: Joey, 2026-11-25; independent AI check: CONCUR on [PR #45 head 73396bc312e24cfccaf16de9ae7a72609faf75b6](https://github.com/TelemetryWorks/irig106-tmats/pull/45/commits/73396bc312e24cfccaf16de9ae7a72609faf75b6), reviewer @thread:thr_qsvwmtnn8f, approval relayed by the owner 2026-11-25; source freshness, development analysis and executable tests remain required before implementation · **Origin**: #44 matrix decisions, 2026-11-25

**Test**: `unreadable_srcc_identity_is_evidence_not_eligibility` (planned; not implemented or executed)

**Development**: intensive testing and deep analysis required · **Analysis**: not yet written

**Decision record**: [dated owner sign-off and alternatives](research/2026-11-25-retransmission-owner-signoffs.md); [explicit matrix](research/2026-11-23-retransmission-decision-matrix.md). Source freshness, development analysis and executable tests still block implementation; completed semantic review does not waive these gates.


### INT-053

**Title**: Later copies compare with the currently governing setup, never a guessed target (QM)

**Sources**:
- 106-24R1 Ch11 §11.2.7.2 pp. 11-50–51: “0 = Setup record configuration has not changed”; changed setup is committed “prior to any new or changed data packets”. The prose does not supply a universal adoption/ambiguity recovery algorithm.
- Ch11 §11.2.1.1 f p. 11-8: “Each channel in a session shall have its own sequence counter”; INT-011 joining, INT-045 closure, INT-039 document-local duplicates and INT-046/048/051/052 retain their separate contracts. Ch6 §6.2.3.11 f checksum verification does not establish governing identity.
- Joey's 2026-11-25 QM sign-off: [verbatim record](research/2026-11-25-qm-governing-comparison-owner-decision.md).

**Behaviour**: Compare each later setup copy with the **currently governing setup**: the original sealed record, or the latest copy that validly took control. Retain that candidate's identity and supporting adoption evidence. A copy known not to have taken control never becomes the comparison target. If whether it took control is unresolved (including changed/integrity-failing warn-mode or Ambiguous evidence where adoption is unresolved), report the target for subsequent copies as **unresolved**; never choose the old, newest, first or best-checksum copy. QD still blocks new adoption of field-ambiguous input; that block alone does not prove the actual governing recorder setup.

Compute raw equality with the immediately previous copy as **information only**, with its separate pair of identities. It must never change eligibility, governing or classification, including when the governing target is unresolved. Apply QI only to an independently established governing-target comparison with its own guards, not to previous-copy equality. Governing-target comparison remains unavailable while the target is unresolved; local framing/ambiguity findings remain reportable. A valid takeover changes the target only on separately established reviewed adoption evidence, not because this rule sees Complete, SRCC, a digest or equal bytes.

Preserve every earlier copy's located findings without overwrite. Earlier governing setups and their identities remain available for data already decoded under them; do not relabel those packets when a later setup takes control or becomes unresolved. QM specifies comparison and evidence retention, not all open INT-028/031 applicability or provisional-use rules.

**Reason**: The last received copy and the original seal can both differ from the governing setup. Guessing one would launder an unresolved change, while treating previous-copy equality as authority would make an unadopted copy control later interpretation. Retained history keeps earlier decoding auditable.

**Design**: approved (Joey, owner sign-off 2026-11-25; independent AI CONCUR at 73396bc) · **Review**: complete under ADR-0033; draft author: maintainer agent; owner sign-off: Joey, 2026-11-25; independent AI check: CONCUR on [PR #45 head 73396bc312e24cfccaf16de9ae7a72609faf75b6](https://github.com/TelemetryWorks/irig106-tmats/pull/45/commits/73396bc312e24cfccaf16de9ae7a72609faf75b6), reviewer @thread:thr_qsvwmtnn8f, approval relayed by the owner 2026-11-25; source freshness, development analysis and executable tests remain required before implementation · **Origin**: QM follow-up to PR #45, 2026-11-25

**Test**: `qm_B_then_D_uses_governing_target`, `qm_H_then_B_previous_identity_is_information_only`, `qm_third_copy_only_checksum_does_not_select_target`, `qm_changed_srcc_claims_never_select_target`, `qm_unresolved_governing_blocks_later_target`, `qm_ambiguous_copy_does_not_prove_old_target`, `qm_valid_takeover_updates_governing_target`, `qm_unresolved_then_valid_takeover_resolves_target`, `qm_unresolved_persists_through_non_takeover`, `qm_previous_identity_mutation_preserves_decisions`, `qm_findings_and_prior_decoding_provenance_are_persistent` (planned; not implemented or executed)

**Development**: intensive testing and deep analysis required · **Analysis**: not yet written

**Decision record**: [owner wording and bounded scope](research/2026-11-25-qm-governing-comparison-owner-decision.md); [sequence contract and named fixtures](research/2026-11-23-retransmission-decision-matrix.md#multiple-following-copies-owner-signed-qm-sequence-contract). The pairwise product stays unchanged; sequence coverage is separate, bounded and planned. Independent AI review is complete at the exact head cited above; source freshness, development analysis and executable verification remain required before implementation.


## Bounded time-view enable context and numeric field-value syntax

### INT-054

**Title**: Disabled TIMEIN entries remain enumerated with CHE context (Option A)

**Sources**:
- 106-24R1 Chapter 9 section 9.5.4 Table 9-4 CHANNEL ENABLE R-x\CHE-n, p. 9-49: "Indicate if source is enabled. Source must be enabled to generate data packets." Usage: "Allowed when: R\N > 0", "Required when: Allowed"; enumeration T True / F False.
- Same table CHANNEL DATA TYPE, p. 9-50: TIMEIN is IRIG Time Input. The packet-generation condition does not define a library enumeration/filter policy.
- Joey, owner decision, 2026-11-25: Option A; [dated decision record](research/2026-11-25-che-numeric-syntax-owner-decisions.md).

**Behaviour**: Still enumerate a confirmed TIMEIN entry whose own R-x\CHE-n
is disabled (F), with CHE exposed as located context. Never silently filter
it; callers may filter explicitly. Enabled (T), disabled, missing, duplicate
or invalid CHE does not replace CDT's TIMEIN classification. Missing CHE
uses existing absent/default-evidence rules, not fabricated enabled/disabled;
known scalar duplicates stay Ambiguous with every occurrence (INT-039/048),
and a demonstrated invalid CHE stays Invalid. Unavailable reviewed rules
retain INT-041's guard. Retain independent applicability/presence findings,
raw bytes, scope and setup qualifications. Enumeration is diagnostic, not
permission to govern, generate packets or bypass assembly ambiguity/cuts.

**Reason**: The owner chose caller-controlled filtering with visible context;
a recorder's enable declaration must not hide the setup information a
consumer needs to inspect.

**Design**: approved (Joey, Option A, 2026-11-25; independent AI final CONCUR at ebd5c98a297a2ad1391cabcb24985a8b789e88f1) · **Review**: complete under ADR-0033; draft author: maintainer agent; owner sign-off: Joey, 2026-11-25; independent AI final CONCUR on PR #51 at `ebd5c98a297a2ad1391cabcb24985a8b789e88f1`, reviewer @thread:thr_qsvwmtnn8f (Behaviour unchanged); source freshness, development analysis and executable tests remain required · **Origin**: owner CHE direction after #50

**Test**: `che_disabled_timein_is_still_enumerated`, `che_missing_duplicate_invalid_preserve_entry`, `che_caller_filter_is_explicit_and_nonmutating` (planned, not implemented or executed)

**Development**: intensive testing and deep analysis required · **Analysis**: not yet written

### INT-055

**Title**: Decimal-integer fields and numeric codes use bounded raw ASCII digits (Option A)

**Sources**:
- 106-24R1 Chapter 9 section 9.5.4 Table 9-4 NUMBER OF SOURCE BITS p. 9-33: "Range: 0 to 13"; TRACK NUMBER/CHANNEL ID p. 9-48: "Range: 1 to 65535"; NUMBER OF TRACKS/CHANNELS p. 9-32: "Range: 1 to 65536". These ranges do not supply a general lexical grammar or digit limit.
- Table 9-4 SECONDARY HEADER TIME FORMAT p. 9-51, TIME DATA TYPE FORMAT pp. 9-71–72 and TIME FORMAT p. 9-72 contain numeric-looking enumeration codes; TK1/CDT/SHTF/DSI/CHE applicability uses R\N > 0 (pp. 9-48–51).
- Section 9.4.1 p. 9-2 governs character projection (INT-036/037); section 9.4.2 p. 9-2 calls value blanks intentional (INT-041). The owner supplies the policy below, not a claim that the standard mandates it.
- Joey, owner decision, 2026-11-25: Option A; [dated decision record](research/2026-11-25-che-numeric-syntax-owner-decisions.md). INT-038 and INT-043 are related canonical-string decisions, not independent authority for this extension.

**Behaviour**: Owner-signed answers, Joey, 2026-11-25, Option A for all
three clarifications; [verbatim answers and exclusion inventory](research/2026-11-25-int055-owner-answers.md).
This applies ONLY to decimal-integer field values (NSB, TK1, R\N condition
operands, and other decimal-integer fields) and numeric-looking enumeration
codes from explicit reviewed enumeration tables, including TTF/SHTF/TFMT
0/1/2. A binary pattern is NEVER a numeric-looking enumeration code.
Floating-point, hexadecimal, binary (including bit strings, patterns and
masks), and ANY OTHER non-decimal representation are explicitly out of
scope and require their own reviewed rules; numeric-only spelling does not
bring an excluded representation into this rule. Choose dispatch from the
reviewed field/representation definition, never the bytes alone. Preserve
binary width/leading zeros; no decimal ten-digit limit, canonicalisation,
warning or range comparison applies. See the [complete exclusion inventory
and every-Range audit](research/2026-11-26-int055-binary-exclusions.md), which
extends the dated floating/hex list without rewriting its history.

Check the RAW value bytes, not the INT-036 projection: require a nonempty
run of ASCII digits, at most TEN before stripping zeros. Any control byte
(CR, LF, NUL, TAB or other) anywhere in the numeric value is Invalid;
INT-036 must not repair a number into usable data. Retain raw bytes and
independent character findings; high bytes remain INT-037 Invalid.
A leading blank makes EVERY decimal-integer value and numeric-looking code
unavailable under INT-041 uniformly, including NSB, TK1, R\N, SHTF, TTF
and numeric TFMT. Trailing/embedded blanks are Invalid. Retain independently
proved failures if the same value has multiple defects; leading-blank
unavailability never erases a proved control/high-byte/syntax failure.
Letter TFMT codes keep their existing enumeration path (`B`, or unavailable
` A`); this decision does not change all nonnumeric blank handling.

Signs (+/-), any other non-digit, empty values and more than ten digits are
Invalid; never repair, clip, wrap or trim. Check the limit before zero
stripping: eleven zeros are Invalid. Strip leading zeros for interpreted
comparison, retaining `0` for all-zero input, without changing source bytes.
Emit a located WARNING for accepted noncanonical spelling: `04` equals 4,
`02` matches code 2. Keep the warning if a later range/membership check fails.
Check the cited field range/enumeration after canonicalisation; out-of-range
is Invalid. Use canonical-string length/lexicographic comparison or u64;
u32 cannot represent all ten-digit values. Code-name indices and G\SHA's
separately reviewed structured grammar are unchanged. General text/name
projection remains INT-036; this is an explicit numeric-value exception.

An Invalid/unavailable R\N cannot be used to conclude R\N > 0: conditions
report cannot-evaluate, retaining operand and independent findings. Missing
and duplicate operands retain existing Missing/Ambiguous behavior; numeric
conversion cannot select a winner or trigger an absence default. Raw and
located binding remain available independently of local conversion.

**Reason**: The owner chose raw-byte strictness over the projection-first
recommendation, uniform leading-blank unavailability and decimal-only scope.
This preserves floating/hex/binary and other non-decimal representations
for separate review, avoids an arbitrary field-by-field blank exception set and never makes a damaged
number usable by omission. Zero-padding remains visible, bounded comparison,
not a source edit.

**Design**: approved (Joey, Option A answers, 2026-11-25; independent AI final CONCUR at ebd5c98a297a2ad1391cabcb24985a8b789e88f1) · **Review**: complete under ADR-0033; draft author: maintainer agent; owner sign-off: Joey, 2026-11-25, including all three clarification answers; independent AI final CONCUR on PR #51 at `ebd5c98a297a2ad1391cabcb24985a8b789e88f1`, reviewer @thread:thr_qsvwmtnn8f; source freshness, development analysis and executable tests remain required · **Origin**: owner numeric field-value direction after #50

**Test**: `numeric_ascii_syntax_and_digit_limit`, `numeric_zero_padding_keeps_warning_and_raw`, `numeric_ranges_follow_canonicalisation`, `numeric_codes_accept_02_without_changing_letters`, `numeric_leading_blank_keeps_int041_exception`, `numeric_rn_condition_never_guesses`, `numeric_projection_keeps_character_findings`, `numeric_exclusions_dispatch_by_definition` (planned, not implemented or executed)

**Development**: intensive testing and deep analysis required · **Analysis**: not yet written

**Review history**: The [earlier concerns](research/2026-11-25-che-numeric-independent-review.md) are retained as history; the [owner answers](research/2026-11-25-int055-owner-answers.md) settle the three choices. Final AI concurrence at #51 ebd5c98 is recorded in the [final review record](research/2026-11-26-int054-int055-final-concurrence.md). The eight-mask inventory correction does not change the concurred decimal-only behaviour. Source freshness, development analysis and executable tests remain required.


## Bounded caller-choice policy (owner answers, 2026-11-26)

### INT-056

**Title**: Bad target data has Strict, Warn-default and Quiet caller levels

**Sources**:
- 106-24R1 Chapter 9 section 9.5.4 Table 9-4 TRACK NUMBER/CHANNEL ID p. 9-48: "Range: 1 to 65535"; TIME FORMAT p. 9-72 supplies field definitions, not caller acceptance levels. INT-039/041 preserve duplicate/unavailable states. The levels and warning emission are OWNER policy.
- Joey, owner answers, 2026-11-26: [verbatim record](research/2026-11-26-caller-choice-owner-answers.md).

**Owner sign-off (verbatim; Joey, 2026-11-26)**:

> 1. The chosen target has bad data (Invalid, duplicate or unavailable ID/format): THREE caller-selectable levels.
>    - Strict: refuse the choice, with a typed reason.
>    - Warn (DEFAULT): accept as a labelled assumption, with all bad-data findings attached.
>    - Quiet: accept as a labelled assumption with NO warning findings emitted.
>    Even in Quiet:
>    - the result is still labelled a caller assumption;
>    - the target's own states (Invalid / duplicate / unavailable) stay as they are and are never upgraded;
>    - the selected level is recorded in the result, so suppression is visible to anyone inspecting it.
>    One fixture per level, for each bad-data kind.

**Owner clarification (verbatim; Joey, 2026-11-26; supersedes the initial Quiet-emission wording above)**:

> (1) A: Quiet suppresses EXACTLY the bad-data findings Warn would attach, of ANY severity. They stay readable from the target's field results and source diagnostics.
> (2) A: every Quiet result carries a marker 'suppressed: N bad-data findings' (N may be 0), so a Quiet result is never confused with clean data.

See the [dated clarification record](research/2026-11-26-int056-quiet-owner-answers.md).

**Behaviour**: For an otherwise eligible selected target with Invalid,
duplicate or unavailable ID/format, offer three caller-selectable levels.
Strict refuses with a typed bad-data reason. Warn is DEFAULT and accepts a
labelled caller assumption with all bad-data findings attached. Quiet accepts
a labelled caller assumption but suppresses EXACTLY the bad-data findings
Warn would attach, of ANY severity; unrelated findings are not suppressed
by this policy. The suppressed findings remain readable from the target's
field results AND source diagnostics; never delete or downgrade them there.

EVERY Quiet result carries the marker `suppressed: N bad-data findings`,
where N is the number of findings in that exact suppressed bad-data set;
N may be 0 and the marker must still appear. Count findings, not invalid
fields, severity categories or warning-only members. Do not suggest Quiet
means clean data. Keep the caller-assumption label, selected level and
original Invalid/Ambiguous/unavailable target states/reasons; no field or
association evidence is upgraded. Dependent results retain the level and
Quiet marker. Original bytes, field diagnostics and source findings remain
unchanged and inspectable. Target eligibility and approved-evidence
contradictions (INT-057/058) are independent refusals at all levels; neither
suppression nor its marker hides a typed refusal or makes a choice accepted.
Accepting a choice is not a usable-field or governing-setup claim.

**Reason**: The owner chose this explicit policy after the #52 independent review identified an unresolved caller-choice decision; these words do not claim the standard mandates the policy.

**Design**: approved (Joey, 2026-11-26; independent AI final CONCUR at 3fcbd2cb0e911d1bc23e4cc3a5b53d503e5d7e23) · **Review**: complete under ADR-0033; draft author: maintainer agent; owner sign-off: Joey, 2026-11-26, including both Quiet clarification answers; independent AI final CONCUR on PR #52 at `3fcbd2cb0e911d1bc23e4cc3a5b53d503e5d7e23`, reviewer @thread:thr_qsvwmtnn8f, relayed by the owner on merge; GitHub main merge commit `cd63e07a58d4d98de6832d8d0340f08ff1da7ff9`; source freshness, development analysis and executable tests remain required · **Origin**: #52 caller-choice owner answers

**Test**: `assumption_strict_invalid_target`, `assumption_warn_invalid_target`, `assumption_quiet_invalid_target`, `assumption_strict_duplicate_target`, `assumption_warn_duplicate_target`, `assumption_quiet_duplicate_target`, `assumption_strict_unavailable_target`, `assumption_warn_unavailable_target`, `assumption_quiet_unavailable_target`, `assumption_quiet_suppressed_count_zero`, `assumption_quiet_suppressed_count_nonzero`, `assumption_quiet_marker_survives_derived_result` (planned, not implemented or executed)

**Development**: intensive testing and deep analysis required · **Analysis**: not yet written

### INT-057

**Title**: Caller choice must be a scoped TIMEIN candidate in the same recorder

**Sources**:
- 106-24R1 Chapter 9 section 9.5.4 Table 9-4 CHANNEL DATA TYPE p. 9-50: "TIMEIN IRIG Time Input"; TRACK NUMBER/CHANNEL ID p. 9-48 identifies the channel; neither row chooses a caller override across recorder scopes. Same-recorder eligibility/refusal is OWNER policy.
- Joey, owner answers, 2026-11-26: [verbatim record](research/2026-11-26-caller-choice-owner-answers.md).

**Owner sign-off (verbatim; Joey, 2026-11-26)**:

> 2. A: refuse a choice that is not a scoped TIMEIN candidate in the same recorder (a non-TIMEIN entry, another recorder, or a nonexistent ID). Typed refusal, with a fixture.

**Behaviour**: Refuse with a typed reason any choice that is not an existing scoped
TIMEIN candidate in the same recorder as the queried data channel. This
includes a non-TIMEIN entry, another recorder or a nonexistent ID, regardless
of Strict/Warn/Quiet. Keep the query/setup/revision scope explicit; never
choose by equal numeric ID across recorders. A scoped TIMEIN entry can be
identified by its canonical entry identity even when its own ID/format data
is bad: apply INT-056 without fabricating a usable ID. This does not turn
an unresolvable/nonexistent ID-only selection into an existing candidate.

**Reason**: The owner chose this explicit policy after the #52 independent review identified an unresolved caller-choice decision; these words do not claim the standard mandates the policy.

**Design**: approved (Joey, 2026-11-26; independent AI final CONCUR at 3fcbd2cb0e911d1bc23e4cc3a5b53d503e5d7e23) · **Review**: complete under ADR-0033; draft author: maintainer agent; owner sign-off: Joey, 2026-11-26; independent AI final CONCUR on PR #52 at `3fcbd2cb0e911d1bc23e4cc3a5b53d503e5d7e23`, reviewer @thread:thr_qsvwmtnn8f, relayed by the owner on merge; GitHub main merge commit `cd63e07a58d4d98de6832d8d0340f08ff1da7ff9`; source freshness, development analysis and executable tests remain required · **Origin**: #52 caller-choice owner answers

**Test**: `assumption_refuses_non_timein_target`, `assumption_refuses_other_recorder`, `assumption_refuses_nonexistent_id` (planned, not implemented or executed)

**Development**: intensive testing and deep analysis required · **Analysis**: not yet written

### INT-058

**Title**: Only an individually approved evidence rule can contradict a caller choice

**Sources**:
- 106-24R1 Chapter 9 section 9.5.4 Table 9-4 TIME SOURCE p. 9-72: "Indicate the time source." It enumerates I/E/R/X categories, not a data-to-time channel field. RELEASE-0.1 records the absence of a cited association field. The required evidence-INT gate and refusal are OWNER policy.
- Joey, owner answers, 2026-11-26: [verbatim record](research/2026-11-26-caller-choice-owner-answers.md).

**Owner sign-off (verbatim; Joey, 2026-11-26)**:

> 3. A: refuse a choice that contradicts evidence. The owner wants the contradiction rules made explicit. State that a contradiction exists ONLY when an owner-approved evidence rule (its own INT) establishes a different association for that data channel. No evidence rule exists in 0.1, since Chapter 9 has no data-to-time field, so this cannot trigger yet. Every future evidence source (e.g. a later edition's field, or a reviewed packet-level rule) needs its own owner-signed INT defining what it proves. Heuristics (order, only candidate, TSRC, DSI, NSB, observed time packets) are never evidence.

**Behaviour**: Refuse a choice with a typed contradiction reason ONLY when an
owner-approved evidence rule, identified by its OWN INT, establishes a
DIFFERENT association for this data channel in the applicable scope. Each
future source (later-edition field, packet-level rule or other source) must
have its own owner-signed INT defining precisely what it proves, and pass
ADR-0033's independent check/readiness gates before implementation. There
is NO such evidence rule in 0.1, so this contradiction branch cannot trigger
yet. Order, only-candidate status, TSRC, DSI, NSB and observed time packets
are NEVER evidence. Do not create a contradiction from bad target data,
competing guesses or an unreviewed/unknown rule. This INT is an admission/
refusal policy, not itself an association-proof rule. It supplies no rule
for choosing among competing approved sources whose assertions are not
resolved; retain INT-059 uncertainty rather than invent a precedence.

**Reason**: The owner chose this explicit policy after the #52 independent review identified an unresolved caller-choice decision; these words do not claim the standard mandates the policy.

**Design**: approved (Joey, 2026-11-26; independent AI final CONCUR at 3fcbd2cb0e911d1bc23e4cc3a5b53d503e5d7e23) · **Review**: complete under ADR-0033; draft author: maintainer agent; owner sign-off: Joey, 2026-11-26; independent AI final CONCUR on PR #52 at `3fcbd2cb0e911d1bc23e4cc3a5b53d503e5d7e23`, reviewer @thread:thr_qsvwmtnn8f, relayed by the owner on merge; GitHub main merge commit `cd63e07a58d4d98de6832d8d0340f08ff1da7ff9`; source freshness, development analysis and executable tests remain required · **Origin**: #52 caller-choice owner answers

**Test**: `assumption_no_01_contradiction_rule`, `assumption_heuristics_never_contradict`, `assumption_future_evidence_requires_own_int` (planned, not implemented or executed)

**Development**: intensive testing and deep analysis required · **Analysis**: not yet written

### INT-059

**Title**: Unproven multiplicity is unestablished, conflicting requires evidence

**Sources**:
- 106-24R1 Chapter 9 section 9.5.4 Table 9-4 CDT p. 9-50 and TTF/TFMT/TSRC pp. 9-71–72 supply local metadata, not a candidate-count classification rule. The association-state definitions and this clarification are OWNER policy in RELEASE-0.1.
- Joey, owner answers, 2026-11-26: [verbatim record](research/2026-11-26-caller-choice-owner-answers.md).

**Owner sign-off (verbatim; Joey, 2026-11-26)**:

> 4. A: several unproven candidates means "unestablished"; "conflicting" is reserved for competing evidence.

**Behaviour**: Several unproven candidates are association_unestablished, not
association_conflicting. Reserve conflicting for competing evidence; retain
all candidates, evidence, scope and reasons without picking a winner. Since
0.1 has no approved association-evidence rule, candidate multiplicity and
heuristic observations cannot produce conflicting/evidenced in production.
Composition tests may receive a preclassified conflicting state to check
retention; they do not implement or claim an actual baseline evidence path.

**Reason**: The owner chose this explicit policy after the #52 independent review identified an unresolved caller-choice decision; these words do not claim the standard mandates the policy.

**Design**: approved (Joey, 2026-11-26; independent AI final CONCUR at 3fcbd2cb0e911d1bc23e4cc3a5b53d503e5d7e23) · **Review**: complete under ADR-0033; draft author: maintainer agent; owner sign-off: Joey, 2026-11-26; independent AI final CONCUR on PR #52 at `3fcbd2cb0e911d1bc23e4cc3a5b53d503e5d7e23`, reviewer @thread:thr_qsvwmtnn8f, relayed by the owner on merge; GitHub main merge commit `cd63e07a58d4d98de6832d8d0340f08ff1da7ff9`; source freshness, development analysis and executable tests remain required · **Origin**: #52 caller-choice owner answers

**Test**: `association_unproven_multiplicity_is_unestablished`, `association_conflict_requires_competing_evidence` (planned, not implemented or executed)

**Development**: intensive testing and deep analysis required · **Analysis**: not yet written

### INT-060

**Title**: Each query supplies its caller choice explicitly

**Sources**:
- 106-24R1 Chapter 9 section 9.5.4 Table 9-4 defines setup metadata, not stateful caller-choice defaults. RELEASE-0.1 labels caller choices as assumptions and binds them to a record/revision; per-query explicitness is OWNER policy.
- Joey, owner answers, 2026-11-26: [verbatim record](research/2026-11-26-caller-choice-owner-answers.md).

**Owner sign-off (verbatim; Joey, 2026-11-26)**:

> 5. A: no remembered or session-wide default; each query carries the choice explicitly.

**Behaviour**: No remembered or session-wide default choice. Each query carries its
choice explicitly, even within the same recording/setup/revision. A prior
accepted choice cannot supply a missing choice in the next query. Dependent
answers keep provenance of their own explicit query, and an independent
unassumed read stays unassumed. INT-056's Warn DEFAULT is a level default,
not a default target; it never supplies a missing choice.

**Reason**: The owner chose this explicit policy after the #52 independent review identified an unresolved caller-choice decision; these words do not claim the standard mandates the policy.

**Design**: approved (Joey, 2026-11-26; independent AI final CONCUR at 3fcbd2cb0e911d1bc23e4cc3a5b53d503e5d7e23) · **Review**: complete under ADR-0033; draft author: maintainer agent; owner sign-off: Joey, 2026-11-26; independent AI final CONCUR on PR #52 at `3fcbd2cb0e911d1bc23e4cc3a5b53d503e5d7e23`, reviewer @thread:thr_qsvwmtnn8f, relayed by the owner on merge; GitHub main merge commit `cd63e07a58d4d98de6832d8d0340f08ff1da7ff9`; source freshness, development analysis and executable tests remain required · **Origin**: #52 caller-choice owner answers

**Test**: `assumption_next_query_needs_explicit_choice`, `assumption_warn_default_is_not_target_default` (planned, not implemented or executed)

**Development**: intensive testing and deep analysis required · **Analysis**: not yet written


### INT-061

**Title**: X occurrence mapping when the original has no numeric index

**Owner sign-off**: Joey, 2026-11-28, under ADR-0033; [verbatim decision](research/2026-11-28-interpretation-owner-signoffs.md#int-061).

**Sources**:
- 106-24R1 Chapter 9 section 9.5.14, p. 9-172, prints `X-x\ ORGANIZATION \ORIGCODE\EXTENSION_CODE-i-j-m-n:Value;`.
- Same section: “Everything to the right of ORGANIZATION that matches an existing TMATS code is used to associate the extension with an existing object defined by the TMATS file.”
- Same section: “The EXTENSION_CODE identifies the specific extension and shall be unique (i.e., not overlapping any existing TMATS code name).” Registry-name matching plus this uniqueness rule determines the reviewed ORIGCODE/EXTENSION_CODE decomposition; missing metadata does not authorize a heuristic. What “existing TMATS code” means is separately owner-signed INT-062, not silently decided here.
- Same section: “The ORIGCODE contains the original group identifier (i.e., G,D,P, etc.)” and “The value of “-x” must match the first level index (P-x, etc.) value”.
- Same section: “The “x” values must match the index of the original code word therefore no new values may be added.”
- Table 9-2 PROGRAM NAME, p. 9-10, prints code name `G\PN`, with no numeric index. Section 9.5.14's worked example uses indexed `D-1\MN-1-2`, not an unindexed original.
- ADR-0002/0008/0026 preserve raw extensions and require evidence for attachment; no invented source index or default is authorized.

**Behaviour (owner-signed Option A)**:
When an original has no numeric index, retain the X item and any plausible
raw original evidence but report the occurrence mapping cannot-evaluate,
with its missing-rule reason; do not report resolved or proven no-target,
invent zero/one, infer an index from data values/order, or rewrite the X name.
Ordinary indexed association keeps its separately sourced rules. This policy
does not claim the standard forbids all extensions of unindexed attributes;
it declines to invent the absent occurrence mapping.

**Options considered (owner selected A, 2026-11-28)**:
- **A — recommend:** leave the mapping unavailable until a separately reviewed representation is established. Raw data remains accessible; no fabricated match enters a consumer's model.
- **B — explicit policy:** define an owner-chosen representation for unindexed targets (for example a labelled sentinel/omitted-occurrence convention) before supporting them. This could enable such attachments but would add project policy not printed in section 9.5.14 and needs its own exact syntax, collision and compatibility tests; no representation is chosen here.
- **C — natural future refinement:** for a once-per-document G attribute such as G\PN, attach by a unique reviewed path when exactly one source occurrence exists, with a labelled attachment and a located finding that occurrence x cannot be verified. No sentinel is needed. This is plausible because G is explicitly listed and the text speaks of an existing object. Duplicates are still ambiguous, record/cut qualifications stay attached, and exact public label wording remains behind the report-format decision. C is not approved here.
- **D — considered and rejected in this draft recommendation:** classify the X as nonconforming/malformed with a located finding because the required index comparison cannot be satisfied. This is the strict reading of the index sentence, but the explicit G listing makes a blanket rejection of such extensions poorly supported; raw bytes would still be kept. D is not selected; A is the owner-signed recovery rule, not a standards ban.

A is reversible: later approved B/C can refine missing-rule cannot-evaluate to resolved where their guards hold, retaining raw bytes/evidence without a breaking source/result-shape change. This does not pre-approve B/C or bypass normal published API/report compatibility review. A is selected now; C remains a possible future refinement, not approved.

**Reason**: The X form/index-match rule assumes an index to compare; its broad original-group description includes G, whose PROGRAM NAME has none. The cited text does not supply a sentinel or omitted-index form. Guessing one can create a false attachment or hide an orphan. Prefer an explicit unavailable result over claiming a standards-defined mapping.

**Design**: approved by owner (Joey, 2026-11-28) · **Review**: complete under ADR-0033 for the signed parts; draft author: maintainer agent; owner sign-off: Joey, 2026-11-28; independent AI FINAL CONCUR at [PR #87 head `e0a0ccea5a61c64f7de9ca0e122adb01f376a3f9`](https://github.com/TelemetryWorks/irig106-tmats/pull/87/commits/e0a0ccea5a61c64f7de9ca0e122adb01f376a3f9), reviewer @thread:thr_qsvwmtnn8f, received 2026-11-28; [final-check record](research/2026-11-28-int022-066-final-concurrences.md); source freshness, written development analysis/review and intensive executable tests remain required · **Origin**: family 2 batch 30 source inspection

**Test**: `x_unindexed_original_mapping_unavailable`, `x_unindexed_zero_one_never_default`, `x_unindexed_value_never_supplies_occurrence` (planned Option A expectations), `x_unindexed_unique_path_option_c`, `x_unindexed_duplicate_path_option_c`, `x_unindexed_nonconforming_option_d` (non-selected alternatives, not active expectations; selected A cases are planned, with final rule check recorded; development-analysis review/test gates retained)

**Development**: intensive testing and deep analysis required · **Analysis**: not yet written


### INT-062

**Title**: Meaning of existing TMATS code in X decomposition

**Owner sign-off**: Joey, 2026-11-28, under ADR-0033; [verbatim decision](research/2026-11-28-interpretation-owner-signoffs.md#int-062).

**Sources**:
- 106-24R1 Chapter 9 section 9.5.14, p. 9-172: “Everything to the right of ORGANIZATION that matches an existing TMATS code is used to associate the extension with an existing object defined by the TMATS file.”
- Same section: “The EXTENSION_CODE identifies the specific extension and shall be unique (i.e., not overlapping any existing TMATS code name).”
- Same section says ORIGCODE contains the original group identifier and original code, potentially more backslashes, and prints the D-1\MN-1-2 / X-1\MYORG\D\MN\SGAIN-1-2 example. Table 9-7 MEASUREMENT NAME p. 9-115 defines the original code-name pattern independently of whether an instance is present.
- ADR-0008/0026 preserve attachment and every candidate; the phrases “existing TMATS code” and “existing object defined by the TMATS file” leave vocabulary versus instance presence to review.

**Behaviour (owner-signed Option A)**:
Use the reviewed 106-24R1 BASELINE code-name registry until edition deltas
exist, recording that registry basis separately from the document's edition
and the INT-044-labelled selection/fallback basis. Do not claim historical
vocabulary coverage or translate an older renamed code into a baseline code.
Use that baseline as the decomposition vocabulary. Match the original group/full path there,
using its reviewed index arity. EXTENSION_CODE uniqueness against that
vocabulary makes a conforming split deterministic. Then search the supplied
document for the actual original occurrences as a separate step; a known
code with no instance is an orphan/unresolved target, not missing grammar.
If registry metadata is missing, overlap is established or the split cannot
be proved unique, retain raw evidence and cannot-evaluate; no greedy/
longest-prefix or document-presence heuristic fills the gap.

**Options considered (owner selected A, 2026-11-28)**:
- **A — recommend:** “existing code” means the reviewed standard code-name vocabulary (106-24R1 baseline until deltas exist), independent of document presence. This separates decomposition from target lookup and permits orphan reporting. **Cost:** X targeting vendor-private V or other non-registry codes can never be split under A and is always cannot-evaluate, even if the target is present. Standard V-x\ID/VN remain registry-defined; private V codes do not become standard vocabulary. Code-name/arity metadata still needs source review, not the full later value registry.
- **B:** “existing code” means usable code names present in the supplied document. The reference to an existing object in the file supports this reading and could admit document-defined/private codes, but the same X text could decompose differently when originals are missing/added; metadata and ambiguity rules still need review, and presence alone cannot invent unknown grammar or objects.
- **C — hybrid proposal:** try reviewed registry vocabulary first; use document-present code names ONLY for groups with no registry vocabulary, with the basis labelled `document-defined vocabulary`. This is not a fallback for every missing registry code. The owner must settle the group/private-namespace boundary: V has standard ID/VN codes but no standard private-item vocabulary, so no blanket V fallback is silently authorized. Missing/ambiguous grammar or nonunique decomposition still stays cannot-evaluate; no overlap, index, record-qualification or source-retention guard is waived.

**Registry edition/basis clarification (coordinator 2026-11-27; owner signed 2026-11-28)**:
Use the 106-24R1 baseline until edition deltas exist, with the registry basis
visible alongside existing INT-044 edition-selection/fallback labelling.
This does not overwrite G\106 or relabel an older declared edition as 24R1.
Older renamed originals lacking baseline metadata (the supplied R\DST to
R\CDT example) are cannot-evaluate, never silently aliased/repaired. Exact
historical edition mappings still need their own cited delta sources.

**Reason**: Registry matching and EXTENSION_CODE uniqueness provide the deterministic decomposition described by the section. The remaining vocabulary-scope question must be an explicit owner choice. A is owner-signed; no namespace, private-code or edition fallback policy is silently added.

**Design**: approved by owner (Joey, 2026-11-28) · **Review**: complete under ADR-0033 for the signed parts; draft author: maintainer agent; owner sign-off: Joey, 2026-11-28; independent AI FINAL CONCUR at [PR #87 head `e0a0ccea5a61c64f7de9ca0e122adb01f376a3f9`](https://github.com/TelemetryWorks/irig106-tmats/pull/87/commits/e0a0ccea5a61c64f7de9ca0e122adb01f376a3f9), reviewer @thread:thr_qsvwmtnn8f, received 2026-11-28; [final-check record](research/2026-11-28-int022-066-final-concurrences.md); source freshness, written development analysis/review and intensive executable tests remain required · **Origin**: PR #63 independent review follow-up

**Test**: `x_registry_decomposition_survives_absent_target`, `x_document_only_code_does_not_define_registry_split`, `x_extension_overlap_prevents_heuristic_split`, `x_private_target_registry_only_cost`, `x_baseline_registry_keeps_edition_basis`, `x_legacy_renamed_original_is_unavailable` (planned Option A/baseline expectations), `x_hybrid_no_vocabulary_uses_labelled_document_names`, `x_hybrid_existing_vocabulary_never_falls_back`, `x_hybrid_v_private_boundary_needs_owner` (non-selected C inputs now assert A refusal/unavailability; final rule check recorded; development-analysis review/test gates retained)

**Development**: intensive testing and deep analysis required · **Analysis**: not yet written


### INT-063

**Title**: Vendor attribution when V header and private path evidence are incomplete or inconsistent

**Owner sign-off**: Joey, 2026-11-28, under ADR-0033; [verbatim decision](research/2026-11-28-interpretation-owner-signoffs.md#int-063).

**Sources**:
- 106-24R1 Chapter 9 section 9.5.13 p. 9-171: “The only V-group attributes defined in this standard are the following.” Data Source ID is V-x\ID; Vendor Name is V-x\VN.
- Same section says Vendor Name is “a three-character acronym that identifies the specific vendor and determines how the rest of the attributes in the V group are interpreted.” It gives all other names as `V-x\acr\attribute-string`, where “acr is the three-character acronym identifying a specific vendor”.
- Table 9-2 DATA SOURCE ID G\DSI-n p. 9-11: “Provide a descriptive name for this source. Each source identifier must be unique.” Its Links to field includes V-x\ID.
- ADR-0008 accepts source/vendor grouping; ADR-0002/0026 retain evidence without a winner; INT-039/048 preserve known scalar multiplicity and record ambiguity. The cited text does not define missing/duplicate/mismatched VN recovery.

**Usable VN (amended definition, 2026-11-27)**:
A usable VN is one present, non-Ambiguous candidate with available INT-041
blank/availability and INT-037 character interpretation, whose INT-036-projected
value has exactly THREE characters (section 9.5.13 p. 9-171). Compare it
with the path acronym ASCII case-insensitively under section 9.4.2 p. 9-2;
no locale/Unicode/high-byte folding, raw trimming or source rewrite. INT-036
provides projection, not case folding. An available projected value of the
wrong length is Invalid, leaving vendor attribution unconfirmed under A;
do not downgrade that known length failure to generic unavailable. If blank
handling itself is unreviewed/unavailable under INT-041, keep that reason
rather than trim blanks or manufacture validity/invalidity by an unreviewed
comparison. Missing, repeated, Invalid or unavailable VN is not usable;
equal duplicate spellings cannot erase Ambiguous. Raw bytes/findings remain.
No additional alphabet rule or INT-055 numeric syntax is imposed on VN.

**Behaviour (owner-signed Option A)**:
Always retain observed V occurrence/private-acronym namespaces and every
local ID/VN result. Confirm vendor attribution only when one usable VN
agrees with the observed path acronym under reviewed ASCII comparison;
otherwise retain unavailable confirmed attribution with its missing/conflict
reason and locations. Do not move raw items, repair a header, pick a duplicate
or invent required-presence/default findings from this policy. Source ID
context and its G-source candidate resolution stay separate from vendor
confirmation. Private meanings remain opaque.

**Options considered (owner selected A, 2026-11-28)**:
- **A — recommend:** retain structural inventory for every namespace, but withhold confirmed vendor attribution if VN evidence is missing, repeated, invalid/unavailable or disagrees. This avoids assigning unknown private fields to a vendor on incomplete/conflicting evidence; raw inspection still works.
- **B — path-first:** use the observed path acronym as labelled vendor attribution despite bad/missing/disagreeing VN, retaining all header findings/states. This makes more raw namespaces attributable but chooses path authority that the recovery text does not specify.
- **C — usable-header-first:** one usable VN determines semantic vendor attribution even when path acronyms disagree, with labels/findings and raw namespace unchanged; absent/ambiguous/unavailable VN still cannot pick a vendor. The VN definition supports this emphasis, but it can misattribute a mixed-vendor occurrence. This C alternative is not selected; exact public wording remains open.

**Reason**: Section 9.5.13 provides two pieces of vendor evidence but no recovery precedence. Reporting the raw grouping separately from confirmed attribution preserves useful data without treating conflicting writers as an authority. All options keep source/field/record evidence; A is owner-signed; B/C are not selected.

**Design**: approved by owner (Joey, 2026-11-28) · **Review**: complete under ADR-0033 for the signed parts; draft author: maintainer agent; owner sign-off: Joey, 2026-11-28; independent AI FINAL CONCUR at [PR #87 head `e0a0ccea5a61c64f7de9ca0e122adb01f376a3f9`](https://github.com/TelemetryWorks/irig106-tmats/pull/87/commits/e0a0ccea5a61c64f7de9ca0e122adb01f376a3f9), reviewer @thread:thr_qsvwmtnn8f, received 2026-11-28; [final-check record](research/2026-11-28-int022-066-final-concurrences.md); source freshness, written development analysis/review and intensive executable tests remain required · **Origin**: family 2 batch 31 source inspection

**Test**: `v_missing_vn_keeps_namespace_unconfirmed`, `v_duplicate_vn_never_picks_vendor`, `v_path_header_mismatch_keeps_both`, `v_invalid_unavailable_vn_keeps_namespace`, `v_vn_projected_three_chars_ascii_match`, `v_vn_wrong_length_is_invalid_unconfirmed`, `v_vn_blank_handling_does_not_trim`, `v_vn_high_byte_is_not_usable` (planned owner-signed A/definition expectations; final rule check recorded; development-analysis review/test gates retained)

**Development**: intensive testing and deep analysis required · **Analysis**: not yet written


## Explicit normalization policy drafts

Joey signed A+D / A / A for INT-064/065/066 on 2026-11-28 under ADR-0033.
L1-WRT-002 and UC-09 retain explicit normalization in 0.1; no code or
completed analysis/testing is claimed. The [source/options note](research/2026-11-27-normalization-policy-drafts.md)
records the boundary. Selected-policy fixture expectations are now owner-decided; development gates remain.

### INT-064

**Title**: Ordering policy for explicitly requested normalization

**Owner sign-off**: Joey, 2026-11-28, under ADR-0033; [verbatim decision](research/2026-11-28-interpretation-owner-signoffs.md#int-064).

**Sources**:
- 106-24R1 Ch9 section 9.4.2 p. 9-2: “Attributes may appear in any order.”
- Ch9 sections 9.5.2–9.5.14, in their printed section order: G, T, R, M, P, D, B, S, Q, C, H, V, X. This supplies a possible project profile sequence, not a mandated serialized order.
- ADR-0002/0007 and L1-WRT-002/UC-09 require preserved source by default and explicit normalization; none defines a standard canonical comparator.

**Behaviour (owner-signed A mechanism plus named D profile)**: Validate the
selected complete-region ordering plan as a bijection over one document/
revision and replay whole raw regions without changing name/value bytes.
Explicitly retained prefix/EOF material stays anchored. If the partition or
mapping is unavailable, refuse rather than invent boundaries. No implicit
normalization, deduplication, case/counter rewrite or insertion is authorized.
The owner-selected named built-in **Chapter 9 group order, v1** profile generates
such a plan using D below; both the mechanism and profile are owner-signed,
with development gates retained.
No profile is automatically selected by default byte-faithful serialization.

**Options considered (owner selected A+D, 2026-11-28)**:
- **A — explicit caller-selected complete-region plan.** A is the validated
  permutation mechanism, not a built-in canonical order by itself.
- **B — full-key sort.** Built-in stable projected-key tuple ordering:
  ASCII-folded group/path, canonical numeric indices by length then digits,
  source-order ties. Standalone comments and opaque/malformed regions are
  anchored barriers; each region retains its leading layout. It gives more
  within-group ordering but has more comparison/tie/placement rules.
- **C — postpone built-in ordering.** Wait for an approved fixed profile,
  retaining exact replay meanwhile. This delays the required capability;
  it cannot be counted as satisfying L1-WRT-002's canonical-order obligation.
- **D — group-order profile (preferred built-in).** Within each reorderable
  run between anchors, use Ch9 section group order G, T, R, M, P, D, B, S,
  Q, C, H, V, X, retaining source order within each group. Standalone COMMENT
  and opaque/malformed regions are anchored as in B; unknown-group regions
  anchor, never acquire a guessed group. Complete positively grouped V/X
  regions retain stable within-group placement and every private byte,
  without decoding private meaning/attachment; if group/region classification
  is unavailable, anchor it. This placement rule is included in the signed profile and does not resolve INT-061/062/063. Anchors can prevent
  global group order across the whole file; no crossing them is implied.

**L1-WRT-002 consequence and recommendation**: **A alone has no built-in
canonical order and does not by itself fully satisfy the current L1.** The
owner must either approve one named built-in B or D profile for 0.1 alongside
A (**A + D recommended**), or explicitly reinterpret canonical order as a
caller-selected identified profile and clarify L1 wording. Joey selected A+D on 2026-11-28; no L1 reinterpretation is needed. D is a less disruptive built-in than full-key
sorting, with fewer tie rules and source order retained inside each group.
Its costs are a project convention rather than a standard-mandated order,
explicit unknown/private placement rules, anchor-limited group ordering and
no cleanup of within-group order. A remains useful for caller-specific plans;
neither it nor D may be advertised as the standard's unique canonical form.

**Design**: approved by owner (Joey, 2026-11-28) · **Review**: owner sign-off recorded under ADR-0033; draft author: maintainer agent; Independent amended recheck CONCUR with A mechanism + named D at `283ffd5` (2026-11-27). Source freshness, written development analysis/review and intensive executable tests remain required · **Origin**: family 2 normalization source review and #77 independent review

**Test**: `normalize_order_explicit_bijection`, `normalize_order_missing_or_repeated_identity`, `normalize_order_preserves_raw_regions`, `normalize_group_profile_section_order`, `normalize_group_profile_stable_within_group`, `normalize_group_profile_anchors`, `normalize_group_profile_private_and_unknown_placement` (planned owner-signed A+D expectations; development/test gates retained)

**Development**: intensive testing and deep analysis required · **Analysis**: not yet written

### INT-065

**Title**: Scope of line-ending normalization

**Owner sign-off**: Joey, 2026-11-28, under ADR-0033; [verbatim decision](research/2026-11-28-interpretation-owner-signoffs.md#int-065).

**Sources**:
- 106-24R1 Ch9 section 9.4.1 p. 9-2 permits LF/CR to “improve readability”. Section 9.4.2 defines field delimiters and intentional value blanks, not a canonical newline representation.
- INT-035/036 distinguish retained bytes, layout and interpreted omission; L1-WRT-002 asks for consistent line endings only on request.

**Behaviour (owner-signed A)**: Require the caller to choose LF or CRLF, with no
implicit platform default. Convert CRLF pairs, lone CR and lone LF only in
source-correspondence ranges positively established as layout, outside name/
value content. Leading/trailing name layout may be included only where its
layout role is established; internal name controls, value bytes, opaque
regions and NUL/0x1A padding are not silently converted or discarded. Unknown
layout classification makes the requested conversion unavailable there,
not permission to guess or claim globally consistent output. CRLF is one
newline token; never process its CR and LF as two tokens.

**Options considered (owner selected A, 2026-11-28)**: A — explicit target and positively identified layout-only ranges
(recommended); B — convert every raw CR/LF token in the selected source,
including names/values, preserving all other bytes; C — require the caller
to supply every exact conversion range rather than discover layout.

**Reason**: A avoids changing COMMENT/description or damaged field bytes under
a formatting request, but cannot claim a global raw-byte newline convention
when field content is retained. B gives that global convention but changes
field source; C is precise but transfers all layout work to callers. This
is a product policy choice, not a new delimiter or an INT-036 view rewrite.

**Design**: approved by owner (Joey, 2026-11-28) · **Review**: complete under ADR-0033 for the signed parts; draft author: maintainer agent; owner sign-off: Joey, 2026-11-28; independent AI FINAL CONCUR at [PR #87 head `e0a0ccea5a61c64f7de9ca0e122adb01f376a3f9`](https://github.com/TelemetryWorks/irig106-tmats/pull/87/commits/e0a0ccea5a61c64f7de9ca0e122adb01f376a3f9), reviewer @thread:thr_qsvwmtnn8f, received 2026-11-28; [final-check record](research/2026-11-28-int022-066-final-concurrences.md); source freshness, written development analysis/review and intensive executable tests remain required · **Origin**: family 2 normalization source review

**Test**: `normalize_newlines_layout_only`, `normalize_newlines_field_controls_untouched`, `normalize_newlines_unknown_layout_unavailable` (planned owner-signed A expectations; development/test gates retained)

**Development**: intensive testing and deep analysis required · **Analysis**: not yet written

### INT-066

**Title**: Preconditions for explicit counter recomputation

**Owner sign-off**: Joey, 2026-11-28, under ADR-0033; [verbatim decision](research/2026-11-28-interpretation-owner-signoffs.md#int-066).

**Sources**:
- 106-24R1 Ch9 Table 9-7 NUMBER OF FRAGMENTS, D-x\MNF\N-y-n-m, p. 9-117 defines the number of occupied word positions, range 1–8; WORD POSITION D-x\WP-y-n-m-e identifies the members. The source calls these “Number of word positions”, not highest index.
- Approved INT-038/040 supply canonical numeric identity, scoped membership and compatible-unassigned evidence; INT-055 supplies bounded decimal/range rules. They do not authorize rewriting counters.

**Behaviour (owner-signed A)**: Recompute only an existing uniquely identified,
well-framed reviewed counter whose applicability, parent scope and complete
member evidence are established, with contiguous distinct identities 1..k
and no compatible unassigned evidence. Write canonical decimal k only when
its cited field range accepts it; never renumber children, insert missing
counters/members or guess opaque X/private counts. A malformed old counter
VALUE may be replaced by this explicit counter operation when its physical
target and independent member evidence are established; that is not a
successful validation of the old counter or an implicit general repair.
Unknown evidence, duplicate target, gap or out-of-range result refuses the
requested counter rewrite; it does not erase the original.

**Options considered (owner selected A, 2026-11-28)**: A — complete, contiguous evidence with range-checked count
(recommended); B — write distinct observed cardinality even with index gaps,
retaining gap/excess findings and never claiming repaired conformance;
C — write highest observed index. C is listed and rejected in the draft
recommendation: it confuses an index bound with the printed member count.

**Reason**: A avoids concealing truncation or making a counter look repaired
while holes remain; it requires refusal where B can still provide a partial
explicit correction. B is more permissive but leaves a knowingly inconsistent
index domain. Equal raw spelling variants count once; duplicate occurrences
and invalid member VALUES remain visible under INT-038/039/040.
The same identity/retention rule applies to reviewed MML\N-style counters,
not only MNF\N; use each counter's own cited scope/range (Table 9-7 NUMBER
OF MEASUREMENT LOCATIONS, p. 9-117), never import MNF\N's 1–8 range.

**Design**: approved by owner (Joey, 2026-11-28) · **Review**: complete under ADR-0033 for the signed parts; draft author: maintainer agent; owner sign-off: Joey, 2026-11-28; independent AI FINAL CONCUR at [PR #87 head `e0a0ccea5a61c64f7de9ca0e122adb01f376a3f9`](https://github.com/TelemetryWorks/irig106-tmats/pull/87/commits/e0a0ccea5a61c64f7de9ca0e122adb01f376a3f9), reviewer @thread:thr_qsvwmtnn8f, received 2026-11-28; [final-check record](research/2026-11-28-int022-066-final-concurrences.md); source freshness, written development analysis/review and intensive executable tests remain required · **Origin**: family 2 normalization source review

**Test**: `normalize_counter_contiguous_distinct_members`, `normalize_counter_gap_refuses`, `normalize_counter_invalid_old_value_explicit_rewrite`, `normalize_counter_compatible_damage_blocks` (planned owner-signed A expectations; development/test gates retained)

**Development**: intensive testing and deep analysis required · **Analysis**: not yet written
