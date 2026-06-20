use anyhow::bail;

use crate::instruction::{Instruction, ParamString, ReadParam};

pub struct Setgameinfo;

impl Instruction for Setgameinfo {
    fn execute(vm: &mut crate::Vm, _info: super::InstructionInfo) -> anyhow::Result<()> {
        let game_info: ParamString = vm.scene.param()?;

        let Some(save) = &mut vm.save else {
            bail!("save is not initialized yet");
        };

        save.game_info = game_info.decode()?.into();

        Ok(())
    }
}
