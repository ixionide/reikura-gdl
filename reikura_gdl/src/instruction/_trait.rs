use std::io::Seek;

use anyhow::Result;
use reikura_util::io::{ReadEndian, ReadExt};

use crate::{Vm, instruction::InstructionInfo, parser::Parser, vm::VmContext};

pub trait Instruction {
    fn execute(vm: &mut Vm, _info: InstructionInfo) -> Result<()> {
        let _ = vm;
        Ok(())
    }

    // we used this for unsupported instruction
    fn skip(vm: &mut Vm, info: InstructionInfo) -> Result<()> {
        vm.parser.seek_relative(info.param_length() as i64)?;
        Ok(())
    }
}

pub trait Evaluate {
    type Evaluated;
    fn evaluate(&self, ctx: &VmContext) -> Self::Evaluated;
}

pub trait Parameters: Sized {
    fn deserialize(scene: &mut Parser) -> anyhow::Result<Self>;
}

impl<T: ReadEndian> Parameters for T {
    fn deserialize(scene: &mut Parser) -> anyhow::Result<Self> {
        let param = scene.read_le::<T>()?;
        Ok(param)
    }
}
