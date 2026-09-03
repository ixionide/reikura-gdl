use std::{
    collections::HashMap,
    fs::File,
    io::{Read, Seek, SeekFrom},
    path::Path,
};

use anyhow::{Context, Result, bail};
use reikura_util::{encoding::InvalidSJIS, io::ReadExt};

use crate::{
    AssetName,
    format::{drs::DrsArc, sm2mpx10::Sm2mpx10},
};

pub struct VmArchive {
    pub main: Archive,
    pub extra: Vec<Archive>,
}

impl VmArchive {
    pub fn load(path: impl AsRef<Path>) -> Result<Self> {
        let path = path.as_ref();
        let main = Archive::load(path)?;
        let mut extra = vec![];

        let mut i = 1;
        while let Ok(extra_arc) = Archive::load(format!("{}{i}", path.to_string_lossy())) {
            extra.push(extra_arc);
            i += 1;
        }

        Ok(Self { main, extra })
    }

    pub fn get_asset(&mut self, name: &AssetName) -> Result<(String, Vec<u8>)> {
        self.extra
            .iter_mut()
            .find(|arc| arc.asset_exist(name))
            .unwrap_or(&mut self.main)
            .get_asset(name)
    }
}

pub struct ArchiveEntry {
    pub filename: String,
    pub offset: usize,
    pub length: usize,
}

pub struct Archive {
    file: File,
    index: HashMap<AssetName, ArchiveEntry>,
}

impl Archive {
    pub fn load(path: impl AsRef<Path>) -> Result<Self> {
        let path = path.as_ref();
        let err_ctx = || format!("failed to load archive: {}", path.display());
        let mut file = File::open(path).with_context(err_ctx)?;

        let index = {
            let magic = file.get_bytes::<8>()?;
            file.rewind()?;

            match &magic {
                b"SM2MPX10" => Sm2mpx10::parse(&mut file).with_context(err_ctx)?.index()?,
                b"SM2MPX20" => bail!("sm2mpx20 archive is not yet supported"),
                _ => DrsArc::parse(&mut file).with_context(err_ctx)?.index()?,
            }
        };

        Ok(Self { file, index })
    }

    fn asset_exist(&self, name: &AssetName) -> bool {
        self.index.contains_key(name)
    }

    pub fn get_asset(&mut self, name: &AssetName) -> Result<(String, Vec<u8>)> {
        let entry = self
            .index
            .get(name)
            .with_context(|| format!("asset {name} not found"))?;

        let pos = SeekFrom::Start(entry.offset as u64);
        let len = entry.length;

        let mut buffer = vec![0; len];

        self.file.seek(pos)?;
        self.file.read_exact(&mut buffer)?;

        Ok((entry.filename.clone(), buffer))
    }
}

pub trait ArchiveIndex: Sized {
    fn entries_len(&self) -> usize;

    fn entries(
        self,
    ) -> impl Iterator<Item = (AssetName, std::result::Result<ArchiveEntry, InvalidSJIS>)>;

    fn index(self) -> Result<HashMap<AssetName, ArchiveEntry>> {
        let mut index = HashMap::with_capacity(self.entries_len());

        for (key, value) in self.entries() {
            index.insert(key, value?);
        }

        Ok(index)
    }
}
