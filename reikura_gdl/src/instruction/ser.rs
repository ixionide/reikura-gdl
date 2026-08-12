use crate::{
    Vm,
    audio::SFX_SLOT,
    instruction::{AssetName, InstructionInfo, Value},
};

pub fn ser(vm: &mut Vm, _info: InstructionInfo) -> anyhow::Result<()> {
    let name: AssetName = vm.parser.read_param()?;
    let slot = vm.parser.read_param::<Value>()?.evaluate(&vm.ctx) as usize;
    let sfx = vm.assets.load_sfx(name)?;

    vm.audio.sfx[slot % SFX_SLOT] = Some(sfx);

    Ok(())
}
