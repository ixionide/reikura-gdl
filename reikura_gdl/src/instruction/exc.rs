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
        bail!("save context is not initialized yet");
    };

    let save_ctx_pos = vm.parser.read_param::<Value>()?.eval(ctx) as usize;
    let vm_ctx_pos = vm.parser.read_param::<Value>()?.eval(ctx) as usize;
    let count = vm.parser.read_param::<Value>()?.eval(ctx) as usize;

    match vm.parser.read_param::<u8>()? {
        COPY_FLAG => {
            let bound_count = min(
                save.flags.len().saturating_sub(save_ctx_pos),
                ctx.flags.len().saturating_sub(vm_ctx_pos),
            );

            for i in 0..count.min(bound_count) {
                let flag = save.flags.get(save_ctx_pos + i);
                ctx.flags.set(vm_ctx_pos + i, flag);
            }
        }
        COPY_REGISTER => {
            let bound_count = min(
                save.registers.len().saturating_sub(save_ctx_pos),
                ctx.registers.len().saturating_sub(vm_ctx_pos),
            );

            for i in 0..count.min(bound_count) {
                let value = save.registers.get(save_ctx_pos + i);
                ctx.registers.set(vm_ctx_pos + i, value);
            }
        }
        _ => bail!("invalid copy"),
    }

    Ok(())
}
