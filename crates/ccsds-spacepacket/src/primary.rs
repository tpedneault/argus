use crate::sequence::{PacketType, SequenceFlag};
use crate::{Error, Result};
use deku::prelude::*;

pub const PRIMARY_HEADER_LENGTH: usize = 6;

#[derive(Debug, Clone, PartialEq, Eq, DekuRead, DekuWrite)]
pub struct PrimaryHeader {
    #[deku(bits = "3", assert_eq = "0")]
    version: u8,
    packet_type: PacketType,
    #[deku(bits = "1")]
    secondary_header: u8,
    #[deku(bits = "11", endian = "big")]
    apid: u16,
    seq_flag: SequenceFlag,
    #[deku(bits = "14", endian = "big")]
    seq_count: u16,
    #[deku(endian = "big")]
    packet_length: u16,
}

impl PrimaryHeader {
    pub fn parse(buf: &[u8]) -> Result<PrimaryHeader> {
        if buf.len() < PRIMARY_HEADER_LENGTH {
            return Err(Error::BufferTooShort);
        }

        let (_, header) = PrimaryHeader::from_bytes((buf, 0))
            .map_err(|_e| Error::InvalidField("primary header"))?;

        Ok(header)
    }

    pub fn version(&self) -> u8 {
        self.version
    }

    pub fn packet_type(&self) -> PacketType {
        self.packet_type
    }

    pub fn has_secondary_header(&self) -> bool {
        self.secondary_header == 1
    }

    pub fn apid(&self) -> u16 {
        self.apid
    }

    pub fn sequence_flags(&self) -> SequenceFlag {
        self.seq_flag
    }

    pub fn sequence_count(&self) -> u16 {
        self.seq_count
    }

    pub fn packet_length(&self) -> u16 {
        self.packet_length
    }

    pub fn data_length(&self) -> u16 {
        self.packet_length.saturating_add(1)
    }

    pub fn total_length(&self) -> usize {
        PRIMARY_HEADER_LENGTH + self.data_length() as usize
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_buffer_too_short() {
        let data = [0x00, 0x00, 0x00, 0x00, 0x00]; // Only 5 bytes
        let result = PrimaryHeader::parse(&data);
        assert!(matches!(result, Err(Error::BufferTooShort)));
    }

    #[test]
    fn test_invalid_version() {
        // Version=1 (invalid, should be 0)
        let data = [
            0x20, 0x00, // ver(1) in top 3 bits
            0x00, 0x00,
            0x00, 0x00,
        ];

        let result = PrimaryHeader::parse(&data);
        assert!(result.is_err());
    }
}

