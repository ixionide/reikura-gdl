use crate::{Vm, instruction::InstructionInfo};

pub fn r#await(vm: &mut Vm, _info: InstructionInfo) -> anyhow::Result<()> {
    vm.runtime.state.wait();

    Ok(())
}
