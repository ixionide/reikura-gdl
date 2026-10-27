use std::time::{Duration, Instant};

use crate::{
    Vm,
    instruction::{InstructionInfo, Value},
};

pub fn atimes(vm: &mut Vm, _info: InstructionInfo) -> anyhow::Result<()> {
    let ms = vm.parser.read_param::<Value>()?.eval(&vm.ctx);
    let duration = Duration::from_millis(u64::try_from(ms)?);

    vm.wait_until = Some(Instant::now() + duration);

    Ok(())
}
