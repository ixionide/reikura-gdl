use crate::instruction::Instruction;

pub struct Ihgc;

impl Instruction for Ihgc {
    fn execute(vm: &mut crate::Vm, _info: super::InstructionInfo) -> anyhow::Result<()> {
        vm.input.hit_mask = None;

        Ok(())
    }
}
