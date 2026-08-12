use crate::{
    Vm,
    instruction::{InstructionInfo, Value},
};

pub fn timerend(vm: &mut Vm, info: InstructionInfo) -> anyhow::Result<()> {
    if info.param_len == 4 {
        _ = vm.parser.read_param::<Value>()?; // XXX: need more research
    }

    vm.ctx.timer = None;

    Ok(())
}
