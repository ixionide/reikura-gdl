use crate::{
    Keymap, Vm,
    input::MAX_HOTSPOTS,
    instruction::{InstructionInfo, Value},
};

pub fn ihk(vm: &mut Vm, _info: InstructionInfo) -> anyhow::Result<()> {
    let id: u8 = vm.parser.read_param()?;
    let index = id as usize % MAX_HOTSPOTS;

    let mut map: [u8; 8] = [0; 8];
    for m in &mut map {
        *m = vm.parser.read_param::<Value>()?.eval(&vm.ctx).try_into()?;
    }

    vm.input.keymaps[index] = Some(Keymap { id, map });

    Ok(())
}
