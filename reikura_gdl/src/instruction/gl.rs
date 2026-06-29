use crate::instruction::{AssetName, Evaluate, Instruction, ReadParam, Value};

pub struct Gl;

impl Instruction for Gl {
    fn execute(vm: &mut crate::Vm, _info: super::InstructionInfo) -> anyhow::Result<()> {
        let id: u8 = vm.scene.param::<Value>()?.evaluate(&vm.ctx).try_into()?;
        let image_name = vm.scene.param::<AssetName>()?.decode()?;
        let image = vm.assets.load_image(&image_name)?;

        vm.gfx
            .load_image(id, image.width, image.height, &image.data)?;

        Ok(())
    }
}
