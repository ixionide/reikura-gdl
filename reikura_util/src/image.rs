pub const PIXEL_STRIDE: usize = 4;

#[inline]
pub const fn mul_div_255(lhs: u8, rhs: u8) -> u8 {
    let mut x = lhs as u16 * rhs as u16;
    x += 1;
    x += x >> 8;
    (x >> 8) as u8
}

#[inline]
pub const fn rgb_color(r: u8, g: u8, b: u8) -> u32 {
    rgba_color(r, g, b, 0xFF)
}

#[inline]
pub const fn rgba_color(r: u8, g: u8, b: u8, a: u8) -> u32 {
    u32::from_le_bytes([r, g, b, a])
}

#[inline]
pub const fn unpack_color(color: u32) -> [u8; 4] {
    color.to_le_bytes()
}

#[inline]
pub const fn mul_alpha_color(color: u32, alpha: u8) -> u32 {
    let [r, g, b, a] = unpack_color(color);
    rgba_color(r, g, b, mul_div_255(a, alpha))
}

#[inline]
pub const fn premul_color(color: u32) -> u32 {
    let [r, g, b, a] = unpack_color(color);

    if a == 0xFF {
        return color;
    }

    rgba_color(mul_div_255(a, r), mul_div_255(a, g), mul_div_255(a, b), a)
}

#[inline]
pub const fn blend_color(bg: u32, fg: u32) -> u32 {
    let [fg_r, fg_g, fg_b, fg_a] = unpack_color(premul_color(fg));
    let [bg_r, bg_g, bg_b, bg_a] = unpack_color(bg);

    rgba_color(
        mul_div_255(255 - fg_a, bg_r) + fg_r,
        mul_div_255(255 - fg_a, bg_g) + fg_g,
        mul_div_255(255 - fg_a, bg_b) + fg_b,
        mul_div_255(255 - fg_a, bg_a) + fg_a,
    )
}

#[inline]
pub const fn blend_premultiplied_color(bg: u32, fg: u32) -> u32 {
    let [bg_r, bg_g, bg_b, bg_a] = unpack_color(bg);
    let [fg_r, fg_g, fg_b, fg_a] = unpack_color(fg);

    rgba_color(
        mul_div_255(255 - fg_a, bg_r) + fg_r,
        mul_div_255(255 - fg_a, bg_g) + fg_g,
        mul_div_255(255 - fg_a, bg_b) + fg_b,
        mul_div_255(255 - fg_a, bg_a) + fg_a,
    )
}

#[inline]
pub fn copy_previous_pixels(pixels: &mut Vec<u8>, pos: usize, count: usize) {
    let mut pos = pixels.len() - pos * PIXEL_STRIDE;
    for _ in 0..count {
        pixels.extend_from_within(pos..pos + PIXEL_STRIDE);
        pos += PIXEL_STRIDE;
    }
}
