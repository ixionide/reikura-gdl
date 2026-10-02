use std::{array, mem::MaybeUninit};

use std::io::{Error, ErrorKind, Read, Result, Write};
use std::marker::PhantomData;
use std::ops::Deref;

use crate::io::{ReadEndian, WriteEndian};

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
    len: usize,
    items: [MaybeUninit<T>; N],
}

impl<T, const N: usize> Default for Stack<T, N> {
    fn default() -> Self {
        Self::new()
    }
}

impl<T, const N: usize> Stack<T, N> {
    pub fn new() -> Self {
        Self {
            len: 0,
            items: array::from_fn(|_| MaybeUninit::uninit()),
        }
    }

    pub fn cap(&self) -> usize {
        N
    }

    pub fn len(&self) -> usize {
        self.len
    }

    pub fn is_empty(&self) -> bool {
        self.len == 0
    }

    pub fn push(&mut self, value: T) -> StackResult<()> {
        if self.len() >= self.cap() {
            return Err(StackError::Overflow);
        }

        self.items[self.len].write(value);
        self.len += 1;

        Ok(())
    }

    pub fn pop(&mut self) -> StackResult<T> {
        if self.len == 0 {
            return Err(StackError::Underflow);
        }

        self.len -= 1;

        unsafe { Ok(self.items[self.len].assume_init_read()) }
    }

    pub fn clear(&mut self) {
        let ptr = self.items.as_mut_ptr() as *mut T;
        let len = self.len();

        self.len = 0;

        unsafe {
            let items = std::slice::from_raw_parts_mut(ptr, len);
            std::ptr::drop_in_place(items);
        }
    }
}

impl<T, const N: usize> Drop for Stack<T, N> {
    fn drop(&mut self) {
        self.clear();
    }
}
