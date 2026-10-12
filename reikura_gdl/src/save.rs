use anyhow::{Context, Ok, Result};
use memmap2::MmapMut;
use reikura_util::{
    flag::Flag,
    register::{MmapReg, Register},
};
use std::path::{Path, PathBuf};

const SAVE_DIR: &str = "saves";
const FLAG_SAVE_NAME: &str = "flags";
const REG_SAVE_NAME: &str = "registers";
const MSG_SAVE_NAME: &str = "message_flags";

#[allow(dead_code)]
fn save_name(index: u8) -> String {
    format!("reikura_{index:02}.sav")
}

pub struct SaveContext {
    pub path: PathBuf,
    pub flags: Flag<MmapMut>,
    pub registers: Register<MmapReg>,
    message_flags: Option<Flag<MmapMut>>,
    pub game_info: Option<String>,
}

impl SaveContext {
    pub fn new(
        game_path: impl AsRef<Path>,
        flag_count: usize,
        register_count: usize,
    ) -> Result<Self> {
        let save_path = game_path.as_ref().join(SAVE_DIR);
        _ = std::fs::create_dir(&save_path);

        let mut opt = std::fs::OpenOptions::new();
        opt.read(true).write(true).create(true);

        let flags = unsafe {
            let flag_save_path = save_path.join(FLAG_SAVE_NAME);
            let len = flag_count.div_ceil(u8::BITS as _);
            let flag_save_file = opt
                .open(flag_save_path)
                .context("failed to create flag save file")?;
            flag_save_file.set_len(len as u64)?;
            let mmap_mut = MmapMut::map_mut(&flag_save_file)?;
            Flag::from_raw(mmap_mut, flag_count)
        };

        let registers = unsafe {
            let reg_save_path = save_path.join(REG_SAVE_NAME);
            let len = register_count * size_of::<i32>();
            let reg_save_file = opt
                .open(reg_save_path)
                .context("failed to create register save file")?;
            reg_save_file.set_len(len as u64)?;
            let mmap_reg = MmapReg::new(MmapMut::map_mut(&reg_save_file)?);
            Register::from(mmap_reg)
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
        let mut opt = std::fs::OpenOptions::new();
        opt.read(true).write(true).create(true);

        let flags = unsafe {
            let msg_save_path = self.path.join(MSG_SAVE_NAME);
            let len = count.div_ceil(u8::BITS as _);
            let msg_save_file = opt
                .open(msg_save_path)
                .context("failed to create message-flag save file")?;
            msg_save_file.set_len(len as u64)?;
            let mmap_mut = MmapMut::map_mut(&msg_save_file)?;
            Flag::from_raw(mmap_mut, count)
        };

        self.message_flags = Some(flags);

        Ok(())
    }

    pub fn is_message_read(&self, index: usize) -> bool {
        self.message_flags
            .as_ref()
            .map(|it| it.get(index))
            .unwrap_or_default()
    }

    pub fn set_message_read(&mut self, index: usize) {
        if let Some(read_flags) = &mut self.message_flags {
            read_flags.set(index, true);
        }
    }
}
