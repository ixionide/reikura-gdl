use reikura_util::flag::GroupMatch;

use crate::{Vm, instruction::InstructionInfo};

pub fn hp(vm: &mut Vm, _info: InstructionInfo) -> anyhow::Result<()> {
    let group_index: u8 = vm.parser.read_param()?;
    let sub_index: u16 = vm.parser.read_param()?;

    let should_jump = vm
        .ctx
        .flags
        .check_group(group_index as usize, GroupMatch::All);

    if should_jump {
        vm.parser.jump_sub(sub_index)?;
    }

    Ok(())
}
