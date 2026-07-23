use crate::instruction::{Evaluate, Instruction, Value};

pub struct Hsg;

impl Instruction for Hsg {
    fn execute(vm: &mut crate::Vm, _info: super::InstructionInfo) -> anyhow::Result<()> {
        let start = vm.parser.read_param::<u16>()? as usize;
        let end = vm.parser.read_param::<u16>()? as usize;
        let var_value: Value = vm.parser.read_param()?;
        let bound_end = vm.ctx.variables.len() - 1;
        let range = start..=end.min(bound_end);

        if var_value.is_random() {
            for index in range {
                // generate random number every iteration
                let random_value = var_value.evaluate(&vm.ctx);
                vm.ctx.variables.set(index, random_value);
            }
        } else {
            // evaluate once
            let value = var_value.evaluate(&vm.ctx);
            for index in range {
                vm.ctx.variables.set(index, value);
            }
        }

        Ok(())
    }
}
