use crate::{
    Vm,
    instruction::{InstructionInfo, Value},
};

pub fn hsg(vm: &mut Vm, _info: InstructionInfo) -> anyhow::Result<()> {
    let start = vm.parser.read_param::<u16>()? as usize;
    let end = vm.parser.read_param::<u16>()? as usize;
    let reg_value: Value = vm.parser.read_param()?;
    let bound_end = vm.ctx.registers.len() - 1;
    let range = start..=end.min(bound_end);

    if reg_value.is_random() {
        for index in range {
            // generate random number every iteration
            let random_value = reg_value.evaluate(&vm.ctx);
            vm.ctx.registers.set(index, random_value);
        }
    } else {
        // evaluate once
        let value = reg_value.evaluate(&vm.ctx);
        for index in range {
            vm.ctx.registers.set(index, value);
        }
    }

    Ok(())
}
