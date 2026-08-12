use crate::{
    Vm,
    instruction::{InstructionInfo, Value},
};

pub fn ihkdef(vm: &mut Vm, _info: InstructionInfo) -> anyhow::Result<()> {
    let default = vm.parser.read_param::<Value>()?.evaluate(&vm.ctx);

    vm.input.default_key_map = Some(default.try_into()?);

    Ok(())
}
