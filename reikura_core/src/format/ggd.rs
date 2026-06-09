use std::{io::Read, rc::Rc};

use anyhow::Result;
use reikura_util::{
    image::{PIXEL_STRIDE, copy_previous_pixels},
    io::ReadExt,
    lzss,
};

use crate::{Image, ImageDecoder};

pub struct GgdFull;

pub struct GgdFullMetadata {
    width: u16,
    height: u16,
}

impl ImageDecoder for GgdFull {
    const MAGIC: &[u8] = b"\xB9\xAA\xB3\xB3";
    type Metadata = GgdFullMetadata;

    fn parse(mut data: &[u8]) -> Result<Self::Metadata> {
        Ok(GgdFullMetadata {
            width: data.read_le()?,
            height: data.read_le()?,
        })
    }

    fn decode(md: Self::Metadata, name: &str, data: &[u8]) -> Result<Image> {
        let size = md.width as usize * md.height as usize * PIXEL_STRIDE;

        let mut pixels = Vec::with_capacity(size);
        let mut reader = &data[8..];
        let mut buf = [0xFF; PIXEL_STRIDE];

        while pixels.len() < size {
            let ctrl = reader.read_le::<u8>()?;

            match ctrl {
                0x0 => {
                    let count = reader.read_le::<u8>()? as usize;
                    let pos = 1;
                    copy_previous_pixels(&mut pixels, pos, count);
                }
                0x1 => {
                    let count = reader.read_le::<u8>()? as usize;
                    let pos = reader.read_le::<u8>()? as usize;
                    copy_previous_pixels(&mut pixels, pos, count);
                }
                0x2 => {
                    let count = reader.read_le::<u8>()? as usize;
                    let pos = reader.read_le::<u16>()? as usize;
                    copy_previous_pixels(&mut pixels, pos, count);
                }
                0x3 => {
                    let count = 1;
                    let pos = reader.read_le::<u8>()? as usize;
                    copy_previous_pixels(&mut pixels, pos, count);
                }
                0x4 => {
                    let count = 1;
                    let pos = reader.read_le::<u16>()? as usize;
                    copy_previous_pixels(&mut pixels, pos, count);
                }
                255 => break,
                num => {
                    let count = num - 4;
                    for _ in 0..count {
                        reader.read_exact(&mut buf[..3])?;
                        buf.swap(0, 2); // convert bgr -> rgb
                        pixels.extend_from_slice(&buf);
                    }
                }
            }
        }

        Ok(Image {
            width: md.width as _,
            height: md.height as _,
            name: name.to_owned().into(),
            data: pixels.into(),
        })
    }
}

pub struct Ggd256g;

pub struct Ggd256gMetadata {
    header_len: u32,
    width: u32,
    height: u32,
    _unknown1: u32,
    _unknown2: u32,
    uncompressed_len: u32,
}

impl ImageDecoder for Ggd256g {
    const MAGIC: &[u8] = b"\xCD\xCA\xC9\xB8";
    type Metadata = Ggd256gMetadata;

    fn parse(mut data: &[u8]) -> Result<Self::Metadata> {
        Ok(Ggd256gMetadata {
            header_len: data.read_le()?,
            width: data.read_le()?,
            height: data.read_le::<i32>()?.unsigned_abs(),
            _unknown1: data.read_le()?,
            _unknown2: data.read_le()?,
            uncompressed_len: data.read_le()?,
        })
    }

    fn decode(md: Self::Metadata, name: &str, data: &[u8]) -> Result<Image> {
        let size = md.width as usize * md.height as usize * PIXEL_STRIDE;
        let palette_pos = md.header_len as usize + 4;
        let palette_len = size_of::<u32>() * 256;
        let compressed_pos = palette_pos + palette_len + 4;

        let compressed = &data[compressed_pos..];
        let mut palette_bytes = &data[palette_pos..][..palette_len];

        let mut palette = Vec::with_capacity(256);
        let mut buf = [0xFF; 4];
        for _ in 0..256 {
            palette_bytes.read_exact(&mut buf[0..3])?;
            buf.swap(0, 2); // convert bgr -> rgb
            palette.push(buf);
        }

        let indexed = lzss::decompress(compressed, md.uncompressed_len as usize);
        let mut pixels = Vec::with_capacity(size);
        for index in indexed {
            pixels.extend_from_slice(&palette[index as usize]);
        }

        Ok(Image {
            width: md.width,
            height: md.height,
            name: name.to_owned().into(),
            data: pixels.into(),
        })
    }
}

// TODO
pub struct GgdHigh;
