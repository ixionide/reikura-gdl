use std::io::Read;

use anyhow::bail;
use reikura_util::{index_vec::IndexVec, io::ReadExt};

use crate::instruction::Instruction;

pub struct Sp;

impl Instruction for Sp {
    fn execute(vm: &mut crate::Vm, info: super::InstructionInfo) -> anyhow::Result<()> {
        let mut buf_params = vec![0; info.param_length()];
        vm.parser.read_exact(&mut buf_params)?;

        match buf_params.pop() {
            Some(end) => assert_eq!(end, 0xFF),
            None => bail!("SP: invalid param length"),
        }

        let mut params = buf_params.as_slice();
        let mut flag_groups: Vec<usize> = Vec::new();

        let group_index: u8 = params.read_le()?;

        while !params.is_empty() {
            let byte_index_start: u16 = params.read_le()?;
            let bits: IndexVec<u16, u8> = params.read_le()?;

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
}
