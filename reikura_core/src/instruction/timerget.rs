use anyhow::bail;

use crate::{
    instruction::{Instruction, ReadParam},
    vm::Timer,
};

pub struct Timerget;

impl Instruction for Timerget {
    fn execute(vm: &mut crate::Vm, _info: super::InstructionInfo) -> anyhow::Result<()> {
        let var_index: u16 = vm.scene.param()?;

        let Some(var_value) = vm.ctx.timer.as_ref().map(Timer::get) else {
            bail!("timer is not yet");
        };

        vm.ctx.variables.set(var_index as usize, var_value);

        Ok(())
    }
}
