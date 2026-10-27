use crate::{Vm, instruction::InstructionInfo};

pub fn exa(vm: &mut Vm, _info: InstructionInfo) -> anyhow::Result<()> {
    let flag_count: u16 = vm.parser.read_param()?;
    let reg_count: u16 = vm.parser.read_param()?;

    // don't add by one, this incosistency with kidfn/hln/fln
    // originate from the original engine
    vm.save.init_flag(flag_count as usize)?;
    vm.save.init_register(reg_count as usize)?;

    Ok(())
}
