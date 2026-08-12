use std::time::Duration;

use crate::{
    Vm,
    instruction::{InstructionInfo, Value},
};

pub fn mf(vm: &mut Vm, _info: InstructionInfo) -> anyhow::Result<()> {
    let ms = vm.parser.read_param::<Value>()?.evaluate(&vm.ctx);
    let fade = ms.is_positive().then(|| Duration::from_millis(ms as u64));

    vm.audio.stop_bgm(fade);

    Ok(())
}
