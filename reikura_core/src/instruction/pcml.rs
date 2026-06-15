use crate::instruction::{AssetName, Instruction, ReadParam};

pub struct Pcml;

impl Instruction for Pcml {
    fn execute(vm: &mut crate::Vm, _info: super::InstructionInfo) -> anyhow::Result<()> {
        let name: AssetName = vm.scene.param()?;
        let voice = vm.assets.load_voice(name.decode()?.as_str())?;

        vm.audio.voice = Some(voice);

        Ok(())
    }
}
