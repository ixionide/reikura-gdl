use anyhow::bail;
use reikura_util::const_iota;

use crate::{
    HotSpot, Vm,
    input::{MAX_HOTSPOTS, StateStorage},
    instruction::{InstructionInfo, Rect, Value},
};

const_iota! {
    u8 = iota,
    REGISTER,
    FLAG,
}

pub fn ih(vm: &mut Vm, _info: InstructionInfo) -> anyhow::Result<()> {
    let id: u8 = vm.parser.read_param()?;
    let rect: Rect<Value> = vm.parser.read_param()?;
    let state_storage: u8 = vm.parser.read_param::<u8>()?;
    let state_index = vm.parser.read_param::<u16>()? as usize;
    let _unknown: [u8; 3] = vm.parser.read_param()?;
    let index = id as usize % MAX_HOTSPOTS;

    let state_storage = match state_storage {
        REGISTER => StateStorage::Register,
        FLAG => StateStorage::Flag,
        _ => bail!("unknown hotspot state storage: {state_storage}"),
    };

    vm.input.hot_spots[index] = Some(HotSpot {
        id,
        rect: rect.evaluate(&vm.ctx).into(),
        state_storage,
        state_index,
        _unknown,
    });

    Ok(())
}
