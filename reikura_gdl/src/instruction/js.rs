use crate::instruction::{Instruction, InstructionInfo, ReadParam};

pub struct Js;

impl Instruction for Js {
    fn execute(vm: &mut crate::Vm, _info: InstructionInfo) -> anyhow::Result<()> {
        let sub_index: u16 = vm.scene.param()?;

        vm.scene.call_sub(sub_index)
    }
}
