#[derive(Clone, Copy)]
pub struct Rect {
    pub(crate) left: u32,
    pub(crate) top: u32,
    pub(crate) right: u32,
    pub(crate) bottom: u32,
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

    pub fn union(&self, other: &Self) -> Self {
        Self {
            left: self.left.min(other.left),
            right: self.right.max(other.right),
            top: self.top.min(other.top),
            bottom: self.bottom.max(other.bottom),
        }
    }

    pub fn width(&self) -> u32 {
        self.right - self.left
    }

    pub fn height(&self) -> u32 {
        self.bottom - self.top
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
