use anyhow::{bail, ensure};

use crate::{
    Scenario,
    instruction::{Evaluate, Instruction, ReadParam, Value},
};

const OP_EQ: u8 = 0;
const OP_LT: u8 = 1;
const OP_LE: u8 = 2;
const OP_GT: u8 = 3;
const OP_GE: u8 = 4;
const OP_NE: u8 = 5;

const CMD_JUMP_SUB: u8 = 0;
const CMD_SET_VAR: u8 = 1;
const CMD_CONTINUE: u8 = 2;

pub struct If;

impl Instruction for If {
    fn execute(vm: &mut crate::Vm, _info: super::InstructionInfo) -> anyhow::Result<()> {
        let mut conds = Vec::with_capacity(10);
        let end: u8;

        fn read_param_cond(scene: &mut Scenario) -> anyhow::Result<(Value, u8, Value)> {
            let lhs: Value = scene.param()?;
            let op: u8 = scene.param()?;
            let rhs: Value = scene.param()?;

            if ![OP_EQ, OP_LT, OP_LE, OP_GT, OP_GE, OP_NE].contains(&op) {
                bail!("unknown IF operator: {op}");
            }

            Ok((lhs, op, rhs))
        }

        let check_cond = |&(lhs, op, rhs): &(Value, u8, Value)| {
            let ctx = &vm.ctx;
            match op {
                OP_EQ => lhs.evaluate(ctx) == rhs.evaluate(ctx),
                OP_LT => lhs.evaluate(ctx) < rhs.evaluate(ctx),
                OP_LE => lhs.evaluate(ctx) <= rhs.evaluate(ctx),
                OP_GT => lhs.evaluate(ctx) > rhs.evaluate(ctx),
                OP_GE => lhs.evaluate(ctx) >= rhs.evaluate(ctx),
                OP_NE => lhs.evaluate(ctx) != rhs.evaluate(ctx),
                _ => unreachable!(),
            }
        };

        loop {
            conds.push(read_param_cond(&mut vm.scene)?);
            let cmd = vm.scene.param()?;

            match cmd {
                CMD_JUMP_SUB => {
                    let sub_index: u16 = vm.scene.param()?;
                    end = vm.scene.param()?;

                    if conds.iter().all(check_cond) {
                        vm.scene.jump_sub(sub_index)?;
                    }
                }
                CMD_SET_VAR => {
                    let var_index = vm.scene.param::<u16>()? as usize;
                    let var_value: Value = vm.scene.param()?;
                    end = vm.scene.param()?;

                    if conds.iter().all(check_cond) {
                        let var_value = var_value.evaluate(&vm.ctx);
                        vm.ctx.variables.set(var_index, var_value);
                    }
                }
                CMD_CONTINUE => continue,
                _ => bail!("unknown IF command: {cmd}"),
            }

            break;
        }

        ensure!(end == 255, "IF instruction isn't end properly: {end}");

        Ok(())
    }
}
