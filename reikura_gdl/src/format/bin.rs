use reikura_util::io::ReadExt;

use crate::ImageDecoder;

pub struct Bin;

impl ImageDecoder for Bin {
    const MAGIC: &[u8] = b"BM";

    type Metadata = [u8; 2];

    fn parse(mut data: &[u8]) -> anyhow::Result<Self::Metadata> {
        let magic = data.read_le::<[u8; 2]>()?;
        Ok(magic)
    }

    fn decode(magic: Self::Metadata, data: &[u8]) -> anyhow::Result<(u32, u32, Vec<u8>)> {
        debug_assert_eq!(magic, Self::MAGIC);

        let image = image::load_from_memory_with_format(data, image::ImageFormat::Bmp)?;

        Ok((image.width(), image.height(), image.to_rgba8().into_vec()))
    }
}
