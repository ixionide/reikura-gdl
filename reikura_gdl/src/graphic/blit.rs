use reikura_util::image::{blend_color, get_alpha, mul_alpha};

use super::Image;

#[rustfmt::skip]
pub struct BlitParam {
    pub(self) sx: i32, pub(self) sy: i32, pub(self) sw: i32, pub(self) sh: i32, pub(self) dx: i32, pub(self) dy: i32, pub(self) dw: i32, pub(self) dh: i32,
}

#[rustfmt::skip]
#[allow(clippy::too_many_arguments)]
impl BlitParam {
    pub fn new(sx: i32, sy: i32, w: i32, h: i32, dx: i32, dy: i32) -> Self {
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
pub fn blend<S: AsRef<[u32]>, D: AsMut<[u32]>>(
    src: &Image<S>,
    dst: &mut Image<D>,
    param: BlitParam,
) {
    blit::<S, D, BLEND>(src, dst, param, 0);
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
        .skip(sy as usize)
        .take(h as usize)
        .map(|row| &row[sx as usize..][..w as usize]);

    let dst_rows = dst_buffer
        .chunks_exact_mut(dst.width as usize)
        .skip(dy as usize)
        .take(h as usize)
        .map(|row| &mut row[dx as usize..][..w as usize]);

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

#[rustfmt::skip]
fn clip_axis(
    mut src: i32, mut dst: i32, mut len: i32, src_end: u32, dst_end: u32,
) -> (u32, u32, u32) {
    if src.is_negative() { len += src; src = 0; }
    if dst.is_negative() { len += dst; src -= dst; dst = 0; }

    let len = len.max(0) as u32;
    let src = src.max(0) as u32;
    let dst = dst.max(0) as u32;

    let src_bound = src_end.saturating_sub(src);
    let dst_bound = dst_end.saturating_sub(dst);
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

        blit::copy(&src, &mut dst, BlitParam::new(0, 0, 2, 2, 0, 0));
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

        blit::blend(&src, &mut dst, BlitParam::new(0, 0, 1, 2, 1, 1));
        #[rustfmt::skip]
        let expected = [
            WHITE, WHITE, WHITE,
            WHITE, BLACK, WHITE,
            WHITE, GRAY, WHITE,
            WHITE, WHITE, WHITE,
        ];
        assert_eq!(dst.buffer, expected);

        let src = create_test_image(3, 4, TRANSPARENT);
        let mut dst = create_test_image(3, 4, GREEN);

        blit::blend(&src, &mut dst, BlitParam::new(0, 0, 2, 2, 0, 0));
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

        blit::blend_alpha(&src, &mut dst, BlitParam::new(0, 0, 2, 2, 0, 0), 0x7F);
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
