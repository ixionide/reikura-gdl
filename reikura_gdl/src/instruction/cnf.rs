use anyhow::anyhow;

use crate::instruction::{AssetName, Instruction};

pub struct Cnf;

impl Instruction for Cnf {
    fn execute(vm: &mut crate::Vm, _info: super::InstructionInfo) -> anyhow::Result<()> {
        let _: u8 = vm.parser.read_param()?;
        let name: AssetName = vm.parser.read_param()?;

        Err(anyhow!("CNF is called: {}", name.decode()?))
    }
}
