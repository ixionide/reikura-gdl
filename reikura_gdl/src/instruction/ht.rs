use anyhow::bail;

use crate::instruction::Instruction;

pub struct Ht;

impl Instruction for Ht {
    fn execute(vm: &mut crate::Vm, _info: super::InstructionInfo) -> anyhow::Result<()> {
        let src = vm.parser.read_param::<u16>()? as usize;
        let dst = vm.parser.read_param::<u16>()? as usize;
        let count = vm.parser.read_param::<u16>()? as usize;
        let bound_count = vm.ctx.variables.len() - src.max(dst);
        let range = 0..count.min(bound_count);

        if src < dst {
            // descending
            for i in range.rev() {
                match vm.ctx.variables.get(src + i) {
                    Some(value) => vm.ctx.variables.set(dst + 1, value),
                    None => bail!("var index out of bounds: {}", src + i),
                };
            }
        } else {
            // ascending
            for i in range {
                match vm.ctx.variables.get(src + i) {
                    Some(value) => vm.ctx.variables.set(dst + 1, value),
                    None => bail!("var index out of bounds: {}", src + i),
                };
            }
        }

        Ok(())
    }
}
