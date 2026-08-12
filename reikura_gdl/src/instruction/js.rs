use crate::{Vm, instruction::InstructionInfo};

pub fn js(vm: &mut Vm, _info: InstructionInfo) -> anyhow::Result<()> {
    let sub_index: u16 = vm.parser.read_param()?;

    vm.parser.call_sub(sub_index)?;

    Ok(())
}
