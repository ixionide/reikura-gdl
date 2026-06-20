use anyhow::bail;

use crate::instruction::Instruction;

pub struct Await;

impl Instruction for Await {
    fn execute(vm: &mut crate::Vm, _info: super::InstructionInfo) -> anyhow::Result<()> {
        match vm.ctx.wait_duration.take() {
            Some(duration) => vm.state.wait(duration),
            None => bail!("wait time is not set yet"),
        }

        Ok(())
    }
}
