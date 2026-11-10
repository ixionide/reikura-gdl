use reikura_util::image::{blend_color, get_alpha, mul_alpha};

use super::Image;

#[rustfmt::skip]
#[derive(Clone, Copy)]
pub struct BlitParam {
    pub(self) sx: i32, pub(self) sy: i32, pub(self) sw: i32, pub(self) sh: i32, pub(self) dx: i32, pub(self) dy: i32, pub(self) dw: i32, pub(self) dh: i32,
}

#[rustfmt::skip]
#[allow(clippy::too_many_arguments)]
impl BlitParam {
    pub fn new(x:i32, y:i32, w:i32, h:i32) -> Self {
        Self { sx: x, sy: y, sw: w, sh: h, dx: x, dy: y, dw: w, dh: h }
    }
    pub fn new_offset(sx: i32, sy: i32, w: i32, h: i32, dx: i32, dy: i32) -> Self {
        Self { sx, sy, sw: w, sh: h, dx, dy, dw: w, dh: h }
    }
    pub fn new_scaled(sx: i32, sy: i32, sw: i32, sh: i32, dx: i32, dy: i32, dw: i32, dh: i32) -> Self {
        Self { sx, sy, sw, sh, dx, dy, dw, dh }
    }
    pub fn is_scaled(&self) -> bool {
        self.sw != self.dw || self.sh != self.dh
    }
}

reikura_util::const_iota! {
    u8 = iota,
    COPY, BLEND, ALPHA
}

#[inline]
pub fn copy<S: AsRef<[u32]>, D: AsMut<[u32]>>(
    src: &Image<S>,
    dst: &mut Image<D>,
    param: BlitParam,
) {
    blit::<S, D, COPY>(src, dst, param, 0);
}

#[inline]
pub fn copy_within<D: AsMut<[u32]>>(image: &mut Image<D>, param: BlitParam) {
    blit_within::<D, COPY>(image, param, 0);
}

#[inline]
pub fn copy_scaled<S: AsRef<[u32]>, D: AsMut<[u32]>>(
    src: &Image<S>,
    dst: &mut Image<D>,
    param: BlitParam,
) {
    if !param.is_scaled() {
        return copy(src, dst, param);
    }

    blit_scaled::<S, D, COPY>(src, dst, param, 0);
}

#[inline]
pub fn blend<S: AsRef<[u32]>, D: AsMut<[u32]>>(
    src: &Image<S>,
    dst: &mut Image<D>,
    param: BlitParam,
) {
    blit::<S, D, BLEND>(src, dst, param, 0);
}

#[inline]
pub fn blend_within<D: AsMut<[u32]>>(image: &mut Image<D>, param: BlitParam) {
    blit_within::<D, BLEND>(image, param, 0);
}

#[inline]
pub fn blend_scaled<S: AsRef<[u32]>, D: AsMut<[u32]>>(
    src: &Image<S>,
    dst: &mut Image<D>,
    param: BlitParam,
) {
    if !param.is_scaled() {
        return blend(src, dst, param);
    }

    blit_scaled::<S, D, BLEND>(src, dst, param, 0);
}

#[inline]
pub fn blend_alpha<S: AsRef<[u32]>, D: AsMut<[u32]>>(
    src: &Image<S>,
    dst: &mut Image<D>,
    param: BlitParam,
    alpha: u8,
) {
    if alpha == 0xFF {
        return blend(src, dst, param);
    }

    blit::<S, D, ALPHA>(src, dst, param, alpha);
}

#[inline]
pub fn blend_alpha_within<D: AsMut<[u32]>>(image: &mut Image<D>, param: BlitParam, alpha: u8) {
    if alpha == 0xFF {
        return blend_within(image, param);
    }

    blit_within::<D, ALPHA>(image, param, alpha);
}

