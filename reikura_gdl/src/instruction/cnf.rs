use anyhow::anyhow;

use crate::instruction::{AssetName, Instruction, ReadParam};

pub struct Cnf;

impl Instruction for Cnf {
    fn execute(vm: &mut crate::Vm, _info: super::InstructionInfo) -> anyhow::Result<()> {
        let _: u8 = vm.scene.param()?;
        let name: AssetName = vm.scene.param()?;

        Err(anyhow!("Cnf is called: {}", name.decode()?))
    }
}
