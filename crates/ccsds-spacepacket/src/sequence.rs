use crate::{Result};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PacketType {
    Telemetry,    // 00
    Telecommand   // 11
}

impl TryFrom<u8> for PacketType {
    type Error = crate::Error;
    fn try_from(v: u8) -> Result<Self> {
        match v & 0b1 {
            0b0 => Ok(PacketType::Telemetry),
            0b1 => Ok(PacketType::Telecommand),
            _ => unreachable!(), // masked to 1 bit
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SequenceFlag {
    Continuation, // 00
    First,        // 01
    Last,         // 10
    Unsegmented   // 11
}

impl TryFrom<u8> for SequenceFlag {
    type Error = crate::Error;
    fn try_from(v: u8) -> Result<Self> {
        match v & 0b11 {
            0b00 => Ok(SequenceFlag::Continuation),
            0b01 => Ok(SequenceFlag::First),
            0b10 => Ok(SequenceFlag::Last),
            0b11 => Ok(SequenceFlag::Unsegmented),
            _ => unreachable!(), // masked to 2 bits
        }
    }
}
