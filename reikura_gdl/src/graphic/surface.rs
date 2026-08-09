use anyhow::bail;
use reikura_util::{
    image::{blend_color, blend_premultiplied_color, mul_alpha, premultiply_color},
    rect::Rect,
};

#[inline]
fn alpha_channel(color: u32) -> u8 {
    (color >> 24) as u8
}

pub struct Surface<P = Vec<u32>> {
    pub width: u32,
    pub height: u32,
    pixels: P,
}

impl<P> Surface<P> {
    pub fn rect(&self) -> Rect {
        Rect::from_xywh(0, 0, self.width, self.height)
    }
}

impl Surface {
    pub fn new(width: u32, height: u32) -> Self {
        Self {
            width,
            height,
            pixels: vec![0; width as usize * height as usize],
        }
    }

    pub fn from_bytes(width: u32, height: u32, bytes: &[u8]) -> anyhow::Result<Self> {
        if bytes.len() != width as usize * height as usize * size_of::<u32>() {
            bail!("invalid bytes length");
        }

        let pixels = unsafe {
            let ptr = bytes.as_ptr().cast::<u32>();
            let len = bytes.len() / size_of::<u32>();
            std::slice::from_raw_parts(ptr, len).to_vec()
        };

        Ok(Self {
            width,
            height,
            pixels,
        })
    }

    pub fn dimension(&self) -> (u32, u32) {
        (self.width, self.height)
    }
}

impl<P: AsRef<[u32]> + AsMut<[u32]>> Surface<P> {
    pub fn from_pixels(width: u32, height: u32, pixels: P) -> anyhow::Result<Self> {
        if pixels.as_ref().len() != width as usize * height as usize {
            bail!("invalid pixels length");
        }

        Ok(Self {
            width,
            height,
            pixels,
        })
    }

    #[inline]
    pub fn clear(&mut self, color: u32) {
        self.pixels.as_mut().fill(color);
    }

    #[inline]
    pub fn clear_rect(&mut self, x: u32, y: u32, w: u32, h: u32, color: u32) {
        let pixels = self.pixels.as_mut();
        let rows = pixels
            .chunks_exact_mut(self.width as usize)
            .skip(y as usize)
            .take(h as usize)
            .map(|row| &mut row[x as usize..][..w as usize]);

        for row in rows {
            row.fill(color);
        }
    }

