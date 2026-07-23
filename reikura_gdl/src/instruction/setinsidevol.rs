use anyhow::bail;

use crate::instruction::{Evaluate, Instruction, Value};

reikura_util::const_iota! {
    u8 = iota,
    VOICE,
    BGM,
}

pub struct Setinsidevol;

impl Instruction for Setinsidevol {
    fn execute(vm: &mut crate::Vm, _info: super::InstructionInfo) -> anyhow::Result<()> {
        let track: u8 = vm.parser.read_param()?;
        let volume: u8 = vm
            .parser
            .read_param::<Value>()?
            .evaluate(&vm.ctx)
            .try_into()?;

        let (audio, track) = {
            match track {
                VOICE => (vm.audio.voice.as_mut(), "voice"),
                BGM => (vm.audio.bgm.as_mut(), "bgm"),
                _ => bail!("invalid SETINSIDEVOL track: {track}"),
            }
        };

        let Some(audio) = audio else {
            bail!("{track} isn't loaded yet");
        };

        audio.volume = Some(f32::from(volume) / f32::from(u8::MAX));

        Ok(())
    }
}
