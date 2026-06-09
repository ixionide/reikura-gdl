use anyhow::Result;
use reikura_util::{encoding::decode_sjis, io::ReadExt};

use crate::{
    Scenario,
    instruction::{Evaluate, Parameters},
    vm::VmContext,
};

#[derive(Clone, Copy)]
pub enum Value {
    Literal(i32),
    Variable(i32),
    Random(i32),
}

impl Value {
    const VAR_BIT: i32 = 1 << 31;
    const RAND_BIT: i32 = 1 << 30;
    const MIN_BIT: i32 = 1 << 29;
    const MASK: i32 = !(Self::VAR_BIT | Self::RAND_BIT); // signed 30bit integer

    pub fn is_random(&self) -> bool {
        matches!(self, Value::Random(_))
    }
}

impl Parameters for Value {
    fn deserialize(scene: &mut Scenario) -> Result<Self> {
        let value: i32 = scene.read_le()?;
        let mut val = value & Self::MASK;

        if value & Self::MIN_BIT != 0 {
            val |= !Self::MASK;
        }

        let result = {
            if value & Self::VAR_BIT != 0 {
                Self::Variable(val)
            } else if value & Self::RAND_BIT != 0 {
                Self::Random(val)
            } else {
                Self::Literal(val)
            }
        };

        Ok(result)
    }
}

impl Evaluate for Value {
    type Evaluated = i32;

    fn evaluate(&self, ctx: &VmContext) -> Self::Evaluated {
        match *self {
            Value::Literal(value) => value,
            Value::Variable(index) => match index.try_into() {
                Ok(index) => ctx.variables.get(index).unwrap_or(0),
                Err(_) => 0,
            },
            Value::Random(modulo) => {
                let random_number = fastrand::i32(0..modulo.abs().max(1));
                random_number * modulo.signum()
            }
        }
    }
}

pub type AssetName = ParamString<12>;

pub struct ParamString<const CAP: usize = 32> {
    buffer: Vec<u8>,
}

impl<const CAP: usize> ParamString<CAP> {
    pub fn decode(self) -> Result<String> {
        let string = decode_sjis(self.buffer)?;
        Ok(string)
    }
}

impl<const CAP: usize> Parameters for ParamString<CAP> {
    fn deserialize(scene: &mut Scenario) -> Result<Self> {
        let mut buffer = Vec::with_capacity(CAP);

        for _ in 0..CAP {
            let byte: u8 = scene.read_le()?;

            if byte == 0 || byte == 13 {
                break;
            }

            buffer.push(byte);
        }

        Ok(Self { buffer })
    }
}
