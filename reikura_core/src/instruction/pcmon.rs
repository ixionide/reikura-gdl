use crate::instruction::{Instruction, ReadParam};

pub struct Pcmon;

impl Instruction for Pcmon {
    fn execute(vm: &mut crate::Vm, _info: super::InstructionInfo) -> anyhow::Result<()> {
        let _unknown: u8 = vm.scene.param()?;

        vm.audio.play_voice(None)?;

        Ok(())
    }
}
