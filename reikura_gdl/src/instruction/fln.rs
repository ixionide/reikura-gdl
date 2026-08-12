use crate::{Vm, instruction::InstructionInfo};

pub fn fln(vm: &mut Vm, _info: InstructionInfo) -> anyhow::Result<()> {
    let flag_count: u16 = vm.parser.read_param()?;

    vm.ctx.flags.resize(flag_count as usize);

    Ok(())
}
