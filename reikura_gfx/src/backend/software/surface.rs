use anyhow::bail;
use reikura_util::image::{blend_color, blend_premultiplied_color, premultiply_color};

use crate::Rect;

pub struct Surface {
    pub width: u16,
    pub height: u16,
    pub pixels: Vec<u32>, // NOTE: consider making this generic
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

    // assume the format is rgba8
    pub fn from_bytes(width: u16, height: u16, bytes: &[u8]) -> anyhow::Result<Self> {
        if bytes.len() != (width * height) as usize * size_of::<u32>() {
            bail!("invalid bytes length");
        }

        let pixels = bytemuck::cast_slice(bytes).to_vec();

        Ok(Self {
            width,
            height,
            pixels,
        })
    }

    pub fn rect(&self) -> Rect {
        Rect::from_xywh(0, 0, self.width, self.height)
    }

    pub fn clear(&mut self, color: u32) {
        self.pixels.fill(color);
    }

    pub fn fill_rect_copy(&mut self, rect: Rect, color: u32) -> anyhow::Result<()> {
        if !self.rect().contains(rect) {
            bail!("rect is outside of surface");
        }

        let copy_len = rect.width() as usize;

        for i in rect.top as usize..rect.bottom as usize {
            let copy_pos = rect.left as usize + i * self.width as usize;
            self.pixels[copy_pos..][..copy_len].fill(color);
        }

        Ok(())
    }

    pub fn fill_rect_blend(&mut self, rect: Rect, color: u32) -> anyhow::Result<()> {
        if !self.rect().contains(rect) {
            bail!("rect is outside of surface");
        }

        // if opaque
        if color >> 24 == 255 {
            self.fill_rect_copy(rect, color)?;
            return Ok(());
        }

        let color = premultiply_color(color);
        let h = rect.width() as usize;
        let w = rect.width() as usize;
        let stride = self.width as usize;
        let start = rect.left as usize + rect.top as usize * stride;

        for y in 0..h as usize {
            let pos = start + y * stride;

            for x in 0..w {
                let dst_color = &mut self.pixels[pos + x];
                *dst_color = blend_premultiplied_color(*dst_color, color);
            }
        }

        Ok(())
    }

    pub fn blit_copy(
        &mut self,
        src_rect: Rect,
        dst_rect: Rect,
        src: &Surface,
    ) -> anyhow::Result<()> {
        if !self.rect().contains(dst_rect) || !src.rect().contains(src_rect) {
            bail!("rect is outside of surface");
        }

        let Some((rect_w, rect_h)) = src_rect.same_size(dst_rect) else {
            bail!("src_rect is not the same size as dst_rect");
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

        Ok(())
    }

    pub fn blit_blend(
        &mut self,
        src_rect: Rect,
        dst_rect: Rect,
        src: &Surface,
    ) -> anyhow::Result<()> {
        if !self.rect().contains(dst_rect) || !src.rect().contains(src_rect) {
            bail!("rect is outside of surface");
        }

        let Some((rect_w, rect_h)) = src_rect.same_size(dst_rect) else {
            bail!("src_rect is not the same size as dst_rect");
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

        Ok(())
    }

    pub fn blit_scale_copy(
        &mut self,
        src_rect: Rect,
        dst_rect: Rect,
        src: &Surface,
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

        Ok(())
    }

    pub fn blit_scale_blend(
        &mut self,
        src_rect: Rect,
        dst_rect: Rect,
        src: &Surface,
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

        Ok(())
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
    const NAVY: u32 = blend_color(BLUE, BLACK_50);

    fn create_test_surface(width: u16, height: u16, color: u32) -> Surface {
        let mut surface = Surface::new(width, height);
        surface.clear(color);

        surface
    }

    #[test]
    fn test_fill_rect_copy() {
        let mut dst = create_test_surface(4, 4, BLUE);

        let rect = Rect::from_xywh(1, 1, 2, 2);
        dst.fill_rect_copy(rect, GRAY).unwrap();

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
    fn test_fill_rect_blend() {
        let mut dst = create_test_surface(4, 4, BLUE);

        let rect = Rect::from_xywh(1, 1, 2, 2);
        dst.fill_rect_blend(rect, BLACK_50).unwrap();

        #[rustfmt::skip]
        let expected = [
            BLUE, BLUE, BLUE, BLUE,
            BLUE, NAVY, NAVY, BLUE,
            BLUE, NAVY, NAVY, BLUE,
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

        dst.blit_copy(src_rect, dst_rect, &src).unwrap();

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
        src.fill_rect_copy(Rect::from_xywh(0, 0, 1, 1), BLACK)
            .unwrap();
        let mut dst = create_test_surface(3, 4, WHITE);

        let src_rect = Rect::from_xywh(0, 0, 1, 2);
        let dst_rect = Rect::from_xywh(1, 1, 1, 2);

        dst.blit_blend(src_rect, dst_rect, &src).unwrap();

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

        dst.blit_blend(rect, rect, &src).unwrap();

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

        assert_eq!(dst.pixels, expected);
    }

    #[test]
    fn test_blit_scale_blend() {
        let mut src = create_test_surface(4, 4, BLACK_50);
        src.fill_rect_copy(Rect::from_xywh(0, 0, 1, 1), GREEN)
            .unwrap();
        let mut dst = create_test_surface(8, 8, WHITE);

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

        assert_eq!(dst.pixels, expected);
    }
}
