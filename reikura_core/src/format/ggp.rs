use anyhow::bail;
use reikura_util::{image::PIXEL_STRIDE, io::ReadExt};

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
        let offset = md.offset as usize;
        let length = md.length as usize;

        md.key
            .iter_mut()
            .zip(Self::MAGIC)
            .for_each(|(key, magic)| *key ^= magic);

        let mut png_data = data[offset..][..length].to_vec();
        png_data
            .iter_mut()
            .zip(md.key.iter().cycle())
            .for_each(|(byte, key)| *byte ^= key);

        todo!()
    }
}

const PNG_MAGIC: &[u8] = b"%PNG\x0D\x0A\x1A\x0A";

pub fn decode_png(data: &[u8]) -> anyhow::Result<(u32, u32, Vec<u8>)> {
    let (magic, mut bytes) = data.split_at(PNG_MAGIC.len());

    let mut width: u32 = 0;
    let mut height: u32 = 0;
    let mut bit_depth: u8 = 0;
    let mut color_type: u8 = 0;
    // let compression_method = chunk.data[10];
    let mut filter_method: u8 = 0;
    let mut interlace_method: u8 = 0;

    let mut palette: Option<&[u8]> = None;
    let mut transparency: Option<&[u8]> = None;
    let mut background: Option<&[u8]> = None;
    let mut image_data: Option<&[u8]> = None;

    if magic != PNG_MAGIC {
        anyhow::bail!("invalid png data");
    }

    // fn read_chunk() {}

    'read_chunk: loop {
        let chunk_len = bytes.read_be::<u32>()? as usize + 4; // plus crc
        let chunk_type: [u8; 4] = bytes.read_le()?;

        let split = bytes.split_at(chunk_len);
        let mut chunk_data = split.0;

        match &chunk_type {
            b"IHDR" => {
                width = chunk_data.read_be()?;
                height = chunk_data.read_be()?;
                bit_depth = chunk_data.read_be()?;
                color_type = chunk_data.read_be()?;
                assert_eq!(chunk_data.read_be::<u8>()?, 0); //compression method
                filter_method = chunk_data.read_be()?;
                interlace_method = chunk_data.read_be()?;
            }
            b"PLTE" => palette = Some(&chunk_data[..chunk_len - 4]),
            b"tRNS" => transparency = Some(&chunk_data[..chunk_len - 4]),
            b"bKGD" => background = Some(&chunk_data[..chunk_len - 4]),
            b"sRGB" => (),
            b"IDAT" => image_data = Some(&chunk_data[..chunk_len - 4]),

            b"IEND" => break 'read_chunk,
            _ => bail!("unknown png chunk type: {chunk_type:?}"),
        }

        bytes = split.1;
    }

    let pixels = vec![0; width as usize * height as usize * PIXEL_STRIDE];

    Ok((width, height, pixels))
}
