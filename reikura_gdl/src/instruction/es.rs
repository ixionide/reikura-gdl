use anyhow::bail;

use crate::{Vm, instruction::InstructionInfo};

pub fn es(vm: &mut Vm, _info: InstructionInfo) -> anyhow::Result<()> {
    let Some(save_flags) = &mut vm.save.flags else {
        bail!("saving flag failed");
    };
    let vm_flags = &mut vm.ctx.flags;

    let start = vm.parser.read_param::<u16>()? as usize;
    let end = vm.parser.read_param::<u16>()? as usize;
    let bound_end = vm_flags.len() - 1;

    let vm_range = start..=end.min(bound_end);
    let save_range = 0..save_flags.len();

    for (vm_pos, save_pos) in vm_range.zip(save_range) {
        let flag = vm_flags.get(vm_pos);
        save_flags.set(save_pos, flag);
    }

    Ok(())
}
