use std::time::Duration;

use crate::{
    Vm,
    instruction::{InstructionInfo, Value},
};

pub fn timerset(vm: &mut Vm, _info: InstructionInfo) -> anyhow::Result<()> {
    let ms = vm.parser.read_param::<Value>()?.eval(&vm.ctx);
    let start = Duration::from_millis(u64::try_from(ms)?);

    vm.runtime.start_timer(start);

    Ok(())
}
