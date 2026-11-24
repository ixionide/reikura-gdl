use crate::{
    Vm,
    instruction::{InstructionInfo, Value},
};

pub fn go(vm: &mut Vm, _info: InstructionInfo) -> anyhow::Result<()> {
    let duration_ms = vm.parser.read_param::<Value>()?.eval(&vm.ctx);
    let [r, g, b]: [u8; 3] = vm.parser.read_param()?;

    // if this is set to nonzero and no input event (click, mouse movement, keyboard)
    // is received during or after the transition. then wait until input received
    let _unknown: u8 = vm.parser.read_param()?;

    let duration_ms = u32::try_from(duration_ms)?;
    if duration_ms != 0 {
        vm.gfx.start_fade_in_color(r, g, b, duration_ms)?;
        vm.runtime.state.wait_transition();
    } else {
        vm.gfx.clear_image(None, r, g, b)?;
    }

    Ok(())
}
