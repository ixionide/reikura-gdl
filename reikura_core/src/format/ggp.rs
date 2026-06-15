use reikura_util::io::ReadExt;

use crate::{Image, ImageDecoder};

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

    fn decode(mut md: Self::Metadata, name: &str, data: &[u8]) -> anyhow::Result<crate::Image> {
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

        Ok(Image {
            width: image.width(),
            height: image.height(),
            name: name.to_owned().into(),
            data: image.to_rgba8().into_vec().into(),
        })
    }
}
