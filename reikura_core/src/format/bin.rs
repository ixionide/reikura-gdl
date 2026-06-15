use reikura_util::io::ReadExt;

use crate::{Image, ImageDecoder};

pub struct Bin;

impl ImageDecoder for Bin {
    const MAGIC: &[u8] = b"BM";

    type Metadata = [u8; 2];

    fn parse(mut data: &[u8]) -> anyhow::Result<Self::Metadata> {
        let magic = data.read_le::<[u8; 2]>()?;
        Ok(magic)
    }

    fn decode(magic: Self::Metadata, name: &str, data: &[u8]) -> anyhow::Result<crate::Image> {
        debug_assert_eq!(magic, Self::MAGIC);

        let image = image::load_from_memory_with_format(data, image::ImageFormat::Bmp)?;

        Ok(Image {
            width: image.width(),
            height: image.height(),
            name: name.to_owned().into(),
            data: image.to_rgba8().into_vec().into(),
        })
    }
}
