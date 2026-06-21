use anyhow::Result;
use reikura_util::{encoding::sjis_to_utf8, io::ReadExt};

use crate::{
    Scenario,
    instruction::{Evaluate, Parameters, ReadParam},
    vm::VmContext,
};

#[derive(Clone, Copy)]
pub struct InstructionInfo {
    val1: u8,
    val2: Option<u8>,
}

impl Parameters for InstructionInfo {
    fn deserialize(scene: &mut Scenario) -> anyhow::Result<Self> {
        let val1 = scene.read_le()?;
        let mut val2 = None;

        if val1 & 0x80 != 0 {
            val2 = Some(scene.read_le()?);
        }

        Ok(Self { val1, val2 })
    }
}

impl InstructionInfo {
    pub fn length(&self) -> usize {
        let val1 = (self.val1 & 0x7F) as usize;
        match self.val2 {
            Some(val2) => (val1 << 8 | val2 as usize).max(3),
            None => val1.max(2),
        }
    }

    pub fn param_offset(&self) -> usize {
        match self.val2 {
            Some(_) => 3,
            None => 2,
        }
    }

    pub fn param_length(&self) -> usize {
        self.length() - self.param_offset()
    }

    pub fn end_of_scenario(&self) -> bool {
        self.val1 == 0 && self.val2.is_none()
    }
}

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
        let string = sjis_to_utf8(self.buffer)?;
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

pub struct Rect<T> {
    pub x: T,
    pub y: T,
    pub w: T,
    pub h: T,
}

impl<T: Parameters> Parameters for Rect<T> {
    fn deserialize(scene: &mut Scenario) -> anyhow::Result<Self> {
        Ok(Self {
            x: scene.param()?,
            y: scene.param()?,
            w: scene.param()?,
            h: scene.param()?,
        })
    }
}

impl<T> From<Rect<T>> for [T; 4] {
    fn from(rect: Rect<T>) -> Self {
        [rect.x, rect.y, rect.w, rect.h]
    }
}

impl<T: Evaluate> Evaluate for Rect<T> {
    type Evaluated = Rect<T::Evaluated>;

    fn evaluate(&self, ctx: &VmContext) -> Self::Evaluated {
        Self::Evaluated {
            x: self.x.evaluate(ctx),
            y: self.y.evaluate(ctx),
            w: self.w.evaluate(ctx),
            h: self.h.evaluate(ctx),
        }
    }
}
