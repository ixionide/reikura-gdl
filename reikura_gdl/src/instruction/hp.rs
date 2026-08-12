use anyhow::bail;

use crate::{Vm, instruction::InstructionInfo};

pub fn hp(vm: &mut Vm, _info: InstructionInfo) -> anyhow::Result<()> {
    let group_index: u8 = vm.parser.read_param()?;
    let sub_index: u16 = vm.parser.read_param()?;

    let check_flag = |index: &usize| vm.ctx.flags.get(*index).unwrap_or(false);

    let jump = match vm.ctx.flag_groups[group_index as usize] {
        Some(ref indices) => indices.iter().all(check_flag),
        None => bail!("HP: flag_groups of index {} is not set", group_index),
    };

    if jump {
        vm.parser.jump_sub(sub_index)?;
    }

    Ok(())
}
