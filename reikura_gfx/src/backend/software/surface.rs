use anyhow::bail;
use reikura_util::{image::blend_color, io::ReadExt};

use crate::Rect;

pub struct Surface {
    pub width: u16,
    pub height: u16,
    pub pixels: Vec<u32>,
}

impl Surface {
    pub fn new(width: u16, height: u16) -> Self {
        const TRANSPARENT: u32 = 0x00_00_00_00;

        Self {
            width,
            height,
            pixels: vec![TRANSPARENT; (width * height) as usize],
        }
    }

    pub fn new_black(width: u16, height: u16) -> Self {
        const BLACK: u32 = 0xFF_00_00_00;

        Self {
            width,
            height,
            pixels: vec![BLACK; (width * height) as usize],
        }
    }

    // assume the bytes is in rgba format
    pub fn from_bytes(width: u16, height: u16, mut bytes: &[u8]) -> anyhow::Result<Self> {
        let len = (width * height) as usize;

        if bytes.len() != len {
            bail!("invalid bytes length");
        }

        let mut pixels = vec![0_u32; len];
        for pixel in &mut pixels {
            *pixel = bytes.read_le().unwrap();
        }

        Ok(Self {
            width,
            height,
            pixels,
        })
    }

    pub fn rect(&self) -> Rect {
        Rect::from_xywh(0, 0, self.width, self.height)
    }

    pub fn fill(&mut self, color: u32) {
        self.pixels.fill(color);
    }

    pub fn fill_rect(&mut self, rect: Rect, color: u32) {
        let copy_len = rect.width() as usize;

        for i in rect.top as usize..rect.bottom as usize {
            let copy_pos = rect.left as usize + i * self.width as usize;
            self.pixels[copy_pos..][..copy_len].fill(color);
        }
    }

    pub fn blit_copy(&mut self, src_rect: Rect, dst_rect: Rect, src: &Surface) {
        let Some((rect_w, rect_h)) = src_rect.same_size(dst_rect) else {
            return;
        };

        let copy_len = rect_w as usize;
        let src_stride = src.width as usize;
        let dst_stride = self.width as usize;

        let src_start = src_rect.left as usize + src_rect.top as usize * src_stride;
        let dst_start = dst_rect.left as usize + dst_rect.top as usize * dst_stride;

        for i in 0..rect_h as usize {
            let src_pos = src_start + i * src_stride;
            let dst_pos = dst_start + i * dst_stride;
            self.pixels[dst_pos..][..copy_len].copy_from_slice(&src.pixels[src_pos..][..copy_len]);
        }
    }

    pub fn blit_blend(&mut self, src_rect: Rect, dst_rect: Rect, src: &Surface) {
        let Some((rect_w, rect_h)) = src_rect.same_size(dst_rect) else {
            return;
        };

        let src_stride = src.width as usize;
        let dst_stride = self.width as usize;

        let src_start = src_rect.left as usize + src_rect.top as usize * src_stride;
        let dst_start = dst_rect.left as usize + dst_rect.top as usize * dst_stride;

        for y in 0..rect_h as usize {
            let src_pos = src_start + y * src_stride;
            let dst_pos = dst_start + y * dst_stride;

            for x in 0..rect_w as usize {
                let src_color = src.pixels[src_pos + x];

                // if transparent
                if src_color >> 24 == 0 {
                    continue;
                }

                let dst_color = &mut self.pixels[dst_pos + x];
                *dst_color = blend_color(*dst_color, src_color);
            }
        }
    }

    pub fn blit_scale_copy(&mut self, src_rect: Rect, dst_rect: Rect, src: &Surface) {
        let src_w = src_rect.width() as u64;
        let src_h = src_rect.height() as u64;
        let dst_w = dst_rect.width() as u64;
        let dst_h = dst_rect.height() as u64;

        if dst_w == 0 || dst_h == 0 {
            return;
        }

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

                self.pixels[dst_idx] = src.pixels[src_idx];
                posx += incx;
            }

