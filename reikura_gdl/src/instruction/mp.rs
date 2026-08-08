use std::time::Duration;

use anyhow::bail;

use crate::instruction::{Instruction, Value};

reikura_util::const_iota! {
    u8 = iota,
    LOOP,
    ONCE,
}

pub struct Mp;

impl Instruction for Mp {
    fn execute(vm: &mut crate::Vm, info: super::InstructionInfo) -> anyhow::Result<()> {
        let cmd: u8 = vm.parser.read_param()?;
        let mut fade = None;

        if info.param_length() == 5 {
            let ms = vm.parser.read_param::<Value>()?.evaluate(&vm.ctx);
            fade = ms.is_positive().then(|| Duration::from_millis(ms as u64));
        }

        let looping = match cmd {
            LOOP => true,
            ONCE => false,
            _ => bail!("unknown MP cmd: {cmd}"),
        };

        vm.audio.play_bgm(looping, fade)?;

        Ok(())
    }
}
