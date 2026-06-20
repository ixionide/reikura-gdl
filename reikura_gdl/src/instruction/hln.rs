use crate::instruction::{Instruction, ReadParam};

pub struct Hln;

impl Instruction for Hln {
    fn execute(vm: &mut crate::Vm, _info: super::InstructionInfo) -> anyhow::Result<()> {
        let var_count: u16 = vm.scene.param()?;

        vm.ctx.variables.resize(var_count as usize);

        Ok(())
    }
}
