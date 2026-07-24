use std::{fmt::Display, hash::Hash};

use anyhow::Result;
use reikura_util::{encoding::sjis_to_utf8, io::ReadExt};

use crate::{
    Parser,
    instruction::{Evaluate, Parameters},
    vm::VmContext,
};

#[derive(Clone, Copy)]
pub struct InstructionInfo {
    val1: u8,
    val2: Option<u8>,
}

impl Parameters for InstructionInfo {
    fn deserialize(scene: &mut Parser) -> anyhow::Result<Self> {
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
    fn deserialize(scene: &mut Parser) -> Result<Self> {
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

pub const MAX_ASSETNAME_LEN: usize = 12;

#[derive(Debug, Clone, Copy)]
pub struct AssetName {
    buffer: [u8; MAX_ASSETNAME_LEN],
    len: usize,
}

impl AssetName {
    pub const fn from_str(name: &'static str) -> Self {
        let len = if name.len() < MAX_ASSETNAME_LEN {
            name.len()
        } else {
            MAX_ASSETNAME_LEN
        };

        let mut buffer = [0; MAX_ASSETNAME_LEN];
        let name_bytes = name.as_bytes();

        let mut i = 0;
        while i < len {
            buffer[i] = name_bytes[i];
            i += 1;
        }

        Self { buffer, len }
    }

    pub fn from_buffer(buffer: [u8; MAX_ASSETNAME_LEN]) -> Self {
        let mut end = MAX_ASSETNAME_LEN;
        let mut ext = None;

        for (i, &b) in buffer.iter().enumerate() {
            if b == 0 || b == 13 {
                end = i;
                break;
            }

            if b == b'.' {
                ext = Some(i);
            }
        }

        Self {
            buffer,
            len: ext.unwrap_or(end),
        }
    }

    #[inline]
    fn buffer(&self) -> &[u8] {
        &self.buffer[..self.len]
    }
}

impl Parameters for AssetName {
    fn deserialize(scene: &mut Parser) -> Result<Self> {
        let mut buffer = [0; MAX_ASSETNAME_LEN];
        let mut end = MAX_ASSETNAME_LEN;
        let mut ext = None;

        for (i, b) in buffer.iter_mut().enumerate() {
            let byte: u8 = scene.read_le()?;

            if !byte.is_ascii() || byte.is_ascii_control() {
                end = i;
                break;
            }

            if byte == b'.' {
                ext = Some(i);
            }

            *b = byte;
        }

        Ok(Self {
            buffer,
            len: ext.unwrap_or(end),
        })
    }
}

impl Eq for AssetName {}
impl PartialEq for AssetName {
    fn eq(&self, other: &Self) -> bool {
        let lhs = self.buffer().iter().map(u8::to_ascii_lowercase);
        let rhs = other.buffer().iter().map(u8::to_ascii_lowercase);
        lhs.eq(rhs)
    }
}

impl Display for AssetName {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&String::from_utf8_lossy(self.buffer()))
    }
}

impl Hash for AssetName {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        for b in self.buffer() {
            state.write_u8(b.to_ascii_lowercase());
        }
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
    fn deserialize(scene: &mut Parser) -> Result<Self> {
        let mut buffer = Vec::with_capacity(32);

        loop {
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
    fn deserialize(scene: &mut Parser) -> anyhow::Result<Self> {
        Ok(Self {
            x: scene.read_param()?,
            y: scene.read_param()?,
            w: scene.read_param()?,
            h: scene.read_param()?,
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
