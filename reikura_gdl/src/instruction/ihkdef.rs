use crate::instruction::{Evaluate, Instruction, ReadParam, Value};

pub struct Ihkdef;

impl Instruction for Ihkdef {
    fn execute(vm: &mut crate::Vm, _info: super::InstructionInfo) -> anyhow::Result<()> {
        let default = vm.scene.param::<Value>()?.evaluate(&vm.ctx);

        vm.input.default_key_map = Some(default.try_into()?);

        Ok(())
    }
}
