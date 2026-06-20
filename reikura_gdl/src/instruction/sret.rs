use crate::instruction::{Instruction, InstructionInfo};

pub struct Sret;

impl Instruction for Sret {
    fn execute(vm: &mut crate::Vm, _info: InstructionInfo) -> anyhow::Result<()> {
        vm.scene.ret()?;

        Ok(())
    }
}
