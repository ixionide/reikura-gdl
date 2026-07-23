use std::time::Duration;

use crate::{
    audio::MAX_SFX_SLOT,
    instruction::{Evaluate, Instruction, Value},
};

pub struct Sep;

impl Instruction for Sep {
    fn execute(vm: &mut crate::Vm, info: super::InstructionInfo) -> anyhow::Result<()> {
        let slot = vm.parser.read_param::<Value>()?.evaluate(&vm.ctx) as usize;
        let mut fade = None;

        if info.param_length() == 5 {
            let ms = vm.parser.read_param::<Value>()?.evaluate(&vm.ctx);
            fade = ms.is_positive().then(|| Duration::from_millis(ms as u64));
        }

        vm.audio.play_sfx(slot % MAX_SFX_SLOT, fade)?;

        Ok(())
    }
}
