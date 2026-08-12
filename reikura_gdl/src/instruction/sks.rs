use anyhow::bail;

use crate::{Vm, instruction::InstructionInfo};

reikura_util::const_iota! {
    u8 = iota,
    UNSET,
    SET,
    TOGGLE,
}

pub fn sks(vm: &mut Vm, _info: InstructionInfo) -> anyhow::Result<()> {
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
