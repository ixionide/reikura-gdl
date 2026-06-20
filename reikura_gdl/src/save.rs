use anyhow::{Context, Ok, Result};
use memmap2::MmapMut;
use reikura_util::{bitset::BitSet, variable::Variables};
use std::path::{Path, PathBuf};

const SAVE_DIR: &str = "saves";
const FLAG_SAVE_NAME: &str = "reikura_f.sav";
const VAR_SAVE_NAME: &str = "reikura_v.sav";
const READFLAG_SAVE_NAME: &str = "reikura_r.sav";

#[allow(dead_code)]
fn save_name(index: u8) -> String {
    format!("reikura_{index}.sav")
}

pub struct SaveManager {
    pub save_path: PathBuf,
    pub flags: BitSet<MmapMut>,
    pub variables: Variables<MmapMut>,
    read_flags: Option<BitSet<MmapMut>>,
    pub game_info: Option<String>,
}

impl SaveManager {
    pub fn new(
        game_path: impl AsRef<Path>,
        flag_count: usize,
        variable_count: usize,
    ) -> Result<Self> {
        let save_path = game_path.as_ref().join(SAVE_DIR);
        _ = std::fs::create_dir(&save_path);

        let mut opt = std::fs::OpenOptions::new();
        opt.read(true).write(true).create(true);

        let flag_save_path = save_path.join(FLAG_SAVE_NAME);
        let var_save_path = save_path.join(VAR_SAVE_NAME);

        let flags = unsafe {
            let count = flag_count.div_ceil(u8::BITS as _);
            let flag_save_file = opt
                .open(flag_save_path)
                .context("failed to create flag save file")?;
            flag_save_file.set_len(count as u64)?;
            let mmapmut = MmapMut::map_mut(&flag_save_file)?;
            BitSet::from_raw(mmapmut, flag_count)
        };

        let variables = unsafe {
            let count = variable_count * size_of::<i32>();
            let var_save_file = opt
                .open(var_save_path)
                .context("failed to create variable save file")?;
            var_save_file.set_len(count as u64)?;
            MmapMut::map_mut(&var_save_file)?.into()
        };

        Ok(Self {
            save_path,
            flags,
            variables,
            read_flags: None,
            game_info: None,
        })
    }

    pub fn init_read_flags(&mut self, count: usize) -> Result<()> {
        let read_flag_save_path = self.save_path.join(READFLAG_SAVE_NAME);
        let mut opt = std::fs::OpenOptions::new();
        opt.read(true).write(true).create(true);

        let flags = unsafe {
            let count = count.div_ceil(u8::BITS as _);
            let read_flag_save_file = opt
                .open(read_flag_save_path)
                .context("failed to create readflag save file")?;
            read_flag_save_file.set_len(count as u64)?;
            let mmapmut = MmapMut::map_mut(&read_flag_save_file)?;
            BitSet::from_raw(mmapmut, count)
        };

        self.read_flags = Some(flags);

        Ok(())
    }

    pub fn is_read(&self, index: usize) -> bool {
        self.read_flags
            .as_ref()
            .and_then(|it| it.get(index))
            .unwrap_or_default()
    }

    pub fn mark_read(&mut self, index: usize) {
        if let Some(read_flags) = &mut self.read_flags {
            read_flags.set(index, true);
        }
    }
}
