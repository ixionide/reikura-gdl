use crate::{
    audio::MAX_SFX_SLOT,
    instruction::{Evaluate, Instruction, Value},
};

pub struct Sed;

impl Instruction for Sed {
    fn execute(vm: &mut crate::Vm, _info: super::InstructionInfo) -> anyhow::Result<()> {
        let slot = vm.parser.read_param::<Value>()?.evaluate(&vm.ctx) as usize;

        vm.audio.stop_sfx(slot % MAX_SFX_SLOT, None);

        Ok(())
    }
}
