use crate::{Vm, instruction::InstructionInfo};

pub fn hinc(vm: &mut Vm, _info: InstructionInfo) -> anyhow::Result<()> {
    let reg_index: u16 = vm.parser.read_param()?;

    vm.ctx.registers.inc(reg_index as usize);

    Ok(())
}
