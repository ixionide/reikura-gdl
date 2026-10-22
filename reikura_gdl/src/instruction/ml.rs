use anyhow::bail;

use crate::{
    Vm,
    instruction::{AssetName, InstructionInfo},
};

reikura_util::const_iota! {
    u8 = iota,
    AUTO_PLAY,
    NO_AUTO_PLAY,
}

pub fn ml(vm: &mut Vm, _info: InstructionInfo) -> anyhow::Result<()> {
    let name: AssetName = vm.parser.read_param()?;
    let cmd: u8 = vm.parser.read_param()?;
    let bgm = vm.assets.load_bgm(name)?;

    vm.audio.set_bgm(bgm);

    match cmd {
        AUTO_PLAY => vm.audio.play_bgm(true, None)?,
        NO_AUTO_PLAY => (),
        _ => bail!("unknown cmd: {cmd}"),
    }

    Ok(())
}
