use std::{collections::HashMap, fs::File, io::Read};

use anyhow::bail;
use reikura_util::{
    encoding::{InvalidSJIS, sjis_to_utf8},
    io::ReadExt,
};

use crate::{
    AssetName,
    asset::{ArchiveEntry, MAX_ASSETNAME_LEN},
};

pub struct DrsArc {
    pub entries: Vec<DrsArcEntry>,
}

const ENTRY_CHUNK_LEN: usize = 16;

impl DrsArc {
    pub fn parse(file: &mut File) -> anyhow::Result<Self> {
        let entries_len = file.read_le::<u16>()? as usize;
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

        for chunk in entries_chunk.chunks_exact(ENTRY_CHUNK_LEN) {
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

    pub(crate) fn entries_index(self) -> anyhow::Result<HashMap<AssetName, ArchiveEntry>> {
        let mut entries = HashMap::with_capacity(self.entries.len());

        for entry in self.entries {
            let key = AssetName::from_buffer(entry.filename);
            let value = ArchiveEntry::try_from(entry)?;

            entries.insert(key, value);
        }

        Ok(entries)
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
            filename: chunk.read_le()?,
            offset: chunk.read_le()?,
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
            .unwrap_or(MAX_ASSETNAME_LEN);

        let filename = sjis_to_utf8(&entry.filename[..end])?;

        Ok(Self {
            filename,
            offset: entry.offset as usize,
            length: entry.length as usize,
        })
    }
}
