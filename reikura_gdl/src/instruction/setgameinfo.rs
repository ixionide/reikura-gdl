use crate::{
    Vm,
    instruction::{InstructionInfo, ParamString},
};

pub fn setgameinfo(vm: &mut Vm, _info: InstructionInfo) -> anyhow::Result<()> {
    let game_info: ParamString = vm.parser.read_param()?;

    vm.save.game_info = Some(game_info.decode_sjis()?);

    Ok(())
}
