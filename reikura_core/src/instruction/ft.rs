use anyhow::bail;

use crate::instruction::{Instruction, ReadParam};

pub struct Ft;

impl Instruction for Ft {
    fn execute(vm: &mut crate::Vm, _info: super::InstructionInfo) -> anyhow::Result<()> {
        let src = vm.scene.param::<u16>()? as usize;
        let dst = vm.scene.param::<u16>()? as usize;
        let count = vm.scene.param::<u16>()? as usize;
        let bound_count = vm.ctx.flags.len() - src.max(dst);
        let range = 0..count.min(bound_count);

        if src < dst {
            // copy in descending order
            for i in range.rev() {
                match vm.ctx.flags.get(src + i) {
                    Some(value) => vm.ctx.flags.set(dst + i, value),
                    None => bail!("flag index out of bounds: {}", src + i),
                };
            }
        } else {
            // copy in ascending order
            for i in range {
                match vm.ctx.flags.get(src + i) {
                    Some(value) => vm.ctx.flags.set(dst + i, value),
                    None => bail!("flag index out of bounds: {}", src + i),
                };
            }
        }

        Ok(())
    }
}
