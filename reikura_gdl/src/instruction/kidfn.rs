use anyhow::bail;

use crate::{Vm, instruction::InstructionInfo};

pub fn kidfn(vm: &mut Vm, _info: InstructionInfo) -> anyhow::Result<()> {
    let count: u32 = vm.parser.read_param()?;

    match &mut vm.save {
        Some(save) => save.init_message_flags(count as usize),
        None => bail!("save is not initialized yet"),
    }
}
