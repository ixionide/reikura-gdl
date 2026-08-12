use anyhow::Result;
use reikura_util::{
    encoding::sjis_to_utf8,
    io::{ReadEndian, ReadExt},
};

use crate::{Parser, vm::VmContext};

pub use crate::AssetName;

pub trait Parameters: Sized {
    fn deserialize(parser: &mut Parser) -> Result<Self>;
}

impl<T: ReadEndian> Parameters for T {
    fn deserialize(parser: &mut Parser) -> Result<Self> {
        let param = parser.read_le::<T>()?;
        Ok(param)
    }
}

#[derive(Clone, Copy)]
pub enum Value {
    Literal(i32),
    Register(i32),
    Random(i32),
}

impl Value {
    const BIT_MASK: i32 = (1 << 30) - 1; // signed 30bit integer
    const MIN_MASK: i32 = !Self::BIT_MASK;
    const REG_FLAG: i32 = 1 << 31;
    const RNG_FLAG: i32 = 1 << 30;
    const MIN_FLAG: i32 = 1 << 29;

    pub fn is_random(&self) -> bool {
        matches!(self, Value::Random(_))
    }

    pub fn evaluate(&self, ctx: &VmContext) -> i32 {
        match *self {
            Value::Literal(value) => value,
            Value::Register(index) => match index.try_into() {
                Ok(index) => ctx.registers.get(index).unwrap_or(0),
                Err(_) => 0,
            },
            Value::Random(modulo) => {
                if modulo == 0 {
                    return 0;
                }

                let random_number = fastrand::i32(0..modulo.abs());
                random_number * modulo.signum()
            }
        }
    }
}

impl Parameters for Value {
    fn deserialize(parser: &mut Parser) -> Result<Self> {
        let value: i32 = parser.read_le()?;
        let mut val = value & Self::BIT_MASK;

        if value & Self::MIN_FLAG != 0 {
            val |= Self::MIN_MASK;
        }

        let result = {
            if value & Self::REG_FLAG != 0 {
                Self::Register(val)
            } else if value & Self::RNG_FLAG != 0 {
                Self::Random(val)
            } else {
                Self::Literal(val)
            }
        };

        Ok(result)
    }
}

pub struct ParamString {
    buffer: Vec<u8>,
}

impl ParamString {
    pub fn decode_sjis(self) -> Result<String> {
        let string = sjis_to_utf8(&self.buffer)?;
        Ok(string)
    }
}

impl Parameters for ParamString {
    fn deserialize(parser: &mut Parser) -> Result<Self> {
        let mut buffer = Vec::with_capacity(32);

        loop {
            let byte: u8 = parser.read_le()?;

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

impl Rect<Value> {
    pub fn evaluate(&self, ctx: &VmContext) -> Rect<i32> {
        Rect {
            x: self.x.evaluate(ctx),
            y: self.y.evaluate(ctx),
            w: self.w.evaluate(ctx),
            h: self.h.evaluate(ctx),
        }
    }
}

impl<T: Parameters> Parameters for Rect<T> {
    fn deserialize(parser: &mut Parser) -> anyhow::Result<Self> {
        Ok(Self {
            x: parser.read_param()?,
            y: parser.read_param()?,
            w: parser.read_param()?,
            h: parser.read_param()?,
        })
    }
}

impl<T> From<Rect<T>> for [T; 4] {
    fn from(rect: Rect<T>) -> Self {
        [rect.x, rect.y, rect.w, rect.h]
    }
}
