use crate::{
    Vm,
    instruction::{InstructionInfo, Value},
};

pub fn vset(vm: &mut Vm, _info: InstructionInfo) -> anyhow::Result<()> {
    let id: u8 = vm
        .parser
        .read_param::<Value>()?
        .evaluate(&vm.ctx)
        .try_into()?;
    let w: u32 = vm.parser.read_param::<Value>()?.evaluate(&vm.ctx) as u32;
    let h: u32 = vm.parser.read_param::<Value>()?.evaluate(&vm.ctx) as u32;

    vm.gfx.new_image(id, w, h)?;

    Ok(())
}
