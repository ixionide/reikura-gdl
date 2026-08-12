use crate::{
    Vm,
    instruction::{AssetName, InstructionInfo},
};

pub fn pcml(vm: &mut Vm, _info: InstructionInfo) -> anyhow::Result<()> {
    let name: AssetName = vm.parser.read_param()?;
    let voice = vm.assets.load_voice(name)?;

    vm.audio.voice = Some(voice);

    Ok(())
}
