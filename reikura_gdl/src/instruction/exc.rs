use std::cmp::min;

use anyhow::bail;

use crate::{
    Vm,
    instruction::{InstructionInfo, Value},
};

reikura_util::const_iota! {
    u8 = iota,
    COPY_FLAG,
    COPY_REGISTER,
}

pub fn exc(vm: &mut Vm, _info: InstructionInfo) -> anyhow::Result<()> {
    let ctx = &mut vm.ctx;
    let Some(save) = &mut vm.save else {
        bail!("EXC: save is not initialized yet");
    };

    let src = vm.parser.read_param::<Value>()?.evaluate(ctx) as usize;
    let dst = vm.parser.read_param::<Value>()?.evaluate(ctx) as usize;
    let count = vm.parser.read_param::<Value>()?.evaluate(ctx) as usize;

    match vm.parser.read_param::<u8>()? {
        COPY_FLAG => {
            let bound_count = min(save.flags.len() - src, ctx.flags.len() - dst);

            for i in 0..count.min(bound_count) {
                let flag = save.flags.get(src + i).unwrap_or(false);
                ctx.flags.set(dst + i, flag);
            }
        }
        COPY_REGISTER => {
            let bound_count = min(save.registers.len() - src, ctx.registers.len() - dst);

            for i in 0..count.min(bound_count) {
                let value = save.registers.get(src + i).unwrap_or(0);
                ctx.registers.set(dst + i, value);
            }
        }
        _ => bail!("invalid EXC copy param"),
    }

    Ok(())
}
