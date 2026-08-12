use crate::{
    Vm,
    instruction::{AssetName, InstructionInfo},
};

pub fn cnf(vm: &mut Vm, _info: InstructionInfo) -> anyhow::Result<()> {
    let unknown: u8 = vm.parser.read_param()?;
    let name: AssetName = vm.parser.read_param()?;

    Err(anyhow::anyhow!("unknown: {unknown}, asset: {name}"))
}
