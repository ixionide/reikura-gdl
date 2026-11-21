use anyhow::bail;
use reikura_util::const_iota;

use crate::{
    Hotspot, Vm,
    input::{MAX_HOTSPOTS, StateStorage},
    instruction::{InstructionInfo, Value},
};

const_iota! {
    u8 = iota,
    FLAG,
    REGISTER,
}

pub fn ih(vm: &mut Vm, _info: InstructionInfo) -> anyhow::Result<()> {
    let id: u8 = vm.parser.read_param()?;
    let x = vm.parser.read_param::<Value>()?.eval(&vm.ctx);
    let y = vm.parser.read_param::<Value>()?.eval(&vm.ctx);
    let w = vm.parser.read_param::<Value>()?.eval(&vm.ctx);
    let h = vm.parser.read_param::<Value>()?.eval(&vm.ctx);
    let state_storage: u8 = vm.parser.read_param::<u8>()?;
    let state_index = vm.parser.read_param::<u16>()? as usize;
    let _unknown: [u8; 3] = vm.parser.read_param()?;
    let index = id as usize % MAX_HOTSPOTS;

    let state_storage = match state_storage {
        FLAG => StateStorage::Flag,
        REGISTER => StateStorage::Register,
        _ => bail!("unknown hotspot state storage: {state_storage}"),
    };

    vm.input.hotspots[index] = Some(Hotspot {
        id,
        rect: [x, y, w, h],
        state_storage,
        state_index,
        _unknown,
    });

    Ok(())
}
