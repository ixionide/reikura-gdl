use std::time::Duration;

use anyhow::bail;

use crate::instruction::{Evaluate, Instruction, ReadParam, Value};

reikura_util::const_iota! {
    u8 = iota,
    LOOP,
    ONCE,
}

pub struct Dap;

impl Instruction for Dap {
    fn execute(vm: &mut crate::Vm, info: super::InstructionInfo) -> anyhow::Result<()> {
        // TODO: implement fakecdda
        let _track_number = vm.scene.param::<Value>()?.evaluate(&vm.ctx);
        let cmd: u8 = vm.scene.param()?;
        let mut fade = None;

        if info.param_length() == 11 {
            let ms = vm.scene.param::<Value>()?.evaluate(&vm.ctx);
            fade = ms.is_positive().then(|| Duration::from_millis(ms as u64));
        }

        let looping = match cmd {
            LOOP => true,
            ONCE => false,
            _ => bail!("unknown DAP cmd: {cmd}"),
        };

        vm.audio.play_bgm(looping, fade)?;

        Ok(())
    }
}