    #[inline]
    pub fn draw_rect(&mut self, x: u32, y: u32, w: u32, h: u32, mut color: u32) {
        match alpha_channel(color) {
            0x00 => return,
            0xFF => {
                self.clear_rect(x, y, w, h, color);
                return;
            }
            _ => color = premultiply_color(color),
        }

        let pixels = self.pixels.as_mut();
        let rows = pixels
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
    #[allow(clippy::too_many_arguments)]
    pub fn blit_copy<T: AsRef<[u32]>>(
        &mut self,
        src_x: u32,
        src_y: u32,
        blit_w: u32,
        blit_h: u32,
        dst_x: u32,
        dst_y: u32,
        src: &Surface<T>,
    ) {
        let src_pixels = src.pixels.as_ref();
        let dst_pixels = self.pixels.as_mut();

        let src_rows = src_pixels
            .chunks_exact(src.width as usize)
            .skip(src_y as usize)
            .take(blit_h as usize)
            .map(|row| &row[src_x as usize..][..blit_w as usize]);

        let dst_rows = dst_pixels
            .chunks_exact_mut(src.width as usize)
            .skip(dst_y as usize)
            .take(blit_h as usize)
            .map(|row| &mut row[dst_x as usize..][..blit_w as usize]);

        for (src_row, dst_row) in src_rows.zip(dst_rows) {
            dst_row.copy_from_slice(src_row);
        }
    }

    #[inline]
    #[allow(clippy::too_many_arguments)]
    pub fn blit_blend<T: AsRef<[u32]>>(
        &mut self,
        src_x: u32,
        src_y: u32,
        blit_w: u32,
        blit_h: u32,
        dst_x: u32,
        dst_y: u32,
        src: &Surface<T>,
    ) {
        let src_pixels = src.pixels.as_ref();
        let dst_pixels = self.pixels.as_mut();

        let src_rows = src_pixels
            .chunks_exact(src.width as usize)
            .skip(src_y as usize)
            .take(blit_h as usize)
            .map(|row| &row[src_x as usize..][..blit_w as usize]);

        let dst_rows = dst_pixels
            .chunks_exact_mut(src.width as usize)
            .skip(dst_y as usize)
            .take(blit_h as usize)
            .map(|row| &mut row[dst_x as usize..][..blit_w as usize]);

        for (src_row, dst_row) in src_rows.zip(dst_rows) {
            let src_row = src_row.iter().copied();
            let dst_row = dst_row.iter_mut();

            for (dst_color, src_color) in dst_row.zip(src_row) {
                match alpha_channel(src_color) {
                    0x00 => continue,
                    0xFF => *dst_color = src_color,
                    _ => *dst_color = blend_color(*dst_color, src_color),
                }
            }
        }
    }

    #[inline]
    #[allow(clippy::too_many_arguments)]
    pub fn blit_blend_alpha<T: AsRef<[u32]>>(
        &mut self,
        src_x: u32,
        src_y: u32,
        blit_w: u32,
        blit_h: u32,
        dst_x: u32,
        dst_y: u32,
        alpha: u8,
        src: &Surface<T>,
    ) {
        if alpha == 0xFF {
            self.blit_blend(src_x, src_y, blit_w, blit_h, dst_x, dst_y, src);
        }

        let src_pixels = src.pixels.as_ref();
        let dst_pixels = self.pixels.as_mut();

        let src_rows = src_pixels
            .chunks_exact(src.width as usize)
            .skip(src_y as usize)
            .take(blit_h as usize)
            .map(|row| &row[src_x as usize..][..blit_w as usize]);

        let dst_rows = dst_pixels
            .chunks_exact_mut(src.width as usize)
            .skip(dst_y as usize)
            .take(blit_h as usize)
            .map(|row| &mut row[dst_x as usize..][..blit_w as usize]);

        for (src_row, dst_row) in src_rows.zip(dst_rows) {
            let src_row = src_row.iter().copied();
            let dst_row = dst_row.iter_mut();

            for (dst_color, src_color) in dst_row.zip(src_row) {
                match alpha_channel(src_color) {
                    0x00 => continue,
                    _ => {
                        let src_color = mul_alpha(src_color, alpha);
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
        src: &Surface<T>,
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

        let src_pixels = src.pixels.as_ref();
        let dst_pixels = self.pixels.as_mut();

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

                dst_pixels[dst_idx] = src_pixels[src_idx];
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
        src: &Surface<T>,
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

        let src_pixels = src.pixels.as_ref();
        let dst_pixels = self.pixels.as_mut();

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

                let src_color = src_pixels[src_row * src_stride + src_start_x + srcx];

                // if transparent
                if alpha_channel(src_color) == 0x00 {
                    posx += incx;
                    continue;
                }

                let dst_color = &mut dst_pixels[dst_row * dst_stride + dst_start_x + dx];
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

    fn create_test_surface(width: u32, height: u32, color: u32) -> Surface {
        let mut surface = Surface::new(width, height);
        surface.clear(color);
        surface
    }

    #[test]
    fn test_clear_rect() {
        let mut dst = create_test_surface(4, 4, BLUE);

        dst.clear_rect(1, 1, 2, 2, GRAY);

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
    fn test_draw_rect() {
        let mut dst = create_test_surface(4, 4, BLUE);

        dst.draw_rect(1, 1, 2, 2, BLACK_50);

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

        dst.blit_copy(0, 0, 2, 2, 0, 0, &src);

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
        src.clear_rect(0, 0, 1, 1, BLACK);
        let mut dst = create_test_surface(3, 4, WHITE);

        dst.blit_blend(0, 0, 1, 2, 1, 1, &src);

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

        dst.blit_blend(0, 0, 2, 2, 0, 0, &src);

        #[rustfmt::skip]
        let expected = [
            GREEN, GREEN, GREEN,
            GREEN, GREEN, GREEN,
            GREEN, GREEN, GREEN,
            GREEN, GREEN, GREEN,
        ];

        assert_eq!(dst.pixels, expected);
    }

    // #[test]
    // fn test_blit_blend_alpha() {
    // }

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
        src.clear_rect(0, 0, 1, 1, GREEN);
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
