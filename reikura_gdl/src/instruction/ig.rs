use crate::{Vm, instruction::InstructionInfo};

pub fn ig(vm: &mut Vm, _info: InstructionInfo) -> anyhow::Result<()> {
    let selected_reg: u16 = vm.parser.read_param()?;
    let state_reg: u16 = vm.parser.read_param()?;
    let hotspot_count: u8 = vm.parser.read_param()?;
    let flags: u8 = vm.parser.read_param()?;

    let _flag1 = flags & 0b001 != 0;
    let _flag2 = flags & 0b010 != 0;
    let _flag3 = flags & 0b100 != 0;

    let selected = vm.input.get_selected(hotspot_count);

    vm.ctx.registers.set(selected_reg as usize, selected);
    vm.ctx.registers.set(state_reg as usize, 0); //TODO

    Ok(())
}
