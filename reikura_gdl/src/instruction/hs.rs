use crate::{
    Vm,
    instruction::{InstructionInfo, Value},
};

pub fn hs(vm: &mut Vm, _info: InstructionInfo) -> anyhow::Result<()> {
    let reg_index = vm.parser.read_param::<u16>()? as usize;
    let reg_value: Value = vm.parser.read_param()?;
    let value = reg_value.evaluate(&vm.ctx);

    vm.ctx.registers.set(reg_index, value);

    Ok(())
}
