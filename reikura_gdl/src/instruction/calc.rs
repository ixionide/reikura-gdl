use anyhow::bail;

use crate::instruction::{Instruction, Value};

reikura_util::const_iota! {
    u8 = iota,
    ADD,
    MIN,
    MUL,
    DIV,
    MOD,
    END,
}

pub struct Calc;

impl Instruction for Calc {
    fn execute(vm: &mut crate::Vm, _info: super::InstructionInfo) -> anyhow::Result<()> {
        let reg_index: u16 = vm.parser.read_param()?;
        let mut result = 0;
        let mut tmp = None;

        loop {
            match vm.parser.read_param::<u8>()? {
                ADD => {
                    let rhs = vm.parser.read_param::<Value>()?.evaluate(&vm.ctx);

                    if let Some(num) = tmp.replace(rhs) {
                        result += num;
                    }
                }
                MIN => {
                    let rhs = vm.parser.read_param::<Value>()?.evaluate(&vm.ctx);

                    if let Some(num) = tmp.replace(-rhs) {
                        result += num;
                    }
                }
                MUL => {
                    let rhs = vm.parser.read_param::<Value>()?.evaluate(&vm.ctx);

                    if let Some(lhs) = &mut tmp {
                        *lhs *= rhs
                    }
                }
                DIV => {
                    let rhs = vm.parser.read_param::<Value>()?.evaluate(&vm.ctx);

                    if let Some(lhs) = &mut tmp
                        && rhs != 0
                    {
                        *lhs /= rhs;
                    }
                }
                MOD => {
                    let rhs = vm.parser.read_param::<Value>()?.evaluate(&vm.ctx);

                    if let Some(lhs) = &mut tmp
                        && rhs != 0
                    {
                        *lhs %= rhs;
                    }
                }
                END => {
                    if let Some(rhs) = tmp.take() {
                        result += rhs;
                    }

                    vm.ctx.registers.set(reg_index as usize, result);
                    break;
                }
                unk => bail!("unknown CALC operator: {unk}"),
            }
        }

        Ok(())
    }
}
