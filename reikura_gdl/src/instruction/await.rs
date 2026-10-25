use anyhow::bail;

use crate::{Vm, instruction::InstructionInfo};

pub fn r#await(vm: &mut Vm, _info: InstructionInfo) -> anyhow::Result<()> {
    match vm.ctx.wait_time.take() {
        Some(wait_time) => vm.state.wait(wait_time),
        None => bail!("wait duration is not set yet"),
    }

    Ok(())
}
