use anyhow::bail;
use reikura_util::{const_iota, flag::GroupMatch};

use crate::{Vm, instruction::InstructionInfo};

const_iota! {
    u8 = iota,
    MATCH_ALL,
    MATCH_NONE,
    MATCH_NOTALL,
    MATCH_ANY,
}

pub fn hxp(vm: &mut Vm, _info: InstructionInfo) -> anyhow::Result<()> {
    let group_index: u8 = vm.parser.read_param()?;
    let group_match: u8 = vm.parser.read_param()?;
    let sub_index: u16 = vm.parser.read_param()?;

    let group_match = match group_match {
        MATCH_ALL => GroupMatch::All,
        MATCH_NONE => GroupMatch::None,
        MATCH_NOTALL => GroupMatch::NotAll,
        MATCH_ANY => GroupMatch::Any,
        _ => bail!("invalid group match"),
    };

    let should_jump = vm.ctx.flags.check_group(group_index as usize, group_match);

    if should_jump {
        vm.parser.jump_sub(sub_index)?;
    }

    Ok(())
}
