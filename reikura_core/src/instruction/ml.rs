use crate::instruction::{AssetName, Instruction, ReadParam};

pub struct Ml;

impl Instruction for Ml {
    fn execute(vm: &mut crate::Vm, _info: super::InstructionInfo) -> anyhow::Result<()> {
        let name: AssetName = vm.scene.param()?;
        let _unknown: u8 = vm.scene.param()?;
        let bgm = vm.assets.load_bgm(name.decode()?.as_str())?;
        vm.audio.bgm = Some(bgm);

        Ok(())
    }
}
