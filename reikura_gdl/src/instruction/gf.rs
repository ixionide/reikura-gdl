use crate::{Vm, instruction::InstructionInfo};

pub fn gf(vm: &mut Vm, _info: InstructionInfo) -> anyhow::Result<()> {
    vm.gfx.unset_target();

    Ok(())
}
