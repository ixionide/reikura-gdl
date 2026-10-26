use anyhow::bail;

use crate::{Vm, instruction::InstructionInfo};

pub fn ec(vm: &mut Vm, _info: InstructionInfo) -> anyhow::Result<()> {
    let ctx = &mut vm.ctx;
    let Some(save) = &vm.save else {
        bail!("save context is not initialized yet");
    };

    let start = vm.parser.read_param::<u16>()? as usize;
    let end = vm.parser.read_param::<u16>()? as usize;
    let bound_end = ctx.flags.len() - 1;

    let save_range = 0..save.flags.len();
    let ctx_range = start..=end.min(bound_end);

    for (save_i, ctx_i) in save_range.zip(ctx_range) {
        let flag = save.flags.get(save_i);
        ctx.flags.set(ctx_i, flag);
    }

    Ok(())
}
