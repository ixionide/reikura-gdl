use crate::instruction::{Instruction, ReadParam};

pub struct Js;

impl Instruction for Js {
    fn execute(vm: &mut crate::Vm) -> anyhow::Result<()> {
        let sub_index: u16 = vm.scene.param()?;
        vm.scene.call_sub(sub_index as _)
    }
}
