use crate::{Result, Error};
use crate::primary::{PrimaryHeader};

pub struct SpacePacket<'a> {
    raw: &'a [u8],
    primary: PrimaryHeader,
}

impl<'a> SpacePacket<'a> {
    pub fn parse(buf: &'a [u8]) -> Result<Self> {
    }
}
