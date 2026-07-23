use crate::instruction::{Instruction, InstructionInfo};

pub struct Rt;

impl Instruction for Rt {
    fn execute(vm: &mut crate::Vm, _info: InstructionInfo) -> anyhow::Result<()> {
        vm.parser.ret_sub()?;

        Ok(())
    }
}
