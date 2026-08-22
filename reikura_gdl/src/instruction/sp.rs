use anyhow::bail;
use reikura_util::{index_vec::IndexVec, io::ReadExt};

use crate::{Vm, instruction::InstructionInfo};

pub fn sp(vm: &mut Vm, info: InstructionInfo) -> anyhow::Result<()> {
    let mut params = vm.parser.read_bytes(info.param_len)?;

    let Some(0xFF) = params.last() else {
        bail!("SP: invalid param terminator");
    };

    params = params.split_at(params.len() - 1).0;

    let mut flag_groups: Vec<usize> = Vec::new();

    let group_index: u8 = params.get_le()?;

    while !params.is_empty() {
        let byte_index_start: u16 = params.get_le()?;
        let bits: IndexVec<u16, u8> = params.get_le()?;

        for (bit, byte_index) in bits.iter().copied().zip(byte_index_start..) {
            // XXX: this logic only work when there is only one bit that is set
            assert_eq!(bit.count_ones(), 1);

            let flag_index = byte_index as usize * 8 + bit.trailing_zeros() as usize;
            flag_groups.push(flag_index);
        }
    }

    vm.ctx.flag_groups[group_index as usize] = Some(flag_groups);

    Ok(())
}
