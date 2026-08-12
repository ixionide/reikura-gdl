use crate::{
    HotSpot, Vm,
    input::MAX_HOTSPOTS,
    instruction::{InstructionInfo, Rect, Value},
};

pub fn ih(vm: &mut Vm, _info: InstructionInfo) -> anyhow::Result<()> {
    let id: u8 = vm.parser.read_param()?;
    let rect: Rect<Value> = vm.parser.read_param()?;
    let flag = vm.parser.read_param::<u8>()? != 0;
    let state_index = vm.parser.read_param::<u16>()? as usize;
    let _unknown: [u8; 3] = vm.parser.read_param()?;
    let index = id as usize % MAX_HOTSPOTS;

    vm.input.hot_spots[index] = Some(HotSpot {
        id,
        rect: rect.evaluate(&vm.ctx).into(),
        flag,
        state_index,
        _unknown,
    });

    Ok(())
}
