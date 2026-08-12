use crate::{AssetName, Vm, instruction::InstructionInfo};

pub fn ls(vm: &mut Vm, _info: InstructionInfo) -> anyhow::Result<()> {
    let scene_name: AssetName = vm.parser.read_param()?;
    let scene = vm.assets.load_scene(scene_name)?;

    vm.parser.jump_scene(scene);

    Ok(())
}
