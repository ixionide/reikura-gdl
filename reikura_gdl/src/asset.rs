use std::{
    collections::HashMap,
    num::NonZeroUsize,
    path::{Path, PathBuf},
};

use anyhow::{Result, anyhow};
use lru::LruCache;

use crate::{Archive, Audio, CacheManager, Image, Scenario};

const DATA: &str = "data";
const GGD: &str = "ggd";
const ISF: &str = "isf";
const SE: &str = "se";
const VOICE: &str = "voice";
const WMSC: &str = "wmsc";
const MIDI: &str = "midi";
const ARCHIVE_NAMES: [&str; 7] = [DATA, GGD, ISF, SE, VOICE, WMSC, MIDI];

pub struct AssetManager {
    data: Archive,
    image: Archive,
    scene: Archive,
    voice: Archive,
    sfx: Archive,
    bgm: Archive,
    bgm_midi: Option<Archive>,
    fakecdda: HashMap<u8, PathBuf>,
    cache: CacheManager,
}

impl AssetManager {
    pub fn new(base_path: impl AsRef<Path>) -> Result<Self> {
        let read_dir = std::fs::read_dir(base_path)?;
        let entries = read_dir.filter_map(std::io::Result::ok);
        let mut fakecdda = HashMap::new();

        let mut archives = HashMap::with_capacity(ARCHIVE_NAMES.len());
        for entry in entries {
            let entry_path = entry.path();
            let entry_file_name = entry.file_name();

            if entry_path.is_dir() {
                continue;
            }

            if let Some(track_num) = cdda_track(&entry_path) {
                fakecdda.insert(track_num, entry_path);
                continue;
            }

            for arc_name in ARCHIVE_NAMES {
                if entry_file_name.eq_ignore_ascii_case(arc_name) {
                    let archive = Archive::load(&entry_path)?;
                    archives.insert(arc_name, archive);
                    break;
                }
            }
        }

        let mut get_archive = |name: &str| {
            archives
                .remove(name)
                .ok_or_else(|| anyhow!("missing {name} archive"))
        };

        Ok(Self {
            data: get_archive(DATA)?,
            image: get_archive(GGD)?,
            scene: get_archive(ISF)?,
            sfx: get_archive(SE)?,
            bgm: get_archive(WMSC)?,
            voice: get_archive(VOICE)?,
            bgm_midi: archives.remove(MIDI),
            fakecdda,
            cache: CacheManager::new(),
        })
    }

    pub fn load_image(&mut self, asset_name: AssetName) -> Result<Image> {
        if let Some(image) = self.cache.image.get(&asset_name) {
            return Ok(image.clone());
        }

        let (name, data) = self.image.get_asset(&asset_name)?;
        let image = Image::load(name, data)?;
        self.cache.image.put(asset_name, image.clone());

        Ok(image)
    }

    pub fn load_wipe_image(&mut self, asset_name: AssetName) -> Result<Image> {
        if let Some(image) = self.cache.wipe_image.get(&asset_name) {
            return Ok(image.clone());
        }

        let (name, data) = self.data.get_asset(&asset_name)?;
        let image = Image::load(name, data)?;
        self.cache.wipe_image.put(asset_name, image.clone());

        Ok(image)
    }

    pub fn load_scene(&mut self, asset_name: AssetName) -> Result<Scenario> {
        if let Some(scene) = self.cache.scene.get(&asset_name) {
            return Ok(scene.clone());
        }

        let (name, data) = self.scene.get_asset(&asset_name)?;
        let scene = Scenario::load(name, data)?;
        self.cache.scene.put(asset_name, scene.clone());

        Ok(scene)
    }

    pub fn load_sfx(&mut self, asset_name: AssetName) -> Result<Audio> {
        if let Some(data) = self.cache.sfx.get(&asset_name) {
            return Ok(data.clone());
        }

        let (name, data) = self.sfx.get_asset(&asset_name)?;
        let audio = Audio::load(name, data)?;
        self.cache.sfx.put(asset_name, audio.clone());

        Ok(audio)
    }

