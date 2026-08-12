use anyhow::bail;

use crate::{instruction::InstructionInfo, vm::Timer};

pub fn timerget(vm: &mut crate::Vm, _info: InstructionInfo) -> anyhow::Result<()> {
    let reg_index: u16 = vm.parser.read_param()?;

    match vm.ctx.timer.as_ref().map(Timer::get) {
        Some(reg_value) => vm.ctx.registers.set(reg_index as usize, reg_value),
        None => bail!("timer is not set yet"),
    };

    Ok(())
}
