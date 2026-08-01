use crate::instruction::Instruction;

pub struct Hdec;

impl Instruction for Hdec {
    fn execute(vm: &mut crate::Vm, _info: super::InstructionInfo) -> anyhow::Result<()> {
        let reg_index: u16 = vm.parser.read_param()?;

        vm.ctx.registers.dec(reg_index as usize);

        Ok(())
    }
}
