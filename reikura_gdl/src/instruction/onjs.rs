use reikura_util::collections::LenVec;

use crate::{
    Vm,
    instruction::{InstructionInfo, Value},
};

pub fn onjs(vm: &mut Vm, _info: InstructionInfo) -> anyhow::Result<()> {
    let switch: Value = vm.parser.read_param()?;
    let branches: LenVec<u8, u16> = vm.parser.read_param()?;
    let index = switch.eval(&vm.ctx) as usize;

    if let Some(sub_index) = branches.get(index).copied() {
        vm.parser.call_sub(sub_index)?;
    }

    Ok(())
}
