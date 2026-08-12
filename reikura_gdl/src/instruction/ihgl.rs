use crate::{
    HitMask, Vm,
    instruction::{AssetName, InstructionInfo, Value},
};

pub fn ihgl(vm: &mut Vm, _info: InstructionInfo) -> anyhow::Result<()> {
    let image_name: AssetName = vm.parser.read_param()?;
    let x = vm.parser.read_param::<Value>()?.evaluate(&vm.ctx);
    let y = vm.parser.read_param::<Value>()?.evaluate(&vm.ctx);
    let image = vm.assets.load_image(image_name)?;

    vm.input.hit_mask = Some(HitMask { x, y, image });

    Ok(())
}
