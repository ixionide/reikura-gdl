use anyhow::bail;
use reikura_util::io::ReadExt;

use crate::instruction::{Evaluate, Instruction, ReadParam, Value};

pub struct Calc;

impl Instruction for Calc {
    fn execute(vm: &mut crate::Vm, _info: super::InstructionInfo) -> anyhow::Result<()> {
        let var_index: u16 = vm.scene.param()?;
        let mut result = 0;

        loop {
            match vm.scene.read_le::<u8>()? {
                0 => {
                    let rhs: Value = vm.scene.param()?;
                    result += rhs.evaluate(&vm.ctx);
                }
                1 => {
                    let rhs: Value = vm.scene.param()?;
                    result -= rhs.evaluate(&vm.ctx);
                }
                2 => {
                    let rhs: Value = vm.scene.param()?;
                    result *= rhs.evaluate(&vm.ctx);
                }
                3 => {
                    let rhs = vm.scene.param::<Value>()?.evaluate(&vm.ctx);
                    if rhs != 0 {
                        result /= rhs;
                    }
                }
                4 => {
                    let rhs = vm.scene.param::<Value>()?.evaluate(&vm.ctx);
                    if rhs != 0 {
                        result %= rhs;
                    }
                }
                5 => {
                    vm.ctx.variables.set(var_index as usize, result);
                    break;
                }
                unk @ 6.. => bail!("unknown calc operation: {unk}"),
            }
        }

        Ok(())
    }
}
