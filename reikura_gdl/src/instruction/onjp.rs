use reikura_util::index_vec::IndexVec;

use crate::instruction::{Evaluate, Instruction, InstructionInfo, ReadParam, Value};

pub struct Onjp;

impl Instruction for Onjp {
    fn execute(vm: &mut crate::Vm, _info: InstructionInfo) -> anyhow::Result<()> {
        let switch: Value = vm.scene.param()?;
        let branches: IndexVec<u8, u16> = vm.scene.param()?;
        let index = switch.evaluate(&vm.ctx) as usize;

        if let Some(sub_index) = branches.get(index).copied() {
            vm.scene.jump_sub(sub_index)?;
        }

        Ok(())
    }
}
