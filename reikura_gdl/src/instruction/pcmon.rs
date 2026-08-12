use crate::{Vm, instruction::InstructionInfo};

pub fn pcmon(vm: &mut Vm, _info: InstructionInfo) -> anyhow::Result<()> {
    let _unknown: u8 = vm.parser.read_param()?;

    vm.audio.play_voice(None)?;

    Ok(())
}
