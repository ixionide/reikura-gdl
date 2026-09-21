use std::{
    io::{ErrorKind, Read, Seek, SeekFrom},
    mem,
    ops::{Deref, DerefMut},
};

use anyhow::{Context, Result, bail};
use reikura_util::stack::Stack;

use crate::{Scenario, instruction::Parameters};

const MAX_SUB_CALL: usize = 1024;
const MAX_SCN_CALL: usize = 256;

pub struct Parser {
    pub state: ScenarioParser,
    stack: Stack<ScenarioParser, MAX_SCN_CALL>,
}

impl Parser {
    pub fn new(start_scene: Scenario) -> Self {
        Self {
            state: ScenarioParser::new(start_scene),
            stack: Stack::new(),
        }
    }

    pub fn jump_scene(&mut self, scenario: Scenario) {
        self.state = ScenarioParser::new(scenario);
    }

    pub fn call_scene(&mut self, scenario: Scenario) -> Result<()> {
        let caller = mem::replace(&mut self.state, ScenarioParser::new(scenario));
        self.stack.push(caller).context("scene call")
    }

    pub fn ret_scene(&mut self) -> Result<()> {
        self.state = self.stack.pop().context("scene return")?;
        Ok(())
    }
}

impl Deref for Parser {
    type Target = ScenarioParser;

    fn deref(&self) -> &Self::Target {
        &self.state
    }
}

impl DerefMut for Parser {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.state
    }
}

pub struct ScenarioParser {
    pub ip: usize,
    pub stack: Stack<usize, MAX_SUB_CALL>,
    pub scenario: Scenario,
}

impl ScenarioParser {
    pub fn new(scenario: Scenario) -> Self {
        Self {
            ip: 0,
            stack: Stack::new(),
            scenario,
        }
    }

    pub fn cur_offset(&self) -> usize {
        self.scenario.code_offset + self.ip
    }

    pub fn remaining_len(&self) -> usize {
        self.scenario.code.len().saturating_sub(self.ip)
    }

    pub fn jump_sub(&mut self, index: u16) -> Result<()> {
        match self.scenario.subroutines.get(index as usize).copied() {
            Some(pos) => self.ip = pos,
            None => bail!("subroutine index out of bounds: {index}"),
        }

        Ok(())
    }

    pub fn call_sub(&mut self, index: u16) -> Result<()> {
        let caller_ip = self.ip;
        self.jump_sub(index)?;
        self.stack.push(caller_ip).context("subroutine call")
    }

    pub fn ret_sub(&mut self) -> Result<()> {
        self.ip = self.stack.pop().context("subroutine return")?;
        Ok(())
    }

    #[inline]
    pub fn peek_opcode(&mut self) -> Option<u8> {
        self.scenario.code.get(self.ip).copied()
    }

    pub fn read_opcode(&mut self) -> Result<u8> {
        let Some(op) = self.peek_opcode() else {
            bail!("end of scenario reached");
        };

        self.ip += 1;
        Ok(op)
    }

    pub fn read_param<P: Parameters>(&mut self) -> Result<P> {
        Parameters::parse(self)
    }

    pub fn read_bytes(&mut self, length: usize) -> Result<&[u8]> {
        let end = self.ip + length;
        let Some(params) = self.scenario.code.get(self.ip..end) else {
            bail!("end of scenario reached");
        };

        self.ip = end;
        Ok(params)
    }
}

impl Read for ScenarioParser {
    fn read(&mut self, buf: &mut [u8]) -> std::io::Result<usize> {
        let len = buf.len().min(self.remaining_len());
        let src = &self.scenario.code[self.ip..][..len];

        buf[..len].copy_from_slice(src);
        self.ip += len;

        Ok(len)
    }
}

impl Seek for ScenarioParser {
    fn seek(&mut self, pos: SeekFrom) -> std::io::Result<u64> {
        let seek_error = || {
            std::io::Error::new(
                ErrorKind::InvalidInput,
                "invalid seek to a negative or overflowing instruction pointer",
            )
        };

        match pos {
            SeekFrom::Start(ip) => self.ip = ip as usize,
            SeekFrom::End(n) => match self.scenario.code.len().checked_add_signed(n as isize) {
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
