use anyhow::bail;

use crate::{
    Vm,
    instruction::{InstructionInfo, ParamString},
};

pub fn setgameinfo(vm: &mut Vm, _info: InstructionInfo) -> anyhow::Result<()> {
    let game_info: ParamString = vm.parser.read_param()?;

    let Some(save) = &mut vm.save else {
        bail!("save is not initialized yet");
    };

    save.game_info = game_info.decode_sjis()?.into();

    Ok(())
}
