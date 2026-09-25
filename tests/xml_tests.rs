// tests/xml_tests.rs
//
// # XML Format Integration Tests
//
// ## Traceability:
//   L3-TEST-009: XML/ASCII conversion round-trip
//   L2-XML-001: XML parse
//   L2-XML-002: XML serialize
//   L2-XML-003: XML/ASCII divergence handling
//   L2-XML-004: Bidirectional conversion

#![cfg(feature = "xml")]

use irig106_tmats::prelude::*;

// ═════════════════════════════════════════════════════════════════════════════
// XML Serialization (L2-XML-002)
// ═════════════════════════════════════════════════════════════════════════════

#[test]
fn xml_serialize_basic() {
    let input = b"G\\PN:FLIGHT_TEST;G\\106:17;G\\OD:03-15-2024;G\\TN:FT-001;";
    let doc = parse(input).expect("parse failed");
    let xml_bytes = serialize_xml_to_vec(&doc).expect("xml serialize failed");
    let xml_str = String::from_utf8_lossy(&xml_bytes);

    assert!(xml_str.contains("<ProgramName>FLIGHT_TEST</ProgramName>"));
    assert!(xml_str.contains("<Irig106Version>17</Irig106Version>"));
    assert!(xml_str.contains("<TestNumber>FT-001</TestNumber>"));
    // Date should be in XML format (L3-XML-006)
    assert!(xml_str.contains("<OriginationDate>2024-03-15</OriginationDate>"));
}

#[test]
fn xml_serialize_r_group_with_channels() {
    let input = b"G\\PN:TEST;G\\106:17;\
        R-1\\ID:MDR;R-1\\N:1;R-1\\TK1-1:5;R-1\\CDT-1:09;R-1\\CDLN-1:PCM1;R-1\\PDP-1:UN;";
    let doc = parse(input).expect("parse failed");
    let xml_bytes = serialize_xml_to_vec(&doc).expect("xml serialize failed");
    let xml_str = String::from_utf8_lossy(&xml_bytes);

    assert!(xml_str.contains("<RecorderID>MDR</RecorderID>"));
    assert!(xml_str.contains("<ChannelID>5</ChannelID>"));
    assert!(xml_str.contains("<DataLinkName>PCM1</DataLinkName>"));
    // L3-XML-004: Keyword should be expanded
    assert!(xml_str.contains("<DataPacking>Unpacked</DataPacking>"));
}

#[test]
fn xml_serialize_p_group() {
    let input = b"G\\PN:TEST;G\\106:17;\
        P-1\\DLN:PCM1;P-1\\D1:5000000;P-1\\D2:NRZ-L;";
    let doc = parse(input).expect("parse failed");
    let xml_bytes = serialize_xml_to_vec(&doc).expect("xml serialize failed");
    let xml_str = String::from_utf8_lossy(&xml_bytes);

    assert!(xml_str.contains("<PCMFormatAttributes>"));
    assert!(xml_str.contains("<BitRate>5000000</BitRate>"));
    // NRZ-L stays as NRZ-L (already readable)
    assert!(xml_str.contains("NRZ-L"));
}

#[test]
fn xml_serialize_no_counters() {
    // L3-XML-003: Counter attributes must NOT appear in XML
    let input = b"G\\PN:TEST;G\\106:17;\
        G\\DSI\\N:2;G\\DSI-1:SRC1;G\\DST-1:REC;G\\DSI-2:SRC2;G\\DST-2:TEL;";
    let doc = parse(input).expect("parse failed");
    let xml_bytes = serialize_xml_to_vec(&doc).expect("xml serialize failed");
    let xml_str = String::from_utf8_lossy(&xml_bytes);

    // No \N counter elements
    assert!(!xml_str.contains("DSI\\N"));
    assert!(!xml_str.contains("NumberOfDataSources"));

    // But data sources are present
    assert!(xml_str.contains("<DataSourceID>SRC1</DataSourceID>"));
    assert!(xml_str.contains("<DataSourceID>SRC2</DataSourceID>"));
}

