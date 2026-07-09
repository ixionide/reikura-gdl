#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Rect {
    pub left: u32,
    pub top: u32,
    pub right: u32,
    pub bottom: u32,
}

impl Rect {
    pub fn from_xywh(x: u32, y: u32, w: u32, h: u32) -> Self {
        Self {
            left: x,
            top: y,
            right: x + w,
            bottom: y + h,
        }
    }

    pub fn from_ltrb(l: u32, t: u32, r: u32, b: u32) -> Option<Self> {
        if l > r || t > b {
            return None;
        }

        Some(Self {
            left: l,
            top: t,
            right: r,
            bottom: b,
        })
    }

    pub fn x(&self) -> u32 {
        self.left
    }

    pub fn y(&self) -> u32 {
        self.top
    }

    pub fn width(&self) -> u32 {
        self.right - self.left
    }

    pub fn height(&self) -> u32 {
        self.bottom - self.top
    }

    pub fn union(&self, other: &Self) -> Self {
        Self {
            left: self.left.min(other.left),
            right: self.right.max(other.right),
            top: self.top.min(other.top),
            bottom: self.bottom.max(other.bottom),
        }
    }

    pub fn contains(&self, other: Self) -> bool {
        self.left <= other.left
            && self.top <= other.top
            && self.right >= other.right
            && self.bottom >= other.bottom
    }

    pub fn same_size(&self, other: Self) -> Option<(u32, u32)> {
        let other_size = other.size();

        (self.size() == other_size).then_some(other_size)
    }

    pub fn size(&self) -> (u32, u32) {
        (self.width(), self.height())
    }
}

pub fn surface_rect(window_size: (u32, u32), view_size: (u32, u32)) -> Rect {
    let window_w = window_size.0 as f32;
    let window_h = window_size.1 as f32;
    let view_w = view_size.0 as f32;
    let view_h = view_size.1 as f32;

    let window_scale = window_w / window_h;
    let view_scale = view_w / view_h;

    if window_scale < view_scale {
        let w = window_w as u32;
        let h = (window_w / view_scale) as u32;
        let x = 0;
        let y = (window_size.1 - h) / 2;

        Rect::from_xywh(x, y, w, h)
    } else {
        let w = (window_h * view_scale) as u32;
        let h = window_h as u32;
        let x = (window_size.0 - w) / 2;
        let y = 0;

        Rect::from_xywh(x, y, w, h)
    }
}
