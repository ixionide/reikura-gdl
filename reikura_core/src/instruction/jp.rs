use crate::instruction::{Instruction, InstructionInfo, ReadParam};

pub struct Jp;

impl Instruction for Jp {
    fn execute(vm: &mut crate::Vm, _info: InstructionInfo) -> anyhow::Result<()> {
        let sub_index: u16 = vm.scene.param()?;

        vm.scene.jump_sub(sub_index)
    }
}
