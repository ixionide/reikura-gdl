use crate::instruction::Instruction;

pub struct Fln;

impl Instruction for Fln {
    fn execute(vm: &mut crate::Vm, _info: super::InstructionInfo) -> anyhow::Result<()> {
        let flag_count: u16 = vm.parser.read_param()?;

        vm.ctx.flags.resize(flag_count as usize);

        Ok(())
    }
}
