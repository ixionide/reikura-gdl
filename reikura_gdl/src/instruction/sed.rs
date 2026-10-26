use crate::{
    Vm,
    instruction::{InstructionInfo, Value},
};

pub fn sed(vm: &mut Vm, _info: InstructionInfo) -> anyhow::Result<()> {
    let slot = vm.parser.read_param::<Value>()?.eval(&vm.ctx) as usize;

    vm.audio.stop_sfx(slot, None);

    Ok(())
}
