use crate::instruction::{AssetName, Instruction, InstructionInfo, ReadParam};

pub struct Ls;

impl Instruction for Ls {
    fn execute(vm: &mut crate::Vm, _info: InstructionInfo) -> anyhow::Result<()> {
        let asset_name: AssetName = vm.scene.param()?;
        let name = asset_name.decode()?;
        let scene = vm.assets.load_scenario(&name)?;

        vm.scene.jump(scene);

        Ok(())
    }
}
