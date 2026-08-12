use crate::{SaveManager, Vm, instruction::InstructionInfo};

pub fn exa(vm: &mut Vm, _info: InstructionInfo) -> anyhow::Result<()> {
    let path = vm.manifest.game_path();
    let flag_count = vm.parser.read_param::<u16>()? as usize;
    let reg_count = vm.parser.read_param::<u16>()? as usize;

    vm.save = Some(SaveManager::new(path, flag_count, reg_count)?);

    Ok(())
}
