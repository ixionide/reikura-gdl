#![allow(clippy::too_many_arguments)]

use anyhow::{Result, bail};
use reikura_util::{
    image::{blend_color, blend_premultiplied_color, get_alpha, mul_alpha_color, premul_color},
    rect::Rect,
};

pub const MAX_IMAGE: usize = 128;

pub mod color {
    pub const TRANSPARENT: u32 = 0x00_00_00_00;
    pub const RED: u32 = 0xFF_00_00_FF;
    pub const GREEN: u32 = 0xFF_00_FF_00;
    pub const BLUE: u32 = 0xFF_FF_00_00;
    pub const BLACK: u32 = 0xFF_00_00_00;
    pub const BLACK_50: u32 = 0x7F_00_00_00;
    pub const WHITE: u32 = 0xFF_FF_FF_FF;
    pub const GRAY: u32 = reikura_util::image::blend_color(WHITE, BLACK_50);
    pub const NAVY: u32 = reikura_util::image::blend_color(BLUE, BLACK_50);
}

pub struct Image<B = Vec<u32>> {
    pub width: u32,
    pub height: u32,
    buffer: B,
}

impl<P> Image<P> {
    pub fn rect(&self) -> Rect {
        Rect::from_xywh(0, 0, self.width, self.height)
    }
}

