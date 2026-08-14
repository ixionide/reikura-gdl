use anyhow::{Context, Ok, Result};
use memmap2::MmapMut;
use reikura_util::{
    bitset::BitSet,
    register::{MmapReg, Register},
};
use std::path::{Path, PathBuf};

const SAVE_DIR: &str = "saves";
const FLAG_SAVE_NAME: &str = "flags";
const REG_SAVE_NAME: &str = "registers";
const MSG_SAVE_NAME: &str = "messages";

#[allow(dead_code)]
fn save_name(index: u8) -> String {
    format!("reikura_{index}.sav")
}

pub struct SaveManager {
    pub path: PathBuf,
    pub flags: BitSet<MmapMut>,
    pub registers: Register<MmapReg>,
    message_flags: Option<BitSet<MmapMut>>,
    pub game_info: Option<String>,
}

impl SaveManager {
    pub fn new(
        game_path: impl AsRef<Path>,
        flag_count: usize,
        register_count: usize,
    ) -> Result<Self> {
        let save_path = game_path.as_ref().join(SAVE_DIR);
        _ = std::fs::create_dir(&save_path);

        let mut opt = std::fs::OpenOptions::new();
        opt.read(true).write(true).create(true);

        let flag_save_path = save_path.join(FLAG_SAVE_NAME);
        let reg_save_path = save_path.join(REG_SAVE_NAME);

        let flags = unsafe {
            let len = flag_count.div_ceil(u8::BITS as _);
            let flag_save_file = opt
                .open(flag_save_path)
                .context("failed to create save flag save file")?;
            flag_save_file.set_len(len as u64)?;
            let mmapmut = MmapMut::map_mut(&flag_save_file)?;
            BitSet::from_raw(mmapmut, flag_count)
        };

        let registers = unsafe {
            let count = register_count * size_of::<i32>();
            let reg_save_file = opt
                .open(reg_save_path)
                .context("failed to create register save file")?;
            reg_save_file.set_len(count as u64)?;
            MmapReg::new(MmapMut::map_mut(&reg_save_file)?).into()
        };

        Ok(Self {
            path: save_path,
            flags,
            registers,
            message_flags: None,
            game_info: None,
        })
    }

    pub fn init_message_flags(&mut self, count: usize) -> Result<()> {
        let msg_save_path = self.path.join(MSG_SAVE_NAME);
        let mut opt = std::fs::OpenOptions::new();
        opt.read(true).write(true).create(true);

        let flags = unsafe {
            let len = count.div_ceil(u8::BITS as _);
            let msg_save_file = opt
                .open(msg_save_path)
                .context("failed to create message save file")?;
            msg_save_file.set_len(len as u64)?;
            let mmapmut = MmapMut::map_mut(&msg_save_file)?;
            BitSet::from_raw(mmapmut, count)
        };

        self.message_flags = Some(flags);

        Ok(())
    }

    pub fn is_message_read(&self, index: usize) -> bool {
        self.message_flags
            .as_ref()
            .and_then(|it| it.get(index))
            .unwrap_or_default()
    }

    pub fn set_message_read(&mut self, index: usize) {
        if let Some(read_flags) = &mut self.message_flags {
            read_flags.set(index, true);
        }
    }
}
