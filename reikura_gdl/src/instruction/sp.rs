use anyhow::bail;
use reikura_util::{collections::LenVec, flag::FlagGroup, io::ReadExt};

use crate::{Vm, instruction::InstructionInfo};

pub fn sp(vm: &mut Vm, info: InstructionInfo) -> anyhow::Result<()> {
    let bytes = vm.parser.read_bytes(info.param_len)?;

    let Some((0xFF, mut params)) = bytes.split_last() else {
        bail!("SP: invalid param terminator");
    };

    let group_index: u8 = params.get_le()?;

    let mut flag_groups: Vec<FlagGroup> = Vec::with_capacity(8);
    while !params.is_empty() {
        let byte_pos: u16 = params.get_le()?;
        let bitmasks: LenVec<u16, u8> = params.get_le()?;
        let flag_group = FlagGroup::new(byte_pos, bitmasks.into_inner());
        flag_groups.push(flag_group);
    }

    if (group_index as usize) < vm.ctx.flags.groups.len() {
        vm.ctx.flags.groups[group_index as usize] = Some(flag_groups);
    }

    Ok(())
}
