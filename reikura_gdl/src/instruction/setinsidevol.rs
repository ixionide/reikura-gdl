use anyhow::bail;

use crate::instruction::{Evaluate, Instruction, ReadParam, Value};

const TRACK_VOICE: u8 = 0;
const TRACK_BGM: u8 = 1;

pub struct Setinsidevol;

impl Instruction for Setinsidevol {
    fn execute(vm: &mut crate::Vm, _info: super::InstructionInfo) -> anyhow::Result<()> {
        let track: u8 = vm.scene.param()?;
        let volume: u8 = vm.scene.param::<Value>()?.evaluate(&vm.ctx).try_into()?;

        let (audio, track) = {
            match track {
                TRACK_VOICE => (vm.audio.voice.as_mut(), "voice"),
                TRACK_BGM => (vm.audio.bgm.as_mut(), "bgm"),
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