    pub fn load_bgm(&mut self, asset_name: AssetName) -> Result<Audio> {
        if let Some(data) = self.cache.bgm.get(&asset_name) {
            return Ok(data.clone());
        }

        let audio = match &mut self.bgm_midi {
            Some(arc) => {
                let (_name, _data) = arc.get_asset(&asset_name)?;
                todo!(); // Audio::load_midi
            }
            None => {
                let (name, data) = self.bgm.get_asset(&asset_name)?;
                Audio::load(name, data)?
            }
        };

        self.cache.bgm.put(asset_name, audio.clone());

        Ok(audio)
    }

    pub fn load_voice(&mut self, asset_name: AssetName) -> Result<Audio> {
        if let Some(data) = self.cache.voice.get(&asset_name) {
            return Ok(data.clone());
        }

        let (name, data) = self.voice.get_asset(&asset_name)?;
        let audio = Audio::load(name, data)?;
        self.cache.voice.put(asset_name, audio.clone());

        Ok(audio)
    }

    pub fn load_cdda(&mut self, track_num: u8) -> Result<Audio> {
        let cdda_cache = self.cache.cdda.get_or_insert_with(|| {
            let cap = NonZeroUsize::new(self.fakecdda.len().clamp(1, 12)).unwrap();
            LruCache::new(cap)
        });

        if let Some(data) = cdda_cache.get(&track_num) {
            return Ok(data.clone());
        }

        let path = self
            .fakecdda
            .get(&track_num)
            .ok_or_else(|| anyhow!("can't find track_num {track_num} in fakecdda"))?;

        let data = std::fs::read(path)?;
        let name = path
            .file_name()
            .map(|name| name.to_string_lossy().into_owned())
            .unwrap_or_default();

        let audio = Audio::load(name, data)?;
        cdda_cache.put(track_num, audio.clone());

        Ok(audio)
    }
}

#[derive(Debug, Clone, Copy)]
pub struct AssetName {
    buffer: [u8; Self::LEN],
    len: usize,
}

impl AssetName {
    pub const LEN: usize = 12;

    pub const START: Self = Self {
        buffer: *b"START\0\0\0\0\0\0\0",
        len: 5,
    };

    pub fn from_buffer(buffer: [u8; Self::LEN]) -> Self {
        let mut end = Self::LEN;
        let mut ext = None;

        for (i, &b) in buffer.iter().enumerate() {
            if b == 0 || b == 13 {
                end = i;
                break;
            }

            if b == b'.' {
                ext = Some(i);
            }
        }

        Self {
            buffer,
            len: ext.unwrap_or(end),
        }
    }

    #[inline]
    fn buffer(&self) -> &[u8] {
        &self.buffer[..self.len]
    }
}

impl crate::instruction::Parameters for AssetName {
    fn deserialize(scene: &mut crate::Parser) -> Result<Self> {
        let mut buffer = [0; Self::LEN];
        let mut end = Self::LEN;
        let mut ext = None;

        for (i, b) in buffer.iter_mut().enumerate() {
            let byte: u8 = scene.read_param()?;

            if byte == 0 || byte == 13 {
                end = i;
                break;
            }

            if byte == b'.' {
                ext = Some(i);
            }

            *b = byte;
        }

        Ok(Self {
            buffer,
            len: ext.unwrap_or(end),
        })
    }
}

impl Eq for AssetName {}
impl PartialEq for AssetName {
    fn eq(&self, other: &Self) -> bool {
        let lhs = self.buffer().iter().map(u8::to_ascii_lowercase);
        let rhs = other.buffer().iter().map(u8::to_ascii_lowercase);
        lhs.eq(rhs)
    }
}

impl std::fmt::Display for AssetName {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&String::from_utf8_lossy(self.buffer()))
    }
}

impl std::hash::Hash for AssetName {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        for b in self.buffer() {
            state.write_u8(b.to_ascii_lowercase());
        }
    }
}

fn cdda_track(path: &Path) -> Option<u8> {
    use std::ffi::OsStr;

    let file_name = path.file_name()?.to_string_lossy();
    let prefix = file_name.get(..2)?;
    let ext = path.extension()?;

    let is_audio = |ext: &OsStr| {
        ["mp3", "ogg", "wav"]
            .iter()
            .any(|it| ext.eq_ignore_ascii_case(it))
    };

    if !(prefix.eq_ignore_ascii_case("tk") && is_audio(ext)) {
        return None;
    }

    let track_num: u8 = file_name.get(2..)?.parse().ok()?;
    track_num.checked_sub(1)
}
