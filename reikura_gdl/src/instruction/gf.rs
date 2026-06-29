use crate::instruction::Instruction;

pub struct Gf;

impl Instruction for Gf {
    fn execute(vm: &mut crate::Vm, _info: super::InstructionInfo) -> anyhow::Result<()> {
        vm.gfx.unset_target();

        Ok(())
    }
}
