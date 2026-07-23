use crate::instruction::{AssetName, Evaluate, Instruction, Value};

pub struct Gl;

impl Instruction for Gl {
    fn execute(vm: &mut crate::Vm, _info: super::InstructionInfo) -> anyhow::Result<()> {
        let id: u8 = vm
            .parser
            .read_param::<Value>()?
            .evaluate(&vm.ctx)
            .try_into()?;
        let image_name = vm.parser.read_param::<AssetName>()?.decode()?;
        let image = vm.assets.load_image(&image_name)?;

        vm.gfx
            .load_image(id, image.width, image.height, &image.data)?;

        Ok(())
    }
}
