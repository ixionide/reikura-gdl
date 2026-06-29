use crate::instruction::{Evaluate, Instruction, ReadParam, Value};

pub struct Vset;

impl Instruction for Vset {
    fn execute(vm: &mut crate::Vm, _info: super::InstructionInfo) -> anyhow::Result<()> {
        let id: u8 = vm.scene.param::<Value>()?.evaluate(&vm.ctx).try_into()?;
        let w: u32 = vm.scene.param::<Value>()?.evaluate(&vm.ctx) as u32;
        let h: u32 = vm.scene.param::<Value>()?.evaluate(&vm.ctx) as u32;

        vm.gfx.new_image(id, w, h)?;

        Ok(())
    }
}
