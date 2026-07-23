use crate::instruction::{Instruction, Value};

pub struct Timerend;

impl Instruction for Timerend {
    fn execute(vm: &mut crate::Vm, info: super::InstructionInfo) -> anyhow::Result<()> {
        if info.param_length() == 4 {
            _ = vm.parser.read_param::<Value>()?; // XXX: need more research
        }

        vm.ctx.timer = None;

        Ok(())
    }
}
