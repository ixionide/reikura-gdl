use std::time::Duration;

use crate::{
    Vm,
    instruction::{InstructionInfo, Value},
};

pub fn atimes(vm: &mut Vm, _info: InstructionInfo) -> anyhow::Result<()> {
    let ms = vm.parser.read_param::<Value>()?.eval(&vm.ctx);
    let duration = Duration::from_millis(u64::try_from(ms)?);

    vm.runtime.set_wait_time(duration);

    Ok(())
}
