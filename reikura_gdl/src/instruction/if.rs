use anyhow::{bail, ensure};

use crate::{
    Scenario,
    instruction::{Evaluate, Instruction, ReadParam, Value},
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
    SET_VAR,
    CONTINUE,
}

pub struct If;

impl Instruction for If {
    fn execute(vm: &mut crate::Vm, _info: super::InstructionInfo) -> anyhow::Result<()> {
        let mut conds = Vec::with_capacity(10);
        let end: u8;

        fn read_param_cond(scene: &mut Scenario) -> anyhow::Result<(Value, u8, Value)> {
            let lhs: Value = scene.param()?;
            let op: u8 = scene.param()?;
            let rhs: Value = scene.param()?;

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
            conds.push(read_param_cond(&mut vm.scene)?);
            let cmd = vm.scene.param()?;

            match cmd {
                JUMP_SUB => {
                    let sub_index: u16 = vm.scene.param()?;
                    end = vm.scene.param()?;

                    if conds.iter().all(check_cond) {
                        vm.scene.jump_sub(sub_index)?;
                    }
                }
                SET_VAR => {
                    let var_index = vm.scene.param::<u16>()? as usize;
                    let var_value: Value = vm.scene.param()?;
                    end = vm.scene.param()?;

                    if conds.iter().all(check_cond) {
                        let var_value = var_value.evaluate(&vm.ctx);
                        vm.ctx.variables.set(var_index, var_value);
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
