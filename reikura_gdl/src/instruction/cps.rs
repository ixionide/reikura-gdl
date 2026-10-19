use crate::{Vm, instruction::InstructionInfo};

#[allow(unused)]
pub fn cps(vm: &mut Vm, info: InstructionInfo) -> anyhow::Result<()> {
    // text message index (maybe??)
    let _: u8 = vm.parser.read_param()?;
    let col1: [u8; 3] = vm.parser.read_param()?;
    let col1: [u8; 3] = vm.parser.read_param()?;
    let col1: [u8; 3] = vm.parser.read_param()?;

    let col1: [u8; 3] = vm.parser.read_param()?;
    let col1: [u8; 3] = vm.parser.read_param()?;
    let col1: [u8; 3] = vm.parser.read_param()?;

    Ok(())
}
