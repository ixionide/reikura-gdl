use crate::instruction::Instruction;

pub struct Ms;

impl Instruction for Ms {
    fn execute(vm: &mut crate::Vm, _info: super::InstructionInfo) -> anyhow::Result<()> {
        vm.audio.stop_bgm(None);

        Ok(())
    }
}
