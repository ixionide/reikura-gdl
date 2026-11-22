use crate::{Vm, instruction::InstructionInfo};

pub fn ig(vm: &mut Vm, _info: InstructionInfo) -> anyhow::Result<()> {
    let hotspot_reg: u16 = vm.parser.read_param()?;
    let action_reg: u16 = vm.parser.read_param()?;
    let hotspot_count: u8 = vm.parser.read_param()?;
    let flags: u8 = vm.parser.read_param()?;

    let _flag1 = flags & 0b001 != 0;
    let _flag2 = flags & 0b010 != 0;
    let _flag3 = flags & 0b100 != 0;

    let hotspot = vm.input.get_active_hotspot(&vm.ctx, hotspot_count);
    let action = vm.input.get_action();

    vm.ctx.registers.set(hotspot_reg as usize, hotspot);
    vm.ctx.registers.set(action_reg as usize, action);

    Ok(())
}
