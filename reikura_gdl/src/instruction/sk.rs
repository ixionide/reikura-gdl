use anyhow::bail;

use crate::instruction::{Instruction, ReadParam};

pub struct Sk;

impl Instruction for Sk {
    fn execute(vm: &mut crate::Vm, _info: super::InstructionInfo) -> anyhow::Result<()> {
        let flag_index = vm.scene.param::<u16>()? as usize;
        let flag_value: u8 = vm.scene.param()?;

        match flag_value {
            0 => vm.ctx.flags.set(flag_index, false),
            1 => vm.ctx.flags.set(flag_index, true),
            2 => vm.ctx.flags.toggle(flag_index),
            unk => bail!("unrecognized flag value: {unk}"),
        };

        Ok(())
    }
}
