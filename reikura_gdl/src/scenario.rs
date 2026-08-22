use std::rc::Rc;

use anyhow::{Result, bail};

use crate::format::isf::IsfMetadata;

const 火: [u8; 2] = [0x89, 0xCE];
const 風: [u8; 2] = [0x95, 0x97];
const 林: [u8; 2] = [0x97, 0xD1];
const 桜: [u8; 2] = [0x8D, 0xF7];

#[derive(Clone)]
pub struct Scenario {
    pub code_offset: usize,
    pub name: Rc<str>,
    pub code: Rc<[u8]>,
    pub subroutines: Rc<[usize]>,
}

impl Scenario {
    pub fn load(name: String, mut data: Vec<u8>) -> Result<Self> {
        let isf = IsfMetadata::parse(&data)?;

        let code_offset = isf.bytecode_offset as usize;
        let table_start = 8; // start offset of jump table. from here on data is encrypted

        // decrypting
        let encrypted = data.iter_mut().skip(table_start);
        match isf.version {
            火 => encrypted.for_each(|byte| *byte ^= isf.xor_key),
            風 => encrypted.for_each(|byte| *byte = byte.rotate_right(2)),
            林 => encrypted.for_each(|byte| *byte = !*byte),
            桜 => (), // debug version, no encryption
            ver => bail!("unsupported scenario version: {ver:?}"),
        }

        let table_count = (code_offset - table_start) / size_of::<u32>();
        let mut jump_table = Vec::with_capacity(table_count);

        for chunk in data[table_start..code_offset].as_chunks().0 {
            let offset = u32::from_le_bytes(*chunk);
            jump_table.push(offset as usize);
        }

        Ok(Self {
            name: name.into(),
            code: data.split_off(code_offset).into(),
            code_offset,
            subroutines: Rc::from(jump_table),
        })
    }
}
