use std::{
    io::{ErrorKind, Read, Seek, SeekFrom},
    mem,
};

use anyhow::{Result, bail};

use crate::{Scenario, instruction::Parameters};

const SUB_CALL_STACK: usize = 1024;
const SCENE_CALL_STACK: usize = 256;

pub struct Parser {
    pub state: ExecutionState,
    stack: Vec<ExecutionState>,
}

impl Parser {
    pub fn new(start_scene: Scenario) -> Self {
        Self {
            state: ExecutionState::new(start_scene),
            stack: Vec::with_capacity(SCENE_CALL_STACK),
        }
    }

    pub fn jump_scene(&mut self, scenario: Scenario) {
        self.state = ExecutionState::new(scenario);
    }

    pub fn call_scene(&mut self, scenario: Scenario) -> Result<()> {
        let caller = mem::replace(&mut self.state, ExecutionState::new(scenario));

        if self.stack.len() < SCENE_CALL_STACK {
            self.stack.push(caller);
        } else {
            bail!("parser call stack overflow");
        }

        Ok(())
    }

    pub fn ret_scene(&mut self) -> Result<()> {
        match self.stack.pop() {
            Some(state) => self.state = state,
            None => bail!("parser call stack underflow"),
        }

        Ok(())
    }

    pub fn jump_sub(&mut self, index: u16) -> Result<()> {
        match self.state.scenario.subroutines.get(index as usize).copied() {
            Some(pos) => self.state.ip = pos,
            None => bail!("subroutine index out of bounds: {index}"),
        }

        Ok(())
    }

    pub fn call_sub(&mut self, index: u16) -> Result<()> {
        let caller_ip = self.state.ip;
        self.jump_sub(index)?;

        if self.state.stack.len() < SUB_CALL_STACK {
            self.state.stack.push(caller_ip);
        } else {
            bail!("state call stack overflow")
        }

        Ok(())
    }

    pub fn ret_sub(&mut self) -> Result<()> {
        match self.state.stack.pop() {
            Some(pos) => self.state.ip = pos,
            None => bail!("state call stack underflow"),
        }

        Ok(())
    }

    #[inline]
    pub fn peek_opcode(&mut self) -> Option<u8> {
        self.state.scenario.code.get(self.state.ip).copied()
    }

    pub fn read_opcode(&mut self) -> Result<u8> {
        let Some(op) = self.peek_opcode() else {
            bail!("end of scenario reached");
        };

        self.state.ip += 1;
        Ok(op)
    }

    pub fn read_param<P: Parameters>(&mut self) -> Result<P> {
        Parameters::parse(self)
    }

    pub fn read_bytes(&mut self, length: usize) -> Result<&[u8]> {
        let end = self.state.ip + length;
        let Some(params) = self.state.scenario.code.get(self.state.ip..end) else {
            bail!("end of scenario reached");
        };

        self.state.ip += end;
        Ok(params)
    }
}

pub struct ExecutionState {
    pub ip: usize,
    pub stack: Vec<usize>,
    pub scenario: Scenario,
}

impl ExecutionState {
    pub fn new(scenario: Scenario) -> Self {
        Self {
            ip: 0,
            stack: Vec::with_capacity(SUB_CALL_STACK),
            scenario,
        }
    }

    pub fn cur_offset(&self) -> usize {
        self.scenario.code_offset + self.ip
    }

    pub fn remaining_len(&self) -> usize {
        self.scenario.code.len().saturating_sub(self.ip)
    }
}

impl Read for Parser {
    fn read(&mut self, buf: &mut [u8]) -> std::io::Result<usize> {
        let state = &mut self.state;

        let len = buf.len().min(state.remaining_len());
        let src = &state.scenario.code[state.ip..][..len];

        buf[..len].copy_from_slice(src);
        state.ip += len;

        Ok(len)
    }
}

impl Seek for Parser {
    fn seek(&mut self, pos: SeekFrom) -> std::io::Result<u64> {
        let state = &mut self.state;

        let seek_error = || {
            std::io::Error::new(
                ErrorKind::InvalidInput,
                "invalid seek to a negative or overflowing instruction pointer",
            )
        };

        match pos {
            SeekFrom::Start(ip) => state.ip = ip as usize,
            SeekFrom::End(n) => match state.scenario.code.len().checked_add_signed(n as isize) {
                Some(ip) => state.ip = ip,
                None => return Err(seek_error()),
            },
            SeekFrom::Current(n) => match state.ip.checked_add_signed(n as isize) {
                Some(ip) => state.ip = ip,
                None => return Err(seek_error()),
            },
        }

        Ok(state.ip as u64)
    }
}
