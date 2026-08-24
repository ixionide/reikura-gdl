use anyhow::Result;
use reikura_util::{
    encoding::sjis_to_utf8,
    io::{ReadEndian, ReadExt},
};

use crate::{Parser, vm::VmContext};

pub use crate::AssetName;

#[rustfmt::skip]
pub const CHARSET: [[u8; 2]; 128] = [
    [0x81, 0x40], [0x81, 0x40], [0x81, 0x41], [0x81, 0x42], [0x81, 0x45], [0x81, 0x48], [0x81, 0x49], [0x81, 0x69],
    [0x81, 0x6A], [0x81, 0x75], [0x81, 0x76], [0x82, 0x4F], [0x82, 0x50], [0x82, 0x51], [0x82, 0x52], [0x82, 0x53],
    [0x82, 0x54], [0x82, 0x55], [0x82, 0x56], [0x82, 0x57], [0x82, 0x58], [0x82, 0xA0], [0x82, 0xA2], [0x82, 0xA4],
    [0x82, 0xA6], [0x82, 0xA8], [0x82, 0xA9], [0x82, 0xAA], [0x82, 0xAB], [0x82, 0xAC], [0x82, 0xAD], [0x82, 0xAE],
    [0x81, 0x40], [0x82, 0xB0], [0x82, 0xB1], [0x82, 0xB2], [0x82, 0xB3], [0x82, 0xB4], [0x82, 0xB5], [0x82, 0xB6],
    [0x82, 0xB7], [0x82, 0xB8], [0x82, 0xB9], [0x82, 0xBA], [0x82, 0xBB], [0x82, 0xBC], [0x82, 0xBD], [0x82, 0xBE],
    [0x82, 0xBF], [0x82, 0xC0], [0x82, 0xC1], [0x82, 0xC2], [0x82, 0xC3], [0x82, 0xC4], [0x82, 0xC5], [0x82, 0xC6],
    [0x82, 0xC7], [0x82, 0xC8], [0x82, 0xC9], [0x82, 0xCA], [0x82, 0xCB], [0x82, 0xCC], [0x82, 0xCD], [0x82, 0xCE],
    [0x82, 0xD0], [0x82, 0xD1], [0x82, 0xD3], [0x82, 0xD4], [0x82, 0xD6], [0x82, 0xD7], [0x82, 0xD9], [0x82, 0xDA],
    [0x82, 0xDC], [0x82, 0xDD], [0x82, 0xDE], [0x82, 0xDF], [0x82, 0xE0], [0x82, 0xE1], [0x82, 0xE2], [0x82, 0xE3],
    [0x82, 0xE4], [0x82, 0xE5], [0x82, 0xE6], [0x82, 0xE7], [0x82, 0xE8], [0x82, 0xE9], [0x82, 0xEA], [0x82, 0xEB],
    [0x82, 0xED], [0x82, 0xF0], [0x82, 0xF1], [0x83, 0x41], [0x83, 0x43], [0x83, 0x45], [0x83, 0x47], [0x83, 0x49],
    [0x83, 0x4A], [0x83, 0x4C], [0x83, 0x4E], [0x83, 0x50], [0x83, 0x52], [0x83, 0x54], [0x83, 0x56], [0x83, 0x58],
    [0x83, 0x5A], [0x83, 0x5C], [0x83, 0x5E], [0x83, 0x60], [0x83, 0x62], [0x83, 0x63], [0x83, 0x65], [0x83, 0x67],
    [0x83, 0x69], [0x83, 0x6A], [0x82, 0xAF], [0x83, 0x6C], [0x83, 0x6D], [0x83, 0x6E], [0x83, 0x71], [0x83, 0x74],
    [0x83, 0x77], [0x83, 0x7A], [0x83, 0x7D], [0x83, 0x7E], [0x83, 0x80], [0x83, 0x81], [0x83, 0x82], [0x83, 0x84],
];

pub trait Parameters: Sized {
    fn parse(parser: &mut Parser) -> Result<Self>;
}

impl<T: ReadEndian> Parameters for T {
    fn parse(parser: &mut Parser) -> Result<Self> {
        let param = parser.get_le::<T>()?;
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
    fn parse(parser: &mut Parser) -> Result<Self> {
        let value: i32 = parser.get_le()?;
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
    fn parse(parser: &mut Parser) -> Result<Self> {
        let mut buffer = Vec::with_capacity(32);

        loop {
            let byte: u8 = parser.get_le()?;

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
    fn parse(parser: &mut Parser) -> anyhow::Result<Self> {
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
