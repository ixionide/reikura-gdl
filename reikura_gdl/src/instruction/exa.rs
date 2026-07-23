use crate::{SaveManager, instruction::Instruction};

pub struct Exa;

impl Instruction for Exa {
    fn execute(vm: &mut crate::Vm, _info: super::InstructionInfo) -> anyhow::Result<()> {
        let path = vm.manifest.game_path();
        let flag_count = vm.parser.read_param::<u16>()? as usize;
        let var_count = vm.parser.read_param::<u16>()? as usize;

        vm.save = Some(SaveManager::new(path, flag_count, var_count)?);

        Ok(())
    }
}
