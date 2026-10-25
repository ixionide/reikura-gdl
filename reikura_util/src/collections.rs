use std::io::{Error, ErrorKind, Read, Result, Write};
use std::marker::PhantomData;
use std::ops::Deref;

use crate::io::{ReadEndian, WriteEndian};

pub struct BitSet<B: AsRef<[u8]> + AsMut<[u8]> = Vec<u8>> {
    inner: B,
    count: usize,
}

impl BitSet<Vec<u8>> {
    pub fn new(bit_count: usize) -> Self {
        // let byte_count = (count - 1) / 8 + 1;
        // let byte_count = (count + 7) / 8;
        let byte_count = bit_count.div_ceil(8);

        Self {
            inner: vec![0; byte_count],
            count: bit_count,
        }
    }

    pub fn resize(&mut self, count: usize) {
        if count < self.count {
            return;
        }

        let byte_count = count.div_ceil(8);
        self.count = count;

        if byte_count > self.inner.len() {
            self.inner.resize(byte_count, 0);
        }
    }
}

impl<B: AsRef<[u8]> + AsMut<[u8]>> BitSet<B> {
    pub fn from_raw(bytes: B, bit_count: usize) -> Self {
        let byte_count = bit_count.div_ceil(8);

        assert!(byte_count == bytes.as_ref().len());

        Self {
            inner: bytes,
            count: bit_count,
        }
    }

    pub fn as_slice(&self) -> &[u8] {
        self.inner.as_ref()
    }

    pub fn len(&self) -> usize {
        self.count
    }

    pub fn is_empty(&self) -> bool {
        self.count == 0
    }

    pub fn get(&self, index: usize) -> Option<bool> {
        if index >= self.count {
            return None;
        }

        let byte_index = index / 8;
        let bitmask = 1 << (index & 7);
        let byte = self.inner.as_ref().get(byte_index)?;

        Some(byte & bitmask != 0)
    }

    pub fn set(&mut self, index: usize, value: bool) -> Option<bool> {
        if index >= self.count {
            return None;
        }

        let byte_index = index / 8;
        let bitmask = 1 << (index & 7);
        let byte = self.inner.as_mut().get_mut(byte_index)?;
        match value {
            true => *byte |= bitmask,
            false => *byte &= !bitmask,
        }

        Some(value)
    }

    pub fn toggle(&mut self, index: usize) -> Option<bool> {
        if index >= self.count {
            return None;
        }

        let byte_index = index / 8;
        let bitmask = 1 << (index & 7);
        let byte = self.inner.as_mut().get_mut(byte_index)?;
        *byte ^= bitmask;

        Some((*byte & bitmask) != 0)
    }
}

pub struct LenVec<L: Into<usize>, T>(Vec<T>, PhantomData<L>);

impl<L: Into<usize>, T> LenVec<L, T> {
    pub fn into_inner(self) -> Vec<T> {
        self.0
    }
}

impl<L: Into<usize>, T> Deref for LenVec<L, T> {
    type Target = Vec<T>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<I: Into<usize>, V> From<LenVec<I, V>> for Vec<V> {
    fn from(val: LenVec<I, V>) -> Self {
        val.0
    }
}

impl<I: Into<usize>, V> From<Vec<V>> for LenVec<I, V> {
    fn from(value: Vec<V>) -> Self {
        LenVec(value, PhantomData)
    }
}

impl<I: ReadEndian + Into<usize>, V: ReadEndian> ReadEndian for LenVec<I, V> {
    fn read_le<R: Read + ?Sized>(reader: &mut R) -> Result<Self> {
        let len = I::read_le(reader)?.into();
        let mut values = Vec::with_capacity(len);

        for _ in 0..len {
            let val = V::read_le(reader)?;
            values.push(val);
        }

        Ok(Self(values, PhantomData))
    }

    fn read_be<R: Read + ?Sized>(reader: &mut R) -> Result<Self> {
        let len = I::read_be(reader)?.into();
        let mut values = Vec::with_capacity(len);

        for _ in 0..len {
            let val = V::read_be(reader)?;
            values.push(val);
        }

        Ok(Self(values, PhantomData))
    }
}

impl<I: WriteEndian + TryFrom<usize> + Into<usize>, V: WriteEndian> WriteEndian for LenVec<I, V> {
    fn write_le<W: Write + ?Sized>(self, writer: &mut W) -> Result<()> {
        let len = I::try_from(self.0.len())
            .map_err(|_| Error::new(ErrorKind::InvalidInput, "length too large for index type"))?;
        I::write_le(len, writer)?;

        for val in self.0 {
            val.write_le(writer)?;
        }

        Ok(())
    }

    fn write_be<W: Write + ?Sized>(self, writer: &mut W) -> Result<()> {
        let len = I::try_from(self.0.len())
            .map_err(|_| Error::new(ErrorKind::InvalidInput, "length too large for index type"))?;
        I::write_be(len, writer)?;

        for val in self.0 {
            val.write_le(writer)?;
        }

        Ok(())
    }
}

impl<I: Into<usize>> TryInto<String> for LenVec<I, u8> {
    type Error = std::string::FromUtf8Error;

    fn try_into(self) -> std::result::Result<String, Self::Error> {
        String::from_utf8(self.0)
    }
}

type StackResult<T> = std::result::Result<T, StackError>;

#[derive(Debug, Clone, Copy)]
pub enum StackError {
    Overflow,
    Underflow,
}

impl std::fmt::Display for StackError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            StackError::Overflow => write!(f, "Stack overflow"),
            StackError::Underflow => write!(f, "Stack underflow"),
        }
    }
}

impl std::error::Error for StackError {}

pub struct Stack<T, const N: usize> {
    inner: Vec<T>,
}

impl<T, const N: usize> Default for Stack<T, N> {
    fn default() -> Self {
        Self::new()
    }
}

impl<T, const N: usize> Stack<T, N> {
    pub fn new() -> Self {
        Self {
            inner: Vec::with_capacity(N),
        }
    }

    pub fn cap(&self) -> usize {
        N
    }

    pub fn len(&self) -> usize {
        self.inner.len()
    }

    pub fn is_empty(&self) -> bool {
        self.inner.is_empty()
    }

    pub fn push(&mut self, value: T) -> StackResult<()> {
        if self.len() >= self.cap() {
            return Err(StackError::Overflow);
        }

        self.inner.push(value);

        Ok(())
    }

    pub fn pop(&mut self) -> StackResult<T> {
        self.inner.pop().ok_or(StackError::Underflow)
    }

    pub fn clear(&mut self) {
        self.inner.clear();
    }
}
