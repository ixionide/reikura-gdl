use anyhow::Result;
use reikura_util::io::ReadExt;

pub struct IsfMetadata {
    pub bytecode_offset: u32,
    pub version: [u8; 2],
    pub xor_key: u8,
    _reserved: u8,
}

impl IsfMetadata {
    pub fn parse(mut data: &[u8]) -> Result<Self> {
        Ok(Self {
            bytecode_offset: data.get_le()?,
            version: data.get_be()?,
            xor_key: data.get_le()?,
            _reserved: 0,
        })
    }
}
