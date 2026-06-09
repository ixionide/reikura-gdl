use crate::instruction::Instruction;

pub struct Sret;

impl Instruction for Sret {
    fn execute(vm: &mut crate::Vm) -> anyhow::Result<()> {
        vm.scene.ret()
    }
}
