use std::time::Duration;

use crate::{
    Vm,
    instruction::{InstructionInfo, Value},
};

pub fn sep(vm: &mut Vm, info: InstructionInfo) -> anyhow::Result<()> {
    let slot = vm.parser.read_param::<Value>()?.eval(&vm.ctx) as usize;
    let mut fade = None;

    if info.param_len == 5 {
        let ms = vm.parser.read_param::<Value>()?.eval(&vm.ctx);
        fade = ms.is_positive().then(|| Duration::from_millis(ms as u64));
    }

    vm.audio.play_sfx(slot, fade)?;

    Ok(())
}
