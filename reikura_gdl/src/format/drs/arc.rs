use std::{fs::File, io::Read};

use anyhow::bail;
use reikura_util::{
    encoding::{InvalidSJIS, sjis_to_utf8},
    io::ReadExt,
};

use crate::{ArchiveEntry, ArchiveIndex, AssetName};

pub struct DrsArc {
    pub entries: Vec<DrsArcEntry>,
}

impl DrsArc {
    pub fn parse(file: &mut File) -> anyhow::Result<Self> {
        const ENTRY_CHUNK_LEN: usize = 16;

        let entries_len = file.get_le::<u16>()? as usize;
        let file_size = file.metadata()?.len() as usize;

        if !entries_len.is_multiple_of(ENTRY_CHUNK_LEN) || entries_len > file_size {
            bail!("invalid Digital Romance System Archive");
        }

        let entries_chunk = {
            let mut buf = vec![0; entries_len];
            file.read_exact(&mut buf)?;
            buf
        };

        let mut entries: Vec<DrsArcEntry> = {
            let count = entries_len / ENTRY_CHUNK_LEN;
            Vec::with_capacity(count)
        };

        for chunk in entries_chunk.as_chunks::<ENTRY_CHUNK_LEN>().0 {
            let entry = DrsArcEntry::parse(chunk)?;

            if let Some(prev) = entries.last_mut() {
                prev.length = entry.offset - prev.offset;
            }

            entries.push(entry);
        }

        if let Some(terminator) = entries.pop() {
            debug_assert_eq!(file_size, terminator.offset as _);
        };

        Ok(Self { entries })
    }
}

impl ArchiveIndex for DrsArc {
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

pub struct DrsArcEntry {
    pub filename: [u8; 12],
    pub offset: u32,
    pub length: u32,
}

impl DrsArcEntry {
    pub fn parse(mut chunk: &[u8]) -> anyhow::Result<Self> {
        Ok(Self {
            filename: chunk.get_le()?,
            offset: chunk.get_le()?,
            length: 0,
        })
    }
}

impl TryFrom<DrsArcEntry> for ArchiveEntry {
    type Error = InvalidSJIS;

    fn try_from(entry: DrsArcEntry) -> Result<Self, Self::Error> {
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
