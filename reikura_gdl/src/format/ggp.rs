use reikura_util::io::ReadExt;

use crate::ImageDecoder;

pub struct GgpFaike;

pub struct GgpFaikeMetadata {
    magic: [u8; 8],
    _unknown: u32,
    key: [u8; 8],
    offset: u32,
    length: u32,
}

impl ImageDecoder for GgpFaike {
    const MAGIC: &[u8] = b"GGPFAIKE";

    type Metadata = GgpFaikeMetadata;

    fn parse(mut data: &[u8]) -> anyhow::Result<Self::Metadata> {
        Ok(GgpFaikeMetadata {
            magic: data.read_le()?,
            _unknown: data.read_le()?,
            key: data.read_le()?,
            offset: data.read_le()?,
            length: data.read_le()?,
        })
    }

    fn decode(mut md: Self::Metadata, data: &[u8]) -> anyhow::Result<(u32, u32, Vec<u8>)> {
        debug_assert_eq!(md.magic, Self::MAGIC);

        let offset = md.offset as usize;
        let length = md.length as usize;

        md.key
            .iter_mut()
            .zip(md.magic)
            .for_each(|(key, magic)| *key ^= magic);

        let mut png_data = data[offset..][..length].to_vec();
        png_data
            .iter_mut()
            .zip(md.key.iter().cycle())
            .for_each(|(byte, key)| *byte ^= key);

        let image = image::load_from_memory_with_format(&png_data, image::ImageFormat::Png)?;

        Ok((image.width(), image.height(), image.to_rgba8().into_vec()))
    }
}
