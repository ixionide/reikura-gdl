use crate::instruction::Instruction;

pub struct Rt;

impl Instruction for Rt {
    fn execute(vm: &mut crate::Vm) -> anyhow::Result<()> {
        vm.scene.ret_sub()
    }
}
