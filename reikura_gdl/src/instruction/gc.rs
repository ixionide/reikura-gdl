use crate::instruction::{Instruction, Value};

pub struct Gc;

impl Instruction for Gc {
    fn execute(vm: &mut crate::Vm, _info: super::InstructionInfo) -> anyhow::Result<()> {
        let id: u8 = vm
            .parser
            .read_param::<Value>()?
            .evaluate(&vm.ctx)
            .try_into()?;
        let [r, g, b]: [u8; 3] = vm.parser.read_param()?;

        vm.gfx.clear_image(id, (r, g, b))?;

        Ok(())
    }
}
