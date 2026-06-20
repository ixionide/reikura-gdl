use crate::instruction::Instruction;

pub struct Pcmend;

impl Instruction for Pcmend {
    fn execute(vm: &mut crate::Vm, _info: super::InstructionInfo) -> anyhow::Result<()> {
        vm.state.wait_voice();

        Ok(())
    }
}
