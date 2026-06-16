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
    pub state: State,
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

pub enum State {
    Exit,
    Running,
    Wait { start: Instant, duration: Duration },
    WaitClick,
    WaitText,
    WaitTransition,
    WaitVoice,
    WaitVideo,
}

impl State {
    pub fn exit(&mut self) {
        *self = Self::Exit;
    }

    pub fn run(&mut self) {
        *self = Self::Running;
    }

    pub fn wait(&mut self, duration: Duration) {
        *self = Self::Wait {
            start: Instant::now(),
            duration,
        }
    }

    pub fn wait_click(&mut self) {
        *self = Self::WaitClick;
    }

    pub fn wait_text(&mut self) {
        *self = Self::WaitText;
    }

    pub fn wait_voice(&mut self) {
        *self = Self::WaitVoice;
    }

    pub fn wait_video(&mut self) {
        *self = Self::WaitVideo;
    }
}
