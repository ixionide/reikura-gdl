use anyhow::bail;

use crate::instruction::Instruction;

pub struct Kidfn;

impl Instruction for Kidfn {
    fn execute(vm: &mut crate::Vm, _info: super::InstructionInfo) -> anyhow::Result<()> {
        let count: u32 = vm.parser.read_param()?;

        match &mut vm.save {
            Some(save) => save.init_read_flags(count as usize),
            None => bail!("save is not initialized yet"),
        }
    }
}
