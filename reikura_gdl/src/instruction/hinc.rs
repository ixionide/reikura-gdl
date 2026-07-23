use crate::instruction::Instruction;

pub struct Hinc;

impl Instruction for Hinc {
    fn execute(vm: &mut crate::Vm, _info: super::InstructionInfo) -> anyhow::Result<()> {
        let var_index: u16 = vm.parser.read_param()?;

        vm.ctx.variables.inc(var_index as usize);

        Ok(())
    }
}
