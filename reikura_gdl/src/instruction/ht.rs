use anyhow::bail;

use crate::{Vm, instruction::InstructionInfo};

pub fn ht(vm: &mut Vm, _info: InstructionInfo) -> anyhow::Result<()> {
    let src = vm.parser.read_param::<u16>()? as usize;
    let dst = vm.parser.read_param::<u16>()? as usize;
    let count = vm.parser.read_param::<u16>()? as usize;
    let bound_count = vm.ctx.registers.len() - src.max(dst);
    let range = 0..count.min(bound_count);

    if src < dst {
        // descending
        for i in range.rev() {
            match vm.ctx.registers.get(src + i) {
                Some(value) => vm.ctx.registers.set(dst + 1, value),
                None => bail!("register index out of bounds: {}", src + i),
            };
        }
    } else {
        // ascending
        for i in range {
            match vm.ctx.registers.get(src + i) {
                Some(value) => vm.ctx.registers.set(dst + 1, value),
                None => bail!("register index out of bounds: {}", src + i),
            };
        }
    }

    Ok(())
}
