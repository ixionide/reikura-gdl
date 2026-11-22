use reikura_util::image::PIXEL_STRIDE;

use crate::{Image, vm::VmContext};

pub const MAX_HOTSPOTS: usize = 0x40;

pub struct ButtonState {
    button: Button,
    down: bool,
}

pub enum Button {
    Left,
    Right,
}

pub struct InputManager {
    // inputs: HashMap<Input, InputState>,
    focused: Option<u8>,
    // TODO: refactor
    button: Option<ButtonState>,
    pub default_key_map: Option<u8>,
    pub keymaps: [Option<Keymap>; MAX_HOTSPOTS],
    pub hotspots: [Option<Hotspot>; MAX_HOTSPOTS],
    pub hitmask: Option<Hitmask>,
    cursor_pos: Option<(i32, i32)>,
}

impl InputManager {
    pub fn new() -> Self {
        Self {
            focused: None,
            button: None,
            default_key_map: None,
            keymaps: [const { None }; MAX_HOTSPOTS],
            hotspots: [const { None }; MAX_HOTSPOTS],
            hitmask: None,
            cursor_pos: None,
        }
    }

    pub(crate) fn reset_cursor_pos(&mut self) {
        self.cursor_pos = None;
    }

    pub(crate) fn reset_focused(&mut self) {
        self.focused = None;
    }

    pub(crate) fn set_cursor_position(&mut self, x: i32, y: i32) {
        self.cursor_pos = Some((x, y));
    }

    pub(crate) fn set_button_state(&mut self, button: Button, down: bool) {
        self.button = Some(ButtonState { button, down });
    }

    pub fn get_active_hotspot(&self, vm_ctx: &VmContext, check_count: u8) -> i32 {
        const CURSOR_OUTSIDE: i32 = -2;
        const HOTSPOT_NONE: i32 = 0;

        if let Some(focused_id) = self.focused {
            return focused_id as i32;
        }

        let Some((x, y)) = self.cursor_pos else {
            return CURSOR_OUTSIDE;
        };

        let hovered_id = self.hitmask.as_ref().and_then(|hm| hm.get(x, y));

        for hotspot in self.hotspots.iter().take(check_count as usize) {
            let Some(hotspot) = hotspot else {
                continue;
            };

            if hotspot.is_hovered(x, y) || hovered_id == Some(hotspot.id) {
                if hotspot.is_enabled(vm_ctx) {
                    return hotspot.id as i32;
                }
                break;
            }
        }

        HOTSPOT_NONE
    }

    pub fn get_action(&self) -> i32 {
        const ACTION_CANCEL: i32 = -1;
        const ACTION_NONE: i32 = 0;
        const ACTION_ADVANCE: i32 = 2;

        if let Some(state) = &self.button
            && state.down
        {
            // is this correct what about the other? enter key?
            return match state.button {
                Button::Left => ACTION_ADVANCE,
                Button::Right => ACTION_CANCEL,
            };
        };

        ACTION_NONE
    }
}

pub struct Hotspot {
    pub id: u8,
    pub rect: [i32; 4],
    pub state_storage: StateStorage,
    pub state_index: usize,
    pub _unknown: [u8; 3],
}

#[repr(u8)]
pub enum StateStorage {
    Flag,
    Register,
}

impl Hotspot {
    pub fn is_enabled(&self, ctx: &VmContext) -> bool {
        match self.state_storage {
            StateStorage::Flag => ctx.flags.get(self.state_index),
            StateStorage::Register => ctx.registers.get(self.state_index) != 0,
        }
    }

    pub fn is_hovered(&self, x: i32, y: i32) -> bool {
        let [left, top, right, bottom] = self.rect;
        x >= left && x <= right && y >= top && y <= bottom
    }
}

pub struct Keymap {
    pub id: u8,
    pub map: [u8; 8], // TODO: figure out the remaining 4 bytes
}

impl Keymap {
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

pub struct Hitmask {
    pub x: i32,
    pub y: i32,
    pub image: Image,
}

impl Hitmask {
    pub fn get(&self, x: i32, y: i32) -> Option<u8> {
        let x: u32 = x.try_into().ok()?;
        let y: u32 = y.try_into().ok()?;

        if self.image.width < x || self.image.height < y {
            return None;
        }

        let index = (x + y * self.image.width) as usize;

        self.image
            .data
            .as_chunks::<PIXEL_STRIDE>()
            .0
            .get(index)
            .map(|it| it[0])
            .filter(|&id| id < MAX_HOTSPOTS as u8)
    }
}