#[test]
fn xml_serialize_keyword_expansion() {
    // L3-XML-004: Keyword values expanded for readability
    let input = b"G\\PN:TEST;G\\106:17;\
        G\\DSI\\N:1;G\\DSI-1:SRC;G\\DST-1:REC;G\\DSC-1:U;\
        B-1\\DLN:BUS1;B-1\\BT:1553;";
    let doc = parse(input).expect("parse failed");
    let xml_bytes = serialize_xml_to_vec(&doc).expect("xml serialize failed");
    let xml_str = String::from_utf8_lossy(&xml_bytes);

    assert!(xml_str.contains("<DataSourceType>Recorder</DataSourceType>"));
    assert!(xml_str.contains("<Classification>Unclassified</Classification>"));
    assert!(xml_str.contains("<BusType>MIL-STD-1553</BusType>"));
}

// ═════════════════════════════════════════════════════════════════════════════
// XML Parsing (L2-XML-001)
// ═════════════════════════════════════════════════════════════════════════════

#[test]
fn xml_parse_basic() {
    let xml = br#"<?xml version="1.0" encoding="UTF-8"?>
<Tmats>
  <GeneralInformation>
    <ProgramName>XML_TEST</ProgramName>
    <Irig106Version>17</Irig106Version>
    <OriginationDate>2024-03-15</OriginationDate>
    <TestNumber>XFT-001</TestNumber>
  </GeneralInformation>
</Tmats>"#;

    let doc = parse_xml(xml).expect("xml parse failed");
    assert_eq!(doc.general.program_name.as_deref(), Some("XML_TEST"));
    assert_eq!(doc.general.irig106_version.as_deref(), Some("17"));
    assert_eq!(doc.general.test_number.as_deref(), Some("XFT-001"));

    // Date should be converted from XML format (L3-XML-006)
    let od = doc.general.origination_date.expect("missing date");
    assert_eq!(od.year, 2024);
    assert_eq!(od.month, 3);
    assert_eq!(od.day, 15);
}

#[test]
fn xml_parse_with_data_sources() {
    let xml = br#"<?xml version="1.0" encoding="UTF-8"?>
<Tmats>
  <GeneralInformation>
    <ProgramName>DS_TEST</ProgramName>
    <Irig106Version>17</Irig106Version>
    <DataSource>
      <DataSourceID>SRC_A</DataSourceID>
      <DataSourceType>Recorder</DataSourceType>
      <Classification>Unclassified</Classification>
    </DataSource>
    <DataSource>
      <DataSourceID>SRC_B</DataSourceID>
      <DataSourceType>Telemetry</DataSourceType>
    </DataSource>
  </GeneralInformation>
</Tmats>"#;

    let doc = parse_xml(xml).expect("xml parse failed");
    assert_eq!(doc.general.data_sources.len(), 2);
    assert_eq!(doc.general.data_sources[&1].data_source_id.as_deref(), Some("SRC_A"));
    // Keyword should be collapsed from expanded XML form
    assert_eq!(doc.general.data_sources[&1].data_source_type.as_deref(), Some("REC"));
    assert_eq!(doc.general.data_sources[&1].classification.as_deref(), Some("U"));
    assert_eq!(doc.general.data_sources[&2].data_source_type.as_deref(), Some("TEL"));
}

#[test]
fn xml_parse_r_group() {
    let xml = br#"<?xml version="1.0" encoding="UTF-8"?>
<Tmats>
  <GeneralInformation>
    <ProgramName>R_TEST</ProgramName>
    <Irig106Version>17</Irig106Version>
  </GeneralInformation>
  <RecorderAttributes>
    <RecorderID>MDR_1</RecorderID>
    <Channel>
      <ChannelID>5</ChannelID>
      <DataType>09</DataType>
      <DataLinkName>PCM1</DataLinkName>
      <DataPacking>Unpacked</DataPacking>
    </Channel>
    <Channel>
      <ChannelID>10</ChannelID>
      <DataType>19</DataType>
      <DataLinkName>BUS1</DataLinkName>
    </Channel>
  </RecorderAttributes>
</Tmats>"#;

    let doc = parse_xml(xml).expect("xml parse failed");
    assert_eq!(doc.recorders.len(), 1);

    let r = &doc.recorders[&1];
    assert_eq!(r.recorder_id.as_deref(), Some("MDR_1"));
    assert_eq!(r.channels.len(), 2);
    assert_eq!(r.num_channels, Some(2)); // Auto-computed

    assert_eq!(r.channels[&1].channel_id, Some(5));
    // Keyword collapsed from "Unpacked" → "UN"
    assert_eq!(r.channels[&1].data_packing_option.as_deref(), Some("UN"));

    assert_eq!(r.channels[&2].channel_id, Some(10));
}

