use std::rc::Rc;

use anyhow::Result;
use reikura_util::io::WriteExt;

use crate::format::isf::IsfMetadata;

const 風: [u8; 2] = [0x95, 0x97];
const 林: [u8; 2] = [0x97, 0xD1];
const 火: [u8; 2] = [0x89, 0xCE];
const 桜: [u8; 2] = [0x8D, 0xF7];

#[derive(Clone)]
pub struct Scenario {
    pub name: Rc<str>,
    pub code: Rc<[u8]>,
    pub subroutines: Rc<[usize]>,
    pub metadata: ScenarioMetadata,
}

impl Scenario {
    pub fn load(name: String, mut data: Vec<u8>) -> Result<Self> {
        let isf = IsfMetadata::parse(&data)?;
        let code_offset = isf.bytecode_offset as usize;
        let subs_offset = 8; // offset of the subroutine table. from here on data is encrypted
        let subs_count = (code_offset - subs_offset) / size_of::<u32>();

        let metadata = ScenarioMetadata::try_from(isf)?;
        metadata.decrypt(&mut data[subs_offset..]);

        // TODO: debug version has different layout
        let mut subs_table = Vec::with_capacity(subs_count);
        for chunk in data[subs_offset..code_offset].as_chunks().0 {
            let sub_offset = u32::from_le_bytes(*chunk);
            subs_table.push(sub_offset as usize);
        }

        Ok(Self {
            name: name.into(),
            code: data.split_off(code_offset).into(),
            subroutines: Rc::from(subs_table),
            metadata,
        })
    }

    pub fn is_debug_version(&self) -> bool {
        if cfg!(debug_assertions) {
            return true;
        }

        self.metadata.version == ScenarioVersion::Sakura
    }

    pub fn sub_offset(&self, index: usize) -> Option<usize> {
        self.subroutines.get(index).copied()
    }
}

#[derive(Debug, Clone, Copy, Default)]
pub struct ScenarioMetadata {
    pub code_offset: usize,
    pub version: ScenarioVersion,
    pub xor_key: u8,
}

impl TryFrom<IsfMetadata> for ScenarioMetadata {
    type Error = InvalidScenarioVersion;

    fn try_from(md: IsfMetadata) -> Result<Self, Self::Error> {
        let version = match md.version {
            風 => ScenarioVersion::Fu,
            林 => ScenarioVersion::Rin,
            火 => ScenarioVersion::Ka,
            桜 => ScenarioVersion::Sakura,
            invalid => return Err(InvalidScenarioVersion(invalid)),
        };

        Ok(Self {
            code_offset: md.bytecode_offset as usize,
            version,
            xor_key: md.xor_key,
        })
    }
}

impl ScenarioMetadata {
    pub fn decrypt(&self, data: &mut [u8]) {
        let data = data.iter_mut();
        match self.version {
            ScenarioVersion::Fu => data.for_each(|byte| *byte = byte.rotate_right(2)),
            ScenarioVersion::Rin => data.for_each(|byte| *byte = !*byte),
            ScenarioVersion::Ka => {
                if self.xor_key != 0 {
                    data.for_each(|byte| *byte ^= self.xor_key);
                }
            }
            ScenarioVersion::Sakura => (), // debug version, no encryption
        }
    }

    pub fn encrypt(&self, data: &mut [u8]) {
        let data = data.iter_mut();
        match self.version {
            ScenarioVersion::Fu => data.for_each(|byte| *byte = byte.rotate_left(2)),
            ScenarioVersion::Rin => data.for_each(|byte| *byte = !*byte),
            ScenarioVersion::Ka => {
                if self.xor_key != 0 {
                    data.for_each(|byte| *byte ^= self.xor_key);
                }
            }
            ScenarioVersion::Sakura => (), // debug version, no encryption
        }
    }

    pub fn serialize(&self) -> [u8; 8] {
        let mut serialized = [0; 8];
        let mut s = serialized.as_mut_slice();

        _ = s.put_le(self.code_offset as u32);
        match self.version {
            ScenarioVersion::Fu => _ = s.put_bytes(風),
            ScenarioVersion::Rin => _ = s.put_bytes(林),
            ScenarioVersion::Ka => _ = s.put_bytes(火),
            ScenarioVersion::Sakura => _ = s.put_bytes(桜),
        };
        _ = s.put_bytes([self.xor_key, 0]);

        serialized
    }
}

#[derive(Debug)]
pub struct InvalidScenarioVersion([u8; 2]);

impl std::fmt::Display for InvalidScenarioVersion {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "invalid scenario version: {:?}", self.0)
    }
}

impl std::error::Error for InvalidScenarioVersion {}

#[derive(Debug, Clone, Copy, PartialEq, Default)]
pub enum ScenarioVersion {
    #[default]
    Fu,
    Rin,
    Ka,
    Sakura,
}
