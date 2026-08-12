use crate::{
    Vm,
    audio::SFX_SLOT,
    instruction::{InstructionInfo, Value},
};

pub fn sed(vm: &mut Vm, _info: InstructionInfo) -> anyhow::Result<()> {
    let slot = vm.parser.read_param::<Value>()?.evaluate(&vm.ctx) as usize;

    vm.audio.stop_sfx(slot % SFX_SLOT, None);

    Ok(())
}
