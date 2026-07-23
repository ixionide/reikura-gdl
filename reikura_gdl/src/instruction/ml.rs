use anyhow::bail;

use crate::instruction::{AssetName, Instruction};

pub struct Ml;

reikura_util::const_iota! {
    u8 = iota,
    AUTO_PLAY,
    NO_AUTO_PLAY,
}

impl Instruction for Ml {
    fn execute(vm: &mut crate::Vm, _info: super::InstructionInfo) -> anyhow::Result<()> {
        let name: AssetName = vm.parser.read_param()?;
        let cmd: u8 = vm.parser.read_param()?;
        let bgm = vm.assets.load_bgm(name.decode()?.as_str())?;

        vm.audio.bgm = Some(bgm);

        match cmd {
            AUTO_PLAY => vm.audio.play_bgm(true, None)?,
            NO_AUTO_PLAY => (),
            _ => bail!("unknown ML cmd: {cmd}"),
        }

        Ok(())
    }
}
