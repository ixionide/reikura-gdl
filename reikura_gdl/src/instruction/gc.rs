use crate::instruction::{Evaluate, Instruction, ReadParam, Value};

pub struct Gc;

impl Instruction for Gc {
    fn execute(vm: &mut crate::Vm, _info: super::InstructionInfo) -> anyhow::Result<()> {
        let id: u8 = vm.scene.param::<Value>()?.evaluate(&vm.ctx).try_into()?;
        let [r, g, b]: [u8; 3] = vm.scene.param()?;

        vm.gfx.clear_image(id, (r, g, b))?;

        Ok(())
    }
}
