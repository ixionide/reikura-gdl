use crate::{
    KeyMap,
    input::MAX_HOTSPOTS,
    instruction::{Evaluate, Instruction, ReadParam, Rect, Value},
};

pub struct Ihk;

impl Instruction for Ihk {
    fn execute(vm: &mut crate::Vm, _info: super::InstructionInfo) -> anyhow::Result<()> {
        let id: u8 = vm.scene.param()?;
        let index = id as usize % MAX_HOTSPOTS;
        let map1 = vm.scene.param::<Rect<Value>>()?.evaluate(&vm.ctx);
        let map2 = vm.scene.param::<Rect<Value>>()?.evaluate(&vm.ctx);
        let map: [u8; 8] = [
            map1.x.try_into()?,
            map1.y.try_into()?,
            map1.w.try_into()?,
            map1.h.try_into()?,
            map2.x.try_into()?,
            map2.y.try_into()?,
            map2.w.try_into()?,
            map2.h.try_into()?,
        ];

        vm.input.key_maps[index] = Some(KeyMap { id, map });

        Ok(())
    }
}
