use std::rc::Rc;

use anyhow::{Result, bail};

use crate::format::{
    bin::Bin,
    gga::Gga,
    ggd::{Ggd256g, GgdFull},
    ggp::GgpFaike,
};

#[derive(Clone)]
pub struct Image {
    pub width: u32,
    pub height: u32,
    pub name: Rc<str>,
    pub data: Rc<[u8]>,
}

impl Image {
    pub fn load(name: String, data: Vec<u8>) -> Result<Self> {
        let magic = &data[0..8];

        match magic {
            b"GGA00000" => Gga::load(name, data),
            b"GGPFAIKE" => GgpFaike::load(name, data),
            [0xCD, 0xCA, 0xC9, 0xB8, ..] => Ggd256g::load(name, data),
            [0xB9, 0xAA, 0xB3, 0xB3, ..] => GgdFull::load(name, data),
            [0x42, 0x4D, ..] => Bin::load(name, data),
            unk => bail!("unknown image format with magic: {unk:?}"),
        }
    }
}

pub trait ImageDecoder {
    const MAGIC: &[u8];
    type Metadata;

    fn parse(data: &[u8]) -> Result<Self::Metadata>;
    fn decode(md: Self::Metadata, data: &[u8]) -> Result<(u32, u32, Vec<u8>)>;
    fn load(name: String, data: Vec<u8>) -> Result<Image> {
        let md = Self::parse(&data)?;
        let (width, height, data) = Self::decode(md, &data)?;

        Ok(Image {
            width,
            height,
            name: name.into(),
            data: data.into(),
        })
    }
}
