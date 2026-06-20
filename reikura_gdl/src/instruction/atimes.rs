use std::time::Duration;

use crate::instruction::{Evaluate, Instruction, ReadParam, Value};

pub struct Atimes;

impl Instruction for Atimes {
    fn execute(vm: &mut crate::Vm, _info: super::InstructionInfo) -> anyhow::Result<()> {
        let ms: u64 = vm.scene.param::<Value>()?.evaluate(&vm.ctx).try_into()?;

        vm.ctx.wait_duration = Some(Duration::from_millis(ms));

        Ok(())
    }
}
