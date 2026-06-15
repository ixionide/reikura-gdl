use std::time::Duration;

use crate::instruction::{Evaluate, Instruction, ReadParam, Value};

pub struct Sep;

impl Instruction for Sep {
    fn execute(vm: &mut crate::Vm, info: super::InstructionInfo) -> anyhow::Result<()> {
        let slot = vm.scene.param::<Value>()?.evaluate(&vm.ctx) as usize;
        let mut fade = None;

        if info.param_length() == 5 {
            let ms = vm.scene.param::<Value>()?.evaluate(&vm.ctx);
            fade = ms.try_into().ok().map(Duration::from_millis);
        }

        vm.audio.play_sfx(slot, fade)?;

        Ok(())
    }
}
