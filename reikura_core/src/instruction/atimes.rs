use crate::instruction::{Evaluate, Instruction, ReadParam, Value};

pub struct Atimes;

impl Instruction for Atimes {
    fn execute(vm: &mut crate::Vm, _info: super::InstructionInfo) -> anyhow::Result<()> {
        let ms: Value = vm.scene.param()?;

        vm.ctx.wait_time = ms.evaluate(&vm.ctx).into();

        Ok(())
    }
}
