use std::io::Seek;

use anyhow::Result;
use reikura_util::io::{ReadEndian, ReadExt};

use crate::{Parser, Vm, instruction::InstructionInfo};

pub trait Instruction {
    fn execute(vm: &mut Vm, _info: InstructionInfo) -> Result<()> {
        let _ = vm;
        Ok(())
    }

    fn skip(vm: &mut Vm, info: InstructionInfo) -> Result<()> {
        vm.parser.seek_relative(info.param_length() as i64)?;
        Ok(())
    }
}

pub trait Parameters: Sized {
    fn deserialize(parser: &mut Parser) -> anyhow::Result<Self>;
}

impl<T: ReadEndian> Parameters for T {
    fn deserialize(parser: &mut Parser) -> anyhow::Result<Self> {
        let param = parser.read_le::<T>()?;
        Ok(param)
    }
}
