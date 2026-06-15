use crate::instruction::{Evaluate, Instruction, ReadParam, Value};

pub struct Sed;

impl Instruction for Sed {
    fn execute(vm: &mut crate::Vm, _info: super::InstructionInfo) -> anyhow::Result<()> {
        let slot = vm.scene.param::<Value>()?.evaluate(&vm.ctx) as usize;

        vm.audio.stop_sfx(slot, None);

        Ok(())
    }
}
