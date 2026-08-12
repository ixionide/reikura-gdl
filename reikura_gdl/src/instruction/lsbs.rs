use crate::{
    Vm,
    instruction::{AssetName, InstructionInfo},
};

pub fn lsbs(vm: &mut Vm, _info: InstructionInfo) -> anyhow::Result<()> {
    let scene_name: AssetName = vm.parser.read_param()?;
    let scene = vm.assets.load_scene(scene_name)?;

    vm.parser.call_scene(scene)?;

    Ok(())
}
