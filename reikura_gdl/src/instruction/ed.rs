use crate::{Vm, instruction::InstructionInfo};

pub fn ed(vm: &mut Vm, _info: InstructionInfo) -> anyhow::Result<()> {
    vm.state.exit();

    Ok(())
}
