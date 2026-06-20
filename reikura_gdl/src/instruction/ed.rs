use crate::instruction::Instruction;

pub struct Ed;

impl Instruction for Ed {
    fn execute(vm: &mut crate::Vm, _info: super::InstructionInfo) -> anyhow::Result<()> {
        vm.state.exit();

        Ok(())
    }
}
