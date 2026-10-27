use anyhow::{Context, Ok, Result};
use memmap2::MmapMut;
use reikura_util::{
    flag::Flag,
    register::{MmapReg, Register},
};
use std::{fs::OpenOptions, path::PathBuf};

pub struct SaveContext {
    pub path: PathBuf,

    pub flags: Option<Flag<MmapMut>>,
    pub registers: Option<Register<MmapReg>>,
    pub message_flags: Option<Flag<MmapMut>>,

    pub game_info: Option<String>,
    open_opt: OpenOptions,
}

impl SaveContext {
    pub fn new(path: PathBuf) -> Self {
        let _ = std::fs::create_dir_all(&path);

        Self {
            path,

            flags: None,
            registers: None,
            message_flags: None,

            game_info: None,
            open_opt: {
                let mut opt = std::fs::OpenOptions::new();
                opt.read(true).write(true).create(true);
                opt
            },
        }
    }

    pub fn init_flag(&mut self, flag_count: usize) -> Result<()> {
        if self.flags.is_some() {
            return Ok(());
        }

        let len = flag_count.div_ceil(u8::BITS as _);
        let flag_save_file = self
            .open_opt
            .open(self.path.join("flags"))
            .context("failed to create flag save file")?;

        flag_save_file.set_len(len as u64)?;

        let mmap_mut = unsafe { MmapMut::map_mut(&flag_save_file)? };

        self.flags = Some(Flag::from_raw(mmap_mut, flag_count));

        Ok(())
    }

    pub fn init_register(&mut self, register_count: usize) -> Result<()> {
        if self.registers.is_some() {
            return Ok(());
        }

        let len = register_count * size_of::<i32>();
        let reg_save_file = self
            .open_opt
            .open(self.path.join("registers"))
            .context("failed to create register save file")?;

        reg_save_file.set_len(len as u64)?;

        let mmap_reg = MmapReg::new(unsafe { MmapMut::map_mut(&reg_save_file)? });

        self.registers = Some(Register::from(mmap_reg));

        Ok(())
    }

    pub fn init_message_flag(&mut self, flag_count: usize) -> Result<()> {
        if self.message_flags.is_some() {
            return Ok(());
        }

        let len = flag_count.div_ceil(u8::BITS as _);
        let msg_save_file = self
            .open_opt
            .open(self.path.join("message_flags"))
            .context("failed to create message-flag save file")?;

        msg_save_file.set_len(len as u64)?;

        let mmap_mut = unsafe { MmapMut::map_mut(&msg_save_file)? };

        self.message_flags = Some(Flag::from_raw(mmap_mut, flag_count));

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
