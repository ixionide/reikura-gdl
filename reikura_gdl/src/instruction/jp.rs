use crate::instruction::InstructionInfo;

pub fn jp(vm: &mut crate::Vm, _info: InstructionInfo) -> anyhow::Result<()> {
    let sub_index: u16 = vm.parser.read_param()?;

    vm.parser.jump_sub(sub_index)?;

    Ok(())
}
