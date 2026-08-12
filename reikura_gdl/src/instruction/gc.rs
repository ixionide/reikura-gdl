use crate::{
    Vm,
    instruction::{InstructionInfo, Value},
};

pub fn gc(vm: &mut Vm, _info: InstructionInfo) -> anyhow::Result<()> {
    let id: u8 = vm
        .parser
        .read_param::<Value>()?
        .evaluate(&vm.ctx)
        .try_into()?;
    let [r, g, b]: [u8; 3] = vm.parser.read_param()?;

    vm.gfx.clear_image(id, (r, g, b))?;

    Ok(())
}
