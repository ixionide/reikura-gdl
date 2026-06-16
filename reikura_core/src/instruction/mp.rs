use std::time::Duration;

use crate::instruction::{Evaluate, Instruction, ReadParam, Value};

pub struct Mp;

impl Instruction for Mp {
    fn execute(vm: &mut crate::Vm, info: super::InstructionInfo) -> anyhow::Result<()> {
        let looping = vm.scene.param::<u8>()? == 0;
        let mut fade = None;

        if info.param_length() == 5 {
            let ms = vm.scene.param::<Value>()?.evaluate(&vm.ctx);
            fade = ms.is_positive().then(|| Duration::from_millis(ms as u64));
        }

        vm.audio.play_bgm(looping, fade)?;

        Ok(())
    }
}
