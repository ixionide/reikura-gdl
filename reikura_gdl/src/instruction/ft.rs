use crate::{Vm, instruction::InstructionInfo};

pub fn ft(vm: &mut Vm, _info: InstructionInfo) -> anyhow::Result<()> {
    let src = vm.parser.read_param::<u16>()? as usize;
    let dst = vm.parser.read_param::<u16>()? as usize;
    let count = vm.parser.read_param::<u16>()? as usize;
    let bound_count = vm.ctx.flags.len() - src.max(dst);
    let range = 0..count.min(bound_count);

    if src < dst {
        // copy in descending order
        for i in range.rev() {
            let value = vm.ctx.flags.get(src + i);
            vm.ctx.flags.set(dst + i, value);
        }
    } else {
        // copy in ascending order
        for i in range {
            let value = vm.ctx.flags.get(src + i);
            vm.ctx.flags.set(dst + i, value);
        }
    }

    Ok(())
}
