use std::{
    fs::File,
    io::{Read, Seek},
};

use anyhow::Result;
use reikura_util::{
    encoding::{InvalidSJIS, sjis_to_utf8},
    io::ReadExt,
};

use crate::{ArchiveEntry, ArchiveIndex, AssetName};

pub struct Sm2mpx10 {
    pub entries: Vec<Sm2mpx10Entry>,
}

impl Sm2mpx10 {
    const MAGIC: &[u8] = b"SM2MPX10";

    pub fn parse(file: &mut File) -> Result<Self> {
        let magic: [u8; 8] = file.get_le()?;

        debug_assert_eq!(magic, Self::MAGIC);

        let count: u32 = file.get_le()?;
        file.seek(std::io::SeekFrom::Start(32))?;

        let entries_chunk = {
            let mut buf = vec![0; size_of::<Sm2mpx10Entry>() * count as usize];
            file.read_exact(&mut buf)?;
            buf
        };
        let mut entries = Vec::with_capacity(count as usize);

        for chunk in entries_chunk
            .as_chunks::<{ size_of::<Sm2mpx10Entry>() }>()
            .0
        {
            let entry = Sm2mpx10Entry::parse(chunk)?;
            entries.push(entry);
        }

        Ok(Self { entries })
    }
}

impl ArchiveIndex for Sm2mpx10 {
    fn entries_len(&self) -> usize {
        self.entries.len()
    }

    fn entries(self) -> impl Iterator<Item = (AssetName, Result<ArchiveEntry, InvalidSJIS>)> {
        self.entries.into_iter().map(|entry| {
            let key = AssetName::from_buffer(entry.filename);
            let value = ArchiveEntry::try_from(entry);

            (key, value)
        })
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
            filename: chunk.get_le()?,
            offset: chunk.get_le()?,
            length: chunk.get_le()?,
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
            .unwrap_or(AssetName::LEN);

        let filename = sjis_to_utf8(&entry.filename[..end])?;

        Ok(Self {
            filename,
            offset: entry.offset as usize,
            length: entry.length as usize,
        })
    }
}
