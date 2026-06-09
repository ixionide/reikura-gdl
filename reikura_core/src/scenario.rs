use std::{
    io::{ErrorKind, Read, Seek, SeekFrom},
    mem,
    rc::Rc,
};

use anyhow::{Result, anyhow, bail};
use reikura_util::io::ReadExt;

use crate::format::isf::IsfMetadata;

const MAX_SCENARIO_STACK: usize = 256;
const MAX_CALL_STACK: usize = 1024;

pub struct Scenario {
    pub ip: usize,
    pub name: Rc<String>,
    bytecode_start_pos: usize,
    bytecode: Rc<[u8]>,
    jump_table: Rc<[usize]>,
    call_stack: Vec<usize>,
    scene_stack: Vec<Scenario>,
}

impl Scenario {
    pub fn load(name: &str, mut data: Vec<u8>) -> Result<Self> {
        let isf = IsfMetadata::parse(&data)?;

        let bytecode_start = isf.bytecode_offset as usize;
        let table_start = 8; // start offset of jump table. from here on data is encrypted

        // decrypting
        let encrypted = data.iter_mut().skip(table_start);
        match isf.version {
            35278 => encrypted.for_each(|byte| *byte ^= isf.xor_key),
            38295 => encrypted.for_each(|byte| *byte = byte.rotate_right(2)),
            38865 => encrypted.for_each(|byte| *byte = !*byte),
            // NOTE: debug version unencrypted
            36343 => (), // log::warn!("scenario {} is a debug version", name)
            ver => bail!("unsupported scenario version: {ver}"),
        }

        let table_count = (bytecode_start - table_start) / size_of::<u32>();
        let mut jump_table = Vec::with_capacity(table_count);

        for mut chunk in data[table_start..bytecode_start].chunks_exact(size_of::<u32>()) {
            let offset: u32 = chunk.read_le()?;
            jump_table.push(offset as usize);
        }

        Ok(Self {
            name: Rc::from(name.to_owned()),
            ip: 0,
            bytecode_start_pos: bytecode_start,
            bytecode: Rc::from(data.split_off(bytecode_start)),
            jump_table: Rc::from(jump_table),
            call_stack: Vec::with_capacity(MAX_CALL_STACK),
            scene_stack: Vec::with_capacity(MAX_SCENARIO_STACK),
        })
    }
    pub fn jump(&mut self, scenario: Scenario) {
        let jumper = mem::replace(self, scenario);
        self.scene_stack = jumper.scene_stack;
    }

    pub fn call(&mut self, scenario: Scenario) -> Result<()> {
        let mut caller = mem::replace(self, scenario);
        self.scene_stack = mem::take(&mut caller.scene_stack);

        if self.scene_stack.len() < MAX_SCENARIO_STACK {
            self.scene_stack.push(caller);
        } else {
            bail!("scenario stack overflow");
        }

        Ok(())
    }

    pub fn ret(&mut self) -> Result<()> {
        match self.scene_stack.pop() {
            Some(scenario) => {
                let returner = mem::replace(self, scenario);
                self.scene_stack = returner.scene_stack;
            }
            None => bail!("scenario stack underflow"),
        }

        Ok(())
    }

    pub fn jump_sub(&mut self, index: u16) -> Result<()> {
        let err = |total| anyhow!("jump table index out of bounds: {index} >= {total}");

        match self.jump_table.get(index as usize).copied() {
            Some(pos) => self.ip = pos,
            None => return Err(err(self.sub_count())),
        }

        Ok(())
    }

    pub fn call_sub(&mut self, index: u16) -> Result<()> {
        let caller_ip = self.ip;
        self.jump_sub(index)?;

        if self.call_stack.len() < MAX_CALL_STACK {
            self.call_stack.push(caller_ip);
        } else {
            bail!("call stack overflow")
        }

        Ok(())
    }

    pub fn ret_sub(&mut self) -> Result<()> {
        match self.call_stack.pop() {
            Some(pos) => self.ip = pos,
            None => bail!("call stack underflow"),
        }

        Ok(())
    }

    pub fn sub_offset(&self, index: usize) -> Option<usize> {
        self.jump_table.get(index).copied()
    }

    pub fn sub_count(&self) -> usize {
        self.jump_table.len()
    }

    pub fn cur_offset(&self) -> usize {
        self.bytecode_start_pos + self.ip
    }

    fn remaining_len(&self) -> usize {
        self.bytecode.len().saturating_sub(self.ip)
    }

    pub fn read_opcode(&mut self) -> Result<u8> {
        let Some(op) = self.bytecode.get(self.ip).copied() else {
            bail!("end of scenario reached");
        };

        self.ip += 1;
        Ok(op)
    }

    pub fn to_cache(&self) -> ScenarioCache {
        ScenarioCache {
            name: self.name.clone(),
            data: self.bytecode.clone(),
            pos: self.bytecode_start_pos,
            table: self.jump_table.clone(),
        }
    }
}

impl Read for Scenario {
    fn read(&mut self, buf: &mut [u8]) -> std::io::Result<usize> {
        let len = buf.len().min(self.remaining_len());
        let src = &self.bytecode[self.ip..][..len];

        buf[..len].copy_from_slice(src);
        self.ip += len;

        Ok(len)
    }
}

impl Seek for Scenario {
    fn seek(&mut self, pos: SeekFrom) -> std::io::Result<u64> {
        let seek_error = || {
            std::io::Error::new(
                ErrorKind::InvalidInput,
                "invalid seek to a negative or overflowing instruction pointer",
            )
        };

        match pos {
            SeekFrom::Start(ip) => self.ip = ip as usize,
            SeekFrom::End(n) => match self.bytecode.len().checked_add_signed(n as isize) {
                Some(ip) => self.ip = ip,
                None => return Err(seek_error()),
            },
            SeekFrom::Current(n) => match self.ip.checked_add_signed(n as isize) {
                Some(ip) => self.ip = ip,
                None => return Err(seek_error()),
            },
        }

        Ok(self.ip as u64)
    }
}

impl From<&ScenarioCache> for Scenario {
    fn from(cache: &ScenarioCache) -> Self {
        Self {
            ip: 0,
            name: cache.name.clone(),
            bytecode_start_pos: cache.pos,
            bytecode: cache.data.clone(),
            jump_table: cache.table.clone(),
            call_stack: Vec::new(),
            scene_stack: Vec::new(),
        }
    }
}

pub struct ScenarioCache {
    pub name: Rc<String>,
    pub data: Rc<[u8]>,
    pub pos: usize,
    pub table: Rc<[usize]>,
}
