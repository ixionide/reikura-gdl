use crate::{Vm, instruction::InstructionInfo};

pub fn sret(vm: &mut Vm, _info: InstructionInfo) -> anyhow::Result<()> {
    vm.parser.ret_scene()?;

    Ok(())
}