// ═════════════════════════════════════════════════════════════════════════════
// Bidirectional Conversion (L2-XML-004, L3-TEST-009)
// ═════════════════════════════════════════════════════════════════════════════

#[test]
fn ascii_to_xml_round_trip() {
    let ascii = b"G\\PN:CONVERT_TEST;G\\106:17;G\\OD:06-20-2024;\
        G\\DSI\\N:1;G\\DSI-1:SRC;G\\DST-1:REC;\
        R-1\\ID:REC;R-1\\N:1;R-1\\TK1-1:1;R-1\\CDLN-1:PCM1;\
        P-1\\DLN:PCM1;P-1\\D1:1000000;P-1\\D2:NRZ-L;P-1\\F1:128;P-1\\F2:16;P-1\\F3:FE;";

    // ASCII → XML
    let xml_bytes = ascii_to_xml(ascii).expect("ascii_to_xml failed");

    // XML → ASCII
    let ascii_out = xml_to_ascii(&xml_bytes).expect("xml_to_ascii failed");

    // Re-parse the round-tripped ASCII
    let doc = parse(&ascii_out).expect("reparse failed");

    assert_eq!(doc.general.program_name.as_deref(), Some("CONVERT_TEST"));
    assert_eq!(doc.recorders.len(), 1);
    assert_eq!(doc.pcm_formats.len(), 1);
}

#[test]
fn xml_to_ascii_preserves_structure() {
    let xml = br#"<?xml version="1.0" encoding="UTF-8"?>
<Tmats>
  <GeneralInformation>
    <ProgramName>XML_SRC</ProgramName>
    <Irig106Version>17</Irig106Version>
  </GeneralInformation>
  <RecorderAttributes>
    <RecorderID>REC1</RecorderID>
    <Channel>
      <ChannelID>1</ChannelID>
      <DataLinkName>LINK1</DataLinkName>
    </Channel>
  </RecorderAttributes>
  <PCMFormatAttributes>
    <DataLinkName>LINK1</DataLinkName>
    <BitRate>2000000</BitRate>
  </PCMFormatAttributes>
</Tmats>"#;

    let ascii = xml_to_ascii(xml).expect("xml_to_ascii failed");
    let doc = parse(&ascii).expect("reparse failed");

    assert_eq!(doc.general.program_name.as_deref(), Some("XML_SRC"));
    assert_eq!(doc.recorders[&1].recorder_id.as_deref(), Some("REC1"));
    assert_eq!(doc.pcm_formats.len(), 1);
}

// ═════════════════════════════════════════════════════════════════════════════
// Keyword Expansion/Collapse
// ═════════════════════════════════════════════════════════════════════════════

#[test]
fn keyword_expand_collapse_round_trip() {
    use irig106_tmats::xml::{expand_keyword, collapse_keyword};

    let test_cases = &[
        ("UN", "Unpacked"),
        ("PFS", "Packed"),
        ("TM", "Throughput"),
        ("REC", "Recorder"),
        ("TEL", "Telemetry"),
        ("U", "Unclassified"),
        ("1553", "MIL-STD-1553"),
        ("PAIR", "PairSets"),
        ("T", "True"),
        ("F", "False"),
    ];

    for &(abbrev, expanded) in test_cases {
        assert_eq!(expand_keyword(abbrev), expanded,
            "expand failed for '{abbrev}'");
        assert_eq!(collapse_keyword(expanded), abbrev,
            "collapse failed for '{expanded}'");
    }
}
