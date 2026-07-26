use std::time::Duration;

use crate::{
    audio::SFX_SLOT,
    instruction::{Evaluate, Instruction, Value},
};

pub struct Ses;

impl Instruction for Ses {
    fn execute(vm: &mut crate::Vm, _info: super::InstructionInfo) -> anyhow::Result<()> {
        let slot = vm.parser.read_param::<Value>()?.evaluate(&vm.ctx) as usize;
        let ms = vm.parser.read_param::<Value>()?.evaluate(&vm.ctx);
        let fade = ms.is_positive().then(|| Duration::from_millis(ms as u64));

        vm.audio.stop_sfx(slot % SFX_SLOT, fade);

        Ok(())
    }
}
