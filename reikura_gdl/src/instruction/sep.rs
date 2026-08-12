use std::time::Duration;

use crate::{
    Vm,
    audio::SFX_SLOT,
    instruction::{InstructionInfo, Value},
};

pub fn sep(vm: &mut Vm, info: InstructionInfo) -> anyhow::Result<()> {
    let slot = vm.parser.read_param::<Value>()?.evaluate(&vm.ctx) as usize;
    let mut fade = None;

    if info.param_len == 5 {
        let ms = vm.parser.read_param::<Value>()?.evaluate(&vm.ctx);
        fade = ms.is_positive().then(|| Duration::from_millis(ms as u64));
    }

    vm.audio.play_sfx(slot % SFX_SLOT, fade)?;

    Ok(())
}
