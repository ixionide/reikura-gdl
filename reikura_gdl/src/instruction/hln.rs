use reikura_util::register::Register;

use crate::{Vm, instruction::InstructionInfo};

pub fn hln(vm: &mut Vm, _info: InstructionInfo) -> anyhow::Result<()> {
    let reg_count: u16 = vm.parser.read_param()?;

    vm.ctx.registers = Register::new(reg_count as usize + 1);

    Ok(())
}
