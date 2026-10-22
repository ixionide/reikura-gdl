use anyhow::bail;

use crate::{
    Vm,
    instruction::{InstructionInfo, Value},
};

reikura_util::const_iota! {
    u8 = iota,
    VOICE,
    BGM,
}

pub fn setinsidevol(vm: &mut Vm, _info: InstructionInfo) -> anyhow::Result<()> {
    let track: u8 = vm.parser.read_param()?;
    let volume: u8 = vm
        .parser
        .read_param::<Value>()?
        .evaluate(&vm.ctx)
        .try_into()?;

    {
        let settings = &mut vm.audio.track.volume;
        let volume = f64::from(volume.min(100)) / f64::from(100_u8);
        match track {
            VOICE => settings.voice_inside.set(volume),
            BGM => settings.bgm_inside.set(volume),
            _ => bail!("invalid SETINSIDEVOL track: {track}"),
        }
    }

    Ok(())
}
