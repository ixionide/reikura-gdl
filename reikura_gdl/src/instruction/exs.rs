use std::cmp::min;

use anyhow::bail;

use crate::instruction::{Evaluate, Instruction, ReadParam, Value};

const COPY_FLAG: u8 = 0;
const COPY_VARIABLE: u8 = 1;

pub struct Exs;

impl Instruction for Exs {
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
                let bound_count = min(ctx.flags.len() - src, save.flags.len() - dst);

                for i in 0..count.min(bound_count) {
                    let flag = ctx.flags.get(src + i).unwrap_or(false);
                    save.flags.set(dst + i, flag);
                }
            }
            COPY_VARIABLE => {
                let bound_count = min(ctx.variables.len() - src, save.variables.len() - dst);

                for i in 0..count.min(bound_count) {
                    let value = ctx.variables.get(src + i).unwrap_or(0);
                    save.variables.set(dst + i, value);
                }
            }
            _ => bail!("invalid EXS copy param"),
        }

        Ok(())
    }
}
