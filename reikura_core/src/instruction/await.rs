use std::time::Duration;

use anyhow::bail;

use crate::instruction::Instruction;

pub struct Await;

impl Instruction for Await {
    fn execute(vm: &mut crate::Vm, _info: super::InstructionInfo) -> anyhow::Result<()> {
        match vm.ctx.wait_time.take() {
            Some(ms) => {
                let ms: u64 = ms.try_into()?;
                vm.state.wait(Duration::from_millis(ms));
            }
            None => bail!("wait time is not set yet"),
        }

        Ok(())
    }
}
