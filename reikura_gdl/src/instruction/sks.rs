use anyhow::bail;

use crate::instruction::{Instruction, ReadParam};

const CMD_UNSET: u8 = 0;
const CMD_SET: u8 = 1;
const CMD_TOGGLE: u8 = 2;

pub struct Sks;

impl Instruction for Sks {
    fn execute(vm: &mut crate::Vm, _info: super::InstructionInfo) -> anyhow::Result<()> {
        let start = vm.scene.param::<u16>()? as usize;
        let end = vm.scene.param::<u16>()? as usize;
        let value: u8 = vm.scene.param()?;
        let bound_end = vm.ctx.flags.len() - 1;
        let range = start..end.min(bound_end);

        match value {
            CMD_UNSET => {
                for i in range {
                    vm.ctx.flags.set(i, false);
                }
            }
            CMD_SET => {
                for i in range {
                    vm.ctx.flags.set(i, true);
                }
            }
            CMD_TOGGLE => {
                for i in range {
                    vm.ctx.flags.toggle(i);
                }
            }
            unk => bail!("unrecognized flag value: {unk}"),
        };

        Ok(())
    }
}
