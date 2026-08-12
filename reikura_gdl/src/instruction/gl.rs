use crate::{
    Vm,
    instruction::{AssetName, InstructionInfo, Value},
};

pub fn gl(vm: &mut Vm, _info: InstructionInfo) -> anyhow::Result<()> {
    let id: u8 = vm
        .parser
        .read_param::<Value>()?
        .evaluate(&vm.ctx)
        .try_into()?;
    let image_name = vm.parser.read_param::<AssetName>()?;
    let image = vm.assets.load_image(image_name)?;

    vm.gfx
        .load_image(id, image.width, image.height, &image.data)?;

    Ok(())
}
