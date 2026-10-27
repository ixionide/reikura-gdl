use std::cmp::min;

use anyhow::bail;

use crate::{
    Vm,
    instruction::{InstructionInfo, Value},
};

reikura_util::const_iota! {
    u8 = iota,
    SAVE_FLAG,
    SAVE_REGISTER,
}

pub fn exs(vm: &mut Vm, _info: InstructionInfo) -> anyhow::Result<()> {
    let save_pos = vm.parser.read_param::<Value>()?.eval(&vm.ctx) as usize;
    let vm_pos = vm.parser.read_param::<Value>()?.eval(&vm.ctx) as usize;
    let count = vm.parser.read_param::<Value>()?.eval(&vm.ctx) as usize;

    match vm.parser.read_param::<u8>()? {
        SAVE_FLAG => {
            let vm_flags = &vm.ctx.flags;
            let Some(save_flags) = &mut vm.save.flags else {
                return Ok(());
            };

            let bound_count = min(
                vm_flags.len().saturating_sub(vm_pos),
                save_flags.len().saturating_sub(save_pos),
            );

            for i in 0..count.min(bound_count) {
                let flag = vm_flags.get(vm_pos + i);
                save_flags.set(save_pos + i, flag);
            }
        }
        SAVE_REGISTER => {
            let vm_registers = &vm.ctx.registers;
            let Some(save_registers) = &mut vm.save.registers else {
                return Ok(());
            };

            let bound_count = min(
                vm_registers.len().saturating_sub(vm_pos),
                save_registers.len().saturating_sub(save_pos),
            );

            for i in 0..count.min(bound_count) {
                let value = vm_registers.get(vm_pos + i);
                save_registers.set(save_pos + i, value);
            }
        }
        _ => bail!("invalid save"),
    }

    Ok(())
}
