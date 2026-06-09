use anyhow::Result;
use reikura_util::io::{ReadEndian, ReadExt};

use crate::{Scenario, Vm, vm::VmContext};

pub trait Instruction {
    fn execute(vm: &mut Vm) -> Result<()> {
        let _ = vm;
        Ok(())
    }
}

pub trait Evaluate {
    type Evaluated;
    fn evaluate(&self, ctx: &VmContext) -> Self::Evaluated;
}

pub trait Parameters: Sized {
    fn deserialize(scene: &mut Scenario) -> anyhow::Result<Self>;
}

impl<T: ReadEndian> Parameters for T {
    fn deserialize(scene: &mut Scenario) -> anyhow::Result<Self> {
        let param = scene.read_le::<T>()?;
        Ok(param)
    }
}

pub trait ReadParam {
    fn param<P: Parameters>(&mut self) -> anyhow::Result<P>;
}

impl ReadParam for Scenario {
    fn param<P: Parameters>(&mut self) -> anyhow::Result<P> {
        Parameters::deserialize(self)
    }
}
