use crate::instruction::{AssetName, Instruction};

pub struct Pcml;

impl Instruction for Pcml {
    fn execute(vm: &mut crate::Vm, _info: super::InstructionInfo) -> anyhow::Result<()> {
        let name: AssetName = vm.parser.read_param()?;
        let voice = vm.assets.load_voice(name.decode()?.as_str())?;

        vm.audio.voice = Some(voice);

        Ok(())
    }
}
