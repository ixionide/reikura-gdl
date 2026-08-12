use std::time::Duration;

use anyhow::bail;

use crate::{
    Vm,
    instruction::{InstructionInfo, Value},
};

reikura_util::const_iota! {
    u8 = iota,
    LOOP,
    ONCE,
}

pub fn dap(vm: &mut Vm, info: InstructionInfo) -> anyhow::Result<()> {
    let track_num = vm.parser.read_param::<Value>()?.evaluate(&vm.ctx) as u8;
    let cmd: u8 = vm.parser.read_param()?;
    let mut fade = None;

    if track_num == 0 {
        bail!("DAP: track_num is zero")
    }

    let looping = match cmd {
        LOOP => true,
        ONCE => false,
        _ => bail!("unknown DAP cmd: {cmd}"),
    };

    if info.param_len == 11 {
        let ms = vm.parser.read_param::<Value>()?.evaluate(&vm.ctx);
        fade = ms.is_positive().then(|| Duration::from_millis(ms as u64));
    }

    let bgm = vm.assets.load_cdda(track_num)?;

    vm.audio.bgm = Some(bgm);
    vm.audio.play_bgm(looping, fade)?;

    Ok(())
}
