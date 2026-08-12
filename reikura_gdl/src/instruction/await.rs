use anyhow::bail;

use crate::{Vm, instruction::InstructionInfo};

pub fn r#await(vm: &mut Vm, _info: InstructionInfo) -> anyhow::Result<()> {
    match vm.ctx.wait_duration.take() {
        Some(duration) => vm.state.wait(duration),
        None => bail!("wait duration is not set yet"),
    }

    Ok(())
}
