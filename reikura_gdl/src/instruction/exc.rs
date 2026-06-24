use std::cmp::min;

use anyhow::bail;

use crate::instruction::{Evaluate, Instruction, ReadParam, Value};

const COPY_FLAG: u8 = 0;
const COPY_VARIABLE: u8 = 1;

pub struct Exc;

impl Instruction for Exc {
    fn execute(vm: &mut crate::Vm, _info: super::InstructionInfo) -> anyhow::Result<()> {
        let ctx = &mut vm.ctx;
        let Some(save) = &mut vm.save else {
            bail!("save is not initialized yet");
        };

        let dst = vm.scene.param::<Value>()?.evaluate(ctx) as usize;
        let src = vm.scene.param::<Value>()?.evaluate(ctx) as usize;
        let count = vm.scene.param::<Value>()?.evaluate(ctx) as usize;

        match vm.scene.param::<u8>()? {
            COPY_FLAG => {
                let bound_count = min(save.flags.len() - src, ctx.flags.len() - dst);

                for i in 0..count.min(bound_count) {
                    let flag = save.flags.get(src + i).unwrap_or(false);
                    ctx.flags.set(dst + i, flag);
                }
            }
            COPY_VARIABLE => {
                let bound_count = min(save.variables.len() - src, ctx.variables.len() - dst);

                for i in 0..count.min(bound_count) {
                    let value = save.variables.get(src + i).unwrap_or(0);
                    ctx.variables.set(dst + i, value);
                }
            }
            _ => bail!("invalid EXC copy param"),
        }

        Ok(())
    }
}
