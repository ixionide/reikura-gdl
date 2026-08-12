use anyhow::bail;

use crate::{Vm, instruction::InstructionInfo};

reikura_util::const_iota! {
    u8 = iota,
    UNSET,
    SET,
    TOGGLE,
}

pub fn sk(vm: &mut Vm, _info: InstructionInfo) -> anyhow::Result<()> {
    let flag_index = vm.parser.read_param::<u16>()? as usize;
    let flag_cmd: u8 = vm.parser.read_param()?;

    match flag_cmd {
        UNSET => vm.ctx.flags.set(flag_index, false),
        SET => vm.ctx.flags.set(flag_index, true),
        TOGGLE => vm.ctx.flags.toggle(flag_index),
        unk => bail!("unrecognized flag value: {unk}"),
    };

    Ok(())
}
