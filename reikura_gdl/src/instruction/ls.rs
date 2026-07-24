use crate::instruction::{AssetName, Instruction, InstructionInfo};

pub struct Ls;

impl Instruction for Ls {
    fn execute(vm: &mut crate::Vm, _info: InstructionInfo) -> anyhow::Result<()> {
        let scene_name: AssetName = vm.parser.read_param()?;
        let scene = vm.assets.load_scene(scene_name)?;

        vm.parser.jump_scene(scene);

        Ok(())
    }
}
