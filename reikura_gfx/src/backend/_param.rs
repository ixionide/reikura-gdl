use crate::Rect;

pub struct BlitParam {
    pub src_id: u8,
    pub dst_id: u8,
    pub src_x: u32,
    pub src_y: u32,
    pub width: u32,
    pub height: u32,
    pub dst_x: u32,
    pub dst_y: u32,
}

impl BlitParam {
    pub fn src_rect(&self) -> Rect {
        Rect::from_xywh(
            self.src_x as u16,
            self.src_y as u16,
            self.width as u16,
            self.height as u16,
        )
    }

    pub fn dst_rect(&self) -> Rect {
        Rect::from_xywh(
            self.src_x as u16,
            self.src_y as u16,
            self.width as u16,
            self.height as u16,
        )
    }
}
