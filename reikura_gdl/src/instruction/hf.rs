use anyhow::bail;

use crate::{Vm, instruction::InstructionInfo};

pub fn hf(vm: &mut Vm, _info: InstructionInfo) -> anyhow::Result<()> {
    let flag_index = vm.parser.read_param::<u16>()? as usize;
    let sub_index = vm.parser.read_param::<u16>()?;

    let Some(flag) = vm.ctx.flags.get(flag_index) else {
        bail!("flag index out of bounds: {flag_index}");
    };

    if flag {
        vm.parser.jump_sub(sub_index)?;
    }

    Ok(())
}
