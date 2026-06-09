use std::{
    collections::HashMap,
    fs::File,
    io::{Read, Seek, SeekFrom},
    path::Path,
    rc::Rc,
};

use anyhow::{Context, Result, anyhow};

use crate::{CacheManager, Image, Scenario, format::sm2mpx10::Sm2mpx10};

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
    cache: CacheManager,
}

impl AssetManager {
    pub fn new(base_path: impl AsRef<Path>) -> Result<Self> {
        let read_dir = std::fs::read_dir(base_path)?;
        let entries = read_dir.filter_map(std::io::Result::ok);

        let mut archives = HashMap::with_capacity(ARCHIVE_NAMES.len());
        for entry in entries {
            let filename = entry.file_name();

            for &name in ARCHIVE_NAMES.iter() {
                if filename.eq_ignore_ascii_case(name) {
                    let archive = Archive::load(entry.path())?;
                    archives.insert(name, archive);
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
            cache: CacheManager::new(),
        })
    }

    pub fn load_image(&mut self, name: &str) -> Result<Image> {
        if let Some(image) = self.cache.image.get(name) {
            return Ok(image.clone());
        }

        let data = self.image.get_asset(name)?;
        let image = Image::load(name, &data)?;
        self.cache.image.put(name.to_string(), image.clone());

        Ok(image)
    }

    pub fn load_scenario(&mut self, name: &str) -> Result<Scenario> {
        if let Some(scene) = self.cache.scene.get(name) {
            return Ok(scene.into());
        }

        let data = self.scene.get_asset(name)?;
        let scene = Scenario::load(name, data)?;
        self.cache.scene.put(name.to_string(), scene.to_cache());

        Ok(scene)
    }

    pub fn load_sfx(&mut self, name: &str) -> Result<Rc<[u8]>> {
        if let Some(data) = self.cache.sfx.get(name) {
            return Ok(data.clone());
        }

        let data: Rc<[u8]> = self.sfx.get_asset(name)?.into();
        self.cache.sfx.put(name.to_string(), data.clone());

        Ok(data)
    }

    pub fn load_bgm(&mut self, name: &str) -> Result<Rc<[u8]>> {
        if let Some(data) = self.cache.bgm.get(name) {
            return Ok(data.clone());
        }

        let data: Rc<[u8]> = match &mut self.bgm_midi {
            Some(arc) => arc.get_asset(name),
            None => self.bgm.get_asset(name),
        }?
        .into();
        self.cache.bgm.put(name.to_string(), data.clone());

        Ok(data)
    }

    pub fn load_voice(&mut self, name: &str) -> Result<Rc<[u8]>> {
        if let Some(data) = self.cache.voice.get(name) {
            return Ok(data.clone());
        }

        let data: Rc<[u8]> = self.voice.get_asset(name)?.into();
        self.cache.voice.put(name.to_string(), data.clone());

        Ok(data)
    }
}

pub(crate) struct ArchiveEntry {
    pub filename: String,
    pub offset: usize,
    pub length: usize,
}

pub struct Archive {
    file: File,
    index: HashMap<String, ArchiveEntry>,
    // some title has extra archive eg. (se & se1), (ggd & ggd1), and so on
    extra: Vec<Archive>,
}

impl Archive {
    pub fn load(path: impl AsRef<Path>) -> Result<Self> {
        let path = path.as_ref();
        let mut arc = Self::open(path)?;

        let mut extra = vec![];
        let mut i = 1;
        while let Ok(extra_arc) = Self::open(format!("{}{i}", path.to_string_lossy())) {
            extra.push(extra_arc);
            i += 1;
        }

        arc.extra = extra;
        Ok(arc)
    }

    pub fn open(path: impl AsRef<Path>) -> Result<Self> {
        let path = path.as_ref();
        let err_ctx = || format!("failed to load archive: {}", path.display());

        let mut file = File::open(path).with_context(err_ctx)?;
        let archive = Sm2mpx10::parse(&mut file).with_context(err_ctx)?;
        let mut index = HashMap::with_capacity(archive.entries.len());

        for entry in archive.entries {
            let entry: ArchiveEntry = entry.try_into().with_context(err_ctx)?;
            let mut key = entry.filename.to_ascii_lowercase();
            remove_extension(&mut key);
            index.insert(key, entry);
        }

        Ok(Self {
            file,
            index,
            extra: Vec::new(),
        })
    }

    fn asset_exist(&self, name: &str) -> bool {
        self.index.contains_key(name)
    }

    pub fn get_asset(&mut self, name: &str) -> Result<Vec<u8>> {
        let mut name = name.to_ascii_lowercase();
        remove_extension(&mut name);

        let extra_arc = self.extra.iter_mut().find(|arc| arc.asset_exist(&name));

        match extra_arc {
            Some(arc) => arc.get_asset(&name),
            None => {
                let entry = self
                    .index
                    .get(&name)
                    .with_context(|| format!("asset {name} not found"))?;

                let pos = SeekFrom::Start(entry.offset as u64);
                let len = entry.length;

                let mut buffer = vec![0; len];

                self.file.seek(pos)?;
                self.file.read_exact(&mut buffer)?;

                Ok(buffer)
            }
        }
    }
}

fn remove_extension(name: &mut String) {
    if let Some(dot_pos) = name.rfind('.') {
        name.truncate(dot_pos);
    }
}
