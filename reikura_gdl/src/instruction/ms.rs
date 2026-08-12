use crate::{Vm, instruction::InstructionInfo};

pub fn ms(vm: &mut Vm, _info: InstructionInfo) -> anyhow::Result<()> {
    vm.audio.stop_bgm(None);

    Ok(())
}
