use anyhow::bail;

use crate::{Vm, instruction::InstructionInfo};

pub fn ec(vm: &mut Vm, _info: InstructionInfo) -> anyhow::Result<()> {
    let Some(save_flags) = &mut vm.save.flags else {
        bail!("loading flag failed");
    };
    let vm_flags = &mut vm.ctx.flags;

    let start = vm.parser.read_param::<u16>()? as usize;
    let end = vm.parser.read_param::<u16>()? as usize;
    let bound_end = vm_flags.len() - 1;

    let save_range = 0..save_flags.len();
    let vm_range = start..=end.min(bound_end);

    for (save_pos, vm_pos) in save_range.zip(vm_range) {
        let flag = save_flags.get(save_pos);
        vm_flags.set(vm_pos, flag);
    }

    Ok(())
}
