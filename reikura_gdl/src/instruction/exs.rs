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

pub fn exs(vm: &mut Vm, _info: InstructionInfo) -> anyhow::Result<()> {
    let ctx = &mut vm.ctx;
    let Some(save) = &mut vm.save else {
        bail!("save context is not initialized yet");
    };

    let save_ctx_pos = vm.parser.read_param::<Value>()?.evaluate(ctx) as usize;
    let vm_ctx_pos = vm.parser.read_param::<Value>()?.evaluate(ctx) as usize;
    let count = vm.parser.read_param::<Value>()?.evaluate(ctx) as usize;

    match vm.parser.read_param::<u8>()? {
        COPY_FLAG => {
            let bound_count = min(
                ctx.flags.len() - vm_ctx_pos,
                save.flags.len() - save_ctx_pos,
            );

            for i in 0..count.min(bound_count) {
                let flag = ctx.flags.get(vm_ctx_pos + i);
                save.flags.set(save_ctx_pos + i, flag);
            }
        }
        COPY_REGISTER => {
            let bound_count = min(
                ctx.registers.len() - vm_ctx_pos,
                save.registers.len() - save_ctx_pos,
            );

            for i in 0..count.min(bound_count) {
                let value = ctx.registers.get(vm_ctx_pos + i);
                save.registers.set(save_ctx_pos + i, value);
            }
        }
        _ => bail!("invalid copy"),
    }

    Ok(())
}
