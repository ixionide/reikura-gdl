use crate::instruction::{Instruction, ReadParam};

pub struct Fln;

impl Instruction for Fln {
    fn execute(vm: &mut crate::Vm, _info: super::InstructionInfo) -> anyhow::Result<()> {
        let flag_count: u16 = vm.scene.param()?;

        vm.ctx.flags.resize(flag_count as usize);

        Ok(())
    }
}