            posy += incy;
        }
    }

    pub fn blit_scale_blend(&mut self, src_rect: Rect, dst_rect: Rect, src: &Surface) {
        let src_w = src_rect.width() as u64;
        let src_h = src_rect.height() as u64;
        let dst_w = dst_rect.width() as u64;
        let dst_h = dst_rect.height() as u64;

        if dst_w == 0 || dst_h == 0 {
            return;
        }

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

                let src_color = src.pixels[src_row * src_stride + src_start_x + srcx];

                // if transparent
                if src_color >> 24 == 0 {
                    posx += incx;
                    continue;
                }

                let dst_color = &mut self.pixels[dst_row * dst_stride + dst_start_x + dx];
                *dst_color = blend_color(*dst_color, src_color);
                posx += incx;
            }

            posy += incy;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const TRANSPARENT: u32 = 0x00_00_00_00;
    const RED: u32 = 0xFF_00_00_FF;
    const GREEN: u32 = 0xFF_00_FF_00;
    const BLUE: u32 = 0xFF_FF_00_00;
    const BLACK: u32 = 0xFF_00_00_00;
    const BLACK_50: u32 = 0x7F_00_00_00;
    const WHITE: u32 = 0xFF_FF_FF_FF;
    const GRAY: u32 = blend_color(WHITE, BLACK_50);

    fn create_test_surface(width: u16, height: u16, color: u32) -> Surface {
        let mut surface = Surface::new(width, height);
        surface.fill(color);

        surface
    }

    #[test]
    fn test_fill_rect() {
        let mut dst = create_test_surface(4, 4, BLUE);

        let rect = Rect::from_xywh(1, 1, 2, 2);
        dst.fill_rect(rect, GRAY);

        #[rustfmt::skip]
        let expected = [
            BLUE, BLUE, BLUE, BLUE,
            BLUE, GRAY, GRAY, BLUE,
            BLUE, GRAY, GRAY, BLUE,
            BLUE, BLUE, BLUE, BLUE,
        ];

        assert_eq!(dst.pixels, expected);
    }

    #[test]
    fn test_blit_copy() {
        let src = create_test_surface(4, 4, RED);
        let mut dst = create_test_surface(4, 4, BLUE);

        let src_rect = Rect::from_xywh(0, 0, 2, 2);
        let dst_rect = Rect::from_xywh(0, 0, 2, 2);

        dst.blit_copy(src_rect, dst_rect, &src);

        #[rustfmt::skip]
        let expected = [
            RED, RED, BLUE, BLUE,
            RED, RED, BLUE, BLUE,
            BLUE, BLUE, BLUE, BLUE,
            BLUE, BLUE, BLUE, BLUE,
        ];

        assert_eq!(dst.pixels, expected);
    }

    #[test]
    fn test_blit_blend() {
        let mut src = create_test_surface(3, 4, BLACK_50);
        src.fill_rect(Rect::from_xywh(0, 0, 1, 1), BLACK);
        let mut dst = create_test_surface(3, 4, WHITE);

        let src_rect = Rect::from_xywh(0, 0, 1, 2);
        let dst_rect = Rect::from_xywh(1, 1, 1, 2);

        dst.blit_blend(src_rect, dst_rect, &src);

        #[rustfmt::skip]
        let expected = [
            WHITE, WHITE, WHITE,
            WHITE, BLACK, WHITE,
            WHITE, GRAY, WHITE,
            WHITE, WHITE, WHITE,
        ];

        assert_eq!(dst.pixels, expected);
    }

    #[test]
    fn test_blit_blend_transparrent() {
        let src = create_test_surface(3, 4, TRANSPARENT);
        let mut dst = create_test_surface(3, 4, GREEN);

        let rect = Rect::from_xywh(0, 0, 2, 2);

        dst.blit_blend(rect, rect, &src);

        #[rustfmt::skip]
        let expected = [
            GREEN, GREEN, GREEN,
            GREEN, GREEN, GREEN,
            GREEN, GREEN, GREEN,
            GREEN, GREEN, GREEN,
        ];

        assert_eq!(dst.pixels, expected);
    }

    #[test]
    fn test_blit_scale_copy() {
        let src = create_test_surface(4, 4, BLACK);
        let mut dst = create_test_surface(8, 8, WHITE);

        let src_rect = Rect::from_xywh(0, 0, 4, 4);
        let dst_rect = Rect::from_xywh(1, 1, 3, 2);

        dst.blit_scale_copy(src_rect, dst_rect, &src);

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

        assert_eq!(dst.pixels, expected);
    }

    #[test]
    fn test_blit_scale_blend() {
        let mut src = create_test_surface(4, 4, BLACK_50);
        src.fill_rect(Rect::from_xywh(0, 0, 1, 1), GREEN);
        let mut dst = create_test_surface(8, 8, WHITE);

        let src_rect = Rect::from_xywh(0, 0, 2, 2);
        let dst_rect = Rect::from_xywh(3, 4, 4, 4);

        dst.blit_scale_blend(src_rect, dst_rect, &src);

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

        assert_eq!(dst.pixels, expected);
    }
}
