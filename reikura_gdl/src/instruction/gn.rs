use crate::{
    Vm,
    instruction::{InstructionInfo, Value},
};

pub fn gn(vm: &mut Vm, _info: InstructionInfo) -> anyhow::Result<()> {
    let id: u8 = vm
        .parser
        .read_param::<Value>()?
        .evaluate(&vm.ctx)
        .try_into()?;
    let x: u32 = vm.parser.read_param::<Value>()?.evaluate(&vm.ctx) as u32;
    let y: u32 = vm.parser.read_param::<Value>()?.evaluate(&vm.ctx) as u32;

    vm.gfx.set_target(id, x, y);

    Ok(())
}
