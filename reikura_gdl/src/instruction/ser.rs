use crate::{
    audio::MAX_SFX_SLOT,
    instruction::{AssetName, Evaluate, Instruction, Value},
};

pub struct Ser;

impl Instruction for Ser {
    fn execute(vm: &mut crate::Vm, _info: super::InstructionInfo) -> anyhow::Result<()> {
        let name: AssetName = vm.parser.read_param()?;
        let slot = vm.parser.read_param::<Value>()?.evaluate(&vm.ctx) as usize;
        let sfx = vm.assets.load_sfx(name)?;

        vm.audio.sfx[slot % MAX_SFX_SLOT] = Some(sfx);

        Ok(())
    }
}
