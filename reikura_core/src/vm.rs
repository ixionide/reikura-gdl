use std::{
    collections::HashMap,
    io::{Seek, SeekFrom},
    time::{Duration, Instant},
};

use reikura_util::{bitset::BitSet, variable::Variables};

use crate::{
    AssetManager, AudioManager, Config, Manifest, SaveManager, Scenario,
    instruction::{INSTRUCTIONS, ReadParam},
};

pub struct VmContext {
    pub flags: BitSet,
    // patterns: HashMap<u8, FlagPattern>,
    pub variables: Variables,
    pub characters: HashMap<u8, String>,
    pub wait_time: Option<i32>,
    pub time_counter: Option<(Instant, Duration)>,
}

pub struct Vm {
    pub manifest: Manifest,
    pub assets: AssetManager,
    pub audio: AudioManager,
    pub config: Config,
    pub ctx: VmContext,
    pub save: Option<SaveManager>,
    pub scene: Scenario,
}

impl Vm {
    pub fn update(&mut self) -> anyhow::Result<()> {
        let op = self.scene.read_opcode()?;
        let inst = INSTRUCTIONS[op as usize];
        let info = self.scene.param()?;
        inst(self, info)?;

        if info.end_of_scenario() {
            self.scene.seek(SeekFrom::End(0))?;
        }

        Ok(())
    }
}
