use std::{fs::OpenOptions, io::Write, path::Path};

use memmap2::MmapMut;

const DEFAULT: &[u8] = &[
    0,   // fullscreen
    1,   // bgm
    1,   // sfx
    1,   // voice
    255, // bgm volume
    255, // sfx volume
    255, // voice volume
    128, // text speed
    0,   // textbox r
    0,   // textbox g
    0,   // textbox b
    128, // textbox a
];

pub struct Config(MmapMut);

impl Config {
    pub fn open(path: impl AsRef<Path>) -> std::io::Result<Self> {
        let mut opt = OpenOptions::new();
        opt.read(true).write(true).create(true);
        let mut file = opt.open(path)?;

        if file.metadata()?.len() != DEFAULT.len() as u64 {
            file.write_all(DEFAULT)?;
        }

        let mmap = unsafe { MmapMut::map_mut(&file)? };
        Ok(mmap.into())
    }

    pub fn fullscreen(&self) -> bool {
        self.0[0] != 0
    }

    pub fn set_fullscreen(&mut self, fullscreen: bool) {
        self.0[0] = fullscreen as u8;
    }

    pub fn bgm(&self) -> bool {
        self.0[1] != 0
    }

    pub fn set_bgm(&mut self, enabled: bool) {
        self.0[1] = enabled as u8;
    }

    pub fn sfx(&self) -> bool {
        self.0[2] != 0
    }

    pub fn set_sfx(&mut self, enabled: bool) {
        self.0[2] = enabled as u8;
    }

    pub fn voice(&self) -> bool {
        self.0[3] != 0
    }

    pub fn set_voice(&mut self, enabled: bool) {
        self.0[3] = enabled as u8;
    }

    pub fn bgm_volume(&self) -> u8 {
        self.0[4]
    }

    pub fn set_bgm_volume(&mut self, vol: u8) {
        self.0[4] = vol;
    }

    pub fn sfx_volume(&self) -> u8 {
        self.0[5]
    }

    pub fn set_sfx_volume(&mut self, vol: u8) {
        self.0[5] = vol;
    }

    pub fn voice_volume(&self) -> u8 {
        self.0[6]
    }

    pub fn set_voice_volume(&mut self, vol: u8) {
        self.0[6] = vol;
    }

    pub fn text_speed(&self) -> u8 {
        self.0[7]
    }

    pub fn set_text_speed(&mut self, speed: u8) {
        self.0[7] = speed;
    }

    pub fn textbox_rgba(&self) -> (u8, u8, u8, u8) {
        (self.0[8], self.0[9], self.0[10], self.0[11])
    }

    pub fn set_textbox_rgba(&mut self, r: u8, g: u8, b: u8, a: u8) {
        self.0[8] = r;
        self.0[9] = g;
        self.0[10] = b;
        self.0[11] = a;
    }

    pub fn reset_to_default(&mut self) {
        let len = DEFAULT.len();
        self.0[..len].copy_from_slice(DEFAULT);
    }
}

impl From<MmapMut> for Config {
    fn from(value: MmapMut) -> Self {
        Self(value)
    }
}
