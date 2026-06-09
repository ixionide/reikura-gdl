use anyhow::bail;

use crate::instruction::{Instruction, ReadParam};

pub struct Sks;

impl Instruction for Sks {
    fn execute(vm: &mut crate::Vm, _info: super::InstructionInfo) -> anyhow::Result<()> {
        let start = vm.scene.param::<u16>()? as usize;
        let end = vm.scene.param::<u16>()? as usize;
        let value: u8 = vm.scene.param()?;
        let bound_end = vm.ctx.flags.len() - 1;
        let range = start..end.min(bound_end);

        match value {
            0 => {
                for i in range {
                    vm.ctx.flags.set(i, false);
                }
            }
            1 => {
                for i in range {
                    vm.ctx.flags.set(i, true);
                }
            }
            2 => {
                for i in range {
                    vm.ctx.flags.toggle(i);
                }
            }
            unk => bail!("unrecognized flag value: {unk}"),
        };

        Ok(())
    }
}
