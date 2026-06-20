use std::{
    io::{Seek, SeekFrom},
    time::{Duration, Instant},
};

use reikura_util::{bitset::BitSet, variable::Variables};

use crate::{
    AssetManager, AudioManager, Config, Manifest, SaveManager, Scenario,
    instruction::{INSTRUCTIONS, ReadParam},
};

pub const FRAME_DURATION: Duration = Duration::from_millis(16);

pub struct VmContext {
    pub flags: BitSet,
    // patterns: HashMap<u8, FlagPattern>,
    pub variables: Variables,
    pub wait_duration: Option<Duration>,
    pub timer: Option<Timer>,
    // TODO: this make much more sense stored in the text renderer struct
    pub char_names: [Option<String>; 256],
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
        match self.state {
            State::Exit => todo!(),
            State::Running => {
                let op = self.scene.read_opcode()?;
                let inst = INSTRUCTIONS[op as usize];
                let info = self.scene.param()?;
                inst(self, info)?;

                if info.end_of_scenario() {
                    self.scene.seek(SeekFrom::End(0))?;
                }
            }
            State::Wait { start, duration } => {
                let elapsed = start.elapsed();

                if elapsed > duration {
                    self.state.run();
                    return Ok(());
                }

                let time_remaining = duration.saturating_sub(elapsed);
                let sleep_duration = FRAME_DURATION.min(time_remaining);
                std::thread::sleep(sleep_duration);
            }
            State::WaitClick => todo!(),
            State::WaitText => todo!(),
            State::WaitTransition => todo!(),
            State::WaitVoice => {
                if self.audio.track.voice.is_audio_finished() {
                    self.state.run();
                }
            }
            State::WaitVideo => todo!(),
        }

        Ok(())
    }
}

pub struct Timer {
    start: Instant,
    delay: i32,
}

impl Timer {
    pub fn new(delay: i32) -> Self {
        Self {
            start: Instant::now(),
            delay,
        }
    }

    pub fn get(&self) -> i32 {
        let elapsed = self.start.elapsed().as_millis() as i32;
        // NOTE: not sure if saturating is accurate here
        elapsed.saturating_sub(self.delay)
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
