use std::io::{Error, ErrorKind, Read, Result, Write};
use std::marker::PhantomData;
use std::ops::Deref;

use crate::io::{ReadEndian, WriteEndian};

pub struct IndexVec<I: Into<usize>, V>(Vec<V>, PhantomData<I>);

impl<I: Into<usize>, V> Deref for IndexVec<I, V> {
    type Target = Vec<V>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<I: Into<usize>, V> From<IndexVec<I, V>> for Vec<V> {
    fn from(val: IndexVec<I, V>) -> Self {
        val.0
    }
}

impl<I: Into<usize>, V> From<Vec<V>> for IndexVec<I, V> {
    fn from(value: Vec<V>) -> Self {
        IndexVec(value, PhantomData)
    }
}

impl<I: ReadEndian + Into<usize>, V: ReadEndian> ReadEndian for IndexVec<I, V> {
    fn get_le<R: Read + ?Sized>(reader: &mut R) -> Result<Self> {
        let len = I::get_le(reader)?.into();
        let mut values = Vec::with_capacity(len);

        for _ in 0..len {
            let val = V::get_le(reader)?;
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

impl<I: WriteEndian + TryFrom<usize> + Into<usize>, V: WriteEndian> WriteEndian for IndexVec<I, V> {
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

impl<I: Into<usize>> TryInto<String> for IndexVec<I, u8> {
    type Error = std::string::FromUtf8Error;

    fn try_into(self) -> std::result::Result<String, Self::Error> {
        String::from_utf8(self.0)
    }
}
