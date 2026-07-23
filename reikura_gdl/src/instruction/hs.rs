use crate::instruction::{Evaluate, Instruction, Value};

pub struct Hs;

impl Instruction for Hs {
    fn execute(vm: &mut crate::Vm, _info: super::InstructionInfo) -> anyhow::Result<()> {
        let var_index = vm.parser.read_param::<u16>()? as usize;
        let var_value: Value = vm.parser.read_param()?;
        let value = var_value.evaluate(&vm.ctx);

        vm.ctx.variables.set(var_index, value);

        Ok(())
    }
}
