use reikura_util::io::ReadExt;

use crate::ImageDecoder;

pub struct Bin;

impl ImageDecoder for Bin {
    const MAGIC: &[u8] = b"BM";

    type Metadata = [u8; 2];

    fn parse(mut data: &[u8]) -> anyhow::Result<Self::Metadata> {
        let magic = data.get_le::<[u8; 2]>()?;
        Ok(magic)
    }

    fn decode(magic: Self::Metadata, data: &[u8]) -> anyhow::Result<(u32, u32, Vec<u8>)> {
        assert_eq!(magic, Self::MAGIC);

        let image = image::load_from_memory_with_format(data, image::ImageFormat::Bmp)?;
        let width = image.width();
        let height = image.height();

        // image crate only give us access in rgba format forced us to make this roundtrip conversion
        let mut data = image.into_rgba8().into_raw();
        for pixel in data.as_chunks_mut::<4>().0 {
            pixel.swap(0, 2); // convert rgb -> bgr
        }

        Ok((width, height, data))
    }
}
