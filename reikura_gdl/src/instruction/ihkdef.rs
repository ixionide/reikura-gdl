use crate::instruction::{Instruction, Value};

pub struct Ihkdef;

impl Instruction for Ihkdef {
    fn execute(vm: &mut crate::Vm, _info: super::InstructionInfo) -> anyhow::Result<()> {
        let default = vm.parser.read_param::<Value>()?.evaluate(&vm.ctx);

        vm.input.default_key_map = Some(default.try_into()?);

        Ok(())
    }
}
