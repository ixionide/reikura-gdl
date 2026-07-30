use crate::instruction::Instruction;

pub struct Hinc;

impl Instruction for Hinc {
    fn execute(vm: &mut crate::Vm, _info: super::InstructionInfo) -> anyhow::Result<()> {
        let reg_index: u16 = vm.parser.read_param()?;

        vm.ctx.registers.inc(reg_index as usize);

        Ok(())
    }
}
