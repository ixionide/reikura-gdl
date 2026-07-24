use crate::{
    HitMask,
    instruction::{AssetName, Evaluate, Instruction, Value},
};

pub struct Ihgl;

impl Instruction for Ihgl {
    fn execute(vm: &mut crate::Vm, _info: super::InstructionInfo) -> anyhow::Result<()> {
        let image_name: AssetName = vm.parser.read_param()?;
        let x = vm.parser.read_param::<Value>()?.evaluate(&vm.ctx);
        let y = vm.parser.read_param::<Value>()?.evaluate(&vm.ctx);
        let image = vm.assets.load_image(image_name)?;

        vm.input.hit_mask = Some(HitMask { x, y, image });

        Ok(())
    }
}
