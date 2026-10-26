use crate::{
    Vm,
    instruction::{AssetName, InstructionInfo, Value},
};

pub fn ser(vm: &mut Vm, _info: InstructionInfo) -> anyhow::Result<()> {
    let name: AssetName = vm.parser.read_param()?;
    let slot = vm.parser.read_param::<Value>()?.eval(&vm.ctx) as usize;
    let sfx = vm.assets.load_sfx(name)?;

    vm.audio.set_sfx(slot, sfx);

    Ok(())
}
