use std::io::Read;

use reikura_util::encoding::sjis_to_utf8;

use crate::instruction::Instruction;

pub struct Cns;

impl Instruction for Cns {
    fn execute(vm: &mut crate::Vm, info: super::InstructionInfo) -> anyhow::Result<()> {
        // window index (maybe??)
        let _: u8 = vm.parser.read_param()?;
        let index: u8 = vm.parser.read_param()?;
        let mut name_buf = vec![0; info.param_length() - 2];
        vm.parser.read_exact(&mut name_buf)?;
        let name = sjis_to_utf8(name_buf)?;

        vm.ctx.char_names[index as usize] = Some(name);

        Ok(())
    }
}
