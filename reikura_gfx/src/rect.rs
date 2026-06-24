#[derive(Clone, Copy)]
pub struct Rect {
    pub(crate) left: u16,
    pub(crate) top: u16,
    pub(crate) right: u16,
    pub(crate) bottom: u16,
}

impl Rect {
    pub fn from_xywh(x: u16, y: u16, w: u16, h: u16) -> Self {
        Self {
            left: x,
            top: y,
            right: x + w,
            bottom: y + h,
        }
    }

    pub fn from_ltrb(l: u16, t: u16, r: u16, b: u16) -> Option<Self> {
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

    pub fn width(&self) -> u16 {
        self.right - self.left
    }

    pub fn height(&self) -> u16 {
        self.bottom - self.top
    }

    pub fn same_size(&self, other: Self) -> Option<(u16, u16)> {
        let other_size = other.size();

        (self.size() == other_size).then_some(other_size)
    }

    pub fn size(&self) -> (u16, u16) {
        (self.width(), self.height())
    }
}
