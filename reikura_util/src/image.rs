pub const PIXEL_STRIDE: usize = 4;

#[inline]
pub fn copy_previous_pixels(pixels: &mut Vec<u8>, pos: usize, count: usize) {
    let mut pos = pixels.len() - pos * PIXEL_STRIDE;
    for _ in 0..count {
        pixels.extend_from_within(pos..pos + PIXEL_STRIDE);
        pos += PIXEL_STRIDE;
    }
}
