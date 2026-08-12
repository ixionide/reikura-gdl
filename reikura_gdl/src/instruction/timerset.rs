use crate::{
    Vm,
    instruction::{InstructionInfo, Value},
    vm::Timer,
};

pub fn timerset(vm: &mut Vm, _info: InstructionInfo) -> anyhow::Result<()> {
    let delay = vm.parser.read_param::<Value>()?.evaluate(&vm.ctx);

    vm.ctx.timer = Some(Timer::new(delay));

    Ok(())
}
