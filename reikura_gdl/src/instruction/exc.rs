use std::cmp::min;

use anyhow::bail;

use crate::{
    Vm,
    instruction::{InstructionInfo, Value},
};

reikura_util::const_iota! {
    u8 = iota,
    LOAD_FLAG,
    LOAD_REGISTER,
}

pub fn exc(vm: &mut Vm, _info: InstructionInfo) -> anyhow::Result<()> {
    let save_pos = vm.parser.read_param::<Value>()?.eval(&vm.ctx) as usize;
    let vm_pos = vm.parser.read_param::<Value>()?.eval(&vm.ctx) as usize;
    let count = vm.parser.read_param::<Value>()?.eval(&vm.ctx) as usize;

    match vm.parser.read_param::<u8>()? {
        LOAD_FLAG => {
            let vm_flags = &mut vm.ctx.flags;
            let Some(save_flags) = &vm.save.flags else {
                return Ok(());
            };

            let bound_count = min(
                save_flags.len().saturating_sub(save_pos),
                vm_flags.len().saturating_sub(vm_pos),
            );

            for i in 0..count.min(bound_count) {
                let flag = save_flags.get(save_pos + i);
                vm_flags.set(vm_pos + i, flag);
            }
        }
        LOAD_REGISTER => {
            let vm_registers = &mut vm.ctx.registers;
            let Some(save_registers) = &vm.save.registers else {
                return Ok(());
            };

            let bound_count = min(
                save_registers.len().saturating_sub(save_pos),
                vm_registers.len().saturating_sub(vm_pos),
            );

            for i in 0..count.min(bound_count) {
                let value = save_registers.get(save_pos + i);
                vm_registers.set(vm_pos + i, value);
            }
        }
        _ => bail!("invalid load"),
    }

    Ok(())
}
