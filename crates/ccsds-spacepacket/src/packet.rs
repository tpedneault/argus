use crate::Result;
use crate::primary::PrimaryHeader;

pub struct SpacePacket<'a> {
    raw: &'a [u8],
    primary: PrimaryHeader,
}

impl<'a> SpacePacket<'a> {
    pub fn parse(buf: &'a [u8]) -> Result<Self> {
        let primary = PrimaryHeader::parse(buf)?;
        
        Ok(Self {
            raw: buf,
            primary,
        })
    }
    
    pub fn raw(&self) -> &[u8] {
        self.raw
    }
    
    pub fn primary(&self) -> &PrimaryHeader {
        &self.primary
    }

    pub fn data_field(&self) -> &[u8] {
        use crate::primary::PRIMARY_HEADER_LENGTH;
        if self.raw.len() > PRIMARY_HEADER_LENGTH {
            &self.raw[PRIMARY_HEADER_LENGTH..]
        } else {
            &[]
        }
    }
}

