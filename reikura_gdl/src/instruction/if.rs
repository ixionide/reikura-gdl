use std::io::{Seek, SeekFrom};

use anyhow::{Ok, bail, ensure};

use crate::instruction::{Evaluate, Instruction, ReadParam, Value};

pub struct If;

impl Instruction for If {
    fn execute(vm: &mut crate::Vm, info: super::InstructionInfo) -> anyhow::Result<()> {
        let next_inst = vm.scene.ip + info.param_length();

        fn check_cond(vm: &mut crate::Vm) -> anyhow::Result<bool> {
            let lhs: Value = vm.scene.param()?;
            let op: u8 = vm.scene.param()?;
            let rhs: Value = vm.scene.param()?;

            let cond = match op {
                0 => lhs.evaluate(&vm.ctx) == rhs.evaluate(&vm.ctx),
                1 => lhs.evaluate(&vm.ctx) < rhs.evaluate(&vm.ctx),
                2 => lhs.evaluate(&vm.ctx) <= rhs.evaluate(&vm.ctx),
                3 => lhs.evaluate(&vm.ctx) > rhs.evaluate(&vm.ctx),
                4 => lhs.evaluate(&vm.ctx) >= rhs.evaluate(&vm.ctx),
                5 => lhs.evaluate(&vm.ctx) != rhs.evaluate(&vm.ctx),
                _ => bail!("unknown if operator: {op}"),
            };

            Ok(cond)
        }

        while check_cond(vm)? {
            let cmd: u8 = vm.scene.param()?;

            match cmd {
                0 => {
                    let sub_index: u16 = vm.scene.param()?;

                    vm.scene.jump_sub(sub_index)?;
                }
                1 => {
                    let var_index = vm.scene.param::<u16>()? as usize;
                    let var_value = vm.scene.param::<Value>()?.evaluate(&vm.ctx);

                    vm.ctx.variables.set(var_index, var_value);
                }
                2 => continue,
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
