#![allow(clippy::too_many_arguments)]

use anyhow::{Result, bail};
use reikura_util::{
    image::{blend_premultiplied_color, get_alpha, premul_color},
    rect::Rect,
};

pub const MAX_IMAGE: usize = 128;

pub struct Image<B = Vec<u32>> {
    pub width: u32,
    pub height: u32,
    pub(crate) buffer: B,
}

impl<B> Image<B> {
    pub fn rect(&self) -> Rect {
        Rect::from_xywh(0, 0, self.width, self.height)
    }
}

impl Image {
    pub fn new(width: u32, height: u32) -> Self {
        Self {
            width,
            height,
            buffer: vec![super::color::BLACK; width as usize * height as usize],
        }
    }

    pub fn from_bytes(width: u32, height: u32, bytes: &[u8]) -> anyhow::Result<Self> {
        if bytes.len() != width as usize * height as usize * size_of::<u32>() {
            bail!("invalid bytes length");
        }

        let buffer = unsafe {
            let ptr = bytes.as_ptr().cast::<u32>();
            let len = bytes.len() / size_of::<u32>();
            std::slice::from_raw_parts(ptr, len).to_vec()
        };

        Ok(Self {
            width,
            height,
            buffer,
        })
    }

    pub fn dimension(&self) -> (u32, u32) {
        (self.width, self.height)
    }
}

impl<P: AsRef<[u32]> + AsMut<[u32]>> Image<P> {
    pub fn from_buffer(width: u32, height: u32, buffer: P) -> anyhow::Result<Self> {
        if buffer.as_ref().len() != width as usize * height as usize {
            bail!("invalid buffer length");
        }

        Ok(Self {
            width,
            height,
            buffer,
        })
    }

    #[inline]
    pub fn clear(&mut self, color: u32) {
        self.buffer.as_mut().fill(color);
    }

    #[inline]
    pub fn clear_rect(&mut self, color: u32, x: u32, y: u32, w: u32, h: u32) {
        let buffer = self.buffer.as_mut();
        let rows = buffer
            .chunks_exact_mut(self.width as usize)
            .skip(y as usize)
            .take(h as usize)
            .map(|row| &mut row[x as usize..][..w as usize]);

        for row in rows {
            row.fill(color);
        }
    }

    #[inline]
    pub fn draw_rect(&mut self, mut color: u32, x: u32, y: u32, w: u32, h: u32) {
        match get_alpha(color) {
            0x00 => return,
            0xFF => {
                self.clear_rect(color, x, y, w, h);
                return;
            }
            _ => color = premul_color(color),
        }

        let buffer = self.buffer.as_mut();
        let rows = buffer
            .chunks_exact_mut(self.width as usize)
            .skip(y as usize)
            .take(h as usize)
            .map(|row| &mut row[x as usize..][..w as usize]);

        for row in rows {
            for dst_color in row {
                *dst_color = blend_premultiplied_color(*dst_color, color);
            }
        }
    }
}

pub struct ImageRegistry {
    pub main: Option<u8>,
    images: [Option<Image>; MAX_IMAGE],
}

impl ImageRegistry {
    pub fn new() -> Self {
        Self {
            main: None,
            images: std::array::from_fn(|_| None),
        }
    }

    #[inline]
    pub fn unset_main(&mut self) {
        self.main = None;
    }

    #[inline]
    pub fn set_main(&mut self, id: u8) {
        self.main = Some(id % MAX_IMAGE as u8);
    }

    #[inline]
    #[allow(dead_code)]
    pub fn get_ref(&self, id: u8) -> Option<&Image> {
        self.images[id as usize % MAX_IMAGE].as_ref()
    }

    #[inline]
    pub fn get_mut(&mut self, id: u8) -> Option<&mut Image> {
        self.images[id as usize % MAX_IMAGE].as_mut()
    }

    #[inline]
    pub fn set(&mut self, id: u8, width: u32, height: u32, data: Option<&[u8]>) -> Result<()> {
        let image = match data {
            Some(bytes) => Image::from_bytes(width, height, bytes)?,
            None => Image::new(width, height),
        };

        self.images[id as usize % MAX_IMAGE] = Some(image);

        Ok(())
    }

    #[inline]
    pub fn get_main_ref(&mut self) -> Option<&Image> {
        self.images[self.main? as usize].as_ref()
    }

    #[inline]
    pub fn get_main_mut(&mut self) -> Option<&mut Image> {
        self.images[self.main? as usize].as_mut()
    }

    #[inline]
    pub fn get_pair(&mut self, src_id: u8, dst_id: Option<u8>) -> Option<(&mut Image, &mut Image)> {
        let indices = [
            src_id as usize % MAX_IMAGE,
            dst_id.or(self.main)? as usize % MAX_IMAGE,
        ];

        match self.images.get_disjoint_mut(indices) {
            Ok([Some(src), Some(dst)]) => Some((src, dst)),
            _ => None,
        }
    }
}
