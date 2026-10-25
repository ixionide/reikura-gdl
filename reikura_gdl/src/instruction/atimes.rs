use std::time::{Duration, Instant};

use crate::{
    Vm,
    instruction::{InstructionInfo, Value},
};

pub fn atimes(vm: &mut Vm, _info: InstructionInfo) -> anyhow::Result<()> {
    let wait_time: u64 = vm
        .parser
        .read_param::<Value>()?
        .evaluate(&vm.ctx)
        .try_into()?;

    vm.ctx.wait_time = Some(Instant::now() + Duration::from_millis(wait_time));

    Ok(())
}
