use crate::{Vm, instruction::InstructionInfo};

pub fn ihgc(vm: &mut Vm, _info: InstructionInfo) -> anyhow::Result<()> {
    vm.input.hit_mask = None;

    Ok(())
}
