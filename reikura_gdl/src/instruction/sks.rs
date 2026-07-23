use anyhow::bail;

use crate::instruction::Instruction;

reikura_util::const_iota! {
    u8 = iota,
    UNSET,
    SET,
    TOGGLE,
}

pub struct Sks;

impl Instruction for Sks {
    fn execute(vm: &mut crate::Vm, _info: super::InstructionInfo) -> anyhow::Result<()> {
        let start = vm.parser.read_param::<u16>()? as usize;
        let end = vm.parser.read_param::<u16>()? as usize;
        let value: u8 = vm.parser.read_param()?;
        let bound_end = vm.ctx.flags.len() - 1;
        let range = start..end.min(bound_end);

        match value {
            UNSET => {
                for i in range {
                    vm.ctx.flags.set(i, false);
                }
            }
            SET => {
                for i in range {
                    vm.ctx.flags.set(i, true);
                }
            }
            TOGGLE => {
                for i in range {
                    vm.ctx.flags.toggle(i);
                }
            }
            unk => bail!("unrecognized flag value: {unk}"),
        };

        Ok(())
    }
}
