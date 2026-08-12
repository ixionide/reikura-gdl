use crate::{Vm, instruction::InstructionInfo};

pub fn rt(vm: &mut Vm, _info: InstructionInfo) -> anyhow::Result<()> {
    vm.parser.ret_sub()?;

    Ok(())
}
