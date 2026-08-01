use crate::instruction::Instruction;

pub struct Hln;

impl Instruction for Hln {
    fn execute(vm: &mut crate::Vm, _info: super::InstructionInfo) -> anyhow::Result<()> {
        let reg_count: u16 = vm.parser.read_param()?;

        vm.ctx.registers.resize(reg_count as usize);

        Ok(())
    }
}
