use crate::instruction::{Instruction, InstructionInfo};

pub struct Js;

impl Instruction for Js {
    fn execute(vm: &mut crate::Vm, _info: InstructionInfo) -> anyhow::Result<()> {
        let sub_index: u16 = vm.parser.read_param()?;

        vm.parser.call_sub(sub_index)?;

        Ok(())
    }
}
