use reikura_util::image::{bgr_color, bgra_color, blend_color};

pub const TRANSPARENT: u32 = bgra_color(0, 0, 0, 0);
pub const RED: u32 = bgr_color(0, 0, 255);
pub const GREEN: u32 = bgr_color(0, 255, 0);
pub const BLUE: u32 = bgr_color(255, 0, 0);
pub const BLACK: u32 = bgr_color(0, 0, 0);
pub const BLACK_50: u32 = bgra_color(0, 0, 0, 127);
pub const WHITE: u32 = bgr_color(255, 255, 255);
pub const GRAY: u32 = blend_color(WHITE, BLACK_50);
pub const NAVY: u32 = blend_color(BLUE, BLACK_50);
pub const TEST: u32 = blend_color(WHITE, bgra_color(255, 0, 0, 64));
