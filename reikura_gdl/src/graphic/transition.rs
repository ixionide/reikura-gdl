use crate::graphic::BlitParam;

pub struct Transition {
    pub elapsed: u32,
    duration: u32,
    pub dirty_region: BlitParam,
}

impl Transition {
    pub fn new(duration: u32, dirty_region: BlitParam) -> Self {
        Self {
            elapsed: 0,
            duration,
            dirty_region,
        }
    }

    pub fn alpha(&self) -> u8 {
        ((self.elapsed as u64 * 255) / self.duration as u64) as u8
    }

    pub fn advance(&mut self, delta: u32) {
        self.elapsed += delta;
    }

    pub fn is_finished(&self) -> bool {
        self.elapsed >= self.duration
    }
}
