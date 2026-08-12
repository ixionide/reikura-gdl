use crate::{Vm, instruction::InstructionInfo};

pub fn hln(vm: &mut Vm, _info: InstructionInfo) -> anyhow::Result<()> {
    let reg_count: u16 = vm.parser.read_param()?;

    vm.ctx.registers.resize(reg_count as usize);

    Ok(())
}
