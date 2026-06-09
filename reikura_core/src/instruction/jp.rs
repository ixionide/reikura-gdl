use crate::instruction::{Instruction, ReadParam};

pub struct Jp;

impl Instruction for Jp {
    fn execute(vm: &mut crate::Vm) -> anyhow::Result<()> {
        let sub_index: u16 = vm.scene.param()?;
        vm.scene.jump_sub(sub_index as _)
    }
}