#[inline]
pub fn blend_alpha_scaled<S: AsRef<[u32]>, D: AsMut<[u32]>>(
    src: &Image<S>,
    dst: &mut Image<D>,
    param: BlitParam,
    alpha: u8,
) {
    if alpha == 0xFF {
        return blend_scaled(src, dst, param);
    }

    if !param.is_scaled() {
        return blend_alpha(src, dst, param, alpha);
    }

    blit_scaled::<S, D, ALPHA>(src, dst, param, alpha);
}

#[inline]
fn blit<S: AsRef<[u32]>, D: AsMut<[u32]>, const MODE: u8>(
    src: &Image<S>,
    dst: &mut Image<D>,
    param: BlitParam,
    alpha: u8,
) {
    let src_buffer = src.buffer.as_ref();
    let dst_buffer = dst.buffer.as_mut();
    let (sx, dx, w) = clip_axis(param.sx, param.dx, param.sw, src.width, dst.width);
    let (sy, dy, h) = clip_axis(param.sy, param.dy, param.sh, src.height, dst.height);

    if w == 0 || h == 0 {
        return;
    }

    let src_rows = src_buffer
        .chunks_exact(src.width as usize)
        .skip(sy)
        .take(h)
        .map(|row| &row[sx..][..w]);

    let dst_rows = dst_buffer
        .chunks_exact_mut(dst.width as usize)
        .skip(dy)
        .take(h)
        .map(|row| &mut row[dx..][..w]);

    match MODE {
        COPY => {
            for (src_row, dst_row) in src_rows.zip(dst_rows) {
                dst_row.copy_from_slice(src_row);
            }
        }
        BLEND => {
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
        ALPHA => {
            for (src_row, dst_row) in src_rows.zip(dst_rows) {
                let src_row = src_row.iter().copied();
                let dst_row = dst_row.iter_mut();

                for (dst_color, src_color) in dst_row.zip(src_row) {
                    match get_alpha(src_color) {
                        0x00 => continue,
                        _ => {
                            let src_color = mul_alpha(src_color, alpha);
                            *dst_color = blend_color(*dst_color, src_color);
                        }
                    }
                }
            }
        }
        _ => unreachable!(),
    }
}

#[inline]
fn blit_within<D: AsMut<[u32]>, const MODE: u8>(image: &mut Image<D>, param: BlitParam, alpha: u8) {
    let buffer = image.buffer.as_mut();
    let width = image.width as usize;
    let (sx, dx, w) = clip_axis(param.sx, param.dx, param.sw, image.width, image.width);
    let (sy, dy, h) = clip_axis(param.sy, param.dy, param.sh, image.height, image.height);

    if w == 0 || h == 0 {
        return;
    }

    let reverse_y = dy > sy;
    let reverse_x = dy == sy && dx > sx;

    match MODE {
        COPY => {
            let mut copy = |y| {
                let src_start = width * (sy + y) + sx;
                let dst_start = width * (dy + y) + dx;
                buffer.copy_within(src_start..src_start + w, dst_start);
            };

            if reverse_y {
                for y in (0..h).rev() {
                    copy(y);
                }
            } else {
                for y in 0..h {
                    copy(y);
                }
            }
        }
        BLEND => {
            let mut blend = |y| {
                let src_start = width * (sy + y) + sx;
                let dst_start = width * (dy + y) + dx;
                let mut blend_pixel = |x| {
                    let src_color = buffer[src_start + x];
                    match get_alpha(src_color) {
                        0x00 => (),
                        0xFF => buffer[dst_start + x] = src_color,
                        _ => {
                            buffer[dst_start + x] = blend_color(buffer[dst_start + x], src_color);
                        }
                    }
                };

                if reverse_x {
                    for x in (0..w).rev() {
                        blend_pixel(x);
                    }
                } else {
                    for x in 0..w {
                        blend_pixel(x);
                    }
                }
            };

            if reverse_y {
                for y in (0..h).rev() {
                    blend(y);
                }
            } else {
                for y in 0..h {
                    blend(y);
                }
            }
        }
        ALPHA => {
            if alpha == 0 {
                return;
            }

            let mut blend = |y| {
                let src_start = width * (sy + y) + sx;
                let dst_start = width * (dy + y) + dx;
                let mut blend_pixel = |x| {
                    let src_color = buffer[src_start + x];
                    if get_alpha(src_color) != 0 {
                        let src_color = mul_alpha(src_color, alpha);
                        buffer[dst_start + x] = blend_color(buffer[dst_start + x], src_color);
                    }
                };

                if reverse_x {
                    for x in (0..w).rev() {
                        blend_pixel(x);
                    }
                } else {
                    for x in 0..w {
                        blend_pixel(x);
                    }
                }
            };

            if reverse_y {
                for y in (0..h).rev() {
                    blend(y);
                }
            } else {
                for y in 0..h {
                    blend(y);
                }
            }
        }
        _ => unreachable!(),
    }
}

#[inline]
fn blit_scaled<S: AsRef<[u32]>, D: AsMut<[u32]>, const MODE: u8>(
    src: &Image<S>,
    dst: &mut Image<D>,
    param: BlitParam,
    alpha: u8,
) {
    if param.sw <= 0 || param.sh <= 0 || param.dw <= 0 || param.dh <= 0 {
        return;
    }

    let src_buffer = src.buffer.as_ref();
    let dst_buffer = dst.buffer.as_mut();
    let src_stride = src.width as usize;
    let dst_stride = dst.width as usize;

    let dx0 = (param.dx as i64).max(0);
    let dy0 = (param.dy as i64).max(0);
    let dx1 = (param.dx as i64 + param.dw as i64).min(dst.width as i64);
    let dy1 = (param.dy as i64 + param.dh as i64).min(dst.height as i64);

    if dx0 >= dx1 || dy0 >= dy1 {
        return;
    }

    let incx = ((param.sw as u64) << 16) / param.dw as u64;
    let incy = ((param.sh as u64) << 16) / param.dh as u64;
    let mut posy = incy / 2 + (dy0 - param.dy as i64) as u64 * incy;

    match MODE {
        COPY => {
            for dy in dy0..dy1 {
                let srcy = param.sy as i64 + (posy >> 16) as i64;
                if srcy >= 0 && srcy < src.height as i64 {
                    let src_row = srcy as usize * src_stride;
                    let dst_row = dy as usize * dst_stride;
                    let mut posx = incx / 2 + (dx0 - param.dx as i64) as u64 * incx;
                    for dx in dx0..dx1 {
                        let srcx = param.sx as i64 + (posx >> 16) as i64;
                        if srcx >= 0 && srcx < src.width as i64 {
                            let src_idx = src_row + srcx as usize;
                            let dst_idx = dst_row + dx as usize;
                            dst_buffer[dst_idx] = src_buffer[src_idx];
                        }
                        posx += incx;
                    }
                }
                posy += incy;
            }
        }
        BLEND => {
            for dy in dy0..dy1 {
                let srcy = param.sy as i64 + (posy >> 16) as i64;
                if srcy >= 0 && srcy < src.height as i64 {
                    let src_row = srcy as usize * src_stride;
                    let dst_row = dy as usize * dst_stride;
                    let mut posx = incx / 2 + (dx0 - param.dx as i64) as u64 * incx;
                    for dx in dx0..dx1 {
                        let srcx = param.sx as i64 + (posx >> 16) as i64;
                        if srcx >= 0 && srcx < src.width as i64 {
                            let src_color = src_buffer[src_row + srcx as usize];
                            let dst_color = &mut dst_buffer[dst_row + dx as usize];
                            match get_alpha(src_color) {
                                0x00 => (),
                                0xFF => *dst_color = src_color,
                                _ => *dst_color = blend_color(*dst_color, src_color),
                            }
                        }
                        posx += incx;
                    }
                }
                posy += incy;
            }
        }
        ALPHA => {
            for dy in dy0..dy1 {
                let srcy = param.sy as i64 + (posy >> 16) as i64;
                if srcy >= 0 && srcy < src.height as i64 {
                    let src_row = srcy as usize * src_stride;
                    let dst_row = dy as usize * dst_stride;
                    let mut posx = incx / 2 + (dx0 - param.dx as i64) as u64 * incx;
                    for dx in dx0..dx1 {
                        let srcx = param.sx as i64 + (posx >> 16) as i64;
                        if srcx >= 0 && srcx < src.width as i64 {
                            let src_color = src_buffer[src_row + srcx as usize];
                            if get_alpha(src_color) != 0 {
                                let src_color = mul_alpha(src_color, alpha);
                                let dst_color = &mut dst_buffer[dst_row + dx as usize];
                                *dst_color = blend_color(*dst_color, src_color);
                            }
                        }
                        posx += incx;
                    }
                }
                posy += incy;
            }
        }
        _ => unreachable!(),
    }
}

#[rustfmt::skip]
fn clip_axis(
    mut src: i32, mut dst: i32, mut len: i32, src_end: u32, dst_end: u32,
) -> (usize, usize, usize) {
    if src.is_negative() { len += src; src = 0; }
    if dst.is_negative() { len += dst; src -= dst; dst = 0; }

    let len = len.max(0) as usize;
    let src = src.max(0) as usize;
    let dst = dst.max(0) as usize;

    let src_bound = (src_end as usize).saturating_sub(src);
    let dst_bound = (dst_end as usize).saturating_sub(dst);
    let len = len.min(src_bound).min(dst_bound);

    (src, dst, len)
}

#[cfg(test)]
mod tests {
    use crate::graphic::{
        Image,
        blit::{self, BlitParam, clip_axis},
        color::*,
    };

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

        blit::copy(&src, &mut dst, BlitParam::new(0, 0, 2, 2));
        #[rustfmt::skip]
        let expected = [
            RED,  RED,  BLUE, BLUE,
            RED,  RED,  BLUE, BLUE,
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

        blit::blend(&src, &mut dst, BlitParam::new_offset(0, 0, 1, 2, 1, 1));
        #[rustfmt::skip]
        let expected = [
            WHITE, WHITE, WHITE,
            WHITE, BLACK, WHITE,
            WHITE, GRAY,  WHITE,
            WHITE, WHITE, WHITE,
        ];
        assert_eq!(dst.buffer, expected);

        let src = create_test_image(3, 4, TRANSPARENT);
        let mut dst = create_test_image(3, 4, GREEN);

        blit::blend(&src, &mut dst, BlitParam::new_offset(0, 0, 2, 2, 0, 0));
        #[rustfmt::skip]
        let expected = [
            GREEN, GREEN, GREEN,
            GREEN, GREEN, GREEN,
            GREEN, GREEN, GREEN,
            GREEN, GREEN, GREEN,
        ];
        assert_eq!(dst.buffer, expected);
    }

    #[test]
    fn test_blit_blend_alpha() {
        let src = create_test_image(3, 4, BLACK);
        let mut dst = create_test_image(3, 4, WHITE);

        blit::blend_alpha(&src, &mut dst, BlitParam::new(0, 0, 2, 2), 0x7F);
        #[rustfmt::skip]
        let expected = [
            GRAY,  GRAY,  WHITE,
            GRAY,  GRAY,  WHITE,
            WHITE, WHITE, WHITE,
            WHITE, WHITE, WHITE,
        ];
        assert_eq!(dst.buffer, expected);
    }

    #[test]
    fn test_blit_copy_within() {
        #[rustfmt::skip]
        let mut image = Image::from_buffer(4, 4, [
            RED,   GREEN,    BLUE, BLUE,
            BLACK, WHITE,    BLUE, BLUE,
            GRAY,  BLACK_50, BLUE, BLUE,
            BLUE,  BLUE,     BLUE, BLUE,
        ]).unwrap();

        blit::copy_within(&mut image, BlitParam::new_offset(0, 0, 2, 2, 1, 1));
        #[rustfmt::skip]
        let expected = [
            RED,   GREEN, BLUE, BLUE,
            BLACK, RED,   GREEN, BLUE,
            GRAY,  BLACK, WHITE, BLUE,
            BLUE,  BLUE,  BLUE,  BLUE,
        ];
        assert_eq!(image.buffer, expected);

        // Reverse
        #[rustfmt::skip]
        let mut image = Image::from_buffer(4, 4, [
            RED,   GREEN,    BLUE, BLUE,
            BLACK, WHITE,    BLUE, BLUE,
            GRAY,  BLACK_50, BLUE, BLUE,
            BLUE,  BLUE,     BLUE, BLUE,
        ]).unwrap();

        blit::copy_within(&mut image, BlitParam::new_offset(1, 1, 2, 2, 0, 0));
        #[rustfmt::skip]
        let expected = [
            WHITE,    BLUE,     BLUE, BLUE,
            BLACK_50, BLUE,     BLUE, BLUE,
            GRAY,     BLACK_50, BLUE, BLUE,
            BLUE,     BLUE,     BLUE, BLUE,
        ];
        assert_eq!(image.buffer, expected);
    }

    #[test]
    fn test_blit_blend_within() {
        let mut image = create_test_image(4, 4, WHITE);
        image.buffer[0] = BLACK;
        image.buffer[1] = BLACK_50;
        image.buffer[4] = TRANSPARENT;
        image.buffer[5] = BLACK;

        blit::blend_within(&mut image, BlitParam::new_offset(0, 0, 2, 2, 1, 1));
        #[rustfmt::skip]
        let expected = [
            BLACK,       BLACK_50, WHITE, WHITE,
            TRANSPARENT, BLACK,    GRAY,  WHITE,
            WHITE,       WHITE,    BLACK, WHITE,
            WHITE,       WHITE,    WHITE, WHITE,
        ];
        assert_eq!(image.buffer, expected);
    }

    #[test]
    fn test_blit_blend_alpha_within() {
        let mut image = create_test_image(4, 4, WHITE);
        image.buffer[0] = BLACK;
        image.buffer[1] = BLACK;
        image.buffer[4] = BLACK;
        image.buffer[5] = BLACK;

        blit::blend_alpha_within(&mut image, BlitParam::new_offset(0, 0, 2, 2, 1, 1), 0x7F);
        #[rustfmt::skip]
        let expected = [
            BLACK, BLACK, WHITE, WHITE,
            BLACK, BLACK, GRAY,  WHITE,
            WHITE, GRAY,  GRAY,  WHITE,
            WHITE, WHITE, WHITE, WHITE,
        ];
        assert_eq!(image.buffer, expected);
    }

    #[test]
    fn test_clip_axis() {
        let (src, dst, len) = clip_axis(10, 20, 30, 100, 100);
        assert!(src == 10 && dst == 20 && len == 30);

        let (src, dst, len) = clip_axis(-10, 20, 30, 100, 100);
        assert!(src == 0 && dst == 20 && len == 20);

        let (src, dst, len) = clip_axis(10, -5, 20, 100, 100);
        assert!(src == 15 && dst == 0 && len == 15);

        let (src, dst, len) = clip_axis(-10, -5, 30, 100, 100);
        assert!(src == 5 && dst == 0 && len == 15);

        let (src, dst, len) = clip_axis(80, 10, 30, 100, 100);
        assert!(src == 80 && dst == 10 && len == 20);

        let (src, dst, len) = clip_axis(10, 90, 20, 100, 100);
        assert!(src == 10 && dst == 90 && len == 10);

        let (.., len) = clip_axis(-50, 0, 10, 100, 100);
        assert!(len == 0);
    }
}
