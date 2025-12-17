pub mod error;
pub mod packet;
pub mod primary;
pub mod sequence;

pub use error::Error;
pub type Result<T> = core::result::Result<T, Error>;

pub use sequence::{PacketType, SequenceFlag};
