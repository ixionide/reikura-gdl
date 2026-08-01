use anyhow::bail;

use crate::{instruction::Instruction, vm::Timer};

pub struct Timerget;

impl Instruction for Timerget {
    fn execute(vm: &mut crate::Vm, _info: super::InstructionInfo) -> anyhow::Result<()> {
        let reg_index: u16 = vm.parser.read_param()?;

        match vm.ctx.timer.as_ref().map(Timer::get) {
            Some(reg_value) => vm.ctx.registers.set(reg_index as usize, reg_value),
            None => bail!("timer is not set yet"),
        };

        Ok(())
    }
}
