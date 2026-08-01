use crate::instruction::{Evaluate, Instruction, Value};

pub struct Hs;

impl Instruction for Hs {
    fn execute(vm: &mut crate::Vm, _info: super::InstructionInfo) -> anyhow::Result<()> {
        let reg_index = vm.parser.read_param::<u16>()? as usize;
        let reg_value: Value = vm.parser.read_param()?;
        let value = reg_value.evaluate(&vm.ctx);

        vm.ctx.registers.set(reg_index, value);

        Ok(())
    }
}