impl Image {
    pub fn new(width: u32, height: u32) -> Self {
        Self {
            width,
            height,
            buffer: vec![color::BLACK; width as usize * height as usize],
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

    #[inline]
    pub fn blit_copy<T: AsRef<[u32]>>(
        &mut self,
        src: &Image<T>,
        src_x: u32,
        src_y: u32,
        dst_x: u32,
        dst_y: u32,
        blit_w: u32,
        blit_h: u32,
    ) {
        let src_buffer = src.buffer.as_ref();
        let dst_buffer = self.buffer.as_mut();

        let src_rows = src_buffer
            .chunks_exact(src.width as usize)
            .skip(src_y as usize)
            .take(blit_h as usize)
            .map(|row| &row[src_x as usize..][..blit_w as usize]);

        let dst_rows = dst_buffer
            .chunks_exact_mut(self.width as usize)
            .skip(dst_y as usize)
            .take(blit_h as usize)
            .map(|row| &mut row[dst_x as usize..][..blit_w as usize]);

        for (src_row, dst_row) in src_rows.zip(dst_rows) {
            dst_row.copy_from_slice(src_row);
        }
    }

    #[inline]
    pub fn blit_blend<T: AsRef<[u32]>>(
        &mut self,
        src: &Image<T>,
        src_x: u32,
        src_y: u32,
        dst_x: u32,
        dst_y: u32,
        blit_w: u32,
        blit_h: u32,
    ) {
        let src_buffer = src.buffer.as_ref();
        let dst_buffer = self.buffer.as_mut();

        let src_rows = src_buffer
            .chunks_exact(src.width as usize)
            .skip(src_y as usize)
            .take(blit_h as usize)
            .map(|row| &row[src_x as usize..][..blit_w as usize]);

        let dst_rows = dst_buffer
            .chunks_exact_mut(self.width as usize)
            .skip(dst_y as usize)
            .take(blit_h as usize)
            .map(|row| &mut row[dst_x as usize..][..blit_w as usize]);

        for (src_row, dst_row) in src_rows.zip(dst_rows) {
            let src_row = src_row.iter().copied();
            let dst_row = dst_row.iter_mut();

            for (dst_color, src_color) in dst_row.zip(src_row) {
                match get_alpha(src_color) {
                    0x00 => continue,
                    0xFF => *dst_color = src_color,
                    _ => *dst_color = blend_color(*dst_color, src_color),
                }
            }
        }
    }

    #[inline]
    pub fn blit_blend_alpha<T: AsRef<[u32]>>(
        &mut self,
        src: &Image<T>,
        src_x: u32,
        src_y: u32,
        dst_x: u32,
        dst_y: u32,
        blit_w: u32,
        blit_h: u32,
        alpha: u8,
    ) {
        if alpha == 0xFF {
            self.blit_blend(src, src_x, src_y, dst_x, dst_y, blit_w, blit_h);
        }

        let src_buffer = src.buffer.as_ref();
        let dst_buffer = self.buffer.as_mut();

        let src_rows = src_buffer
            .chunks_exact(src.width as usize)
            .skip(src_y as usize)
            .take(blit_h as usize)
            .map(|row| &row[src_x as usize..][..blit_w as usize]);

        let dst_rows = dst_buffer
            .chunks_exact_mut(self.width as usize)
            .skip(dst_y as usize)
            .take(blit_h as usize)
            .map(|row| &mut row[dst_x as usize..][..blit_w as usize]);

        for (src_row, dst_row) in src_rows.zip(dst_rows) {
            let src_row = src_row.iter().copied();
            let dst_row = dst_row.iter_mut();

            for (dst_color, src_color) in dst_row.zip(src_row) {
                match get_alpha(src_color) {
                    0x00 => continue,
                    _ => {
                        let src_color = mul_alpha_color(src_color, alpha);
                        *dst_color = blend_color(*dst_color, src_color);
                    }
                }
            }
        }
    }

    pub fn blit_scale_copy<T: AsRef<[u32]>>(
        &mut self,
        src_rect: Rect,
        dst_rect: Rect,
        src: &Image<T>,
    ) -> anyhow::Result<()> {
        if !self.rect().contains(dst_rect) || !src.rect().contains(src_rect) {
            bail!("rect is outside of surface");
        }

        let src_w = src_rect.width() as u64;
        let src_h = src_rect.height() as u64;
        let dst_w = dst_rect.width() as u64;
        let dst_h = dst_rect.height() as u64;

        if dst_w == 0 || dst_h == 0 {
            return Ok(());
        }

        let src_buffer = src.buffer.as_ref();
        let dst_buffer = self.buffer.as_mut();

        let src_stride = src.width as usize;
        let dst_stride = self.width as usize;

        let incy = (src_h << 16) / dst_h;
        let incx = (src_w << 16) / dst_w;

        let mut posy = incy / 2;
        let src_start_y = src_rect.top as usize;
        let src_start_x = src_rect.left as usize;
        let dst_start_y = dst_rect.top as usize;
        let dst_start_x = dst_rect.left as usize;

        for dy in 0..dst_h as usize {
            let srcy = (posy >> 16) as usize;
            let src_row = src_start_y + srcy;
            let dst_row = dst_start_y + dy;

            let mut posx = incx / 2;

            for dx in 0..dst_w as usize {
                let srcx = (posx >> 16) as usize;
                let src_idx = src_row * src_stride + src_start_x + srcx;
                let dst_idx = dst_row * dst_stride + dst_start_x + dx;

                dst_buffer[dst_idx] = src_buffer[src_idx];
                posx += incx;
            }

            posy += incy;
        }

        Ok(())
    }

    pub fn blit_scale_blend<T: AsRef<[u32]>>(
        &mut self,
        src_rect: Rect,
        dst_rect: Rect,
        src: &Image<T>,
    ) -> anyhow::Result<()> {
        if !self.rect().contains(dst_rect) || !src.rect().contains(src_rect) {
            bail!("rect is outside of surface");
        }

        let src_w = src_rect.width() as u64;
        let src_h = src_rect.height() as u64;
        let dst_w = dst_rect.width() as u64;
        let dst_h = dst_rect.height() as u64;

        if dst_w == 0 || dst_h == 0 {
            return Ok(());
        }

        let src_buffer = src.buffer.as_ref();
        let dst_buffer = self.buffer.as_mut();

        let src_stride = src.width as usize;
        let dst_stride = self.width as usize;

        let incy = (src_h << 16) / dst_h;
        let incx = (src_w << 16) / dst_w;

        let mut posy = incy / 2;
        let src_start_y = src_rect.top as usize;
        let src_start_x = src_rect.left as usize;
        let dst_start_y = dst_rect.top as usize;
        let dst_start_x = dst_rect.left as usize;

        for dy in 0..dst_h as usize {
            let srcy = (posy >> 16) as usize;
            let src_row = src_start_y + srcy;
            let dst_row = dst_start_y + dy;

            let mut posx = incx / 2;

            for dx in 0..dst_w as usize {
                let srcx = (posx >> 16) as usize;

                let src_color = src_buffer[src_row * src_stride + src_start_x + srcx];

                // if transparent
                if get_alpha(src_color) == 0x00 {
                    posx += incx;
                    continue;
                }

                let dst_color = &mut dst_buffer[dst_row * dst_stride + dst_start_x + dx];
                *dst_color = blend_color(*dst_color, src_color);
                posx += incx;
            }

            posy += incy;
        }

        Ok(())
    }
}

pub struct Images {
    pub main: Option<u8>,
    images: [Option<Image>; MAX_IMAGE],
}

impl Images {
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
    #[allow(dead_code)]
    pub fn get_main_mut(&mut self) -> Option<&mut Image> {
        self.images[self.main? as usize].as_mut()
    }

    #[inline]
    #[allow(dead_code)]
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

#[cfg(test)]
mod tests {
    use super::{color::*, *};

    fn create_test_image(width: u32, height: u32, color: u32) -> Image {
        let mut surface = Image::new(width, height);
        surface.clear(color);
        surface
    }

    #[test]
    fn test_clear_rect() {
        let mut dst = create_test_image(4, 4, BLUE);

        dst.clear_rect(GRAY, 1, 1, 2, 2);

        #[rustfmt::skip]
        let expected = [
            BLUE, BLUE, BLUE, BLUE,
            BLUE, GRAY, GRAY, BLUE,
            BLUE, GRAY, GRAY, BLUE,
            BLUE, BLUE, BLUE, BLUE,
        ];

        assert_eq!(dst.buffer, expected);
    }

    #[test]
    fn test_draw_rect() {
        let mut dst = create_test_image(4, 4, BLUE);

        dst.draw_rect(BLACK_50, 1, 1, 2, 2);

        #[rustfmt::skip]
        let expected = [
            BLUE, BLUE, BLUE, BLUE,
            BLUE, NAVY, NAVY, BLUE,
            BLUE, NAVY, NAVY, BLUE,
            BLUE, BLUE, BLUE, BLUE,
        ];

        assert_eq!(dst.buffer, expected);
    }

    #[test]
    fn test_blit_copy() {
        let src = create_test_image(4, 4, RED);
        let mut dst = create_test_image(4, 4, BLUE);

        dst.blit_copy(&src, 0, 0, 0, 0, 2, 2);

        #[rustfmt::skip]
        let expected = [
            RED, RED, BLUE, BLUE,
            RED, RED, BLUE, BLUE,
            BLUE, BLUE, BLUE, BLUE,
            BLUE, BLUE, BLUE, BLUE,
        ];

        assert_eq!(dst.buffer, expected);
    }

    #[test]
    fn test_blit_blend() {
        let mut src = create_test_image(3, 4, BLACK_50);
        src.clear_rect(BLACK, 0, 0, 1, 1);
        let mut dst = create_test_image(3, 4, WHITE);

        dst.blit_blend(&src, 0, 0, 1, 1, 1, 2);

        #[rustfmt::skip]
        let expected = [
            WHITE, WHITE, WHITE,
            WHITE, BLACK, WHITE,
            WHITE, GRAY, WHITE,
            WHITE, WHITE, WHITE,
        ];

        assert_eq!(dst.buffer, expected);
    }

    #[test]
    fn test_blit_blend_transparrent() {
        let src = create_test_image(3, 4, TRANSPARENT);
        let mut dst = create_test_image(3, 4, GREEN);

        dst.blit_blend(&src, 0, 0, 0, 0, 2, 2);

        #[rustfmt::skip]
        let expected = [
            GREEN, GREEN, GREEN,
            GREEN, GREEN, GREEN,
            GREEN, GREEN, GREEN,
            GREEN, GREEN, GREEN,
        ];

        assert_eq!(dst.buffer, expected);
    }

    // #[test]
    // fn test_blit_blend_alpha() {
    // }

    #[test]
    fn test_blit_scale_copy() {
        let src = create_test_image(4, 4, BLACK);
        let mut dst = create_test_image(8, 8, WHITE);

        let src_rect = Rect::from_xywh(0, 0, 4, 4);
        let dst_rect = Rect::from_xywh(1, 1, 3, 2);

        dst.blit_scale_copy(src_rect, dst_rect, &src).unwrap();

        #[rustfmt::skip]
        let expected = [
            WHITE, WHITE, WHITE, WHITE, WHITE, WHITE, WHITE, WHITE,
            WHITE, BLACK, BLACK, BLACK, WHITE, WHITE, WHITE, WHITE,
            WHITE, BLACK, BLACK, BLACK, WHITE, WHITE, WHITE, WHITE,
            WHITE, WHITE, WHITE, WHITE, WHITE, WHITE, WHITE, WHITE,
            WHITE, WHITE, WHITE, WHITE, WHITE, WHITE, WHITE, WHITE,
            WHITE, WHITE, WHITE, WHITE, WHITE, WHITE, WHITE, WHITE,
            WHITE, WHITE, WHITE, WHITE, WHITE, WHITE, WHITE, WHITE,
            WHITE, WHITE, WHITE, WHITE, WHITE, WHITE, WHITE, WHITE,
        ];

        assert_eq!(dst.buffer, expected);
    }

    #[test]
    fn test_blit_scale_blend() {
        let mut src = create_test_image(4, 4, BLACK_50);
        src.clear_rect(GREEN, 0, 0, 1, 1);
        let mut dst = create_test_image(8, 8, WHITE);

        let src_rect = Rect::from_xywh(0, 0, 2, 2);
        let dst_rect = Rect::from_xywh(3, 4, 4, 4);

        dst.blit_scale_blend(src_rect, dst_rect, &src).unwrap();

        #[rustfmt::skip]
        let expected = [
            WHITE, WHITE, WHITE, WHITE, WHITE, WHITE, WHITE, WHITE,
            WHITE, WHITE, WHITE, WHITE, WHITE, WHITE, WHITE, WHITE,
            WHITE, WHITE, WHITE, WHITE, WHITE, WHITE, WHITE, WHITE,
            WHITE, WHITE, WHITE, WHITE, WHITE, WHITE, WHITE, WHITE,
            WHITE, WHITE, WHITE, GREEN, GREEN, GRAY, GRAY, WHITE,
            WHITE, WHITE, WHITE, GREEN, GREEN, GRAY, GRAY, WHITE,
            WHITE, WHITE, WHITE, GRAY, GRAY, GRAY, GRAY, WHITE,
            WHITE, WHITE, WHITE, GRAY, GRAY, GRAY, GRAY, WHITE,
        ];

        assert_eq!(dst.buffer, expected);
    }
}
