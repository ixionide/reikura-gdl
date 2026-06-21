use crate::{
    HitMask,
    instruction::{AssetName, Evaluate, Instruction, ReadParam, Value},
};

pub struct Ihgl;

impl Instruction for Ihgl {
    fn execute(vm: &mut crate::Vm, _info: super::InstructionInfo) -> anyhow::Result<()> {
        let image_name: AssetName = vm.scene.param()?;
        let x = vm.scene.param::<Value>()?.evaluate(&vm.ctx);
        let y = vm.scene.param::<Value>()?.evaluate(&vm.ctx);
        let image = vm.assets.load_image(image_name.decode()?.as_str())?;

        vm.input.hit_mask = Some(HitMask { x, y, image });

        Ok(())
    }
}
