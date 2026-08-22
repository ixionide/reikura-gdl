use std::io::{Cursor, Read};

use anyhow::Result;
use reikura_util::{
    image::{PIXEL_STRIDE, copy_previous_pixels},
    io::ReadExt,
};

use crate::image::ImageDecoder;

pub struct Gga;

pub struct GgaMetadata {
    magic: [u8; 8],
    width: u16,
    height: u16,
    _unknown: u16,
    bpp: u8,
    flags: u8,
    pixel_offset: u32,
    compressed_len: u32,
}

impl ImageDecoder for Gga {
    const MAGIC: &[u8] = b"GGA00000";
    type Metadata = GgaMetadata;

    fn parse(mut data: &[u8]) -> Result<Self::Metadata> {
        Ok(GgaMetadata {
            magic: data.get_le()?,
            width: data.get_le()?,
            height: data.get_le()?,
            _unknown: data.get_le()?,
            bpp: data.get_le()?,
            flags: data.get_le()?,
            pixel_offset: data.get_le()?,
            compressed_len: data.get_le()?,
        })
    }

    fn decode(md: Self::Metadata, data: &[u8]) -> Result<(u32, u32, Vec<u8>)> {
        debug_assert_eq!(md.magic, Self::MAGIC);
        debug_assert_eq!(md.bpp, 32);

        let opaque = (md.flags & 1) == 0;
        let compressed = &data[md.pixel_offset as usize..][..md.compressed_len as usize];
        let mut pixels = Vec::with_capacity(md.width as usize * md.height as usize * PIXEL_STRIDE);
        let mut cursor = Cursor::new(compressed);
        let mut buf = [0xFF; PIXEL_STRIDE];

        while cursor.position() < md.compressed_len as u64 {
            let cmd = cursor.get_le::<u8>()?;
            match cmd {
                0x0 => {
                    let pos = 1;
                    let count = cursor.get_le::<u8>()? as usize;
                    copy_previous_pixels(&mut pixels, pos, count);
                }
                0x1 => {
                    let pos = 1;
                    let count = cursor.get_le::<u16>()? as usize;
                    copy_previous_pixels(&mut pixels, pos, count);
                }
                0x2 => {
                    let pos = cursor.get_le::<u8>()? as usize;
                    let count = 1;
                    copy_previous_pixels(&mut pixels, pos, count);
                }
                0x3 => {
                    let pos = cursor.get_le::<u16>()? as usize;
                    let count = 1;
                    copy_previous_pixels(&mut pixels, pos, count);
                }
                0x4 => {
                    let pos = cursor.get_le::<u8>()? as usize;
                    let count = cursor.get_le::<u8>()? as usize;
                    copy_previous_pixels(&mut pixels, pos, count);
                }
                0x5 => {
                    let pos = cursor.get_le::<u8>()? as usize;
                    let count = cursor.get_le::<u16>()? as usize;
                    copy_previous_pixels(&mut pixels, pos, count);
                }
                0x6 => {
                    let pos = cursor.get_le::<u16>()? as usize;
                    let count = cursor.get_le::<u8>()? as usize;
                    copy_previous_pixels(&mut pixels, pos, count)
                }
                0x7 => {
                    let pos = cursor.get_le::<u16>()? as usize;
                    let count = cursor.get_le::<u16>()? as usize;
                    copy_previous_pixels(&mut pixels, pos, count);
                }
                0x8 => {
                    let pos = 1;
                    let count = 1;
                    copy_previous_pixels(&mut pixels, pos, count);
                }
                0x9 => {
                    let pos = md.width as usize;
                    let count = 1;
                    copy_previous_pixels(&mut pixels, pos, count);
                }
                0xA => {
                    let pos = md.width as usize + 1;
                    let count = 1;
                    copy_previous_pixels(&mut pixels, pos, count);
                }
                0xB => {
                    let pos = md.width as usize - 1;
                    let count = 1;
                    copy_previous_pixels(&mut pixels, pos, count);
                }
                num => {
                    let count = num - 11;
                    for _ in 0..count {
                        cursor.read_exact(&mut buf)?;

                        buf.swap(0, 2); // convert bgr -> rgb
                        if opaque {
                            buf[3] = 0xFF;
                        }

                        pixels.extend_from_slice(&buf);
                    }
                }
            }
        }

        Ok((md.width as _, md.height as _, pixels))
    }
}
