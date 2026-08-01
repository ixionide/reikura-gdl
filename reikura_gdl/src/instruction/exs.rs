use std::cmp::min;

use anyhow::bail;

use crate::instruction::{Evaluate, Instruction, Value};

reikura_util::const_iota! {
    u8 = iota,
    COPY_FLAG,
    COPY_REGISTER,
}

pub struct Exs;

impl Instruction for Exs {
    fn execute(vm: &mut crate::Vm, _info: super::InstructionInfo) -> anyhow::Result<()> {
        let ctx = &mut vm.ctx;
        let Some(save) = &mut vm.save else {
            bail!("EXS: save is not initialized yet");
        };

        let dst = vm.parser.read_param::<Value>()?.evaluate(ctx) as usize;
        let src = vm.parser.read_param::<Value>()?.evaluate(ctx) as usize;
        let count = vm.parser.read_param::<Value>()?.evaluate(ctx) as usize;

        match vm.parser.read_param::<u8>()? {
            COPY_FLAG => {
                let bound_count = min(ctx.flags.len() - src, save.flags.len() - dst);

                for i in 0..count.min(bound_count) {
                    let flag = ctx.flags.get(src + i).unwrap_or(false);
                    save.flags.set(dst + i, flag);
                }
            }
            COPY_REGISTER => {
                let bound_count = min(ctx.registers.len() - src, save.registers.len() - dst);

                for i in 0..count.min(bound_count) {
                    let value = ctx.registers.get(src + i).unwrap_or(0);
                    save.registers.set(dst + i, value);
                }
            }
            _ => bail!("invalid EXS copy param"),
        }

        Ok(())
    }
}
