use deku::prelude::*;

#[derive(Debug, Clone, Copy, PartialEq, Eq, DekuRead, DekuWrite)]
#[deku(id_type = "u8", bits = "1")]
pub enum PacketType {
    #[deku(id = "0b0")]
    Telemetry,
    #[deku(id = "0b1")]
    Telecommand,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, DekuRead, DekuWrite)]
#[deku(id_type = "u8", bits = "2")]
pub enum SequenceFlag {
    #[deku(id = "0b00")]
    Continuation,
    #[deku(id = "0b01")]
    First,
    #[deku(id = "0b10")]
    Last,
    #[deku(id = "0b11")]
    Unsegmented,
}
