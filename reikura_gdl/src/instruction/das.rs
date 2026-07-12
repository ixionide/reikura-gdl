use std::time::Duration;

use crate::instruction::{Evaluate, Instruction, ReadParam, Value};

pub struct Das;

impl Instruction for Das {
    fn execute(vm: &mut crate::Vm, _info: super::InstructionInfo) -> anyhow::Result<()> {
        let ms = vm.scene.param::<Value>()?.evaluate(&vm.ctx);
        let fade = ms.is_positive().then(|| Duration::from_millis(ms as u64));

        vm.audio.stop_bgm(fade);

        Ok(())
    }
}
