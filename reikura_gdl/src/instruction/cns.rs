use reikura_util::encoding::sjis_to_utf8;

use crate::{Vm, instruction::InstructionInfo};

pub fn cns(vm: &mut Vm, info: InstructionInfo) -> anyhow::Result<()> {
    // window index (maybe??)
    let _: u8 = vm.parser.read_param()?;
    let index: u8 = vm.parser.read_param()?;
    let name_bytes = vm.parser.read_bytes(info.param_len - 2)?;
    let name = sjis_to_utf8(&name_bytes)?;

    vm.ctx.char_names[index as usize] = Some(name);

    Ok(())
}
