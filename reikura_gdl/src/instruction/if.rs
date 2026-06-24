use std::io::{Seek, SeekFrom};

use anyhow::{Ok, bail, ensure};

use crate::instruction::{Evaluate, Instruction, ReadParam, Value};

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
    fn execute(vm: &mut crate::Vm, info: super::InstructionInfo) -> anyhow::Result<()> {
        let next_inst = vm.scene.ip + info.param_length();

        fn check_cond(vm: &mut crate::Vm) -> anyhow::Result<bool> {
            let lhs: Value = vm.scene.param()?;
            let op: u8 = vm.scene.param()?;
            let rhs: Value = vm.scene.param()?;

            let cond = match op {
                OP_EQ => lhs.evaluate(&vm.ctx) == rhs.evaluate(&vm.ctx),
                OP_LT => lhs.evaluate(&vm.ctx) < rhs.evaluate(&vm.ctx),
                OP_LE => lhs.evaluate(&vm.ctx) <= rhs.evaluate(&vm.ctx),
                OP_GT => lhs.evaluate(&vm.ctx) > rhs.evaluate(&vm.ctx),
                OP_GE => lhs.evaluate(&vm.ctx) >= rhs.evaluate(&vm.ctx),
                OP_NE => lhs.evaluate(&vm.ctx) != rhs.evaluate(&vm.ctx),
                _ => bail!("unknown IF operator: {op}"),
            };

            Ok(cond)
        }

        while check_cond(vm)? {
            let cmd: u8 = vm.scene.param()?;

            match cmd {
                CMD_JUMP_SUB => {
                    let sub_index: u16 = vm.scene.param()?;

                    vm.scene.jump_sub(sub_index)?;
                }
                CMD_SET_VAR => {
                    let var_index = vm.scene.param::<u16>()? as usize;
                    let var_value = vm.scene.param::<Value>()?.evaluate(&vm.ctx);

                    vm.ctx.variables.set(var_index, var_value);
                }
                CMD_CONTINUE => continue,
                _ => bail!("unknown if command: {cmd}"),
            }

            let end: u8 = vm.scene.param()?;
            ensure!(end == 255, "if instruction isn't end properly");
            return Ok(());
        }

        vm.scene.seek(SeekFrom::Start(next_inst as u64))?;
        Ok(())
    }
}
