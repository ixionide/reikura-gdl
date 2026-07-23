use crate::instruction::{AssetName, Instruction, InstructionInfo};

pub struct Lsbs;

impl Instruction for Lsbs {
    fn execute(vm: &mut crate::Vm, _info: InstructionInfo) -> anyhow::Result<()> {
        let asset_name: AssetName = vm.parser.read_param()?;
        let name = asset_name.decode()?;
        let scene = vm.assets.load_scene(&name)?;

        vm.parser.call_scene(scene)?;

        Ok(())
    }
}
