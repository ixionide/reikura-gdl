use anyhow::bail;

use crate::instruction::{Instruction, ReadParam};

reikura_util::const_iota! {
    u8 = iota,
    UNSET,
    SET,
    TOGGLE,
}

pub struct Sk;

impl Instruction for Sk {
    fn execute(vm: &mut crate::Vm, _info: super::InstructionInfo) -> anyhow::Result<()> {
        let flag_index = vm.scene.param::<u16>()? as usize;
        let flag_cmd: u8 = vm.scene.param()?;

        match flag_cmd {
            UNSET => vm.ctx.flags.set(flag_index, false),
            SET => vm.ctx.flags.set(flag_index, true),
            TOGGLE => vm.ctx.flags.toggle(flag_index),
            unk => bail!("unrecognized flag value: {unk}"),
        };

        Ok(())
    }
}
