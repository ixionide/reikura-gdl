use crate::instruction::Instruction;

pub struct Ig;

impl Instruction for Ig {
    fn execute(vm: &mut crate::Vm, _info: super::InstructionInfo) -> anyhow::Result<()> {
        let selected_var: u16 = vm.parser.read_param()?;
        let state_var: u16 = vm.parser.read_param()?;
        let hotspot_count: u8 = vm.parser.read_param()?;
        let flags: u8 = vm.parser.read_param()?;

        let _flag1 = flags & 0b001 != 0;
        let _flag2 = flags & 0b010 != 0;
        let _flag3 = flags & 0b100 != 0;

        let selected = vm.input.get_selected(hotspot_count);

        vm.ctx.variables.set(selected_var as usize, selected);
        vm.ctx.variables.set(state_var as usize, 0); //TODO

        Ok(())
    }
}
