use std::{
    io::{Seek, SeekFrom},
    time::{Duration, Instant},
};

use anyhow::Context;
use reikura_gfx::GraphicEngine;
use reikura_util::{bitset::BitSet, register::Register};

use crate::{
    AssetManager, AssetName, AudioManager, Config, InputManager, Manifest, Parser, SaveManager,
    instruction::{INSTRUCTIONS, InstructionInfo},
};

pub const FRAME_DURATION: Duration = Duration::from_millis(16);

pub struct VmContext {
    pub flags: BitSet,
    pub registers: Register,
    pub wait_duration: Option<Duration>,
    pub timer: Option<Timer>,
    // TODO: this make much more sense stored in the text renderer struct
    pub char_names: [Option<String>; 256],
    pub flag_groups: [Option<Vec<usize>>; 256],
}

impl VmContext {
    pub fn new() -> Self {
        Self {
            flags: BitSet::new(2048),
            registers: Register::new(2048),
            wait_duration: None,
            timer: None,
            char_names: [const { None }; 256],
            flag_groups: [const { None }; 256],
        }
    }
}

pub struct Vm {
    pub manifest: Manifest,
    pub assets: AssetManager,
    pub parser: Parser,
    pub audio: AudioManager,
    pub gfx: GraphicEngine,
    pub ctx: VmContext,
    pub save: Option<SaveManager>,
    pub config: Config,
    pub input: InputManager,
    pub state: State,
}

impl Vm {
    pub fn new(manifest: Manifest, gfx: GraphicEngine) -> anyhow::Result<Self> {
        let mut assets = AssetManager::new(&manifest)?;
        let input = InputManager::new(manifest.view_size);
        let config_path = manifest.game_path().join("user_setup");
        let start_scene = assets
            .load_scene(AssetName::START)
            .context("failed to load start script")?;

        Ok(Self {
            manifest,
            assets,
            parser: Parser::new(start_scene),
            audio: AudioManager::new(1.0, 1.0, 1.0)?,
            gfx,
            ctx: VmContext::new(),
            save: None,
            config: Config::open(config_path)?,
            input,
            state: State::Running,
        })
    }

    pub fn update(&mut self) -> anyhow::Result<()> {
        match self.state {
            State::Exit => todo!(),
            State::Running => {
                let op = self.parser.read_opcode()?;
                let inst = INSTRUCTIONS[op as usize];
                let info = self.parser.read_param::<InstructionInfo>()?;

                #[cfg(debug_assertions)]
                {
                    let next_ip = self.parser.state.ip + info.param_length();
                    if let Err(err) = inst(self, info) {
                        dbg!(err);
                        self.parser.state.ip = next_ip
                    }
                }

                #[cfg(not(debug_assertions))]
                inst(self, info)?;

                if info.end_of_scenario() {
                    self.parser.seek(SeekFrom::End(0))?;
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
    duration: i32,
}

impl Timer {
    pub fn new(duration: i32) -> Self {
        Self {
            start: Instant::now(),
            duration,
        }
    }

    pub fn get(&self) -> i32 {
        let elapsed = self.start.elapsed().as_millis() as i32;
        // NOTE: not sure if saturating is accurate here
        elapsed.saturating_sub(self.duration)
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
