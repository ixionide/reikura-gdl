use crate::{Vm, instruction::InstructionInfo};

pub fn pcmend(vm: &mut Vm, _info: InstructionInfo) -> anyhow::Result<()> {
    vm.state.wait_voice();

    Ok(())
}
