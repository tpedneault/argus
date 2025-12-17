use crate::{Result, Error};
use crate::sequence::{PacketType, SequenceFlag};

pub const PRIMARY_HEADER_LENGTH : usize = 6;

pub struct PrimaryHeader {
    version: u8,
    packet_type: PacketType,
    secondary_header: bool,
    apid: u16,
    seq_flag: SequenceFlag,
    seq_count: u16,
    packet_length: u16,
}

impl PrimaryHeader {
    pub fn parse(buf: &[u8]) -> Result<PrimaryHeader> {
        if buf.len() < PRIMARY_HEADER_LENGTH {
            return Err(Error::BufferTooShort);
        }

        let version = buf[0] >> 5;
        if version != 0 {
            return Err(Error::BadVersion(version));
        }

        let packet_type_bits = (buf[0] & 0x10) >> 4;
        let packet_type = PacketType::try_from(packet_type_bits)?;

        let secondary_header = (buf[0] >> 3) & 0x01 != 0;

        let apid_upper_bits = (buf[0] & 0b111) as u16;
        let apid_lower_bits = buf[1] as u16;
        let apid = (apid_upper_bits << 8) | apid_lower_bits;

        let seq_flag_bits = buf[2] >> 6;
        let seq_flag = SequenceFlag::try_from(seq_flag_bits)?;

        let seq_count_upper_bits = (buf[2] & 0x3f) as u16;
        let seq_count_lower_bits = buf[3] as u16;
        let seq_count = (seq_count_upper_bits << 8) | seq_count_lower_bits;

        let packet_length = u16::from_be_bytes([buf[4], buf[5]]);

        Ok(Self {
            version,
            packet_type,
            secondary_header,
            apid,
            seq_flag,
            seq_count,
            packet_length
        })
    }
}
