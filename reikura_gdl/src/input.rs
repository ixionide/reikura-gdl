use reikura_util::image::PIXEL_STRIDE;

use crate::{Image, vm::VmContext};

pub const MAX_HOTSPOTS: usize = 0x40;

pub struct InputManager {
    #[allow(dead_code)]
    selected: Option<u8>,
    pub default_key_map: Option<u8>,
    pub key_maps: [Option<KeyMap>; MAX_HOTSPOTS],
    pub hot_spots: [Option<HotSpot>; MAX_HOTSPOTS],
    pub hit_mask: Option<HitMask>,
    // inputs: HashMap<Input, InputState>,
}

impl InputManager {
    pub fn new() -> Self {
        Self {
            selected: None,
            default_key_map: None,
            key_maps: [const { None }; MAX_HOTSPOTS],
            hot_spots: [const { None }; MAX_HOTSPOTS],
            hit_mask: None,
        }
    }
}

pub struct HotSpot {
    pub id: u8,
    pub rect: [i32; 4],
    pub flag: bool,
    pub state_index: usize,
    pub _unknown: [u8; 3],
}

impl HotSpot {
    pub fn is_enabled(&self, ctx: &VmContext) -> bool {
        if self.flag {
            ctx.flags.get(self.state_index).unwrap_or(false)
        } else {
            ctx.variables.get(self.state_index).unwrap_or(0) != 0
        }
    }

    pub fn is_hovered(&self, x: i32, y: i32) -> bool {
        let [left, top, right, bottom] = self.rect;
        x >= left && x <= right && y >= top && y <= bottom
    }
}

pub struct KeyMap {
    pub id: u8,
    pub map: [u8; 8], // TODO: figure out the remaining 4 bytes
}

impl KeyMap {
    pub fn up(&self) -> u8 {
        self.map[0]
    }

    pub fn down(&self) -> u8 {
        self.map[1]
    }

    pub fn left(&self) -> u8 {
        self.map[2]
    }

    pub fn right(&self) -> u8 {
        self.map[3]
    }
}

pub struct HitMask {
    pub x: i32,
    pub y: i32,
    pub image: Image,
}

impl HitMask {
    pub fn get(&self, x: i32, y: i32) -> Option<u8> {
        let x: u32 = x.try_into().ok()?;
        let y: u32 = y.try_into().ok()?;

        if self.image.width < x || y > self.image.height {
            return None;
        }

        let index = (x + y * self.image.width) as usize * PIXEL_STRIDE;

        self.image
            .data
            .get(index)
            .copied()
            .filter(|&id| id < MAX_HOTSPOTS as u8)
    }
}
