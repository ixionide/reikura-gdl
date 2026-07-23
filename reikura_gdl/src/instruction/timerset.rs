use crate::{
    instruction::{Evaluate, Instruction, Value},
    vm::Timer,
};

pub struct Timerset;

impl Instruction for Timerset {
    fn execute(vm: &mut crate::Vm, _info: super::InstructionInfo) -> anyhow::Result<()> {
        let delay = vm.parser.read_param::<Value>()?.evaluate(&vm.ctx);

        vm.ctx.timer = Some(Timer::new(delay));

        Ok(())
    }
}
