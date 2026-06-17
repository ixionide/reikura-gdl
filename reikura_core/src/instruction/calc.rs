use anyhow::bail;
use reikura_util::io::ReadExt;

use crate::instruction::{Evaluate, Instruction, ReadParam, Value};

pub struct Calc;

impl Instruction for Calc {
    fn execute(vm: &mut crate::Vm, _info: super::InstructionInfo) -> anyhow::Result<()> {
        let var_index: u16 = vm.scene.param()?;
        let mut result = 0;
        let mut tmp = None;

        loop {
            match vm.scene.read_le::<u8>()? {
                0 => {
                    let rhs = vm.scene.param::<Value>()?.evaluate(&vm.ctx);

                    if let Some(num) = tmp.replace(rhs) {
                        result += num;
                    }
                }
                1 => {
                    let rhs = vm.scene.param::<Value>()?.evaluate(&vm.ctx);

                    if let Some(num) = tmp.replace(-rhs) {
                        result += num;
                    }
                }
                2 => {
                    let rhs = vm.scene.param::<Value>()?.evaluate(&vm.ctx);

                    if let Some(lhs) = &mut tmp {
                        *lhs *= rhs
                    }
                }
                3 => {
                    let rhs = vm.scene.param::<Value>()?.evaluate(&vm.ctx);

                    if let Some(lhs) = &mut tmp
                        && rhs != 0
                    {
                        *lhs /= rhs;
                    }
                }
                4 => {
                    let rhs = vm.scene.param::<Value>()?.evaluate(&vm.ctx);

                    if let Some(lhs) = &mut tmp
                        && rhs != 0
                    {
                        *lhs %= rhs;
                    }
                }
                5 => {
                    if let Some(rhs) = tmp.take() {
                        result += rhs;
                    }

                    vm.ctx.variables.set(var_index as usize, result);
                    break;
                }
                unk => bail!("unknown calc operator: {unk}"),
            }
        }

        Ok(())
    }
}
