use crate::instruction::{Evaluate, Instruction, ReadParam, Value};

pub struct Hs;

impl Instruction for Hs {
    fn execute(vm: &mut crate::Vm, _info: super::InstructionInfo) -> anyhow::Result<()> {
        let var_index = vm.scene.param::<u16>()? as usize;
        let var_value: Value = vm.scene.param()?;
        let value = var_value.evaluate(&vm.ctx);

        vm.ctx.variables.set(var_index, value);

        Ok(())
    }
}
