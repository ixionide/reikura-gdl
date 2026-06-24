use anyhow::bail;

use crate::instruction::{Instruction, ReadParam};

const CMD_UNSET: u8 = 0;
const CMD_SET: u8 = 1;
const CMD_TOGGLE: u8 = 2;

pub struct Sk;

impl Instruction for Sk {
    fn execute(vm: &mut crate::Vm, _info: super::InstructionInfo) -> anyhow::Result<()> {
        let flag_index = vm.scene.param::<u16>()? as usize;
        let flag_cmd: u8 = vm.scene.param()?;

        match flag_cmd {
            CMD_UNSET => vm.ctx.flags.set(flag_index, false),
            CMD_SET => vm.ctx.flags.set(flag_index, true),
            CMD_TOGGLE => vm.ctx.flags.toggle(flag_index),
            unk => bail!("unrecognized flag value: {unk}"),
        };

        Ok(())
    }
}
