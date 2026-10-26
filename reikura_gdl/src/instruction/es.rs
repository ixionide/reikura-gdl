use anyhow::bail;

use crate::{Vm, instruction::InstructionInfo};

pub fn es(vm: &mut Vm, _info: InstructionInfo) -> anyhow::Result<()> {
    let ctx = &vm.ctx;
    let Some(save) = &mut vm.save else {
        bail!("save context is not initialized yet");
    };

    let start = vm.parser.read_param::<u16>()? as usize;
    let end = vm.parser.read_param::<u16>()? as usize;
    let bound_end = ctx.flags.len() - 1;

    let ctx_range = start..=end.min(bound_end);
    let save_range = 0..save.flags.len();

    for (ctx_i, save_i) in ctx_range.zip(save_range) {
        let flag = ctx.flags.get(ctx_i);
        save.flags.set(save_i, flag);
    }

    Ok(())
}
