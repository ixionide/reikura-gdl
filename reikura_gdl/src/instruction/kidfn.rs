use crate::{Vm, instruction::InstructionInfo};

pub fn kidfn(vm: &mut Vm, _info: InstructionInfo) -> anyhow::Result<()> {
    let msg_flag_count: u32 = vm.parser.read_param()?;

    vm.save.init_message_flag(msg_flag_count as usize + 1)?;

    Ok(())
}
