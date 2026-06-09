use reikura_util::index_vec::IndexVec;

use crate::instruction::{Evaluate, Instruction, ReadParam, Value};

pub struct Onjs;

impl Instruction for Onjs {
    fn execute(vm: &mut crate::Vm) -> anyhow::Result<()> {
        let switch: Value = vm.scene.param()?;
        let branches: IndexVec<u8, u16> = vm.scene.param()?;
        let index = switch.evaluate(&vm.ctx) as usize;

        if let Some(sub_index) = branches.get(index).copied() {
            vm.scene.call_sub(sub_index)?
        }

        Ok(())
    }
}
