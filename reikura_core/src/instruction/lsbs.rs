use crate::instruction::{AssetName, Instruction, ReadParam};

pub struct Lsbs;

impl Instruction for Lsbs {
    fn execute(vm: &mut crate::Vm) -> anyhow::Result<()> {
        let asset_name: AssetName = vm.scene.param()?;
        let name = asset_name.decode()?;
        let scene = vm.assets.load_scenario(&name)?;
        vm.scene.call(scene)
    }
}
