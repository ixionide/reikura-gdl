use std::{
    fs::File,
    io::{Read, Seek},
};

use anyhow::Result;
use reikura_util::{
    encoding::{InvalidSJIS, sjis_to_utf8},
    io::ReadExt,
};

use crate::asset::ArchiveEntry;

const MAX_NAME_LENGTH: usize = 12;

pub struct Sm2mpx10 {
    pub entries: Vec<Sm2mpx10Entry>,
}

impl Sm2mpx10 {
    const MAGIC: &[u8] = b"SM2MPX10";

    pub fn parse(file: &mut File) -> Result<Self> {
        let magic: [u8; 8] = file.read_le()?;

        debug_assert_eq!(magic, Self::MAGIC);

        let count: u32 = file.read_le()?;
        file.seek(std::io::SeekFrom::Start(32))?;

        let mut buf = vec![0; size_of::<Sm2mpx10Entry>() * count as usize];
        file.read_exact(&mut buf)?;
        let mut entries = Vec::with_capacity(count as usize);

        for chunk in buf.chunks_exact(size_of::<Sm2mpx10Entry>()) {
            let entry = Sm2mpx10Entry::parse(chunk)?;
            entries.push(entry);
        }

        Ok(Self { entries })
    }
}

pub struct Sm2mpx10Entry {
    pub filename: [u8; 12],
    pub offset: u32,
    pub length: u32,
}

impl Sm2mpx10Entry {
    pub fn parse(mut chunk: &[u8]) -> anyhow::Result<Self> {
        Ok(Self {
            filename: chunk.read_le()?,
            offset: chunk.read_le()?,
            length: chunk.read_le()?,
        })
    }
}

impl TryFrom<Sm2mpx10Entry> for ArchiveEntry {
    type Error = InvalidSJIS;

    fn try_from(entry: Sm2mpx10Entry) -> Result<Self, Self::Error> {
        let end = entry
            .filename
            .iter()
            .position(|b| *b == 0)
            .unwrap_or(MAX_NAME_LENGTH);

        let filename = sjis_to_utf8(entry.filename[..end].to_vec())?;

        Ok(Self {
            filename,
            offset: entry.offset as usize,
            length: entry.length as usize,
        })
    }
}
