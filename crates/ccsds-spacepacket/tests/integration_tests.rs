use ccsds_spacepacket::packet::SpacePacket;
use ccsds_spacepacket::primary::PrimaryHeader;
use ccsds_spacepacket::sequence::{PacketType, SequenceFlag};

#[test]
fn test_parse_complete_packet() {
    // Complete packet with header + 4 bytes data
    let data = [
        0x01, 0x23, // APID 0x123, TM, no sec hdr
        0xC0, 0x05, // Unsegmented, seq 5
        0x00, 0x03, // Length 3 (4 bytes data)
        0xAA, 0xBB, 0xCC, 0xDD, // Data field
    ];

    let packet = SpacePacket::parse(&data).unwrap();
    
    assert_eq!(packet.raw(), &data);
    assert_eq!(packet.primary().apid(), 0x123);
    assert_eq!(packet.primary().packet_type(), PacketType::Telemetry);
    assert_eq!(packet.primary().sequence_flags(), SequenceFlag::Unsegmented);
    assert_eq!(packet.primary().sequence_count(), 5);
    assert_eq!(packet.data_field(), &[0xAA, 0xBB, 0xCC, 0xDD]);
}

#[test]
fn test_parse_packet_no_data() {
    // Packet with only header
    let data = [
        0x00, 0x00,
        0x00, 0x00,
        0x00, 0x00,
    ];

    let packet = SpacePacket::parse(&data).unwrap();
    assert_eq!(packet.data_field(), &[]);
}

#[test]
fn test_parse_packet_with_data() {
    let data = [
        0x18, 0xFF, // TC, sec hdr, APID 0xFF
        0x80, 0x00, // Last segment, seq 0
        0x00, 0x0F, // Length 15 (16 bytes)
        // 16 bytes of data
        0x00, 0x01, 0x02, 0x03,
        0x04, 0x05, 0x06, 0x07,
        0x08, 0x09, 0x0A, 0x0B,
        0x0C, 0x0D, 0x0E, 0x0F,
    ];

    let packet = SpacePacket::parse(&data).unwrap();
    
    assert_eq!(packet.primary().packet_type(), PacketType::Telecommand);
    assert!(packet.primary().has_secondary_header());
    assert_eq!(packet.primary().apid(), 0xFF);
    assert_eq!(packet.primary().sequence_flags(), SequenceFlag::Last);
    assert_eq!(packet.primary().packet_length(), 15);
    assert_eq!(packet.data_field().len(), 16);
    assert_eq!(packet.data_field()[0], 0x00);
    assert_eq!(packet.data_field()[15], 0x0F);
}

#[test]
fn test_multiple_packets_from_stream() {
    // Simulate parsing multiple packets from a buffer
    let stream = [
        // Packet 1: 6 byte header + 2 byte data
        0x00, 0x01, 0xC0, 0x00, 0x00, 0x01, 0xFF, 0xFF,
        // Packet 2: 6 byte header + 3 byte data
        0x00, 0x02, 0xC0, 0x01, 0x00, 0x02, 0xAA, 0xBB, 0xCC,
    ];

    let packet1 = SpacePacket::parse(&stream[0..8]).unwrap();
    assert_eq!(packet1.primary().apid(), 1);
    assert_eq!(packet1.primary().sequence_count(), 0);
    assert_eq!(packet1.data_field(), &[0xFF, 0xFF]);

    let packet2 = SpacePacket::parse(&stream[8..17]).unwrap();
    assert_eq!(packet2.primary().apid(), 2);
    assert_eq!(packet2.primary().sequence_count(), 1);
    assert_eq!(packet2.data_field(), &[0xAA, 0xBB, 0xCC]);
}

#[test]
fn test_parse_basic_telemetry_packet() {
    // Version=0, Type=TM(0), SecHdr=0, APID=0x123
    // SeqFlags=Unsegmented(11), SeqCount=42
    // PacketLength=9 (10 bytes of data)
    let data = [
        0x01, 0x23, // First 2 bytes: ver(0) + type(0) + sec(0) + apid(0x123)
        0xC0, 0x2A, // Seq flags (11) + count (42)
        0x00, 0x09, // Packet length (9 = 10-1)
    ];

    let header = PrimaryHeader::parse(&data).unwrap();
    assert_eq!(header.version(), 0);
    assert_eq!(header.packet_type(), PacketType::Telemetry);
    assert!(!header.has_secondary_header());
    assert_eq!(header.apid(), 0x123);
    assert_eq!(header.sequence_flags(), SequenceFlag::Unsegmented);
    assert_eq!(header.sequence_count(), 42);
    assert_eq!(header.packet_length(), 9);
    assert_eq!(header.data_length(), 10);
    assert_eq!(header.total_length(), 16);
}

#[test]
fn test_parse_telecommand_with_secondary_header() {
    // Version=0, Type=TC(1), SecHdr=1, APID=2047 (max 11-bit)
    // SeqFlags=First(01), SeqCount=16383 (max 14-bit)
    // PacketLength=255
    let data = [
        0x1F, 0xFF, // ver(0) + type(1) + sec(1) + apid(0x7FF)
        0x7F, 0xFF, // flags(01) + count(0x3FFF)
        0x00, 0xFF, // length
    ];

    let header = PrimaryHeader::parse(&data).unwrap();
    assert_eq!(header.version(), 0);
    assert_eq!(header.packet_type(), PacketType::Telecommand);
    assert!(header.has_secondary_header());
    assert_eq!(header.apid(), 0x7FF);
    assert_eq!(header.sequence_flags(), SequenceFlag::First);
    assert_eq!(header.sequence_count(), 0x3FFF);
    assert_eq!(header.packet_length(), 255);
}

#[test]
fn test_parse_all_sequence_flags() {
    let test_cases = [
        (0b00, SequenceFlag::Continuation),
        (0b01, SequenceFlag::First),
        (0b10, SequenceFlag::Last),
        (0b11, SequenceFlag::Unsegmented),
    ];

    for (flag_bits, expected_flag) in test_cases {
        let data = [
            0x00, 0x00,           // ver(0) + type(0) + sec(0) + apid(0)
            flag_bits << 6, 0x00, // flags in top 2 bits
            0x00, 0x00,           // length
        ];

        let header = PrimaryHeader::parse(&data).unwrap();
        assert_eq!(header.sequence_flags(), expected_flag);
    }
}

#[test]
fn test_roundtrip_serialization() {
    use deku::DekuContainerWrite;
    
    let original = [
        0x08, 0xAB, // ver(0) + type(0) + sec(1) + apid(0xAB)
        0x40, 0x64, // flags(01) + count(100)
        0x01, 0x23, // length(291)
    ];

    let header = PrimaryHeader::parse(&original).unwrap();
    let serialized = header.to_bytes().unwrap();
    
    assert_eq!(&serialized[..], &original[..]);
}
