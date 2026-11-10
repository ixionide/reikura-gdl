use crate::{Vm, instruction::InstructionInfo};

pub fn ed(vm: &mut Vm, _info: InstructionInfo) -> anyhow::Result<()> {
    vm.runtime.state.exit();

    Ok(())
}
