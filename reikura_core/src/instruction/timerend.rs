use crate::instruction::{Instruction, ReadParam, Value};

pub struct Timerend;

impl Instruction for Timerend {
    fn execute(vm: &mut crate::Vm, info: super::InstructionInfo) -> anyhow::Result<()> {
        if info.param_length() == 4 {
            _ = vm.scene.param::<Value>()?; // XXX: need more research
        }

        vm.ctx.timer = None;

        Ok(())
    }
}
