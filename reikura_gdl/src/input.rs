use reikura_util::{
    image::PIXEL_STRIDE,
    rect::{Rect, surface_rect},
};

use crate::{Image, vm::VmContext};

pub const MAX_HOTSPOTS: usize = 0x40;

pub struct InputManager {
    // inputs: HashMap<Input, InputState>,
    selected: Option<u8>,
    pub default_key_map: Option<u8>,
    pub key_maps: [Option<KeyMap>; MAX_HOTSPOTS],
    pub hot_spots: [Option<HotSpot>; MAX_HOTSPOTS],
    pub hit_mask: Option<HitMask>,

    pub mouse_pos: Option<(i32, i32)>,
    surface_rect: Rect,
    view_size: (u32, u32),
}

impl InputManager {
    pub fn new(view_size: (u32, u32)) -> Self {
        Self {
            selected: None,
            default_key_map: None,
            key_maps: [const { None }; MAX_HOTSPOTS],
            hot_spots: [const { None }; MAX_HOTSPOTS],
            hit_mask: None,
            mouse_pos: None,
            surface_rect: Rect::from_xywh(0, 0, view_size.0, view_size.1),
            view_size,
        }
    }

    pub fn get_selected(&self, check_count: u8) -> i32 {
        if let Some(selected) = self.selected {
            return selected as i32;
        }

        let Some((x, y)) = self.mouse_pos else {
            return -2;
        };

        let hovered_id = self.hit_mask.as_ref().and_then(|hm| hm.get(x, y));

        for hotspot in self.hot_spots.iter().take(check_count as usize) {
            let Some(hot_spot) = hotspot else {
                continue;
            };

            if hot_spot.is_hovered(x, y) || hot_spot.is_id_equal(hovered_id) {
                return hot_spot.id as i32;
            }
        }

        -1
    }

    pub fn _mouse_moved(&mut self, (x, y): (i32, i32)) {
        let x = (x - self.surface_rect.left as i32) as f32 / self.surface_rect.width() as f32;
        let y = (y - self.surface_rect.top as i32) as f32 / self.surface_rect.height() as f32;
        let (view_w, view_h) = (self.view_size.0 as f32, self.view_size.1 as f32);

        self.mouse_pos = Some(((x * view_w) as i32, (y * view_h) as i32));
        self.selected = None;
    }

    pub fn _resized(&mut self, window_size: (u32, u32)) {
        self.surface_rect = surface_rect(window_size, self.view_size);
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
            ctx.registers.get(self.state_index).unwrap_or(0) != 0
        }
    }

    pub fn is_hovered(&self, x: i32, y: i32) -> bool {
        let [left, top, right, bottom] = self.rect;
        x >= left && x <= right && y >= top && y <= bottom
    }

    pub fn is_id_equal(&self, id: Option<u8>) -> bool {
        id.is_some_and(|id| self.id == id)
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

        if self.image.width < x || self.image.height < y {
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
