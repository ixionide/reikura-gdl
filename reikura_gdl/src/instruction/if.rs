use anyhow::{bail, ensure};

use crate::{
    Parser,
    instruction::{Instruction, Value},
};

reikura_util::const_iota! {
    u8 = iota,
    EQ,
    LT,
    LE,
    GT,
    GE,
    NE,
}

reikura_util::const_iota! {
    u8 = iota,
    JUMP_SUB,
    SET_REG,
    CONTINUE,
}

pub struct If;

impl Instruction for If {
    fn execute(vm: &mut crate::Vm, _info: super::InstructionInfo) -> anyhow::Result<()> {
        let mut conds = Vec::with_capacity(10);
        let end: u8;

        fn read_param_cond(parser: &mut Parser) -> anyhow::Result<(Value, u8, Value)> {
            let lhs: Value = parser.read_param()?;
            let op: u8 = parser.read_param()?;
            let rhs: Value = parser.read_param()?;

            if ![EQ, LT, LE, GT, GE, NE].contains(&op) {
                bail!("unknown IF operator: {op}");
            }

            Ok((lhs, op, rhs))
        }

        let check_cond = |&(lhs, op, rhs): &(Value, u8, Value)| {
            let ctx = &vm.ctx;
            match op {
                EQ => lhs.evaluate(ctx) == rhs.evaluate(ctx),
                LT => lhs.evaluate(ctx) < rhs.evaluate(ctx),
                LE => lhs.evaluate(ctx) <= rhs.evaluate(ctx),
                GT => lhs.evaluate(ctx) > rhs.evaluate(ctx),
                GE => lhs.evaluate(ctx) >= rhs.evaluate(ctx),
                NE => lhs.evaluate(ctx) != rhs.evaluate(ctx),
                _ => unreachable!(),
            }
        };

        loop {
            conds.push(read_param_cond(&mut vm.parser)?);
            let cmd = vm.parser.read_param()?;

            match cmd {
                JUMP_SUB => {
                    let sub_index: u16 = vm.parser.read_param()?;
                    end = vm.parser.read_param()?;

                    if conds.iter().all(check_cond) {
                        vm.parser.jump_sub(sub_index)?;
                    }
                }
                SET_REG => {
                    let reg_index = vm.parser.read_param::<u16>()? as usize;
                    let reg_value: Value = vm.parser.read_param()?;
                    end = vm.parser.read_param()?;

                    if conds.iter().all(check_cond) {
                        let reg_value = reg_value.evaluate(&vm.ctx);
                        vm.ctx.registers.set(reg_index, reg_value);
                    }
                }
                CONTINUE => continue,
                _ => bail!("unknown IF command: {cmd}"),
            }

            break;
        }

        ensure!(end == 255, "IF instruction isn't end properly: {end}");

        Ok(())
    }
}
