use std::time::Duration;

use crate::{
    Vm,
    instruction::{InstructionInfo, Value},
};

pub fn atimes(vm: &mut Vm, _info: InstructionInfo) -> anyhow::Result<()> {
    let ms: u64 = vm
        .parser
        .read_param::<Value>()?
        .evaluate(&vm.ctx)
        .try_into()?;

    vm.ctx.wait_duration = Some(Duration::from_millis(ms));

    Ok(())
}
