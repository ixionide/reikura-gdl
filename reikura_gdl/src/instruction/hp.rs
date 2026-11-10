use reikura_util::flag::GroupMatch;

use crate::{Vm, instruction::InstructionInfo};

pub fn hp(vm: &mut Vm, _info: InstructionInfo) -> anyhow::Result<()> {
    let group_index: u8 = vm.parser.read_param()?;
    let sub_index: u16 = vm.parser.read_param()?;

    // TODO: log
    let Some(groups) = vm.ctx.flag_groups.get(&group_index) else {
        return Ok(());
    };

    if vm.ctx.flags.match_group(GroupMatch::All, groups) {
        vm.parser.jump_sub(sub_index)?;
    }

    Ok(())
}
